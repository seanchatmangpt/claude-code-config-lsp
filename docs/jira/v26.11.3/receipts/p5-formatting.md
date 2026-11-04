# Receipt: P5 — document/range/on-type formatting for JSON config surfaces

Lane: P5 (IMPLEMENTATION). Owned files: `src/formatting.rs` (new), `tests/formatting.rs` (new).
Disclosed edits outside owned set:
- `Cargo.toml`: `serde_json = "1.0"` → `serde_json = { version = "1.0", features = ["preserve_order"] }` (contract-authorized, the one permitted addition).
- `src/lib.rs`: one line, `pub mod formatting;` (module registration, required to expose the module; other lanes were concurrently adding their own module lines to the same file).

## Law (module doc, src/formatting.rs header)

- Key order is PRESERVED as the user wrote it — `settings.local.json` overlays depend on the
  author's grouping. Enforced structurally by serde_json `preserve_order` (IndexMap-backed
  Map), never by sorting. Never reorder keys.
- Invalid JSON is never reformatted — parse failure ⇒ `None`, all three entry points.
- Idempotence: `format(format(x)) == format(x)`, enforced by test.

## Surface

- `format_document(text, range) -> Option<Vec<TextEdit>>` — whole-document 2-space pretty
  print as a single full-range TextEdit; `None` on parse failure.
- `format_range(text, range, options) -> Option<Vec<TextEdit>>` — parses the whole document
  (tolerance law applies globally), slices the requested line range out of the formatted
  output; in-bounds only, `None` on parse failure or out-of-bounds.
- `on_type_format(text, position, trigger) -> Option<Vec<TextEdit>>` — triggers `"` / `:`
  only; returns a minimal indentation TextEdit for the current line when the raw text's
  indent depth disagrees with the structural depth from brace scanning, `None` on correct
  indent, non-trigger chars, or invalid JSON.

## Verification (commands + exits, this session)

- `cargo test --test formatting` — **15 passed; 0 failed** (exit 0), ran after all edits.
  Covers: 2-space pretty print exact-output, key-preservation (top-level + nested, incl.
  deny-before-allow and Z-before-A orderings), invalid-JSON tolerance (6 malformed cases),
  idempotence (fixed point over settings-shaped input + 6 varied valid shapes), range
  slicing (edit range == requested range, out-of-range keys absent from slice), out-of-bounds
  → None, on-type triggers/no-op/indent correction (over- and under-indent), invalid-JSON
  refusal on trigger chars, nested depth exact output, empty object/array inline, seam-type
  guard.
- `cargo check` — 0 errors after final edits (lib compiles clean with formatting.rs wired).
- Full-suite context (shared tree, concurrent lanes): an earlier full run (before other
  lanes' later in-flight edits) showed **192 passed; 2 failed** — the 2 failures were
  `analyzers::json::tests::span_ccc_json_007_points_at_root_object` and
  `analyzers::json::tests::ts_key_span_walks_nested_paths_and_arrays`, both in
  `src/analyzers/json.rs` — NOT lane P5's files. Cause classification UNVERIFIED: the lib
  test target currently does not compile due to other lanes' in-flight test-cfg code
  (`src/tokens_delta.rs` E0271/E0599/E0624, `src/workspace_symbols.rs:157`), so the two
  failures cannot be re-run to completion on the shared tree right now. Neither failing
  test's code path touches serde_json Map ordering, and `preserve_order` only changes
  iteration order of serde_json maps, so a P5-induced cause is unlikely but not ruled out.
  Flagged for the owning lane / coordinator.

## Falsifiers (all exercised by tests)

- Key reordering would fail `key_order_preserved_as_written` (deny<allow, Z<A preserved).
- Non-idempotent formatter would fail `idempotence_format_of_format_is_fixed_point`.
- Reformatting invalid JSON would fail `invalid_json_is_never_reformatted` (6 cases).
- A slicing bug would fail `range_format_slices_affected_lines_only`.
