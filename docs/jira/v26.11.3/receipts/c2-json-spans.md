# C2: JSON analyzer real spans (de-sentinel)

- **Lane**: C2 (IMPLEMENTATION). Owned files: `src/analyzers/json.rs`, `src/quickfix.rs` (span-dependency doc update only).
- **Status**: ALIVE (lib+test targets compile clean on my files; full `cargo test` green run pending — see Blocking note).

## Subject

`src/analyzers/json.rs` — all `(0, 0)` sentinel spans in the serde-based validators
(`validate_settings_json_enums`, `validate_plugin_json`) replaced with real byte-offset spans
computed from the tree-sitter-json parse tree.

## Diff

- `src/analyzers/json.rs`:
  - New pub helpers `ts_key_span(content, path)` and `ts_root_span(content)` — locate the offending key node in the
    tree-sitter-json parse tree by path walk (object keys by name, arrays by index, last-duplicate-key-wins
    to mirror serde_json). Byte offsets only; UTF-16 conversion stays in `backend.rs::span_to_range`
    (untouched).
  - ~13 `(0, 0)` sites rewired: CCC-JSON-004/005/006(×2 sites)/008/009(×3)/010/011/012/013 + CCC-HOOK-001/004.
  - CCC-JSON-007 (missing `$schema`) anchors at the root object node span.
  - 4 constants (`VALID_MODELS`, `VALID_EFFORT_LEVELS`, `VALID_HOOK_EVENTS`, `VALID_PERMISSION_MODES`) `const` → `pub const`.
  - 17 new tests (span assertions per rule + `ts_key_span` path-walk/duplicate-key/UTF-16 tests).
- `src/quickfix.rs`: doc-comment scope note updated (span-dependency update; message_prefix constants
  byte-identical, no logic change).

## Compile-freeze SLA disclosures

1. While polling the shared build, C5's mid-flight `completion.rs` referenced my four constants as
   private → E0603 froze the shared compile. Minimal unblock applied: `const` → `pub const` on the four
   constants in `src/analyzers/json.rs` (single owned-file edit, disclosed in both lanes' receipts
   per the disclosure law — C5 should name it in its receipt too).
2. Concurrent edit collision (both lanes pub'd the same constants → `pub pub const`): resolved by
   collapsing to `pub const`; final state on disk is the intended one. (Earlier drift note: a stray
   `ts ts_root_span` typo in this receipt's first bullet is receipt-text only, not code.)
3. Zero edits to existing `scan.rs` functions — helper lives in `src/analyzers/json.rs`, so scan.rs is
   untouched entirely.

## Verification ladder

- Baseline `cargo build` (pre-edit): exit 0, 52s.
- Post-edit `cargo test --lib`: **252 passed, 1 failed** — the sole failure is
  `harness_docs::tests::standing_fixture_partition` (`src/harness_docs.rs`, another lane's new module,
  failing before my last edits). All 54 `analyzers::json::tests` pass (incl. 17 new span tests);
  all 6 `quickfix::tests` pass. Zero clippy findings attributed to `src/analyzers/json.rs` /
  `src/quickfix.rs`.
- Full `cargo test` (all targets): BLOCKED by concurrent lanes — at final poll the only error was a
  half-landed `example/h1probe` (missing `main`) owned by another lane; error attribution never
  landed in json.rs/quickfix.rs across ~35 min of polling.

## Standing

- `src/analyzers/json.rs`: ALIVE (`cargo test --lib json::tests` 54/54, exit 0, this session).
- `src/quickfix.rs`: ALIVE (doc-only change; `message_prefix` constants byte-identical; 6/6 tests).
- Full-suite gate: PARTIAL — lib target green for my modules; workspace-wide green BLOCKED on other
  lanes' in-flight files (example/h1probe, harness_docs, standing.rs churn).
- scan.rs: untouched (0 edits — helper lives entirely in json.rs).
