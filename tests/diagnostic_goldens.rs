//! Golden tests: one per ontology DiagnosticRule (schema/claude-code-config.ttl),
//! driven through the PUBLIC scan API (`scan::analyze_document`) — the exact
//! dispatch the LSP backend runs per document.
//!
//! Each golden pins CURRENT analyzer behavior for a fixture in tests/goldens/:
//! exact finding code(s), message prefix, and span when the span is real.
//! Findings carrying a (0,0) span are marked `// SPAN-PENDING: see c2 lane` —
//! when the span lane lands real spans, update the pinned assertion.
//! Rules with no implementation in any analyzer get a GOLDEN-GAP golden that
//! asserts the code is never emitted (gaps are deliverables, not failures).
//!
//! Contract note: CCC-XFILE-001 (cross-file rule) is evaluated by lsp-max's
//! WorkspaceRuleEvaluator over OPEN documents inside the LSP backend
//! (src/backend.rs `build_cross_file_rules`) and is NOT reachable through the
//! pure per-document `analyze_document` API, so it has no golden here.

use claude_code_config_lsp::scan::analyze_document;
use std::path::Path;

/// Load a fixture from tests/goldens/ and run it through the public scan API
/// under the canonical URI it would have in a real workspace.
fn golden(fixture: &str, uri: &str) -> Vec<claude_code_config_lsp::scan::ScanFinding> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/goldens")
        .join(fixture);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {fixture} unreadable: {e}"));
    analyze_document(uri, &content)
}

fn with_code<'a>(
    findings: &'a [claude_code_config_lsp::scan::ScanFinding],
    code: &str,
) -> Vec<&'a claude_code_config_lsp::scan::ScanFinding> {
    findings.iter().filter(|f| f.code == code).collect()
}

// ── CCC-JSON-001: deprecated settings.json key ───────────────────────────────
#[test]
fn ccc_json_001_deprecated_key_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/settings-json-ccc-json-001.json"),
    )
    .unwrap();
    let findings = golden(
        "settings-json-ccc-json-001.json",
        "/proj/.claude/settings.json",
    );
    let hits = with_code(&findings, "CCC-JSON-001");
    assert_eq!(hits.len(), 1, "exactly one CCC-JSON-001, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "json");
    assert!(
        f.message
            .starts_with("Deprecated settings.json key — use mcpServers"),
        "message: {}",
        f.message
    );
    // Real span: the literal key substring in the fixture.
    let s = content.find("enabledMcpjsonServers").unwrap();
    assert_eq!(
        f.span,
        (s, s + "enabledMcpjsonServers".len()),
        "SPAN OK: real span"
    );
}

// ── CCC-JSON-002: wrong hook key casing ──────────────────────────────────────
#[test]
fn ccc_json_002_wrong_hook_key_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/settings-json-ccc-json-002.json"),
    )
    .unwrap();
    let findings = golden(
        "settings-json-ccc-json-002.json",
        "/proj/.claude/settings.json",
    );
    let hits = with_code(&findings, "CCC-JSON-002");
    assert_eq!(hits.len(), 1, "exactly one CCC-JSON-002, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "json");
    assert!(
        f.message
            .starts_with("Wrong hook key format — use hooks.PreToolUse / hooks.PostToolUse"),
        "message: {}",
        f.message
    );
    let s = content.find("hookEvents").unwrap();
    assert_eq!(f.span, (s, s + "hookEvents".len()), "SPAN OK: real span");
}

