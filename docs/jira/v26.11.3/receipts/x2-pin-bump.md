# X2 — lsp-max pin bump 26.7.3 → 26.11.3 (config-lsp consumer)

Lane: IMPLEMENTATION (X2). Repo: /Users/sac/claude-code-config-lsp (writer).
No git commands run.

## Registry-vs-path decision (disclosed per contract)

`cargo search lsp-max` this session shows crates.io head at **26.7.3** — L1's bump
(see `l1-version-bump.md`) is local-repo-only and F4 (notification-dispatch fix) was
still in-flight there, so 26.11.3 is not published. Decision: **PATH dep** per
contract branch (b):

```toml
lsp-max = { version = "26.11.3", path = "../lsp-max" }
```

with `version = "26.11.3"` retained as a compat bound. When 26.11.3 publishes to
crates.io, drop `path` and keep the version pin.

## Changes

- `Cargo.toml:19` — `lsp-max = "26.7.3"` → `lsp-max = { version = "26.11.3", path = "../lsp-max" }` (only edit).
- `Cargo.lock` — regenerated via `cargo update -p lsp-max`: 5 packages swapped
  26.7.3 (registry) → 26.11.3 (path): lsp-max, lsp-max-ast, lsp-max-lsif,
  lsp-max-macros, lsp-max-protocol. 140 unchanged deps.
- `src/**` — **zero edits needed**. The F4 Client-trait changes in lsp-max are
  compile-compatible with this consumer as-is; no API breakage surfaced.

## Verification (commands + exits, 2026-11-03)

- `cargo search lsp-max` → crates.io max 26.7.3 (path-dep justification).
- `cargo update -p lsp-max` → exit 0, lockfile swaps recorded above.
- `cargo build` → **exit 0**, `Finished dev profile in 42.75s`.
- `cargo test --test protocol_roundtrip --test diagnostic_goldens --test acceptance_cli`
  → **exit 0**: protocol_roundtrip 3/3, diagnostic_goldens 16/16, acceptance_cli
  green (all three suites in one run, TEST_EXIT=0).
- `node scripts/dogfood.mjs` → **exit 0**; server healthy over real wire; observed
  diagnostic set = `CCC-XFILE-001` (settings.json hook-shebang finding, fixture-driven,
  pre-existing — not bump-induced).
- **Payoff test** `ggen sync --dry-run` → **exit 0**; `grep -ci 'FM-GEN-007'` on
  output = **0**; 18 template writes planned (`mode=create: target already exists`).
  **FM-GEN-007 era CLOSED** for this repo: the pack template source now resolves.
  Real sync/reconciliation left to I4's lane (not run here).

## Status

ALIVE — all edits on disk, verified by build/test/dogfood/dry-run this session.

## Standing

CANDIDATE — compile+test+dogfood gates green this session against path-dep
lsp-max 26.11.3 (local, F4-included). Promotes when 26.11.3 publishes to crates.io
and I4's real-sync lane lands.
