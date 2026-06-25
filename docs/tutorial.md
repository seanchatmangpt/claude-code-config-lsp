# Tutorial: Wire claude-code-config-lsp into a Claude Code agent loop

This tutorial walks an agent developer through installing claude-code-config-lsp
and observing it govern a Claude Code agent that edits its own configuration.
By the end, the agent loop has an ambient witness that pushes diagnostics on
every settings.json, CLAUDE.md, hook script, and agent definition the agent
writes — without the agent having to ask.

**What this is not:** a tutorial for building an LSP server from scratch.
claude-code-config-lsp is a finished server. This tutorial is about wiring it
in and watching it work.

---

## Step 1 — Install the plugin

claude-code-config-lsp ships as a Claude Code plugin. Install it with two
commands inside a Claude Code session:

```
/plugin marketplace add https://github.com/seanchatmangpt/claude-code-config-lsp
/plugin install claude-code-config-lsp@claude-code-config-lsp
```

After install, the plugin registers the LSP server in `.claude/settings.json`
under `lspServers`. Claude Code starts the server process the next time a
covered config file is opened.

**What the plugin provides:**

- `lspServers` entry pointing at the `claude-code-config-lsp` binary
- `extensionToLanguage` mappings: `.json` → `claude-config-json`,
  `.md` (in `.claude/agents/` and `.claude/skills/*/`) → `claude-config-markdown`,
  `.sh` (in `.claude/hooks/`) → `shellscript`,
  `.toml` (`.claude/lsp-max-auto.toml`) → `claude-config-toml`

The server activates on the covered file patterns — no further configuration
is required.

---

## Step 2 — What happens when Claude Code opens settings.json

When the agent opens `.claude/settings.json`, the LSP server:

1. Receives `textDocument/didOpen` from the Claude Code runtime
2. Dispatches the `json` analyzer for the `claude-config-json` surface
3. Validates the document against the `StaticSchema` field registry
4. Calls `publish_diagnostics()` with any violations — Claude Code receives
   these as inline diagnostics immediately, before the agent finishes reading
   the file

The agent sees diagnostics the same way a human editor would, except there is
no human. The agent reads the diagnostic message, corrects the file, and the
cycle repeats on `textDocument/didChange`.

No polling. No separate validation step. The server is always watching.

---

## Step 3 — Example: invalid model name in settings.json

An agent adds a model name that does not match the known model pattern:

```json
{
  "model": "claude-haiku-latest"
}
```

The `json` analyzer recognizes this as the `model` field on the
`claude-config-json` surface and emits:

```
CCC-JSON-001  [line 2]  Unknown model identifier "claude-haiku-latest".
              Use a versioned model ID such as "claude-haiku-4-5-20251001".
```

The agent receives this diagnostic via `textDocument/publishDiagnostics`, reads
the message, and corrects the file:

```json
{
  "model": "claude-haiku-4-5-20251001"
}
```

On the next `didChange`, the analyzer finds no violations and publishes an empty
diagnostic list. The agent proceeds.

**Haiku policy note:** agents that only read files may use Haiku. Any agent that
writes files — including edits to settings.json — must use `claude-sonnet-4-6`.
This is enforced by the same `CCC-JSON-001` family when an agent writes a Haiku
model ID into a config surface that governs a writing agent role.

---

## Step 4 — Example: CLAUDE.md missing required sections

An agent creates a `CLAUDE.md` for a new project:

```markdown
# My Project

Some general notes.
```

The `claude_md` analyzer checks for required sections and finds `## Commands`
absent. It emits:

```
CCC-MD-001  [line 1]  CLAUDE.md is missing required section "## Commands".
                      Add a Commands section documenting the project's build
                      and test recipes.
```

The agent adds the section:

```markdown
# My Project

Some general notes.

## Commands

- `cargo test` — run unit tests
- `just dx-polish` — format and lint
```

On the next `didChange`, the analyzer passes. The agent continues with its
primary task.

The `claude_md` analyzer also checks:

- `## See Also` links that resolve to nonexistent files (broken relative paths)
- Hook pattern references (`PreToolUse`, `PostToolUse`) that appear in prose but
  are not wired in `settings.json`

---

## Step 5 — Example: hook script with no shebang

An agent writes a `PreToolUse` hook at `.claude/hooks/gate-check.sh`:

```sh
lsp-max-cli gate check || exit 1
```

The `hook` analyzer checks for a shebang on line 1 and finds none. It emits:

```
CCC-HOOK-001  [line 1]  Hook script missing shebang. Add "#!/usr/bin/env bash"
                         as the first line. Without a shebang, the shell
                         executing this hook is undefined.
```

The agent prepends the shebang:

```sh
#!/usr/bin/env bash
lsp-max-cli gate check || exit 1
```

The `hook` analyzer also validates:

- Exit code semantics: a `PreToolUse` hook that exits 0 on all paths provides no
  blocking capability — `CCC-HOOK-002` warns when no non-zero exit path is
  reachable
- Env var references: `$CLAUDE_TOOL_NAME`, `$CLAUDE_TOOL_INPUT` — warns on
  references to variables the Claude Code runtime does not inject

---

## Step 6 — How the Declare model proves the lawful sequence

The server's `declare_model.rs` encodes four Van der Aalst Declare constraints
that govern the diagnostic pipeline:

```
Precedence("WorkspaceIndexed", "LsifExported")
Response("CompletionRequested", "CompletionProvided")
Response("DocumentOpened", "DiagnosticsPublished")
Response("HoverRequested", "HoverProvided")
```

The third constraint — `Response("DocumentOpened", "DiagnosticsPublished")` —
is the law that the agent-correction loop satisfies:

1. Agent opens `settings.json` → `DocumentOpened` event accumulated
2. Server dispatches analyzer → `DiagnosticsPublished` event accumulated
3. Agent corrects file → `didChange` triggers another analysis
4. Server re-runs analyzer → `DiagnosticsPublished` again (clean)

The OCEL accumulator records all six activity types across this flow:
`ConfigSurfaceOpened`, `DiagnosticPublished`, `ConformanceChecked`,
`HoverRequested`, `CompletionRequested`, `RepairApplied`.

After a correction cycle, `RepairApplied` appears in the log between two
`DiagnosticPublished` events. A pm4py conformance check against the Declare
model verifies that the normative Response constraint held: every open was
followed by a publish.

---

## Step 7 — Check workspace conformance

The virtual document `claude-config://health` shows live server state. In an
agent loop, request it as a hover target or via the `max/snapshot` method if
the compositor exposes it.

The health document reports:

```
WorkspaceConformance {
  score: 0.73,
  surfaces_checked: 8,
  surfaces_admitted: 6,
  violations: [
    "CCC-JSON-001: settings.json model field unresolved",
    "CCC-MD-001: AGENTS.md missing required section Commands"
  ]
}
```

A score of 1.0 means all surfaces passed their analyzers on the most recent
open/change cycle. An agent can gate a commit or a release on this score by
reading `claude-config://health` and parsing the `score` field before
proceeding.

---

## What the agent gains

Without this server, an agent writing invalid configuration gets no signal until
the configuration is used — which may be in a future session, after context has
been lost. The server closes the gap: every write to a covered config surface
produces a diagnostic within the same LSP exchange, in the same agent turn.

The agent-correction loop is: write → diagnostic → correct → clean diagnostic →
proceed. No human in the loop. No separate validation step. The server is always
watching.