// ── CCC-JSON-003: generic invalid-enum catch-all (settings.json) ─────────────
#[test]
fn ccc_json_003_generic_invalid_enum_golden() {
    // Real violation fixture: two enum fields without a field-specific code —
    // editorMode: "emacs" and statusLine.type: "script" — both must fire
    // CCC-JSON-003 with real spans (full quoted key tokens, from C2's
    // ts_key_span helpers).
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/settings-json-ccc-json-003.json"),
    )
    .unwrap();
    let findings = golden(
        "settings-json-ccc-json-003.json",
        "/proj/.claude/settings.json",
    );
    let hits = with_code(&findings, "CCC-JSON-003");
    assert_eq!(
        hits.len(),
        2,
        "exactly two CCC-JSON-003 findings, got {findings:?}"
    );
    let f0 = hits[0];
    assert_eq!(f0.category, "json");
    assert!(
        f0.message
            .starts_with("Invalid enum value for field 'editorMode': 'emacs'"),
        "message: {}",
        f0.message
    );
    let f1 = hits[1];
    assert!(
        f1.message
            .starts_with("Invalid enum value for field 'statusLine.type': 'script'"),
        "message: {}",
        f1.message
    );
    // Real spans: the full quoted key tokens, in document order.
    let s0 = content.find("\"editorMode\"").unwrap();
    assert_eq!(
        f0.span,
        (s0, s0 + "\"editorMode\"".len()),
        "SPAN OK: real span"
    );
    let s1 = content.find("\"type\"").unwrap();
    assert_eq!(f1.span, (s1, s1 + "\"type\"".len()), "SPAN OK: real span");
    // No double-fire with field-specific codes (model is not in the fixture's
    // enum-catchall scope; a field-specific field still maps to its own code):
    // a model violation fires CCC-JSON-004, never CCC-JSON-003.
    let model_findings = analyze_document("/proj/.claude/settings.json", "{\"model\": \"gpt-4\"}");
    assert!(
        !with_code(&model_findings, "CCC-JSON-004").is_empty()
            && with_code(&model_findings, "CCC-JSON-003").is_empty(),
        "field-specific code must win for model: {model_findings:?}"
    );
    // Clean-fixture negative (non-vacuity, per the C11 analog discipline): a
    // fully valid settings.json emits no CCC-JSON-003 (and no findings at all).
    let clean = golden("settings-json-clean.json", "/proj/.claude/settings.json");
    assert!(
        clean.is_empty(),
        "clean settings fixture must emit zero findings: {clean:?}"
    );
}

