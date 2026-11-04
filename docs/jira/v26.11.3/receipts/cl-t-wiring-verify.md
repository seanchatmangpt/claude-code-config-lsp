# CL-T — post-I4 provider wiring verification (receipt)

Lane: CL-T IMPLEMENTATION (verification). Repo: /Users/sac/claude-code-config-lsp. Date: 2026-11-03.
No git run by this lane. Owned files: this receipt only (backend.rs/capabilities.rs untouched — no fixes needed).

## Verdict: ALIVE — wiring survived I4 intact; no reconciliation required

I4's sync (i4-sync-restore.md) classified `src/backend.rs` and `src/capabilities.rs` as
hand-owned via ggen mode="Create" (write-once, preserved hand edits on existing targets;
verified firing: sync2/sync3 reported these targets untouched). Diff-first check confirms:
all 20 delegating overrides are present in `src/backend.rs` (20 override fns at lines
649–967, 21 `provider_registry::` references), and all 15 advertised capability fields are
present in `src/capabilities.rs` (definition/references/document_highlight/rename/
inlay_hint/code_lens/selection_range/moniker/call_hierarchy (Simple(true))/
signature_help/workspace_symbol/color (Simple(true))/document_formatting/
document_range_formatting/document_on_type_formatting, `on_type` first_trigger `"`,
code_lens resolve_provider: None per P12 limitation 4). No clobber occurred; contract step
(3) (restore delegations) was not needed and not executed.

## Commands + exits (real run this session)

- `cargo build` — exit 0 (49.17s, Finished dev profile).
- `cargo test --lib` — 264 passed / 0 failed.
- `cargo test --test protocol_roundtrip` — 3 passed / 0 failed (incl.
  `fixture_corpus_roundtrip_publishes_well_formed_diagnostics`).
- `node scripts/dogfood.mjs --allow CCC-JSON-001` — exit 0 over the real stdio
  Content-Length-framed wire against 12 fixtures; 11/12 clean; the only diagnostic
  observed is `CCC-XFILE-001` on `.claude/settings.json` (`hooks/session-start.sh`:
  referenced hook script with shebang not found in workspace index) — a real workspace
  finding, not a wiring failure. No CCC-JSON-001 occurrences, so the allow flag had
  nothing to suppress.

## P12 registered limitations — post-I4 re-verification (all five confirmed standing)

1. **tokens-delta stateless** — CONFIRMED. `semantic_tokens_full_delta` (backend.rs:959)
   recomputes the full token set via `semantic_tokens::build_tokens` and returns
   `SemanticTokensFullDeltaResult::Tokens` with `result_id: None`; in-code comment cites
   p12-wiring.md and names the true-delta design (`Mutex<HashMap<Uri, SemanticTokens>>`
   keyed by result_id). Close requires state on the backend struct — not a trivial close;
   contract marks this note-only. STANDING.
2. **moniker project_root: None** — CONFIRMED. backend.rs:805 passes literal `None` as
   project_root; project-scope monikers fall back to filename-relative paths. Fix needs a
   stored workspace-root field on ClaudeCodeConfigBackend (initialize params capture +
   field + did_change_workspace_folders handling) — not trivial. STANDING.
3. **per-request corpus rebuilds** — CONFIRMED. Five `self.provider_corpus()` call sites
   (backend.rs:680, 751, 766, 782, 839: references, prepare/incoming/outgoing call
   hierarchy, code_lens) each rebuild via fs walk on every request. Cache would need
   invalidation wiring against did_change_watched_files — not a mechanical close.
   STANDING.
4. **code_lens resolve_provider: None** — CONFIRMED (capabilities.rs:59-61). Lenses carry
   commands inline; code_lens_resolve has no override. STANDING (by design).
5. **workspace symbol root = first default root** — CONFIRMED (backend.rs:825-828:
   `roots.into_iter().next()` with cwd fallback). Multi-root workspaces collapse to one
   root. STANDING.

None of the five is a trivial close: each requires new backend state or cache-invalidation
machinery. All recorded as standing limitations; none closed this session.

## Lane-law disclosures

- Zero file edits outside this receipt. backend.rs/capabilities.rs were read and verified,
  not modified — the delegation wiring needed no repair after I4's Create-mode sync.
- Standing: ALIVE (all gates ran this session, real output cited above).
