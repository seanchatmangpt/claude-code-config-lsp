# R2 Multiplex Real LSPs Receipt

Date: 2026-11-03. Lane: IMPLEMENTATION (R2). Status: **ALIVE** — the compositor
multiplexes four real language servers and relays per-child diagnostics with
per-child attribution, witnessed over the real wire.

## Per-language verdict table

| Child | Binary (resolved) | Version | Verdict | Evidence |
|---|---|---|---|---|
| rust-analyzer | rustup stable 1.97.0 (2d8144b7) | component installed this session (`rustup component add rust-analyzer`) | ALIVE | 4 diagnostics relayed for `src/main.rs` (`syntax-error` code from rust-analyzer parse errors + flycheck), source `compositor/primary:rust-analyzer`; didClose sent |
| gopls | `~/go/bin/gopls` | v0.24.0 (installed this session, `go install golang.org/x/tools/gopls@latest`) | ALIVE | `UndeclaredName` compile diagnostic relayed for `main.go`, source `compositor/primary:gopls`; didClose sent |
| elixir-ls | `~/.vscode/extensions/jakebecker.elixir-ls-0.31.1/elixir-ls-release/language_server.sh` | ElixirLS v0.31.1 (VS Code extension release, precompiled) | ALIVE | 2 severity-1 diagnostics relayed for `lib/app.ex` (unclosed-`do` fixture violation); message-based attribution, child codes empty on the wire |
| claude-code-config-lsp | `target/release/claude-code-config-lsp` (built this session) | 26.6.24 | ALIVE | `CCC-JSON-004` relayed verbatim for `settings.json` violation, source `compositor/diagnostics-only:claude-code-config-lsp` |

Test: `cargo test --test multiplex_real_lsps` → `test result: ok. 2 passed; 0 failed` in 24.31s
(`--nocapture` transcript above). Test file: `tests/multiplex_real_lsps.rs`. Manifest variant:
`tests/fixtures/multiplex/lsp-max.toml` (schema-exact, eight keys per entry, placeholder-rendered
at test time). Fixtures: `tests/fixtures/multiplex/{main.rs,main.go,app.ex}` — sanitized from
ggen `crates/ggen-lsp/src/error.rs`, arcus `internal/arcus/adapter/types.go`, and ggen_igniter
`lib/ggen_igniter/application.ex`, each carrying one deliberate real-diagnostic violation.

## Attribution law (the assertion)

The compositor rewrites every relayed child diagnostic's `source` to
`compositor/<tier>:<server_id>` (`lsp-max/crates/lsp-max-compositor/src/server.rs:597-599`) and
preserves the child's `code` verbatim. The test asserts BOTH: relayed source names the child,
AND the expected code/message family arrives (CCC-JSON-004 exact; rustc/rust-analyzer
`syntax-error`; gopls `UndeclaredName`; elixir-ls severity-1 syntax message).

## Build record

- config-lsp release binary: built 13:39 + rebuilt with the tree (1m32s / 1m41s release builds).
- compositor: rebuilt from current lsp-max source (release, 2m57s + incremental).
- New host toolchains provisioned this session: rust-analyzer (rustup stable component),
  gopls v0.24.0 (go install). elixir-ls used the precompiled VS Code release script — no
  `mix elixir_ls.generate` needed.

## Findings (real, recorded, not fixed by this lane)

1. **Relay regression (resolved by rebuild, root cause stale-link)**: the pre-existing
   `compositor_e2e` and lsp-max's own `diagnostic_relay_probe::run_stdio_relay_works_end_to_end`
   both FAILED against the 13:30 compositor release binary — child diagnostics deposited but
   never published to the editor. An echo-LSP child (python, logs everything it receives) proved
   the child DID receive didOpen and DID publish; the drop was inside the compositor child→editor
   relay. Rebuilding the compositor release binary (`touch` + `cargo build --release`) fixed both.
   Evidence: failing `cargo test -p lsp-max-compositor --test diagnostic_relay_probe` output
   above in session; passing echo probe after rebuild.
2. **RUSTUP_TOOLCHAIN pinning hazard**: cargo test pins `RUSTUP_TOOLCHAIN=nightly` (from
   `claude-code-config-lsp/rust-toolchain.toml`) into the test process env; the compositor
   inherits it, its rust-analyzer child then dies with `Unknown binary 'rust-analyzer' in
   official toolchain 'nightly-aarch64-apple-darwin'` (the nightly toolchain lacks the
   rust-analyzer component). Fix landed in the test: `.env_remove("RUSTUP_TOOLCHAIN")`
   on the compositor spawn. Any fleet test that spawns rust-analyzer through a compositor under
   cargo needs this.
3. **Shared-target-dir races**: during the wave, lanes' concurrent cargo invocations repeatedly
   removed `target/release` binaries mid-run (config-lsp and lsp-max-compositor both vanished
   twice). The final green run required rebuilding both release binaries in the same command as
   the test. Documented per [[same-checkout-fanout]] compile-freeze law.
4. **Shutdown-teardown latency (transient)**: one run observed the compositor not exiting within
   60s after shutdown/exit/stdin-EOF with the full four-child pool (full-tier child teardown).
   The final witness runs exited cleanly; the test treats a >120s shutdown hang as a loud
   `[MULTIPLEX-PARTIAL]` finding, not a faked pass.
5. **SLA unblock disclosed (both-lane receipt law)**: `src/tokens_delta.rs` — the private
   `use lsp_max::lsp_types::{...}` made the shared release build fail E0603 for another lane's
   new `src/admission.rs` import. This lane changed the `SemanticTokensDelta` import to a
   `pub use` re-export (3 lines). Owning lane should re-shape at integration.
6. **Compositor andon injection**: relayed batches include a synthetic
   `LSPMAX-ANDON-PUSH-MISSING` diagnostic (source `lsp-max-andon`) appended by the compositor's
   andon layer to every batch. Expected behavior, not a child diagnostic; the attribution
   assertion ignores it.