// ── CCC-JSON-005: invalid effortLevel ────────────────────────────────────────
#[test]
fn ccc_json_005_invalid_effort_level_golden() {
    let findings = golden(
        "settings-json-ccc-json-005.json",
        "/proj/.claude/settings.json",
    );
    let hits = with_code(&findings, "CCC-JSON-005");
    assert_eq!(hits.len(), 1, "exactly one CCC-JSON-005, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "json");
    assert!(
        f.message.starts_with("Invalid effortLevel 'extreme'"),
        "message: {}",
        f.message
    );
    // Real span: the full quoted key token `"effortLevel"` (bytes 1..14).
    assert_eq!(f.span, (1, 14), "SPAN OK: real span");
}

// ── CCC-JSON-007: plugin.json missing $schema ────────────────────────────────
#[test]
fn ccc_json_007_missing_schema_golden() {
    let findings = golden(
        "plugin-json-ccc-json-007.json",
        "/proj/.claude-plugin/plugin.json",
    );
    let hits = with_code(&findings, "CCC-JSON-007");
    assert_eq!(hits.len(), 1, "exactly one CCC-JSON-007, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "json");
    assert!(
        f.message
            .starts_with("plugin.json missing required '$schema' field"),
        "message: {}",
        f.message
    );
    // Real span: the whole document (0..trimmed len) — the missing field has
    // no token to span, so the analyzer spans the document body minus the
    // trailing newline (fixture is 22 bytes, observed span end 21).
    assert_eq!(f.span, (0, 21), "SPAN OK: real span");
}

// ── CCC-JSON-009: invalid skillOverrides value ───────────────────────────────
#[test]
fn ccc_json_009_invalid_skill_overrides_golden() {
    let findings = golden(
        "settings-json-ccc-json-009.json",
        "/proj/.claude/settings.json",
    );
    let hits = with_code(&findings, "CCC-JSON-009");
    assert_eq!(hits.len(), 1, "exactly one CCC-JSON-009, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "json");
    assert!(
        f.message.starts_with("skillOverrides must be a record"),
        "message: {}",
        f.message
    );
    // Real span: the full quoted key token `"skillOverrides"` (bytes 1..17).
    assert_eq!(f.span, (1, 17), "SPAN OK: real span");
}

// ── CCC-MD-001: agent frontmatter missing the `name` field ───────────────────
#[test]
fn ccc_md_001_missing_name_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/agent-md-ccc-md-001.md"),
    )
    .unwrap();
    let findings = golden("agent-md-ccc-md-001.md", "/proj/.claude/agents/worker.md");
    let hits = with_code(&findings, "CCC-MD-001");
    assert_eq!(hits.len(), 1, "exactly one CCC-MD-001, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "md");
    assert!(
        f.message
            .starts_with("Missing required name field in front-matter"),
        "message: {}",
        f.message
    );
    // Real span: the frontmatter block between the --- markers (starts at the
    // newline after the opening marker, byte 3).
    let fm_body = content.split("---").nth(1).unwrap();
    assert_eq!(f.span, (3, 3 + fm_body.len()), "SPAN OK: real span");
    // Non-vacuity: the frontmatter-family analog still fires for the condition.
    assert!(
        !with_code(&findings, "CCC-AGENT-001").is_empty(),
        "expected the analog CCC-AGENT-001 to still fire"
    );
    // Clean negative: a complete agent fixture emits no md-family findings.
    let clean = golden("agent-md-clean.md", "/proj/.claude/agents/clean.md");
    assert!(
        with_code(&clean, "CCC-MD-001").is_empty()
            && with_code(&clean, "CCC-MD-002").is_empty()
            && with_code(&clean, "CCC-MD-003").is_empty(),
        "clean agent fixture must emit no CCC-MD-* findings: {clean:?}"
    );
}

// ── CCC-MD-002: agent frontmatter missing the `description` field ────────────
#[test]
fn ccc_md_002_missing_description_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/agent-md-ccc-md-002.md"),
    )
    .unwrap();
    let findings = golden("agent-md-ccc-md-002.md", "/proj/.claude/agents/worker.md");
    let hits = with_code(&findings, "CCC-MD-002");
    assert_eq!(hits.len(), 1, "exactly one CCC-MD-002, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "md");
    assert!(
        f.message
            .starts_with("Missing required description field in front-matter"),
        "message: {}",
        f.message
    );
    // Real span: the frontmatter block between the --- markers (byte 3 onward).
    let fm_body = content.split("---").nth(1).unwrap();
    assert_eq!(f.span, (3, 3 + fm_body.len()), "SPAN OK: real span");
    // Non-vacuity: analog intact.
    assert!(
        !with_code(&findings, "CCC-AGENT-005").is_empty(),
        "expected the analog CCC-AGENT-005 to still fire"
    );
}

// ── CCC-MD-003: unknown tool in the agent tools array ────────────────────────
#[test]
fn ccc_md_003_unknown_tool_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/agent-md-ccc-md-003.md"),
    )
    .unwrap();
    let findings = golden("agent-md-ccc-md-003.md", "/proj/.claude/agents/worker.md");
    let hits = with_code(&findings, "CCC-MD-003");
    assert_eq!(hits.len(), 1, "exactly one CCC-MD-003, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "md");
    assert!(
        f.message
            .starts_with("Unknown tool 'SuperTool' in tools array"),
        "message: {}",
        f.message
    );
    // Real span: the unknown tool literal token inside the tools list.
    let s = content.find("SuperTool").unwrap();
    assert_eq!(f.span, (s, s + "SuperTool".len()), "SPAN OK: real span");
    // Non-vacuity: analog intact.
    assert!(
        !with_code(&findings, "CCC-AGENT-006").is_empty(),
        "expected the analog CCC-AGENT-006 to still fire"
    );
}

