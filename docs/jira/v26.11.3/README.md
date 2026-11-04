# v26.11.3

Wave workspace for claude-code-config-lsp v26.11.3. Plans live at this level
(`c1-acceptance.md`, `m*.md`, `p*.md`, ...); receipts live in `receipts/`;
closure records in `closure/`.

## C-wave lane table

Standing is as-of 2026-11-03 closure re-run (docs/jira/v26.11.3/closure/c-wave-close.md).
Full-repo `cargo test` single-command green run: not witnessed at closure.

| lane | receipt | plan | standing | scoped gate (this session) |
|---|---|---|---|---|
| C1 | receipts/c1-acceptance.md | c1-acceptance.md | ALIVE | acceptance_cli 13/0 |
| C2 | receipts/c2-json-spans.md | c2-json-spans.md | ALIVE | --lib analyzers::json 64/0; goldens 16/0 |
| C3 | receipts/c3-position-aware.md | c3-position-aware.md | ALIVE | --lib completion 9/0, hover 6/0 |
| C4 | receipts/c4-markdown-nav.md | c4-markdown-nav.md | ALIVE | markdown_nav 5/0 |
| C5 | receipts/c5-protocol.md | c5-protocol.md | ALIVE | protocol_roundtrip 3/0 |
| C6 | receipts/c6-evidence-layer.md | c6-evidence-layer.md | ALIVE | receipt_layer 14/0, praxis 7/0 |
| C7 | receipts/c7-bcinr-parity.md | c7-bcinr-parity.md | PARTIAL_ALIVE | bcinr_parity 4/0 naive; SIMD leg UNSUPPORTED (Cargo.toml dep-name defect) |
| C9 | receipts/c9-schema-expansion.md | c9-schema-expansion.md | BLOCKED | schema_expansion 4/2 (court source-grep vs rustfmt'd rows) |
| C10 | receipts/c10-readme-langid.md | c10-readme-langid.md | ALIVE | no cargo target; receipt hook gate (run-twice) verified in receipt |
| C11 | receipts/c11-goldens.md | c11-goldens.md | ALIVE | diagnostic_goldens 16/0 |
| I2 | receipts/i2-json003.md | — | ALIVE | diagnostic_goldens 16/0 |
| I3 | receipts/i3-tokens-substrate.md | — | ALIVE | semantic_tokens 10/0, tokens_color 11/0 |
| I7 | receipts/i7-delta-boundary.md | — | ALIVE | tokens_delta 5/0, tokens_color 11/0 |
| K4 | receipts/k4-md-rules.md | — | ALIVE | diagnostic_goldens 16/0; --lib claude_md 37/0 |
| R1 | receipts/r1-admission.md | — | ALIVE | admission 12/0 |
| R2 | receipts/r2-multiplex-real.md | — | ALIVE | multiplex_real_lsps 2 passed; 0 failed (four real LSPs — rust-analyzer, gopls, elixir-ls, config-lsp — multiplexed over the real wire; receipt landed after the closure re-run, row reconciled 2026-11-03) |

## CL / E80 wave section

Standing words re-read from each receipt body on disk (2026-11-03, cl-close
lane), never from memory. Exit record law: A gate is witnessed by an exit record: either (a) the process exit status, or (b) cargo's "N passed; 0 failed" summary line. A passed-count without a failed-count is not an exit record.

| Lane | Receipt | Standing | Gate (exit record) |
|---|---|---|---|
| CL-A | receipts/cl-a-smalt-layout.md | ALIVE (contracted scope) | marketplace.py validate exit 2 → exit 0 (both X5 refusal classes gone for smalt-core-pack); rdflib parse exit 0. Subject repo /Users/sac/ggen-marketplace |
| CL-B | receipts/cl-b-c9-court.md | ALIVE | schema_expansion 6 passed; 0 failed — exit 0 |
| CL-C | receipts/cl-c-clippy.md | ALIVE | clippy -D warnings exit 0; cargo test 438 passed / 1 failed (the 1 failure pre-existing, environmental, byte-identical to baseline — zero-delta) |
| CL-L | receipts/cl-l-writev.md | ALIVE | stream_writev_witness 3 passed; 0 failed — exit 0. Subject repo /Users/sac/cgap |
| CL-M | receipts/cl-m-dogfood-fix.md | ALIVE | dogfood_compositor.mjs both invocations exit 0 |
| CL-T | receipts/cl-t-wiring-verify.md | ALIVE | cargo build exit 0; --lib 264 passed / 0 failed; protocol_roundtrip 3 passed / 0 failed; dogfood.mjs exit 0 |
| CL-U | receipts/cl-u-smalt-qual.md | ALIVE (contracted scope; pack projection not witnessed — templates/ empty) | qualify_packs.py exit 0 twice, byte-identical replay. Subject repo /Users/sac/ggen-marketplace |
| e80-06 | receipts/e80-06.md | ALIVE | stream_writer_type_test 7 courts green, exit 0. Subject repo /Users/sac/cgap; receipt hosted in config-lsp's wave tree; cgap has no v26.11.3 wave tree — pointer-only, no relocation |
| e80-07 | receipts/e80-07.md | ALIVE | ipc_body_integrity_test 5/5 green. Subject repo /Users/sac/cgap; receipt hosted in config-lsp's wave tree; cgap now has its own index at /Users/sac/cgap/docs/jira/v26.11.3/README.md (created cl-close-2) |
| CFL-1 | receipts/cfl-1.md | lane complete, ready for coordinator integration (standing word per receipt body) | ttl_reconciliation "13 passed; 0 failed"; commands_dispatch "12 passed; 0 failed"; diagnostic_goldens "19 passed; 0 failed"; compositor_e2e "2 passed; 0 failed"; full `cargo test` "591 passed, 0 failed across 33 suites", exit 0 |
| SA-9 | receipts/sa-9.md | ALIVE | `ggen sync run` exit 0, `written: []`, all six tracked generated files byte-identical; cargo test 591 passed / 0 failed (CFL-1 baseline); clippy -D warnings per receipt |

## Legacy receipts predating this README (pointer-only, cl-close-2)

37 receipts landed in `receipts/` before this README existed; their register
of truth is closure/c-wave-close.md (per-receipt exits, residuals, verdict).
Disclosed pointer-only: not re-indexed per-receipt in this pass (cl-close-2
retro-index decision). Files on disk (census 2026-11-03):

f3-registry-ontology, f4-notification-dispatch, ga4-cgap-arrow, h1-harness-docs,
h2-ops-matrix, h3-packaging, h4-standing, h5-overlay, i4-sync-restore,
l1-version-bump, l2-docs-bump, m1-schema, m2-registry-gen, m3-build,
m4-routing, m5-merge, m6-arbitration, m7-andon, m8-crash, m9-priority,
m10-e2e, m11-registration, m12-dogfood, m13-docs, p1-navigation, p2-ws-symbols,
p3-rename-inlay, p4-lens-signature, p5-formatting, p6-selection-highlight,
p7-hierarchy-moniker, p8-delta-color, p9-commands, p10-ttl-reconciliation,
p12-wiring, x2-pin-bump, x5-smalt-core-manifest.

## Full receipts index (zd-25, 2026-11-04)

One row per file on disk in `receipts/` (69 files). Subject = first non-empty
body line; standing = first ALIVE/PARTIAL/BLOCKED/UNVERIFIED/PASS token found
in the file, UNSTATED if none. This supersedes the pointer-only legacy list
above — the register of truth for legacy rows remains closure/c-wave-close.md.

| receipt | file | subject | standing |
|---|---|---|---|
| ab-18-relay-stress | receipts/ab-18-relay-stress.md | AB-18 — Compositor relay stress (STRESS lane receipt) | ALIVE |
| af-23-h4-wiring | receipts/af-23-h4-wiring.md | AF-23 — H4 standing-diagnostics publish wiring (IMPLEMENTATION lane receipt) | ALIVE |
| c1-acceptance | receipts/c1-acceptance.md | C1 Acceptance Receipt — conformance/receipt/fix CLI verbs | ALIVE |
| c10-readme-langid | receipts/c10-readme-langid.md | Receipt: c10-readme-langid (v26.11.3) | PARTIAL |
| c11-goldens | receipts/c11-goldens.md | C11 — Diagnostic goldens receipt | ALIVE |
| c2-json-spans | receipts/c2-json-spans.md | C2: JSON analyzer real spans (de-sentinel) | ALIVE |
| c3-position-aware | receipts/c3-position-aware.md | Receipt: C3 — position-aware completion/hover (claude-code-config-lsp) | PARTIAL |
| c4-markdown-nav | receipts/c4-markdown-nav.md | C4 Receipt: Markdown Navigation (documentSymbol / foldingRange / documentLink) | ALIVE |
| c5-protocol | receipts/c5-protocol.md | Receipt: C5 protocol round-trip suite (claude-code-config-lsp) | ALIVE |
| c6-evidence-layer | receipts/c6-evidence-layer.md | C6 — Evidence Layer Receipt (praxis Evidence + receipt chain + conformance/coverage/declare) | ALIVE |
| c7-bcinr-parity | receipts/c7-bcinr-parity.md | Receipt: C7 — bcinr SIMD vs naive parity | ALIVE |
| c9-schema-expansion | receipts/c9-schema-expansion.md | C9 Schema Expansion Receipt — settings.json key inventory | BLOCKED |
| cfl-1 | receipts/cfl-1.md | CFL-1 Implementation Lane Receipt (G1 + G2) | ALIVE |
| cl-a-smalt-layout | receipts/cl-a-smalt-layout.md | CL-A: smalt-core-pack layout (ONTOLOGY_SOURCE_MISSING unblock) | ALIVE |
| cl-b-c9-court | receipts/cl-b-c9-court.md | cl-b-c9 — schema_expansion court rustfmt-stability receipt | ALIVE |
| cl-c-clippy | receipts/cl-c-clippy.md | cl-c-clippy — clippy -D warnings gate (lane cl-c, v26.11.3) | ALIVE |
| cl-l-writev | receipts/cl-l-writev.md | cl-l-writev — StreamWriter writev / UDS witness (lane cl-l, v26.11.3) | ALIVE |
| cl-m-dogfood-fix | receipts/cl-m-dogfood-fix.md | Receipt: cl-m-dogfood-fix — dogfood_compositor.mjs arg-parse + rebuild gate + de-stale BLOCKED peers | BLOCKED |
| cl-t-wiring-verify | receipts/cl-t-wiring-verify.md | CL-T — post-I4 provider wiring verification (receipt) | ALIVE |
| cl-u-smalt-qual | receipts/cl-u-smalt-qual.md | CL-U: smalt-core-pack qualification (post-CL-A unblock) | ALIVE |
| e80-06 | receipts/e80-06.md | e80-06 | ALIVE |
| e80-07 | receipts/e80-07.md | e80-07 — ipc body integrity at the framing layer (RAISE) | PASS |
| f3-registry-ontology | receipts/f3-registry-ontology.md | F3 — registry ontology-first generation receipt | ALIVE |
| f4-notification-dispatch | receipts/f4-notification-dispatch.md | F4: Client notification dispatch — M12 falsifier flip | BLOCKED |
| ga4-cgap-arrow | receipts/ga4-cgap-arrow.md | GA4 cgap-arrow Import Primitives Receipt | ALIVE |
| h1-harness-docs | receipts/h1-harness-docs.md | H1 Receipt: harness virtual documents | UNVERIFIED |
| h2-ops-matrix | receipts/h2-ops-matrix.md | Receipt: H2 operations matrix (claude-code-config-lsp) | ALIVE |
| h3-packaging | receipts/h3-packaging.md | H3 Packaging/Compositor Receipt — v26.11.3 | ALIVE |
| h4-standing | receipts/h4-standing.md | H4 Standing Lane Receipt | ALIVE |
| h5-overlay | receipts/h5-overlay.md | H5 — Settings Overlay Intelligence (IMPLEMENTATION lane receipt) | ALIVE |
| i2-json003 | receipts/i2-json003.md | I2 — CCC-JSON-003 implementation lane receipt | ALIVE |
| i3-tokens-substrate | receipts/i3-tokens-substrate.md | I3 — Semantic-Tokens Substrate Fix (generated classifier hand-correction) | ALIVE |
| i4-sync-restore | receipts/i4-sync-restore.md | I4: Sync Restore — Three-Zone Law Executed (receipt) | PARTIAL |
| i7-delta-boundary | receipts/i7-delta-boundary.md | I7 — Delta-Encoding Boundary Fix (encode_abs continuation seed) | ALIVE |
| k4-md-rules | receipts/k4-md-rules.md | K4 — CCC-MD golden-gap closure receipt | ALIVE |
| l1-version-bump | receipts/l1-version-bump.md | L1 — Version bump 26.7.3 → 26.11.3 (all 7 workspace crates) | ALIVE |
| l2-docs-bump | receipts/l2-docs-bump.md | L2 Docs Version Bump — Receipt (v26.11.3) | ALIVE |
| m1-schema | receipts/m1-schema.md | M1 — Compositor Manifest Schema Extraction + Manifest Fix | PASS |
| m10-e2e | receipts/m10-e2e.md | M10: Compositor front-door end-to-end test — Claude-Code-shaped LSP loop | PASS |
| m11-registration | receipts/m11-registration.md | Receipt: M11 — compositor registered as Claude Code front door | ALIVE |
| m12-dogfood | receipts/m12-dogfood.md | M12 — Dogfood Through the Compositor (scripts/dogfood_compositor.mjs) | BLOCKED |
| m13-docs | receipts/m13-docs.md | M13 — compositor documentation receipt | ALIVE |
| m2-registry-gen | receipts/m2-registry-gen.md | M2 Receipt: compositor registry regeneration (lsp-max gen.toml rule) | PASS |
| m3-build | receipts/m3-build.md | M3 Build Receipt — lsp-max-compositor | PASS |
| m4-routing | receipts/m4-routing.md | M4 Routing Dispatch — Receipt | ALIVE |
| m5-merge | receipts/m5-merge.md | Receipt: m5-merge — FanAll diagnostics merge semantics | ALIVE |
| m6-arbitration | receipts/m6-arbitration.md | M6 — Request Arbitration Tests | PARTIAL |
| m7-andon | receipts/m7-andon.md | M7 Receipt — andon_code_prefixes compositor tests | ALIVE |
| m8-crash | receipts/m8-crash.md | M8 — Compositor child-crash isolation (IMPLEMENTATION lane receipt) | PASS |
| m9-priority | receipts/m9-priority.md | M9 Receipt — Priority Semantics (diagnostics-only vs full) | ALIVE |
| p1-navigation | receipts/p1-navigation.md | P1 Navigation Lane Receipt (v26.11.3) | UNSTATED |
| p10-ttl-reconciliation | receipts/p10-ttl-reconciliation.md | P10 — TTL Reconciliation Receipt | ALIVE |
| p12-wiring | receipts/p12-wiring.md | P12 — provider wiring receipt (integration wave input) | UNSTATED |
| p2-ws-symbols | receipts/p2-ws-symbols.md | P2 workspace/symbol Receipt | ALIVE |
| p3-rename-inlay | receipts/p3-rename-inlay.md | Receipt: P3 rename + inlay lane | PASS |
| p4-lens-signature | receipts/p4-lens-signature.md | Receipt: p4-lens-signature | ALIVE |
| p5-formatting | receipts/p5-formatting.md | Receipt: P5 — document/range/on-type formatting for JSON config surfaces | UNVERIFIED |
| p6-selection-highlight | receipts/p6-selection-highlight.md | Receipt: P6 — selectionRange + documentHighlight (JSON & markdown) | ALIVE |
| p7-hierarchy-moniker | receipts/p7-hierarchy-moniker.md | Receipt: P7 — Call hierarchy + monikers for config surfaces | ALIVE |
| p8-delta-color | receipts/p8-delta-color.md | P8 Receipt: semantic-tokens delta + documentColor (v26.11.3) | ALIVE |
| p9-commands | receipts/p9-commands.md | Receipt: P9 — slash-command surface (commands analyzer) | ALIVE |
| r1-admission | receipts/r1-admission.md | R1 — admission-controller lane receipt | PARTIAL |
| r2-multiplex-real | receipts/r2-multiplex-real.md | R2 Multiplex Real LSPs Receipt | ALIVE |
| r80-13 | receipts/r80-13.md | r80-13 — ggen-lsp compositor child registration (IMPLEMENTATION lane) | UNSTATED |
| sa-9 | receipts/sa-9.md | SA-9 — ggen regeneration-stability court (GGM-SRC-002 / TX-9) | ALIVE |
| x2-pin-bump | receipts/x2-pin-bump.md | X2 — lsp-max pin bump 26.7.3 → 26.11.3 (config-lsp consumer) | ALIVE |
| x5-smalt-core-manifest | receipts/x5-smalt-core-manifest.md | X5: smalt-core-pack manifest (MANIFEST_MISSING unblock) | PARTIAL |
| xd-16-relay-standing | receipts/xd-16-relay-standing.md | XD-16 — Relay-path standing-unverified reds (root-cause + fix receipt) | ALIVE |
| xd-9-release-bin-env | receipts/xd-9-release-bin-env.md | XD-9: Release Binary Environment Repair (XA-5 Finding) | ALIVE |
| zd-25-index | receipts/zd-25-index.md | ZD-25 — receipts-dir index receipt (v26.11.3) | ALIVE |

## See Also

- `closure/c-wave-close.md` — full C-wave closure record (per-target exits, residuals, verdict)
- `receipts/` — per-lane receipts (all read-only verified at closure)
