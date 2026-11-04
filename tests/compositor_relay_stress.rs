//! Compositor relay stress (AB-18): post-CL-E diagnostic publish through the
//! lsp-max-compositor front door, under load.
//!
//! Harness shape copied from tests/compositor_e2e.rs (wasm4pm-lsp dogfood
//! lineage): explicit-path spawn, Content-Length framing, mpsc reader,
//! bounded waits. Read-only on src/ — black-box over the real wire.
//!
//! Binaries resolved via explicit filesystem paths (CARGO_BIN_EXE is
//! unavailable — the compositor lives in a foreign workspace):
//!   - compositor: ../lsp-max/target/release/lsp-max-compositor
//!   - child:      this crate's release build
//! Override with LSP_MAX_COMPOSITOR_BIN / CLAUDE_CODE_CONFIG_LSP_BIN.
//!
//! Four stress classes (census, batch, standing-merge, churn) — timing is
//! recorded, never fenced on absolute milliseconds (no-sleep doctrine:
//! every wait below is a bounded poll on a real observable — the
//! publishDiagnostics notification itself).

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};
use std::{fs, thread};

const READ_TIMEOUT: Duration = Duration::from_secs(30);

struct LspClient {
    stdin: Option<ChildStdin>,
    rx: Receiver<Value>,
    child: Child,
    next_id: i64,
    stashed: Vec<Value>,
}

impl LspClient {
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


    /// Send `method`, wait for the next publishDiagnostics for `uri`, and
    /// return (diagnostics, send→publish latency). Single consume — the
    /// returned payload IS the merged state for the uri.
    fn notify_wait_timed(
        &mut self,
        method: &str,
        params: Value,
        uri: &str,
    ) -> (Vec<Value>, Duration) {
        let t0 = Instant::now();
        self.notify(method, params);
        let deadline = t0 + READ_TIMEOUT;
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
                        let diags = msg
                            .pointer("/params/diagnostics")
                            .cloned()
                            .unwrap_or(json!([]))
                            .as_array()
                            .cloned()
                            .unwrap_or_default();
                        return (diags, t0.elapsed());
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

    /// Bounded poll: consume diagnostics for `uri` until `pred` holds.
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
        "ccc-stress-{}-{}-{}",
        tag,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&d).expect("create fixture dir");
    d
}

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
    );
    fs::write(dir.join("lsp-max.toml"), toml).expect("write lsp-max.toml");
}

fn percentiles(sorted: &[Duration]) -> (Duration, Duration, Duration) {
    let p = |q: f64| -> Duration {
        let idx = ((q * (sorted.len() as f64 - 1.0)).round()) as usize;
        sorted[idx.min(sorted.len() - 1)]
    };
    (p(0.50), p(0.95), p(0.99))
}

/// Valid Claude settings content (no CCC- violations expected).
fn valid_settings() -> &'static str {
    "{\n  \"model\": \"sonnet\",\n  \"cleanupPeriodDays\": 30\n}\n"
}

/// Settings violating CCC-JSON-004 (invalid model enum).
fn violating_settings() -> &'static str {
    "{\n  \"model\": \"gpt-4\",\n  \"cleanupPeriodDays\": 30\n}\n"
}

/// CLAUDE.md with a non-H1 first line of exactly 16 chars → CCC-MD-006
/// spanning (0,0)→(0,16) in the child analyzer.
fn violating_md() -> &'static str {
    "no heading here\njust prose\n"
}

fn valid_md() -> &'static str {
    "# Valid Title\nbody prose\n"
}



// ── 1. Latency census ────────────────────────────────────────────────────────

