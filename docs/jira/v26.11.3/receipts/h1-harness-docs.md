# H1 Receipt: harness virtual documents

Lane: H1 (IMPLEMENTATION) · Repo: /Users/sac/claude-code-config-lsp · 2026-11-03

## Subject

`claude-config://inventory`, `claude-config://receipts`,
`claude-config://standing` — three markdown-rendered harness inspection
documents expanding the virtual-doc surface beyond `claude-config://health`.

## Diff

- **NEW `src/harness_docs.rs`** (owned): `render_inventory` /
  `render_receipts` / `render_standing` + pure helpers
  (`surface_rows`, `collect_receipt_rows`, `standing_rows`, `score_summary`,
  `staleness_of`, `agent_scope_of`, `render_uri`, `is_harness_doc_uri`),
  `KNOWN_SURFACES` registry, 9 inline tests.
- **`src/virtual_docs.rs`** (owned): added `render_uri(uri, root)` dispatcher
  (health keeps its render; new URIs → `harness_docs`; unknown → `None`).
- **`src/lib.rs`** (not in ownership list — disclosed minimal addition):
  `pub mod harness_docs;` — required for compile; other lanes concurrently
  added `harness_ops`/`standing` modules to the same file.
- **`tests/harness_docs.rs`** (owned): 11 Chicago tests over the real C5
  fixture corpus (no mocks; one temp-dir real `latest.json` fleet file).
- **Fixture restore**: earlier runs (pre-fix) minted 2 receipts into
  `tests/fixtures/claude-config/.claude/receipts/` via
  `conformance_report → maybe_issue_receipt`; both deleted, the original
  `19b294d5….json` kept.

## Commands / exits

- `cargo test --test harness_docs` → 11 passed, 0 failed
- `cargo test --lib` → 253 passed, 0 failed (exit 0; includes 9 inline
  harness_docs tests)
- `cargo clippy --lib --tests` → zero findings attributable to lane files
- `rustfmt --check` on all three lane files → clean
- Determinism: lane tests rerun 3x, all green each time

## Notes for P12 (backend wiring owner)

- Registration point:
  `ClaudeCodeConfigBackend::text_document_content` (backend.rs ~line 463)
  currently calls `virtual_docs::render()` unconditionally. Replace with
  `virtual_docs::render_uri(uri, root)` where `root` = backend workspace
  root; `None` → `method_not_found()`. `is_virtual_doc_uri` already accepts
  all `claude-config://` URIs so no capability flag change is required.
- `HarnessScope` in harness_docs.rs mirrors backend's private `AgentScope`
  by path shape; unify if desired when wiring.

## Design decisions

- **Read-only law**: `render_inventory` deliberately does NOT call
  `conformance_report` — that function writes a receipt via
  `maybe_issue_receipt`. Virtual documents must never mutate the workspace;
  `score_summary` recomputes the same score formula from `surface_rows` +
  `classify_severity` with no side effect. This also removes a test-order
  race (a parallel inventory render minting a receipt flips standing
  staleness).
- **Staleness law** (require-gate-receipt): receipt fresh iff its age
  ≤ newest config source file mtime age; missing either side → "unknown";
  a workspace with no receipts renders a loud UNVERIFIED notice.
- **Standing vocabulary**: surface ALIVE iff diagnostics clean AND a fresh
  receipt exists; PARTIAL if findings OR clean-but-no-fresh-receipt;
  UNVERIFIED if no files of the surface discovered. `agent` scope
  classification mirrors backend `AgentScope` semantics.
- Test premises checked against real substrate: the fixture's findings live
  on `settings.json` (2), `settings.local.json` (1), `agent` (1); skill is
  clean — tests assert the real partition, not the pre-probe model.

## Standing

ALIVE — all three documents render from the real fixture corpus with
stable sections; commands + exits cited above.
