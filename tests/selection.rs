//! Chicago tests for `selection` — real parses of real document content, no
//! mocks. Real tree-sitter JSON parses over nested fixture content (hooks
//! array-of-objects) and real line-based markdown with wikilinks; expected
//! ranges are computed from the fixture text by independent helpers, not
//! from the implementation under test.

use claude_code_config_lsp::selection::{
    highlight, highlight_json, highlight_markdown, selection_range, selection_range_markdown,
};
use lsp_types_max::{DocumentHighlightKind, Position};

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

/// Nested settings JSON: hooks array-of-objects, a key/value sharing a word
/// for WRITE/READ classification, and a multi-byte key for the UTF-16 path.
const NESTED_JSON: &str = r#"{
  "hooks": [
    {
      "matcher": "PreToolUse",
      "hooks": [
        { "type": "command", "command": "echo hi" }
      ]
    }
  ],
  "émoji_ключ": "v1",
  "plain": "émoji_ключ"
}
"#;

/// Markdown with wikilinks: two refs to one target, one defining heading.
const WIKILINK_MD: &str = r#"# Intro

See [[chatman-equation]] first.

## chatman-equation

Body of the equation section.
Second line mentions [[chatman-equation]] later.

## Other

Another [[operating-doctrine]] ref.
"#;

// ---------------------------------------------------------------------------
// Independent expectation helpers (derived from fixture text, not from
// src/selection.rs)
// ---------------------------------------------------------------------------

/// Byte offset of the Nth line's start: sum of line lens + 1.
fn line_start(content: &str, line_idx: usize) -> usize {
    content
        .split('\n')
        .take(line_idx)
        .map(|l| l.len() + 1)
        .sum()
}

/// Position of `byte_in_line` within line `line_idx`, via UTF-16 count.
fn pos_at(content: &str, line_idx: usize, byte_in_line: usize) -> Position {
    let ls = line_start(content, line_idx);
    let line_text = content[ls..].split('\n').next().unwrap_or("");
    let upto = &line_text[..byte_in_line.min(line_text.len())];
    Position::new(line_idx as u32, upto.encode_utf16().count() as u32)
}

/// Position of the first char of `needle` in line `line_idx`.
fn pos_of(content: &str, line_idx: usize, needle: &str) -> Position {
    let ls = line_start(content, line_idx);
    let line_text = content[ls..].split('\n').next().unwrap_or("");
    let byte = line_text.find(needle).expect("needle in line");
    pos_at(content, line_idx, byte)
}

/// `line:sc-ec` range string for a substring starting at `needle` with
/// UTF-16 length `units`.
fn sub_range(content: &str, line_idx: usize, needle: &str, units: usize) -> String {
    let start = pos_of(content, line_idx, needle);
    format!(
        "{}:{}-{}:{}",
        line_idx,
        start.character,
        line_idx,
        start.character as usize + units
    )
}

fn range_str(r: &lsp_types_max::Range) -> String {
    format!(
        "{}:{}-{}:{}",
        r.start.line, r.start.character, r.end.line, r.end.character
    )
}

/// Walk a SelectionRange chain into flat range strings, innermost first.
fn chain(content: &str, pos: Position, markdown: bool) -> Vec<String> {
    let sr = selection_range(content, pos, markdown).expect("selection chain exists");
    let mut cur = Some(&sr);
    let mut out = Vec::new();
    while let Some(link) = cur {
        out.push(range_str(&link.range));
        cur = link.parent.as_deref();
    }
    out
}

// ---------------------------------------------------------------------------
// selectionRange — JSON
// ---------------------------------------------------------------------------

