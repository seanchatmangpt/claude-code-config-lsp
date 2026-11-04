# claude-code-config-lsp v26.11.3 wave-closure seal receipt

Date: 2026-11-04. Wave-closing seal for claude-code-config-lsp's
contribution to the five-repo v26.11.3 closure wave (mirror of
`/Users/sac/ggen/docs/jira/v26.11.3/closure/v26.11.3-closure.md`,
condensed — config-lsp is a small contributor to the wave). Zero src/test
edits in this receipt's own actuation; all code landed via receipted
lanes cited below.

## Exact subject

- Repo: /Users/sac/claude-code-config-lsp, branch main, HEAD 6a04124
  (pushed).
- Receipts dir: docs/jira/v26.11.3/receipts/ — **70 files** on disk
  (`ls | wc -l` at seal time), one row each in the zd-25 index
  (README.md, receipts/zd-25-index.md).

## Deliverables (each cites its on-disk receipt)

- **XD-16 standing_layer workspace-root fix** — the production standing
  layer resolved the receipt root from the navigation corpus roots
  (`~/.claude/rules`) instead of the workspace, so standing could never
  be ALIVE in any governed workspace and every publish batch carried
  standing-unverified noise; workspace-root resolver + governance gate
  landed; relay stress 5/0 ×2, AF-23 courts 15/15 unchanged, lib 264/0:
  receipts/xd-16-relay-standing.md (commit 8340bf6).
- **AF-23 standing publish wiring** — standing vocabulary wired into the
  real did_open/did_change publish batch (AF-2/AB-18 disclosed gap):
  receipts/af-23-h4-wiring.md.
- **AB-18 relay stress + court un-ignored** — compositor front-door
  stress suite (tests/compositor_relay_stress.rs) + the AF-19 close-
  condition court `clh_end_position_survives_front_door` un-ignored and
  live: receipts/ab-18-relay-stress.md (commit 3efe202).
- **ZD-25 receipts-dir index** — one row per file in the receipts dir,
  70 rows = 70 files verified equal: receipts/zd-25-index.md
  (commit 6a04124, HEAD).

## Verification ladder (this session, real runs)

- `cargo test --test standing` → **15 passed; 0 failed** (AF-23 courts),
  exit 0.
- Post-XD-16 standing (from receipt body, 2026-11-04): relay stress 5/0
  ×2, compositor_e2e green, lib 264/0, clippy no NEW warnings (one
  pre-existing warning in AB-18's owned file).

## Standing

ALIVE for the contracted scope: HEAD 6a04124 pushed, every deliverable
above resolves to a receipt file and commit on disk, verification exit 0
with the cited count, and the 70-row index equals `ls | wc -l`.

Falsifier: any cited receipt missing or disagreeing with its body, the
standing court failing to reproduce, or the index row count disagreeing
with a fresh `ls | wc -l` of the receipts dir.
