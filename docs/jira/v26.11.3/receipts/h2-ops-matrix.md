# Receipt: H2 operations matrix (claude-code-config-lsp)

- **Lane**: IMPLEMENTATION (H2) — repo `/Users/sac/claude-code-config-lsp`
- **Owned files**: `src/harness_ops.rs` (new), `tests/harness_ops.rs` (new)
- **Scope disclosure (minimal unblock, same-checkout rule 6)**: one line added
  to `src/lib.rs` — `pub mod harness_ops;` — required to register the owned
  module; no backend.rs/capabilities.rs edits.
- **Date**: 2026-11-03

## Status: ALIVE — 25/25 tests pass; module compiles warning-free under
## clippy; 3 pre-existing failures elsewhere are not this lane's diff

## What landed

### 1. src/harness_ops.rs (new, ~640 LOC)

`harness_operation(op, uri, line_1based, char_1based, doc_text, corpus_index)
-> OperationResult` — the 9-operation Claude Code LSP tool surface over config
files, 1-based -> 0-based conversion inside the entry point (saturating; junk
coordinates degrade to typed misses, never underflow). Unknown op names are a
typed refusal (`unknown-operation`).

Operations-coverage matrix (also in the module doc):

| op | config semantics | backing module | status |
|---|---|---|---|
| goToDefinition | wikilink -> target H1; hook/script path token -> file on disk; JSON key -> property individual in schema/claude-code-config.ttl | navigation.rs, schema.rs, completion.rs | ALIVE |
| goToImplementation | command value / path token -> resolved script on disk; inline shell -> binary via $PATH; missing -> typed NotOnDisk | navigation.rs, completion.rs | ALIVE |
| findReferences | wikilink mentions + definition site; skill/agent name -> corpus referers | navigation.rs, hierarchy.rs | ALIVE |
| hover | keyed schema entry (JSON, tree-sitter cursor) or surface dump (markdown) | hover.rs, completion.rs, schema.rs | ALIVE |
| documentSymbol | markdown heading outline; JSON key tree as typed symbols | markdown_nav.rs; local JSON key-tree builder | ALIVE |
| workspaceSymbol | landed workspace walker over the corpus root; query = word under cursor, empty = all | workspace_symbols.rs | ALIVE |
| prepareCallHierarchy | frontmatter name -> callable unit, position clamped to the name range anywhere in a named doc | hierarchy.rs | ALIVE |
| incomingCalls | corpus referers of this doc's name | hierarchy.rs | ALIVE |
| outgoingCalls | wikilinks + path-like refs resolved against the corpus | hierarchy.rs | ALIVE |

Typed misses: `NotOnDisk { path, reason: "not-on-disk" | "not-on-path" }`,
`Unresolved { reason, detail }` with stable tokens (`unknown-operation`,
`wikilink-unresolvable`, `key-not-in-schema`, `key-not-in-ontology`,
`cursor-on-nothing-navigable`, `no-callable-unit`, `no-references-found`,
`no-symbols`, `ontology-unreadable`). Never a silent empty.

### 2. tests/harness_ops.rs (new, 25 Chicago tests)

Real collaborators only (fixture corpus + hermetic temp workspaces, no
mocks/assert-on-interaction). Per op:

- dispatch: 9 names advertised, unknown op -> typed refusal
- 1-based coords: hover on `"model"` (line 2, col 4, 1-based) returns the
  keyed `**`model`**` entry
- goToDefinition: wikilink -> target H1; wikilink-unresolvable (C5 fixture
  `[[local-dfcm-manufacturing-engine]]`); hook path -> on-disk script; JSON
  key -> ccc:settings_model line in the real ttl; unknown key ->
  key-not-in-schema; plain prose -> typed miss
- goToImplementation: on-disk script Location; inline `sh` -> binary on
  PATH; missing binary -> NotOnDisk(not-on-path); missing script ->
  NotOnDisk(not-on-disk)
- findReferences: wikilink channel (definition site + mentions); name channel
  (frontmatter-name fallback finds [[ggen]] referer); unreferenced word ->
  typed miss
- hover: CLAUDE.md surface-dump fallback over the C5 corpus
- documentSymbol: markdown outline (SymbolKind::STRING roots per
  markdown_nav); JSON key tree (model=STRING, hooks=OBJECT with PreToolUse
  child)
- workspaceSymbol: empty query over the C5 corpus root finds the `ggen`
  skill symbol
- hierarchy trio: prepare clamps anywhere-in-doc -> item name `ggen`;
  nameless doc -> no-callable-unit; incoming finds the [[ggen]] referer;
  outgoing resolves [[testing-chicago-style]]; no outgoing refs -> typed
  miss
- dirty-buffer law: a doc_text override pointing at `hooks/replaced_hook.sh`
  yields NotOnDisk, proving ops consult the in-editor text, not the stale
  corpus copy

## Commands / exits (this session)

| command | exit | note |
|---|---|---|
| `cargo build` | 0 | lib compiles; 0 warnings in harness_ops.rs |
| `cargo clippy --lib` | 0 mentions of harness_ops | clean after 2 lint fixes |
| `cargo test --test harness_ops` | 0 | **25 passed; 0 failed** |
| `cargo test --lib --tests` | 1 | 250 passed, 3 failed — failures are in `src/analyzers/json.rs` (span tests x2) and `harness_docs::standing_fixture_partition`, none in this lane's files; classified pre-existing/contemporaneous to other lanes' in-flight edits (my diff touches only harness_ops.rs, one lib.rs line, tests/harness_ops.rs) |
| `cargo test` (whole) | 1 | `examples/h1probe` fails to compile (`main` not found) — pre-existing, other lane's in-flight example, not this lane's diff |

## Defects filed (not edited — not this lane's files)

1. `examples/h1probe` does not compile (`E0601: main function not found`) and
   therefore blocks whole-`cargo test` target selection. Owner: whatever lane
   introduced the example.
2. Pre-existing failures in `src/analyzers/json.rs` span tests and
   `harness_docs::standing_fixture_partition` — owners should re-check on
   their next verify run; noted so they are not attributed to H2.

## Replay

    cd /Users/sac/claude-code-config-lsp
    cargo test --test harness_ops
    cargo clippy --lib
