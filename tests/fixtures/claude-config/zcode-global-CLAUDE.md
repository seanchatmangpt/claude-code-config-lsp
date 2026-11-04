# Global Claude Code Configuration

## Autonomy Boundaries
Personal, non-production fleet — read before applying default caution on
permission decisions. When the user has explicitly authorized credential or
secret setup in this session (e.g. "run the gh auth script", "wire up this
token"), run CLI-driven flows directly — `gh auth login` (device-code flow),
`gh secret set`, `doppler run`, piping a token from an env var or file — rather
than handing back manual steps. This does not relax the harness's own
prohibited-action list: never type a password, API key, or token into a web
form/field, never create accounts, never enter financial or payment
credentials — those stay refused regardless of in-chat authorization, per the
harness's own safety rules, which take precedence over this file. Permission
granted for a specific action in this session is not revoked by the same class
of action recurring later in the same session — don't re-ask for something
already approved — but a fresh session or a materially different action still
needs its own ask.

## Verification Discipline
- Never report work as done without running the real verification gate for the
  language in play (build + tests + fmt + lint — clippy/credo/eslint/ruff/mypy
  as applicable) and pasting the actual output, not a description of it.
- After any Edit, re-read the file (or otherwise confirm on disk) to verify the
  change actually landed before moving on — don't assume a tool call succeeded
  silently.
- Distinguish pre-existing failures from failures introduced this session and
  state which is which explicitly, every time a status is reported — not just
  once per session.

## Agent & Workflow Scoping
- Give every subagent and verification workflow an explicit absolute repo
  path — never rely on session cwd. State the target repo in the first line of
  every agent prompt; have the agent `cd` there and confirm `pwd` before it
  reports findings.
- Before launching a workflow or background agent against a repo, check
  whether another workflow is already running against that same repo
  (`/workflows`, `TaskOutput`) — don't fire concurrent workflows onto one
  working tree; queue or serialize instead.

## Models
Default: Sonnet 5 (`claude-sonnet-5`). Also available: Opus 5 (`claude-opus-5`), Fable 5
(`claude-fable-5`), Haiku 4.5 (`claude-haiku-4-5-20251001`). Fast mode (`/fast`) speeds up
output on Opus — it does not downgrade to a smaller model.

## Git workflow: fix forward only
- Never `git reset --hard` or other destructive git operations.
- Fix issues in place — debug, find root cause, apply targeted fixes.
- Commits are immutable — solve problems by adding commits, never removing them.
- Exception: `git revert` (creates a new commit) is allowed when necessary.

## Testing discipline: Chicago school, not London school
Default to real collaborators and state-based assertions (Chicago/classicist), never
interaction-based mocking of collaborators this codebase owns or that are realistically
runnable in-process/locally (London/mockist) — `unittest.mock`/`Mock`/`patch`/
`monkeypatch` and equivalents are banned by default, not just discouraged. Full
discipline, the one legitimate exception, and the verification requirement (a real
grep for mock usage + real passing test output, every time, not a memory of a prior
run): `~/.zcode/rules/testing-chicago-style.md`.

## Role: local DfCM research/manufacturing engine (vs. the portfolio system)
Claude ≈ local research + local manufacturing substrate (maximize the local
possibility/manufacturing graph); the user's separate portfolio-level control plane
handles cross-repo orchestration (maximize the portfolio graph's rate of expansion) —
different scales of cognition, not competing jobs. On a research/repo problem:
maximize the admissible search space before selection (parallel agents, real
implementations, competing hypotheses, non-dominated alternatives preserved); make
research manufacture real capital (code/tests/receipts), not just prose; ONE CANONICAL
CHECKOUT PER REPOSITORY, NORMAL GIT BRANCHES, NO WORKTREES (operator 2026-09-23: no
`git worktree`, no `~/wt/...` development, no shadow integration clones or parallel copies
of a repository) — alternatives live in the code and semantic graph (branches, explicit
alternatives, reversible history), and only the consequential merge step is serialized;
a blocker branches the search graph, it never halts the whole session; never use
blanket `-X theirs`/`-X ours` or other destructive conflict shortcuts; end substantial
runs with a structured manufacturing receipt, not a narrated summary. Full seven-law
constitution: `~/.zcode/rules/local-dfcm-manufacturing-engine.md`.

## Research Sources
Prefer `ggen`/`ggen-marketplace` packs and local ontologies as the source of
truth for code generation and templates over ad hoc web research — check
`~/ggen-marketplace/packs/` and the `ggen` skill before reaching for
`WebFetch`/`WebSearch` on a codegen or template question. Same instinct as
[[local-dfcm-manufacturing-engine]]'s "Where reusable structure belongs" —
search existing manufacturing capital first, write consumer-local code only
when nothing reusable already covers it.

## Disk Cleanup Rules
- When freeing disk space, always thin APFS local Time Machine snapshots
  (`tmutil thinlocalsnapshots / <bytes> 4`) in addition to deleting files —
  file deletion alone often shows no visible free-space gain until local
  snapshots are thinned.
- Don't let file-recency guards hide large reclaimable build-artifact
  directories (`target/`, `_build/`, `node_modules/`, `.venv/`) — a directory
  being recently touched doesn't mean its contents aren't disposable; scan by
  size, not just age.
- Never propose deleting all of `~/Library/Caches` in one shot — scope to
  specific subdirectories and confirm the specific list with the user before
  deleting, per the harness's standing destructive-action confirmation rule.

## See Also
- `~/.zcode/rules/operating-doctrine.md` — Sean's operating doctrine SPR (役割/核/HDDL SOLVE/DfCM/知能則/BRCE/
  Ontology-first/最大公理, FOND-HDDL before prose, token discipline); global for every session
- `~/.zcode/rules/chatman-equation.md` — A=μ(O*), R=receipt(A) as types; failure taxonomy by broken term;
  recursion O*_{t+1}=admit(observe ∪ R_t) = autonomy; receipts validate via `~/.zcode/dfcm/validate_receipt.py`
- `~/.zcode/rules/dfcm-composition.md` — DfCM composition method + laws (stability, edge standing, derived
  standing, conservation, zero-information checks, O(N²) drift, generate-and-kill, retirement, self-hosting)
- `~/.zcode/rules/dfcm-composition-catalog.md` — GENERATED fleet primitive/composition catalog; source
  `~/.zcode/dfcm/composition-space.ttl`, projector `python3 ~/.zcode/dfcm/project.py [--check]`
- `~/.zcode/rules/local-dfcm-manufacturing-engine.md` — the seven-law local
  research/manufacturing-engine constitution summarized above
- `~/.zcode/rules/tools.md` — LSP-first navigation for Rust/TypeScript/Python (Java
  currently inactive — jdtls-lsp disabled), markdown standards
- `~/.zcode/rules/testing-chicago-style.md` — Chicago vs. London testing discipline, no mocking by default
- `~/.zcode/rules/no-overclaiming-rust.md` — status vocabulary floor (ALIVE/PARTIAL/BLOCKED/MOCKED/REFUSED/UNSUPPORTED/UNVERIFIED), scoped to `**/*.rs`
- `~/.zcode/rules/no-overclaiming-conversational.md` — grounding claims in a run, not a survey; precision in prose, not just code
- `~/.zcode/rules/criticism-discipline.md` — anti-dismissal rules for critiquing others' claims
- `~/.zcode/rules/explore-exploit-premises.md` — when to reason past current evidence (explicit thought exercises) vs. ground it
- `~/.zcode/rules/enacted-not-narrated-restraint.md` — don't narrate restraint, just exercise it
- `~/.zcode/rules/skill-disambiguation.md` — quick-reference for overlapping skill triggers (code-review/simplify/security-review, skill-creation family, loop/schedule)
- `~/.zcode/rules/dmedi-methodology.md` — DMEDI (Define/Measure/Explore/Develop/Implement) as the standing work framework
- `~/.claude/loops-overview.md` — index of active standing loops (cron jobs, trackers, scope)
- `~/CLAUDE.md` — personal/project index
- `~/ggen-marketplace/packs/` — reusable generation capital referenced by Research Sources
