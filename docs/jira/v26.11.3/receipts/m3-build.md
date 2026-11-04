# M3 Build Receipt — lsp-max-compositor

Date: 2026-11-03. Lane: IMPLEMENTATION (M3 build + fix minimal compile errors).
Repo: /Users/sac/lsp-max. Status vocabulary per AGENTS.md (no victory language).

## Baseline

- `cargo check -p lsp-max-compositor` — exit 0, `Finished dev profile in 1m 41s`.
  The lib+bin target was already compile-green at session start; the dirty-tree
  drift was confined to test targets.
- Sibling checkouts verified present: `/Users/sac/lsp-types-max`,
  `/Users/sac/wasm4pm`, `/Users/sac/wasm4pm-compat` (GETTING_STARTED paths).

## Fixes made (all in crates/lsp-max-compositor)

1. `tests/request_arbitration.rs` (compile errors):
   - Added `use std::time::Duration;` (E0433, line ~317).
   - `format!("{content}")` → `format!("{content:?}")` — `MarkedString` does not
     implement `Display` (E0308/E0277). Disclosed hand-fix; content assertion
     semantics unchanged (debug format of the same value).
   - Note: the harness section of this file was concurrently restructured
     (two-duplex stubs) by another lane mid-session; my edits were applied on
     top of the current on-disk state, not the version I first read.
2. `compositor.ttl` + `src/registry_init.rs` (stale generated registry vs
   lsp-max.toml, the drift the lane contract anticipated):
   - Added two ChildServer individuals to the ontology source
     (`crates/lsp-max-compositor/compositor.ttl`): `ggen-lsp`
     (Primary, primaryExtension .ttl/.rq/.tera, andonPrefix GGEN-) and
     `wasm4pm-lsp` (DiagnosticsOnly, .ocel.json/.ocel.jsonl/.receipt.json,
     andonPrefix WASM4PM-) — matching lsp-max.toml [[server]] declarations.
   - `src/registry_init.rs` hand-extended in exact registry_init.tera template
     format (per-(id,ext) rows, extensions sorted). DISCLOSED HAND-FIX: ggen
     26.10.11 `ggen sync run` in crates/lsp-max-compositor is blocked by the
     unrelated `cc-chicago-tests` rule failing with
     `[FM-GEN-008][TEMPLATE_VARIABLE_MISSING] scenario_slug not found` (pack
     chicago-tdd-tools template/manifest mismatch, pre-existing). The
     registry-init rule itself was proven byte-identical reproducible
     (baseline copy diff → IDENTICAL) before hand-editing, so the hand edit is
     the mechanical template projection of the edited graph; a ggen re-run
     after FM-GEN-008 is fixed will reproduce the file.

## Verification ladder (actual commands + exits)

- `cargo check -p lsp-max-compositor` → exit 0 (1m41s).
- `cargo build -p lsp-max-compositor --release` → exit 0 (3m51s).
- `cargo test -p lsp-max-compositor --no-fail-fast` → exit 101; 18 targets:
  17/18 green, 1 target flaky under parallel-suite load. Counts:
  - lib unittests 52 passed; lsp_echo_server bin 0; main bin 0
  - andon_prefixes 11, cc_007_lsif_routing 8, diagnostics_merge 9, e2e 2,
    fanout_latency 1, gate_path_roundtrip_witness 1, integration 47,
    priority_semantics 10, registry_ontology_test 3, request_arbitration 9
    (1 ignored separately counted — reported "9 passed; 1 ignored"),
    routing_dispatch 6, routing_integration 21, speciation 12,
    v26_6_28_gate_tests 5 — all 0 failed
  - crash_isolation: PASSED 3/3 when run in isolation (`cargo test -p
    lsp-max-compositor --test crash_isolation` → exit 0, 30.07s); FAILED 1/3
    or 3/3 variably inside the full parallel suite with
    `initialize handshake must complete within bound: Elapsed` at
    tests/crash_isolation.rs:72-73. Real-subprocess handshake under a fixed
    10 s bound (HANDSHAKE_BOUND) races machine load from sibling target builds
    running in the same second — flake class, not a compile or logic defect.
    SLEEP-CLASS note: the bound is a fixed-duration fence; a load-free rerun
    is the current disambiguator.
- `cargo check --workspace` → exit 0 (2m32s), warnings only (branchless-lsp
  unused vars — not this crate).

## Workspace status

`cargo check --workspace` exit 0 — no sibling crates block the workspace
build. All three sibling checkouts exist at expected paths.

## OPEN / disclosed items

- ggen `cc-chicago-tests` rule FM-GEN-008 (pre-existing, outside lane scope —
  blocks full `ggen sync run` for the compositor consumer).
- crash_isolation parallel-suite flake (fixed 10 s handshake bound under load).
- Concurrent-lane collisions this session: tests/request_arbitration.rs,
  tests/crash_isolation.rs, tests/registry_ontology_test.rs,
  src/registry_init.rs were edited by another lane mid-flight (compile error
  in registry_ontology_test.rs observed in-flight, later resolved); the final
  state on disk compiles and the counts above are from the final state.
- The 30s-class runtimes on crash_isolation/request_arbitration are the tests'
  own bounded-timeout design (ServerHandle 30 s pending timeout under test).
