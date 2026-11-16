# claude-code-config-lsp migration plan v26.11.16

Standing: UNVERIFIED

## Identity
- Path: /Users/sac/claude-code-config-lsp
- Remote: https://github.com/seanchatmangpt/claude-code-config-lsp.git
- Branch: main@af67d0e (tracks origin/main, 0/0)
- Last commit: 2026-11-04; commits: 46; size: 1,732 MB (.git 1.7 GB)
- Languages/build: Rust (Cargo.toml, rust-toolchain.toml), ggen.toml, praxis/, sparql/
- Nested repos: none

## Observed state
- Dirty: 0; Untracked: 0
- Ahead/behind: 0/0
- Stash: 0
- Unpushed local branches: 7 — security/rotate-signing-keys-v26.9.24,
  worktree-wf_3252d092-1fb-{1..6} (Phase 2A-2E snapshots + auto-snapshot)
- Build dirs: dist/ 15M (committed artifact dir); no target/
- Submodules: none
- Odd: Cargo.toml.backup at top level; .git is 1.7 GB for 46 commits (large blobs likely).

## What it is
LSP 3.18 server for Claude Code config files (settings.json, CLAUDE.md, agents, skills,
hooks). Rust source is generated from schema/claude-code-config.ttl via ggen lsp-max pack.

## Classification
ACTIVE (census agrees).

## Migration risk
- Working tree clean and pushed.
- 7 unpushed branches: the security key-rotation branch and six worktree phase snapshots.
- No local data files found.
Rate: MEDIUM (branches only)

## Recommended action
CLONE-FRESH
1. `cd /Users/sac/claude-code-config-lsp && git branch -vv` — confirm the 7 local-only branches.
2. DECISION: push `security/rotate-signing-keys-v26.9.24` (`git push origin <br>`) — it may
   contain key material; inspect `git show --stat d35c8c9` first. Do not push secrets.
3. DECISION: worktree-wf_* branches look like phase snapshots already landed on main;
   push for history or drop.
4. Delete `Cargo.toml.backup` only after confirming it matches Cargo.toml (`diff`).
5. Clone fresh on the new machine; do not copy the 1.7 GB .git.

## First commands on the new machine
`git clone https://github.com/seanchatmangpt/claude-code-config-lsp.git && cargo check`
(stated, not run)

## See Also
- ../v26.11.3/
- /Users/sac/docs/jira/v26.11.16/README.md
