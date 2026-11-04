//! G2 dispatch tests: the commands analyzer routed through the PUBLIC scan API
//! (`scan::classify` + `scan::analyze_document`) — the exact dispatch the LSP
//! backend runs per document. Chicago-style: real fixture files on disk, real
//! filesystem base_dir resolution, no mocks.

use claude_code_config_lsp::scan::{analyze_document, classify};
use std::path::Path;

fn fixture_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/commands")
}

fn read(name: &str) -> String {
    std::fs::read_to_string(fixture_dir().join(name)).expect("fixture readable")
}

// ── classify ─────────────────────────────────────────────────────────────────

#[test]
fn classify_routes_commands_dir_markdown() {
    assert_eq!(classify("/proj/.claude/commands/specify.md"), "commands");
    assert_eq!(classify("file:///proj/.claude/commands/plan.md"), "commands");
    // Plugin-shaped commands dir too.
    assert_eq!(
        classify("/proj/.claude/plugins/myplugin/commands/foo.md"),
        "commands"
    );
}

#[test]
fn classify_commands_wins_over_doc_surfaces_by_directory() {
    // Precedence note: /commands/ + .md wins BEFORE claude_md/agent — the
    // directory, not the filename, is the ontology. A command file literally
    // named claude.md (or an agent-shaped path inside commands/) still routes
    // to "commands".
    assert_eq!(classify("/proj/.claude/commands/CLAUDE.md"), "commands");
    assert_eq!(classify("/proj/.claude/commands/sub/worker.md"), "commands");
    // ...while the same shapes OUTSIDE commands/ keep their own routes.
    assert_eq!(classify("/proj/CLAUDE.md"), "claude_md");
    assert_eq!(classify("/proj/.claude/agents/worker.md"), "agent");
}

#[test]
fn classify_non_md_in_commands_is_not_commands() {
    // The rule is /commands/ AND .md — a stray file in the dir isn't routed.
    assert_eq!(classify("/proj/.claude/commands/notes.txt"), "unknown");
}

// ── dispatch ─────────────────────────────────────────────────────────────────

#[test]
fn missing_description_fixture_fires_cmd_001_through_scan() {
    // missing-description.md has frontmatter but no description surface →
    // CCC-CMD-001 through the public dispatch, category "commands".
    let findings = analyze_document(
        &fixture_dir().join("missing-description.md").display().to_string(),
        &read("missing-description.md"),
    );
    assert!(
        findings.iter().any(|f| f.code == "CCC-CMD-001"),
        "CCC-CMD-001 must fire via analyze_document: {findings:?}"
    );
    assert!(
        findings.iter().all(|f| f.category == "commands"),
        "category must be commands: {findings:?}"
    );
}

#[test]
fn cmd_002_fires_through_scan_with_real_base_dir() {
    let path = fixture_dir().join("unbalanced-arguments.md");
    let findings = analyze_document(&path.display().to_string(), &read("unbalanced-arguments.md"));
    assert!(
        findings.iter().any(|f| f.code == "CCC-CMD-002"),
        "{findings:?}"
    );
    assert!(findings.iter().all(|f| f.category == "commands"));
}

#[test]
fn cmd_004_fires_through_scan() {
    let path = fixture_dir().join("unresolvable-atref.md");
    let findings = analyze_document(&path.display().to_string(), &read("unresolvable-atref.md"));
    assert!(
        findings.iter().any(|f| f.code == "CCC-CMD-004"),
        "unclosed span / bad tool grammar family intact: {findings:?}"
    );
}

#[test]
fn cmd_006_fires_through_scan() {
    let path = fixture_dir().join("unclosed-shell-span.md");
    let findings = analyze_document(&path.display().to_string(), &read("unclosed-shell-span.md"));
    assert!(
        findings.iter().any(|f| f.code == "CCC-CMD-006"),
        "{findings:?}"
    );
}

#[test]
fn valid_fixture_is_clean_through_scan() {
    let path = fixture_dir().join("valid.md");
    let findings = analyze_document(&path.display().to_string(), &read("valid.md"));
    assert_eq!(findings, vec![], "valid.md must be clean: {findings:?}");
}

#[test]
fn real_base_dir_enables_cmd_003_resolution() {
    // Unresolvable @ref fires CCC-CMD-003 when the parent dir is REAL.
    let path = fixture_dir().join("unresolvable-atref.md");
    let findings = analyze_document(&path.display().to_string(), &read("unresolvable-atref.md"));
    let hits: Vec<_> = findings
        .iter()
        .filter(|f| f.code == "CCC-CMD-003")
        .collect();
    assert_eq!(hits.len(), 1, "{findings:?}");
    assert!(
        hits[0].message.contains("does-not-exist-anywhere.md"),
        "{findings:?}"
    );
}

#[test]
fn nonexistent_base_dir_suppresses_cmd_003() {
    // Same content under a synthetic URI whose parent does NOT exist on disk:
    // base_dir is None → no CCC-CMD-003 false-fire (and the other family codes
    // still fire, proving the analyzer ran).
    let content = read("unresolvable-atref.md");
    let findings = analyze_document("/synthetic/.claude/commands/atref.md", &content);
    assert!(
        findings.iter().all(|f| f.code != "CCC-CMD-003"),
        "no CCC-CMD-003 without a real base dir: {findings:?}"
    );
    assert!(
        findings.iter().any(|f| f.code == "CCC-CMD-004"),
        "other rules still fire: {findings:?}"
    );
}

#[test]
fn virtual_uri_suppresses_cmd_003() {
    // A claude-config:// virtual URI must not produce a base_dir at all —
    // its "path" is not a filesystem location.
    let content = read("unresolvable-atref.md");
    let findings = analyze_document("claude-config://commands/atref.md", &content);
    assert!(
        findings.iter().all(|f| f.code != "CCC-CMD-003"),
        "virtual URI must not false-fire CCC-CMD-003: {findings:?}"
    );
    assert!(findings.iter().all(|f| f.category == "commands"));
}

#[test]
fn file_uri_resolves_base_dir_after_scheme_strip() {
    // file:// URI form: base_dir resolution must work after stripping the
    // scheme (this is the shape the LSP backend actually delivers).
    let path = fixture_dir().join("unresolvable-atref.md");
    let uri = format!("file://{}", path.display());
    let findings = analyze_document(&uri, &read("unresolvable-atref.md"));
    assert!(
        findings.iter().any(|f| f.code == "CCC-CMD-003"),
        "file:// URI must still resolve the real parent dir: {findings:?}"
    );
}
