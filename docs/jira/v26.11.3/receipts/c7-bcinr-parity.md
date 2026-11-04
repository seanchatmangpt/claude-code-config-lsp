# Receipt: C7 — bcinr SIMD vs naive parity

Date: 2026-11-03 · Lane: C7 (implementation) · Repo: /Users/sac/claude-code-config-lsp

## Scope executed

- Created `tests/bcinr_parity.rs` (owned file): 4 parity tests comparing the real
  analyzer output (whichever path the feature flag selects) against a literal
  reference reimplementation of the private `analyze_rule_naive` spec, on real TOML
  and hook fixtures (valid CalVer, broken CalVer, prose hits, no-hits, adjacent
  hits, repeated patterns; hook misspelled/invalid event names, no-hits, mixed).
- No mocks; reference implementation is real code asserted on final state
  (Chicago). Each finding compared on code + message + span.
- Covered hook.rs event rules (contract item 3): hook.rs HAS an identical bcinr
  branch, so it is covered by the same mechanism (`hook_parity` test).

## Named defect (contract clause 2): bcinr feature does not compile

Exact error (cargo build --features bcinr, exit 101):

    error[E0432]: unresolved import `bcinr_logic`
      --> src/analyzers/claude_md.rs:11:5
      --> src/analyzers/frontmatter.rs:11:5
      --> src/analyzers/hook.rs:11:5
      --> src/analyzers/json.rs:11:5
      --> src/analyzers/toml.rs:11:5

The analyzer sources import `bcinr_logic::swar_str`, but `Cargo.toml` declares
`bcinr = { version = "26.4", optional = true }` — the wrong crate. `bcinr-logic`
26.6.24 already exists in `Cargo.lock` (transitive), so the fix is a one-line
dependency-name correction in `Cargo.toml` (`bcinr` → `bcinr-logic`) — NOT executed
here: lane contract forbids Cargo.toml additions; coordinator should apply it and
rerun `cargo test --features bcinr` to close the SIMD-path parity leg.

## Verification ladder (commands + exits)

- `cargo test --test bcinr_parity` — exit 0, re-witnessed as the FINAL gate after
  concurrent-lane churn in src/navigation.rs / src/nouns/config.rs settled. 4 passed, 0 failed:

      test toml_parity_default_analyzer ... ok
      test toml_parity ... ok
      test hook_parity ... ok
      test parity_single_byte_pattern ... ok

  This exercises the NAIVE path (default features).
- `cargo build --features bcinr` — exit 101, E0432 unresolved import `bcinr_logic`
  (defect above). SIMD-path parity = UNSUPPORTED in-repo pending the manifest fix.
  The reference implementation in tests/bcinr_parity.rs is already written so the
  identical test exercises the SIMD path once the feature compiles — no test
  change needed to close the leg.
- `cargo test` (full default suite) — compile error in the **bin** test target
  only: `src/nouns/config.rs:313` and `src/analyzers/json.rs:404` (other lanes'
  concurrent in-flight edits; pre-existing to this lane, not introduced by C7).
  The **lib** target + all integration tests including bcinr_parity compile and
  pass.

## Disclosed SLA minimal-unblock (other lanes' files, per same-checkout law 2)

- `src/analyzers/json.rs` `ts_parse`: `tree_sitter_json::language()` does not
  exist in tree-sitter-json 0.23 (0.23 exposes `LANGUAGE: LanguageFn`); fixed to
  `parser.set_language(&tree_sitter_json::LANGUAGE.into())`.
- `src/analyzers/json.rs` `ts_array_element`: `cursor` borrow outlives-temporary
  fix (bind result to local before cursor drop).

Both were disclosed in-repo with comments naming this lane; without them the lib
target did not compile at all. No other analyzers touched; backend/capabilities
untouched.

## Standing

- Naive-path parity: ALIVE (commands above).
- SIMD-path parity: UNSUPPORTED in-repo (bcinr feature does not compile — named
  defect). Test file is SIMD-ready; closing = fix Cargo.toml dep name + rerun.
- Full `cargo test`: PARTIAL — lib+integration PASS, bin-target compile errors
  owned by other lanes.
