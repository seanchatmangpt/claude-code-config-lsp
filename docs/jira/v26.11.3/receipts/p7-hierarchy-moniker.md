# Receipt: P7 — Call hierarchy + monikers for config surfaces

Lane: IMPLEMENTATION (owns `src/hierarchy.rs`, `src/moniker.rs`,
`tests/hierarchy_moniker.rs`; no backend.rs / capabilities.rs edits — none made).

## Subject

- `src/moniker.rs` (new): `ConfigScope` (local re-implementation of
  `backend.rs::AgentScope` URI-prefix logic, cited as prior art, read-only —
  backend.rs untouched), `config_scope`, `settings_keys` (single-pass
  structural scanner over raw JSON text; objects contribute member names,
  arrays contribute live element indices — every scanned dotted path
  resolves in the real serde_json parse, cross-checked by test with exact
  string-set equality over a recursive walk of the fixture parse, 112 paths
  on the real `settings.json` fixture), `moniker_for_key`
  (`claude-config:global:settings.<dotted.path>` /
  `claude-config:project:<relative-path>:<key>`; plugin keys by cache path,
  like project), `moniker_at_position` (scope + key-at-position + moniker;
  UTF-16 columns, multibyte-safe).
- `src/hierarchy.rs` (new): corpus-index API (`Corpus = Vec<(PathBuf, String)>`,
  no fs walking inside the functions). `frontmatter_name` (quoted/unquoted
  `name:` values, value-token range), `prepare_call_hierarchy` (cursor on a
  SKILL.md / agent frontmatter name → `CallHierarchyItem`, kind FUNCTION,
  file:// URI), `incoming_calls` (one edge per referencing doc; channels:
  frontmatter `key: name` values, `[[wikilink]]` stems, settings.json
  `"plugin@marketplace"` keys; the definition itself excluded; exact segment
  match — "ggen" does not match "ggen-lsp@ggen"), `outgoing_calls`
  (wikilinks + path-like refs; slash-ful tokens resolve by exact path-suffix
  match, bare stems by stem/frontmatter name — fixes the every-SKILL.md
  collision class). Typed helpers `incoming_calls_typed` /
  `outgoing_calls_typed` return the LSP response structs.
- `src/lib.rs`: two `pub mod` lines added (hierarchy, moniker). Other
  concurrent lanes' module additions in lib.rs left intact.
- `tests/hierarchy_moniker.rs` (new): 13 Chicago tests over the real
  `tests/fixtures/claude-config/` tree (real file IO into the corpus; the
  three referencing channels verified against fixture content; serde walk
  oracle cross-check). No mocks.

## Commands / exits

- `cargo check --lib` — exit 0 (owned files compile clean, no warnings).
- `cargo test --test hierarchy_moniker` — exit 0,
  `13 passed; 0 failed` (final run).

## Verification ladder

1. Scanner-vs-serde exact-set equality over the real fixture (112 dotted
   paths, arrays as live indices) — catches path drift mechanically.
2. Scope detection prefix truth table (global / plugin-beats-global /
   project / no-HOME).
3. Moniker string scheme + position lookup on the real fixture
   (enabledPlugins.ggen-lsp@ggen) + negative (value string → None) +
   project-relative-path + multibyte UTF-16 columns.
4. Hierarchy: frontmatter names on real skill + agent fixtures; prepare
   on/off-cursor; incoming via wikilink + frontmatter channels; incoming via
   the real fixture's `enabledPlugins.ggen-lsp@ggen` key; exact-segment
   negative; outgoing path-suffix + wikilink resolution; self-reference
   exclusion.

## Standing

ALIVE for the pure/corpus layer (executed this session, receipts above).
Not wired into backend.rs request handlers (outside this lane's file
ownership) — the typed helpers are the seam for that integration.
Pre-existing, session-independent: `src/nouns/config.rs` and
`src/provider_registry.rs` compile breaks from concurrent lanes froze the
shared bin/lib twice mid-wave; both were other lanes' in-flight files and
resolved by their owners within the SLA. Zero defects found in owned files
at final run.
