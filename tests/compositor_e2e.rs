//! Compositor end-to-end: the Claude-Code-shaped LSP loop THROUGH the
//! lsp-max-compositor front door (not against the child server directly).
//!
//! Shape copied from the wasm4pm-lsp dogfood harness (dogfood_gc005.rs):
//! explicit-path spawn, Content-Length framing, mpsc reader, bounded waits.
//!
//! CARGO_BIN_EXE is unavailable here (the compositor lives in a foreign
//! workspace), so both binaries are resolved via explicit filesystem paths:
//!   - compositor: ../lsp-max/target/release/lsp-max-compositor
//!   - child:      the config-lsp binary built by this crate's release build
//!
//! Override with LSP_MAX_COMPOSITOR_BIN / CLAUDE_CODE_CONFIG_LSP_BIN.
//!
//! Loop: initialize → didOpen settings.json with an invalid model enum
//! (CCC-JSON-004) → diagnostics arrive through the compositor → didChange
//! fixes the violation → diagnostics clear → .md route (CCC-MD-006) →
//! shutdown/exit.

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};
use std::{fs, thread};

const READ_TIMEOUT: Duration = Duration::from_secs(30);
const EXIT_TIMEOUT: Duration = Duration::from_secs(15);

struct LspClient {
    stdin: Option<ChildStdin>,
    rx: Receiver<Value>,
    child: Child,
    next_id: i64,
    stashed: Vec<Value>,
}

impl LspClient {
    /// Spawn `bin` with cwd `cwd` and speak LSP over its stdio.
    fn spawn(bin: &str, cwd: &std::path::Path, label: &'static str) -> Self {
        let mut child = Command::new(bin)
            .arg("serve")
            .arg("--stdio")
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap_or_else(|e| panic!("spawn {label} at {bin}: {e}"));

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
                if let Some(rest) = line.strip_prefix("Content-Length: ") {
                    let len: usize = rest.trim().parse().expect("Content-Length parse");
                    // Header/JSON-RPC pipes are ASCII here, so byte length ==
                    // char length; still read the terminator as bytes to be exact.
                    let mut term = [0u8; 2];
                    let _ = reader.read_exact(&mut term);
                    let mut body = vec![0u8; len];
                    if reader.read_exact(&mut body).is_err() {
                        break;
                    }
                    let msg: Value = match serde_json::from_slice(&body) {
                        Ok(v) => v,
                        Err(_) => break,
                    };
                    if tx.send(msg).is_err() {
                        break;
                    }
                }
            }
        });

        Self {
            stdin: Some(stdin),
            rx,
            child,
            next_id: 1,
            stashed: Vec::new(),
        }
    }

    fn send(&mut self, msg: Value) {
        let body = msg.to_string();
        let frame = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        let stdin = self.stdin.as_mut().expect("stdin still open");
        stdin.write_all(frame.as_bytes()).expect("write frame");
        stdin.flush().expect("flush stdin");
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        self.send(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}));
        loop {
            let msg = self
                .rx
                .recv_timeout(READ_TIMEOUT)
                .unwrap_or_else(|_| panic!("{method} request timeout"));
            if msg.get("id") == Some(&json!(id)) {
                return msg;
            }
            self.stashed.push(msg);
        }
    }

    fn notify(&mut self, method: &str, params: Value) {
        self.send(json!({"jsonrpc": "2.0", "method": method, "params": params}));
    }

    /// Close stdin: the transport's serve loop reads until EOF, so `exit`
    /// alone does not terminate the compositor process — EOF does.
    fn close_stdin(&mut self) {
        self.stdin = None;
    }

    /// Bounded poll: keep consuming diagnostics for `uri` until `pred` holds
    /// or the deadline passes. Asserts the final observed state.
    fn wait_diagnostics_until<F>(&mut self, uri: &str, pred: F) -> Vec<Value>
    where
        F: Fn(&[Value]) -> bool,
    {
        let deadline = Instant::now() + READ_TIMEOUT;
        loop {
            let diags = self.next_diagnostics_for_with_deadline(uri, deadline);
            if pred(&diags) {
                return diags;
            }
            if Instant::now() >= deadline {
                panic!("deadline passed; last diagnostics for {uri}: {diags:?}");
            }
        }
    }

    fn next_diagnostics_for_with_deadline(&mut self, uri: &str, deadline: Instant) -> Vec<Value> {
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                panic!("timed out waiting for publishDiagnostics for {uri}");
            }
            match self.rx.recv_timeout(remaining) {
                Ok(msg) => {
                    if msg.get("method") == Some(&json!("textDocument/publishDiagnostics"))
                        && msg.pointer("/params/uri").and_then(|u| u.as_str()) == Some(uri)
                    {
                        return msg
                            .pointer("/params/diagnostics")
                            .cloned()
                            .unwrap_or(json!([]))
                            .as_array()
                            .cloned()
                            .unwrap_or_default();
                    }
                    self.stashed.push(msg);
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    panic!("timed out waiting for publishDiagnostics for {uri}");
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    panic!("compositor stdout closed before diagnostics for {uri}");
                }
            }
        }
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn codes(diags: &[Value]) -> Vec<String> {
    diags
        .iter()
        .filter_map(|d| d.get("code").and_then(|c| c.as_str()))
        .map(|s| s.to_string())
        .collect()
}