// ── CCC-MD-005: non-boolean user-invocable in SKILL.md frontmatter ───────────
#[test]
fn ccc_md_005_user_invocable_not_boolean_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/skill-md-ccc-md-005.md"),
    )
    .unwrap();
    let findings = golden(
        "skill-md-ccc-md-005.md",
        "/proj/.claude/skills/my-skill/SKILL.md",
    );
    let hits = with_code(&findings, "CCC-MD-005");
    assert_eq!(hits.len(), 1, "exactly one CCC-MD-005, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "skill");
    assert!(
        f.message
            .starts_with("user-invocable field must be boolean"),
        "message: {}",
        f.message
    );
    // Real span: the offending value token `yes` in the frontmatter.
    let s = content.find("yes").unwrap();
    assert_eq!(f.span, (s, s + 3), "SPAN OK: real span");
    // Non-vacuity: analog intact.
    assert!(
        !with_code(&findings, "CCC-SKILL-006").is_empty(),
        "expected the analog CCC-SKILL-006 to still fire"
    );
    // Clean negative: a complete skill fixture emits no CCC-MD-005 / CCC-SKILL-006.
    let clean = golden("skill-md-clean.md", "/proj/.claude/skills/clean/SKILL.md");
    assert!(
        with_code(&clean, "CCC-MD-005").is_empty() && with_code(&clean, "CCC-SKILL-006").is_empty(),
        "clean skill fixture must emit no boolean-findings: {clean:?}"
    );
}

// ── CCC-MD-006: CLAUDE.md missing H1 title ───────────────────────────────────
#[test]
fn ccc_md_006_missing_h1_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/claude-md-ccc-md-006.md"),
    )
    .unwrap();
    let findings = golden("claude-md-ccc-md-006.md", "/proj/CLAUDE.md");
    let hits = with_code(&findings, "CCC-MD-006");
    assert_eq!(hits.len(), 1, "exactly one CCC-MD-006, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "claude_md");
    assert!(
        f.message
            .starts_with("CLAUDE.md should start with an H1 title"),
        "message: {}",
        f.message
    );
    // Real span: (0, len of the offending first non-empty line).
    let first_line_len = content.lines().next().unwrap().len();
    assert_eq!(f.span, (0, first_line_len), "SPAN OK: real span");
}