#[test]
fn json_selection_chain_widens_from_value_to_root() {
    // On the "PreToolUse" value (line 3): innermost → outermost must be
    // string_content → string → pair → object → array → object → root
    // object → document, strictly widening.
    let pos = pos_of(NESTED_JSON, 3, "PreToolUse");
    let ranges = chain(NESTED_JSON, pos, false);
    assert_eq!(ranges.len(), 8, "chain: {ranges:?}");
    for w in ranges.windows(2) {
        assert_ne!(w[0], w[1], "links strictly widen: {ranges:?}");
    }
    // Innermost = the value's string_content ("PreToolUse", 10 chars).
    assert_eq!(ranges[0], sub_range(NESTED_JSON, 3, "PreToolUse", 10));
    // Next = the string node incl. quotes.
    assert_eq!(ranges[1], sub_range(NESTED_JSON, 3, "\"PreToolUse\"", 12));
    // Next = the pair (key through value).
    assert_eq!(
        ranges[2],
        sub_range(NESTED_JSON, 3, "\"matcher\": \"PreToolUse\"", 23)
    );
    // The hooks array-of-objects appears as its own link starting at the
    // `[` on line 1 (col 11) and spanning to the inner object's close.
    assert!(
        ranges.iter().any(|r| r.starts_with("1:11-")),
        "array link present: {ranges:?}"
    );
    // Outermost = whole document.
    let doc_end = pos_at(NESTED_JSON, NESTED_JSON.split('\n').count() - 1, 0);
    let want_doc = format!("0:0-{}:{}", doc_end.line, doc_end.character);
    assert_eq!(ranges.last().unwrap(), &want_doc);
}

#[test]
fn json_selection_chain_on_key_word_innermost_at_string_content() {
    // On the multi-byte key "émoji_ключ" (line 9): innermost = its
    // string_content, then string, then pair, then root object, document.
    let pos = pos_of(NESTED_JSON, 9, "émoji_ключ");
    let ranges = chain(NESTED_JSON, pos, false);
    assert_eq!(
        ranges.len(),
        5,
        "string_content → string → pair → object → doc: {ranges:?}"
    );
    // Innermost = the key text: UTF-16 width 11 (é is 2 units, the rest 1).
    let units: usize = "émoji_ключ".encode_utf16().count();
    assert_eq!(units, 10);
    assert_eq!(ranges[0], sub_range(NESTED_JSON, 9, "émoji_ключ", units));
}

#[test]
fn json_selection_between_tokens_selects_the_enclosing_pair() {
    // Column just after the closing quote of the "v1" value — between
    // tokens; the enclosing pair is the innermost named node.
    let ls = line_start(NESTED_JSON, 9);
    let line_text = NESTED_JSON[ls..].split('\n').next().unwrap();
    let colon = line_text.find(':').unwrap(); // between key and value tokens
    let pos = pos_at(NESTED_JSON, 9, colon);
    let ranges = chain(NESTED_JSON, pos, false);
    assert_eq!(ranges.len(), 3, "pair → object → document: {ranges:?}");
    assert_eq!(
        ranges[0],
        sub_range(NESTED_JSON, 9, "\"émoji_ключ\": \"v1\"", 18)
    );
}

#[test]
fn json_selection_out_of_range_position_clamps_to_document() {
    let ranges = chain(NESTED_JSON, Position::new(999, 999), false);
    // Clamped to EOF: the innermost covering node is the root object, then
    // the document.
    assert_eq!(ranges.len(), 2, "{ranges:?}");
    let e = pos_at(NESTED_JSON, NESTED_JSON.split('\n').count() - 1, 0);
    let want = format!("0:0-{}:{}", e.line, e.character);
    assert_eq!(ranges.last().unwrap(), &want);
}

// ---------------------------------------------------------------------------
// selectionRange — markdown
// ---------------------------------------------------------------------------

