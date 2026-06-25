# Why Claude Code config is a first-class law surface

Conceptual background — the "why" behind the design. This is not a how-to;
it is an account of the forces that produced the architecture.

---

## Claude Code as the agent being governed

Claude Code is an AI coding agent. It reads files, writes files, runs shell
commands, and manages its own configuration. The configuration surfaces —
`settings.json`, `CLAUDE.md`, `agents/*.md`, `skills/*/SKILL.md`,
`hooks/*.sh`, `marketplace.json`, `plugin.json` — are the law that governs
what the agent is allowed to do, how it routes tasks to subagents, and which
tools it may invoke.

When the agent edits these surfaces, it is rewriting its own governing law.
This is not a hypothetical: agents routinely add hook scripts, extend
`CLAUDE.md` with new instructions, and write agent definitions for subagent
delegation. Every one of these edits is a write to a law surface.

Without an ambient witness, the agent has no signal that a law surface edit
was malformed until it tries to use the result — which may be sessions later,
after the context that produced the error is long gone.

---

## Why config errors are silent without LSP

The Claude Code runtime validates configuration lazily. A `settings.json`
with an unknown model ID does not fail at write time; it fails when the agent
tries to start a session with that model. A `CLAUDE.md` missing a required
section does not fail at write time; it degrades agent behavior silently,
because the section that would have guided the agent simply does not exist.

This is the gap: **between the moment the agent writes a law surface and the
moment the agent exercises it, there is no witness.**

LSP closes the gap. The LSP protocol's `textDocument/publishDiagnostics`
notification is push-based: the server sends diagnostics to the client without
the client asking. This means the moment the agent writes to a covered file,
the server can immediately — in the same agent turn — send back a diagnostic
describing what is wrong and why.

The agent receives the diagnostic the same way a human editor receives a red
squiggle. The correction loop is synchronous with the write, not deferred to
a future session.

---

## The ggen + ontology architecture

claude-code-config-lsp is generated from an RDF ontology
(`schema/claude-code-config.ttl`). The ontology describes:

- Which config file types exist (`ccc:ConfigSurface` individuals)
- Which schema fields each surface exposes (`ccc:SchemaField`)
- Which diagnostic rules apply to each surface (`ccc:DiagnosticRule`)
- Which Declare constraints govern the event flow (`ccc:DeclareConstraint`)
- Which OCEL activity types the server emits

SPARQL queries extract rows from the ontology. Tera templates render those
rows into Rust source: `src/file_types.rs`, `src/schema.rs`,
`src/analyzers/mod.rs`, hover tables, completion tables, the Declare model.

The consequence is that the TTL is the source of truth, not the Rust code.
When a new field is added to the Claude Code agent frontmatter spec, the
change goes into the TTL, `ggen sync` runs, and hover documentation,
completion items, and diagnostic codes all update in lockstep. There is no
risk of documentation describing fields the analyzer does not check, or the
analyzer checking fields that hover does not document, because both are
projections of the same ontology.

Generated Rust files are first-class source. The `GGEN-SRC-*` diagnostic
family enforces this: generated files must not carry `DO NOT EDIT` banners,
must not live in `generated/` or `output/` paths, and must be inspected and
maintained as source. If a generated analyzer stub contains wrong logic, the
fix goes into the ontology or the pack template — not into a hand-edited
workaround that diverges from the TTL on the next `ggen sync`.

---

## The five surfaces of Claude Code config law

Claude Code configuration is not a single file. It is five distinct law
surfaces, each governing a different aspect of agent behavior:

**1. Settings** (`settings.json`, `settings.local.json`) — what the agent
can do. Model selection, tool permissions, hook wiring, MCP server
declarations. This is the runtime permission surface: it determines which
tools the agent may call, which hooks block or observe those calls, and which
model executes the agent's reasoning.

**2. Marketplace** (`marketplace.json`) — what plugins are available. The
marketplace surface governs which external capabilities can be added to the
agent's environment. A malformed marketplace entry can cause silent failures
when the agent tries to install a plugin.

**3. Plugins** (`plugin.json`) — what a plugin provides. Each plugin
declares its LSP servers, skills, agents, hooks, and MCP servers. The
plugin.json surface is the contract between the plugin author and the Claude
Code runtime: a missing `lspServers[].command` means the LSP server never
starts, and the agent gets no diagnostics from it.

