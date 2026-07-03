---
name: conformance-report
description: Query claude-config://health and render the workspace's WorkspaceConformance report as a readable summary. Use when asked for the current conformance score, or to check for open violations before a release.
user-invocable: true
default-enabled: true
disable-model-invocation: false
context: fork
agent: config-conformance-reviewer
effort: low
argument-hint: "[--json]"
allowed-tools:
  - Read
  - Bash(cargo build *)
disallowed-tools:
  - Write
  - Edit
hooks:
  PreToolUse:
    - type: command
      command: echo "conformance-report: about to run a tool" >&2
metadata:
  category: conformance
  owner: config-conformance-reviewer
---

# conformance-report

Arguments: `$ARGUMENTS`

1. Build the server: `cargo build`
2. Query `claude-config://health` via the running LSP session.
3. If `$ARGUMENTS` contains `--json`, print the raw JSON. Otherwise render a
   short table: score, surfaces checked, surfaces admitted, open violations.
