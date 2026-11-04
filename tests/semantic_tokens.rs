//! Chicago-style substrate tests for the semantic-tokens classifier.
//!
//! Real collaborators only: the real tree-sitter JSON grammar parses real
//! settings.json-shaped fixtures, the real `build_tokens` pipeline emits the
//! tokens, and assertions land on final state — no mocks, no call-counting.
//!
//! Substrate receipt (I3, 2026-11-03): the generated classifier mapped legend
//! strings ("enumMember", "keyword", …) to node kinds the JSON grammar never
//! emits, so `build_tokens` returned zero tokens for every document. Fixed by
//! mapping the kinds tree-sitter-json actually produces. See
//! docs/jira/v26.11.3/receipts/i3-tokens-substrate.md.

use claude_code_config_lsp::semantic_tokens::{build_tokens, legend, token_type_strings, Document};
use claude_code_config_lsp::tokens_delta::{semantic_tokens_delta, SLOTS_PER_TOKEN};
use lsp_max::lsp_types::{Position, Range};

fn doc(src: &str) -> Document {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .expect("grammar");
    let tree = parser.parse(src, None).expect("parse");
    Document::new(src.to_string(), tree, None)
}

fn legend_index(name: &str) -> u32 {
    token_type_strings()
        .iter()
        .position(|s| s == name)
        .unwrap_or_else(|| panic!("legend lacks {name}")) as u32
}

fn tokens_at(src: &str) -> Vec<(u32, u32, u32, u32)> {
    // (start.line, start.character, length, token_type) — decoded back from
    // the delta-encoded wire form so tests assert on real emission, not on
    // the encoder's arithmetic.
    let data = &build_tokens(&doc(src)).data;
    let mut out = Vec::new();
    let mut line = 0u32;
    let mut ch = 0u32;
    for t in data {
        if t.delta_line > 0 {
            line += t.delta_line;
            ch = t.delta_start;
        } else {
            ch += t.delta_start;
        }
        out.push((line, ch, t.length, t.token_type));
    }
    out
}

const SETTINGS_FIXTURE: &str = r#"{
  "model": "opus",
  "permissions": { "allow": ["Bash(cargo test:*)"], "deny": [] },
  "statusLine": { "padding": 0 },
  "theme": "dark",
  "verbose": true,
  "disabled": false,
  "missing": null
}"#;

#[test]
fn real_settings_fixture_emits_tokens() {
    let toks = tokens_at(SETTINGS_FIXTURE);
    assert!(
        toks.len() > 10,
        "real settings.json fixture must emit tokens, got {}",
        toks.len()
    );
}

