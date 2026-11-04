# ZD-107 — claude-code-config-lsp v26.11.3 wave-closure seal receipt

Date: 2026-11-04. Standing: ALIVE.

Repo: /Users/sac/claude-code-config-lsp @ main, HEAD 6a04124 (pushed).
Condensed wave-closing seal receipt; full body:
docs/jira/v26.11.3/closure/config-lsp-closure.md.

## Deliverables

XD-16 standing_layer workspace-root fix — receipt root was resolving to
the navigation corpus roots (~/.claude/rules), never the workspace; every
surface stood unverified. Workspace-root resolver + governance gate;
relay 5/0 x2, AF-23 15/15 unchanged, lib 264/0
(receipts/xd-16-relay-standing.md, commit 8340bf6) · AF-23 standing
publish wiring (receipts/af-23-h4-wiring.md) · AB-18 relay stress +
court un-ignored (receipts/ab-18-relay-stress.md, commit 3efe202) ·
ZD-25 receipts-dir index 70/70 (receipts/zd-25-index.md, commit 6a04124).

## Verification (this session, real runs)

- `cargo test --test standing` → 15 passed; 0 failed, exit 0 (AF-23
  courts).

## Standing

ALIVE for the contracted scope: HEAD pushed, all cited receipts resolve
on disk, the standing court exits 0 with 15/15, and the zd-25 index row
count (70) equals `ls | wc -l` of the receipts dir at seal time.

Falsifier: any cited receipt missing or disagreeing with its body, the
standing court failing to reproduce, or the index count disagreeing with
a fresh `ls | wc -l`.
