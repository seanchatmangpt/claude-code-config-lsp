# claude-code-config-lsp surface reference

Authoritative lookup tables. Stable facts; no procedural guidance.

---

## Config surfaces

| File pattern | Language ID | Analyzer | Primary diagnostic family |
|---|---|---|---|
| `**/.claude/settings*.json` | `claude-config-json` | `json` | `CCC-JSON-*` |
| `**/.claude/agents/*.md` | `claude-config-markdown` | `frontmatter` | `CCC-FM-*` |
| `**/.claude/skills/*/SKILL.md` | `claude-config-markdown` | `frontmatter` | `CCC-FM-*` |
| `**/.claude/hooks/*.sh` | `shellscript` | `hook` | `CCC-HOOK-*` |
| `**/.claude/lsp-max-auto.toml` | `claude-config-toml` | `toml` | `CCC-TOML-*` |
| `**/CLAUDE.md`, `**/AGENTS.md` | `claude-config-markdown` | `claude_md` | `CCC-MD-*` |
| `**/marketplace.json` | `claude-config-json` | `json` | `CCC-JSON-*` |
| `**/plugin.json` | `claude-config-json` | `json` | `CCC-JSON-*` |
| `**/mcp.json` | `claude-config-json` | `json` | `CCC-JSON-*` |
| `**/keybindings.json` | `claude-config-json` | `json` | `CCC-JSON-*` |

---

## settings.json schema

| Field | Type | Required | Notes |
|---|---|---|---|
| `model` | string | no | Versioned model ID. Haiku permitted for read-only agent roles only. |
| `effort` | string enum | no | `low` \| `medium` \| `high` \| `max` |
| `permissions.allow` | string[] | no | Tool names or glob patterns explicitly allowed |
| `permissions.deny` | string[] | no | Tool names or glob patterns explicitly denied |
| `hooks` | object | no | Keys: `PreToolUse`, `PostToolUse`, `Notification`, `Stop`, `SubagentStop` |
| `hooks.<event>[].matcher` | string | no | Tool name or glob; empty string matches all |
| `hooks.<event>[].hooks[].type` | string | yes | Must be `"command"` |
| `hooks.<event>[].hooks[].command` | string | yes | Shell command to run |
| `mcpServers` | object | no | Keyed by server name; each entry: `command`, `args`, `env` |
| `lspServers` | array | no | Each entry: `name`, `command`, `args`, `extensionToLanguage` |

---

## CLAUDE.md structure

| Section | Required | Notes |
|---|---|---|
| `## Commands` | yes | Build and test recipes |
| `## See Also` | no | Relative file paths; server checks that targets exist |
| Hook event names in prose | no | `PreToolUse`, `PostToolUse` references are cross-checked against `settings.json` |

Frontmatter is not required in CLAUDE.md or AGENTS.md.

---

## marketplace.json schema

| Field | Type | Required | Notes |
|---|---|---|---|
| `name` | string | yes | Cannot begin with `claude-`, `anthropic-`, `mcp-core-` |
| `owner` | string | yes | GitHub organization or user |
| `plugins` | array | yes | One entry per plugin |
| `plugins[].source` | string | yes | HTTPS URL or `local:<path>`. Bare paths and `http://` are refused. |
| `plugins[].name` | string | no | Display name |
| `plugins[].description` | string | no | Short description |

---

## plugin.json schema

| Field | Type | Required | Notes |
|---|---|---|---|
| `name` | string | yes | Plugin identifier |
| `version` | string | yes | CalVer or SemVer |
| `lspServers` | array | no | Each entry requires `name` and `command` |
| `lspServers[].command` | string | yes | Binary name or path |
| `lspServers[].args` | string[] | no | Defaults to `["--stdio"]` |
| `lspServers[].extensionToLanguage` | object | yes | Extension → language ID mapping |
| `skills` | array | no | Paths to SKILL.md files included in the plugin |
| `agents` | array | no | Paths to agent definition files |
| `hooks` | object | no | Same structure as `settings.json` hooks |
| `mcpServers` | object | no | Same structure as `settings.json` mcpServers |

