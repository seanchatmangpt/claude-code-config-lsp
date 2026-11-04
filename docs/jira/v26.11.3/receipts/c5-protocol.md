# Receipt: C5 protocol round-trip suite (claude-code-config-lsp)

- **Lane**: IMPLEMENTATION (C5) — repo `/Users/sac/claude-code-config-lsp`
- **Owned files**: `tests/protocol_roundtrip.rs`, `tests/fixtures/claude-config/**`, `scripts/dogfood.mjs` (--fixtures flag)
- **Scope boundary**: `src/**` read-only for this lane; defects found go in this receipt, no edits
- **Date**: 2026-11-03

## Status: ALIVE — 3/3 protocol tests pass; dogfood run witnesses real wire diagnostics; 1 src defect filed (not edited)

## What landed

### 1. tests/protocol_roundtrip.rs (new, ~330 LOC)

Harness pattern borrowed from `lsp-max/examples/wasm4pm-lsp/tests/dogfood_gc007_lsp318.rs`:
`CARGO_BIN_EXE_claude-code-config-lsp` spawn of `serve --stdio`, Content-Length
framing, background reader thread + mpsc channel, stashed notifications drained
while waiting for a specific method.

Three tests:

1. **lifecycle_initialize_to_exit_roundtrip** — initialize → initialized →
   didOpen → assert well-formed `textDocument/publishDiagnostics` (uri echo +
   diagnostics array + per-diagnostic code/message) → shutdown → exit →
   close client stdin → bounded-poll for clean process exit (code 0) within 5s.
   Verified behavior discovered during development: the server stops its loop on
   the exit notification but the process only reaps after stdin EOF, so the test
   closes the client-side pipe before polling `try_wait`. Bounded poll on a real
   observable (process exit status), no sleeps-as-methodology (a 50ms poll step
   inside a deadline loop is a poll-step, not a coordination sleep).
   - Ran this session: 2 consecutive runs — FAILED both times while src/** was
     mid-edit by another lane, but the failure mode was NOT contention: first
     run was a real protocol finding (exit-notification alone does not reap the
     process; stdin EOF required), fixed in the test harness.
2. **diagnostic_roundtrip_invalid_model_enum_fires_ccc_json_004** — didOpen
   text with `"model": "gpt-4"` (same premise as src unit test
   `invalid_model_fires_ccc_json_004`) and require a `CCC-JSON-004` diagnostic
   with non-empty message and a real range object over the wire.
3. **fixture_corpus_roundtrip_publishes_well_formed_diagnostics** — walks
   `tests/fixtures/claude-config/` (≥10 files asserted), didOpen's each over the
   real wire, and requires every returned diagnostic to carry a `CCC-*` code,
   non-empty message; uri echo per fixture.

### 2. tests/fixtures/claude-config/ (new sanitized corpus, 12 files)

| fixture | source | redactions |
|---|---|---|
| settings.json | ~/.claude/settings.json | `ANTHROPIC_AUTH_TOKEN` → `<REDACTED-TOKEN>` |
| settings.local.json | ~/.claude/settings.local.json | none needed |
| claude-global-CLAUDE.md | ~/.claude/CLAUDE.md | none needed |
| zcode-global-CLAUDE.md | ~/.zcode/CLAUDE.md | none needed |
| agents/default-agent.md | ~/.zcode/agents/default-agent.md | none needed |
| skills/{config-audit,errc-cycle,ggen,mcp-doctor,run-ggen,source-of-truth-check}/SKILL.md | ~/.claude/skills/*/SKILL.md | none needed |
| invalid/settings.json | synthesized (intentional CCC-JSON-004 premise) | n/a |

Redaction note: the only credential found in any source was the
`ANTHROPIC_AUTH_TOKEN` in ~/.claude/settings.json; a post-redaction grep for
token/key material across the corpus found zero residual matches. Absolute
paths (/Users/sac/...) were retained deliberately — they are not credentials
and the analyzers scope-detect global vs project from path shape.

