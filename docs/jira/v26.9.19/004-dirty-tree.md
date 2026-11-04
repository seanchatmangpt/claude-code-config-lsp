# claude-code-config-lsp: commit or clean uncommitted changes

- Standing: OPEN
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: working tree dirty at survey time
- Evidence: `git status --porcelain` → 37 path(s) (tracked-modified: 35, untracked: 2); sample:  D .agents/ORIGINAL_REQUEST.md; D .agents/explorer_1/BRIEFING.md; D .agents/explorer_1/ORIGINAL_REQUEST.md;?? .claude/receipts/36232e08e5c49bbafb0fc0a760e6d7fb34b54cb4a995056177ee9a036e2e4940.json;?? 

## Work to complete
- Review the 35 tracked-modified path(s); commit them as atomic pieces on a purpose branch, or revert what is transient.
- For the 2 untracked path(s): add intentional files to git and commit; gitignore or delete build artifacts/temp files.
- Note: this survey's ticket files under docs/jira/v26.9.19/ are intentionally uncommitted; include or exclude them deliberately in the commit plan.

## Acceptance
- `git status --porcelain` is clean (except items deliberately deferred and recorded here).

## History
- 2026-09-19 | OPEN | survey found dirty tree | 37 paths (T35/U2) | commit/clean pending
