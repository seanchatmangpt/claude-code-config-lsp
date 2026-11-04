//! Delta-encoding boundary tests over REAL `build_tokens` output.
//!
//! These replace the synthetic token arrays that previously masked the
//! `encode_abs` boundary defect (see docs/jira/v26.11.3/receipts/i3-tokens-substrate.md
//! and i7-delta-boundary.md): the edit's `data` must CONTINUE the array's
//! delta encoding from the token preceding the edit region — seeding at
//! absolute (0,0) only coincides with continuation when the preceding tokens
//! sit near the origin, which is exactly what the old synthetic fixtures did.
//!
//! Chicago discipline: real tree-sitter JSON grammar, real build_tokens
//! pipeline, real splices, final-state asserts — no mocks.

use claude_code_config_lsp::semantic_tokens::{build_tokens, Document};
use claude_code_config_lsp::tokens_delta::{decode, semantic_tokens_delta, SLOTS_PER_TOKEN};
use lsp_max::lsp_types::SemanticToken;

fn doc(src: &str) -> Document {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .expect("grammar");
    let tree = parser.parse(src, None).expect("parse");
    Document::new(src.to_string(), tree, None)
}

fn splice(
    prev: &[SemanticToken],
    d: &lsp_max::lsp_types::SemanticTokensDelta,
) -> Vec<SemanticToken> {
    let mut data = prev.to_vec();
    for e in &d.edits {
        let s = e.start as usize / SLOTS_PER_TOKEN;
        let end = s + e.delete_count as usize / SLOTS_PER_TOKEN;
        data.splice(s..end, e.data.clone().unwrap_or_default());
    }
    data
}

/// The client-side contract, at absolute coordinates: splicing the delta into
/// the cached array and DECODING the result must reproduce the fresh build's
/// absolute tokens. This is the law the (0,0) seed violated — a wrong seed
/// keeps slot counts identical but corrupts every decoded position downstream.
fn assert_reconstructs(v1: &str, v2: &str) {
    let t1 = build_tokens(&doc(v1));
    let t2 = build_tokens(&doc(v2));
    let d = semantic_tokens_delta(&t1, &t2, 42);
    let spliced = splice(&t1.data, &d);
    assert_eq!(
        decode(&spliced),
        decode(&t2.data),
        "spliced prev must decode to next's absolute tokens: v1={v1:?} v2={v2:?}"
    );
}

/// Pinned regression for the I3-witnessed pair: `{"model": "opus"}` →
/// `{"model": "sonnet"}`. The edit inserts ONE token whose delta_start must be
/// relative to the preceding property token's column, not absolute-zero.
/// Asserted against the value the LSP continuation semantics require,
/// computed from the real arrays — not from a hand-copied constant.
#[test]
fn i3_witnessed_pair_continues_encoding_from_boundary_token() {
    let v1 = r#"{"model": "opus"}"#;
    let v2 = r#"{"model": "sonnet"}"#;
    let t1 = build_tokens(&doc(v1));
    let t2 = build_tokens(&doc(v2));
    assert!(
        t1.data.len() >= 2,
        "real substrate must emit the key+value tokens"
    );

    let prev_abs = decode(&t1.data);
    let next_abs = decode(&t2.data);
    // Common prefix = 1 (key token unchanged); the edit region starts at
    // index 1, so continuation seeds from prev token 0's column.
    let boundary = &prev_abs[0];
    let expected_delta_line = next_abs[1].line - boundary.line;
    let expected_delta_start = if expected_delta_line > 0 {
        next_abs[1].start_char
    } else {
        next_abs[1].start_char - boundary.start_char
    };

    let d = semantic_tokens_delta(&t1, &t2, 1);
    assert_eq!(d.edits.len(), 1, "one changed value => exactly one edit");
    let data = d.edits[0].data.as_deref().expect("edit carries data");
    assert_eq!(data.len(), 1, "one replacement token");
    assert_eq!(data[0].delta_line, expected_delta_line, "continuation dl");
    assert_eq!(
        data[0].delta_start, expected_delta_start,
        "continuation ds: relative to the boundary token's column, not origin \
         (I3 witnessed [dl0, ds9] where continuation requires the boundary-\
         relative value)"
    );
    assert_eq!(splice(&t1.data, &d), t2.data, "slot-exact reconstruct");
}

