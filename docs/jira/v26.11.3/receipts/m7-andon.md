# M7 Receipt — andon_code_prefixes compositor tests

Subject: /Users/sac/lsp-max/crates/lsp-max-compositor/tests/andon_prefixes.rs (new; only file written)

## Semantics established (read from compositor source, no src changes)

- `andon_code_prefixes` (per-server key in `lsp-max.toml`) is consumed by
  `CompositorConfig::per_server_andon_prefixes()` / `all_andon_prefixes()`
  (src/config.rs) and wired into `MergeContext::from_config` (src/merge.rs),
  which compiles a per-server daachorse automaton = that server's C_D.
- Runtime behavior: diagnostic CODE ATTRIBUTION + BLOCKING, not suppression or
  routing. `attribute_andon()` returns PerServer (matched own C_D) / Union
  (unconfigured server or no server_id, matched baseline) / NotAndon.
  `has_andon_block` = severity 1 AND andon match. Andon Errors sort first;
  law-code dedup lets the lower severity number win.
- Defaults when key absent: WASM4PM-/ANTI-LLM-/GGEN- (static).
- Isolation: a configured server is never classified by the union — a CCC-
  code from ggen-lsp is NotAndon.
- Unknown-server fallback for `prefixes_for_server` is the full deduplicated
  union (includes declared prefixes of other servers).

## ANDON law cross-check (config-lsp backend.rs ↔ lsp-max client)

- The law ("findings never emit ERROR severity") is enforced upstream in
  lsp-max CLIENT `publish_diagnostics` (src/service/client/lsp_methods.rs):
  any ERROR with source != "lsp-max-andon" triggers injection of
  LSPMAX-ANDON-PUSH-MISSING (source "lsp-max-andon").
- The compositor PRESERVES the mechanism: merge_for_upstream never drops the
  injected diagnostic (key-scoped dedup; its Error severity wins same-key
  ties), and "LSPMAX-" is not a declared prefix, so attribution stays honest
  (NotAndon under both config-lsp's C_D and the union). Tests 9-11 pin this;
  test 11 is the falsifier that would catch loss of the injection contract
  downstream (violating ERROR finding + injected diagnostic both survive).

## Commands / exits

- `cargo test -p lsp-max-compositor --test andon_prefixes`
  → 11 passed; 0 failed (exit 0). Two initial failures were test-side
  expectation fixes against real semantics (union fallback content;
  andon_codes() is static-law-only — documented divergence, asserted).
- `cargo clippy -p lsp-max-compositor --test andon_prefixes -- -D warnings`
  → Finished, no warnings (exit 0).

## Standing

- Tests: ALIVE this session (commands above).
- Compositor src: unmodified (lane constraint honored). No git commands run.
