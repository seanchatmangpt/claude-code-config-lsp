//! Chicago-style integration tests for wikilink + hook-path navigation.
//!
//! Real collaborators: the fixture tree on disk
//! (`tests/fixtures/navigation/`) plus the real claude-config corpus
//! fixtures (`tests/fixtures/claude-config/`); real file IO. No mocks;
//! assertions on final state.

use claude_code_config_lsp::navigation::*;
use lsp_types_max::Position;

fn nav_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/navigation")
}

fn corpus_roots() -> Vec<std::path::PathBuf> {
    vec![nav_root().join("rules")]
}

/// Read the shared claude-config fixture (9 wikilinks, counted with grep
/// against the file on disk — fixture content is the oracle).
fn global_claude_md() -> String {
    std::fs::read_to_string(nav_root().join("../claude-config/claude-global-CLAUDE.md"))
        .expect("claude-global-CLAUDE.md fixture must exist")
}

// ---------------------------------------------------------------------------
// Wikilink scanning
// ---------------------------------------------------------------------------

#[test]
fn scans_wikilinks_in_shared_corpus_fixture() {
    let text = global_claude_md();
    let links = find_wikilinks(&text);
    // Oracle: count `[[` openers in the file ON DISK right now (re-read).
    let opener_count = global_claude_md().matches("[[").count();
    assert_eq!(
        links.len(),
        opener_count,
        "every [[ opener on disk must scan as a link (19 occurrences)"
    );
    assert_eq!(links.len(), 19);
    assert!(
        links.iter().any(|l| l.stem == "operating-doctrine"),
        "fixture links [[operating-doctrine]]"
    );
    assert!(
        links.iter().all(|l| !l.stem.starts_with('[')),
        "stems never contain bracket chars"
    );
}

#[test]
fn composite_and_parenthetical_links_parse() {
    let text = "See [[testing-chicago-style]] (SPR) and [[a]]/[[b]] chain.";
    let links = find_wikilinks(text);
    let stems: Vec<&str> = links.iter().map(|l| l.stem.as_str()).collect();
    assert_eq!(stems, vec!["testing-chicago-style", "a", "b"]);
}

#[test]
fn malformed_brackets_are_skipped() {
    let text = "[[not a link [nested] ]] and [[unclosed and [[]] empty";
    assert!(find_wikilinks(text).is_empty());
}

#[test]
fn link_ranges_point_at_stem_text() {
    let text = "before [[operating-doctrine]] after";
    let links = find_wilinks_helper(text);
    assert_eq!(links.len(), 1);
    let l = &links[0];
    assert_eq!(l.range.start.line, 0);
    assert_eq!(l.range.start.character, 9, "after `before [[` (9 chars)");
    assert_eq!(l.range.end.character, 27, "stem is 18 chars");
    // The range actually covers the stem text on disk.
    let s: usize = "before [[".len();
    assert_eq!(&text[s..s + 18], "operating-doctrine");
}

fn find_wilinks_helper(text: &str) -> Vec<Wikilink> {
    find_wikilinks(text)
}

#[test]
fn position_on_stem_resolves_link() {
    let text = "rule [[testing-chicago-style]] here";
    // Cursor inside the stem (character 12).
    let pos = Position {
        line: 0,
        character: 12,
    };
    let link = wikilink_at_position(text, pos).expect("cursor on stem");
    assert_eq!(link.stem, "testing-chicago-style");
    // Cursor on the brackets does not crash; outside the link is None.
    assert!(wikilink_at_position(
        text,
        Position {
            line: 0,
            character: 3
        }
    )
    .is_none());
}

#[test]
fn position_past_eof_is_none_not_crash() {
    let text = "no links here";
    assert!(wikilink_at_position(
        text,
        Position {
            line: 99,
            character: 0
        }
    )
    .is_none());
    assert!(wikilink_at_position(
        "",
        Position {
            line: 0,
            character: 0
        }
    )
    .is_none());
}