#[test]
fn markdown_selection_word_line_section_document() {
    // On "See" (line 2): word → line → section → document.
    let ranges = chain(WIKILINK_MD, pos_of(WIKILINK_MD, 2, "See"), true);
    assert_eq!(ranges.len(), 4, "word → line → section → doc: {ranges:?}");
    assert_eq!(ranges[0], sub_range(WIKILINK_MD, 2, "See", 3));
    // Line 2 full text.
    let ls = line_start(WIKILINK_MD, 2);
    let line_len = WIKILINK_MD[ls..].split('\n').next().unwrap().len();
    assert_eq!(ranges[1], format!("2:0-2:{}", line_len));
    // Section: nearest heading at/above line 2 is line 0 (`# Intro`), next
    // heading at line 4 → section spans lines 0..=3, i.e. 0:0 to line-4 col 0.
    assert_eq!(ranges[2], "0:0-4:0");
    // Document link.
    assert_eq!(
        ranges[3],
        format!("0:0-{}:0", WIKILINK_MD.split('\n').count() - 1)
    );
}

#[test]
fn markdown_selection_section_under_level2_heading() {
    // Line 6 (Body...) sits under `## chatman-equation` (line 4); the next
    // heading is `## Other` at line 9 → section = 4:0 to start of line 9.
    let ranges = chain(WIKILINK_MD, pos_of(WIKILINK_MD, 6, "Body"), true);
    assert_eq!(ranges.len(), 4, "{ranges:?}");
    assert_eq!(ranges[2], "4:0-9:0");
}

#[test]
fn markdown_selection_empty_document_returns_none() {
    assert!(selection_range_markdown("", Position::new(0, 0)).is_none());
}

// ---------------------------------------------------------------------------
// documentHighlight — JSON
// ---------------------------------------------------------------------------

#[test]
fn json_highlight_key_is_write_value_is_read() {
    // On the "plain" key: only occurrence, a pair key → WRITE.
    let pos = pos_of(NESTED_JSON, 10, "plain");
    let highlights = highlight_json(NESTED_JSON, pos);
    assert_eq!(highlights.len(), 1, "{highlights:?}");
    assert_eq!(highlights[0].kind, Some(DocumentHighlightKind::WRITE));
    assert_eq!(
        range_str(&highlights[0].range),
        sub_range(NESTED_JSON, 10, "plain", 5)
    );
}

