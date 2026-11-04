//! Chicago-TDD over the standing vocabulary: fixture receipts (green fresh,
//! green stale, red, absent) x diagnostic expectations, over real tempdir
//! files — no mocks.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;

use claude_code_config_lsp::standing::{
    latest_receipt_path, load_receipt, parse_receipt, standing_diagnostics, standing_for_surface,
    FleetReceipt, Standing, CODE_BLOCKED, CODE_PARTIAL, CODE_UNVERIFIED,
};

/// Unique per-test temp workspace under the OS temp dir (no tempfile dep).
fn temp_workspace(tag: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "ccc-lsp-standing-{}-{}-{}",
        tag,
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    std::fs::create_dir_all(&dir).expect("create temp workspace");
    dir
}

/// Write a fixture receipt and return its receipt struct with the mtime the
/// file actually got on disk.
fn write_receipt(dir: &Path, status: &str, commands: &[&str]) -> FleetReceipt {
    let path = latest_receipt_path(dir);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let json = format!(
        "{{\"status\":\"{}\",\"commands\":[{}]}}",
        status,
        commands
            .iter()
            .map(|c| format!("\"{c}\""))
            .collect::<Vec<_>>()
            .join(",")
    );
    std::fs::write(&path, json).unwrap();
    load_receipt(dir)
        .unwrap()
        .expect("receipt just written must load")
}

/// Touch a fake tracked source file, return its mtime.
fn write_source(dir: &Path, name: &str, when: Option<SystemTime>) -> SystemTime {
    let p = dir.join(name);
    std::fs::write(&p, "fn main() {}").unwrap();
    if let Some(_t) = when {
        let f = std::fs::File::options().append(true).open(&p).unwrap();
        f.set_modified(when.unwrap()).unwrap();
    }
    std::fs::metadata(&p).unwrap().modified().unwrap()
}

#[test]
fn green_fresh_clean_is_alive() {
    let dir = temp_workspace("alive");
    // Source first, receipt after: receipt mtime >= source mtime -> fresh.
    let src = write_source(&dir, "lib.rs", None);
    let receipt = write_receipt(&dir, "green", &["just gate"]);
    let s = standing_for_surface(&url(&dir), Some(&receipt), Some(src), 0);
    assert_eq!(s, Standing::ALIVE);
    // Never-round-up: ALIVE emits zero diagnostics.
    assert!(standing_diagnostics(&url(&dir), s, Some(&receipt)).is_empty());
}

#[test]
fn green_stale_is_blocked() {
    let dir = temp_workspace("stale");
    // Receipt first, then bump the source file's mtime to now — newer than
    // the receipt: receipt is stale.
    let receipt = write_receipt(&dir, "green", &["just gate"]);
    let src = write_source(&dir, "lib.rs", Some(SystemTime::now()));
    assert!(src > receipt.mtime().unwrap());
    let s = standing_for_surface(&url(&dir), Some(&receipt), Some(src), 0);
    assert_eq!(s, Standing::BLOCKED);
    let diags = standing_diagnostics(&url(&dir), s, Some(&receipt));
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, Some(lsp_warning()));
    assert_eq!(code(&diags[0]), CODE_BLOCKED);
    // Remediation carries the receipt's own command list.
    assert!(
        diags[0].message.contains("just gate"),
        "message: {}",
        diags[0].message
    );
    assert!(diags[0].message.contains("stale"));
}

#[test]
fn red_receipt_is_blocked() {
    let dir = temp_workspace("red");
    let src = write_source(&dir, "lib.rs", None);
    let receipt = write_receipt(&dir, "red", &["just gate"]);
    let s = standing_for_surface(&url(&dir), Some(&receipt), Some(src), 0);
    assert_eq!(s, Standing::BLOCKED);
    assert_eq!(
        code(&standing_diagnostics(&url(&dir), s, Some(&receipt))[0]),
        CODE_BLOCKED
    );
}

#[test]
fn absent_receipt_is_unverified() {
    let dir = temp_workspace("unverified");
    let src = write_source(&dir, "lib.rs", None);
    let s = standing_for_surface(&url(&dir), None, Some(src), 0);
    assert_eq!(s, Standing::UNVERIFIED);
    let diags = standing_diagnostics(&url(&dir), s, None);
    assert_eq!(code(&diags[0]), CODE_UNVERIFIED);
    assert!(diags[0].message.contains("never scanned"));
}

