# L1 — Version bump 26.7.3 → 26.11.3 (all 7 workspace crates)

Lane: IMPLEMENTATION (L1). Repo: /Users/sac/lsp-max (writer). No git commands run.

## Changes

- `/Users/sac/lsp-max/Cargo.toml:142` — `[workspace.package] version` `26.7.3` → `26.11.3`.
  All 7 target member manifests use `version.workspace = true`, so this single field
  covers `lsp-max`, `lsp-max-macros`, `lsp-max-protocol`, `lsp-max-ast`,
  `lsp-max-lsif`, `lsp-max-compositor`, `lsp-max-cli` (playground/scaffold inherit
  it too — 9 members total, reported for transparency).
- 11 intra-workspace dependency pins bumped `26.7.3` → `26.11.3` in:
  root `[workspace.dependencies]` (4 pins: lsp-max-macros, lsp-max-protocol,
  lsp-max-lsif, lsp-max-ast) and member manifests of lsp-max-lsif (1),
  lsp-max-cli (4), lsp-max-compositor (2). All path deps kept.
- `Cargo.lock` — regenerated as a side effect; 9 `lsp-max*` package entries now
  `version = "26.11.3"`.
- `/Users/sac/lsp-max/CHANGELOG.md` — new `## [26.11.3] — 2026-11-03` entry
  summarizing landed session work (M4–M9 compositor courts, F3 ontology-first
  registry, M13 packaging docs) + F4 notification-dispatch fix marked "in
  progress"; upgrade notes record the required consumer pin bump
  (`lsp-max = "26.7.3"` → `"26.11.3"` in /Users/sac/claude-code-config-lsp,
  deferred to its own migration lane — that repo untouched).

## Verification (commands + exits, 2026-11-03)

- `grep -rn '26\.7\.3' --include=Cargo.toml . | grep -v target` → 0 matches (exit 1).
- `cargo metadata --no-deps` → all 7 target crates report `26.11.3`:
  lsp-max, lsp-max-macros, lsp-max-protocol, lsp-max-ast, lsp-max-lsif,
  lsp-max-compositor, lsp-max-cli.
- `cargo check --workspace` → **exit 0**, `Finished dev profile in 1m 02s`
  (no retries needed; warnings are pre-existing in `crates/branchless-lsp`,
  not in owned files).

## Status

ALIVE — all owned-file edits on disk, verified by grep/metadata/check this session.
Version bump is metadata-only; src/, tests/, examples/, docs/ untouched (L2 owns docs).

## Standing

CANDIDATE — compile gate green this session, but release standing is wave-gated
behind F4 (src/client/builder.rs mid-fix) and final fleet integration.
