# AB-18 — Compositor relay stress (STRESS lane receipt)

Date: 2026-11-03. Repo: `/Users/sac/claude-code-config-lsp`. Lane: STRESS (AB-18).
No git commands run (coordinator owns git).

## Subject

Stress the compositor relay path post-CL-E: diagnostic publish through the
lsp-max-compositor front door under census / batch / standing / churn load,
over real Content-Length-framed stdio wire. Read-only on src/ — black-box
over the real transport.

## Diff (owned files only)

- `tests/compositor_relay_stress.rs` (new) — 4 active stress tests + 1
  registered-divergence court. Harness shape copied from
  `tests/compositor_e2e.rs` (explicit-path spawn, mpsc reader, bounded
  polls; timing recorded, never fenced on absolute-ms).
- `docs/jira/v26.11.3/receipts/ab-18-relay-stress.md` — this receipt.

Precondition builds (disclosed, in-lane):
- `cargo build --release -p lsp-max-compositor --bin lsp-max-compositor`
  from `/Users/sac/lsp-max` — exit 0 (1m40s), binary at
  `/Users/sac/lsp-max/target/release/lsp-max-compositor`.
- `cargo build --release` from `/Users/sac/claude-code-config-lsp` — exit 0
  (1m18s), binary at `target/release/claude-code-config-lsp`.

## Findings

### F1 — REGRESSION, out-of-lane (lsp-max): end position collapses on the debounced flush publish site

Evidence chain (all run this session):
1. Child probe (raw child wire, no compositor): child publishes CCC-JSON-004
   with range **(1,2)→(1,9)** — full span.
2. Identical fixture through the compositor front door: arrives
   **(1,2)→(1,2)** — point-collapsed.
3. Root cause located by reading
   `/Users/sac/lsp-max/crates/lsp-max-compositor/src/flush_coordinator.rs`
   (editor-side materialization, `end: Position { line: d.line, character: d.character }`
   — end hardcoded to start). CL-H fixed `server.rs:582` to use
   `d.end_line`/`d.end_character` but missed this second publish site, which
   is the site the stdio run loop actually uses (the debounced flush path).
4. lsp-max's own CL-H court (`cargo test -p lsp-max-compositor --test
   diagnostic_end_position_test` → `4 passed; 0 failed` this session) is
   green while the front door is red — the court exercises buffer + merge
   in-process and never rides the flush coordinator. Coverage gap witnessed:
   a court that does not ride the production publish path cannot see this
   class (F1b).

### F1 close condition (named)

lsp-max changes flush_coordinator.rs's editor-side
`end: Position { line: d.line, character: d.character }` to
`end: Position { line: d.end_line, character: d.end_character }` (same shape
as server.rs:582). Then `#[ignore]` is removed from
`clh_end_position_survives_front_door` in `tests/compositor_relay_stress.rs`
and the court must pass with the child's (1,2)→(1,9) span relayed verbatim.
Until then the divergence is LOUD (`#[ignore = "AB-18 F1: ..."]`, printed on
every suite run), never a faked pass. The three stress tests deliberately do
not re-assert the width law in-file — commented pointers to the F1 court
instead.

## Census (the deliverable — recorded, not fenced)

n=1000 didOpen→publishDiagnostics round-trips through the compositor front
door over alternating config-heavy fixtures (violating settings.json →
CCC-JSON-004, no-H1 CLAUDE.md → CCC-MD-006). Warm-up (child spawn + pool
cost, ~1.2 ms steady) excluded. Percentile ordering asserted; no absolute-ms
fence. Post-census coherence: one more md round-trip must carry CCC-MD-006 —
holds in both runs.

| run | warmup | p50 | p95 | p99 | max |
|---|---|---|---|---|---|
| census run 1 | 1.24 ms | 1.90 ms | 3.94 ms | 7.39 ms | 17.79 ms |
| census run 2 | 1.24 ms | 1.79 ms | 3.07 ms | 5.79 ms | 32.10 ms |

Batch figures likewise from that pair: run 1 mean=17.58 ms / max=71.18 ms;
run 2 mean=11.96 ms / max=29.60 ms per file. Final gate runs (post binary
rebuild, same test code) re-witnessed the full suite green — 56.31 s /
60.66 s.

## Batch (100-file workspace, merge correctness at scale)

