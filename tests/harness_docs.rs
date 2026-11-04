//! Chicago-style tests for the H1 harness virtual documents
//! (`claude-config://inventory|receipts|standing`) over the real C5 fixture
//! corpus at `tests/fixtures/claude-config/`. Real renderer functions, real
//! files, real receipts on disk — no mocks.

use claude_code_config_lsp::harness_docs;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/claude-config");

// ---------------------------------------------------------------------------
// inventory
// ---------------------------------------------------------------------------

#[test]
fn inventory_fixture_has_stable_sections_and_real_counts() {
    let md = harness_docs::render_inventory(FIXTURE);
    assert!(md.contains("# Workspace Config Inventory"));
    assert!(md.contains("## Summary"));
    assert!(md.contains("## Surfaces"));
    assert!(md.contains("## Findings by Code"));
    // summary carries the root and a conformance score line
    assert!(md.contains(&format!("Root: `{FIXTURE}`")));
    assert!(md.contains("Conformance score: 0."));
}

#[test]
fn inventory_fixture_surface_table_matches_tree_on_disk() {
    // Cross-check the rendered table against the real inventory API output.
    let tree = claude_code_config_lsp::inventory::build_tree(FIXTURE);
    let md = harness_docs::render_inventory(FIXTURE);
    let skills_on_disk = tree.files.iter().filter(|f| f.surface == "skill").count();
    assert!(skills_on_disk > 0, "fixture must contain skill files");
    let skill_row = harness_docs::surface_rows(FIXTURE)
        .into_iter()
        .find(|r| r.surface == "skill")
        .expect("skill surface must appear in rows");
    assert_eq!(skill_row.files, skills_on_disk);
    assert!(md.contains("| skill |"));
}

#[test]
fn inventory_empty_root_renders_placeholder_section() {
    let md = harness_docs::render_inventory("/nonexistent-root-h1");
    assert!(md.contains("No config surfaces discovered."));
}

// ---------------------------------------------------------------------------
// receipts
// ---------------------------------------------------------------------------

#[test]
fn receipts_fixture_chain_receipt_rendered_with_score() {
    let md = harness_docs::render_receipts(FIXTURE);
    assert!(md.contains("# Gate Receipts"));
    assert!(md.contains("## Status"));
    assert!(md.contains("## Receipts"));
    // the fixture ships one hash-named chain receipt with score 0.7
    assert!(md.contains("chain"));
    assert!(md.contains("conformance 0.700"));
    // age and staleness columns are populated
    assert!(md.contains("fresh") || md.contains("stale") || md.contains("unknown"));
}

#[test]
fn receipts_fleet_latest_json_convention_is_read() {
    // Real fleet-convention file: write latest.json into a temp copy of the
    // fixture root, then render and assert it is surfaced as fleet-latest.
    let tmp = std::env::temp_dir().join(format!("h1-latest-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(tmp.join(".claude/receipts")).unwrap();
    std::fs::write(
        tmp.join(".claude/receipts/latest.json"),
        r#"{"status":"green","commands":["cargo test","cargo clippy"]}"#,
    )
    .unwrap();
    let md = harness_docs::render_receipts(tmp.to_str().unwrap());
    assert!(md.contains("fleet-latest"));
    assert!(md.contains("green"));
    assert!(md.contains("cargo test"));
    assert!(md.contains("cargo clippy"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn receipts_missing_root_is_loud_unverified() {
    let md = harness_docs::render_receipts("/nonexistent-root-h1");
    assert!(md.contains("No receipts found"));
    assert!(md.contains("UNVERIFIED"));
}

// ---------------------------------------------------------------------------
// standing
// ---------------------------------------------------------------------------

#[test]
fn standing_fixture_partition_matches_real_analyzer_output() {
    // The fixture conformance score is 0.7 → real findings exist on the JSON
    // and agent surfaces; skill (6 clean SKILL.md files) is ALIVE because the
    // fixture's chain receipt is timestamped today (fresh). Assert the real
    // partition, not the test's own model of it.
    let rows = harness_docs::standing_rows(FIXTURE);
    let settings = rows
        .iter()
        .find(|r| r.surface == "settings.json")
        .expect("settings.json row");
    assert_eq!(settings.standing, "PARTIAL");
    assert!(settings.finding_count > 0);
    let agent = rows
        .iter()
        .find(|r| r.surface == "agent")
        .expect("agent row");
    assert_eq!(agent.standing, "PARTIAL");
    let skill = rows
        .iter()
        .find(|r| r.surface == "skill")
        .expect("skill row");
    assert_eq!(skill.standing, "ALIVE");
    assert_eq!(skill.finding_count, 0);
    assert_eq!(skill.files, 6, "fixture ships six SKILL.md files");
}

#[test]
fn standing_absent_surfaces_are_unverified() {
    let rows = harness_docs::standing_rows(FIXTURE);
    assert_eq!(rows.len(), harness_docs::KNOWN_SURFACES.len());
    for surface in ["plugin.json", "marketplace.json", "hook", "mcp.json"] {
        let row = rows
            .iter()
            .find(|r| r.surface == surface)
            .unwrap_or_else(|| panic!("{surface} row missing"));
        assert_eq!(row.standing, "UNVERIFIED", "{surface}");
        assert!(row.reason.contains("never scanned"));
    }
}

#[test]
fn standing_covers_every_known_surface() {
    let md = harness_docs::render_standing(FIXTURE);
    assert!(md.contains("# Surface Standing"));
    for surface in harness_docs::KNOWN_SURFACES {
        assert!(md.contains(surface), "standing table missing {surface}");
    }
}

// ---------------------------------------------------------------------------
// dispatch
// ---------------------------------------------------------------------------

#[test]
fn dispatch_routes_all_three_and_health_falls_through() {
    for uri in [
        "claude-config://inventory",
        "claude-config://receipts",
        "claude-config://standing",
    ] {
        let doc = claude_code_config_lsp::virtual_docs::render_uri(uri, FIXTURE);
        assert!(doc.is_some(), "{uri} must render");
        assert!(doc.unwrap().starts_with("# "), "{uri} must be markdown");
    }
    // health still renders through the same entry point
    let health =
        claude_code_config_lsp::virtual_docs::render_uri("claude-config://health", FIXTURE);
    assert!(health.is_some());
    // unknown virtual URIs are None, not a silent fallback
    assert!(
        claude_code_config_lsp::virtual_docs::render_uri("claude-config://bogus", FIXTURE)
            .is_none()
    );
    assert!(!claude_code_config_lsp::harness_docs::is_harness_doc_uri(
        "claude-config://health"
    ));
}

#[test]
fn scope_classification_matches_backend_semantics() {
    use claude_code_config_lsp::harness_docs::{agent_scope_of, HarnessScope};
    assert_eq!(
        agent_scope_of("/u/.claude/plugins/cache/p/plugin.json"),
        HarnessScope::Plugin
    );
    assert_eq!(agent_scope_of("/repo/settings.json"), HarnessScope::Project);
}
