# C1 Acceptance Receipt — conformance/receipt/fix CLI verbs

## Date
2026-11-03

## Repo
/Users/sac/claude-code-config-lsp (canonical checkout)

## Lane
IMPLEMENTATION (C1). Owned: src/nouns/config.rs, tests/chicago_tdd*.rs additions,
ORIGINAL_REQUEST.md, tests/acceptance_cli.rs (new). Out-of-lane minimal unblocks
(disclosed): src/main.rs — one argv-shim block for `receipt verify [PATH]`
(see Standing/Falsifiers for why it cannot live in the owned file).

## Files owned
- /Users/sac/claude-code-config-lsp/src/nouns/config.rs — conformance verb now
  wires `--format json|yaml|table` (json: serde_json pretty, health-vector shape;
  yaml: hand-emitted with quoted scalars; table: aligned summary rows + CODE/FILE/
  MESSAGE columns; unknown format exits 1 loudly). `receipt verify` logic shipped
  as receipt_verify_cli. Inline render unit tests added.
- /Users/sac/claude-code-config-lsp/tests/acceptance_cli.rs — 13 end-to-end tests
  against the real compiled binary (CARGO_BIN_EXE) over real temp workspaces:
  R1 x5 (json/yaml/table/alignment/loud-refusal), R2 x4 (valid chain, config
  tamper, receipt-byte tamper, chain listing), R3 x4 (dry-run no-write,
  --apply writes $schema, -y alias, clean workspace zero proposals).
- /Users/sac/claude-code-config-lsp/src/main.rs (disclosed minimal unblock, outside
  owned set) — argv shim: `receipt verify [PATH]` handled before framework dispatch.
- /Users/sac/claude-code-config-lsp/ORIGINAL_REQUEST.md — checkboxes ticked only
  for criteria run green this session.

## Commands + exits
| command | exit | note |
|---|---|---|
| cargo build | 0 | baseline and post-implementation |
| cargo test --test acceptance_cli | 0 | 13 passed; 0 failed (final run) |
| cargo test (full) | 0 | FINAL: 27 test targets, every line `test result: ok`, 0 failures (253 lib + 13 acceptance_cli + chicago/inventory/etc.). Earlier runs were blocked by concurrent lanes' in-flight edits (src/rename.rs, src/tokens_delta.rs, src/harness_ops.rs, examples/h1probe.rs) — polled per compile-freeze SLA each time |
| cargo clippy -- -D warnings | FAIL pre-existing | baseline: clippy::mutable_key_type in lib (AuthData/Cell interior mutability); not lane-introduced |
| receipt verify /tmp/cccv (manual) | 0 | valid chain |
| receipt verify on tampered fixture (manual+test) | 1 | both tamper classes |

## Standing
- conformance --format json|yaml|table: ALIVE (13/13 acceptance tests, exit 0)
- receipt chain: ALIVE (chain test exit 0, genesis prior-hash null asserted)
- receipt verify valid/tamper: ALIVE (exit 0 valid; exit 1 on config tamper and on
  receipt-byte mutation via BLAKE3 hash mismatch)
- fix dry-run/--apply/-y: ALIVE (dry-run asserts no on-disk change; apply asserts
  $schema written and file still valid JSON)
- full cargo test: ALIVE (27 targets, all `test result: ok`, 0 failures, final run
  2026-11-03; includes 13/13 acceptance_cli)
- clippy -D warnings: BLOCKED pre-existing (baseline mutable_key_type in lib,
  file:line outside lane)

## Falsifiers
1. `receipt verify` cannot be served by the framework registry at all: affidavit
   (transitive via lsp-max) registers `#[verb("verify","receipt")]` and
   CommandRegistry::init() re-runs the registration slices on EVERY get() (the
   loops sit outside get_or_init), so any pre-dispatch override is re-overwritten.
   Witnessed: PROBE_AFTER read back affidavit's about after a provably-executed
   register_verb_with_args insert. Receipt-chain verification therefore ships as
   an argv-shim in main.rs (disclosed; the only lawful dispatch path left).
2. Baseline clippy fails before any lane edit (mutable_key_type) — the acceptance
   checkbox "passes clippy" cannot be ticked truthfully.
3. Compile-freeze incidents by concurrent lanes (all recovered, all outside this
   lane): src/rename.rs E0308/E0599, src/tokens_delta.rs E0271/E0599/E0624,
   src/harness_ops.rs E0063, examples/h1probe.rs E0601. Also: rustfmt of my file
   tripped the Poka-Yoke FM-1.1 complexity guard on the `chain` verb (6 > 5) —
   fixed by extracting receipt_chain_or_die(); clippy/fmt clean on all owned
   files; remaining clippy -D warnings failures are pre-existing lib errors in
   files owned by other lanes.
