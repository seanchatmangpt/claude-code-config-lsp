# How-to: common tasks with claude-code-config-lsp

Concrete recipes for specific tasks. Each recipe assumes the plugin is installed
and the server is running inside a Claude Code session.

---

## 1. Validate all config surfaces in a workspace

When the plugin is installed, validation is automatic — every covered file that
the agent opens or changes is analyzed immediately. To trigger validation across
all surfaces at once, instruct the agent to open each covered file:

```
Open .claude/settings.json, .claude/settings.local.json, CLAUDE.md, AGENTS.md,
all files in .claude/agents/, all SKILL.md files under .claude/skills/,
all .sh files under .claude/hooks/, and .claude/lsp-max-auto.toml.
```

The server publishes diagnostics for each file as it is opened. After all files
are opened, query `claude-config://health` to see the aggregate
`WorkspaceConformance` score and the full violation list.

---

## 2. Add a new config surface to the ontology

Never edit generated source directly. The ontology is the source of truth.

**1 — Add a `ConfigSurface` individual to `schema/claude-code-config.ttl`:**

```turtle
ccc:MyNewSurface a ccc:ConfigSurface ;
    ccc:filePattern        "**/.claude/my-new-config.json" ;
    ccc:languageId         "claude-config-json" ;
    ccc:analyzerModule     "json" ;
    ccc:analyzerModulePascal "Json" .
```

The `analyzerModule` value maps to the existing `src/analyzers/json.rs` — if
the new surface needs its own analyzer, give it a unique module name and
implement the stub that `ggen sync` generates.

**2 — Run `ggen sync`:**

```sh
ggen sync
```

`src/file_types.rs` is regenerated with the new pattern in the `language_id`
match. If a new analyzer module was declared, the stub appears at
`src/analyzers/<module>.rs`.

**3 — Verify:**

```sh
cargo check
cargo test
```

---

## 3. Add a new diagnostic rule to an existing analyzer

All diagnostic rules flow from the ontology. Adding a rule means:

**1 — Add a `DiagnosticRule` individual to the TTL:**

```turtle
ccc:RequireHooksSection a ccc:DiagnosticRule ;
    ccc:code         "CCC-MD-003" ;
    ccc:surface      ccc:ClaudeMd ;
    ccc:severity     ccc:Error ;
    ccc:message      "CLAUDE.md is missing a Hooks section documenting registered hooks." .
```

**2 — Run `ggen sync`** to propagate the code into the `StaticSchema` registry
and any hover/completion tables that list diagnostic codes.

**3 — Implement the check** in `src/analyzers/claude_md.rs`:

```rust
// Check for ## Hooks section
if !content.contains("## Hooks") {
    diags.push(Diagnostic {
        range: Range::new(Position::new(0, 0), Position::new(0, 0)),
        severity: Some(DiagnosticSeverity::ERROR),
        code: Some(NumberOrString::String("CCC-MD-003".into())),
        message: "CLAUDE.md is missing a Hooks section documenting registered hooks.".into(),
        ..Default::default()
    });
}
```

The ontology entry ensures the code appears in hover docs and the reference
table. The Rust implementation is what enforces it at runtime.

---

## 4. Use hover to get field documentation in an agent loop

The `hover.rs` module returns field documentation when the agent positions the
cursor over a recognized key. In an agent loop, the agent sends
`textDocument/hover` before writing to a field it is uncertain about.

**settings.json — hovering on `"model"`:**

```
Field: model
Type: string
Required: no (defaults to claude-sonnet-4-6)
Description: Model ID for this workspace. Must be a versioned identifier.
             Haiku models are permitted only for agents in read-only roles.
Example: "claude-sonnet-4-6"
```

**Agent frontmatter — hovering on `effort`:**

```
Field: effort
Type: string enum
Values: low | medium | high | max
Required: no (defaults to medium)
Description: Token budget hint for this subagent. "max" disables budget
             enforcement; use only for tasks that require deep reasoning.
```

**Hook scripts — hovering on `PreToolUse`:**

```
Event: PreToolUse
Injected env vars: CLAUDE_TOOL_NAME, CLAUDE_TOOL_INPUT (JSON)
Exit semantics: exit 0 = allow tool call; exit 1 = block tool call
Scope: Runs before every tool invocation that matches the hook's matcher.
```

An agent that uses hover before writing avoids the write → diagnostic → correct
cycle for fields it already knows about.

---

## 5. Fix marketplace.json violations

Common `CCC-JSON-*` violations in `marketplace.json`:

**Missing `name` field (CCC-JSON-002):**

```json
{
  "plugins": [{ "source": "..." }]
}
```

Fix: add `"name"` at the top level:

```json
{
  "name": "my-org-marketplace",
  "owner": "my-org",
  "plugins": [{ "source": "..." }]
}
```

**Invalid source format (CCC-JSON-003):** `source` must be a full HTTPS URL or
a `local:<path>` reference. Bare paths and `http://` sources are refused.

```json
{ "source": "github.com/my-org/my-plugin" }   // CCC-JSON-003
{ "source": "https://github.com/my-org/my-plugin" }  // CANDIDATE
```

**Reserved plugin name (CCC-JSON-004):** names beginning with `claude-`,
`anthropic-`, or `mcp-core-` are reserved.

---

## 6. Fix plugin.json violations

The `plugin.json` at `.claude-plugin/plugin.json` declares the server. Common
violations:

**Missing `lspServers.command` (CCC-JSON-005):**

```json
{
  "lspServers": [{ "name": "my-lsp" }]
}
```

Fix: add `"command"` pointing at the binary:

```json
{
  "lspServers": [{
    "name": "my-lsp",
    "command": "my-lsp-binary",
    "args": ["--stdio"]
  }]
}
```

**Missing `extensionToLanguage` (CCC-JSON-006):** without this mapping, Claude
Code does not know which files to send to the server.

```json
{
  "extensionToLanguage": {
    ".json": "claude-config-json",
    ".md": "claude-config-markdown"
  }
}
```

---

## 7. Check config conformance before committing

Before committing changes to config surfaces, query `WorkspaceConformance`:

```
Read the virtual document at claude-config://health.
If score < 1.0, list the violations and fix each one before proceeding.
```

In an automated pre-commit hook at `.claude/hooks/pre-commit.sh`:

```sh
#!/usr/bin/env bash
# Query the health virtual doc via the LSP client CLI
score=$(lsp-max-cli snapshot read --uri "claude-config://health" \
        | jq -r '.score')

if [ "$(echo "$score < 1.0" | bc)" = "1" ]; then
    echo "Config conformance BLOCKED: score=$score"
    lsp-max-cli snapshot read --uri "claude-config://health" \
        | jq -r '.violations[]'
    exit 1
fi
```

This gate prevents the agent from committing config surfaces that the analyzer
has already rejected.
