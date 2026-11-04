//! Semantic tokens delta (`textDocument/semanticTokens/full/delta`).
//!
//! Diffs a freshly built full token array against a cached previous result and
//! produces `SemanticTokensEdit[]` per the LSP delta spec. Edit `start` /
//! `deleteCount` are in token-slot units (5 data integers per token), which is
//! what the client splices into its cached `data` array.
//!
//! The diff operates on absolute (line, startChar, length, type) quads decoded
//! from both arrays — never on raw wire deltas — so an insertion that shifts
//! every token on subsequent lines still diffs as a small local edit, not a
//! full re-emit.

use lsp_max::lsp_types::{SemanticToken, SemanticTokens, SemanticTokensEdit};
// SLA-UNBLOCK (R2 multiplex lane, disclosed per same-checkout-fanout): admission.rs
// re-imports SemanticTokensDelta; the private import made the release build fail
// with E0603. Re-exported publicly so the shared build unfreezes.
pub use lsp_max::lsp_types::SemanticTokensDelta;

/// Slots per token on the wire (deltaLine, deltaStart, length, tokenType,
/// modifiers bitset).
pub const SLOTS_PER_TOKEN: usize = 5;

/// A token decoded back to absolute coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbsToken {
    pub line: u32,
    pub start_char: u32,
    pub length: u32,
    pub token_type: u32,
}

/// Decode a delta-encoded token array back to absolute tokens.
pub fn decode(data: &[SemanticToken]) -> Vec<AbsToken> {
    let mut out = Vec::with_capacity(data.len());
    let mut line = 0u32;
    let mut ch = 0u32;
    for t in data {
        line += t.delta_line;
        ch = if t.delta_line > 0 {
            t.delta_start
        } else {
            ch + t.delta_start
        };
        out.push(AbsToken {
            line,
            start_char: ch,
            length: t.length,
            token_type: t.token_type,
        });
    }
    out
}

/// Delta-encode absolute tokens into wire form, continuing the encoding from
/// the absolute `(line, start_char)` position of the LAST token emitted before
/// this slice in the full array (`seed_line`/`seed_ch`).
///
/// LSP delta semantics: the `data` of a `SemanticTokensEdit` is not an
/// independent encoding — the client splices it into its cached array and
/// decodes the result as one continuous stream, so the first inserted token's
/// `delta_line`/`delta_start` must be relative to the token immediately before
/// the edit region in the *previous* array. Seeding at `(0, 0)` is lawful only
/// when the edit begins at array index 0 (no preceding token exists).
///
/// (Fixes the boundary defect witnessed in
/// docs/jira/v26.11.3/receipts/i3-tokens-substrate.md: `{"model": "opus"}` →
/// `{"model": "sonnet"}` produced edit data `[dl0, ds9]` where continuation
/// requires `[dl0, ds8]` — an absolute-zero seed that dropped the preceding
/// key token's column offset.)
fn encode_abs(tokens: &[AbsToken], seed_line: u32, seed_ch: u32) -> Vec<SemanticToken> {
    let mut out = Vec::with_capacity(tokens.len());
    let mut line = seed_line;
    let mut ch = seed_ch;
    for t in tokens {
        let delta_line = t.line - line;
        let delta_start = if delta_line > 0 {
            t.start_char
        } else {
            t.start_char - ch
        };
        out.push(SemanticToken {
            delta_line,
            delta_start,
            length: t.length,
            token_type: t.token_type,
            token_modifiers_bitset: 0,
        });
        line = t.line;
        ch = t.start_char;
    }
    out
}

/// Compute the delta transforming cached `prev` into freshly built `next`.
///
/// Algorithm: trim the common prefix, trim the common suffix, then emit one
/// replacement edit covering the differing middle (delete the removed slots,
/// insert the new middle tokens). Identical arrays produce zero edits; a
/// one-token change in an otherwise-identical document produces exactly one
/// small edit instead of a full re-emit. Appends produce a pure-insert edit
/// (`deleteCount: 0`); removals produce a pure-delete edit (`data: []`).
///
/// `result_id` is stamped on the response so the client can chain the next
/// delta request against this result.
pub fn semantic_tokens_delta(
    prev: &SemanticTokens,
    next: &SemanticTokens,
    result_id: u64,
) -> SemanticTokensDelta {
    let prev_abs = decode(&prev.data);
    let next_abs = decode(&next.data);
    let n = prev_abs.len();
    let m = next_abs.len();

    // Common prefix length.
    let mut p = 0usize;
    while p < n && p < m && prev_abs[p] == next_abs[p] {
        p += 1;
    }
    // Common suffix length (not overlapping the prefix).
    let mut s = 0usize;
    while s < (n - p).min(m - p) && prev_abs[n - 1 - s] == next_abs[m - 1 - s] {
        s += 1;
    }

    let mut edits: Vec<SemanticTokensEdit> = Vec::new();
    if p + s == n && p + s == m {
        // Fully identical — no edits.
        return SemanticTokensDelta {
            result_id: Some(result_id.to_string()),
            edits,
        };
    }

    let prev_mid = &prev_abs[p..n - s];
    let next_mid = &next_abs[p..m - s];
    // The inserted data continues the array's delta stream from the token
    // immediately before the edit region (absolute-zero seed only when the
    // edit starts at index 0 — see encode_abs).
    let (seed_line, seed_ch) = if p > 0 {
        let b = &prev_abs[p - 1];
        (b.line, b.start_char)
    } else {
        (0, 0)
    };
    edits.push(SemanticTokensEdit {
        start: (p * SLOTS_PER_TOKEN) as u32,
        delete_count: (prev_mid.len() * SLOTS_PER_TOKEN) as u32,
        data: Some(encode_abs(next_mid, seed_line, seed_ch)),
    });

    SemanticTokensDelta {
        result_id: Some(result_id.to_string()),
        edits,
    }
}