#[test]
fn json_highlight_both_hooks_keys_are_write() {
    // "hooks" is a key twice (lines 1 and 4, at different indent → cols).
    let pos = pos_of(NESTED_JSON, 1, "hooks");
    let highlights = highlight_json(NESTED_JSON, pos);
    assert_eq!(highlights.len(), 2, "{highlights:?}");
    assert!(highlights
        .iter()
        .all(|h| h.kind == Some(DocumentHighlightKind::WRITE)));
    let mut got: Vec<String> = highlights.iter().map(|h| range_str(&h.range)).collect();
    got.sort();
    let mut want = vec![
        sub_range(NESTED_JSON, 1, "hooks", 5),
        sub_range(NESTED_JSON, 4, "hooks", 5),
    ];
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn json_highlight_shared_key_and_value_word() {
    // "émoji_ключ": key (line 9) is WRITE, value occurrence (line 10, value
    // of "plain") is READ. UTF-16 columns verified via helper.
    let pos = pos_of(NESTED_JSON, 9, "émoji_ключ");
    let highlights = highlight_json(NESTED_JSON, pos);
    assert_eq!(highlights.len(), 2, "{highlights:?}");
    let mut got: Vec<(String, Option<DocumentHighlightKind>)> = highlights
        .iter()
        .map(|h| (range_str(&h.range), h.kind))
        .collect();
    got.sort_by_key(|(r, _)| r.clone());
    let mut want: Vec<(String, Option<DocumentHighlightKind>)> = vec![
        ("9:3-9:13".to_string(), Some(DocumentHighlightKind::WRITE)),
        (
            sub_range(NESTED_JSON, 10, "émoji_ключ", "émoji_ключ".chars().count()),
            Some(DocumentHighlightKind::READ),
        ),
    ];
    want.sort_by_key(|(r, _)| r.clone());
    assert_eq!(got, want);
}

#[test]
fn json_highlight_position_on_non_string_node_yields_empty() {
    // On the root `{` punctuation, not inside a string.
    let highlights = highlight_json(NESTED_JSON, Position::new(0, 0));
    assert!(highlights.is_empty(), "{highlights:?}");
}

// ---------------------------------------------------------------------------
// documentHighlight — markdown
// ---------------------------------------------------------------------------

#[test]
fn markdown_highlight_wikilinks_are_read_heading_is_write() {
    // On the wikilink at line 2: two READ refs (lines 2 and 7) + one WRITE
    // heading (line 4).
    let pos = pos_of(WIKILINK_MD, 2, "chatman-equation");
    let highlights = highlight_markdown(WIKILINK_MD, pos);
    assert_eq!(highlights.len(), 3, "{highlights:?}");
    let reads: Vec<String> = highlights
        .iter()
        .filter(|h| h.kind == Some(DocumentHighlightKind::READ))
        .map(|h| range_str(&h.range))
        .collect();
    let writes: Vec<String> = highlights
        .iter()
        .filter(|h| h.kind == Some(DocumentHighlightKind::WRITE))
        .map(|h| range_str(&h.range))
        .collect();
    // Targets exclude the brackets: `[[` is 2 wide.
    let t2 = sub_range(WIKILINK_MD, 2, "chatman-equation", "chatman-equation".len());
    let t7 = sub_range(WIKILINK_MD, 7, "chatman-equation", "chatman-equation".len());
    let mut got_reads = reads.clone();
    got_reads.sort();
    let mut want_reads = vec![t2, t7];
    want_reads.sort();
    assert_eq!(got_reads, want_reads, "reads: {highlights:?}");
    // Heading line `## chatman-equation` — full line 4 (19 chars).
    let ls4 = line_start(WIKILINK_MD, 4);
    let h4len = WIKILINK_MD[ls4..].split('\n').next().unwrap().len();
    assert_eq!(writes, vec![format!("4:0-4:{}", h4len)]);
}

#[test]
fn markdown_highlight_case_insensitive_target_match() {
    let doc = "# Intro\n\nsee [[Operating-Doctrine]] here.\n";
    let highlights = highlight_markdown(doc, pos_of(doc, 2, "Operating-Doctrine"));
    assert_eq!(highlights.len(), 1, "{highlights:?}");
    assert_eq!(highlights[0].kind, Some(DocumentHighlightKind::READ));
    assert_eq!(
        range_str(&highlights[0].range),
        sub_range(doc, 2, "Operating-Doctrine", "Operating-Doctrine".len())
    );
}

#[test]
fn markdown_highlight_plain_word_yields_no_wikilink_matches() {
    // "See" is not a wikilink target and matches no heading.
    let highlights = highlight_markdown(WIKILINK_MD, pos_of(WIKILINK_MD, 2, "See"));
    assert!(highlights.is_empty(), "{highlights:?}");
}

// ---------------------------------------------------------------------------
// Dispatchers
// ---------------------------------------------------------------------------

#[test]
fn dispatcher_routes_markdown_vs_json() {
    // Markdown path returns the full chain on markdown content.
    assert_eq!(
        chain(WIKILINK_MD, pos_of(WIKILINK_MD, 2, "See"), true).len(),
        4
    );
    // JSON path on markdown content: tree-sitter JSON parses error-tolerantly
    // (ERROR nodes) so a selection chain still exists — but highlight finds
    // no string_content words → empty.
    let json_view = selection_range(WIKILINK_MD, pos_of(WIKILINK_MD, 2, "See"), false);
    assert!(
        json_view.is_some(),
        "error-tolerant parse still yields a chain"
    );
    assert!(highlight(WIKILINK_MD, pos_of(WIKILINK_MD, 2, "See"), false).is_empty());
}
