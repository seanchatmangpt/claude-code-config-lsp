# C11 — Diagnostic goldens receipt

Lane: C11 (implementation — goldens). Repo: /Users/sac/claude-code-config-lsp.
Date: 2026-11-03. Owner files only: `tests/diagnostic_goldens.rs` (new), `tests/goldens/` (16 fixtures). No `src/**` touched. No git run.

## Commands / exits

- `cargo test --test diagnostic_goldens` → exit 0, **16 passed / 0 failed** (final run; lib target churned compilable/broken throughout the session due to concurrent src/ lanes; every run used the bounded 60s build-poll protocol).
- `cargo test` (full suite) → exit 101, compile error in **src/nouns/config.rs:313** (E0308/E0277, bin test target) — other-lane-owned, pre-existing at my final run; recorded, not fixed (out of lane scope).
- Build contention episodes: 3 (E0425/E0599 completion/hover mid-edit, then E0765, then E0277). All resolved by bounded polling; final runs in clean compile windows.

## Deliverable 1: golden coverage of the 15 ontology DiagnosticRules

| Rule | Golden test | Pinned behavior |
|---|---|---|
| CCC-JSON-001 | ccc_json_001_deprecated_key_golden | 1 finding, msg prefix "Deprecated settings.json key — use mcpServers", span = key token bytes (real) |
| CCC-JSON-002 | ccc_json_002_wrong_hook_key_golden | 1 finding, prefix "Wrong hook key format — use hooks.PreToolUse / hooks.PostToolUse", span = key token (real) |
| CCC-JSON-003 | ccc_json_003_golden_gap | **GOLDEN-GAP**: code never emitted by any analyzer; the invalid-enum condition itself fires under CCC-JSON-004 (asserted) |
| CCC-JSON-005 | ccc_json_005_invalid_effort_level_golden | 1 finding, prefix "Invalid effortLevel 'extreme'", span (1,14) = quoted key token `"effortLevel"` (real) |
| CCC-JSON-007 | ccc_json_007_missing_schema_golden | 1 finding, prefix "plugin.json missing required '$schema' field", span (0,21) = doc body minus trailing newline (real) |
| CCC-JSON-009 | ccc_json_009_invalid_skill_overrides_golden | 1 finding, prefix "skillOverrides must be a record…", span (1,17) = quoted key token (real) |
| CCC-MD-001 | ccc_md_001_golden_gap | **GOLDEN-GAP**: never emitted; missing-name condition fires under CCC-AGENT-001 (asserted) |
| CCC-MD-002 | ccc_md_002_golden_gap | **GOLDEN-GAP**: never emitted; missing-description fires under CCC-AGENT-005 (asserted) |
| CCC-MD-003 | ccc_md_003_golden_gap | **GOLDEN-GAP**: never emitted; unknown-tool fires under CCC-AGENT-006 (asserted) |
| CCC-MD-005 | ccc_md_005_golden_gap | **GOLDEN-GAP**: never emitted; non-boolean user-invocable fires under CCC-SKILL-006 (asserted) |
| CCC-MD-006 | ccc_md_006_missing_h1_golden | 1 finding, prefix "CLAUDE.md should start with an H1 title", span (0, first-line len) (real) |
| CCC-TOML-001 | ccc_toml_001_semver_version_golden | 1 finding, prefix "SemVer version detected — this workspace uses CalVer (YY.M.D)", span (0,13) = literal `version = "1.` (real) |
| CCC-TOML-002 | ccc_toml_002_outdated_edition_golden | 1 finding, prefix "Outdated Rust edition — use edition = \"2021\"", span (0,16) = literal `edition = "2015"` (real) |
| CCC-HOOK-001 | ccc_hook_001_misspelled_event_golden + ccc_hook_001_invalid_event_in_settings_golden | Both surfaces pinned: hook-script — prefix "Misspelled hook event: use PreToolUse", span = literal token (real); settings — prefix "Invalid hook event 'on_tool_use'", span (11,24) (real) |
| CCC-HOOK-002 | ccc_hook_002_invalid_event_name_golden | 1 finding, prefix "Invalid hook event name — use PreToolUse/PostToolUse", span = literal token (real) |

Gap rules: 5 total (CCC-JSON-003, CCC-MD-001, CCC-MD-002, CCC-MD-003, CCC-MD-005) — asserted as zero-findings-for-code PLUS the analog code fires, so the goldens are information-bearing, not vacuous. Zero SPAN-PENDING markers remain in the final file (see below).

## Deliverable 2: CCC-XFILE-001 reachability

**NOT reachable via public API.** The cross-file rule is defined in src/backend.rs
(`build_cross_file_rules`) and evaluated by lsp-max's `WorkspaceRuleEvaluator` over
OPEN documents in the LSP workspace index. `scan::analyze_document` is pure
per-document — no workspace index, no open-document set. Per the contract's
"if reachable" clause: recorded unreachable, no golden written.

## Mid-flight span-drift note (c2 lane interaction)

This lane ran while the c2 span lane was landing real spans into the analyzers.
First-run observed spans replaced the anticipated (0,0) SPAN-PENDING pins: settings-enum
rules now span the full quoted key token (`"effortLevel"` bytes 1..14, `"skillOverrides"`
1..17, `"on_tool_use"` 11..24); plugin-json missing-$schema spans doc body minus trailing
newline (0,21). All SPAN-PENDING markers consumed; none remain. My own TOML byte-count
arithmetic was off by one (13/16, not 14/15) — corrected from observed output.

## Verification ladder

1. Per-rule fixture (16 files, tests/goldens/) → public API scan::analyze_document → exact code + message-prefix + span. Chicago-style: real fixture files, real dispatch, no mocks, final-state assertions.
2. Gap rules: zero-findings-for-code + analog-code-fires (non-vacuous).
3. 16/16 ALIVE this session (cargo test --test diagnostic_goldens, exit 0).
4. Full-suite compile break (src/nouns/config.rs:313) = other-lane, cited, not mine.

## Standing

ALIVE for this lane's surface (tests/diagnostic_goldens.rs + tests/goldens/).