/// Second I3 witness: a pretty-printed (multi-line) document. Origin seeding
/// emitted `[dl1, ds11]`; continuation from the same-line boundary token
/// requires `[dl0, ds11]`.
#[test]
fn pretty_printed_one_key_edit_continues_from_same_line_boundary() {
    let v1 = "{\n  \"model\": \"opus\"\n}";
    let v2 = "{\n  \"model\": \"sonnet\"\n}";
    let t1 = build_tokens(&doc(v1));
    let t2 = build_tokens(&doc(v2));
    let prev_abs = decode(&t1.data);
    // Key token and value token share line 1 in the pretty-printed shape.
    assert_eq!(prev_abs[1].line, prev_abs[0].line, "fixture shape");
    let d = semantic_tokens_delta(&t1, &t2, 2);
    let data = d.edits[0].data.as_deref().expect("edit carries data");
    assert_eq!(
        data[0].delta_line, 0,
        "value token is on the same line as the boundary key token — an \
         absolute-zero seed emits dl1 here (I3 multi-line witness)"
    );
    assert_eq!(decode(&splice(&t1.data, &d)), decode(&t2.data));
}

/// Edit at array start: with no preceding token, the (0,0) seed is the
/// LEGITIMATE continuation — the first token's deltas are relative to origin.
#[test]
fn edit_at_array_start_keeps_absolute_zero_seed() {
    let v1 = r#"{"m": "opus"}"#;
    let v2 = r#"{"theme": "opus"}"#;
    let t1 = build_tokens(&doc(v1));
    let t2 = build_tokens(&doc(v2));
    let prev_abs = decode(&t1.data);
    let next_abs = decode(&t2.data);
    // Key token lengths differ (`"m"`=3 vs `"theme"`=7) so the common
    // prefix trims to 0 and the edit region starts at array index 0.
    assert_ne!(
        prev_abs[0], next_abs[0],
        "fixture: first token must differ so the edit starts at index 0"
    );

    let d = semantic_tokens_delta(&t1, &t2, 3);
    let data = d.edits[0].data.as_deref().expect("edit carries data");
    assert_eq!(data[0].delta_line, next_abs[0].line);
    assert_eq!(
        data[0].delta_start, next_abs[0].start_char,
        "edit at array start: first inserted token encodes relative to origin"
    );
    assert_eq!(decode(&splice(&t1.data, &d)), decode(&t2.data));
}

/// Append AFTER a token deep in the array: the old (0,0) seed corrupted the
/// inserted token's line delta (dl = absolute line instead of relative line),
/// silently shifting every decoded position after the splice.
#[test]
fn append_after_late_token_continues_from_last_cached_token() {
    let v1 = "{\n  \"a\": \"one\",\n  \"b\": \"two\",\n  \"c\": \"three\"\n}";
    let v2 = "{\n  \"a\": \"one\",\n  \"b\": \"two\",\n  \"c\": \"three\",\n  \"d\": \"four\"\n}";
    let t1 = build_tokens(&doc(v1));
    let t2 = build_tokens(&doc(v2));
    let prev_abs = decode(&t1.data);
    let next_abs = decode(&t2.data);

    let d = semantic_tokens_delta(&t1, &t2, 4);
    assert_eq!(d.edits.len(), 1);
    assert_eq!(d.edits[0].delete_count, 0, "append deletes nothing");
    let data = d.edits[0].data.as_deref().expect("edit carries data");
    let boundary = prev_abs.last().expect("prev nonempty");
    let expected_dl = next_abs[prev_abs.len()].line - boundary.line;
    assert_eq!(
        data[0].delta_line, expected_dl,
        "inserted token's line delta is relative to the LAST cached token, \
         not to the array origin"
    );
    assert_eq!(decode(&splice(&t1.data, &d)), decode(&t2.data));
}

/// The general law over a spread of real one-token edits, including edits
/// whose boundary tokens sit far from the origin (where seeding coincidences
/// cannot mask a wrong seed).
#[test]
fn real_edits_reconstruct_next_across_boundary_positions() {
    let pairs: [(&str, &str); 5] = [
        (r#"{"model": "opus"}"#, r#"{"model": "sonnet"}"#),
        (
            r#"{"model": "opus"}"#,
            r#"{"model": "opus", "theme": "dark"}"#,
        ),
        (r#"{"a": 1, "b": 2}"#, r#"{"a": 1}"#),
        (
            "{\n  \"statusLine\": {\n    \"padding\": \"#ff8800\"\n  }\n}",
            "{\n  \"statusLine\": {\n    \"padding\": \"#00ff00\"\n  }\n}",
        ),
        (
            r#"{"permissions": {"allow": ["Bash(ls:*)"]}}"#,
            r#"{"permissions": {"allow": ["Bash(cat:*)"]}}"#,
        ),
    ];
    for (v1, v2) in pairs {
        assert_reconstructs(v1, v2);
    }
}
