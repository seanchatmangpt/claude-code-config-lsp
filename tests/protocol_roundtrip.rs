//! Protocol round-trip suite: drives the real compiled `claude-code-config-lsp`
//! binary over the actual LSP wire (Content-Length-framed JSON-RPC on stdio).
//!
//! Harness pattern borrowed from lsp-max
//! `examples/wasm4pm-lsp/tests/dogfood_gc007_lsp318.rs` (CARGO_BIN_EXE spawn,
//! background reader + mpsc, stashed notifications).
//!
//! Covers:
//! 1. lifecycle: initialize → initialized → didOpen → publishDiagnostics → shutdown → exit
//! 2. diagnostic round-trip: a fixture settings.json with a known violation
//!    (`model: "gpt-4"` → CCC-JSON-004 invalid model enum)
//! 3. fixture corpus: every sanitized file under tests/fixtures/claude-config/
//!    gets didOpen'd; each must receive a well-formed publishDiagnostics notification.

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

const READ_TIMEOUT: Duration = Duration::from_secs(15);

// ── LspClient harness (pattern: dogfood_gc007_lsp318.rs) ──────────────────────

struct LspClient {
    stdin: Option<ChildStdin>,
    rx: Receiver<Value>,
    child: Child,
    next_id: i64,
    stashed_notifications: Vec<Value>,
}

impl LspClient {
    fn new() -> Self {
        let bin = env!("CARGO_BIN_EXE_claude-code-config-lsp");
        let mut child = Command::new(bin)
            .arg("serve")
            .arg("--stdio")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("spawn claude-code-config-lsp serve --stdio");

        let stdout = child.stdout.take().expect("take stdout");
        let stdin = child.stdin.take().expect("take stdin");
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).is_err() || line.is_empty() {
                    break;
                }
                if line.starts_with("Content-Length: ") {
                    let len: usize = line
                        .trim_start_matches("Content-Length: ")
                        .trim()
                        .parse()
                        .expect("parse Content-Length");
                    reader.read_line(&mut line).expect("read header terminator");
                    let mut body = vec![0u8; len];
                    reader.read_exact(&mut body).expect("read framed body");
                    let msg: Value = serde_json::from_slice(&body).expect("parse JSON-RPC body");
                    tx.send(msg).expect("reader thread send");
                }
            }
        });

        Self {
            stdin: Some(stdin),
            rx,
            child,
            next_id: 1,
            stashed_notifications: Vec::new(),
        }
    }

    fn send(&mut self, msg: Value) {
        let body = msg.to_string();
        let frame = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        let stdin = self.stdin.as_mut().expect("client stdin already closed");
        stdin.write_all(frame.as_bytes()).expect("write LSP frame");
        stdin.flush().expect("flush LSP frame");
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        self.send(json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }));
        loop {
            let msg = self
                .rx
                .recv_timeout(READ_TIMEOUT)
                .expect("LSP request timeout");
            if msg.get("id") == Some(&json!(id)) {
                return msg;
            }
            self.stashed_notifications.push(msg);
        }
    }

    fn notify(&mut self, method: &str, params: Value) {
        self.send(json!({ "jsonrpc": "2.0", "method": method, "params": params }));
    }

    /// Close the client side of the stdio pipe (drop the ChildStdin).
    fn close_stdin(&mut self) {
        self.stdin = None;
    }

    fn wait_for_notification(&mut self, method: &str) -> Value {
        if let Some(pos) = self
            .stashed_notifications
            .iter()
            .position(|n| n.get("method") == Some(&json!(method)))
        {
            return self.stashed_notifications.remove(pos);
        }
        loop {
            let msg = self
                .rx
                .recv_timeout(READ_TIMEOUT)
                .expect("LSP notification timeout");
            if msg.get("method") == Some(&json!(method)) {
                return msg;
            }
            self.stashed_notifications.push(msg);
        }
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

// ── Fixture helpers ───────────────────────────────────────────────────────────

fn fixtures_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/claude-config")
}

/// Invalid-model fixture: `"model": "gpt-4"` must fire CCC-JSON-004.
const INVALID_MODEL_TEXT: &str = r#"{
  "model": "gpt-4",
  "permissions": { "defaultMode": "bypassPermissions" }
}"#;

fn did_open(client: &mut LspClient, uri: &str, language_id: &str, text: &str) {
    client.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": uri,
                "languageId": language_id,
                "version": 1,
                "text": text
            }
        }),
    );
}

// ── 1. Lifecycle: initialize → initialized → didOpen → diagnostics → shutdown → exit ──