Generated fixtures: 50 × `dNNN/.claude/settings.json` (violating →
CCC-JSON-004) + 50 × `dNNN/CLAUDE.md` (no-H1 → CCC-MD-006). Opened in
sequence; per-file merged publish asserted on arrival: expected code present,
every code CCC- family. Timing recorded, not fenced. Span-width law (CL-H) deliberately not
asserted per-file here — owned by the F1 court.

## Standing diagnostics under load (merged-publish hazard class)

Interpretation disclosed: `src/standing.rs` (`standing_diagnostics`, H4
surface) has no callers outside its own module — the H4 standing publish
path is not wired into the backend publish path (verified by grep this
session; pre-existing, registered, not this lane's diff). The standing class
observable through the front door is compositor-buffer standing state:
diagnostics persisting in the merge buffer across re-deposits. Soak: 100
files opened, then every uri re-deposited (didOpen again, same text);
per-uri asserted: identical code set first vs second pass, no duplicate
identical ranges within a merged batch (sorted-range dedup length equality).

- `standing_diagnostics_100_file_soak_no_duplicate_uri_batch_clobbering` — ok ×2.

Note on the discovery trail: during harness bring-up, churn runs witnessed
two anomalies later isolated: (a) the (1,2)→(1,2) collapse = F1 (now owned by
its court), (b) one stale-violation publish for a uri whose final text was
valid. Witness (b) was observed only in runs where the F1-corrupted state was
in flight before the harness consume bugs were fixed; in the final gate runs
the churn ground truth holds, so the hazard class (duplicate-uri batch
clobbering / stale survival) is NOT witnessed post-isolation. The early
witnesses are retained here as the F1 discovery trail: the stress suite found
F1 before the probes did.

## Churn (10k didOpen/didChange/didClose storm)

20 files (10 settings.json + 10 CLAUDE.md), exactly 10,000 notification
sends, fixed replayable schedule; per-file final text tracked as the storm
runs (sequential-replay ground truth), then final-state check: re-open each
with its storm-final text; violating → violation code present; valid → no
stale violation survives. Buffer-leak observable: per-uri final state ==
ground truth and the transport stays responsive (every publish arrives well
inside the bounded transport timeout). Worst final-publish latency recorded:
15.79 / 17.65 ms (not fenced).

- `churn_storm_10k_events_final_state_matches_sequential_replay` — ok ×2.

## Gates (final code, this session)

```
cargo test --test compositor_relay_stress   (gate 1)
  → ok. 4 passed; 0 failed; 1 ignored; 56.31 s   exit 0
cargo test --test compositor_relay_stress   (gate 2)
  → ok. 4 passed; 0 failed; 1 ignored; 60.66 s   exit 0
cargo test (full workspace, unregression)
  → passed=595 failed=0, exit 0
```

The one ignored is the F1 court with the registered-divergence reason string
(loud, printed every run).

Environmental note: mid-session, BOTH release binaries were evicted from
their target/ dirs by a concurrent process (observed twice: lsp-max
compositor at ~22:2x, then the child at ~22:3x; rebuilt in-lane both times,
disclosed — final gate runs above are on the rebuilt pair, mtimes 22:31 /
22:35). Concurrent-lane binary eviction in shared trees is a coordinator-level
hazard worth registering.

## Verification ladder

- Child probe vs compositor probe over real stdio wire (narrow, F1 chain).
- 1000-round-trip census (integration, timing census).
- 100-file batch + standing soak (integration at scale).
- 10k-event churn vs replay ground truth (e2e).
- Full existing suite (regression): 595 passed / 0 failed.

## Standing

- Census / batch / standing soak / churn: ALIVE (ran + passed this session,
  ×2, tables above).
- CL-H end-position through the front door: **registered divergence F1** —
  blocked on lsp-max repair; court loud-ignored with named close condition.
- F1b (coverage gap): lsp-max's CL-H court does not ride the production
  flush path; green-while-red witnessed. Belongs to lsp-max lanes.
- H4 standing-diagnostics wiring (no backend caller): pre-existing,
  registered, not this lane's diff.

## Falsifier for the F1 close condition

After the lsp-max fix lands:
`cargo test --test compositor_relay_stress clh_end_position_survives_front_door -- --ignored`
must pass with range (1,2)→(1,9); if it still arrives (1,2)→(1,2), the close
condition is not met.
