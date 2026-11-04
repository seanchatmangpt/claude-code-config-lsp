# M12 — Dogfood Through the Compositor (scripts/dogfood_compositor.mjs)

Date: 2026-11-03
Lane: IMPLEMENTATION (owns `scripts/dogfood_compositor.mjs` only; `scripts/dogfood.mjs` untouched)
Verdict: **BLOCKED** — real upstream gap in lsp-max; the harness lands as a working falsifier that witnesses the exact dropped hop.

## What was built

`scripts/dogfood_compositor.mjs` (new):

- Spawns the **lsp-max-compositor** front door (`/Users/sac/lsp-max/target/debug/lsp-max-compositor`) with cwd = repo root, so `CompositorConfig::load()` finds this repo's `lsp-max.toml` by walking upward; prepends `target/release` to PATH so the manifest's bare command `claude-code-config-lsp` resolves. Compositor binary was absent at lane start; built with `cargo build -p lsp-max-compositor` in `/Users/sac/lsp-max` (dev profile, exit 0, ~1m13s).
- Framed JSON-RPC over the front door: `initialize` → `initialized` → `didOpen tests/fixtures/claude-config/settings.json` → bounded poll (25 ms tick, 15 s deadline, no fixed sleep) for `publishDiagnostics` → `didOpen tests/fixtures/claude-config/claude-global-CLAUDE.md` → poll → `shutdown`/`exit`.
- Asserts the merged stream carries the child's `CCC-*` codes; `--allow <CODE>` (repeatable) semantics mirror `dogfood.mjs`. Buffers use `subarray` throughout (no `buf.slice` deprecation warnings).

## Run transcript (2026-11-03, exit 2 = BLOCKED)

```text
$ node scripts/dogfood_compositor.mjs
[compositor] lsp-max-compositor: 2 server(s) registered
  claude-code-config-lsp (diagnostics-only): primary=[".json", ".md"] secondary=[]
  justfile-lsp (diagnostics-only): primary=[".just"] secondary=[]
[compositor] merge context: 2 andon prefix(es)
```

```text
BLOCKED: no publishDiagnostics crossed the compositor front door within 15s.
  Stuck URI: file:///Users/sac/claude-code-config-lsp/tests/fixtures/claude-config/settings.json
  ...
```

## Findings

1. **Control (child over the direct wire works):** driving `target/release/claude-code-config-lsp serve --stdio` directly, the child publishes `textDocument/publishDiagnostics` with `CCC-JSON-001` for `tests/fixtures/claude-config/settings.json` within 8 s (witnessed 2026-11-03). The child is not the blocker.
2. **Compositor admits the child over the wire:** spawns the child via PATH, `initialize` round-trips (merged capabilities returned to the harness: `{"capabilities":{"textDocumentSync":1},"serverInfo":{"name":"lsp-max-compositor","version":"26.7.3"}}`), and `ps` witnessed the child process `claude-code-config-lsp serve --stdio` alive during the session.
3. **The child→compositor notification hop is dropped upstream.** Witnessed root cause: `/Users/sac/lsp-max/src/client/builder.rs` `read_loop` — the `has_method` branch is a stub (literally `// Future: dispatch to _client trait methods based on method name`); child notifications are parsed and discarded. So `CompositorClient::publish_diagnostics` → `DiagnosticBuffer::deposit` → `FlushCoordinator` → editor never fires from real child traffic. The `.claude/lsp-max-fitness.json` written during runs is the startup baseline (flush_coordinator.rs writes it once at spawn, `ocel_event_count: 0`), not flush evidence.
4. Exit codes: 0 = CCC-* codes through the front door, 1 = assertion failure (unexpected codes, none allowed), 2 = BLOCKED (missing binaries or the witnessed upstream drop — the harness names the exact file:line).

## How to unblock

Implement notification dispatch in the lsp-max client `read_loop` (`/Users/sac/lsp-max/src/client/builder.rs`). When landed, rerun `node scripts/dogfood_compositor.mjs` — exit 0 requires CCC-* codes through the front door (with `--allow` for intentionally invalid fixture codes if those fixtures are opened).

## Statuses

- Harness `scripts/dogfood_compositor.mjs`: **ALIVE** (ran this session, exit 2, typed output, real transcript above).
- Wire-level CCC-* multiplexing through the compositor: **BLOCKED** (upstream lsp-max client read_loop notification dispatch unimplemented).
- Direct-wire child diagnostics: **ALIVE** — `CCC-JSON-001` witnessed (control run, 2026-11-03).
- `scripts/dogfood.mjs`: untouched, still warning-bearing by contract.