// ── CCC-TOML-001: SemVer version where CalVer is required ────────────────────
#[test]
fn ccc_toml_001_semver_version_golden() {
    let findings = golden(
        "lsp-max-auto-ccc-toml-001.toml",
        "/proj/.claude/lsp-max-auto.toml",
    );
    let hits = with_code(&findings, "CCC-TOML-001");
    assert_eq!(hits.len(), 1, "exactly one CCC-TOML-001, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "toml");
    assert!(
        f.message
            .starts_with("SemVer version detected — this workspace uses CalVer (YY.M.D)"),
        "message: {}",
        f.message
    );
    // Real span: the literal pattern substring `version = "1.` (13 bytes).
    assert_eq!(f.span, (0, 13), "SPAN OK: real span");
}

// ── CCC-TOML-002: outdated Rust edition ──────────────────────────────────────
#[test]
fn ccc_toml_002_outdated_edition_golden() {
    let findings = golden(
        "lsp-max-auto-ccc-toml-002.toml",
        "/proj/.claude/lsp-max-auto.toml",
    );
    let hits = with_code(&findings, "CCC-TOML-002");
    assert_eq!(hits.len(), 1, "exactly one CCC-TOML-002, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "toml");
    assert!(
        f.message
            .starts_with("Outdated Rust edition — use edition = \"2021\""),
        "message: {}",
        f.message
    );
    // Real span: the literal pattern substring `edition = "2015"` (16 bytes).
    assert_eq!(f.span, (0, 16), "SPAN OK: real span");
}

// ── CCC-HOOK-001: misspelled hook event (hook-script surface) ────────────────
#[test]
fn ccc_hook_001_misspelled_event_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/hooks-ccc-hook-001.sh"),
    )
    .unwrap();
    let findings = golden("hooks-ccc-hook-001.sh", "/proj/hooks/session-start.sh");
    let hits = with_code(&findings, "CCC-HOOK-001");
    assert_eq!(hits.len(), 1, "exactly one CCC-HOOK-001, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "hook");
    assert!(
        f.message
            .starts_with("Misspelled hook event: use PreToolUse"),
        "message: {}",
        f.message
    );
    // Real span: the literal misspelling in the script body.
    let s = content.find("PreTooluse").unwrap();
    assert_eq!(f.span, (s, s + "PreTooluse".len()), "SPAN OK: real span");
}

// ── CCC-HOOK-001 second surface: invalid hooks key in settings.json ─────────
#[test]
fn ccc_hook_001_invalid_event_in_settings_golden() {
    let findings = golden(
        "settings-json-ccc-hook-001-settings-surface.json",
        "/proj/.claude/settings.json",
    );
    let hits = with_code(&findings, "CCC-HOOK-001");
    assert_eq!(hits.len(), 1, "exactly one CCC-HOOK-001, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "json");
    assert!(
        f.message.starts_with("Invalid hook event 'on_tool_use'"),
        "message: {}",
        f.message
    );
    // Real span: the full quoted key token `"on_tool_use"` (bytes 11..24).
    assert_eq!(f.span, (11, 24), "SPAN OK: real span");
}

// ── CCC-HOOK-002: invalid hook event name (hook-script surface) ──────────────
#[test]
fn ccc_hook_002_invalid_event_name_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/hooks-ccc-hook-002.sh"),
    )
    .unwrap();
    let findings = golden("hooks-ccc-hook-002.sh", "/proj/hooks/session-start.sh");
    let hits = with_code(&findings, "CCC-HOOK-002");
    assert_eq!(hits.len(), 1, "exactly one CCC-HOOK-002, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "hook");
    assert!(
        f.message
            .starts_with("Invalid hook event name — use PreToolUse/PostToolUse"),
        "message: {}",
        f.message
    );
    // Real span: the literal invalid event name in the script body.
    let s = content.find("on_tool_use").unwrap();
    assert_eq!(f.span, (s, s + "on_tool_use".len()), "SPAN OK: real span");
}

// ── CCC-CMD-001: command markdown with no description surface ────────────────
#[test]
fn ccc_cmd_001_missing_description_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/commands-ccc-cmd-001.md"),
    )
    .unwrap();
    let findings = golden(
        "commands-ccc-cmd-001.md",
        "/proj/.claude/commands/specify.md",
    );
    let hits = with_code(&findings, "CCC-CMD-001");
    assert_eq!(hits.len(), 1, "exactly one CCC-CMD-001, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "commands");
    assert!(
        f.message.starts_with("Command frontmatter is missing a description"),
        "message: {}",
        f.message
    );
    // Real span: the frontmatter slice the analyzer reports — from after the
    // opening "---" to the newline before the closing "---".
    let fm_end = content.find("\n---").unwrap();
    assert_eq!(f.span, (3, fm_end), "SPAN OK: real span");
}

// ── CCC-CMD-002: argument-hint implies arguments, body lacks $ARGUMENTS ─────
#[test]
fn ccc_cmd_002_unbalanced_arguments_golden() {
    let content = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/commands-ccc-cmd-002.md"),
    )
    .unwrap();
    let findings = golden("commands-ccc-cmd-002.md", "/proj/.claude/commands/plan.md");
    let hits = with_code(&findings, "CCC-CMD-002");
    assert_eq!(hits.len(), 1, "exactly one CCC-CMD-002, got {findings:?}");
    let f = hits[0];
    assert_eq!(f.category, "commands");
    assert!(
        f.message.starts_with("argument-hint is set but body has no $ARGUMENTS placeholder"),
        "message: {}",
        f.message
    );
    // Real span: the analyzer spans the body start, clamped to 64 bytes.
    // body starts at the newline just before "## Clarifications".
    let s = content.find("## Clarifications").unwrap() - 1;
    assert_eq!(f.span, (s, s + 64), "SPAN OK: real span");
}

// ── clean command markdown: zero findings (non-vacuity) ─────────────────────
#[test]
fn ccc_cmd_clean_fixture_emits_nothing() {
    let clean = golden("commands-clean.md", "/proj/.claude/commands/specify.md");
    assert!(
        clean.is_empty(),
        "clean command fixture must emit zero findings: {clean:?}"
    );
}