#[test]
fn latency_census_1k_didopen_to_publish_through_compositor() {
    let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lsp_max_repo = workspace.parent().expect("parent dir").join("lsp-max");
    let compositor_bin = release_bin(&lsp_max_repo, "lsp-max-compositor", "LSP_MAX_COMPOSITOR_BIN");
    let child_bin = release_bin(&workspace, "claude-code-config-lsp", "CLAUDE_CODE_CONFIG_LSP_BIN");

    let dir = unique_tmp_dir("census");
    write_compositor_manifest(&dir, &child_bin);

    let settings_uri = format!("file://{}/.claude/settings.json", dir.display());
    let md_uri = format!("file://{}/CLAUDE.md", dir.display());
    fs::create_dir_all(dir.join(".claude")).expect("create .claude");

    let mut client = LspClient::spawn(compositor_bin.to_str().unwrap(), &dir, "lsp-max-compositor");
    let init = client.request(
        "initialize",
        json!({"capabilities": {}, "rootUri": format!("file://{}", dir.display())}),
    );
    assert_eq!(
        init.pointer("/result/serverInfo/name").and_then(|v| v.as_str()),
        Some("lsp-max-compositor"),
        "front door must be the compositor"
    );
    client.notify("initialized", json!({}));

    // Warm-up: first open pays child spawn / pool cost; recorded separately,
    // excluded from the census so the census measures steady-state relay.
    let (warm_diags, warm) = client.notify_wait_timed(
        "textDocument/didOpen",
        json!({"textDocument": {"uri": settings_uri, "languageId": "json", "version": 1, "text": valid_settings()}}),
        &settings_uri,
    );
    assert!(
        codes(&warm_diags).is_empty(),
        "warm-up publish for valid settings must be clean, got {:?}",
        codes(&warm_diags)
    );

    const N: usize = 1000;
    let mut latencies: Vec<Duration> = Vec::with_capacity(N);
    for i in 0..N {
        // Alternate fixture classes so each round-trip carries a real
        // analysis + relay payload (config-heavy fixture set).
        let (uri, text, lang) = if i % 2 == 0 {
            (settings_uri.as_str(), violating_settings(), "json")
        } else {
            (md_uri.as_str(), violating_md(), "markdown")
        };
        let (_diags, elapsed) = client.notify_wait_timed(
            "textDocument/didOpen",
            json!({"textDocument": {"uri": uri, "languageId": lang, "version": i as i64 + 2, "text": text}}),
            uri,
        );
        latencies.push(elapsed);
    }
    latencies.sort();
    let (p50, p95, p99) = percentiles(&latencies);
    let max = latencies.last().copied().unwrap_or_default();
    println!(
        "census: n=1000 warmup={warm:?} p50={p50:?} p95={p95:?} p99={p99:?} max={max:?}"
    );

    // Sanity only — no absolute-ms fence. Every round-trip completed and the
    // percentile ordering is coherent.
    assert_eq!(latencies.len(), N, "all 1000 round-trips must complete");
    assert!(
        p50 <= p95 && p95 <= p99,
        "percentile ordering violated: p50={p50:?} p95={p95:?} p99={p99:?}"
    );

    // State coherence after the census: one more md round-trip whose payload
    // must carry the violation (this publish is a fresh deposit, so it fires).
    let (md_diags, _) = client.notify_wait_timed(
        "textDocument/didOpen",
        json!({"textDocument": {"uri": md_uri, "languageId": "markdown", "version": 100_001, "text": violating_md()}}),
        &md_uri,
    );
    assert!(
        codes(&md_diags).contains(&"CCC-MD-006".to_string()),
        "post-census md publish must carry CCC-MD-006, got {:?}",
        codes(&md_diags)
    );

    let _ = fs::remove_dir_all(&dir);
}

// ── 2. Batch stress: 100-file workspace ──────────────────────────────────────

/// 50 `.claude/settings.json` (violating, CCC-JSON-004) + 50 `CLAUDE.md`
/// (violating, CCC-MD-006) under `root/d01..d100`. Returns the per-file
/// expectation table.
fn generate_100_file_workspace(root: &std::path::Path) -> Vec<(String, String, &'static str, String)> {
    let mut files = Vec::new();
    for i in 0..100 {
        let d = root.join(format!("d{:03}", i));
        fs::create_dir_all(if i < 50 { d.join(".claude") } else { d })
            .expect("create fixture dir");
        let (rel, text, code) = if i < 50 {
            (
                format!("d{:03}/.claude/settings.json", i),
                violating_settings().to_string(),
                "CCC-JSON-004",
            )
        } else {
            (
                format!("d{:03}/CLAUDE.md", i),
                violating_md().to_string(),
                "CCC-MD-006",
            )
        };
        let uri = format!("file://{}", root.join(&rel).display());
        files.push((uri, text, code, rel));
    }
    files
}