## src/** defects found (read-only lane — not edited)

1. **Classifier/scan surface mismatch (filed, not fixed — src is not mine)**:
   `src/file_types.rs:16` advertises glob `**/.claude/settings*.json` →
   `claude-config-json`, but `src/scan.rs:49` and `src/scan.rs:119` gate the
   settings enum validation on the URI ending exactly `settings.json` /
   `settings.local.json`. Consequence: a file like `settings.invalid-model.json`
   (matches the advertised glob) receives ZERO diagnostics over the wire even
   for violations that fire on `settings.json`. Witnessed this session: the
   invalid-model fixture at `settings.invalid-model.json` scanned clean; the
   byte-identical text at `.../invalid/settings.json` fired CCC-JSON-004.
   Suggested owner: whichever lane owns `src/scan.rs`. Fix direction: widen the
   suffix gate to match the advertised glob (or document the gate as normative
   and narrow the glob).
2. **Pre-publish race in scripts/dogfood.mjs snapshot (harness, pre-existing)**:
   the 2s `setTimeout` snapshot can run before publishDiagnostics arrive under
   load — one run this session printed all 21 fixtures "clean" and exit 0 while
   a byte-identical rerun printed 4 diagnostics and exit 1. Recommend raising
   the deadline to a bounded poll on "received a publish for every didOpen'ed
   uri" instead of a fixed 2000ms.

Also observed during contention window (not defects): transient compile errors
in `src/completion.rs`/`src/hover.rs`/`src/scan.rs` (`TreeCursor::children`,
`tree_placeholder`) — mid-edit churn from other lanes, resolved by their owners
within ~6 minutes.

## Commands / exits

| command | exit | note |
|---|---|---|
| `cargo build` | 0 | initial full build, before src contention began |
| `cargo test --test protocol_roundtrip` | 1 | run 1: lifecycle exit-reap finding (protocol-true) |
| `cargo test -- table::lifecycle...` | 1 | run 2 after stdin-EOF fix: compile abort (src churn) |
| `cargo build` retry @+60s | 1 | BLOCKED(build-contention): 2 src errors, other lanes' files |
| `cargo build` retry @+90s | 1 | 5 src errors, different set — active mid-edit churn |
| `cargo build` retry @+300s | 0 | contention resolved by owning lanes |
| `cargo test --test protocol_roundtrip` | 0 | **3 passed; 0 failed** (final run, post-contention) |
| `node scripts/dogfood.mjs --fixtures tests/fixtures/claude-config` | 1 | by design: 4 diagnostics witnessed — CCC-JSON-004 (invalid fixture), CCC-JSON-001 x2 (deprecated enabledMcpjsonServers/disabledMcpjsonServers on sanitized settings files), CCC-XFILE-001 (repo's own .claude/settings.json → hooks/session-start.sh missing shebang in index) |
| `node --check scripts/dogfood.mjs` | 0 | --fixtures extension parses |
| corpus secret grep | 0 matches | token/key material grep over fixture corpus |

## Sanitized-corpus findings (expected, not defects)

The two sanitized settings fixtures fire CCC-JSON-001 over the wire:
`enabledMcpjsonServers` (settings.json) and `disabledMcpjsonServers`
(settings.local.json) are deprecated keys per the server's law — real findings
against the live ~/.claude config shape, preserved deliberately so the corpus
exercises the deprecation rule.

## Dogfood harness timing caveat (pre-existing, one observed run)

One dogfood run printed all fixtures clean (exit 0) because the 2s snapshot
ran before publishes arrived; a byte-identical rerun showed 4 diagnostics and
exit 1. Flake, not deterministic. Filed above as harness defect #2.

## Replay

```sh
cd /Users/sac/claude-code-config-lsp
cargo test --test protocol_roundtrip
node scripts/dogfood.mjs --fixtures tests/fixtures/claude-config
```
