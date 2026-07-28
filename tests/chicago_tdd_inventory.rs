//! Chicago TDD suite for the CLI-backing modules: `scan` (shared classify +
//! analyze dispatch) and `inventory` (the `export` / `unused` verbs).
//!
//! Chicago/classicist style: exercise the real functions against real inputs
//! (in-memory strings and the actual repo tree) and assert on the resulting
//! state — no mocks. AAA (Arrange-Act-Assert) throughout, using
//! chicago-tdd-tools assertion macros (`assert_eq_msg!`, `assert_in_range!`).

use chicago_tdd_tools::prelude::*;
use claude_code_config_lsp::{inventory, scan};

// ────────────────────────────────────────────────────────────────────────────
// Fixtures
// ────────────────────────────────────────────────────────────────────────────

/// Paths that exercise the classification scoping rules (config vs. not).
struct ClassifyFixture {
    config_toml: &'static str,
    repo_cargo_toml: &'static str,
    toolchain_toml: &'static str,
    hook_script: &'static str,
    stray_shell_script: &'static str,
    settings_json: &'static str,
    readme: &'static str,
}

impl ClassifyFixture {
    fn new() -> Self {
        Self {
            config_toml: "/proj/.claude/lsp-max-auto.toml",
            repo_cargo_toml: "/proj/Cargo.toml",
            toolchain_toml: "/proj/rust-toolchain.toml",
            hook_script: "/proj/hooks/session-start.sh",
            stray_shell_script: "/proj/scripts/build.sh",
            settings_json: "/proj/.claude/settings.json",
            readme: "/proj/README.md",
        }
    }
}

/// CLAUDE.md content that legitimately mentions Claude/Anthropic many times —
/// must NOT trip the skill-name reserved-word rule.
struct ClaudeMdFixture {
    heavy_claude_mentions: String,
}