**4. Agents** (`agents/*.md`) — how subagents behave. Agent definitions
declare the model, effort, tool set, isolation mode, and routing description
for each subagent. The `description` field is used by Claude Code for
automatic subagent routing: if it is vague, the wrong subagent is invoked.
If the model field specifies Haiku for a writing agent, the Haiku policy is
violated.

**5. Skills** (`skills/*/SKILL.md`) — what skills are available. Skill
definitions declare the discovery metadata and tool restrictions for each
skill. The `description` field is injected into the system prompt for skill
discovery: if it exceeds 1024 characters, Claude Code may truncate it,
breaking discovery. The `allowed-tools` field is CLI-only: SDK callers
control tool access through a different mechanism, and an agent relying on
`allowed-tools` for security in an SDK context has a silent permission gap.

---

## How plugin marketplace distribution closes the deployment gap

An LSP server that is not installed provides no diagnostics. The plugin
marketplace mechanism solves this by making claude-code-config-lsp
installable with a single command:

```
/plugin marketplace add https://github.com/seanchatmangpt/claude-code-config-lsp
/plugin install claude-code-config-lsp@claude-code-config-lsp
```

The plugin's `plugin.json` declares the `lspServers` entry that wires the
binary into Claude Code's LSP client. After install, the server is ambient:
it starts automatically, watches covered file patterns, and pushes diagnostics
without any further configuration by the agent or the agent developer.

The marketplace model also means the server can be distributed alongside the
skills and agents that depend on correct configuration. A skill that requires
a specific `settings.json` structure can declare that structure in its
`SKILL.md`, and the LSP server will validate it the moment the agent writes
the settings file.

---

## The relationship to lsp-max

claude-code-config-lsp is a consumer of the lsp-max framework. It implements
the `RulePackServer` trait from lsp-max, which means:

- It gains `ConformanceVector` with admitted/refused/unknown law-axis sets
- Its diagnostics feed into the lsp-max compositor's OCEL event stream
- The `max/snapshot` and `max/gate` surfaces from lsp-max are available to
  agents querying the server's state
- The ANDON gate mechanism — which blocks tool calls when active diagnostics
  are present — can be wired to fire on `CCC-*` diagnostic families

The lsp-max compositor accumulates OCEL 2.0 events from all wired servers,
including claude-code-config-lsp. This means the config surface's event flow
— opens, diagnostic publications, hover requests, completion requests, repairs
— appears in the same process log as events from other LSP servers in the
workspace. A pm4py conformance check can verify that the normative Declare
constraints held across all servers simultaneously.

---

## Why tower-lsp is a law violation

The lsp-max framework is a fork and extension of tower-lsp. Using tower-lsp
directly instead of lsp-max bypasses three critical mechanisms:

**1. ConformanceVector.** lsp-max's `RulePackServer` trait carries a
`ConformanceVector` that tracks admitted, refused, and unknown law-axis sets
for every capability the server advertises. tower-lsp has no such mechanism.
An LSP server built on tower-lsp can claim any capability without evidence,
and there is no runtime enforcement that the claim is honest.

**2. Receipt chain.** lsp-max gates capability status changes on receipt
artifacts: a capability cannot move from `CANDIDATE` to `ADMITTED` without
a transcript, a negative control, and a SHA256-bounded receipt. tower-lsp has
no receipt mechanism. An agent that builds on tower-lsp can assert `ADMITTED`
for capabilities it has never demonstrated.

**3. Gate enforcement.** The lsp-max ANDON gate blocks tool calls and file
mutations when active diagnostics are present. This gate is wired at the
`PreToolUse` hook level and depends on `lsp-max-cli gate check`. A server
built on tower-lsp is invisible to the gate: its diagnostics do not feed into
the gate's decision, and the agent can proceed past diagnostic violations that
should block it.

The `anti-llm-cheat-lsp` server detects plain `tower-lsp` and `tower_lsp`
references in Rust source and emits `ANTI-LLM-CHEAT-LSP-FORBIDDEN-REF`
diagnostics. This is enforced by `just dx-verify` across the workspace.
Using tower-lsp is not a style choice; it is a law violation that routes
around the evidence, admission, and gate machinery that makes lsp-max a
trustworthy runtime for agent-governed law surfaces.
