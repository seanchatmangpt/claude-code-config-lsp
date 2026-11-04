# P2 workspace/symbol Receipt

- **Lane**: IMPLEMENTATION (p2-ws-symbols)
- **Repo**: /Users/sac/claude-code-config-lsp
- **Date**: 2026-11-03
- **Standing**: ALIVE (ran this session; commands + exits below)

## Subject

`src/workspace_symbols.rs` (new) + `tests/workspace_symbols.rs` (new) + fixture
tree `tests/fixtures/ws_symbols/` (new) + one-line module registration in
`src/lib.rs` (`pub mod workspace_symbols;` — infra seam, disclosed: the
contract forbade backend.rs/capabilities.rs edits, not lib.rs, and a new
module cannot compile undeclared).

## Contract mapping

- Scan root `&Path` via a own walk: depth cap 4, prune
  `.git`/`target`/`node_modules`/`_build`.
- Reuse `crate::scan::classify` (read-only import, zero edits to scan.rs):
  json -> PROPERTY symbols named by dotted JSON key path (`env.PATH`);
  skill/agent -> CLASS named by frontmatter `name`; claude_md -> FILE named by
  H1 title; commands (classify says "unknown", surfaced by `commands/` parent
  dir convention) -> FUNCTION named by file stem.
- Query: case-insensitive substring, empty query = all (LSP semantics).
- SymbolInformation carries `container_name` = root-relative dir; `file://`
  URIs percent-encoded (spaces -> %20, verified by unit test).

## Deviations / notes

- `/commands/*.md` classifies "unknown" in scan::classify — surfaced by
  directory convention, documented in the module doc comment.
- Shared-compile freeze windows hit twice (other lanes' in-flight files:
  formatting.rs, rename.rs, signature_help.rs, tokens_delta.rs); per the
  compile-freeze SLA this lane waited and polled, no unblock edits applied to
  foreign files.

## Verification ladder (real commands, this session)

1. `cargo build` — clean (exit 0) after lanes stabilized.
2. `cargo test --test workspace_symbols` — 5/5 PASS (integration over the
   real fixture tree: all surfaces, kinds, case-insensitive query, locations,
   depth cap 4 with l4 visible / l5 pruned, target//_build pruned, missing
   root empty not panic).
3. `cargo test --lib workspace_symbols` — 6/6 PASS (5 inline unit tests +
   `provider_registry::workspace_symbols_finds_symbols_in_real_tree` — another
   lane's test importing this module over a real tree; cross-lane corroboration).
4. `cargo clippy --all-targets` — zero mentions of workspace_symbols.rs
   (map_or/as_deref/too-many-args findings fixed in-session).

## Falsifiers honored

- `tooDeep` (depth 5) absent, `atLimit` (depth 4) present — proves the cap
  binds, not vacuous.
- `hidden` (in target/ and _build/) absent — proves pruning binds.
- `zzz-no-such-symbol` query returns empty — proves the filter binds.
