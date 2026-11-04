# C4 Receipt: Markdown Navigation (documentSymbol / foldingRange / documentLink)

Lane: C4 (IMPLEMENTATION) — 2026-11-03
Repo: /Users/sac/claude-code-config-lsp (canonical checkout, no worktree)

## Subject (files owned + touched)

- `src/analyzers/markdown_nav.rs` — NEW. Pure line-wise markdown structuring:
  `parse_headings` (ATX `#`..`######`, fence-aware), `document_symbols`
  (outline-stack tree, same-level = siblings, deeper nests under enclosing
  shallower heading), `folding_ranges` (one `Region` fold per heading section,
  end = line before next same-or-higher heading or EOF), `document_links`
  (relative-path tokens with config extensions `.sh/.json/.md/.toml/.yml/
  .yaml/.py/.ts`, must contain `/` or start `./`, fence-skipping, resolved
  against the doc's directory, `file://` targets, minimal percent-encoding).
  8 inline unit tests.
- `src/analyzers/markdown_nav.rs` tests assert fence-skipping, sibling vs
  nested semantics, range/fold line math, link skip rules (absolute paths,
  bare filenames, fenced tokens are NOT linked).
- `src/analyzers/mod.rs` — one-line registration (`pub mod markdown_nav;`).
- `src/capabilities.rs` — re-enabled `document_link_provider`
  (resolve_provider: false), `document_symbol_provider` (Left(true)),
  `folding_range_provider` (Simple(true)) + 3 capability tests.
- `src/backend.rs` — appended 3 `LanguageServer` overrides only
  (`document_symbol`, `folding_range`, `document_link`), no restructuring:
  classify gate via `crate::scan::classify` returning None for non-markdown
  families (`skill` | `claude_md` | `agent` only), document content via
  `adapter().get_document`, doc dir via new `doc_dir_from_uri` helper. Two
  helpers appended before the tests module (`is_markdown_config_doc`,
  `doc_dir_from_uri`). No existing fn bodies modified.
- `tests/markdown_nav.rs` — NEW Chicago integration tests (real fixture tree
  on disk: `tests/fixtures/md_nav_proj/{CLAUDE.md,hooks/session-start.sh,
  docs/reference.md}`), real file IO, no mocks. 5 tests.
- `tests/fixtures/md_nav_proj/**` — NEW fixture tree.

## Commands / exits (ALIVE = ran this session)

- Baseline `cargo build` before edits: Finished dev profile, 0 errors (40.9s).
- `cargo build` (after all edits): exit 0, 0 errors.
- `cargo test --lib markdown_nav`: 8 passed, 0 failed.
- `cargo test --lib capabilities`: 8 passed, 0 failed.
- `cargo test --test markdown_nav`: 5 passed, 0 failed.
- `cargo test --test chicago_tdd`: 27 passed, 0 failed.
- Target-scoped `cargo test --tests` (chicago_tdd + inventory + markdown_nav):
  27/11/5, all pass.
- `cargo clippy --lib`: 3 warnings, NONE in files this lane touched
  (backend.rs:196 mutable-key HashMap = pre-existing; completion.rs:107 and
  analyzers/json.rs:404 = other lanes' in-flight edits).
- Full `cargo test`: BLOCKED by foreign in-flight files — bin test target
  fails to compile at `src/nouns/config.rs:313` and `src/analyzers/json.rs:404`
  (set_language misuse mid-edit), plus 2 failing `analyzers::json` span tests
  and tests/bcinr_parity.rs (another lane, mid-write). All sites are files
  this lane does NOT own; disclosed under the compile-freeze SLA, not fixed
  here. My lane's scoped gates are green (13 owned tests + 38 pre-existing
  chicago tests re-verified).

## Verification ladder

1. Pure-function unit tests (8, inline) — heading math, tree semantics,
   fold boundaries, link resolution/skip rules.
2. Chicago integration (5, on-disk fixture tree) — symbols/folds/links over
   the real fixture doc with real sibling files; link targets asserted to be
   real files on disk.
3. Capability advertisement — 3 new tests assert the server now advertises
   all three providers; `server_capabilities()` merge in
   `RulePackServer::server_capabilities` picks them up via `..` spread.
4. Handlers classify-gated: non-markdown URIs return None (unit-testable
   helper `is_markdown_config_doc` mirrors the exact classify families).
5. NOT yet run: wire-level dogfood (`node scripts/dogfood.mjs`) — left to
   integration; the three methods are reachable via the LanguageServer trait
   override + capability advertisement.