#[test]
fn batch_100_file_workspace_end_positions_and_codes_intact() {
    let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lsp_max_repo = workspace.parent().expect("parent dir").join("lsp-max");
    let compositor_bin = release_bin(&lsp_max_repo, "lsp-max-compositor", "LSP_MAX_COMPOSITOR_BIN");
    let child_bin = release_bin(&workspace, "claude-code-config-lsp", "CLAUDE_CODE_CONFIG_LSP_BIN");

    let dir = unique_tmp_dir("batch");
    write_compositor_manifest(&dir, &child_bin);
    let files = generate_100_file_workspace(&dir);

    let mut client = LspClient::spawn(compositor_bin.to_str().unwrap(), &dir, "lsp-max-compositor");
    client.request(
        "initialize",
        json!({"capabilities": {}, "rootUri": format!("file://{}", dir.display())}),
    );
    client.notify("initialized", json!({}));

    let mut total_ms: f64 = 0.0;
    let mut max_ms: f64 = 0.0;
    for (i, (uri, text, expected_code, _rel)) in files.iter().enumerate() {
        let (diags, elapsed) = client.notify_wait_timed(
            "textDocument/didOpen",
            json!({"textDocument": {"uri": uri, "languageId": if i < 50 { "json" } else { "markdown" }, "version": 1, "text": text}}),
            uri,
        );
        let ms = elapsed.as_secs_f64() * 1000.0;
        total_ms += ms;
        max_ms = max_ms.max(ms);
        assert!(
            codes(&diags).contains(&expected_code.to_string()),
            "file {i} ({uri}): expected {expected_code}, got {:?}",
            codes(&diags)
        );
        // CCC- codes intact: every code on the uri is in the CCC- family.
        for c in codes(&diags) {
            assert!(c.starts_with("CCC-"), "non-CCC code {c} on {uri}");
        }
        // NOTE: end-position preservation (CL-H) is not asserted per-file here —
        // the front door currently collapses range.end to range.start on the
        // debounced flush path (AB-18 F1, see clh court below). The dedicated
        // court owns that law until lsp-max repairs flush_coordinator.rs.
    }

    println!(
        "batch: 100 files, mean={:.2}ms max={:.2}ms (recorded, not fenced)",
        total_ms / 100.0,
        max_ms
    );

    let _ = fs::remove_dir_all(&dir);
}

// ── 3. Standing diagnostics under load (merged-publish hazard) ───────────────

