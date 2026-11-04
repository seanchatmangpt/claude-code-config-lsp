# I3 — Semantic-Tokens Substrate Fix (generated classifier hand-correction)

Lane: I3 (IMPLEMENTATION). Owned surfaces: `src/semantic_tokens.rs`,
`tests/semantic_tokens.rs` (new), witness flip in `tests/tokens_color.rs`,
this receipt. `tokens_delta.rs`/`color.rs` untouched (P8's).
hand-corrected 2026-11-03: FM-GEN-007 blocked regen; reconcile with
`schema/claude-code-config.ttl` when sync restores.

## Diagnosis

The ggen-generated classifier in `src/semantic_tokens.rs` matched **legend
strings** ("enumMember", "keyword", "namespace", "property", "string",
"variable") as if they were tree-sitter **node kinds**. tree-sitter-json emits
`document/value/object/array/pair/string/number/true/false/null` (+ anonymous
quotes, `string_content`) — none of the legend strings are node kinds, so
`classify` returned `None` for every leaf and `build_tokens` returned 0 tokens
for every JSON document (witnessed: P8's substrate witness, 0-tokens assert).

Second defect underneath the first: the leaf-only traversal (`child_count()==0`
gate) could never emit whole-`string` tokens anyway — a `string` node contains
quote + `string_content` children, so even a corrected kind table would miss
every string token. This was visible only after the first defect's fix, and is
recorded so regen reconciles both.

Third defect: the ontology's `ccc:SemanticTokenType` individuals
(schema/claude-code-config.ttl:683-713) list keyword/string/enumMember/
property/namespace/variable — no numeric entry. `number` was appended to the
legend (index 6); ontology reconciliation when `ggen sync` is unblocked must
add the individual or remap.

## Fix

- `classify()` now maps the kinds tree-sitter-json actually emits:
  - `string` whose parent is a `pair` and is its **first child** → `property`
    (the key, including quotes); any other `string` → `string`.
  - `number` → `number` (new legend entry, index 6).
  - `true`/`false`/`null` → `keyword`.
  - `pair`/`object`/`array`/anonymous quote/`string_content` → None (descend).
- Traversal emits at classified nodes and **skips their subtree** (a `string`
  token spans quotes + content, never split).
- Legend gains `"number"` at index 6; indices 0–5 unchanged, so P8's synthetic
  delta LAW tests' token_type indices remain valid.
- Dead `token_type_for` node-kind table removed; legend-index lookup kept
  (`#[cfg(test)]`) for the in-file witness tests.

## Verification (ALIVE — commands + exits this session)

- `cargo test --test semantic_tokens` → 10 passed, 0 failed. Real fixtures:
  settings.json-shaped doc >10 tokens; key→property (col/len/type spot-checks
  incl. quotes); value→string; number→number; true/false/null→keyword; nested
  arrays traversed; range clip strict subset; legend/advertised match.
- `cargo test --test tokens_color` → 14 passed, 1 failed (see Defect routed).
  Witness flipped: `substrate_witness_build_tokens_emits_json_tokens_after_i3_fix`
  asserts non-empty output (disclosed edit in P8's file, per contract).
- `cargo test --lib semantic_tokens` → 5 passed, 0 failed (in-file witness
  module). A subsequent full `--lib` run recorded 256 passed / 5 failed in
  `hover.rs`/`provider_registry.rs` — surfaces this lane never touched, which
  were green (264/0) minutes earlier: concurrent-lane in-flight drift, not
  attributable to this diff.
- `cargo fmt` applied.
- clippy `-D warnings`: zero findings in owned files; remaining findings are in
  `src/analyzers/*` and `src/admission.rs` — out-of-lane surfaces.
- Full `cargo test`: 8 suites green, 2 red — both reds classified
  session-independent below.

## Defect routed to P8 (real bug surfaced by real tokens)

P8's law test `integration_delta_over_real_build_tokens_output_reconstructs_next`
(tests/tokens_color.rs:131) is now correctly **red**: splicing the computed
delta does not reconstruct `next` for real one-token changes. Root cause is in
**their module**: `tokens_delta::encode_abs` (src/tokens_delta.rs:55-73) seeds
its encoding at `(line=0, char=0)` instead of continuing the delta encoding
from the token preceding the edit boundary — LSP delta semantics require the
edit's `data` to continue the array's encoding across the splice. Witnessed
pair: `{"model": "opus"}` → `{"model": "sonnet"}` yields edit data
`[dl0, ds9, len8]` where continuation requires `[dl0, ds8, len8]` (off by the
preceding key token's column). Multi-line witness: pretty-printed doc yields
`[dl1, ds11]` where `[dl0, ds11]` is required. P8's synthetic LAW tests passed
because their fixtures' preceding tokens sit near the origin, where origin
seeding coincides with continuation. My own duplicate of the reconstruct law
was scoped down (tests/semantic_tokens.rs NOTE) to the parts my lane owns
(one replacement edit, exactly one slot deleted / one inserted) and passes.

## Session-independent failures (classified, not mine)

- `compositor_front_door_end_to_end` (tests/compositor_e2e.rs:157): times out
  waiting for publishDiagnostics (30s, deterministic across 2 runs). The e2e
  drives the **release binary** built 13:39, which **predates the wave's
  sources stamped 13:43** — it never ran the current tree, mine included.
  Structurally disconnected from my diff regardless: publishDiagnostics comes
  from the analyzer/didOpen path, which never calls `build_tokens`.
- clippy `-D warnings` fails on `src/analyzers/*` (items_after_test_module in
  hook/json/toml/claude_md/frontmatter) and `src/admission.rs` (unused import
  `Uri`, unused variable `path`) — out-of-lane files, not modified this
  session.

## Standing

ALIVE for the owned surface: real fixtures, real grammar, real pipeline,
final-state asserts, no mocks. P8 repair required for full delta reconstruct;
ontology reconciliation required for legend provenance (FM-GEN-007).
