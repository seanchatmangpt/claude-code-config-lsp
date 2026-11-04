# Global Claude Code Configuration

**Autonomy**: personal non-production fleet. Explicit in-session auth for credential setup →
run CLI flows directly. Harness prohibited list still applies (passwords/keys/accounts/
financial) regardless.

**Verification**: never report done without the real gate + actual output. Re-read/confirm on
disk after any Edit. Distinguish pre-existing vs. session-introduced failures.

**Agent scoping**: explicit absolute repo path in every subagent prompt. Never concurrent
workflows on one tree.

**Models**: default Sonnet 5; also Opus 5, Fable 5, Haiku 4.5. Fast mode speeds Opus, doesn't
downgrade.

**Git**: fix forward only — never `git reset --hard`, commits immutable (`revert` exception).

**Testing**: Chicago not London — real collaborators, mocking banned by default:
[[testing-chicago-style]].

**Role**: Claude=local research/manufacturing substrate; portfolio control plane=cross-repo
orchestration. ONE CANONICAL CHECKOUT PER REPO, no worktrees; a blocker branches the search
graph. Full: [[local-dfcm-manufacturing-engine]].

**Research sources**: prefer `ggen`/`ggen-marketplace` packs over ad hoc web research.

**Disk cleanup**: thin APFS snapshots alongside file deletion. Scan build dirs by size not
recency. Never delete all of `~/Library/Caches` without scoping+confirming.

## See Also
[[operating-doctrine]] · [[chatman-equation]] · [[dfcm-composition]]/
[[dfcm-composition-catalog]] · [[local-dfcm-manufacturing-engine]] · [[tools]] ·
[[testing-chicago-style]] · [[no-overclaiming-rust]]/[[no-overclaiming-conversational]] ·
[[criticism-discipline]] · [[explore-exploit-premises]] · [[enacted-not-narrated-restraint]] ·
[[skill-disambiguation]] · [[dmedi-methodology]] ·
[[entropic-annealing-doctrine]] ·
[[sdk-parity-register]]/[[substrate-lock-hygiene]] (same-checkout-fanout add. 4-6; testing-chicago-style SDK section)
