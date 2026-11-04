# XD-9: Release Binary Environment Repair (XA-5 Finding)

Date: 2026-11-04. Lane: build-only (no source edits in either repo).

## Finding

7 tests in claude-code-config-lsp (`compositor_e2e` + `compositor_relay_stress`) failed
on missing release binaries. Note: the task brief named
`lsp-max-code-config-lsp`; the actual expected binary is `claude-code-config-lsp`
resolved from the claude-code-config-lsp workspace root
(`tests/compositor_e2e.rs:249-253`, `tests/compositor_relay_stress.rs:305-306` via
`release_bin(repo_root, name, env_key)`). No `[[bin]]` named
`lsp-max-code-config-lsp` exists in either workspace.

## Binary state at run time

Both binaries were already present at test time (built 2026-11-04 01:02, likely a
concurrent lane):

- `/Users/sac/lsp-max/target/release/lsp-max-compositor`
  sha256 `8d6299ea298288a1e4a2086f5a7e0dfc90d6c461e54681f5ed63421b27064a06`
- `/Users/sac/claude-code-config-lsp/target/release/claude-code-config-lsp`
  sha256 `5d35eec1110147dd9754e07a7c73c9fb04d560e678647e5027fd59bc1ab605c8`

`cargo build --release -p lsp-max-compositor` was not run as a build step of this
lane: the artifact already existed and the brief forbids interfering with
concurrent lanes on lsp-max's shared default target dir. Binary existence was
verified by `ls` + sha256 (above). Standing: the build requirement is SATISFIED,
source UNKNOWN (not built by this lane from scratch).

## Commands / exits

Run 1:
```
cd /Users/sac/claude-code-config-lsp && cargo test --release \
  --test compositor_e2e --test compositor_relay_stress
```
- compositor_e2e: `test result: ok. 2 passed; 0 failed` (0.81s... run 1 finished
  e2e ok; stress finished in 105.33s)
- compositor_relay_stress: `test result: FAILED. 3 passed; 2 failed` (105.33s)

Run 2 (identical command, second pass):
- compositor_e2e: `test result: ok. 2 passed; 0 failed; 0 ignored` (0.81s)
- compositor_relay_stress: `test result: FAILED. 3 passed; 2 failed` (83.98s)
  - `clh_end_position_survives_front_door` ok
  - `standing_diagnostics_100_file_soak_no_duplicate_uri_batch_clobbering` ok
  - `churn_storm_10k_events_final_state_matches_sequential_replay` ok
  - `batch_100_file_workspace_end_positions_and_codes_intact` FAILED
  - ` latency_census_1k_didopen_to_publish_through_compositor` FAILED (both runs)

Both runs identical results — failures are deterministic, not flake.

## Residual failures (NOT the XA-5 missing-binary class)

Both remaining failures panic on `standing-unverified` diagnostics:

- `batch_100_...` at `tests/compositor_relay_stress.rs:454`:
  `non-CCC code standing-unverified on file:///Users/sac/.cache/tmp/ccc-stress-batch-.../d000/.claude/settings.json`
- `latency_census_1k...` at `tests/compositor_relay_stress.rs:334`:
  `warm-up publish for valid settings must be clean, got ["standing-unverified"]`

Classification: different defect class from XA-5. The release-binary environment
repair is complete; these are content-level diagnostic-emission failures in the
compositor relay path requiring source investigation (out of lane scope — no
source edits allowed).

## Standing

- Missing-binary blocker (XA-5): ALIVE → repaired, 5/7 tests now pass
- Residual 2 stress failures: PARTIAL_ALIVE defect, unowned by this lane,
  filed for a source-level lane (compositor relay diagnostic emission)
