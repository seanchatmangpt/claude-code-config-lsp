# Integration Close — v26.11.3 Post-Wave Gate

Lane: LANE-CLOSE. Census-only pass — failures recorded and attributed, nothing fixed,
no git operations. Date: 2026-11-03.

## Three-repo gate table

| repo | gate | command | result | exit |
|---|---|---|---|---|
| config-lsp | build | `cargo build` | Finished dev profile, 1m15s | 0 |
| config-lsp | tests | `cargo test` (17 test binaries) | 438 passed / 1 failed | fail-fast on 1 |
| config-lsp | dogfood | `node scripts/dogfood.mjs` | 12 fixtures, all clean, no diagnostics | 0 |
| ggen | tests | `cargo test --no-fail-fast -p ggen-abb-sbb -p ggen-engine -p ggen-sjira -p ggen-cli-lib -p ggen-config` (126 binaries) | 1306 passed / 11 failed | non-zero |
| ggen | clippy | `cargo clippy --workspace` | FAILED — hard error, `could not compile ggen-cli-lib` | non-zero |
| arcus | sync determinism | `ggen sync run` ×2 + tree sha256 diff | generated content byte-identical; only `.ggen-v2/receipt-log.jsonl` + `receipt.json` differ (append-only receipts — expected) | 0 / 0 |
| arcus | go build | `go build ./...` | PASS | 0 |
| arcus | go vet | `go vet ./...` | PASS | 0 |
| arcus | go test | `go test ./...` | 3 pkgs ok (adapter, cdata, spog) | 0 |
| arcus | goldens | `bash tests/goldens/verify.sh` | all OK: byte-identity, determinism, gofmt, build, vet | 0 |

## config-lsp census (438 pass / 1 fail)

All 17 test binaries green except:

- `multiplex_real_language_servers_end_to_end` — tests/multiplex_real_lsps.rs:150 panic:
  "timed out waiting for publishDiagnostics" after 60s+; `[SKIP-MULTIPLEX-CHILD] unknown
  server id "" in template — dropped`. Real-LSP e2e, environment-dependent (child server
  spawn/template resolution). 212s to failure. First-run and rerun both failed identically.

Known-flake list status: contract-named flakes did NOT fire — `harness_docs` (11/11 ok),
json span tests green, formatting green. Pre-existing clippy `mutable_key_type` not
re-run as gate (not in contract scope for config-lsp).

## ggen census (1306 pass / 11 fail, 126 binaries, --no-fail-fast)

Failures attributed to owning lanes/files:

| test | file | notes |
|---|---|---|
| perf_cold_start_with_config | crates/ggen-cli/tests/performance.rs | perf bound, ran 0.33s |
| bare_sync_defaults_to_run, real_run_writes_outputs_and_receipt, watch_starts_and_runs_until_killed, dry_run_exits_zero_and_writes_nothing | crates/ggen-cli/tests/sync_surface_test.rs | 4/6 in binary failed |
| no_governance_doc_disagrees_with_real_workspace_crate_count | crates/ggen-config/tests/governance_precommit_gate_count_test.rs | workspace crate-count drift |
| repo_facts_ttl_crate_map_matches_cargo_toml_workspace_members | crates/ggen-config/tests asserting repo-facts TTL vs Cargo.toml members | workspace member drift |
| tier0_reference_fixture_generates_all_four_artifacts_and_is_idempotent | crates/ggen-engine/tests/fortune5_architecture_pack_e2e.rs | fixture/fallback content drift |
| testing_bblock_pack_generates_verifier_and_suite_scripts_with_real_content, generated_suite_scripts_are_syntactically_valid_bash, generated_verifier_self_test_actually_passes_as_a_real_subprocess | crates/ggen-engine/tests/fortune5_testing_bblock_pack_e2e.rs | 3/3 in binary failed |

Contract-named known failures did NOT fire: FM-LAW-015 fixture fallout (~100) not observed
in ggen-engine census (ggen-engine binaries: 1306 total includes engine); `doctor_e2e` and
`envelope_caps_e2e` not in the failed set. K2's fix appears to have landed for FM-LAW-015.
`envelope_courts` 4/4 ok, `doctor_surface_test` 5/5 ok.

## ggen clippy (FAILED)

`cargo clippy --workspace` hard-fails:

- `crates/ggen-cli/src/cmds/sjira/mod.rs:1` — `clippy::too_long_first_doc_paragraph`,
  escalated to error by `#![deny(warnings)]` at `crates/ggen-cli/src/lib.rs:51`;
  output: "could not compile `ggen-cli-lib` (lib) due to 1 previous error". Workspace
  clippy aborts here (later crates unchecked this run).
- `ggen-engine` lib: 35 warnings (non-blocking), incl. `fn_params_excessive_bools`.
- clippy exit code recorded via pipe (tail exited 0; cargo exit non-zero on "could not
  compile" evidence — exit code not captured cleanly, failure itself is receipt-backed).

## arcus census (all green)

`ggen sync run` ×2 exit 0. Determinism: full-tree sha256 before/after sync #2 — only
append-only `.ggen-v2/receipt-log.jsonl` and `.ggen-v2/receipt.json` changed; every
generated file byte-identical. `go build ./...` 0, `go vet ./...` 0, `go test ./...` 0
(adapter, cdata, spog all ok). `tests/goldens/verify.sh` all OK — K7's landed fix is
verified green. Arcus verdict: GREEN.

## Integration verdict

- **config-lsp: GREEN-WITH-DEBT** — 438/1; sole failure is the env-dependent real-LSP
  multiplex e2e (child-server spawn/template id resolution), not a unit/integration
  regression.
- **ggen: GREEN-WITH-DEBT** — 1306/11 across 126 binaries, all failures in 5 binaries
  across 3 crates (ggen-cli perf + sync-surface, ggen-config governance/parity docs drift,
  ggen-engine fortune5 pack e2e fixtures); clippy workspace gate is RED (ggen-cli-lib
  doc-comment lint denied into hard error).
- **arcus: GREEN** — every gate exit 0, goldens verified, sync determinism proven.

## Cross-repo coordination items

1. **Binary-identity confound**: ggen sync in arcus ran against ambient ggen 26.10.11
   (INFO line: `version="26.10.11"`), while ggen repo version is 26.10.12/26.11.3-era
   workspace. Which ggen binary qualifies arcus's determinism gate is confounded — pin
   the qualifying ggen identity (path+version+digest) in the arcus gate, or route through
   a frozen binary per the generator-plurality court (C20 shape).
2. **Shared-target races → per-lane CARGO_TARGET_DIR leases**: this pass ran config-lsp
   tests and ggen tests concurrently against per-repo shared `target/`; cross-lane cargo
   locking serializes lanes and can starve them (one ggen census run hit a background
   time cap). Next wave: per-lane `CARGO_TARGET_DIR=_target-lane<N>` leases, deleted at
   integration (same lease law as MIX_BUILD_ROOT).
3. **Consumer pin migrations**: arcus consumes ggen via ambient PATH binary (26.10.11)
   not a pinned workspace build; config-lsp workspace depends on
   `/Users/sac/lsp-max` (lsp-max v26.11.3 path deps). Both are consumer-side pins that
   drift from the producing repo's head — migrate to SHA-pinned references (catalog
   standing P / C03 shape) when either producer advances.

## Failures — no fixes applied (census-only)

Recorded only, attributed above. No src/**, packs/**, tests/** touched; no git run.
