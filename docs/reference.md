# Reference

Authoritative lookup tables. Stable facts; no step-by-step guidance.

---

## SKILL.md Frontmatter Field Reference

| Field | Required | Type | Constraints | Scope |
|---|---|---|---|---|
| `name` | yes | string | ≤ 64 chars; lowercase, numbers, hyphens only; cannot contain `anthropic` or `claude` | CLI + SDK |
| `description` | yes | string | Non-empty; ≤ 1024 chars; third-person; must state WHAT and WHEN | CLI + SDK |
| `allowed-tools` | no | list | Tool names or `mcp__<server>__*` wildcards | **CLI only** |
| `disallowed-tools` | no | list | Tool names explicitly blocked | CLI + SDK |
| `context` | no | enum | `fork` — isolate Skill in a branched context | CLI + SDK |
| `paths` | no | glob list | File path patterns the Skill may access | CLI + SDK |
| `disable-model-invocation` | no | bool | Prevents the Skill from invoking additional model calls | CLI + SDK |

### Skill Name Violation Codes

| Code | Description |
|---|---|
| `CCC-SKILL-001` | Name exceeds 64 characters |
| `CCC-SKILL-002` | Name contains uppercase characters |
| `CCC-SKILL-003` | Name contains reserved word (`anthropic`, `claude`) |
| `CCC-SKILL-004` | Description is missing or empty |
| `CCC-SKILL-005` | Description exceeds 1024 characters |
| `CCC-SKILL-006` | `allowed-tools` present — only applies in CLI mode (warning in SDK context) |

---

## AgentDefinition Fields (SDK)

| Field | Required | Type | Notes |
|---|---|---|---|
| `description` | yes | string | When to auto-invoke this subagent; used for task matching |
| `prompt` | yes | string | System prompt for the subagent context |
| `tools` | no | string[] | Allowlist of tool names; defaults to parent's tool set |
| `model` | no | string | Model override (e.g. `claude-haiku-4-5-20251001`) |

---

## MCP Tool Naming

| Pattern | Meaning |
|---|---|
| `mcp__<server>__<tool>` | Specific tool from a named MCP server |
| `mcp__<server>__*` | All tools from a named MCP server (wildcard) |

`<server>` is the MCP server name as declared in `.claude/settings.json` under
`mcpServers`. `<tool>` is the tool's registered name on that server.

---

## Claude Code Config File Surfaces

| File / Pattern | Language ID | Analyzer Module | Config Surface |
|---|---|---|---|
| `CLAUDE.md` | `markdown` | `claude_md` | Project instructions |
| `SKILL.md` | `markdown` | `claude_md` | Skill definition |
| `.claude/settings.json` | `json` | `json` | Workspace settings |
| `.claude/settings.local.json` | `json` | `json` | Local overrides |
| `*.claude.json` | `json` | `json` | Portable config |
| `.claude/hooks/*.sh` | `shellscript` | `hook` | Hook scripts |
| Frontmatter in `.md` | `markdown` | `frontmatter` | YAML frontmatter |
| `*.toml` (Claude config) | `toml` | `toml` | TOML config |

---

## settings.json Schema

```json
{
  "model": "string — model ID",
  "mcpServers": {
    "<server-name>": {
      "command": "string",
      "args": ["string"],
      "env": { "KEY": "value" }
    }
  },
  "permissions": {
    "allow": ["tool-name or glob"],
    "deny": ["tool-name or glob"]
  },
  "hooks": {
    "PreToolUse": [{ "matcher": "ToolName", "hooks": [{ "type": "command", "command": "..." }] }],
    "PostToolUse": [...],
    "Notification": [...],
    "Stop": [...],
    "SubagentStop": [...]
  }
}
```

---

## LSP Capabilities Emitted by claude-code-config-lsp

| LSP Method | Status | Notes |
|---|---|---|
| `textDocument/hover` | CANDIDATE | Field documentation from SKILL.md ontology |
| `textDocument/completion` | CANDIDATE | Frontmatter field names, MCP tool patterns |
| `textDocument/publishDiagnostics` | CANDIDATE | CCC-SKILL-* codes |
| `textDocument/semanticTokens/full` | CANDIDATE | Token types: keyword, string, property, enumMember, namespace, variable |
| `textDocument/semanticTokens/range` | CANDIDATE | Range-restricted semantic tokens |