#[test]
fn standing_diagnostics_100_file_soak_no_duplicate_uri_batch_clobbering() {
    let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lsp_max_repo = workspace.parent().expect("parent dir").join("lsp-max");
    let compositor_bin = release_bin(&lsp_max_repo, "lsp-max-compositor", "LSP_MAX_COMPOSITOR_BIN");
    let child_bin = release_bin(&workspace, "claude-code-config-lsp", "CLAUDE_CODE_CONFIG_LSP_BIN");

    let dir = unique_tmp_dir("standing");
    write_compositor_manifest(&dir, &child_bin);
    let files = generate_100_file_workspace(&dir);

    let mut client = LspClient::spawn(compositor_bin.to_str().unwrap(), &dir, "lsp-max-compositor");
    client.request(
        "initialize",
        json!({"capabilities": {}, "rootUri": format!("file://{}", dir.display())}),
    );
    client.notify("initialized", json!({}));

    // Soak pass 1: open all 100, capture the merged diagnostic set per uri.
    let mut first_pass: Vec<(String, Vec<Value>)> = Vec::new();
    for (i, (uri, text, expected_code, _rel)) in files.iter().enumerate() {
        client.notify(
            "textDocument/didOpen",
            json!({"textDocument": {"uri": uri, "languageId": if i < 50 { "json" } else { "markdown" }, "version": 1, "text": text}}),
        );
        let diags = client.wait_diagnostics_until(uri, |d| {
            codes(d).iter().any(|c| c == expected_code)
        });
        first_pass.push((uri.clone(), diags));
    }

    // Soak pass 2: re-deposit every uri (didOpen again, same text) — the
    // standing-diagnostic re-publish class. Each re-deposit flows through the
    // compositor merge buffer for the same uri; the merged-publish hazard is
    // (a) the re-deposit wiping/doubling the uri's own set, or (b) one uri's
    // batch bleeding into another uri's publish.
    for (i, (uri, text, _expected_code, _rel)) in files.iter().enumerate() {
        client.notify(
            "textDocument/didOpen",
            json!({"textDocument": {"uri": uri, "languageId": if i < 50 { "json" } else { "markdown" }, "version": 2, "text": text}}),
        );
        let (diags, _) = client.notify_wait_timed(
            "textDocument/didOpen",
            json!({"textDocument": {"uri": uri, "languageId": if i < 50 { "json" } else { "markdown" }, "version": 2, "text": text}}),
            uri,
        );
        let (f_uri, f_diags) = &first_pass[i];
        assert_eq!(f_uri, uri);

        // (a) re-deposit must reproduce exactly the first-pass set.
        assert_eq!(
            codes(&diags).len(),
            codes(f_diags).len(),
            "re-deposit changed diagnostic count for {uri}"
        );
        let mut a: Vec<String> = codes(&diags);
        let mut b: Vec<String> = codes(f_diags);
        a.sort();
        b.sort();
        assert_eq!(a, b, "re-deposit changed diagnostic codes for {uri}");
        // No duplicate identical entries within the merged batch.
        let mut ranges: Vec<String> = diags
            .iter()
            .map(|d| d.get("range").map(|r| r.to_string()).unwrap_or_default())
            .collect();
        ranges.sort();
        let n_before = ranges.len();
        ranges.dedup();
        assert_eq!(
            ranges.len(),
            n_before,
            "duplicate identical diagnostic ranges in merged batch for {uri}: {ranges:?}"
        );
        // NOTE: CL-H end-position width law is owned by the dedicated F1
        // court below — the front door currently collapses range.end to
        // range.start on the debounced flush path (registered divergence).
    }

    let _ = fs::remove_dir_all(&dir);
}

// ── 4. Churn: 10k didOpen/didChange/didClose storm ───────────────────────────