impl ClaudeMdFixture {
    fn new() -> Self {
        Self {
            heavy_claude_mentions: "# Project Guide\n\nThis project configures Claude Code. \
                Claude reads this file. Anthropic ships Claude. Claude, Claude, Claude.\n"
                .to_string(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// scan::classify — scoping rules
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn classify_scopes_toml_to_claude_directory() {
    // Arrange
    let fx = ClassifyFixture::new();

    // Act
    let config = scan::classify(fx.config_toml);
    let cargo = scan::classify(fx.repo_cargo_toml);
    let toolchain = scan::classify(fx.toolchain_toml);

    // Assert — only .claude/ TOML is a config surface; build TOML is not.
    assert_eq_msg!(config, "toml", "config TOML under .claude/ must classify as toml");
    assert_eq_msg!(cargo, "unknown", "Cargo.toml must NOT be treated as config TOML");
    assert_eq_msg!(toolchain, "unknown", "rust-toolchain.toml must NOT be config TOML");
}

#[test]
fn classify_scopes_hooks_to_hooks_directory() {
    // Arrange
    let fx = ClassifyFixture::new();

    // Act
    let hook = scan::classify(fx.hook_script);
    let stray = scan::classify(fx.stray_shell_script);

    // Assert — only scripts under a hooks/ dir are hooks.
    assert_eq_msg!(hook, "hook", "scripts under hooks/ must classify as hook");
    assert_eq_msg!(stray, "unknown", "a stray *.sh must NOT be treated as a hook");
}

#[test]
fn classify_recognizes_json_and_ignores_docs() {
    // Arrange
    let fx = ClassifyFixture::new();

    // Act / Assert
    assert_eq_msg!(scan::classify(fx.settings_json), "json", "settings.json is a json surface");
    assert_eq_msg!(scan::classify(fx.readme), "unknown", "README.md is not a config surface");
}

// ────────────────────────────────────────────────────────────────────────────
// scan::analyze_document — dispatch behavior + false-positive guards
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn analyze_document_flows_settings_enum_findings() {
    // Arrange — a bare-string skillOverrides must fire CCC-JSON-009.
    let uri = "/proj/.claude/settings.json";
    let content = r#"{"skillOverrides": "off"}"#;

    // Act
    let findings = scan::analyze_document(uri, content);

    // Assert
    assert!(
        findings.iter().any(|f| f.code == "CCC-JSON-009"),
        "settings.json enum findings must flow through analyze_document"
    );
}

#[test]
fn analyze_document_claude_md_has_no_skill_reserved_word_false_positive() {
    // Arrange — CLAUDE.md that mentions Claude/Anthropic repeatedly.
    let fx = ClaudeMdFixture::new();

    // Act
    let findings = scan::analyze_document("/proj/CLAUDE.md", &fx.heavy_claude_mentions);

    // Assert — the skill-name reserved-word rule must not apply to CLAUDE.md.
    assert!(
        !findings.iter().any(|f| f.code == "CCC-SKILL-003"),
        "CLAUDE.md's legitimate Claude/Anthropic mentions must not trip CCC-SKILL-003"
    );
}

#[test]
fn analyze_document_unknown_surface_is_clean() {
    // Arrange / Act
    let findings = scan::analyze_document("/proj/Cargo.toml", "[package]\nversion = \"1.0.0\"\n");

    // Assert — Cargo.toml is not a config surface, so no CalVer noise.
    assert!(findings.is_empty(), "non-config files must yield zero findings");
}

// ────────────────────────────────────────────────────────────────────────────
// A self-contained fixture repo covering every config surface, plus the
// "everything resolves cleanly" invariants `usage_report_flags_orphans_
// without_false_positives` asserts (declared skill resolves, the one hook is
// referenced, every settings key recognized). Previously these tests called
// `inventory::build_tree(".")` / `inventory::usage_report(".")` directly
// against the live repo, which broke whenever `.claude/agents/*.md` was
// absent from the working tree (e.g. mid uncommitted deletion) — a test
// coupled to filesystem state outside its control. Mirrors the equivalent
// fixture already added to `src/inventory.rs`'s own unit tests, and the
// temp-dir idiom `conformance_report_calculates_correct_score_and_tracks_
// surfaces` below already uses.
// ────────────────────────────────────────────────────────────────────────────

struct FixtureRepo {
    dir: std::path::PathBuf,
}

impl FixtureRepo {
    fn new(name: &str) -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("ccc_lsp_inventory_it_{name}_{ts}"));
        let write = |rel: &str, content: &str| {
            let path = dir.join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, content).unwrap();
        };
        write(
            ".claude/settings.json",
            r#"{"model": "sonnet", "hooks": {"SessionStart": [{"hooks": [{"type": "command", "command": "${CLAUDE_PROJECT_DIR}/hooks/session-start.sh"}]}]}}"#,
        );
        write(
            ".claude-plugin/plugin.json",
            r#"{
                "$schema": "https://json.schemastore.org/claude-code-plugin.json",
                "skills": {
                    "validate-config": {"path": "skills/validate-config/SKILL.md"}
                }
            }"#,
        );
        write(
            ".claude-plugin/skills/validate-config/SKILL.md",
            "---\nname: validate-config\ndescription: Fixture declared skill.\n---\nBody.\n",
        );
        write(
            ".claude/agents/worker.md",
            "---\nname: worker\ndescription: A fixture agent.\n---\nDo the work.\n",
        );
        write(
            ".claude/skills/undeclared/SKILL.md",
            "---\nname: undeclared\ndescription: An undeclared fixture skill.\n---\nBody.\n",
        );
        write(".claude/lsp-max-auto.toml", "edition = \"2024\"\nversion = \"26.7.3\"\n");
        write("hooks/session-start.sh", "#!/bin/sh\necho fixture\n");
        write(".mcp.json", r#"{"mcpServers": {"example": {"command": "example-mcp"}}}"#);
        Self { dir }
    }

    fn path(&self) -> &str {
        self.dir.to_str().unwrap()
    }
}

impl Drop for FixtureRepo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// inventory::build_tree — export state over a fixture repo
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn build_tree_covers_expected_surfaces_and_parses_json() {
    // Arrange
    let fixture = FixtureRepo::new("build_tree");
    // Act
    let tree = inventory::build_tree(fixture.path());

    // Assert — file_count is consistent with the node list.
    assert_eq_msg!(
        tree.file_count,
        tree.files.len(),
        "file_count must equal the number of emitted nodes"
    );

    let surfaces: Vec<&str> = tree.files.iter().map(|f| f.surface).collect();
    for expected in ["settings.json", "plugin.json", "agent", "skill", "toml", "hook", "mcp.json"] {
        assert!(
            surfaces.contains(&expected),
            "export tree missing expected surface: {expected}"
        );
    }

    // A json node's content is fully parsed (an object), not a raw string.
    let settings = tree
        .files
        .iter()
        .find(|f| f.surface == "settings.json")
        .expect("settings.json node present");
    assert!(settings.content.is_object(), "settings.json content must be parsed JSON");

    // Non-config TOML must not leak into the tree.
    assert!(
        !tree.files.iter().any(|f| f.path.ends_with("Cargo.toml")),
        "Cargo.toml must not appear as a config surface in the export tree"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// inventory::usage_report — unused-config audit over the real repo
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn usage_report_percent_within_bounds_and_invariant_holds() {
    // Arrange / Act
    let report = inventory::usage_report(".");

    // Assert — percentage is a real proportion.
    assert_in_range!(report.unused_percent, 0.0, 100.0);

    // Invariant: unused_items is exactly the sum of the four buckets.
    let bucket_sum = report.declared_but_missing.len()
        + report.orphaned_on_disk.len()
        + report.orphaned_hooks.len()
        + report.unknown_settings_keys.len();
    assert_eq_msg!(
        report.unused_items,
        bucket_sum,
        "unused_items must equal the sum of all four buckets"
    );
    assert!(report.total_items >= report.unused_items, "total must be >= unused");
}

#[test]
fn usage_report_flags_orphans_without_false_positives() {
    // Arrange
    let fixture = FixtureRepo::new("orphans");
    // Act
    let report = inventory::usage_report(fixture.path());

    // Assert — project-local skills and agents are surfaced as orphaned-on-disk.
    assert!(
        report.orphaned_on_disk.iter().any(|i| i.surface == "skill"),
        "expected undeclared project-local skills to be reported"
    );
    assert!(
        report.orphaned_on_disk.iter().any(|i| i.surface == "agent"),
        "expected undeclared project-local agents to be reported"
    );

    // No false positives in the other buckets for this repo:
    // the declared `validate-config` skill resolves under .claude-plugin/,
    // the one hook is referenced, and every settings key is recognized.
    assert!(
        report.declared_but_missing.is_empty(),
        "validate-config resolves on disk; nothing should be declared-but-missing"
    );
    assert!(
        report.orphaned_hooks.is_empty(),
        "the inline echo hook is not a script reference; no orphaned hooks expected"
    );
    assert!(
        report.unknown_settings_keys.is_empty(),
        "all settings keys in this repo are recognized"
    );
}

#[test]
fn conformance_report_calculates_correct_score_and_tracks_surfaces() {
    // Arrange
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let test_dir = std::env::temp_dir().join(format!("test_conformance_{ts}"));
    let claude_dir = test_dir.join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // 1 error (invalid model: CCC-JSON-004), 1 warning (invalid worktree baseRef: CCC-JSON-008)
    let settings_content = r#"{
        "model": "gpt-4",
        "worktree": {
            "baseRef": "bogus"
        }
    }"#;
    std::fs::write(claude_dir.join("settings.json"), settings_content).unwrap();

    // Act
    let report = inventory::conformance_report(&test_dir.to_string_lossy());

    // Clean up
    let _ = std::fs::remove_dir_all(&test_dir);

    // Assert
    assert_eq_msg!(report.surfaces_checked, 1, "should have checked 1 surface");
    assert_eq_msg!(report.surfaces_admitted, 0, "should have admitted 0 surfaces");
    assert_eq_msg!(report.violations.len(), 2, "should have exactly 2 violations");

    // Check violations content format: "{code}: {relative_path} {message}"
    let v1 = &report.violations[0];
    assert!(v1.contains("CCC-JSON-004") || v1.contains("CCC-JSON-008"));
    assert!(v1.contains(".claude/settings.json"));
    
    // Score should be (100 - 10 * 1 - 3 * 1) / 100.0 = 0.87
    let expected_score = 0.87;
    assert_eq_msg!(report.score, expected_score, "score should match formula");
}

#[test]
fn virtual_doc_health_matches_conformance_report() {
    // Arrange
    let root = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());

    // Act
    let report = inventory::conformance_report(&root);
    let virtual_doc_text = claude_code_config_lsp::virtual_docs::render();
    let deserialized_report: inventory::WorkspaceConformance = serde_json::from_str(&virtual_doc_text).unwrap();

    // Assert
    assert_eq_msg!(report.score, deserialized_report.score, "scores must match");
    assert_eq_msg!(report.surfaces_checked, deserialized_report.surfaces_checked, "surfaces checked must match");
    assert_eq_msg!(report.surfaces_admitted, deserialized_report.surfaces_admitted, "surfaces admitted must match");
    assert_eq_msg!(report.violations, deserialized_report.violations, "violations list must match");
}
