---
name: changelog-miner
description: Mines the Claude Code changelog for new config fields, enums, and surfaces not yet covered by this project's ontology. Use periodically to find new validation gaps as Claude Code ships new releases.
user-invocable: true
default-enabled: true
effort: max
context: fork
agent: research-scout
argument-hint: "[since-version]"
---

# changelog-miner

Arguments: `$ARGUMENTS`

Scans the Claude Code changelog (optionally starting from `$ARGUMENTS` as a
version floor) for new `settings.json` fields, agent/skill frontmatter
fields, hook events, and plugin manifest fields, and reports each one not
already present in `schema/claude-code-config.ttl`.
