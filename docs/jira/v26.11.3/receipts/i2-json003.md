# I2 — CCC-JSON-003 implementation lane receipt

Lane: I2 (implementation — last golden-gap JSON rule). Repo: /Users/sac/claude-code-config-lsp.
Date: 2026-11-03. Owner files only: `src/analyzers/json.rs` (FREE — C2 landed), `tests/diagnostic_goldens.rs`, `tests/goldens/`. No git run.

## Deliverable

CCC-JSON-003 ("Invalid enum value for field", settings.json surface, per
`schema/claude-code-config.ttl` ccc:CCC_JSON_003) is now emitted by the analyzer.
Defined scope: the **generic enum catch-all** for enum-valued settings.json fields that
carry NO field-specific code — `tui` (fullscreen), `editorMode` (normal|vim),
`crossSessionInbound` (accept), `statusLine.type` (command), all four taken from the
generated schema's enum list (`src/schema.rs`). Field-specific codes (CCC-JSON-004/005/
006/008-012) are untouched and win for their fields — no double-fire (pinned by test
`field_specific_codes_still_win_over_generic`).

Implementation: table-driven loop over `GENERIC_ENUM_FIELDS` in
`validate_settings_json_enums` (src/analyzers/json.rs), real spans via C2's
`ts_key_span` tree-sitter helpers (span = full quoted key token; no (0,0) sentinels).

## Commands / exits

- `cargo test --test diagnostic_goldens` → exit 0, **16 passed / 0 failed** (baseline before
  edit: also 16/16, gap-shaped; after: same 16 tests with `ccc_json_003_golden_gap` replaced
  by `ccc_json_003_generic_invalid_enum_golden`).
- `cargo test --lib json` → exit 0, **73 passed / 0 failed** (C2's 54 json.rs tests + this
  lane's 10 new tests + json-adjacent lib tests matched by the filter).
- `cargo clippy --lib` → exit 0; 6 warnings, **none in src/analyzers/json.rs** (pre-existing
  in other modules; verified by grep of clippy output for json.rs — zero hits).

## Golden rewrite (per C11's non-vacuity discipline)

- `tests/goldens/settings-json-ccc-json-003.json` (NEW): `editorMode: "emacs"` +
  `statusLine.type: "script"` → exactly **2 CCC-JSON-003 findings**, message prefixes
  `Invalid enum value for field 'editorMode': 'emacs'` and `... 'statusLine.type': 'script'`,
  spans = the full quoted key tokens `"editorMode"` / `"type"` (real, asserted against
  fixture byte offsets).
- `tests/goldens/settings-json-clean.json` (NEW): fully valid settings (editorMode vim,
  statusLine command, model sonnet, hooks {}) → asserts **zero findings** through the
  public `scan::analyze_document` API.
- `tests/goldens/settings-json-ccc-json-003-gap.json` (REMOVED).

## Verification ladder

1. Unit tests (real serde_json + real tree-sitter parse, no mocks) fire/clean per field,
   no-double-fire, span-pins via the existing assert_span helper.
2. Golden through the PUBLIC API — the exact dispatch the LSP backend runs.
3. Clean negative — non-vacuity: the rule provably does not fire on valid input.
4. Clippy clean on the owned file.

## Standing

ALIVE for this lane's surface (src/analyzers/json.rs CCC-JSON-003 + the two new fixtures +
the rewritten golden). Gap inventory shrinks: 5 golden-gap rules → 4 (CCC-MD-001/002/003/005).
