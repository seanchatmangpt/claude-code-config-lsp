---
name: validate-config
description: Validate Claude Code config files and report workspace conformance. Use when the user asks to check, validate, or diagnose settings.json, CLAUDE.md, plugin.json, marketplace.json, mcp.json, hooks, agents, or skills — or to get a conformance score for the workspace.
user-invocable: true
allowed-tools:
  - Read
  - Bash(cargo build *)
  - Bash(cargo check *)
  - Bash(find * -name "settings.json" *)
  - Bash(find * -name "CLAUDE.md" *)
  - Bash(find * -name "plugin.json" *)
  - Bash(find * -name "marketplace.json" *)
  - Bash(find * -name "mcp.json" *)
  - Bash(find * -name "*.sh" *)
  - Bash(cat *)
  - Bash(jq *)
---

# validate-config

Validates Claude Code configuration files in the current workspace using the
`claude-code-config-lsp` server's conformance rules.

Arguments: `$ARGUMENTS`

---

## What this skill does

1. **Discovers** all Claude Code config surfaces in the workspace:
   - `settings.json` / `settings.local.json` (permissions, model, hooks)
   - `CLAUDE.md` / `AGENTS.md` (project instructions)
   - `plugin.json` / `marketplace.json` (plugin declarations)
   - `mcp.json` (MCP server wiring)
   - `hooks/*.sh` (hook scripts)
   - `agents/*.md` (agent frontmatter)
   - `skills/*/SKILL.md` (skill metadata)

2. **Validates** each file against the LSP's conformance rules:
   - JSON schema correctness
   - Required fields present
   - Hook scripts executable and syntactically valid
   - Frontmatter YAML well-formed
   - Declare constraints satisfied

3. **Reports** findings with severity (error / warning / info) and location.

4. **Scores** the workspace conformance (0–100) based on violations found.

---

## Execution

When invoked, perform these steps:

### Step 1 — Discover config files

```bash
find . -maxdepth 5 \( \
  -name "settings.json" -o \
  -name "settings.local.json" -o \
  -name "CLAUDE.md" -o \
  -name "AGENTS.md" -o \
  -name "plugin.json" -o \
  -name "marketplace.json" -o \
  -name "mcp.json" -o \
  -name "keybindings.json" \
\) -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/target/*"
```

Also find hooks and agent/skill markdown:
```bash
find . -maxdepth 6 -path "*/hooks/*.sh" -not -path "*/target/*"
find . -maxdepth 6 \( -name "agents/*.md" -o -name "skills/*/SKILL.md" \) -not -path "*/target/*"
```

### Step 2 — Validate each file

For JSON files, check with `jq`:
```bash
jq empty <file> 2>&1 && echo "valid JSON" || echo "INVALID JSON"
```

For `settings.json`, check for required structure:
- Must have at least one of: `permissions`, `model`, `hooks`, `env`
- `permissions.allow` and `permissions.deny` must be arrays if present
- Hook entries must have `hooks` key with array value

For `plugin.json`, check:
- `name`, `version`, `description` present
- `lspServers` or `mcpServers` or `skills` present

For `marketplace.json`, check:
- `plugins` array present with at least one entry
- Each plugin has `name`, `source`, `description`

For hook scripts (`.sh`), check:
```bash
bash -n <file> 2>&1 && echo "syntax OK" || echo "SYNTAX ERROR"
```

For agent/skill frontmatter (`.md`), read the `---` block and verify YAML:
- `name` field present
- `description` field present (skills)
- `user-invocable` is boolean if present

### Step 3 — Report findings

Output a summary table:

```
File                        Status    Findings
─────────────────────────── ──────── ────────────────────────────────
settings.json               ✓ OK     —
plugin.json                 ✗ ERROR  Missing required field: version
hooks/pre-commit.sh         ⚠ WARN   Script not executable (chmod +x)
agents/review.md            ✓ OK     —
```

Then list each finding with:
- File path (relative)
- Line number if determinable
- Severity: ERROR / WARNING / INFO
- Message

### Step 4 — Conformance score

Compute score:
- Start at 100
- Subtract 10 per ERROR
- Subtract 3 per WARNING
- Clamp to 0–100

Report:
```
Workspace conformance: 87/100
  3 warnings, 0 errors across 8 config files
```

If score < 70: "Action required — fix errors to restore conformance."
If score 70–89: "Acceptable — review warnings when convenient."
If score ≥ 90: "Conformant workspace."

---

## Argument handling

- No args: validate entire workspace from current directory
- File path arg: validate only that file
- `--score`: print only the numeric conformance score (for scripting)
- `--fix`: attempt auto-fix for simple issues (chmod +x on hook scripts, add missing trailing newlines)