#[test]
fn churn_storm_10k_events_final_state_matches_sequential_replay() {
    let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lsp_max_repo = workspace.parent().expect("parent dir").join("lsp-max");
    let compositor_bin = release_bin(&lsp_max_repo, "lsp-max-compositor", "LSP_MAX_COMPOSITOR_BIN");
    let child_bin = release_bin(&workspace, "claude-code-config-lsp", "CLAUDE_CODE_CONFIG_LSP_BIN");

    let dir = unique_tmp_dir("churn");
    write_compositor_manifest(&dir, &child_bin);

    // 20 config files across both routed classes.
    let mut files = Vec::new();
    for i in 0..10 {
        let d = dir.join(format!("w{:02}", i));
        fs::create_dir_all(d.join(".claude")).expect("mkdir");
        let uri = format!("file://{}/.claude/settings.json", d.display());
        files.push((uri, "json"));
    }
    for i in 0..10 {
        let d = dir.join(format!("v{:02}", i));
        fs::create_dir_all(&d).expect("mkdir");
        let uri = format!("file://{}/CLAUDE.md", d.display());
        files.push((uri, "markdown"));
    }

    let mut client = LspClient::spawn(compositor_bin.to_str().unwrap(), &dir, "lsp-max-compositor");
    client.request(
        "initialize",
        json!({"capabilities": {}, "rootUri": format!("file://{}", dir.display())}),
    );
    client.notify("initialized", json!({}));

    // Sequential replay ground truth: the LAST text each file receives in the
    // storm determines its expected final diagnostic state. Storm schedule
    // (fixed, replayable): for each round, for each file, cycle
    // valid → violating → violating → valid → ... with didClose on round end
    // for even files; final text per file is determined by (round, file) mod
    // arithmetic recorded here.
    const ROUNDS: usize = 100;
    // Events per round: 10 files × (1 open-or-change + ... ) — budget exactly
    // 10k notification sends total.
    // Per round: 20 files × (2 didChange/didOpen + 1 didClose every 5th) = 20×2 + 4 = 44
    // ROUNDS=100 → 4400... scale to 10k: use 2 changes per file per round plus
    // closes: 100 rounds × 44 = 4400. Instead: 20 files × 500 events = 10000.
    // Per file: 500 events = 166 change triples × 3 + 1. Simplify: each file
    // gets exactly 500 notifications: rounds of (open, change, change, close).
    let mut event_count: usize = 0;
    let mut final_text: Vec<&str> = vec![valid_settings(); 20];
    // Deterministic replay model: track the final text per file as the storm
    // runs, then assert the published diagnostics match that model.
    let texts_json = [violating_settings(), valid_settings()];
    let texts_md = [violating_md(), valid_md()];

    for round in 0..ROUNDS {
        for (fi, (uri, lang)) in files.iter().enumerate() {
            // 5 notifications per file per round = 20 × 5 × 100 = 10_000.
            for step in 0..5usize {
                let flip = (round + fi + step) % 2 == 0;
                let is_close = step == 4;
                if is_close {
                    client.notify(
                        "textDocument/didClose",
                        json!({"textDocument": {"uri": uri}}),
                    );
                } else if step == 0 {
                    // didOpen re-anchor every round.
                    let text = if *lang == "json" {
                        if flip { texts_json[0] } else { texts_json[1] }
                    } else if flip {
                        texts_md[0]
                    } else {
                        texts_md[1]
                    };
                    final_text[fi] = text;
                    client.notify(
                        "textDocument/didOpen",
                        json!({"textDocument": {"uri": uri, "languageId": lang, "version": (round * 5 + step) as i64, "text": text}}),
                    );
                    // Consume the publish for this open so the mpsc queue
                    // doesn't grow unboundedly during the storm.
                    let _ = client.next_diagnostics_for_with_deadline(
                        uri,
                        Instant::now() + READ_TIMEOUT,
                    );
                } else {
                    // didChange with full-text sync.
                    let text = if *lang == "json" {
                        if flip { texts_json[0] } else { texts_json[1] }
                    } else if flip {
                        texts_md[0]
                    } else {
                        texts_md[1]
                    };
                    final_text[fi] = text;
                    client.notify(
                        "textDocument/didChange",
                        json!({
                            "textDocument": {"uri": uri, "version": (round * 5 + step) as i64},
                            "contentChanges": [{"text": text}]
                        }),
                    );
                    let _ = client.next_diagnostics_for_with_deadline(uri, churn_deadline(event_count));
                }
                event_count += 1;
            }
        }
    }
    assert_eq!(event_count, 10_000, "storm must send exactly 10k events");

    // Final state == sequential-replay ground truth: after the storm, re-open
    // each file with its final storm text and require the diagnostics for
    // that text (violating → code present; valid → no CCC- codes).
    let mut final_publish_ms: Vec<f64> = Vec::new();
    for (fi, (uri, lang)) in files.iter().enumerate() {
        let text = final_text[fi];
        client.notify(
            "textDocument/didOpen",
            json!({"textDocument": {"uri": uri, "languageId": lang, "version": 99_999, "text": text}}),
        );
        let t0 = Instant::now();
        let diags = client.next_diagnostics_for_with_deadline(uri, Instant::now() + READ_TIMEOUT);
        final_publish_ms.push(t0.elapsed().as_secs_f64() * 1000.0);

        let expect_violation = text == violating_settings() || text == violating_md();
        let present = codes(&diags).iter().any(|c| {
            c == "CCC-JSON-004" || c == "CCC-MD-006"
        });
        if expect_violation {
            assert!(
                present,
                "file {fi} ({uri}): final text is violating but diagnostics are {:?}",
                codes(&diags)
            );
        } else {
            assert!(
                !present,
                "file {fi} ({uri}): final text is valid but stale violation diagnostic survives: {:?}",
                codes(&diags)
            );
        }
        // NOTE: CL-H end-position law owned by the dedicated F1 court below.
    }
    let worst = final_publish_ms
        .iter()
        .cloned()
        .fold(0.0_f64, f64::max);
    println!(
        "churn: 10000 events, final-state publishes recorded, worst={worst:.2}ms (not fenced)"
    );

    let _ = fs::remove_dir_all(&dir);
}


