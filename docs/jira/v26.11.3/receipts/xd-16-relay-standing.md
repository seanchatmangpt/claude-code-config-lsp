# XD-16 — Relay-path standing-unverified reds (root-cause + fix receipt)

Date: 2026-11-04. Repo: `/Users/sac/claude-code-config-lsp`. No git commands run.

## Subject

Two deterministic reds in `cargo test --test compositor_relay_stress`:
`batch_100_file_workspace_end_positions_and_codes_intact` (stress.rs:454,
"non-CCC code standing-unverified") and
`latency_census_1k_didopen_to_publish_through_compositor` (stress.rs:334,
"warm-up publish for valid settings must be clean") — both panicking on
`standing-unverified` riding the merged publish batch through the compositor
front door.

## Root cause (two defects, one site: `src/backend.rs` `standing_layer`)

1. **Wrong root resolution.** The production standing layer resolved the
   receipt root as `provider_roots()[0]`, which is
   `navigation::default_corpus_roots()[0]` = `$HOME/.claude/rules`
   (`src/navigation.rs:151-155`) — the navigation CORPUS roots, not the
   workspace. The fleet-receipt lookup at `<root>/.claude/receipts/latest.json`
   therefore never read the opened workspace at all: standing could never be
   ALIVE in any governed workspace, and every config surface in every
   workspace resolved standing against an unrelated receiptless root
   (`~/.claude/rules/.claude/receipts/latest.json` does not exist).
2. **Un-governed-workspace noise.** With receipt `None`,
   `standing_for_surface` → UNVERIFIED → `standing-unverified` (INFORMATION)
   was appended to every publish batch, so valid content in any receiptless
   workspace carried editor noise. Witnessed pre-fix (first run, this
   session): both stress reds reproduced byte-identical to the dispatched
   failure output.

The AF-23 direct-call courts (`tests/standing.rs`) exercise the pure core
`standing_layer_for_root` with explicit roots and never ride
`standing_layer()` — hence 15/15 green while the relay was red. There was no
compositor defect: **no lsp-max files touched** (compositor release binary
rebuilt in-session only because it had been evicted from
`target/release/` — the concurrent-eviction hazard already recorded in
ab-18-relay-stress.md; not a diff).

## Diff (single file)

- `src/backend.rs` only:
  - `standing_layer()`: root resolution via new
    `surface_workspace_root(uri)` — the path prefix before the last
    `/.claude/` segment (the directory owning the surface's `.claude/`
    tree), falling back to the surface's parent directory for config
    surfaces outside a `.claude/` tree. Plus a governance gate: standing
    diagnostics ride the publish batch only when
    `load_receipt(root)` actually finds a fleet receipt; an un-governed
    workspace's merged batch stays single-family (analyzer codes only) —
    `standing-unverified` is resolvable vocabulary state
    (`standing_for_surface`), never editor noise on the publish path.
  - `surface_workspace_root()` (new, private helper).
  - Updated the stale H4 wiring comment (provider_roots law) accordingly.
- The pure core `standing_layer_for_root` (the AF-23 court seam) is
  untouched: UNVERIFIED remains its emitted vocabulary value, so all 15
  AF-23 courts stay byte-identical and green.

## Publish-path standing law (post-fix)

- Governed workspace (receipt exists): BLOCKED/PARTIAL published against the
  workspace's REAL receipt (root now resolves correctly); ALIVE silent.
- Un-governed workspace (no receipt): no standing diagnostics on the publish
  batch — analyzer family only.
- `protocol_roundtrip` raw-child fixtures (`file:///fixtures/...`,
  nonexistent disk paths → no receipt) now publish analyzer-family only —
  still lawful under its `CCC-*` / `standing-*` acceptor.

## Gates (this session, real output)

Reproduction (pre-fix): `cargo test --test compositor_relay_stress` →
`2 failed; 3 passed` — panics at stress.rs:334 and :454 on
`standing-unverified`, matching the dispatched failures.

```
cargo build --release                          → exit 0 (child binary rebuilt)
cargo clippy --lib -- -D warnings              → exit 0, zero findings
cargo test --test standing                     → 15 passed; 0 failed; exit 0
cargo test --test protocol_roundtrip           → 3 passed; 0 failed; exit 0
cargo test --test compositor_e2e               → 2 passed; 0 failed; exit 0
cargo test --test compositor_relay_stress (1)  → 5 passed; 0 failed; 61.35 s; exit 0
cargo test --test compositor_relay_stress (2)  → 5 passed; 0 failed; 49.88 s; exit 0
cargo test --lib                               → 264 passed; 0 failed; exit 0
cargo clippy --all-targets                     → 1 pre-existing warning
  (tests/compositor_relay_stress.rs:12 doc-list — AB-18's owned, not-owned-by-
  this-lane file, identical to the warning recorded in af-23 receipt; zero
  warnings in src/.)
```

## Verification ladder

Reproduce → root-cause by reading `provider_roots` → `default_roots` →
`default_corpus_roots` → fix in the production wiring only → narrow gates
(standing courts, raw-wire roundtrip) → integration (compositor_e2e) →
stress ×2 → lib regression sweep.

## Standing

- The two dispatched reds and the full stress suite: ALIVE (×2 this session).
- AF-23 direct-call courts: ALIVE 15/15, file untouched.
- compositor_e2e: ALIVE (after in-session rebuild of the evicted compositor
  binary — environmental, zero code delta).
- protocol_roundtrip: ALIVE.
- clippy: no NEW warnings (the one warning is pre-existing, in a file this
  lane does not own).
- lsp-max: untouched (no cross-repo diff). Disclosed environmental action:
  `cargo build --release -p lsp-max-compositor --bin lsp-max-compositor`
  from `/Users/sac/lsp-max` — exit 0 (2m35s), rebuild only, restoring the
  evicted binary; no lsp-max source changes.