#[test]
fn multibyte_stem_positions_survive() {
    let text = "凡例 [[operating-doctrine]] 終";
    let pos = Position {
        line: 0,
        character: 8,
    }; // inside the stem (UTF-16 units)
    let link = wikilink_at_position(text, pos).expect("multibyte prefix handled");
    assert_eq!(link.stem, "operating-doctrine");
}

// ---------------------------------------------------------------------------
// Stem resolution + definition
// ---------------------------------------------------------------------------

#[test]
fn stem_resolves_to_fixture_rule_file() {
    let p = resolve_stem("operating-doctrine", &corpus_roots()).expect("fixture rule exists");
    assert!(p.ends_with("rules/operating-doctrine.md"));
    assert!(p.is_file());
}

#[test]
fn unknown_stem_is_none() {
    assert!(resolve_stem("no-such-rule-anywhere", &corpus_roots()).is_none());
    // Empty roots: no crash.
    assert!(resolve_stem("operating-doctrine", &[]).is_none());
}

#[test]
fn definition_returns_location_at_target_h1() {
    let text = "See [[operating-doctrine]] for the core law.";
    let pos = Position {
        line: 0,
        character: 8,
    };
    let loc = wikilink_definition("file:///doc.md", text, pos, &corpus_roots())
        .expect("fixture rule must resolve");
    let expected_uri = format!(
        "file://{}",
        nav_root().join("rules/operating-doctrine.md").display()
    );
    assert_eq!(loc.uri.as_str(), expected_uri);
    assert_eq!(loc.range.start.line, 0, "H1 is the first line");
    assert!(loc.range.end.character > 0, "H1 range spans the heading");
}

#[test]
fn definition_of_unresolvable_link_is_none() {
    let text = "dangling [[never-exists-xyz]] link";
    let pos = Position {
        line: 0,
        character: 14,
    };
    assert!(
        wikilink_definition("file:///doc.md", text, pos, &corpus_roots()).is_none(),
        "tolerance law: unresolvable link -> None, no crash"
    );
}

#[test]
fn definition_with_cursor_off_link_is_none() {
    let text = "See [[operating-doctrine]] here.";
    let pos = Position {
        line: 0,
        character: 1,
    };
    assert!(wikilink_definition("file:///doc.md", text, pos, &corpus_roots()).is_none());
}

// ---------------------------------------------------------------------------
// References (reverse map)
// ---------------------------------------------------------------------------

#[test]
fn references_reverse_map_across_doc_set() {
    let target = nav_root().join("rules/operating-doctrine.md");
    let docs = vec![
        (
            "file:///a.md".to_string(),
            "links [[operating-doctrine]] once and [[testing-chicago-style]] too".to_string(),
        ),
        (
            "file:///b.md".to_string(),
            "again [[operating-doctrine]] plus [[a]]/[[b]] and [[operating-doctrine]] twice"
                .to_string(),
        ),
        (
            "file:///c.md".to_string(),
            "unrelated [[testing-chicago-style]] only".to_string(),
        ),
    ];
    let refs = wikilink_references(&docs, &target, &corpus_roots());
    assert_eq!(refs.len(), 3, "one in a.md, two in b.md");
    let uris: Vec<&str> = refs.iter().map(|r| r.uri.as_str()).collect();
    assert_eq!(uris, vec!["file:///a.md", "file:///b.md", "file:///b.md"]);
}

#[test]
fn references_to_unresolvable_target_are_empty() {
    let docs = vec![(
        "file:///a.md".to_string(),
        "[[operating-doctrine]]".to_string(),
    )];
    let refs = wikilink_references(&docs, &nav_root().join("rules/nope.md"), &corpus_roots());
    assert!(refs.is_empty());
}

// ---------------------------------------------------------------------------
// Document highlight
// ---------------------------------------------------------------------------

