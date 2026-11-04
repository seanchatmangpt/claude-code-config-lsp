# Receipt: p4-lens-signature

**Lane**: P4 (code lens + signature help) — v26.11.3 wave
**Repo**: /Users/sac/claude-code-config-lsp
**Standing**: ALIVE for owned surface (see verification below)

## Delivered

Owned files (all new):

- `src/code_lens.rs` — `code_lens::code_lens(uri, content, corpus) -> Vec<CodeLens>`
  - settings.json → "Run conformance audit" lens, command
    `claude-code-config/conformanceAudit` with the doc URI as sole argument
    (matches the dispatch string in `backend.rs::execute_command`; note the
    contract said `conformanceAudit`, the backend's registered command name is
    the fully-qualified `claude-code-config/conformanceAudit`, so the lens
    binds to the name the backend actually dispatches on).
  - SKILL.md → invocation-count lens: occurrences of the skill name (frontmatter
    `name:` else parent dir) across a corpus passed in as a parameter.
  - CLAUDE.md → wikilink-count lens (`[[...]]` occurrences in the doc).
  - All lens ranges = first line of the doc; non-lens docs → empty vec.
- `src/signature_help.rs` — `signature_help_at_position(content, position)`
  and `signature_help_at_offset(root, content, offset)`; resolves hook
  entries in settings.json via tree-sitter JSON (same import pattern as
  completion.rs): cursor inside a matcher object → hook-object signature
  `matcher: string, hooks: [{type, command, timeout?}]` (label constant
  `HOOK_SIGNATURE_LABEL`), activeParameter = matcher(0)/hooks(1); cursor
  inside a command object → activeParameter = type(0)/command(1)/timeout(2).
  Positions outside hook entries → None. Handles the real nesting
  `"hooks": { Event: [matcher objects] }` (event map is object-valued).
- `tests/lens_signature.rs` — 7 Chicago tests (real tree-sitter parses,
  real corpus, assert final state; no mocks).

## Disclosure: seam edit outside owned files

`src/lib.rs` — appended `pub mod code_lens;` and `pub mod signature_help;`
(additive; required for compilation). No backend.rs/capabilities.rs edits.

## Verification ladder (commands + exits)

Run on the live tree this session:

- `cargo test --test lens_signature` → `test result: ok. 7 passed; 0 failed`
  (exit 0). ALIVE.
- `cargo check` → `Finished dev profile` (exit 0) at time of run; note
  `cargo test --lib` currently fails to compile in
  `src/tokens_delta.rs:182,196,208,229` — another lane's in-flight
  `#[cfg(test)]` code, disclosed as a concurrent-lane failure, not
  introduced by this lane (my files do not reference tokens_delta).
  Isolated-scratch run (selection.rs + tokens_delta.rs quarantined, no
  shared-tree mutation) additionally passed `cargo test --lib code_lens`
  → `3 passed; 0 failed` for the in-module unit tests.

## Zero-mock statement

No mocks, no stubs, no sleeps: real tree-sitter JSON parses, real corpus
slices, assertions on returned state.

## Known divergences from contract

- Command name bound as `claude-code-config/conformanceAudit` (the backend's
  actual dispatch name) rather than bare `conformanceAudit`.
- SKILL.md and CLAUDE.md lens commands use `codeLens/resolve` as the command
  string with the count carried in the title (no server-side executeCommand
  exists for those surfaces).
