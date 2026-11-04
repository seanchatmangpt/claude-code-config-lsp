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

## See Also

- `closure/c-wave-close.md` — full C-wave closure record (per-target exits, residuals, verdict)
- `receipts/` — per-lane receipts (all read-only verified at closure)