#[test]
fn highlight_returns_all_same_stem_ranges() {
    let text = "x [[operating-doctrine]] y [[testing-chicago-style]] z [[operating-doctrine]]";
    let pos = Position {
        line: 0,
        character: 4,
    };
    let hl = wikilink_highlight("file:///d.md", text, pos, &corpus_roots());
    assert_eq!(hl.len(), 2, "both operating-doctrine occurrences");
    // Sanity: each highlighted range covers the stem text.
    for loc in &hl {
        let s = loc.range.start.character as usize;
        let e = loc.range.end.character as usize;
        assert_eq!(&text[s..e], "operating-doctrine");
    }
}

#[test]
fn highlight_of_unresolvable_stem_is_empty() {
    let text = "dangling [[never-exists-xyz]] here";
    let pos = Position {
        line: 0,
        character: 12,
    };
    assert!(wikilink_highlight("file:///d.md", text, pos, &corpus_roots()).is_empty());
}

#[test]
fn highlight_off_link_is_empty() {
    let text = "plain text, no link";
    let pos = Position {
        line: 0,
        character: 2,
    };
    assert!(wikilink_highlight("file:///d.md", text, pos, &corpus_roots()).is_empty());
}

// ---------------------------------------------------------------------------
// Real corpus fixture: highlight inside claude-global-CLAUDE.md
// ---------------------------------------------------------------------------

#[test]
fn highlight_in_real_corpus_fixture_groups_same_stem() {
    let text = global_claude_md();
    // First link in the fixture is [[operating-doctrine]] (line 6-ish);
    // find its real position from the scan instead of hardcoding.
    let links = find_wikilinks(&text);
    let first = links
        .iter()
        .find(|l| l.stem == "operating-doctrine")
        .expect("fixture references operating-doctrine");
    let hl = wikilink_highlight("file:///g.md", &text, first.range.start, &corpus_roots());
    // Against the shared corpus roots the fixture's links resolve only if a
    // matching rule exists; either way the call must not crash, and with the
    // local fixture roots the doctrine stem resolves.
    assert!(
        !hl.is_empty(),
        "operating-doctrine resolves against fixture rules"
    );
}

// ---------------------------------------------------------------------------
// Hook path references
// ---------------------------------------------------------------------------

#[test]
fn hook_path_refs_scan_relative_script_paths() {
    let text = r#"Run "hooks/foo.sh" and scripts/repair.py; skip /abs/no.sh and ~/x/abs2.sh"#;
    let refs = find_hook_path_refs(text);
    let tokens: Vec<&str> = refs.iter().map(|(t, _)| t.as_str()).collect();
    assert_eq!(tokens, vec!["hooks/foo.sh", "scripts/repair.py"]);
    for (_, r) in &refs {
        let s = r.start.character as usize;
        let e = r.end.character as usize;
        assert!(text[s..e].contains('/'));
    }
}

#[test]
fn hook_path_definition_resolves_on_disk() {
    let text = "Use hooks/doctrine_hook.sh for doctrine";
    let pos = Position {
        line: 0,
        character: 8,
    };
    let base = nav_root();
    let loc = hook_path_definition("file:///doc.md", text, pos, &base)
        .expect("fixture hook script exists");
    let expected_uri = format!(
        "file://{}",
        nav_root().join("hooks/doctrine_hook.sh").display()
    );
    assert_eq!(loc.uri.as_str(), expected_uri);
}

#[test]
fn hook_path_definition_missing_file_is_none() {
    let text = "Use hooks/missing.sh here";
    let pos = Position {
        line: 0,
        character: 8,
    };
    assert!(hook_path_definition("file:///doc.md", text, pos, &nav_root()).is_none());
}

#[test]
fn hook_path_off_token_is_none() {
    let text = "Use hooks/doctrine_hook.sh here";
    let pos = Position {
        line: 0,
        character: 1,
    };
    assert!(hook_path_definition("file:///doc.md", text, pos, &nav_root()).is_none());
}
