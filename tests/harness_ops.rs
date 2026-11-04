//! Chicago tests for harness_ops: every one of the 9 LSP-tool operations
//! over real fixture files (the C5 fixture corpus at
//! tests/fixtures/claude-config plus hermetic temp workspaces for
//! disk-dependent premises). Real collaborators, no mocks; typed misses
//! asserted as variants, not unwraps.

use claude_code_config_lsp::harness_ops::{harness_operation, OperationResult, OPERATIONS};
use claude_code_config_lsp::hierarchy::Corpus;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Unique per-test temp workspace under the OS temp dir (no tempfile dep).
fn temp_workspace(tag: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "ccc-lsp-harness-{}-{}-{}",
        tag,
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    std::fs::create_dir_all(&dir).expect("create temp workspace");
    dir
}

/// Recursively collect (path, text) pairs under `root` (prune .git/target).
fn corpus_under(root: &Path) -> Corpus {
    fn walk(dir: &Path, out: &mut Corpus) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(&p, out);
            } else if let Ok(t) = std::fs::read_to_string(&p) {
                out.push((p, t));
            }
        }
    }
    let mut out: Corpus = Vec::new();
    walk(root, &mut out);
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// The C5 fixture corpus (real repo fixtures, absolute paths).
fn c5_corpus() -> Corpus {
    let mut c = corpus_under(Path::new("tests/fixtures/claude-config"));
    c.sort_by(|a, b| a.0.cmp(&b.0));
    c
}

/// (path, text) of a corpus entry whose path contains `needle`.
fn corpus_entry(corpus: &Corpus, needle: &str) -> (PathBuf, String) {
    corpus
        .iter()
        .find(|(p, _)| p.to_string_lossy().contains(needle))
        .map(|(p, t)| (p.clone(), t.clone()))
        .unwrap_or_else(|| panic!("no corpus entry contains {needle}"))
}

/// URI for a corpus path, matching the harness file:// convention.
fn uri_for(path: &Path) -> String {
    format!("file://{}", path.display())
}

/// Run an op with 1-based coordinates against a corpus entry.
fn op(name: &str, corpus: &Corpus, needle: &str, line_1: u32, char_1: u32) -> OperationResult {
    let (path, text) = corpus_entry(corpus, needle);
    harness_operation(name, &uri_for(&path), line_1, char_1, &text, corpus)
}

/// Run an op with explicit doc text (dirty-buffer shape).
fn op_text(
    name: &str,
    uri: &str,
    text: &str,
    corpus: &Corpus,
    line_1: u32,
    char_1: u32,
) -> OperationResult {
    harness_operation(name, uri, line_1, char_1, text, corpus)
}

/// A hermetic config workspace: real files on disk (skill, agent, rules
/// with wikilinks, settings.json with a hook command, the hook script).
fn hermetic_ws() -> Corpus {
    let ws = temp_workspace("ws");
    std::fs::create_dir_all(ws.join("skills/ggen")).unwrap();
    std::fs::create_dir_all(ws.join("skills/other")).unwrap();
    let rules_dir = ws.join("rules");
    std::fs::create_dir_all(&rules_dir).unwrap();
    let hooks_dir = ws.join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();
    std::fs::write(
        ws.join("skills/ggen/SKILL.md"),
        "---\nname: ggen\ndescription: test skill\n---\n\n# ggen\n\nBody text.\n",
    )
    .unwrap();
    std::fs::write(
        ws.join("skills/other/SKILL.md"),
        "---\nname: other\n---\n\n# other\n\nMentions [[ggen]] here.\n",
    )
    .unwrap();
    std::fs::write(
        ws.join("rules/operating-doctrine.md"),
        "# Operating Doctrine\n\nSee also [[testing-chicago-style]].\n",
    )
    .unwrap();
    std::fs::write(
        ws.join("rules/testing-chicago-style.md"),
        "# Testing Discipline\n\nChicago rules body.\n",
    )
    .unwrap();
    std::fs::write(
        ws.join("settings.json"),
        "{\n  \"model\": \"sonnet\",\n  \"hooks\": {\n    \"PreToolUse\": [\n      {\n        \"matcher\": \"Bash\",\n        \"hooks\": [\n          {\n            \"type\": \"command\",\n            \"command\": \"hooks/doctrine_hook.sh\"\n          }\n        ]\n      }\n    ]\n  }\n}\n",
    ).unwrap();
    std::fs::write(ws.join("hooks/doctrine_hook.sh"), "#!/bin/sh\necho ok\n").unwrap();
    corpus_under(&ws)
}

