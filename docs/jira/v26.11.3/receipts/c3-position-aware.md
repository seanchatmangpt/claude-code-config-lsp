# Receipt: C3 — position-aware completion/hover (claude-code-config-lsp)

Lane: IMPLEMENTATION (owns src/completion.rs, src/hover.rs, src/schema.rs, completion/hover handlers in src/backend.rs)
Status: PARTIAL - my scope compiles + tests green; full `cargo test` blocked by concurrent lanes' in-flight files I do not own (see Shared-checkout note).

## Subject
- Repo: /Users/sac/claude-code-config-lsp (shared checkout, no git run by this lane)

## Diff (owned files)
- src/completion.rs: rewritten position-aware. New `CursorContext` enum
  (Key{key,parent_key,siblings,top_level} / Value{key} / Unknown),
  `position_to_offset` (UTF-16 col -> byte offset), `resolve_cursor_context`
  (tree-sitter ancestor walk from Position -> byte offset), enum-value
  completion for model/effortLevel/permissionMode(+defaultMode)/hooks events
  importing `VALID_MODELS`/`VALID_EFFORT_LEVELS`/`VALID_PERMISSION_MODES`/`VALID_HOOK_EVENTS`
  from `src/analyzers/json.rs` (not duplicated), top-level key completion
  filtered by sibling keys already present, hook-event completion inside the
  `hooks` object filtered by existing events. Old unfiltered behavior kept
  as fallback (Unknown context or missing tree/URI classification).
- src/hover.rs: `hover_at_position` returns the hovered key's schema entry
  (type/optional/stability/enum detail); falls back to the whole-surface dump
  (old behavior) when the position doesn't resolve to a known key, when the
  tree is unavailable, or the URI doesn't classify. `schema_key_for_uri` unchanged.
- src/schema.rs: untouched (no change needed; field tables already sufficient).
- src/analyzers/json.rs: 9 enum consts `const` -> `pub const` (sanctioned by
  the lane contract's "import them, don't duplicate"). No logic change.
- src/backend.rs (the ONLY two handler bodies + zero other lines):
  - hover(): clones uri, reads position, resolves via
    `adapter.get_document(...)` closure calling
    `crate::hover::hover_at_position(uri, doc.as_str(), doc.tree.root_node().into(), position)`,
    returns `Ok(hover.flatten())`.
  - completion(): same shape calling `crate::completion::completions_at_position`.
  - Backend.rs was also modified on disk mid-session by another lane
    (added markdown-doc test code referencing `is_markdown_config_doc` /
    `disc_dir_from_uri`-class helpers). Those edits are NOT mine; disclosed
    per compile-freeze disclosure law.

## Commands / exits
- cargo build: PASS (clean, before other lane's backend.rs in-flight edit)
- cargo test --lib completion: 9 passed, 0 failed (ALIVE)
- cargo test --lib hover: 6 passed, 0 failed (ALIVE 6/6)
- cargo test --lib (all): compile BLOCKED by concurrent lanes' in-flight
  test code, migrating over the session (not this lane's files, disclosed
  per compile-freeze disclosure law): src/backend.rs:474/492/509/512
  (`is_markdown_config_doc`/`doc_dir_from_uri` — later resolved by its
  owner), src/formatting.rs:104 (usize vs String — later resolved), and at
  final poll src/tokens_delta.rs:182/196/208/229 (private `len`, Option
  is_empty, Iterator item mismatch). My completion/hover tests compiled and
  passed 15/15 when the shared lib test target was compilable.
- cargo test (full): 3 targets fail at compile in files I do not own:
  tests/bcinr_parity.rs (HookAnalyzer trait import + &Vec vs &[Rule]),
  tests/acceptance_cli.rs (.bin() duplicate + TackedWorkspace), src/nouns/config.rs:313
  (usize vs bool assert) - all pre-existing or other-lane, none reference my diff.

## Verification ladder
Chicago-style unit tests over real fixture JSON parsed with the real
tree-sitter JSON grammar (no mocks):
- top-level completion excludes existing keys (model filtered, effortLevel present)
- hooks-object key completion suggests hook events minus existing events,
  no schema fields
- value completion: model enum, effortLevel enum, permissionMode enum
- Unknown context -> full unfiltered fallback (old behavior pinned)
- unknown URI -> None
- hover on key/value -> keyed entry only (asserts absence of whole-surface dump)
- hover on unknown key / no tree -> whole-surface fallback pinned
- unknown URI -> None

## Standing
- My files: ALIVE (cargo build PASS; 9 completion + 6 hover tests, 0 failures)
- Repo-wide cargo test: BLOCKED by other lanes' in-flight files (named above).
