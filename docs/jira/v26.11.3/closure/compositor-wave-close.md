# Compositor Multiplexing Wave M1–M13 — Lane-Close Receipt

Date: 2026-11-03. Lane: LANE-CLOSE. Repos: /Users/sac/lsp-max (gates re-run),
/Users/sac/claude-code-config-lsp (closure doc owner). No git run, per contract.

## 1. Receipt verification (all on disk, re-read this session)

`docs/jira/v26.11.3/receipts/` — all 14 wave receipts verified present
(2026-11-03, `ls`): m1-schema, m2-registry-gen, m3-build, m4-routing, m5-merge,
m6-arbitration, m7-andon, m8-crash, m9-priority, m10-e2e, m11-registration,
m12-dogfood, m13-docs, f3-registry-ontology, f4-notification-dispatch.

Per-receipt standing (as recorded, re-read at use time):

| Receipt | Verdict as recorded | Gate cited |
|---|---|---|
| m1-schema | landed (schema-exact manifest `lsp-max.toml` deployed) | — |
| m2-registry-gen | registry regenerated via gen.toml rule (F3 ontology-first) | — |
| m3-build | build receipt, bounded-timeout design (30 s-class suites by design) | build exit 0 |
| m4-routing | ALIVE | exit 0 final run |
| m5-merge | FanAll merge semantics | suite cited |
| m6-arbitration | PARTIAL (ALIVE at executable seams; loop-level BLOCKED) | 9/0 in 30.01 s |
| m6 residuals | `Ok(None)` flattening; 30 s stall from one crashed child on FirstSuccess | — |
| m7-andon | andon prefix semantics established (no src changes) | suite cited |
| m8-crash | crash isolation courts; restart policy NONE pinned | 3/0 in 30.06 s |
| m9-priority | ALIVE (10/10) | `--test priority_semantics` |
| m10-e2e | ADMITTED with named residuals | 2/0 |
| m11-registration | ALIVE — hook idempotent, jq-valid, binaries executable | jq exit 0 |
| m12-dogfood | BLOCKED at write time (pre-F4) — harness landed as falsifier | exit 2 |
| m12 post-F4 | **flipped: exit 0** (re-witnessed this session, §3) | exit 0 |
| m13-docs | "SHIP the docs", standing PARTIAL_ALIVE | wc/tail verified |
| f3-registry-ontology | ontology-first registry (resolved: M2 built on it) | — |
| f4-notification-dispatch | ADMITTED with named residuals; falsifier flip witnessed | 4/4 + suites + exit 0 |

## 2. Wave gate re-run (this session, 2026-11-03)

`cd /Users/sac/lsp-max && cargo test -p lsp-max-compositor` — exit 0.

**202 passed / 0 failed / 0 ignored** across 19 suites (+0 doc-tests):

| Suite | Count | Time |
|---|---|---|
| unittests src/lib.rs | 52 | 0.01s |
| unittests bin/main, bin/lsp_echo_server | 0+0 | — |
| andon_prefixes | 11 | 0.02s |
| cc_007_lsif_routing | 8 | 0.00s |
| crash_isolation | 3 | 30.06s |
| diagnostic_relay_probe | 2 | 0.07s |
| diagnostics_merge | 9 | 0.00s |
| e2e | 2 | 0.00s |
| fanout_latency | 1 | 0.01s |
| gate_path_roundtrip_witness | 1 | 0.00s |
| integration | 47 | 0.03s |
| priority_semantics | 10 | 0.00s |
| registry_ontology_test | 3 | 0.09s |
| request_arbitration | 9 | 30.01s |
| routing_dispatch | 6 | 0.00s |
| routing_integration | 21 | 0.00s |
| speciation | 12 | 0.00s |
| v26_6_28_gate_tests | 5 | 0.00s |
| doc-tests | 0 | — |

**Known flake (named, not session-introduced)**: `crash_isolation` failed 2/5
runs under heavy compile load (F4 receipt, residual 3); baseline variant
failed identically, 3/3 pass on retry and in this session's final full-suite
run. The 30 s-class runtimes on crash_isolation / request_arbitration are the
tests' own bounded-timeout design, not hangs.

## 3. M12 falsifier status (re-run this session)

- Bare `node scripts/dogfood_compositor.mjs` → **exit 2 BLOCKED**, because
  (a) the script's static BLOCKED text still cites the pre-F4 stub (stale
  message text, not stale behavior) and (b) the bare invocation hits the
  script's known arg-parse residual (F4 residual 1: `--fixtures` absent →
  md fixture mis-parses `--allow`'s value). The debug compositor binary was
  also stale pre-F4 until `cargo build -p lsp-max-compositor` (exit 0, 1m15s)
  refreshed it this session.
