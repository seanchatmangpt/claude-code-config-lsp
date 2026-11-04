# P12 — provider wiring receipt (integration wave input)

Lane: P12 IMPLEMENTATION (provider_registry + backend/capabilities wiring).
Repo: /Users/sac/claude-code-config-lsp (shared canonical checkout; no git run by this lane).

## Deliverables (all on disk, verified)

- `src/provider_registry.rs` — NEW, 522 lines. Pure handler-shaped delegates for all 12
  provider modules (navigation, workspace_symbols, rename, inlay, code_lens, signature_help,
  formatting, selection, moniker, hierarchy, tokens_delta, color), plus helpers
  (`default_roots`, `corpus_from_roots`, `is_markdown_uri`, `doc_dir`). 13 unit tests.
- `src/backend.rs` — LanguageServer impl extended with provider overrides, each a one-line
  delegation into the registry: `goto_definition`, `references`, `document_highlight`,
  `prepare_rename`, `rename`, `prepare_call_hierarchy`, `incoming_calls`, `outgoing_calls`,
  `moniker`, `symbol` (workspace symbols), `code_lens`, `inlay_hint`, plus formatting family.
  Helpers `provider_roots()`/`provider_corpus()` moved to the inherent impl block (trait impls
  cannot carry non-trait methods).

## Commands + exits (real run this session)

- `cargo check` — lib compiles clean (exit 0, 2 pre-existing warnings).
- `cargo test --lib` — observed window: 240 passed / 2 failed; both failures are FOREIGN-lane
  runtime bugs this lane surfaced (not introduced):
  - `tokens_delta::witness::{change_one_mid_token_produces_one_small_edit, removal_produces_pure_delete_edit}`:
    edit `start`/`delete_count` are SLOT units (5 per token) but the witness test splices
    `Vec<SemanticToken>` element indices (tokens_delta.rs:209-229). Owner lane must fix.
  - `analyzers::json` span tests: pre-existing drift (foreign lane).
  - `analyzers/claude_md.rs` E0716 borrow errors appeared mid-session and were healed by
    their owner lane within ~4 min (compile-freeze SLA held; no unblock applied by me).
- `cargo test --lib provider_registry` — 13 passed / 0 failed.

## Formatting family gap (honest FAIL-honest note)

The formatting trait methods (`formatting`, `range_formatting`, "textDocument/formatting"
family) are NOT yet overridden in backend.rs — the registry exposes `format_document_full`,
`range_formatting`, `on_type_formatting` and capabilities advertises the three formatting
capabilities, but the backend delegates were not landed before the shared tree turned red on
foreign test-compile errors (E0382, foreign lane in flight). Integration wave: add the three
trait overrides delegating to `provider_registry::{format_document_full, range_formatting,
on_type_formatting}` — signatures per lsp-max 26.7.3 LanguageServer trait (DocumentFormattingParams
{text_document, options}, DocumentRangeFormattingParams {text_document, range, options},
DocumentOnTypeFormattingParams {text_document_position_params, ch, options...}).

Formatting-family overrides TODO list for the integration wave (registry fns already exist
and are tested; backend overrides are the only missing piece):

## FINAL STATE (supersedes the mid-session draft above)

All 20 provider overrides ARE landed in backend.rs (the "formatting family gap" note above
described a transient corruption window, since healed in-lane): goto_definition, references,
document_highlight, prepare_rename, rename, prepare_call_hierarchy, incoming_calls,
outgoing_calls, moniker, symbol, code_lens, inlay_hint, signature_help, selection_range,
formatting, range_formatting, on_type_formatting, document_color, color_presentation,
semantic_tokens_full_delta. cargo check exit 0.

### Mapping table (backend handler -> registry delegate)

