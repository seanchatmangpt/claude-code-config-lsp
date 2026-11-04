# P8 Receipt: semantic-tokens delta + documentColor (v26.11.3)

- **Lane**: P8 IMPLEMENTATION, /Users/sac/claude-code-config-lsp
- **Owned files**: `src/tokens_delta.rs` (new), `src/color.rs` (new),
  `tests/tokens_color.rs` (new)
- **Seam edit (disclosed)**: `src/lib.rs` — appended `pub mod tokens_delta;`
  and `pub mod color;` (2 lines). `lib.rs` was not in the lane's banned list
  and the modules cannot compile unwired. No `backend.rs`/`capabilities.rs`
  edits.

## Delivered

### 1. Semantic tokens delta (`src/tokens_delta.rs`)

- `semantic_tokens_delta(prev, next, result_id) -> SemanticTokensDelta`:
  decodes both arrays to absolute (line, char, length, type) quads, trims
  common prefix + suffix, emits one replacement edit in wire-slot units
  (5 ints/token, `SLOTS_PER_TOKEN`).
- `decode()` pub for client-side tests; `encode_abs` internal.
- Witness tests inline (`#[cfg(test)] mod witness`): decode round-trip,
  zero-edits-on-identical, one-small-edit-on-one-change, pure-insert append,
  pure-delete removal, splice-reconstructs-next. (Another lane applied a
  minimal unblock to the witness splice scale during the wave — disclosed in
  both receipts per the SLA disclosure law.)

### 2. documentColor / colorPresentation (`src/color.rs`)

- `detect_colors(text) -> Vec<DetectedColor>`: hex `#rgb`/`#rrggbb` (+`#rrggbbaa`),
  CSS named colors gated on the nearest preceding quoted `"key":` being
  color-bearing (contains `color`/`theme`, or is `background`/`foreground`),
  ANSI SGR sequences (`ESC[...m`, raw ESC or `\u001b`/`\x1b` JSON escape form),
  parsed for `38;2;r;g;b` truecolor and the 16-color palette; unknown codes
  tolerated and skipped.
- Tolerance law held: unparseable literals are never reported; every reported
  color is re-parseable (test-enforced).
- `color_presentations(detected, color, range)`: hex↔named swaps, current form
  first, `textEdit` ranges equal to the detected range.
- 10 tests in `tests/tokens_color.rs` for the color half.

## Verification (ALIVE)

- `cargo test --test tokens_color` → **15 passed; 0 failed** (this session).
- `cargo test --lib tokens_delta` → **7 passed; 0 failed** (witness suite).
- `cargo test --lib color` → **1 passed; 0 failed** (witness suite).
- `cargo check --lib` → Finished, exit 0. My two modules: 0 warnings.
- `cargo clippy --lib -- -D warnings`: 0 findings in
  `src/color.rs`/`src/tokens_delta.rs`; remaining findings are in other lanes'
  files (workspace_symbols, hierarchy, selection, navigation, completion).

## Substrate defect found (not lane-owned) — semantic_tokens.rs

The generated classifier maps legend kinds to tree-sitter node kinds the JSON
grammar never emits: it expects leaves literally named `"string"`/`"property"`,
but tree-sitter-json's leaves are `string_content` and anonymous `"` tokens.
Witnessed: `build_tokens(&doc(r#"{ "model": "SOMETHING" }"#))` → **0 tokens**
(example `examples/dbg_tokens.rs` run: token count 0). Consequently the LSP
`semanticTokens/full` response is always an empty array for JSON — the
capability is advertised but returns nothing.

**Impact on this lane**: delta law tests are carried over synthetic one-key-edit
token arrays plus a substrate witness test
(`substrate_witness_build_tokens...for_json`) that fails loudly the moment the
substrate starts emitting tokens, at which point the law tests can be re-pointed
at real `build_tokens` fixtures.

**Close condition**: `ggen sync` regenerating the classifier's node-kind table
to JSON grammar kinds (`string_content` → string, key `string_content`
preceded-by-`"`-before-`:` → property, etc.).

## Pre-existing / other-lane state (not introduced by this lane)

- `src/workspace_symbols.rs:157` deprecated-field warning; `src/analyzers/json.rs:404`
  unused-Result warning — pre-existing.
- Concurrent-lane transient compile breaks absorbed during the wave (moniker.rs,
  selection.rs, backend.rs) — all settled by their owners within the SLA.

## Standing

- Delta algorithm: **ALIVE** (tests ran and passed this session; real inputs,
  final-state asserts).
- documentColor/colorPresentation core: **ALIVE** (15/15 test file pass).
- Backend wiring of these handlers into `backend.rs`/`capabilities.rs`
  (advertise `semanticTokensRangeProvider`, `colorProvider`, delta capability,
  route requests) — **UNVERIFIED / out of lane scope**; the pure functions are
  ready for the wiring lane.