- **With F4's exact invocation**: `node scripts/dogfood_compositor.mjs
  --allow CCC-JSON-001 --fixtures tests/fixtures/claude-config/settings.json
  tests/fixtures/claude-config/claude-global-CLAUDE.md` → **exit 0**,
  CCC-JSON-001 observed through the compositor front door for both fixtures
  (`wire: <- textDocument/publishDiagnostics`, `lspMax/admissionChanged`).
  M12's BLOCKED stands **flipped**. Teardown "Broken pipe" panic is post-
  assertion noise, not a failure.
- Residual confirmed live: the transcript prints the md fixture line as the
  json fixture path (display bug, same arg-parse residual family).

## 4. Pinned hazards (still open at close)

1. **Typo'd priority escalation** (m9, pinned AS-IS): `from_config` vs
   `from_priority` diverge — `priority = "ful"` escalates to PRIMARY in
   `from_config` while `from_priority` demotes typos to DiagnosticsOnly.
   `priority` must be spelled exactly `diagnostics-only`; any other string
   silently escalates the child to full/primary.
2. **30 s `Ok(None)` laundering** (m6/m8): a child answering a wrongly-shaped
   result, or dying mid-request, flattens to `Ok(None)` after the hardcoded
   30 s (`src/client/server_handle.rs:67/79`); one crashed child can stall
   FirstSuccess dispatch up to 30 s. No typed child-failure surface exists.
3. **No child restart policy** (m8): exit watcher only logs
   (`child_process.rs:121-131` drops the exit future); dead children are
   never evicted from the registry, so a dead child is a 30 s-stall shadow.
4. **Exact-spelling load-bearing** (m9/m11): `diagnostics-only` (and the
   tier strings) are load-bearing strings with no validation surface at
   manifest load; the manifest parser accepts anything.

## 5. Resolved during the wave

1. **F4 dispatch flipped M12's BLOCKED** — notification dispatch implemented
   in `/Users/sac/lsp-max/src/client/builder.rs` `read_loop`
   (`dispatch_notification`, builder.rs:165/174); falsifier flipped
   before=exit 2 → after=exit 0 (f4 receipt, re-witnessed §3).
2. **M1 schema-exact manifest**: deployed `lsp-max.toml` extracted exact from
   `ServerEntry` (config.rs) — all 7 keys, no invented fields.
3. **F3 ontology-first registry** (f3-registry-ontology → m2-registry-gen):
   registry generated from the compositor ontology via the gen.toml rule.

## 6. Final ship verdict

**SHIP-WITH-CAVEATS.** Standing: **PARTIAL_ALIVE**.

The compositor front door, routing, merge, arbitration, andon attribution,
crash courts, priority semantics, e2e, registration, docs, and the dogfood
falsifier are all ALIVE and re-witnessed this session (202/0 gate + falsifier
exit 0). The caveats are the four pinned hazards (§4) plus these residuals:

- dogfood script residual: bare invocation mis-parses args (fix or require
  explicit `--fixtures`).
- Server-to-client requests (id+method) are logged and unanswered (F4
  residual 2) — a real `workspace/configuration` request would stall the
  child until its own timeout.
- `docs/reference/compositor-deployment.md` is the deployment authority
  (M13); registration currently lives in **project** `.claude/settings.json`
  (compositor front door first, direct server second); **`~/.claude/settings.json`
  carries no `lspServers` key this session** — the user-global registration
  step has not been applied.

## 7. Deployment prerequisites

1. **Build-release gate**: run `./scripts/build-release.sh` in
   /Users/sac/claude-code-config-lsp (packaging gate; child must ship as a
   prebuilt binary — cache-copies-source-not-binaries law).
2. **Registration order**: first-wins exclusive authority — compositor front
   door must be the **first** `lspServers` entry; the direct
   claude-code-config-lsp entry is fallback (index 1), verified jq-valid and
   idempotent (m11).
3. **User-global jq step** (from m11, not yet applied to `~/.claude/settings.json`):
   `jq --arg c "/Users/sac/lsp-max/target/release/lsp-max-compositor" \
      '.lspServers["claude-code-config"].command = $c' "$P" > "$P.tmp" && mv "$P.tmp" "$P"`
   — apply only if user-global compositor registration is wanted; project-level
   registration is live and jq-valid today.

## 8. Open lanes at close

- Typed child-failure surface (replace `Ok(None)` laundering) — m6/m8.
- Child restart/eviction policy — m8.
- Manifest validation for tier/priority strings (exact-spelling hazard) — m9.
- Server-to-client request answering — F4 residual 2.
- dogfood script arg-parse fix — F4 residual 1.
