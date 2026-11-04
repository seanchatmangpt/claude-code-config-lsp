# P1 Navigation Lane Receipt (v26.11.3)

**Lane**: IMPLEMENTATION (navigation) — repo `/Users/sac/claude-code-config-lsp`.
**Owned files**: `src/navigation.rs` (new), `tests/navigation.rs` (new),
`tests/fixtures/navigation/` (new, self-authored since C5's corpus was present but lacked
hook-script fixtures).

## Delivered

`src/navigation.rs` — pure functions over `(text, position, uri)`, no backend state:

- `find_wikilinks`, `wikilink_at_position` — `[[stem]]` scan; composite `[[a]]/[[b]]` and
  parenthetical `[[stem]] (SPR)` annotations parse; malformed/unclosed/empty brackets skipped.
- `resolve_stem(stem, roots)` — recursive `stem.md` lookup under corpus roots, first match in
  root order, depth-capped at 8, unreadable dirs tolerated.
- `wikilink_definition(uri, text, position, roots) -> Option<Location>` — target `file://`
  URI + H1 range (falls back to line 0 when target has no H1).
- `wikilink_references(docs, target, roots) -> Vec<Location>` — reverse map over an open doc
  set; unparseable doc URIs skipped.
- `wikilink_highlight(uri, text, position, roots) -> Vec<Location>` — all ranges of the same
  resolved target in the doc.
- `find_hook_path_refs` / `hook_path_definition(uri, text, position, base_dir)` — relative
  `hooks/foo.sh`-style script paths resolve relative to the doc dir; `~/` and `/` forms
  expand and resolve as-is; missing file -> `None`.
- `position_to_offset` / `offset_to_position` — UTF-16-safe, clamped to char boundaries.

Tolerance law: every unresolvable input returns `None`/empty — no panic paths on external
input; `.unwrap()`/`.expect()` appear only in test code against fixture files.

## Wiring points for the integration wave (none executed by this lane)

1. `src/lib.rs` already carries `pub mod navigation;` (one-line registration — minimal
   wiring edit this lane made, disclosed).
2. `src/backend.rs` `textDocument/definition` handler: call
   `navigation::wikilink_definition(uri, text, position, &navigation::default_corpus_roots())`
   when `classify_uri(uri)` is a markdown class, falling through to hook-path resolution
   (`hook_path_definition` with the doc's on-disk parent dir) when the wikilink path returns
   `None`.
3. `src/backend.rs` `textDocument/references`: build `docs: Vec<(String,String)>` from the
   open-doc store and call `navigation::wikilink_references(&docs, &target_path, &roots)`.
4. `src/backend.rs` `textDocument/documentHighlight`: call
   `navigation::wikilink_highlight(...)`.
5. `src/capabilities.rs`: advertise `definition_provider`, `references_provider`,
   `document_highlight_provider` (Some/`DocumentHighlightOptions`) when wiring lands.

## Verification

- Gate (this session, final): `cargo test --test navigation` ->
  `test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured` (exit 0).
  Total runtime < 1s. Earlier in-session failures were two of this lane's own test
  expectations (fixture link count was 19 occurrences not 9 lines; stem length is 18 chars
  not 19) — fixed, not waived; the implementation never failed.
- Shared-lib compile freezes from concurrent lanes (`tests/workspace_symbols.rs` line 27
  syntax error, `src/formatting.rs:104` usize==String) were NOT edited by this lane; gate
  scoped to `--test navigation` per fan-out law.

## Note on shared fixtures

`tests/fixtures/claude-config/` (C5's corpus) was present and used read-only: the shared
`claude-global-CLAUDE.md` fixture contains 9 wikilink-bearing lines (grep-counted), used as
the real-corpus oracle in `scans_wikilinks_in_shared_corpus_fixture`.
