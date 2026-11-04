# M8 — Compositor child-crash isolation (IMPLEMENTATION lane receipt)

Lane: IMPLEMENTATION (M8), repo `/Users/sac/lsp-max`, canonical checkout.
Owned file: `crates/lsp-max-compositor/tests/crash_isolation.rs` (new, ~290 LOC).
No compositor src modified. No git commands run.

## Subject

Child-crash isolation court over REAL child processes. No mocks. Stable child =
`lsp-echo-server` (`env!("CARGO_BIN_EXE_lsp-echo-server")`); unstable child =
`/bin/sh -c` stubs (respond to `initialize`, then SIGKILL self, or sleep forever).
Every assert is on final/observable state (exit status, bounded round-trip,
pool membership), never on call counts.

## What is (real behavior, read from src before testing)

- **Restart policy: NONE.** The exit watcher (`server.rs:99-126`) only logs and
  clears buffer URIs. The dead child's `ChildProcessPool` entry is never evicted
  and never respawned. Claude Code's `restartOnCrash`/`maxRestarts` govern the
  FRONT-door process, not the compositor's children. Pinned as `crashed_child_is_not_restarted_by_the_compositor`.
- **In-flight request on child death: 30 s stall into `Ok(None)`.**
  `ServerHandle::request` (`src/client/server_handle.rs:67-84`) has a hardcoded
  30 s timeout; on child death the read loop EOFs and the pending oneshot stays
  in the pending map (builder.rs read_loop returns at EOF without draining),
  so resolution comes only from the timeout, and launders into `Ok(None)` —
  indistinguishable from "server has no hover". Pinned as
  `in_flight_request_resolves_after_real_mid_request_crash`.

## The four witnesses

| # | court | assert (final state) | status |
|---|---|---|---|
| C1 | `child_a_crashes_child_b_continues_and_session_survives` | crash stub A dies by signal (exit watcher status `code().is_none()`); B answers a real round-trip AFTER A is dead (< SERVE_BOUND=5 s); pool still lists both ids | PASS |
| C2 | (same test) | front-door session survives: exit watcher resolved Ok, pool intact, B's connection live | PASS |
| C3 | `crashed_child_is_not_restarted_by_the_compositor` | over a 2 s observe window pool membership frozen at 2, dead entry remains, B still serves | PASS (pins no-restart law) |
| C4 | `in_flight_request_resolves_after_real_mid_request_crash` | mid-request SIGKILL (pkill by unique argv marker, bounded poll); request resolves within INFLIGHT_BOUND=40 s (no hang) — real shape is `Ok(None)` after ~30 s | PASS (pins the laundering AS-IS) |

## Commands / exits (this session)

```
cargo test -p lsp-max-compositor --test crash_isolation
→ 5 consecutive green runs post-hardening:
  ok. 3 passed; 0 failed ... 30.06s / 30.08s / 30.05s / (fmt+clippy run) / 30.08s / 30.05s
cargo clippy -p lsp-max-compositor --test crash_isolation -- -D warnings → exit 0
rustfmt --check --edition 2024 .../tests/crash_isolation.rs → clean
```

Pre-hardening flakes witnessed and fixed: (a) stub framing — `read` leaves `\r`
on header lines so the `""` blank-line break never fired while stdin stayed
open (verified with an open-stdin Python driver); (b) `dd` echoed the consumed
body to stdout, polluting the frame stream (redirected to /dev/null); (c)
fixed pre-kill sleep replaced by bounded pkill poll (no-sleep discipline);
(d) HANDSHAKE_BOUND 10→30 s after two load-correlated handshake timeouts while
other lanes were compiling (292 ms typical handshake; margin now 100×).

## Defects filed (IS vs SHOULD-BE)

1. **OPEN — typed-failure laundering**: child death during an in-flight request
   resolves `Ok(None)` after the hardcoded 30 s (`server_handle.rs:67`), not a
   typed `ClientError` at stdout EOF. Two fixes available in `builder.rs`
   read_loop: on EOF, drain `pending` with typed senders; or have the exit
   watcher fail pending requests. Front-door hover on a dead Primary child
   also stalls FirstSuccess dispatch for 30 s before falling through.
2. **OPEN — no child restart policy**: dead children are never evicted from
   `ChildProcessPool`; the stale handle stays in every FirstSuccess candidate
   list. If restart/eviction is wanted, C3 is the falsifier surface that flips.
3. **Pre-existing, out of lane scope**: `server.rs` exit watcher + client
   request path live in `server.rs`/root `src/client/` — owned by other lanes.

## Standing

- Tests: ALIVE (ran+passed this session, receipts above).
- Defect findings: CANDIDATE (witnessed by C4/C3 courts; fixes belong to the
  src-owning lanes).
- Env note: during the session another lane's mid-flight edit to root
  `src/client/builder.rs` transiently broke the shared `lsp-max` lib compile
  ("unexpected closing delimiter", builder.rs:209); waited out the SLA window,
  tree healed, gate re-run green. Not a session-introduced failure.