fn release_bin(repo_root: &std::path::Path, name: &str, env_key: &str) -> std::path::PathBuf {
    if let Ok(p) = std::env::var(env_key) {
        let path = std::path::PathBuf::from(&p);
        assert!(path.exists(), "{env_key}={} does not exist", p);
        return path;
    }
    let path = repo_root.join("target/release").join(name);
    assert!(
        path.exists(),
        "release binary missing at {}; build it first (cargo build --release)",
        path.display()
    );
    path
}

fn unique_tmp_dir(tag: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!(
        "ccc-e2e-{}-{}-{}",
        tag,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(d.join(".claude")).expect("create fixture dir");
    d
}

/// Build the compositor manifest for a fixture workspace: claude-code-config-lsp
/// as the diagnostics-only child for .json/.md, command = absolute binary path.
fn write_compositor_manifest(dir: &std::path::Path, child_bin: &std::path::Path) {
    let child_bin = child_bin.to_str().expect("utf8 path");
    let toml = format!(
        r#"
[[server]]
id = "claude-code-config-lsp"
primary_extensions = [".json", ".md"]
secondary_extensions = []
priority = "diagnostics-only"
andon_code_prefixes = ["CCC-"]
command = "{child_bin}"
args = ["serve", "--stdio"]
"#,
        child_bin = child_bin,
    );
    fs::write(dir.join("lsp-max.toml"), toml).expect("write lsp-max.toml");
}

#[test]
fn compositor_front_door_end_to_end() {
    let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lsp_max_repo = workspace.parent().expect("parent dir").join("lsp-max");

    let compositor_bin = release_bin(
        &lsp_max_repo,
        "lsp-max-compositor",
        "LSP_MAX_COMPOSITOR_BIN",
    );
    let child_bin = release_bin(
        &workspace,
        "claude-code-config-lsp",
        "CLAUDE_CODE_CONFIG_LSP_BIN",
    );

    // Fixture workspace: manifest + config files. The compositor resolves
    // lsp-max.toml by walking up from its cwd, so cwd must be this dir.
    let dir = unique_tmp_dir("frontdoor");
    write_compositor_manifest(&dir, &child_bin);

    let settings_path = dir.join("settings.json");
    let md_path = dir.join("CLAUDE.md");
    let settings_uri = format!("file://{}", settings_path.display());
    let md_uri = format!("file://{}", md_path.display());

    let mut client = LspClient::spawn(
        compositor_bin.to_str().expect("utf8 compositor path"),
        &dir,
        "lsp-max-compositor",
    );

    // ── initialize ───────────────────────────────────────────────────────────
    let init = client.request(
        "initialize",
        json!({
            "capabilities": {},
            "rootUri": format!("file://{}", dir.display()),
        }),
    );
    let caps = init.get("result").expect("initialize result");
    assert_eq!(
        caps.pointer("/serverInfo/name").and_then(|v| v.as_str()),
        Some("lsp-max-compositor"),
        "front door must be the compositor, not the child"
    );
    client.notify("initialized", json!({}));

    // ── didOpen settings.json with invalid model enum → CCC-JSON-004 ────────
    let violating = r#"{
  "model": "gpt-4",
  "cleanupPeriodDays": 30
}"#;
    client.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": settings_uri,
                "languageId": "json",
                "version": 1,
                "text": violating,
            }
        }),
    );

    let diags = client.wait_diagnostics_until(&settings_uri, |d| {
        codes(d).iter().any(|c| c == "CCC-JSON-004")
    });
    assert!(
        codes(&diags).contains(&"CCC-JSON-004".to_string()),
        "expected CCC-JSON-004 through the compositor, got {:?}",
        codes(&diags)
    );
    // The diagnostic must genuinely be the child's, relayed by the compositor:
    // source relayed as compositor/<tier>:<server_id>, code preserved verbatim.
    let diag = diags
        .iter()
        .find(|d| d.get("code").and_then(|c| c.as_str()) == Some("CCC-JSON-004"))
        .expect("CCC-JSON-004 diag present");
    let source = diag.get("source").and_then(|s| s.as_str()).unwrap_or("");
    assert!(
        source.contains("compositor") && source.contains("claude-code-config-lsp"),
        "diagnostic source should identify compositor relay of the child, got {source:?}"
    );

    // ── didChange fixes the violation → diagnostics clear ────────────────────
    let fixed = r#"{
  "model": "sonnet",
  "cleanupPeriodDays": 30
}"#;
    client.notify(
        "textDocument/didChange",
        json!({
            "textDocument": { "uri": settings_uri, "version": 2 },
            "contentChanges": [{ "text": fixed }]
        }),
    );
    client.wait_diagnostics_until(&settings_uri, |d| {
        !codes(d).iter().any(|c| c == "CCC-JSON-004")
    });

    // ── .md route: CLAUDE.md without an H1 title → CCC-MD-006 ───────────────
    client.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": md_uri,
                "languageId": "markdown",
                "version": 1,
                "text": "no heading here\njust prose\n",
            }
        }),
    );
    let md_diags =
        client.wait_diagnostics_until(&md_uri, |d| codes(d).iter().any(|c| c == "CCC-MD-006"));
    assert!(
        codes(&md_diags).contains(&"CCC-MD-006".to_string()),
        "expected CCC-MD-006 through the compositor .md route, got {:?}",
        codes(&md_diags)
    );

    // ── commands route: .claude/commands/*.md without a description surface
    // → CCC-CMD-001, relayed through the compositor with source attribution.
    // The path is a REAL file path under the fixture workspace, so the scan
    // dispatch resolves its parent dir as base_dir (no CCC-CMD-003 here — the
    // fixture carries no @file references).
    let cmd_path = dir.join(".claude/commands/deploy.md");
    fs::create_dir_all(cmd_path.parent().expect("commands parent dir"))
        .expect("create commands dir");
    fs::write(&cmd_path, "---\nhandoffs:\n  - label: Ship\n---\n\nDeploy it.\n")
        .expect("write command fixture");
    let cmd_uri = format!("file://{}", cmd_path.display());
    client.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": cmd_uri,
                "languageId": "markdown",
                "version": 1,
                "text": "---\nhandoffs:\n  - label: Ship\n---\n\nDeploy it.\n",
            }
        }),
    );
    let cmd_diags = client.wait_diagnostics_until(&cmd_uri, |d| {
        codes(d).iter().any(|c| c == "CCC-CMD-001")
    });
    let cmd_diag = cmd_diags
        .iter()
        .find(|d| d.get("code").and_then(|c| c.as_str()) == Some("CCC-CMD-001"))
        .expect("CCC-CMD-001 diag present");
    let cmd_source = cmd_diag.get("source").and_then(|s| s.as_str()).unwrap_or("");
    assert!(
        cmd_source.contains("compositor") && cmd_source.contains("claude-code-config-lsp"),
        "commands diagnostic must be relayed with compositor attribution, got {cmd_source:?}"
    );

    // ── shutdown / exit ──────────────────────────────────────────────────────
    let shutdown = client.request("shutdown", json!(null));
    assert!(
        shutdown.get("error").is_none(),
        "shutdown must not error: {shutdown:?}"
    );
    client.notify("exit", json!(null));
    client.close_stdin(); // EOF terminates the serve loop

    let deadline = Instant::now() + EXIT_TIMEOUT;
    loop {
        match client.child.try_wait().expect("try_wait") {
            Some(status) => {
                assert!(
                    status.success(),
                    "compositor exit status not success: {status}"
                );
                break;
            }
            None => {
                assert!(
                    Instant::now() < deadline,
                    "compositor did not exit within {EXIT_TIMEOUT:?}"
                );
                thread::sleep(Duration::from_millis(50));
            }
        }
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn compositor_manifest_shape_matches_deployed_config() {
    // The deployed /Users/sac/claude-code-config-lsp/lsp-max.toml claims the
    // .json and .md extension routes for this server with diagnostics-only
    // priority. Pin that shape so a manifest drift fails here, not in the editor.
    let manifest =
        fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("lsp-max.toml"))
            .expect("read deployed lsp-max.toml");
    assert!(manifest.contains("id = \"claude-code-config-lsp\""));
    assert!(manifest.contains("primary_extensions = [\".json\", \".md\"]"));
    assert!(manifest.contains("priority = \"diagnostics-only\""));
}