#[test]
fn lifecycle_initialize_to_exit_roundtrip() {
    let mut client = LspClient::new();

    // initialize — the response must be a well-formed JSON-RPC result object
    // advertising the core capabilities this suite exercises.
    let init_resp = client.request("initialize", json!({ "capabilities": {} }));
    assert!(
        init_resp.get("error").is_none(),
        "initialize must not error, got: {init_resp}"
    );
    let caps = init_resp["result"]["capabilities"]
        .as_object()
        .expect("initialize result.capabilities object");
    assert!(
        caps.contains_key("textDocumentSync"),
        "textDocumentSync must be advertised, got: {caps:?}"
    );
    assert!(
        caps.contains_key("hoverProvider"),
        "hoverProvider must be advertised, got: {caps:?}"
    );

    client.notify("initialized", json!({}));

    // didOpen a fixture and require a well-formed publishDiagnostics notification.
    let uri = "file:///tmp/protocol-roundtrip-lifecycle/settings.json";
    did_open(&mut client, uri, "json", INVALID_MODEL_TEXT);
    let diag = client.wait_for_notification("textDocument/publishDiagnostics");
    assert_eq!(
        diag["params"]["uri"].as_str(),
        Some(uri),
        "publishDiagnostics must echo the didOpen uri, got: {diag}"
    );
    assert!(
        diag["params"]["diagnostics"].is_array(),
        "publishDiagnostics params.diagnostics must be an array, got: {diag}"
    );
    assert!(
        diag["params"]["diagnostics"]
            .as_array()
            .expect("diagnostics array")
            .iter()
            .all(|d| d.get("code").is_some() || d.get("message").is_some()),
        "each diagnostic must carry code and/or message, got: {diag}"
    );

    // shutdown → exit (graceful lifecycle close, not a kill)
    let shutdown = client.request("shutdown", json!(null));
    assert!(
        shutdown.get("error").is_none(),
        "shutdown must not error, got: {shutdown}"
    );
    client.notify("exit", json!(null));

    // Close our side of stdin so the server's stdio read loop unblocks;
    // the server stops its main loop on `exit` and the process reaps once
    // the stdin reader sees EOF. (The ChildStdin lives in LspClient.stdin,
    // not child.stdin — that was taken at construction.)
    client.close_stdin();

    // Wait for the process to exit on its own after `exit`.
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        match client.child.try_wait().expect("try_wait") {
            Some(status) => {
                assert!(
                    status.success(),
                    "server must exit cleanly after exit notification, got: {status}"
                );
                break;
            }
            None => {
                assert!(
                    deadline > std::time::Instant::now(),
                    "server did not exit within 5s of the exit notification"
                );
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

// ── 2. Diagnostic round-trip: known violation → coded diagnostic ──────────────

#[test]
fn diagnostic_roundtrip_invalid_model_enum_fires_ccc_json_004() {
    let mut client = LspClient::new();
    let _init = client.request("initialize", json!({ "capabilities": {} }));
    client.notify("initialized", json!({}));

    let uri = "file:///tmp/protocol-roundtrip-diag/settings.json";
    did_open(&mut client, uri, "json", INVALID_MODEL_TEXT);
    let diag = client.wait_for_notification("textDocument/publishDiagnostics");

    assert_eq!(diag["params"]["uri"].as_str(), Some(uri));
    let diags = diag["params"]["diagnostics"]
        .as_array()
        .expect("diagnostics array");
    assert!(
        !diags.is_empty(),
        "model=gpt-4 must produce at least one diagnostic, got: {diag}"
    );
    let ccc_json_004 = diags
        .iter()
        .find(|d| d["code"].as_str() == Some("CCC-JSON-004"))
        .unwrap_or_else(|| panic!("expected a CCC-JSON-004 diagnostic, got: {diags:?}"));
    assert!(
        !ccc_json_004["message"].as_str().unwrap_or("").is_empty(),
        "CCC-JSON-004 diagnostic must carry a non-empty message"
    );
    // Range must be a real range object (not null/missing).
    assert!(
        ccc_json_004["range"]["start"].is_object() && ccc_json_004["range"]["end"].is_object(),
        "CCC-JSON-004 diagnostic must carry a range, got: {ccc_json_004}"
    );
}

// ── 3. Sanitized fixture corpus: every file gets diagnostics of the right shape ──

#[test]
fn fixture_corpus_roundtrip_publishes_well_formed_diagnostics() {
    let dir = fixtures_dir();
    let mut fixture_files: Vec<std::path::PathBuf> = Vec::new();
    collect(&dir, &mut fixture_files);
    fixture_files.sort();
    assert!(
        fixture_files.len() >= 10,
        "fixture corpus must contain at least 10 sanitized files, found: {fixture_files:?}"
    );

    let mut client = LspClient::new();
    let _init = client.request("initialize", json!({ "capabilities": {} }));
    client.notify("initialized", json!({}));

    for path in &fixture_files {
        let rel = path.strip_prefix(&dir).expect("strip fixture root");
        let uri = format!("file:///fixtures/claude-config/{}", rel.to_string_lossy());
        let text = std::fs::read_to_string(path).expect("read fixture");
        let language_id = if path.extension().and_then(|e| e.to_str()) == Some("json") {
            "json"
        } else {
            "markdown"
        };
        did_open(&mut client, &uri, language_id, &text);

        let diag = client.wait_for_notification("textDocument/publishDiagnostics");
        assert_eq!(
            diag["params"]["uri"].as_str(),
            Some(uri.as_str()),
            "publishDiagnostics uri must match the didOpen uri for {rel:?}, got: {diag}"
        );
        let diags = diag["params"]["diagnostics"]
            .as_array()
            .unwrap_or_else(|| panic!("diagnostics must be an array for {rel:?}, got: {diag}"));
        for d in diags {
            let code = d["code"].as_str().unwrap_or("");
            assert!(
                code.starts_with("CCC-"),
                "diagnostic code must be a CCC-* code for {rel:?}, got: {d}"
            );
            assert!(
                !d["message"].as_str().unwrap_or("").is_empty(),
                "diagnostic message must be non-empty for {rel:?}, got: {d}"
            );
        }
    }
}

fn collect(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            collect(&p, out);
        } else {
            out.push(p);
        }
    }
}