---

## mcp.json schema

| Field | Type | Required | Notes |
|---|---|---|---|
| `mcpServers` | object | yes | Keyed by server name |
| `mcpServers.<name>.command` | string | yes | Executable |
| `mcpServers.<name>.args` | string[] | no | Command arguments |
| `mcpServers.<name>.env` | object | no | Environment variables injected at startup |
| `mcpServers.<name>.type` | string | no | `stdio` (default) \| `sse` |

---

## Agent frontmatter schema (`.claude/agents/*.md`)

| Field | Type | Required | Notes |
|---|---|---|---|
| `name` | string | yes | Identifier; lowercase, hyphens, max 64 chars |
| `description` | string | yes | When to invoke this agent; used for auto-routing |
| `model` | string | no | Model ID override. Haiku: read-only roles only. |
| `effort` | string | no | `low` \| `medium` \| `high` \| `max` |
| `maxTurns` | integer | no | Maximum turns before the agent stops |
| `tools` | string[] | no | Tool allowlist; defaults to parent's set |
| `disallowedTools` | string[] | no | Tools explicitly blocked |
| `isolation` | string | no | `worktree` — agent runs in a git worktree |

---

## Skill frontmatter schema (`.claude/skills/*/SKILL.md`)

| Field | Type | Required | Notes |
|---|---|---|---|
| `name` | string | yes | Identifier; lowercase, hyphens, max 64 chars |
| `description` | string | yes | Max 1024 chars. States WHAT and WHEN. Used for discovery. |
| `disable-model-invocation` | boolean | no | Prevents additional model calls from within the Skill |

---

## Hook script requirements

| Requirement | Diagnostic | Notes |
|---|---|---|
| Shebang on line 1 | `CCC-HOOK-001` | `#!/usr/bin/env bash` or `#!/bin/sh` |
| At least one non-zero exit path | `CCC-HOOK-002` | `PreToolUse` hooks with only `exit 0` provide no blocking |
| Valid env var references | `CCC-HOOK-003` | Only injected vars: `CLAUDE_TOOL_NAME`, `CLAUDE_TOOL_INPUT`, `CLAUDE_SESSION_ID` |

Hook type is inferred from the filename stem: `pre-tool-use*` → `PreToolUse`,
`post-tool-use*` → `PostToolUse`, `stop*` → `Stop`, `subagent-stop*` →
`SubagentStop`, `notification*` → `Notification`.

---

## Virtual document URIs

| URI | Content | Notes |
|---|---|---|
| `claude-config://health` | `WorkspaceConformance` JSON | Live server state: score, surfaces_checked, surfaces_admitted, violations list |

---

## Diagnostic code families

### CCC-JSON-* — JSON surface violations

| Code | Severity | Surface | Description |
|---|---|---|---|
| `CCC-JSON-001` | Error | settings.json | Unknown or unversioned model identifier |
| `CCC-JSON-002` | Error | marketplace.json | Missing required top-level field |
| `CCC-JSON-003` | Error | marketplace.json | Invalid source URL format |
| `CCC-JSON-004` | Error | marketplace.json | Reserved plugin name prefix |
| `CCC-JSON-005` | Error | plugin.json | Missing `lspServers[].command` |
| `CCC-JSON-006` | Warning | plugin.json | Missing `extensionToLanguage` mapping |
| `CCC-JSON-007` | Error | mcp.json | Missing `mcpServers.<name>.command` |
| `CCC-JSON-008` | Warning | keybindings.json | Unknown key binding action |

### CCC-MD-* — Markdown surface violations

| Code | Severity | Surface | Description |
|---|---|---|---|
| `CCC-MD-001` | Error | CLAUDE.md, AGENTS.md | Missing required section |
| `CCC-MD-002` | Warning | CLAUDE.md | Broken `## See Also` link (target file not found) |
| `CCC-MD-003` | Warning | CLAUDE.md | Hook event name referenced in prose but not wired in settings.json |

### CCC-FM-* — Frontmatter violations