All are CANDIDATE — receipt chains OPEN.

---

## RulePackServer Trait (lsp-max)

### Required methods

```rust
fn rule_packs(&self)  -> &ValidatedRulePackSet;
fn grammar(&self)     -> tree_sitter::Language;
fn server_name(&self) -> &'static str;
fn client(&self)      -> &Client;
fn adapter(&self)     -> &AutoLspAdapter;
```

### Optional overrides

```rust
fn workspace_index(&self)    -> Option<&WorkspaceIndex> { None }
fn scan_uri_classified(&self, uri: &DocumentUri, content: &str) -> ClassifiedFindings {
    // default: runs ValidatedRulePackSet against AST
}
```

### Key types

| Type | Import path | Notes |
|---|---|---|
| `AutoLspAdapter` | `lsp_max::ast::AutoLspAdapter` | Tree-sitter AST adapter |
| `ValidatedRulePackSet` | `lsp_max::rule_pack_server::ValidatedRulePackSet` | Use `::empty()` for engine-bridge servers |
| `WorkspaceIndex` | `lsp_max::rule_pack_server::WorkspaceIndex` | `Arc<DashMap<String, IndexedDoc>>` |
| `ClassifiedFindings` | `lsp_max::rule_pack_server::ClassifiedFindings` | `(Vec<Finding>, Vec<Finding>)` sync + background |
| `Finding` | `lsp_max::rule_pack_server::Finding` | `(MaxDiagnostic, Diagnostic)` |
| `LawAxis` | `lsp_max::max_protocol::LawAxis` | `Domain`, `Security`, `Custom(String)`, etc. |

---

## ggen.toml Rule Fields

| Field | Type | Default | Notes |
|---|---|---|---|
| `name` | string | required | Unique rule identifier |
| `query` | Pack\|File\|Inline | required | SPARQL SELECT source |
| `template` | Pack\|File\|Inline | required | Tera template source |
| `output_file` | Tera pattern | required | May contain `{{ var }}` for dynamic output |
| `mode` | enum | `Create` | `Create` (fails if exists), `Overwrite`, `Merge` |
| `skip_empty` | bool | `false` | If true, skip rule when query returns 0 rows |
| `when` | SPARQL ASK | — | Gate rule on a boolean SPARQL query |

### Query source variants

```toml
query = { inline = "SELECT ('value' AS ?col) WHERE {}" }
query = { file  = "sparql/my_query.rq" }
query = { pack  = "lsp-max", output = "queries", file = "capabilities.sparql" }
```

### Template source variants

```toml
template = { inline = "{% for row in results %}...{% endfor %}" }
template = { file   = "templates/my_template.rs.tera" }
template = { pack   = "lsp-max", output = "templates", file = "backend.rs.tera" }
```

---

## Tera Context Variables

| Variable | Type | Value |
|---|---|---|
| `results` | array | All rows from the SPARQL SELECT |
| `row.<col>` | string | Column value for current row (in `{% for row in results %}`) |
| `<col>` | string | First-row scalar shorthand (static templates only) |

Zero-row behavior: if `skip_empty = false` (default) and query returns 0 rows,
the output file is not written — the declared `output_file` never appears. Use
`skip_empty = true` only where an absent file is acceptable.

---

## Conformance Status Vocabulary

| Status | Meaning |
|---|---|
| `ADMITTED` | Law axis passes; receipt chain closed |
| `CANDIDATE` | Implementation present; receipt chain OPEN |
| `BLOCKED` | Dependency or prerequisite unmet |
| `REFUSED` | Law axis explicitly rejects (e.g. forbidden ref detected) |
| `UNKNOWN` | Axis not yet traced; cannot collapse to ADMITTED or REFUSED |
| `PARTIAL` | Some axes admitted, others open |
| `OPEN` | Absent; setup required |

Never use: `done`, `complete`, `solved`, `guaranteed`, `all clean`.