// ---------------------------------------------------------------------------
// Dispatch + 1-based conversion
// ---------------------------------------------------------------------------

#[test]
fn nine_operations_advertised_and_unknown_op_is_typed_refusal() {
    assert_eq!(OPERATIONS.len(), 9);
    assert!(OPERATIONS.contains(&"goToDefinition"));
    assert!(OPERATIONS.contains(&"prepareCallHierarchy"));
    let corpus = hermetic_ws();
    let (path, text) = corpus_entry(&corpus, "settings.json");
    let result = harness_operation(
        "teleportToDefinition",
        &uri_for(&path),
        1,
        1,
        &text,
        &corpus,
    );
    match result {
        OperationResult::Unresolved { reason, detail } => {
            assert_eq!(reason, "unknown-operation");
            assert!(detail.contains("teleportToDefinition"));
        }
        other => panic!("expected typed refusal, got {other:?}"),
    }
}

#[test]
fn one_based_coords_hover_the_model_key() {
    // Line 2 col 4 (1-based) is inside `"model"` on the hermetic settings.
    let corpus = hermetic_ws();
    let result = op("hover", &corpus, "settings.json", 2, 4);
    let OperationResult::Hover(hovered) = result else {
        panic!("expected Hover, got {result:?}");
    };
    let hover = hovered.expect("settings.json always classifies for hover");
    let lsp_types_max::HoverContents::Markup(m) = hover.contents else {
        panic!("expected markup contents");
    };
    assert!(m.value.contains("**`model`**"), "got: {}", m.value);
}

// ---------------------------------------------------------------------------
// goToDefinition
// ---------------------------------------------------------------------------

#[test]
fn definition_wikilink_resolves_to_target_h1() {
    let corpus = hermetic_ws();
    let result = op(
        "goToDefinition",
        &corpus,
        "rules/operating-doctrine.md",
        3,
        14,
    );
    let OperationResult::Definition(loc) = result else {
        panic!("expected Definition, got {result:?}");
    };
    assert!(
        loc.uri.as_str().ends_with("testing-chicago-style.md"),
        "got {}",
        loc.uri.as_str()
    );
}

#[test]
fn definition_wikilink_unresolvable_is_typed_miss() {
    // C5 fixture: [[local-dfcm-manufacturing-engine]] has no target file
    // under the fixture roots.
    let corpus = c5_corpus();
    let result = op("goToDefinition", &corpus, "zcode-global-CLAUDE.md", 82, 4);
    match result {
        OperationResult::Unresolved { reason, .. } => {
            assert_eq!(reason, "wikilink-unresolvable");
        }
        other => panic!("expected typed miss, got {other:?}"),
    }
}

#[test]
fn definition_hook_path_resolves_on_disk() {
    let corpus = hermetic_ws();
    // Line 10 char 25 is the first char of the command value.
    let result = op("goToDefinition", &corpus, "settings.json", 10, 25);
    let OperationResult::Definition(loc) = result else {
        panic!("expected Definition, got {result:?}");
    };
    assert!(
        loc.uri.as_str().ends_with("doctrine_hook.sh"),
        "got {}",
        loc.uri.as_str()
    );
}
#[test]
fn definition_json_key_resolves_to_ontology_property() {
    let corpus = hermetic_ws();
    let result = op("goToDefinition", &corpus, "settings.json", 2, 4);
    let OperationResult::Definition(loc) = result else {
        panic!("expected Definition, got {result:?}");
    };
    assert!(
        loc.uri.as_str().ends_with("claude-code-config.ttl"),
        "got {}",
        loc.uri.as_str()
    );
    // The range must point at the real ccc:settings_model line in the ttl.
    let ttl = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/schema/claude-code-config.ttl"
    ))
    .unwrap();
    let declared = ttl
        .lines()
        .nth(loc.range.start.line as usize)
        .expect("range line inside ttl");
    assert!(
        declared.trim_start().starts_with("ccc:settings_model"),
        "got line: {declared}"
    );
}