| Code | Severity | Surface | Description |
|---|---|---|---|
| `CCC-FM-001` | Error | agents/*.md, SKILL.md | Missing required frontmatter field |
| `CCC-FM-002` | Error | agents/*.md, SKILL.md | `name` field violates naming constraints |
| `CCC-FM-003` | Warning | agents/*.md | Haiku model in a writing agent role |
| `CCC-FM-004` | Warning | SKILL.md | `description` exceeds 1024 characters |
| `CCC-FM-005` | Info | SKILL.md | `allowed-tools` present — CLI only, ignored by SDK |

### CCC-HOOK-* — Hook script violations

| Code | Severity | Description |
|---|---|---|
| `CCC-HOOK-001` | Error | Missing shebang on line 1 |
| `CCC-HOOK-002` | Warning | No non-zero exit path — hook cannot block tool calls |
| `CCC-HOOK-003` | Warning | Reference to env var not injected by Claude Code |

### CCC-TOML-* — TOML config violations

| Code | Severity | Description |
|---|---|---|
| `CCC-TOML-001` | Error | Parse error in `.claude/lsp-max-auto.toml` |
| `CCC-TOML-002` | Warning | Unknown top-level key |

---

## OCEL event types

All six activities are object-centric; the object is the config surface URI.

| Activity | Attributes | Notes |
|---|---|---|
| `ConfigSurfaceOpened` | `uri`, `language_id`, `surface_type` | Emitted on `didOpen` |
| `DiagnosticPublished` | `uri`, `code`, `severity`, `message` | One event per diagnostic emitted |
| `ConformanceChecked` | `uri`, `score`, `admitted`, `refused` | After each `analyze()` call |
| `HoverRequested` | `uri`, `line`, `character`, `field_name` | On `textDocument/hover` |
| `CompletionRequested` | `uri`, `line`, `character`, `trigger` | On `textDocument/completion` |
| `RepairApplied` | `uri`, `code`, `before_count`, `after_count` | When diagnostic count decreases after a change |

---

## Declare constraints (formal semantics)

All four constraints are LTL-based Declare templates applied to the OCEL event
stream, per trace keyed on config surface URI.

| Constraint | Formal | Meaning |
|---|---|---|
| `Precedence("WorkspaceIndexed", "LsifExported")` | □(LsifExported → ◇[past] WorkspaceIndexed) | LSIF export can only occur after workspace indexing |
| `Response("CompletionRequested", "CompletionProvided")` | □(CompletionRequested → ◇ CompletionProvided) | Every completion request must be answered |
| `Response("DocumentOpened", "DiagnosticsPublished")` | □(DocumentOpened → ◇ DiagnosticsPublished) | Every open must produce a diagnostic publication |
| `Response("HoverRequested", "HoverProvided")` | □(HoverRequested → ◇ HoverProvided) | Every hover request must be answered |

A pm4py conformance check against these constraints detects silent failures:
e.g., `DiagnosticsPublished` events without a preceding `DocumentOpened` indicate
a server publishing unsolicited diagnostics, which violates `Response` in reverse.

---

## Haiku policy

Haiku models (`claude-haiku-*`) are permitted only for agent roles that are
**read-only**: roles that never write files, never edit configuration, and never
invoke tools that mutate state.

Any agent that writes files — including configuration surfaces — must use
`claude-sonnet-4-6`. This is a hard policy, not a recommendation. The
`CCC-FM-003` diagnostic enforces it for agent frontmatter.

---

## Conformance status vocabulary

| Status | Meaning |
|---|---|
| `ADMITTED` | Law axis passes; receipt chain closed |
| `CANDIDATE` | Implementation present; receipt chain OPEN |
| `BLOCKED` | Dependency or prerequisite unmet |
| `REFUSED` | Law axis explicitly rejects |
| `UNKNOWN` | Axis not yet traced; cannot collapse to ADMITTED or REFUSED |
| `PARTIAL` | Some axes admitted, others OPEN |
| `OPEN` | Absent; setup required |

Never use: `done`, `complete`, `solved`, `guaranteed`, `all clean`.
