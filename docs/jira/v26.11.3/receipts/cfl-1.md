# CFL-1 Implementation Lane Receipt (G1 + G2)

Wave: v26.11.3 CFL-1, rank 2, IMPLEMENTATION lane. One canonical checkout:
`/Users/sac/claude-code-config-lsp`. No git commands run (coordinator owns transitions).

## G1 — TTL reconciliation court rot (P0)

**Court-rot cause**: `every_deprecated_key_literal_appears_in_ttl` anchored the rule
slice with the literal `Rule::new("CCC-JSON-001"` — a single-line shape. json.rs:295-306
had been rustfmt'd to the multiline form
(`Rule::new(\n "CCC-JSON-001",`), so `find` returned None and the court panicked at
`tests/ttl_reconciliation.rs:163`. Formatting-sensitive anchor in a court that fences a
rustfmt'd generated file = the defect class.

**Fix (locator-only)**: new `fn rule_slice(src, code) -> &str` in
`tests/ttl_reconciliation.rs` — anchors on the code literal `"CCC-JSON-001"` then scans
BACKWARD (`rfind`) to the enclosing `Rule::new(`. Formatting-stable under both shapes.
The anchors table now carries `Option<&str>` explicit-comment anchors (CCC-JSON-013 keeps
its `// CCC-JSON-013:` comment anchor; the two Rule-declared codes route through
`rule_slice`).

**Non-vacuity**: two new unit tests pin the locator against BOTH shapes —
`rule_slice_finds_single_line_rule` (historical anchor) and
`rule_slice_finds_rustfmt_multiline_rule` (the shape that broke the court).

## G2 — commands analyzer dispatched through the public scan API

- `src/scan.rs::classify`: new branch `u.contains("/commands/") && u.ends_with(".md")`
  → `"commands"`, placed BEFORE the `claude_md` branch (`/commands/` wins by directory,
  even for a file literally named CLAUDE.md). Doc surfaces outside `commands/` unchanged.
- `src/scan.rs::analyze_document`: new `"commands"` arm calling
  `crate::analyzers::commands::validate_command_file(content, base_dir)` with
  `base_dir` = parent of `uri_path(uri_or_path)` (new helper strips `file://`), mapped
  via `.filter(|p| p.is_dir())` — Some only when the parent is a REAL directory on
  disk, `None` for virtual `claude-config://` URIs and synthetic paths, so
  CCC-CMD-003 cannot false-fire. Findings mapped with category `"commands"`.
- `src/analyzers/commands.rs` NOT touched (read-only import, per lane contract).

## Files owned + changed (all absolute)

- `/Users/sac/claude-code-config-lsp/tests/ttl_reconciliation.rs` — rule_slice + 2 unit
  tests + anchors table rework (G1)
- `/Users/sac/claude-code-config-lsp/src/scan.rs` — classify branch, uri_path helper,
  commands dispatch arm (G2)
- `/Users/sac/claude-code-config-lsp/tests/commands_dispatch.rs` — NEW, 12 tests
  (classify routes/precedence/non-md, dispatch of CCC-CMD-001/002/004/006, real vs
  nonexistent vs virtual base_dir CCC-CMD-003 behavior, file:// scheme strip, category
  asserts)
- `/Users/sac/claude-code-config-lsp/tests/diagnostic_goldens.rs` — 3 new goldens
  (ccc-cmd-001, ccc-cmd-002, clean) pinning code+category+message prefix+real span
- `/Users/sac/claude-code-config-lsp/tests/goldens/commands-ccc-cmd-001.md`,
  `commands-ccc-cmd-002.md`, `commands-clean.md` — NEW fixtures
- `/Users/sac/claude-code-config-lsp/tests/compositor_e2e.rs` — commands case: didOpen
  of `<workspace>/.claude/commands/deploy.md` lacking a description surface →
  CCC-CMD-001 relayed with compositor source attribution asserted
- `/Users/sac/claude-code-config-lsp/docs/jira/v26.11.3/receipts/cfl-1.md` — this receipt

## Gate commands + exits

See GATES section below — recorded at run time, appended after execution.
## GATES (executed 2026-11-03, exit codes + counts from real runs)

| gate | command | exit | result |
|---|---|---|---|
| G1 court | `cargo test --test ttl_reconciliation` | 0 | 13 passed; 0 failed (11 pre-existing incl. the formerly-red `every_deprecated_key_literal_appears_in_ttl` + 2 new locator unit tests) |
| G2 dispatch | `cargo test --test commands_dispatch` | 0 | 12 passed; 0 failed (NEW file) |
| goldens | `cargo test --test diagnostic_goldens` | 0 | 19 passed; 0 failed (16 pre-existing + 3 new CCC-CMD goldens) |
| e2e | `cargo test --test compositor_e2e` | 0 | 2 passed; 0 failed (commands case passed through the REAL lsp-max-compositor release binary over the wire, 26.7s) |
| full | `cargo test` | 0 | 591 passed, 0 failed across 33 suites |
| lint | `cargo clippy --workspace --all-targets` | 0 | 0 warnings, 0 errors (one `clippy::redundant_slicing` in my own test code found and fixed mid-lane) |

Environment disclosures:
- The first two compositor_e2e runs FAILED pre-lane on missing release binaries
  (`/Users/sac/lsp-max/target/release/lsp-max-compositor` and
  `/Users/sac/claude-code-config-lsp/target/release/claude-code-config-lsp`), not on
  test logic. Built
  both release binaries during the lane (`cargo build --release` in each repo, exit 0)
  and reran green. lsp-max's `builder.rs` was mid-fix by lane LM-1 during my first
  config-lsp release attempt (E0382 in lsp-max lib); by the time I rebuilt, the fix had
  landed and the build passed. I did not touch /Users/sac/lsp-max source.
- One truncated build attempt was self-inflicted (grep `head -20` SIGPIPE'd cargo) —
  rerun without truncation.

Non-vacuity notes:
- G1's locator unit tests assert against both the single-line and rustfmt-multiline
  `Rule::new` shapes; the court itself exercises the multiline shape against the REAL
  json.rs on disk (Chicago: no mocks, no fixtures for the court's subject).
- G2's dispatch tests prove CCC-CMD-003 base_dir discipline three ways: real dir →
  fires; synthetic nonexistent dir → suppressed; claude-config:// virtual URI →
  suppressed, while family codes still fire in both suppression cases.

## Status

**ALIVE** — all lane gates executed this session with exit 0; court-rot cause documented
(formatting-sensitive anchor); files changed limited to the lane contract.
**Standing: lane complete, ready for coordinator integration.**
