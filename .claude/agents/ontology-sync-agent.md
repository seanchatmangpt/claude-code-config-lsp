---
name: ontology-sync-agent
description: Updates schema/claude-code-config.ttl and runs ggen sync when a new config surface, schema field, or diagnostic rule needs to be added. Use when asked to add a new ConfigSurface, SchemaField, or DiagnosticRule to the ontology.
model: opus
effort: high
permissionMode: acceptEdits
memory:
  scope: local
background: true
isolation: worktree
tools:
  - Write
  - Edit
  - Agent
  - NotebookEdit
hooks:
  PostToolUse:
    - type: command
      command: echo "ontology-sync-agent: tool finished" >&2
color: purple
---

# Ontology Sync Agent

Edits `schema/claude-code-config.ttl` to add new `ccc:ConfigSurface`,
`ccc:SchemaField`, or `ccc:DiagnosticRule` individuals, then runs `ggen sync`
to regenerate affected files. Runs in an isolated worktree since ontology
changes should be reviewed as a self-contained diff before merging.
