# F4: Client notification dispatch — M12 falsifier flip

Status: ADMITTED (with named residuals)
Lane: IMPLEMENTATION (owner: src/client/builder.rs, tests/client_notification_dispatch.rs)
Date: 2026-11-03

## Change

`/Users/sac/lsp-max/src/client/builder.rs` — implemented the notification dispatch stub in
`read_loop` (the M12 witnessed root cause). The notification branch now calls
`dispatch_notification(&client, &method, params)`:

- `textDocument/publishDiagnostics` → `client.publish_diagnostics(params)` via
  `serde_json::from_value` into `PublishDiagnosticsParams`
  (the `LanguageClient` trait in `src/client/client.rs` already carried all required
  methods; no trait changes were needed).
- `window/logMessage` → `client.log_message`; `window/showMessage` → `client.show_message`;
  `telemetry/event` → `client.telemetry_event`; `$/progress` → typed acknowledge-and-drop
  (no handler yet, doc comment in place).
- Unknown methods → `tracing::debug!` + drop; malformed params on known methods →
  `tracing::warn!` + drop; session stays alive.
- Server-to-client requests (id+method) are loudly logged (`tracing::warn!`) and left
  unanswered — a named residual (no write-path access from read_loop).
- Compositor attribution respected: child diagnostics pass through unchanged (andon
  prefixes intact); no rewriting at this layer.

New integration tests: `/Users/sac/lsp-max/tests/client_notification_dispatch.rs` (4 tests,
real duplex streams, real client recording state, no mocks):

1. `publish_diagnostics_reaches_client_with_intact_params` — andon-prefixed child
   diagnostic arrives with params unchanged
2. `unknown_notification_dropped_session_alive` — unknown method dropped, later valid
   notification still lands
3. `malformed_frame_skipped_session_alive` — invalid-JSON body skipped, loop continues
4. `malformed_params_dropped_session_alive` — bad params on a known method dropped,
   valid one dispatched

## Verification ladder

Commands run this session (cwd `/Users/sac/lsp-max` unless noted):

- `cargo test --test client_notification_dispatch` → 4 passed, 0 failed
- `cargo test -p lsp-max-compositor` → all suites 0 failed (52+11+8+3+1+9+2+1+47+10+3+9+6+21+12+5 across suites; the 30.05s suite is the fixed-window crash-isolation court)
- `cargo test -p lsp-max --lib client` → 11 passed, 0 failed
- Dogfood falsifier (cwd `/Users/sac/claude-code-config-lsp`):

```
node scripts/dogfood_compositor.mjs --allow CCC-JSON-001 \
  --fixtures tests/fixtures/claude-config/settings.json tests/fixtures/claude-config/claude-global-CLAUDE.md
```

### Before (baseline stub, exit 2)

Baseline = this diff's `dispatch_notification` call site replaced with the original drop
behavior, rebuilt, re-run. Transcript excerpt (`/tmp/dogfood_before.txt`):

```
BLOCKED: no publishDiagnostics crossed the compositor front door within 15s.
```

before_exit=2 (BLOCKED)

### After (this diff, exit 0)

after_exit=0, transcript excerpt (`/tmp/dogfood_after.txt`):

```
Compositor front door: /Users/sac/lsp-max/target/debug/lsp-max-compositor
JSON fixture: tests/fixtures/claude-config/settings.json -> 1 diagnostic(s)
MD fixture:   tests/fixtures/claude-config/claude-global-CLAUDE.md -> 1 diagnostic(s)
Diagnostic codes observed through the compositor: CCC-JSON-001
CCC-* codes (child multiplexed at the wire level): CCC-JSON-001
```

## Residuals (named, not hidden)

1. `scripts/dogfood_compositor.mjs` md-fixture default mis-parses when `--fixtures` is
   absent (`args[args.indexOf("--fixtures") + 1]` with indexOf -1 yields args[0], i.e. the
   `--allow` value). Worked around by passing `--fixtures` explicitly; the script is not
   in this lane's ownership, so filed here rather than fixed.
2. Server-to-client requests (id+method) are logged-warned and unanswered; a real
   server-to-client request (e.g. `workspace/configuration`) would stall the child until
   its own timeout. Named residual, open lane (needs write-path access from read_loop).
3. Flakiness note: `crash_isolation` (compositor) failed 2/5 runs under heavy compile
   load. Pre-existing load flake — the baseline variant failed the same way in one run,
   and it passed 3/3 on retry and in the final full-suite run. Not session-introduced.

## Standing

ALIVE — dispatch implemented; 4/4 new dispatch tests, compositor suite, and lsp-max
client lib tests pass this session (commands + exits cited above); falsifier flipped
before=exit 2 (BLOCKED) → after=exit 0 with CCC-JSON-001 crossing the front door.