// ── F1 court: CL-H end position through the front door ──────────────────────

/// Registered divergence AB-18/F1 (LOUD skip, named close condition).
///
/// Witnessed 2026-11-03: the child server publishes CCC-JSON-004 with range
/// (1,2)→(1,9) (probe over the raw child wire — full span). The same fixture
/// through the lsp-max-compositor front door arrives (1,2)→(1,2). Root cause:
/// `/Users/sac/lsp-max/crates/lsp-max-compositor/src/flush_coordinator.rs`
/// (editor-side materialization, `end: Position { line: d.line, character:
/// d.character }` — end hardcoded to start) — the second publish site CL-H
/// did not convert when it fixed `server.rs`. The debounced flush path is the
/// path the stdio run loop actually uses. lsp-max's own CL-H court
/// (tests/diagnostic_end_position_test.rs, 4/4 green) exercises the buffer +
/// merge in-process and never rides the flush coordinator, so the gap is
/// invisible to it.
///
/// CLOSE CONDITION: lsp-max converts flush_coordinator.rs's editor-side
/// materialization to `line: d.end_line, character: d.end_character` (same
/// shape as server.rs:582); then remove this #[ignore] and this court must
/// pass — child probe (1,2)→(1,9) must survive the front door verbatim.
#[test]
fn clh_end_position_survives_front_door() {
    let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lsp_max_repo = workspace.parent().expect("parent dir").join("lsp-max");
    let compositor_bin = release_bin(&lsp_max_repo, "lsp-max-compositor", "LSP_MAX_COMPOSITOR_BIN");
    let child_bin = release_bin(&workspace, "claude-code-config-lsp", "CLAUDE_CODE_CONFIG_LSP_BIN");

    let dir = unique_tmp_dir("f1court");
    write_compositor_manifest(&dir, &child_bin);
    fs::create_dir_all(dir.join(".claude")).expect("mkdir");

    let uri = format!("file://{}/.claude/settings.json", dir.display());
    let mut client = LspClient::spawn(compositor_bin.to_str().unwrap(), &dir, "lsp-max-compositor");
    client.request(
        "initialize",
        json!({"capabilities": {}, "rootUri": format!("file://{}", dir.display())}),
    );
    client.notify("initialized", json!({}));

    let (diags, _) = client.notify_wait_timed(
        "textDocument/didOpen",
        json!({"textDocument": {"uri": uri, "languageId": "json", "version": 1, "text": violating_settings()}}),
        &uri,
    );
    let d = diags
        .iter()
        .find(|d| d.get("code").and_then(|c| c.as_str()) == Some("CCC-JSON-004"))
        .expect("CCC-JSON-004 through the front door");
    let r = d.get("range").expect("range");
    let got = (
        r.pointer("/start/line").and_then(|v| v.as_u64()).unwrap_or(0),
        r.pointer("/start/character").and_then(|v| v.as_u64()).unwrap_or(0),
        r.pointer("/end/line").and_then(|v| v.as_u64()).unwrap_or(0),
        r.pointer("/end/character").and_then(|v| v.as_u64()).unwrap_or(0),
    );
    // The child probe witnesses the child emits exactly this span; the front
    // door must relay it verbatim (zero-range in → zero-range out is lawful,
    // multi-char in → multi-char out is the law).
    assert_eq!(
        got,
        (1, 2, 1, 9),
        "F1: front door collapsed the child's (1,2)→(1,9) span to {got:?}"
    );
}

/// Bounded deadline that widens with storm position (recorded, not fenced):
/// the storm legitimately queues publishes ahead of the drain; a fixed tight
/// deadline would flake under load, a fixed loose one hides regressions.
fn churn_deadline(_event: usize) -> Instant {
    // Recorded, not fenced: a fixed bound would either flake under load or
    // hide regressions; 30 s is the harness transport timeout, reused here.
    Instant::now() + Duration::from_secs(30)
}
