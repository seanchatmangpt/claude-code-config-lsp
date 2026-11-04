# Receipt: m5-merge — FanAll diagnostics merge semantics

Lane: IMPLEMENTATION (M5). Repo: /Users/sac/lsp-max. Owned file only:
`/Users/sac/lsp-max/crates/lsp-max-compositor/tests/diagnostics_merge.rs` (new).
Compositor src untouched. No git commands run.

## Subject

Contract: test diagnostics merge semantics for the FanAll publish path
("responses and diagnostics from multiple child processes are merged into a
single event stream"). Drove the real merge code path —
`crates/lsp-max-compositor/src/merge.rs::merge_for_upstream` (the FanAll
publish merge, CC-005) and `MergeContext::merge` / `attribute_andon`
(per-server ANDON attribution) — with stub children as
`(server_id, Vec<Diagnostic>)` contributions, the exact shape the router
hands the merge after a flush. No mocks; real collaborators (real
`lsp_max::lsp_types::Diagnostic` values, real merge functions).

## Merge-order law (documented + asserted)

`merge_for_upstream` processes contributions in the ORDER GIVEN (child
priority order as supplied by the caller/router). Within a child, per-child
publish sequence is preserved (Vec insertion order). The merged Vec keeps
first-occurrence position; at a duplicate (range, code) key: ANDON-prefix
code beats non-ANDON, then lower severity number wins, equal severity →
first occurrence wins. No re-sorting within a child's own run.
`MergeContext::merge` (ConformanceVector-aware path) additionally sorts
law-errors first, then severity, uri, line/char.

## Coverage vs contract

1. Two children, same URI → both sets present:
   `two_children_same_uri_both_sets_survive_merge` — exact merged sequence
   asserted. Also dedup law at duplicate key:
   `two_children_same_uri_dedup_law_andon_beats_non_andon_then_severity`,
   `same_key_first_occurrence_wins_at_equal_severity`.
2. One child producing, other silent → single set passes through:
   `silent_child_passes_through_single_set_untouched` (silent child passed
   as real empty contribution).
3. andon_code_prefixes attribution survives merge:
   `andon_prefixes_stay_attributable_to_emitting_child` (CCC- vs ALPHA-,
   distinct prefixes, both survive with distinct codes); plus per-server
   isolation via `attribute_andon`:
   `andon_attribution_via_merge_context_is_per_server` — CCC-GATE under
   alpha-lsp's own prefix set is `AndonRoute::NotAndon` and vice versa;
   each child's ANDON classification uses only its own prefix set.
4. Per-child publish ordering preserved in merged publishes:
   `merged_publish_preserves_per_child_sequence` (5-entry exact sequence),
   `interleave_children_without_key_collision_preserves_both_runs`,
   `severity_mix_orders_law_errors_first_within_one_child` (no re-sorting
   within one child's run in merge_for_upstream).

## Verification ladder

- `cargo test -p lsp-max-compositor --test diagnostics_merge`
  → exit 0: 9 passed, 0 failed.
- Non-vacuity (shim-court style, real restore): mutated
  `merged.len(), 5` → `4` in a temp copy → same test FAILED (exit 1);
  restore byte-identical, sha256
  `20022c2b7e23bba27e36d834fc35e40dd46fac360617d4bba3f0aade21b72abd`
  before == after. First probe attempt (sed on the long assert string) did
  not match the file — discarded, never claimed as a mutation witness.
- Note: one probe ran while a concurrent lane held the cargo build lock;
  tests still executed after the wait.

## Standing

ALIVE — ran + passed this session, commands + exits cited above. Test
premises checked against real semantics: `merge_for_upstream` uses (range,
code) keys and preserves insertion order (src/merge.rs:508-577);
`MergeContext` per-server isolation via `server_automatons`
(src/merge.rs:439-497) witnessed by existing tests/speciation.rs.