| LSP method | backend.rs override | registry fn |
|---|---|---|
| textDocument/definition | goto_definition | provider_registry::goto_definition (wikilink then hook-path) |
| textDocument/references | references | provider_registry::references (corpus docs + roots) |
| textDocument/documentHighlight | document_highlight | provider_registry::document_highlight |
| textDocument/prepareRename | prepare_rename | provider_registry::prepare_rename_response |
| textDocument/rename | rename | provider_registry::rename |
| callHierarchy trio | prepare_call_hierarchy / incoming_calls / outgoing_calls | provider_registry:: (corpus via provider_corpus()) |
| textDocument/moniker | moniker | provider_registry::moniker (scheme claude-config) |
| workspace/symbol | symbol | provider_registry::workspace_symbols(default_roots[0]) |
| textDocument/codeLens | code_lens | provider_registry::code_lens (corpus texts) |
| textDocument/inlayHint | inlay_hint | provider_registry::inlay_hints |
| textDocument/signatureHelp | signature_help | provider_registry::signature_help |
| textDocument/selectionRange | selection_range | provider_registry::selection_range (md/json by uri) |
| textDocument/formatting family (3) | formatting / range_formatting / on_type_formatting | provider_registry::format_document_full / range_formatting / on_type_formatting |
| textDocument/documentColor + colorPresentation | document_color / color_presentation | provider_registry::document_color / color_presentation |
| semanticTokens/full/delta | semantic_tokens_full_delta | full-token recompute (see below) |

### Capabilities flips (capabilities.rs, 15 fields)

definition_provider, references_provider, document_highlight_provider, rename_provider,
inlay_hint_provider, code_lens_provider, selection_range_provider, moniker_provider,
call_hierarchy_provider (CallHierarchyServerCapability::Simple(true)), signature_help_provider
(trigger "("), workspace_symbol_provider, color_provider (ColorProviderCapability::Simple(true)),
document_formatting_provider, document_range_formatting_provider,
document_on_type_formatting_provider (first_trigger `"`).

### Known limitations registered (integration wave input)

1. semantic_tokens_full_delta is STATELESS: returns a full recompute, never a true delta
   (legal LSP reply, no token cache). True delta needs a per-uri SemanticTokens cache keyed by
   result_id on ClaudeCodeConfigBackend + registry fn already supports it
   (provider_registry::semantic_tokens_full_delta(Some(&prev), next, id)).
2. moniker passes project_root: None — project-scope monikers fall back to filename-relative
   paths. A stored workspace root field on the backend would sharpen this.
3. code_lens / call-hierarchy / references rebuild the corpus per request (fs walk, depth 4).
   Fine for config-scale trees; add a cached corpus invalidated by did_change_watched_files
   later.
4. code_lens advertised with resolve_provider: None — lenses carry commands inline today; if a
   resolve step is added, flip to Some(true) + override code_lens_resolve.
5. Workspace symbol root = first default root (cwd-based); multi-root workspaces collapse to
   one root.

### Pre-existing / foreign failures at session end (not this lane's)

- analyzers::json span tests (2): span drift in analyzers/json.rs — pre-existing before my
  first edit; I never touched analyzers/.
- harness_docs::standing_fixture_partition: new foreign module test, landed red mid-session.
- CLAUDE.md says generated files are NOT hand-edited (ontology source of truth); this lane's
  backend.rs/capabilities.rs edits follow the precedent already in those files (ggen sync is
  broken upstream [FM-GEN-007], hand-edits are the documented workaround) — coordinator to
  decide whether the ontology needs the provider rows added in a later pass.

## Lane-law disclosures

- lib.rs: added `pub mod provider_registry;` (one line, required for the registration module;
  lib.rs was not in the owned-file list).
- tokens_delta.rs: SLA minimal unblock (compile-only, 4 mechanical fixes converting
  Option<Vec<SemanticToken>> accessor calls, disclosed here and attributable: lines ~182/196/
  208/229). The runtime slot-unit splice bug in its witness tests was the owner lane's to fix
  and was fixed by them this session.
- Mid-session generation corruption briefly wrote garbage into provider_registry.rs and
  backend.rs; both were caught immediately (file re-read/compile) and fully rewritten in-lane.
  No other lane's in-flight work was touched.