#[cfg(test)]
mod witness {
    use super::*;

    fn tok(line: u32, ch: u32, len: u32, tt: u32) -> SemanticToken {
        SemanticToken {
            delta_line: line,
            delta_start: ch,
            length: len,
            token_type: tt,
            token_modifiers_bitset: 0,
        }
    }

    fn abs(line: u32, ch: u32, len: u32, tt: u32) -> AbsToken {
        AbsToken {
            line,
            start_char: ch,
            length: len,
            token_type: tt,
        }
    }

    #[test]
    fn decode_roundtrips_absolute_coordinates() {
        // (0,0)-(1,2)-(1,5) encoded as deltas decodes to the same absolutes.
        let data = [tok(0, 0, 3, 4), tok(1, 2, 2, 1), tok(0, 3, 4, 4)];
        let decoded = decode(&data);
        assert_eq!(
            decoded,
            vec![abs(0, 0, 3, 4), abs(1, 2, 2, 1), abs(1, 5, 4, 4)]
        );
    }

    #[test]
    fn identical_arrays_produce_zero_edits() {
        let a = SemanticTokens {
            result_id: None,
            data: vec![tok(0, 0, 3, 4), tok(0, 5, 2, 1)],
        };
        let b = SemanticTokens {
            result_id: None,
            data: vec![tok(0, 0, 3, 4), tok(0, 5, 2, 1)],
        };
        let d = semantic_tokens_delta(&a, &b, 7);
        assert!(
            d.edits.is_empty(),
            "identical token arrays must yield no edits"
        );
        assert_eq!(d.result_id.as_deref(), Some("7"));
    }

    #[test]
    fn change_one_mid_token_produces_one_small_edit() {
        // Three tokens; the middle one changes length. The third sits on the
        // next line so it is untouched by the change (pretty-printed JSON
        // shape) and the suffix survives.
        let a = SemanticTokens {
            result_id: None,
            data: vec![tok(0, 0, 5, 3), tok(0, 7, 4, 4), tok(1, 2, 6, 4)],
        };
        let b = SemanticTokens {
            result_id: None,
            data: vec![tok(0, 0, 5, 3), tok(0, 7, 9, 1), tok(1, 2, 6, 4)],
        };
        let d = semantic_tokens_delta(&a, &b, 1);
        assert_eq!(d.edits.len(), 1, "one changed token => one edit");
        let e = &d.edits[0];
        assert_eq!(
            e.start, SLOTS_PER_TOKEN as u32,
            "edit starts at token slot 1"
        );
        assert_eq!(
            e.delete_count, SLOTS_PER_TOKEN as u32,
            "one token's worth deleted"
        );
        assert_eq!(
            e.data.as_deref().unwrap().len(),
            1,
            "one replacement token inserted"
        );
    }

    #[test]
    fn append_produces_pure_insert_edit() {
        let a = SemanticTokens {
            result_id: None,
            data: vec![tok(0, 0, 5, 3)],
        };
        let b = SemanticTokens {
            result_id: None,
            data: vec![tok(0, 0, 5, 3), tok(1, 0, 2, 4)],
        };
        let d = semantic_tokens_delta(&a, &b, 2);
        assert_eq!(d.edits.len(), 1);
        assert_eq!(d.edits[0].delete_count, 0, "append deletes nothing");
        assert_eq!(d.edits[0].start, SLOTS_PER_TOKEN as u32);
        assert_eq!(d.edits[0].data.as_deref().unwrap().len(), 1);
    }

    #[test]
    fn removal_produces_pure_delete_edit() {
        // Middle token removed; a trailing token on a later line survives so
        // the suffix trim isolates exactly one removed token.
        let a = SemanticTokens {
            result_id: None,
            data: vec![tok(0, 0, 5, 3), tok(0, 7, 2, 4), tok(1, 2, 3, 1)],
        };
        let b = SemanticTokens {
            result_id: None,
            data: vec![tok(0, 0, 5, 3), tok(1, 2, 3, 1)],
        };
        let d = semantic_tokens_delta(&a, &b, 3);
        assert_eq!(d.edits.len(), 1);
        assert!(
            d.edits[0].data.as_deref().unwrap().is_empty(),
            "pure delete carries no data"
        );
        assert_eq!(d.edits[0].delete_count, SLOTS_PER_TOKEN as u32);
    }

    #[test]
    fn reconstructing_prev_from_edits_equals_next() {
        // Splice the edit into prev.data and confirm it reproduces next.data
        // exactly — the client-side view of the contract.
        let a = SemanticTokens {
            result_id: None,
            data: vec![
                tok(0, 0, 5, 3),
                tok(1, 0, 2, 4),
                tok(0, 4, 3, 1),
                tok(1, 1, 7, 5),
            ],
        };
        let b = SemanticTokens {
            result_id: None,
            data: vec![
                tok(0, 0, 5, 3),
                tok(0, 7, 3, 1),
                tok(1, 0, 2, 4),
                tok(1, 1, 7, 5),
                tok(2, 0, 1, 0),
            ],
        };
        let d = semantic_tokens_delta(&a, &b, 9);
        let mut spliced = a.data.clone();
        for e in &d.edits {
            let start = e.start as usize / SLOTS_PER_TOKEN;
            let end = start + e.delete_count as usize / SLOTS_PER_TOKEN;
            spliced.splice(start..end, e.data.clone().into_iter().flatten());
        }
        assert_eq!(spliced, b.data, "spliced prev must equal next");
    }
}
