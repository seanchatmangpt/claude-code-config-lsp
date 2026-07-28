//! stdout carries LSP frames and nothing else.
//!
//! Regression guard for a defect that silently disabled the whole server for
//! strict clients: `tracing`'s default subscriber writes to stdout, so a single
//! startup INFO line (`lsp_max::service::watchdog` announcing the parent PID)
//! was emitted *ahead of* the first `Content-Length:` header. A client that
//! parses the header strictly fails to find one, reads no diagnostics, and
//! reports zero problems -- indistinguishable from a clean bill of health.
//!
//! This asserts the first bytes on stdout are a frame header. It is deliberately
//! a process-level test: the bug lives in global subscriber setup, so nothing
//! short of spawning the real binary can observe it.

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

fn frame(body: &str) -> Vec<u8> {
    format!("Content-Length: {}\r\n\r\n{}", body.len(), body).into_bytes()
}

#[test]
fn stdout_begins_with_an_lsp_frame_header() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_claude-code-config-lsp"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the LSP binary");

    // A *real* `processId` is load-bearing, not incidental: the offending log
    // line is the watchdog announcing the parent PID, and it is only emitted
    // when the client supplies one. With `processId: null` the watchdog stays
    // quiet, stdout is clean either way, and this test passes even against the
    // unfixed binary -- a guard that cannot fail.
    let init = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"initialize","params":{{"processId":{},"rootUri":null,"capabilities":{{}}}}}}"#,
        std::process::id()
    );
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(&frame(&init))
        .expect("write initialize");

    // Give the server time to emit the response (and any stray startup log).
    std::thread::sleep(Duration::from_secs(3));
    let _ = child.kill();

    let mut out = Vec::new();
    child
        .stdout
        .take()
        .expect("stdout")
        .read_to_end(&mut out)
        .expect("read stdout");

    assert!(
        !out.is_empty(),
        "server produced no stdout at all; it did not answer `initialize`"
    );
    assert!(
        out.starts_with(b"Content-Length:"),
        "stdout must open with an LSP frame header, but began with: {:?}\n\
         Something is writing non-protocol bytes to stdout -- most likely a \
         `tracing` subscriber left on its stdout default.",
        String::from_utf8_lossy(&out[..out.len().min(120)])
    );
}