#[test]
fn findings_on_green_fresh_is_partial() {
    let dir = temp_workspace("partial");
    let src = write_source(&dir, "lib.rs", None);
    let receipt = write_receipt(&dir, "green", &["just gate"]);
    let s = standing_for_surface(&url(&dir), Some(&receipt), Some(src), 3);
    assert_eq!(s, Standing::PARTIAL);
    let diags = standing_diagnostics(&url(&dir), s, Some(&receipt));
    assert_eq!(code(&diags[0]), CODE_PARTIAL);
    assert_eq!(diags[0].severity, Some(lsp_warning()));
}

#[test]
fn parsed_without_disk_witness_is_never_fresh() {
    // Never-round-up: a receipt parsed from text alone carries no mtime
    // witness, so freshness cannot be claimed and standing rounds down.
    let receipt = parse_receipt(r#"{"status":"green","commands":["just gate"]}"#).unwrap();
    assert_eq!(receipt.mtime(), None);
    assert!(!receipt.is_fresh(Some(SystemTime::now())));
    let src = SystemTime::now();
    let s = standing_for_surface(
        &url(&temp_workspace("nowitness")),
        Some(&receipt),
        Some(src),
        0,
    );
    assert_eq!(s, Standing::BLOCKED);
}

#[test]
fn load_receipt_absent_is_ok_none() {
    let dir = temp_workspace("absent");
    assert!(load_receipt(&dir).unwrap().is_none());
}

#[test]
fn malformed_receipt_is_typed_error() {
    let dir = temp_workspace("malformed");
    let path = latest_receipt_path(&dir);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "{not json").unwrap();
    assert!(load_receipt(&dir).is_err());
}