#[test]
fn definition_unknown_key_is_typed_key_not_in_schema() {
    let corpus = hermetic_ws();
    let (path, _) = corpus_entry(&corpus, "settings.json");
    let dirty = "{\n  \"bogusKey\": 1,\n  \"model\": \"x\"\n}\n";
    let result = op_text("goToDefinition", &uri_for(&path), dirty, &corpus, 2, 4);
    match result {
        OperationResult::Unresolved { reason, .. } => {
            assert_eq!(reason, "key-not-in-schema");
        }
        other => panic!("expected typed miss, got {other:?}"),
    }
}

#[test]
fn definition_cursor_on_plain_prose_is_typed_miss() {
    let corpus = hermetic_ws();
    let result = op(
        "goToDefinition",
        &corpus,
        "rules/testing-chicago-style.md",
        3,
        8,
    );
    match result {
        OperationResult::Unresolved { reason, .. } => {
            assert_eq!(reason, "cursor-on-nothing-navigable");
        }
        other => panic!("expected typed miss, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// goToImplementation
// ---------------------------------------------------------------------------

#[test]
fn implementation_command_value_resolves_script_on_disk() {
    let corpus = hermetic_ws();
    let result = op("goToImplementation", &corpus, "settings.json", 10, 25);
    let OperationResult::Definition(loc) = result else {
        panic!("expected Definition, got {result:?}");
    };
    assert!(
        loc.uri.as_str().ends_with("doctrine_hook.sh"),
        "got {}",
        loc.uri.as_str()
    );
}

#[test]
fn implementation_inline_shell_resolves_binary_on_path() {
    let corpus = hermetic_ws();
    let (path, _) = corpus_entry(&corpus, "settings.json");
    let dirty = "{\n  \"command\": \"sh -c 'echo hi'\"\n}\n";
    let result = op_text("goToImplementation", &uri_for(&path), dirty, &corpus, 2, 15);
    let OperationResult::Definition(loc) = result else {
        panic!("expected Definition (sh is on PATH), got {result:?}");
    };
    assert!(loc.uri.as_str().contains("/sh"), "got {}", loc.uri.as_str());
}

#[test]
fn implementation_inline_shell_missing_binary_is_not_on_path() {
    let corpus = hermetic_ws();
    let (path, _) = corpus_entry(&corpus, "settings.json");
    let dirty = "{\n  \"command\": \"definitely-not-a-real-binary-xyz run\"\n}\n";
    let result = op_text("goToImplementation", &uri_for(&path), dirty, &corpus, 2, 15);
    match result {
        OperationResult::NotOnDisk { path, reason } => {
            assert_eq!(path, "definitely-not-a-real-binary-xyz");
            assert_eq!(reason, "not-on-path");
        }
        other => panic!("expected typed NotOnDisk, got {other:?}"),
    }
}

#[test]
fn implementation_missing_script_is_not_on_disk() {
    let corpus = hermetic_ws();
    let (path, _) = corpus_entry(&corpus, "settings.json");
    let dirty = "{\n  \"command\": \"hooks/missing_hook.sh\"\n}\n";
    let result = op_text("goToImplementation", &uri_for(&path), dirty, &corpus, 2, 15);
    match result {
        OperationResult::NotOnDisk { path, reason } => {
            assert!(path.ends_with("missing_hook.sh"), "got {path}");
            assert_eq!(reason, "not-on-disk");
        }
        other => panic!("expected typed NotOnDisk, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// findReferences
// ---------------------------------------------------------------------------

#[test]
fn references_wikilink_channel_finds_definition_and_mentions() {
    let corpus = hermetic_ws();
    let result = op(
        "findReferences",
        &corpus,
        "rules/operating-doctrine.md",
        3,
        14,
    );
    let OperationResult::References(locs) = result else {
        panic!("expected References, got {result:?}");
    };
    let targets: Vec<String> = locs.iter().map(|l| l.uri.as_str().to_string()).collect();
    assert!(
        targets
            .iter()
            .any(|t| t.ends_with("testing-chicago-style.md")),
        "definition site missing: {targets:?}"
    );
    assert!(
        targets.iter().any(|t| t.ends_with("operating-doctrine.md")),
        "mention site missing: {targets:?}"
    );
}

#[test]
fn references_name_channel_finds_corpus_referers() {
    // Cursor on a blank line of the ggen SKILL.md: reference name falls
    // back to the frontmatter name `ggen`; skills/other/SKILL.md has
    // [[ggen]].
    let corpus = hermetic_ws();
    let result = op("findReferences", &corpus, "skills/ggen/SKILL.md", 7, 1);
    let OperationResult::References(locs) = result else {
        panic!("expected References, got {result:?}");
    };
    let targets: Vec<String> = locs.iter().map(|l| l.uri.as_str().to_string()).collect();
    assert!(
        targets.iter().any(|t| t.ends_with("skills/other/SKILL.md")),
        "referer missing: {targets:?}"
    );
}

#[test]
fn references_unreferenced_word_is_typed_miss() {
    // Cursor on the word "Body": not a wikilink, not referenced anywhere.
    let corpus = hermetic_ws();
    let result = op("findReferences", &corpus, "skills/ggen/SKILL.md", 8, 2);
    match result {
        OperationResult::Unresolved { reason, .. } => {
            assert_eq!(reason, "no-references-found");
        }
        other => panic!("expected typed miss, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// hover + documentSymbol + workspaceSymbol
// ---------------------------------------------------------------------------

#[test]
fn hover_on_claude_md_falls_back_to_surface_dump() {
    let corpus = c5_corpus();
    let result = op("hover", &corpus, "claude-global-CLAUDE.md", 3, 1);
    let OperationResult::Hover(hovered) = result else {
        panic!("expected Hover, got {result:?}");
    };
    let hover = hovered.expect("CLAUDE.md classifies to a schema surface");
    let lsp_types_max::HoverContents::Markup(m) = hover.contents else {
        panic!("expected markup contents");
    };
    assert!(
        m.value.contains("## "),
        "surface dump expected: {}",
        m.value
    );
}

#[test]
fn document_symbol_markdown_returns_heading_outline() {
    let corpus = c5_corpus();
    let result = op("documentSymbol", &corpus, "zcode-global-CLAUDE.md", 1, 1);
    let OperationResult::DocumentSymbols(symbols) = result else {
        panic!("expected DocumentSymbols, got {result:?}");
    };
    assert!(!symbols.is_empty(), "markdown outline must be non-empty");
    assert!(symbols[0].kind == lsp_types_max::SymbolKind::STRING);
    assert!(!symbols[0].name.is_empty());
}

#[test]
fn document_symbol_json_builds_key_tree() {
    let corpus = hermetic_ws();
    let result = op("documentSymbol", &corpus, "settings.json", 1, 1);
    let OperationResult::DocumentSymbols(symbols) = result else {
        panic!("expected DocumentSymbols, got {result:?}");
    };
    let model = symbols
        .iter()
        .find(|s| s.name == "model")
        .expect("model key");
    assert_eq!(model.kind, lsp_types_max::SymbolKind::STRING);
    let hooks = symbols
        .iter()
        .find(|s| s.name == "hooks")
        .expect("hooks key");
    assert_eq!(hooks.kind, lsp_types_max::SymbolKind::OBJECT);
    let children = hooks.children.as_ref().expect("hooks has children");
    assert!(children.iter().any(|c| c.name == "PreToolUse"));
}

#[test]
fn workspace_symbol_scans_the_corpus_root() {
    let corpus = c5_corpus();
    // Empty query = all symbols; supplied via a blank doc-text override so
    // word_at finds no word (the on-disk frontmatter would yield `---`).
    let (path, _) = corpus_entry(&corpus, "skills/ggen/SKILL.md");
    let result = op_text("workspaceSymbol", &uri_for(&path), "\n", &corpus, 1, 1);
    let OperationResult::WorkspaceSymbols(symbols) = result else {
        panic!("expected WorkspaceSymbols, got {result:?}");
    };
    assert!(
        symbols.iter().any(|s| s.name == "ggen"),
        "skill symbol ggen expected, got: {:?}",
        symbols.iter().map(|s| s.name.clone()).collect::<Vec<_>>()
    );
}

#[test]
fn prepare_clamps_to_frontmatter_name_anywhere_in_doc() {
    // Cursor on the word "Body" (line 8) - not the name value - still
    // yields the skill's callable unit via the clamp.
    let corpus = hermetic_ws();
    let result = op(
        "prepareCallHierarchy",
        &corpus,
        "skills/ggen/SKILL.md",
        8,
        2,
    );
    let OperationResult::CallHierarchyItem(item) = result else {
        panic!("expected CallHierarchyItem, got {result:?}");
    };
    let item = item.expect("clamped prepare must succeed for a named doc");
    assert_eq!(item.name, "ggen");
}

#[test]
fn prepare_on_nameless_doc_is_typed_miss() {
    let corpus = hermetic_ws();
    let (path, _) = corpus_entry(&corpus, "settings.json");
    let result = harness_operation(
        "prepareCallHierarchy",
        &uri_for(&path),
        1,
        1,
        "{\n  \"model\": \"sonnet\"\n}\n",
        &corpus,
    );
    match result {
        OperationResult::Unresolved { reason, .. } => {
            assert_eq!(reason, "no-callable-unit");
        }
        other => panic!("expected typed miss, got {other:?}"),
    }
}

#[test]
fn incoming_calls_for_skill_name_find_corpus_referers() {
    // Cursor on blank line 7 -> frontmatter name fallback -> ggen.
    let corpus = hermetic_ws();
    let result = op("incomingCalls", &corpus, "skills/ggen/SKILL.md", 7, 1);
    let OperationResult::IncomingCalls(calls) = result else {
        panic!("expected IncomingCalls, got {result:?}");
    };
    let sources: Vec<String> = calls
        .iter()
        .map(|c| c.from.uri.as_str().to_string())
        .collect();
    assert!(
        sources.iter().any(|s| s.ends_with("skills/other/SKILL.md")),
        "referer missing: {sources:?}"
    );
}

#[test]
fn outgoing_calls_from_doctrine_find_resolved_targets() {
    let corpus = hermetic_ws();
    let result = op(
        "outgoingCalls",
        &corpus,
        "rules/operating-doctrine.md",
        1,
        1,
    );
    let OperationResult::OutgoingCalls(calls) = result else {
        panic!("expected OutgoingCalls, got {result:?}");
    };
    let targets: Vec<String> = calls
        .iter()
        .map(|c| c.to.uri.as_str().to_string())
        .collect();
    assert!(
        targets
            .iter()
            .any(|t| t.ends_with("testing-chicago-style.md")),
        "resolved target missing: {targets:?}"
    );
}

#[test]
fn outgoing_calls_with_no_refs_is_typed_miss() {
    let corpus = hermetic_ws();
    let result = op(
        "outgoingCalls",
        &corpus,
        "rules/testing-chicago-style.md",
        1,
        1,
    );
    match result {
        OperationResult::Unresolved { reason, .. } => {
            assert_eq!(reason, "no-references-found");
        }
        other => panic!("expected typed miss, got {other:?}"),
    }
}

#[test]
fn dirty_buffer_semantics_use_doc_text_not_stale_corpus() {
    // The corpus has the on-disk hook; the editor buffer points elsewhere.
    // The op must consult the dirty text (typed NotOnDisk), not the stale
    // corpus copy.
    let corpus = hermetic_ws();
    let (path, _) = corpus_entry(&corpus, "settings.json");
    let dirty = "{\n  \"command\": \"hooks/replaced_hook.sh\"\n}\n";
    let result = op_text("goToDefinition", &uri_for(&path), dirty, &corpus, 2, 15);
    match result {
        OperationResult::NotOnDisk { path, reason } => {
            assert!(path.ends_with("replaced_hook.sh"), "got {path}");
            assert_eq!(reason, "not-on-disk");
        }
        other => panic!("expected typed NotOnDisk, got {other:?}"),
    }
}
