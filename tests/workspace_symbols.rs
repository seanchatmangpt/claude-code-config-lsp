//! Chicago-style integration tests for workspace/symbol extraction
//! (`src/workspace_symbols.rs`) — real files on disk over a real fixture tree,
//! real functions, no mocks.

use claude_code_config_lsp::workspace_symbols::workspace_symbols;
use lsp_types_max::SymbolKind;

fn fixture_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ws_symbols")
}

/// Names of every symbol in the result, sorted for order-independent asserts.
fn sorted_names(syms: Vec<lsp_types_max::SymbolInformation>) -> Vec<String> {
    let mut v: Vec<String> = syms.into_iter().map(|s| s.name).collect();
    v.sort();
    v
}

fn symbols_with(query: &str) -> Vec<lsp_types_max::SymbolInformation> {
    workspace_symbols(&fixture_path(), query)
}

#[test]
fn all_surfaces_produce_symbols() {
    let names = sorted_names(symbols_with(""));

    // settings.json properties (dotted key paths).
    for expected in ["env.MAX", "env.PATH", "model"] {
        assert!(
            names.iter().any(|n| n == expected),
            "missing {expected} in {names:?}"
        );
    }

    // SKILL.md + agent frontmatter names -> CLASS symbols.
    assert!(names.contains(&"alpha-skill".to_string()), "{names:?}");
    assert!(names.contains(&"beta-agent".to_string()), "{names:?}");

    // CLAUDE.md H1 -> FILE symbol named by the document title.
    assert!(
        names.contains(&"Workspace Symbols Fixture".to_string()),
        "{names:?}"
    );

    // commands/*.md -> FUNCTION symbols named by the file stem.
    assert!(names.contains(&"deploy".to_string()), "{names:?}");

    // Depth cap: deep/l2/l3/l4/settings.json is at depth 4 (visible);
    // deep/l2/l3/l4/l5/settings.json is beyond it (pruned).
    assert!(names.contains(&"atLimit".to_string()), "{names:?}");
    assert!(!names.contains(&"tooDeep".to_string()), "{names:?}");

    // Pruned dirs: target/ and _build/ copies must not surface.
    assert!(!names.contains(&"hidden".to_string()), "{names:?}");
}

#[test]
fn kinds_match_surface_contract() {
    let syms = symbols_with("");
    let kind_of = |name: &str| {
        syms.iter()
            .find(|s| s.name == name)
            .map(|s| s.kind)
            .unwrap_or_else(|| panic!("no symbol named {name}"))
    };
    assert_eq!(kind_of("env.PATH"), SymbolKind::PROPERTY);
    assert_eq!(kind_of("alpha-skill"), SymbolKind::CLASS);
    assert_eq!(kind_of("beta-agent"), SymbolKind::CLASS);
    assert_eq!(kind_of("Workspace Symbols Fixture"), SymbolKind::FILE);
    assert_eq!(kind_of("deploy"), SymbolKind::FUNCTION);
}

#[test]
fn query_filters_case_insensitively() {
    // Exact-case substring.
    let names = sorted_names(symbols_with("alpha"));
    assert_eq!(names, vec!["alpha-skill".to_string()]);

    // Case-insensitive: query casing differs from symbol casing.
    let names = sorted_names(symbols_with("ALPHA"));
    assert_eq!(names, vec!["alpha-skill".to_string()]);

    // Dotted-path properties match on the full dotted name.
    let names = sorted_names(symbols_with("env."));
    assert_eq!(sorted_names(symbols_with("ENV.")), names);
    assert!(names.contains(&"env.PATH".to_string()), "{names:?}");
    assert!(!names.contains(&"model".to_string()), "{names:?}");

    // No match -> empty.
    assert!(symbols_with("zzz-no-such-symbol").is_empty());
}

#[test]
fn locations_point_at_real_files_and_lines() {
    let syms = symbols_with("");
    let loc = |name: &str| {
        syms.iter()
            .find(|s| s.name == name)
            .map(|s| s.location.clone())
            .unwrap_or_else(|| panic!("no symbol named {name}"))
    };

    // env.PATH sits on line 3 of the fixture settings.json (1-based line 3,
    // 0-based 2); the key column points past the opening quote.
    let l = loc("env.PATH");
    assert!(
        l.uri.as_str().ends_with("ws_symbols/settings.json"),
        "{}",
        l.uri.as_str()
    );
    assert_eq!(l.range.start.line, 2, "env.PATH line");

    // The H1 symbol points at CLAUDE.md line 0.
    let l = loc("Workspace Symbols Fixture");
    assert!(
        l.uri.as_str().ends_with("ws_symbols/CLAUDE.md"),
        "{}",
        l.uri.as_str()
    );
    assert_eq!(l.range.start.line, 0);

    // alpha-skill points at the SKILL.md under .claude/skills/.
    let l = loc("alpha-skill");
    assert!(
        l.uri.as_str().contains("skills/alpha/SKILL.md"),
        "{}",
        l.uri.as_str()
    );
}

#[test]
fn missing_root_is_empty_not_panic() {
    let empty = workspace_symbols(std::path::Path::new("/nonexistent/ws-symbols-probe"), "");
    assert!(empty.is_empty());
}
