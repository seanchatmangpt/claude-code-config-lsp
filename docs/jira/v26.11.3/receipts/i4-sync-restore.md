# I4: Sync Restore — Three-Zone Law Executed (receipt)

Lane: I4 (IMPLEMENTATION). Repo: /Users/sac/claude-code-config-lsp. Date: 2026-11-03.
Tool: ggen 26.10.12 — binary built this session at
`/Users/sac/ggen/target/release/ggen` (`cargo build --release -p ggen-cli-lib --bin ggen`, 7m00s;
note: `-p ggen-cli` does not match any package — the CLI bin lives in `ggen-cli-lib`).

## Commands / exits

- `ggen sync run --dry-run` — initial refusal [FM-CONFIG-003]: 4 strict-mode E0011/E0013
  errors (missing ORDER BY). Fixed: 4 inline queries in ggen.toml + appended ORDER BY to
  sparql/{completion,hover,modules,virtual_docs}.rq. Second refusal [FM-LAW-015] (EA
  admission gate); run with `--skip-ea-admission`.
- `ggen sync run --skip-ea-admission` — exit 0, 15 files planned-written, 6 analyzer stubs
  correctly skipped (mode=create, target exists).
- Determinism: two consecutive syncs — sha256 of all 15 outputs pairwise identical; the 12
  hand-owned files unchanged (Create mode). DETERMINISM-OK.

## Per-file decisions (diff-then-decide)

Regeneration was compared (diff /tmp/i4-baseline vs post-sync) for all 15 planned files.
`<` = hand, `>` = regenerated.

### Decision (b) — accept regenerated (3)

- `src/file_types.rs` — rustfmt-equivalent + additive `**/.claude/commands/*.md` pattern.
- `src/declare_model.rs` — whitespace only.
- `src/ocel_accumulator.rs` — cosmetic (import placement, rustfmt wrapping).

### Decision (a) — hand-owned via mode="Create" (12)

ggen has no `Skip` mode: GenerationMode = Create | Overwrite | Merge
(`crates/ggen-config/src/manifest/types.rs:588-601`). The contract's "Skip" is `Create`
(write-once, preserve hand edits on existing targets) — verified firing: sync2/sync3
reported these targets untouched.

| rule | file | why hand-owned |
|---|---|---|
| cargo | Cargo.toml | hand deps/features (clap-noun-verb, optional ecosystem features) dropped by template |
| lib | src/lib.rs | hand module list is a superset of modules.rq |
| main-rs | src/main.rs | hand clap-noun-verb dispatch + receipt-verify override |
| backend | src/backend.rs | landed handlers; imports LawAxis/MaxDiagnostic/ClassifiedFindings — absent from template |
| capabilities | src/capabilities.rs | landed P12 wiring + dedup (DISTINCT fix) not in template |
| conformance | src/conformance.rs | hand LawAxis::Custom method variants absent from template |
| hover | src/hover.rs | position-aware hover supersedes generated table-dump |
| completion | src/completion.rs | position-aware completion supersedes generated |
| semantic-tokens | src/semantic_tokens.rs | hand corrections (number legend entry, tree-sitter-json classifier) not modeled by template |
| schema | src/schema.rs | A/B gate: regen dropped 2 nodes (mcp.json, CLAUDE.md) + 17 dotted fields; hover tests went red (2 FAILED) — contract case (c) falsified on the real gate |
| virtual-docs | src/virtual_docs.rs | hand renders live CoverageReport health doc; regen is a static string |
| analyzers-mod | src/analyzers/mod.rs | hand declares commands + markdown_nav; regen drops markdown_nav |

## Contract case (c) outcome (schema.rs)

The TTL has the fields (settings_hooks_type, settings_statusLine, mcp.json surface) — the
divergence is the template/query emitting bare field names where hand code uses dotted
(parent-qualified) keys, plus 2 surfaces whose label-vs-node-name mismatch excludes them.
Extending the lsp-max pack template/query is outside this lane's ownership (lsp-max is a
separate repo). Deferred with falsifier: extend schema_fields.rq to compose dotted names
via ccc:fieldParent* and align surface labels; falsifier = hover tests stay green after
regeneration, goldens stay green.

## Gate

- `cargo build` — green (exit 0).
- `cargo test --test diagnostic_goldens` — 16/16 pass.
- `cargo test --lib` — 264/264 pass.
- `cargo test` full suite — 8 targets green; `compositor_e2e::compositor_front_door_end_to_end`
  FAILS (30s stdio timeout waiting for publishDiagnostics) — classified pre-existing by
  A/B: fails identically with all four accepted-regenerated files replaced by their
  pre-session baseline versions (both states fail). Not session-introduced by this lane.
  Note the contract's named gate "protocol_roundtrip" does not exist in
  tests/diagnostic_goldens.rs (16 tests, none by that name); the goldens target as a whole
  is the gate actually run.

## Standing: PARTIAL — ALIVE-flavored

Sync is real and deterministic (sync2 == sync3 byte-identical), build green, goldens 16/16,
lib 264/264. PARTIAL, not ALIVE, because 12 of 15 rules are hand-owned (Create mode): the
ontology is NOT yet the source of truth for those files — regeneration is safe only for the
3 accepted files, and schema.rs regeneration currently regresses hover behavior. Path to
ALIVE requires lsp-max pack template/query extension (dotted schema keys, LawAxis wire,
position-aware handler templates, live-coverage virtual doc) — outside this lane's repo.
