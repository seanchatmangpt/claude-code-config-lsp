# Ph Wave Close

P-wave (P1-P12) and H-wave (H1-H5) + X2 closure record, v26.11.3.
Repo: /Users/sac/claude-code-config-lsp, HEAD 29c967fafbcbb53e2e64fee396d32012bce20d2d
at closure time. No git run by this lane; src/** untouched.

## Receipt inventory (all 17 on disk)

docs/jira/v26.11.3/receipts/: p1-navigation, p2-ws-symbols, p3-rename-inlay,
p4-lens-signature, p5-formatting, p6-selection-highlight, p7-hierarchy-moniker,
p8-delta-color, p9-commands, p10-ttl-reconciliation, p12-wiring,
h1-harness-docs, h2-ops-matrix, h3-packaging, h4-standing, h5-overlay,
x2-pin-bump — every file present with session commands + exits.

## Scoped gate re-runs (this lane, 2026-11-03)

| Target | Exit | Result |
|---|---|---|
| navigation | 0 | 22 passed / 0 failed |
| workspace_symbols | 0 | 5 passed / 0 failed |
| rename_inlay | 0 | 13 passed / 0 failed |
| lens_signature | 0 | 7 passed / 0 failed |
| formatting | 0 | 15 passed / 0 failed |
| selection | 0 | 15 passed / 0 failed |
| hierarchy_moniker | 0 | 13 passed / 0 failed |
| tokens_delta | 0 | 5 passed / 0 failed |
| tokens_color | 0 | 11 passed / 0 failed |
| commands_analyzer | 0 | 5 passed / 0 failed |
| ttl_reconciliation | **101** | 10 passed / 1 failed — see Gap G1 |
| harness_docs | 0 | 11 passed / 0 failed |
| harness_ops | 0 | 25 passed / 0 failed |
| standing | 0 | 10 passed / 0 failed |
| overlay | 0 | 9 passed / 0 failed |

14/15 targets exit 0. Aggregate: 156 passed / 1 failed across 15 targets.

## Script gates

- `scripts/build-release.sh` — **exit 0**. Release binary + sha256 stamped into
  `dist/claude-code-config-lsp-26.6.24`; plugin cache binary regenerated
  (`~/.claude/plugins/cache/claude-code-config-lsp/.../0.1.0`).
- `scripts/dogfood_compositor.mjs` — **exit 1** — see Gap G2 (this FAIL is the
  F4 flip consequence, not a regression).

## F4 flip status (recorded per contract)

F4 (notification dispatch in lsp-max read_loop) is **ADMITTED** per
`receipts/f4-notification-dispatch.md` — publishDiagnostics now reaches the
compositor client. Consequence witnessed live in the dogfood run: the wire log
shows `textDocument/publishDiagnostics` frames arriving, and the child's real
`CCC-JSON-001` diagnostic surfaces to the client for the first time — which is
what trips G2.

## Per-lane standing

| Lane | Surface | Standing |
|---|---|---|
| P1 | navigation | ALIVE (22/0) |
| P2 | workspace symbols | ALIVE (5/0) |
| P3 | rename + inlay | ALIVE (13/0) |
| P4 | code lens + signature help | ALIVE (7/0) |
| P5 | formatting | ALIVE (15/0) |
| P6 | selection + highlight | ALIVE (15/0) |
| P7 | hierarchy + moniker | ALIVE (13/0) |
| P8 | tokens delta + color | ALIVE (5/0 + 11/0) |
| P9 | commands analyzer | ALIVE (5/0) |
| P10 | TTL reconciliation | **PARTIAL** — court anchor rot (G1) |
| P12 | provider wiring | ALIVE (provider_registry 13/0; delegation one-liners in backend.rs) |
| H1 | harness docs | ALIVE (11/0) |
| H2 | ops matrix | ALIVE (25/0) |
| H3 | packaging | ALIVE (build-release exit 0, re-witnessed this lane) |
| H4 | standing vocabulary | ALIVE (10/0) |
| H5 | overlay | ALIVE (9/0) |
| X2 | lsp-max pin bump | ALIVE (path dep until 26.11.3 publishes to crates.io) |

## Provider-surface table (12 provider modules → P12 backend overrides)

`src/provider_registry.rs` (522 lines, 13 unit tests) carries pure
handler-shaped delegates for all 12 modules; `src/backend.rs` wires each via a
one-line override into the `LanguageServer` impl:

| Provider module | Test target | Exit | Backend override(s) |
|---|---|---|---|
| navigation | navigation | 0 | goto_definition, references, document_highlight |
| workspace_symbols | workspace_symbols | 0 | symbol |
| rename_inlay (rename) | rename_inlay | 0 | prepare_rename, rename |
| rename_inlay (inlay) | rename_inlay | 0 | inlay_hint |
| code_lens | lens_signature | 0 | code_lens |
| signature_help | lens_signature | 0 | (signature help, lens_signature target) |
| formatting | formatting | 0 | formatting family |
| selection | selection | 0 | document_highlight family |
| moniker | hierarchy_moniker | 0 | moniker |
| hierarchy | hierarchy_moniker | 0 | prepare_call_hierarchy, incoming_calls, outgoing_calls |
| tokens_delta | tokens_delta | 0 | semanticTokens full/delta |
| color | tokens_color | 0 | documentColor / colorPresentation |

X2's pin bump to lsp-max 26.11.3 (path dep) carries the whole surface; zero
src/** edits were needed for the bump itself.

## Harness-assistant surface (H-wave)

- Virtual docs (H1): `claude-config://health` and kin served through the
  harness; harness_docs 11/0.
- 9-op matrix (H2): 25/25 harness_ops tests, warning-free module.
- Standing diagnostics (H4): ALIVE = receipt green + fresh + zero findings,
  PARTIAL = green+fresh with findings; standing target 10/0.
- Overlay (H5): partial overlay layers, missing keys are normal never
  diagnosed; overlay target 9/0.
- Packaging + compositor (H3 + X2 + F4): build-release exit 0; lsp-max.toml
  routes .json/.md at diagnostics-only priority; F4 flipped — diagnostics now
  flow across the compositor wire (witnessed in G2's run).

## Honest gaps (never rounded up)

### G1 — P10 court anchor rot (real, post-landing drift)

`cargo test --test ttl_reconciliation` exits 101:
`every_deprecated_key_literal_appears_in_ttl` panics at
tests/ttl_reconciliation.rs:163 "court cannot find rule CCC-JSON-001". The
court anchors on the single-line form `Rule::new("CCC-JSON-001"`, but
src/analyzers/json.rs:789-812 now carries the multi-line form
(`Rule::new(` + `"CCC-JSON-001",` on the next line) — a later lane's json.rs
edit (post-P10-landing, I2/C2-era) desynced the anchor. The court's semantic
content is intact: all 3 codes and 9 deprecated-key literals are present in
json.rs; only the locator rotted. P10 landed 11/0; this lane re-ran and got
10/1. Classification: S1-class anchor drift; fix is a one-line anchor widening
in tests/ttl_reconciliation.rs (not this lane's files). Standing for P10
recorded PARTIAL, attributed to the post-landing json.rs edit, not to P10's
diff.

### G2 — dogfood_compositor.mjs FAIL = F4 flip consequence

`node scripts/dogfood_compositor.mjs` exits 1: "diagnostics not covered by
--allow: tests/fixtures/claude-config/settings.json CCC-JSON-001". Root cause:
the fixture itself carries deprecated key `enabledMcpjsonServers`; before F4,
no diagnostics ever reached the client, so the allow-list never needed this
entry; after F4's dispatch fix, the real diagnostic flows and the script
correctly fails closed. The fix is a fixture key fix (settings.json fixture
should be clean) or an allow-list entry in scripts/dogfood_compositor.mjs
(not this lane's files). F4 itself is ADMITTED — this FAIL is the flip
working, diagnostics demonstrably reach the client over the real wire.

### G3 — P12 registered limitations (from p12-wiring.md, unchanged)

- 2 pre-existing compile warnings (not P12's diff).
- Helpers `provider_roots`/`provider_corpus` moved into the inherent impl
  block (trait impls cannot carry non-trait methods) — registered, not a
  defect.
- The 2 foreign-lane runtime failures P12 witnessed (tokens_delta witness
  splice units, json span drift) were subsequently fixed by their owners
  (I3 witness flip; I2 goldens 16/16) — both green in this lane's re-run.

### G4 — pre-existing, repo-wide

- FM-GEN-007: `ggen sync` blocked upstream (ggen core bug, documented in
  CLAUDE.md); I3's hand-corrected classifier must reconcile with the TTL when
  sync restores (number legend entry pending an ontology individual).
- X2: lsp-max 26.11.3 is a PATH dep; registry head is 26.7.3. Drop `path`
  when 26.11.3 publishes.

## Wave verdict

**ALIVE with 2 named residuals.** All 17 receipts verified on disk; 14/15
scoped test targets exit 0 (156/157 tests); build-release exit 0; dogfood
exit 1 is the F4 flip consequence with a named one-line-class fix (G2), and
the ttl court failure is locator rot with intact semantics (G1). Neither gap
invalidates any lane's delivered surface. P10 standing PARTIAL (G1); all
other lanes ALIVE.
