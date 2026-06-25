# Explanation

Conceptual background — the "why" behind the design.

---

## Why ggen and the lsp-max pack?

An LSP server is a large, repetitive artifact: `initialize`, `textDocument/hover`,
`textDocument/completion`, `textDocument/publishDiagnostics`, `shutdown`, and
many more — all following identical structural patterns but with domain-specific
logic at the leaves. Hand-writing this structure invites drift between capability
advertisement (what the server claims it can do) and handler registration (what
it actually does), between the diagnostic codes in the hover docs and the codes
the analyzers emit, and between the tree-sitter grammar and the semantic token
legend.

The ggen + lsp-max pack approach collapses all these surfaces to a single source
of truth: the domain ontology (`.ttl`). The ontology describes:
- Which config file types exist (`ccc:ConfigSurface`)
- Which LSP methods are ADMITTED/CANDIDATE/REFUSED (`law:status`)
- Which schema fields each surface exposes
- Which semantic token types the grammar produces

Every generated Rust file is a projection of that ontology through a Tera
template. When a new Skill frontmatter field is added to the spec, you add one
triple to the `.ttl`, run `ggen sync`, and hover docs, completion items, and
diagnostic codes all update in lockstep.

---

## The three-tier Skill loading architecture and LSP hover

Claude Code loads Skills lazily:

1. **Level 1** (~100 tokens, always present): `name` and `description` metadata.
   Claude uses these for discovery — the description text is injected into the
   system prompt and matched against the current task.
2. **Level 2** (loaded on trigger): Full `SKILL.md` content. This is the
   detailed instructions layer.
3. **Level 3+** (on demand): Resources referenced in the SKILL.md body.

This architecture explains why the LSP hover for the `description` field must
emphasise the discovery contract — if a description is vague or passive, Claude
cannot select the Skill even if it is formally correct. The LSP diagnostic
`CCC-SKILL-005` (description too long, > 1024 chars) is not just a formatting
issue: a long description pushes more tokens into the Level 1 context, raising
costs for every Claude Code session that has the Skill installed.

---

## CLI vs SDK tool permissioning and why the LSP warns

`allowed-tools` in SKILL.md frontmatter only takes effect when Skills are
invoked through the Claude Code CLI. When a Skill is accessed through the Agent
SDK (`query({ skills: ['my-skill'] })`), tool access is governed by the SDK
caller's `allowedTools` option — not by the frontmatter.

This is a common source of confusion and a real divergence in security posture:
a Skill author might believe they have restricted a Skill to `Read`-only access,
but SDK callers can override that by not passing any `allowedTools` restriction.

The LSP surfaces this with a `CCC-SKILL-006` warning on `allowed-tools:` lines:
> This field only applies in CLI mode. SDK callers control tool access through
> `allowedTools` in the query configuration.

---

## Why generated files are source

The ggen-generated `.rs` files under `src/` are first-class source. The
`GGEN-SRC-*` diagnostic family enforces this:
- Files must not be in `generated/`, `output/`, `gen/` paths
- Files must not carry `DO NOT EDIT` banners
- The fact that ggen produced a file does not demote it

The practical consequence: if a generated analyzer stub contains wrong logic,
the fix goes into either the domain ontology (if the schema changed) or the pack
template (if the code pattern was wrong). If neither, the stub is a starting
point and the implementation body is a legitimate developer contribution — ggen
provides structure, not finished logic.

---

## Receipt chain and CANDIDATE status

All capabilities in this server are currently `CANDIDATE`. A capability moves to
`ADMITTED` only when three artifacts exist:

1. **Transcript**: An LSP exchange (request + response) demonstrating the
   capability in operation, stored as a `.transcript.json` file.
2. **Negative control**: A test demonstrating that the capability correctly
   refuses a malformed or out-of-scope input.
3. **Receipt**: A `-----BEGIN RECEIPT-----` bounded record with a SHA256 digest
   chain linking the transcript and negative control artifacts.

Until all three exist, `CANDIDATE` is the honest status. Collapsing `CANDIDATE`
to `ADMITTED` without evidence is a law violation detected by anti-llm-cheat-lsp.

---

## The Van der Aalst process mining hook

The lsp-max compositor accumulates OCEL 2.0 events for every LSP interaction.
This is not observability for its own sake — it is a process conformance
mechanism. The Declare constraint model (`src/declare_model.rs`) defines the
normative process for how config surfaces should flow through the validation
pipeline:

1. `didOpen` → analyzer runs → diagnostics published
2. `didChange` → analyzer runs → diagnostics updated
3. `hover` → field looked up → docs returned

A pm4py-based conformance check against the OCEL log can detect violations like
diagnostics published without a preceding `didOpen`, or hover responses for URIs
that were never opened. These are impossible to detect from unit tests alone —
they require event log evidence against the normative process model.

---

## Why `DISTINCT` in config_surfaces.rq matters

The domain ontology can have many `ConfigSurface` individuals that share the
same `analyzerModule` — e.g., `.claude/settings.json` and `.claude/settings.local.json`
both map to the `json` analyzer module. Without `DISTINCT`, the SPARQL query
returns one row per surface, producing duplicate `pub mod json;` declarations in
`src/analyzers/mod.rs` and duplicate analyzer files attempted by the dynamic
`output_file = "src/analyzers/{{ analyzer_module }}.rs"` rule.

`DISTINCT` collapses these to one row per unique module name. The rule generates
one file per module, not one file per surface — the analyzer file itself uses
a pattern match on the URI to distinguish which surface it is validating.

---

## ConformanceVector and the Unknown invariant

`ConformanceVector` carries three sets: `admitted`, `refused`, `unknown`. The
`unknown` set is not a default or a placeholder — it is a first-class state that
means "this law axis has not been traced and we cannot claim it passes or fails."

Collapsing `unknown` into either `admitted` or `refused` is a law violation.
In practice this means:
- A capability that has never been tested is `UNKNOWN`, not `ADMITTED`
- A diagnostic code that exists in the reference but has not been verified
  against a real Claude Code session is `UNKNOWN`
- `UNKNOWN` is not a bug to be fixed by asserting a value; it is an honest
  statement of incomplete evidence that requires tracing work