#[test]
fn keys_classify_as_property_including_quotes() {
    // `"model"` spans cols 2..9 including quotes, on line 1.
    let toks = tokens_at(r#"{ "model": "opus" }"#);
    let prop = legend_index("property");
    assert!(
        toks.contains(&(0, 2, 7, prop)),
        "key token must cover `\"model\"` (line 0, col 2, len 7) as property: {toks:?}"
    );
}

#[test]
fn string_values_classify_as_string_not_property() {
    let toks = tokens_at(r#"{ "model": "opus" }"#);
    let string = legend_index("string");
    let property = legend_index("property");
    let value = toks
        .iter()
        .find(|t| t.1 == 11)
        .expect("value token at col 11");
    assert_eq!(
        value,
        &(0, 11, 6, string),
        "value string must be `string`, distinct from the key's `property`"
    );
    assert_ne!(value.3, property);
}

#[test]
fn numbers_classify_as_number() {
    let toks = tokens_at(r#"{ "padding": 0 }"#);
    let number = legend_index("number");
    assert!(
        toks.contains(&(0, 13, 1, number)),
        "numeric leaf must classify as `number`: {toks:?}"
    );
}

#[test]
fn true_false_null_classify_as_keyword() {
    let keyword = legend_index("keyword");
    for (src, col) in [
        (r#"{ "verbose": true }"#, 13),
        (r#"{ "verbose": false }"#, 13),
        (r#"{ "verbose": null }"#, 13),
    ] {
        let toks = tokens_at(src);
        assert!(
            toks.contains(&(0, col, src.len() as u32 - 15, keyword)),
            "{src} must classify its literal as keyword: {toks:?}"
        );
    }
}

#[test]
fn nested_arrays_and_objects_are_traversed() {
    let toks = tokens_at(r#"{ "allow": ["Bash(cargo test:*)", "Read"] }"#);
    let property = legend_index("property");
    let string = legend_index("string");
    assert!(
        toks.iter().any(|t| t.3 == property),
        "outer key classified: {toks:?}"
    );
    assert_eq!(
        toks.iter().filter(|t| t.3 == string).count(),
        2,
        "both array string elements emitted: {toks:?}"
    );
}

#[test]
fn multiline_clip_restricts_to_range() {
    let src = SETTINGS_FIXTURE;
    let d = doc(src);
    let full = build_tokens(&d).data.len();
    let ranged =
        build_tokens_in_range_pub(&d, Range::new(Position::new(1, 0), Position::new(2, 40)))
            .data
            .len();
    assert!(ranged > 0, "range covering lines 1-2 must contain tokens");
    assert!(
        ranged < full,
        "range must be a strict subset of full: {ranged} vs {full}"
    );
}

// build_tokens_in_range is private-ish through the lib surface; re-exported
// helper keeps the test honest while using the public module API.
fn build_tokens_in_range_pub(d: &Document, r: Range) -> lsp_max::lsp_types::SemanticTokens {
    claude_code_config_lsp::semantic_tokens::build_tokens_in_range(d, r)
}

// --- delta integration over real classifier output ---


// NOTE (I3, 2026-11-03): a full "splice reconstructs next" law test over real
// classifier output was attempted here and FAILED — not in the classifier, but
// in tokens_delta::encode_abs, which seeds its delta encoding at (line 0,
// char 0) instead of continuing from the token preceding the edit boundary
// (LSP delta data must continue the array's encoding across the splice).
// Witnessed pair: `{"model": "opus"}` → `{"model": "sonnet"}` yields edit
// data `[dl0 ds9 len8]` where continuation semantics require `[dl0 ds8 len8]`.
// That law test lives in tests/tokens_color.rs (P8's lane) and is currently
// red for exactly this reason — it is the honest signal for a P8 repair.
#[test]
fn delta_over_real_output_produces_one_replacement_edit_for_value_change() {
    let v1 = r#"{
  "model": "opus",
  "theme": "dark"
}"#;
    let v2 = r#"{
  "model": "sonnet",
  "theme": "dark"
}"#;
    let t1 = build_tokens(&doc(v1));
    let t2 = build_tokens(&doc(v2));
    let d = semantic_tokens_delta(&t1, &t2, 7);
    assert_eq!(d.edits.len(), 1, "value change on one token => one edit");
    assert_eq!(
        d.edits[0].delete_count as usize, SLOTS_PER_TOKEN,
        "exactly one cached token slot replaced"
    );
    assert_eq!(
        d.edits[0]
            .data
            .as_ref()
            .map(|v| v.len() * SLOTS_PER_TOKEN)
            .unwrap_or(0),
        SLOTS_PER_TOKEN,
        "exactly one new token slot inserted"
    );
}

#[test]
fn one_key_value_change_produces_single_small_edit() {
    let v1 = r#"{
  "model": "opus",
  "theme": "dark"
}"#;
    let v2 = v1.replace("opus", "sonnet");
    let t1 = build_tokens(&doc(v1));
    let t2 = build_tokens(&doc(&v2));
    let d = semantic_tokens_delta(&t1, &t2, 9);
    assert_eq!(
        d.edits.len(),
        1,
        "one changed token => exactly one edit, got {:?}",
        d.edits
    );
    let e = &d.edits[0];
    let inserted = (e.data.as_ref().map_or(0, |v| v.len()) as u32) * SLOTS_PER_TOKEN as u32;
    let edit_size = e.delete_count + inserted;
    let total = (t2.data.len() * SLOTS_PER_TOKEN) as u32;
    assert!(
        edit_size < total,
        "edit must be smaller than full re-emit: {edit_size} vs {total}"
    );
}

#[test]
fn legend_includes_number_and_matches_advertised() {
    let l = legend();
    assert_eq!(l.token_types.len(), token_type_strings().len());
    assert!(
        token_type_strings().iter().any(|s| s == "number"),
        "`number` must be in the advertised legend"
    );
}