#[test]
fn non_green_status_is_not_green() {
    let r: FleetReceipt = serde_json::from_str(r#"{"status":"YELLOW"}"#).unwrap();
    assert!(!r.is_green());
    let r2: FleetReceipt = serde_json::from_str(r#"{"status":"green "}"#).unwrap();
    assert!(!r2.is_green());
}

#[test]
fn blocked_without_commands_still_names_the_receipt() {
    let dir = temp_workspace("nocmds");
    let src = write_source(&dir, "lib.rs", None);
    let receipt = write_receipt(&dir, "red", &[]);
    let s = standing_for_surface(&url(&dir), Some(&receipt), Some(src), 0);
    let diags = standing_diagnostics(&url(&dir), s, Some(&receipt));
    assert!(diags[0].message.contains("latest.json"));
}

// -- helpers ---------------------------------------------------------------

fn url(dir: &Path) -> lsp_types_max::Uri {
    format!("file://{}/settings.json", dir.display())
        .parse()
        .unwrap()
}

fn lsp_warning() -> lsp_types_max::DiagnosticSeverity {
    lsp_types_max::DiagnosticSeverity::WARNING
}

fn code(d: &lsp_types_max::Diagnostic) -> &str {
    match d.code.as_ref().unwrap() {
        lsp_types_max::NumberOrString::String(s) => s,
        _ => panic!("expected string code"),
    }
}

// -- H4 wiring courts (AF-23): standing merged into the publish batch ------
// The merge these courts exercise is the exact composition
// `ClaudeCodeConfigBackend::scan_uri_classified` performs per did_open /
// did_change: analyzer findings via `scan::analyze_document`, then the
// standing layer over `provider_roots()[0]`'s fleet receipt.

use claude_code_config_lsp::backend::standing_layer_for_root;
use claude_code_config_lsp::scan::analyze_document;
use lsp_types_max::{DiagnosticSeverity as LspSeverity, NumberOrString as LspNumStr};

/// The analyzer findings for a settings.json surface (real analyzers), scoped
/// the way the backend scopes them.
fn analyzer_findings(uri: &str, content: &str) -> Vec<lsp_types_max::Diagnostic> {
    analyze_document(uri, content)
        .into_iter()
        .map(|raw| lsp_types_max::Diagnostic {
            code: Some(LspNumStr::String(raw.code)),
            message: raw.message,
            ..Default::default()
        })
    .collect()
}

/// Combined publish batch: analyzers + standing, the scan_uri_classified merge.
fn merged_batch(root: &Path, uri_str: &str, content: &str, surface: &lsp_types_max::Uri) -> Vec<lsp_types_max::Diagnostic> {
    let mut batch = analyzer_findings(uri_str, content);
    let count = batch.len();
    batch.extend(
        standing_layer_for_root(root, surface, count)
            .into_iter()
            .map(|(_max, diag)| diag),
    );
    batch
}

#[test]
fn receiptless_workspace_emits_unverified_merged_with_analyzer_findings() {
    let dir = temp_workspace("h4-unverified");
    let surface: lsp_types_max::Uri =
        format!("file://{}/.claude/settings.json", dir.display()).parse().unwrap();
    let uri_str = surface.as_str();
    // Real analyzer finding: bare-string skillOverrides fires CCC-JSON-009.
    let content = r#"{"skillOverrides": "off"}"#;
    let batch = merged_batch(&dir, uri_str, content, &surface);
    let codes: Vec<&str> = batch
        .iter()
        .filter_map(|d| match &d.code {
            Some(LspNumStr::String(s)) => Some(s.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        codes.iter().any(|c| c.starts_with("CCC-JSON")),
        "analyzer finding missing from merged batch: {codes:?}"
    );
    assert!(
        codes.contains(&CODE_UNVERIFIED),
        "standing-unverified missing from merged batch: {codes:?}"
    );
    // source tag on the standing diag, per the G3 design.
    let standing = batch
        .iter()
        .find(|d| d.source.as_deref() == Some("standing"))
        .unwrap();
    assert_eq!(code(standing), CODE_UNVERIFIED);
}

#[test]
fn alive_receipt_emits_no_standing_diagnostic() {
    let dir = temp_workspace("h4-alive");
    write_receipt(&dir, "green", &["just gate"]);
    let surface: lsp_types_max::Uri =
        format!("file://{}/.claude/settings.json", dir.display()).parse().unwrap();
    // Clean content: zero analyzer findings -> green+fresh+clean = ALIVE.
    let batch = merged_batch(&dir, surface.as_str(), "{}", &surface);
    assert!(
        batch.iter().all(|d| d.source.as_deref() != Some("standing")),
        "ALIVE must emit no standing diagnostic: {:?}",
        batch
    );
}

#[test]
fn green_receipt_with_findings_emits_standing_partial() {
    let dir = temp_workspace("h4-partial");
    write_receipt(&dir, "green", &["just gate"]);
    let surface: lsp_types_max::Uri =
        format!("file://{}/.claude/settings.json", dir.display()).parse().unwrap();
    let content = r#"{"skillOverrides": "off"}"#;
    let batch = merged_batch(&dir, surface.as_str(), content, &surface);
    let standing = batch
        .iter()
        .find(|d| d.source.as_deref() == Some("standing"))
        .expect("PARTIAL standing diagnostic must be present");
    assert_eq!(code(standing), CODE_PARTIAL);
}

#[test]
fn standing_severities_are_never_error() {
    // Across all three non-ALIVE standings reachable through the wiring layer.
    for tag in ["h4-sev-unverified", "h4-sev-blocked", "h4-sev-partial"] {
        let dir = temp_workspace(tag);
        let surface: lsp_types_max::Uri =
            format!("file://{}/.claude/settings.json", dir.display()).parse().unwrap();
        if tag.ends_with("blocked") || tag.ends_with("partial") {
            write_receipt(&dir, "red", &["just gate"]);
        }
        let content = if tag.ends_with("partial") {
            r#"{"skillOverrides": "off"}"#
        } else {
            "{}"
        };
        let batch = merged_batch(&dir, surface.as_str(), content, &surface);
        for d in batch.iter().filter(|d| d.source.as_deref() == Some("standing")) {
            assert_ne!(
                d.severity,
                Some(LspSeverity::ERROR),
                "standing diagnostic carried ERROR severity: {}",
                code(d)
            );
        }
    }
}

#[test]
fn unclassified_surface_emits_no_standing_diagnostic() {
    // Standing is a property of config surfaces; classify() == "unknown"
    // (README, src/main.rs) must not carry a standing diagnostic even in a
    // receiptless workspace — the gate lives in standing_layer_for_root.
    assert_eq!(claude_code_config_lsp::scan::classify("/x/README.md"), "unknown");
    let dir = temp_workspace("h4-unclassified");
    let surface: lsp_types_max::Uri =
        format!("file://{}/README.md", dir.display()).parse().unwrap();
    assert!(standing_layer_for_root(&dir, &surface, 0).is_empty());
}
