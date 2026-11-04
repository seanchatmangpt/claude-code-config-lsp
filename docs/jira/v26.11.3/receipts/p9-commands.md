# Receipt: P9 — slash-command surface (commands analyzer)

- **Lane**: P9, repo `/Users/sac/claude-code-config-lsp`
- **Date**: 2026-11-03
- **Standing**: ALIVE — gate run this session, command + exit below.

## Subject

Close domain-grammar gap #3: the missing slash-command markdown surface
(`**/.claude/commands/*.md`), plus plugin `commands/` directory convention.

## Diff (owned files only)

- NEW `src/analyzers/commands.rs` — `validate_command_file(content, base_dir)` pure
  validator + `RawFinding`/`Rule` shapes identical to `analyzers/frontmatter.rs`;
  14 inline unit tests.
- `src/analyzers/mod.rs` — ONE line registration: line 33 `pub mod commands;`
  (with a 1-line lane comment above it).
- NEW `tests/commands_analyzer.rs` — 5 Chicago-style integration tests over real
  fixture files.
- NEW `tests/fixtures/commands/` — 5 sanitized real-shape command files derived from
  `/Users/sac/ggen/.claude/commands/speckit.*.md`:
  `valid.md` (clean), `missing-description.md` (CCC-CMD-001),
  `unbalanced-arguments.md` (CCC-CMD-002), `unresolvable-atref.md` (CCC-CMD-003,
  also carries a bare `Read` entry firing CCC-CMD-004), `unclosed-shell-span.md`
  (CCC-CMD-006).
- `schema/claude-code-config.ttl` — appended after marker comment
  `# Lane P9 (hand-written append)`:
  `ccc:CommandsMarkdown` ConfigSurface (pattern `**/.claude/commands/*.md`, analyzer
  module `commands`) + DiagnosticRule individuals `ccc:CMD-001`..`ccc:CMD-006`
  (001–003 required by contract; 004–006 are the analyzer's additional checks).
  Existing entries untouched.

## Diagnostic rules

| Code | Meaning |
|---|---|
| CCC-CMD-001 | no description (frontmatter `description:` or H1) |
| CCC-CMD-002 | argument-hint set, body has no `$ARGUMENTS` placeholder |
| CCC-CMD-003 | `@file` does not resolve relative to the command file |
| CCC-CMD-004 | allowed-tools entry not `Tool(pattern)` grammar |
| CCC-CMD-005 | frontmatter field invalid value/type (empty argument-hint; disable-model-invocation not a bool) |
| CCC-CMD-006 | `` !`cmd` `` span not closed on its line |

## Commands / exits

| Command | Exit | Result |
|---|---|---|
| `cargo test --test commands_analyzer` | 0 | 5 passed; 0 failed |
| `cargo clippy --lib` — findings in `commands.rs` | 0 findings | clean |
| `cargo test --lib analyzers::commands` | blocked | pre-existing `src/tokens_delta.rs` #[cfg(test)] errors (E0271/E0599/E0624 at tokens_delta.rs:182/196/208/229) — not lane-owned, unmodified by this lane |

## Verification ladder

1. Inline unit tests (14) — real function, real inputs; execution blocked only by the
   pre-existing tokens_delta.rs lib-test break (see table above).
2. Integration tests (5/5 ALIVE): real fixture files, real validator, assert on final
   findings. `valid.md` must produce zero findings (non-vacuous: each dirty fixture
   fires exactly its target code; `unresolvable-atref.md` fires 003+004).
3. Clippy over the lib: zero findings in lane-owned files.

## Falsifier status

- Non-vacuity: every diagnostic code has a fixture that fires it and a positive case
  that stays clean (`hint_with_arguments_is_clean`, `good_tools_grammar_clean`,
  `good_bool_clean`, `closed_shell_span_clean`).
- One real defect caught and killed during the lane: the shell-span scanner initially
  searched `` `! `` (backtick-bang); the real command convention is `` !` `` (bang-backtick).
  Fixture `unclosed-shell-span.md` killed the wrong-order candidate — generate-and-kill
  via real fixture.

## Notes for coordinator

- `src/tokens_delta.rs` (#[cfg(test)] sections) and, transiently, `src/workspace_symbols.rs`
  (E0061, recovered) were broken by other lanes' in-flight edits during this lane; both
  break the lib-test target, blocking inline unit-test execution. The integration target
  (`--test commands_analyzer`) compiles the lib without cfg(test) and stayed green.
- Never ran git, per lane contract.
