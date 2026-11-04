# K4 — CCC-MD golden-gap closure receipt

Lane: K4 (implementation — MD rules). Repo: /Users/sac/claude-code-config-lsp.
Date: 2026-11-03. Owned files: `src/analyzers/claude_md.rs`, `tests/diagnostic_goldens.rs`, `tests/goldens/`. One disclosed minimal dispatch wiring in `src/scan.rs` (shared seam, unassigned). No git run.

## Commands / exits

- `cargo test --test diagnostic_goldens` → exit 0, **16 passed / 0 failed** (final run; includes 4 rewritten MD goldens + clean-fixture negatives).
- `cargo test --lib -- claude_md` → exit 0, **37 passed / 0 failed** (all claude_md unit tests incl. 10 new MD-rule tests).
- `cargo test --lib` (full) → 250 passed / 3 failed in the final observed window; the 3 failures are `analyzers::json::tests::span_ccc_json_007_points_at_root_object`, `analyzers::json::tests::ts_key_span_walks_nested_paths_and_arrays`, `harness_docs::tests::standing_fixture_partition` — **C2-owned / other-lane surfaces, pre-existing in-window, not mine** (I touched no json.rs or harness code).
- Other-lane compile churn windows observed (capabilities.rs E0308, then E0382/E0432/E0433, bin-target `render_tests` duplicate, missing `overlay` module); all resolved by bounded 60–120s polling; every gate run above landed in a clean compile window.

## Implemented rules (src/analyzers/claude_md.rs)

Distinct behavior per ontology individual (`schema/claude-code-config.ttl`), layered on C11's gap analysis (analog codes untouched):

| Code | Ontology message | Distinct behavior implemented |
|---|---|---|
| CCC-MD-001 | "Missing required name field in front-matter" | Fires when the `name` key is entirely ABSENT from agent-md frontmatter (the frontmatter-family analog CCC-AGENT-001 covers the empty-valued case); span = frontmatter block between the `---` markers, real. |
| CCC-MD-002 | "Missing required description field in front-matter" | Field-absence analog of the above; span = frontmatter block, real. |
| CCC-MD-003 | "Unknown tool in tools array" | Per-tool findings with TOKEN spans (the analog CCC-AGENT-006 spans the whole line); inline `[..]` and indented block lists both parsed; `mcp__` escape hatch honored. |
| CCC-MD-005 | "user-invocable field must be boolean" | Emitted alongside CCC-SKILL-006 on the SkillMd surface with a VALUE-TOKEN span (analog spans the whole line). |

New: `pub fn validate_agent_md(content) -> Vec<RawFinding>` (+ `is_known_tool` mirroring frontmatter.rs VALID_TOOLS). CCC-MD-005 added inside `validate_skill_frontmatter`.

## Dispatch wiring (disclosed minimal unblock, src/scan.rs)

`analyze_document`'s `"agent"` branch now also runs `claude_md::validate_agent_md` with category `"md"` — without it the MD codes were unreachable through the public scan API and the goldens could not go non-gap. The skill branch already ran `validate_skill_frontmatter`, so CCC-MD-005 needed no wiring. C2's json.rs untouched; CCC-JSON-003 remains a recorded gap, C2-owned.

## Goldens (tests/diagnostic_goldens.rs + tests/goldens/)

- Fixtures renamed gap→violation: `agent-md-ccc-md-001/002/003.md`, `skill-md-ccc-md-005.md`; new clean negatives `agent-md-clean.md`, `skill-md-clean.md` (assert zero CCC-MD-* findings).
- Each rewritten golden asserts: exact code count, category, ontology message prefix, real byte span computed from fixture content, and analog-code non-vacuity (CCC-AGENT-001/005/006 and CCC-SKILL-006 still fire).
- Zero GOLDEN-GAP markers remain for MD rules; CCC-JSON-003 golden-gap test unchanged (C2).

## Verification ladder

1. 10 new unit tests in claude_md.rs (Chicago: real strings, final-state assertions, no mocks) — 37/37 green.
2. 4 rewritten goldens + 2 clean negatives through public `scan::analyze_document` — 16/16 green.
3. Full lib suite observed: only C2/other-lane failures remain; none in claude_md.rs, scan.rs, or the goldens.
4. Zero MD GOLDEN-GAP markers (grep over tests/ confirms).

## Standing

ALIVE for this lane's surface (claude_md.rs MD rules + goldens + scan.rs agent-branch wiring), all runs this session.
