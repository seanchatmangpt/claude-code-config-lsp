# claude-code-config-lsp

A Language Server Protocol (LSP 3.18) server for Claude Code configuration
files — real-time diagnostics, hover documentation, completion, semantic
tokens, and a subset of quick fixes for your `.claude/` setup.

Generated from an RDF ontology (`schema/claude-code-config.ttl`) via the
`ggen` `lsp-max` pack; see CLAUDE.md for the generation pipeline.

## Surfaces covered

Five config "law surfaces" plus related files
(CLAUDE.md "Overview" / "Architecture"):

| Surface | Files | Language ID |
|---|---|---|
| Settings | `**/.claude/settings*.json` | `claude-config-json` |
| Marketplace | `**/marketplace.json` | `claude-config-json` |
| Plugins | `**/plugin.json`, `**/mcp.json`, `**/keybindings.json` | `claude-config-json` |
| Agents | `**/.claude/agents/*.md`, `**/AGENTS.md` | `claude-config-markdown` |
| Skills | `**/.claude/skills/*/SKILL.md` | `claude-config-markdown` |
| Other | `**/CLAUDE.md`, `**/.claude/hooks/*.sh`, `**/.claude/lsp-max-auto.toml` | per pattern |

Language IDs are the runtime truth from
`src/file_types.rs` (`SUPPORTED_FILE_TYPES`); `hooks/session-start.sh`
must stay aligned with it (2026-11-03: hook was registering
`claude-code-config` — aligned to `claude-config-json` /
`claude-config-markdown`).

## Build

```bash
cargo build
```

(CLAUDE.md "Build, Test, Run".)

## Run

The server speaks JSON-RPC/LSP over stdio. A bare (arg-less) invocation
is the server — `src/main.rs` defaults a no-args launch to the stdio
server so editors and Claude Code can spawn the binary directly
(`src/main.rs:10-20`, `src/nouns/config.rs::start_stdio_server`).

```bash
cargo run
```

## CLI verbs

Dispatched via `clap-noun-verb` (`src/main.rs`, `src/nouns/config.rs`):

| Verb | Purpose |
|---|---|
| `serve --stdio` | Start the LSP server over stdio (exits 1 without `--stdio`; `config.rs::serve`) |
| `scan [path]` | Claude Code config diagnostics for a path (defaults `.`, `config.rs::scan`) |
| `export [path]` | Dump the config tree as JSON (`config.rs::export`) |
| `unused [path]` | Dead-config audit — present but unused config (`config.rs::unused`) |
| `conformance [path]` | Workspace conformance scorecard (`config.rs::conformance`) |
| `fix [path] [--apply]` | Auto-fix repairable issues; dry-run by default (`config.rs::fix`) |
| `chain [path]` | Receipt chain from genesis to latest (`config.rs::chain`) |
| `check [path]` | Verify receipt chain integrity (prints "Verification succeeded..." on valid; `config.rs::check`) |

## Install (`.claude/settings.json` `lspServers`)

`hooks/session-start.sh` idempotently registers the binary in
`.claude/settings.json`'s `lspServers` array (jq-based; refuses to
modify invalid JSON). Run it once, or let your SessionStart hook invoke
it. Resulting entry shape:

```json
{
  "name": "claude-code-config-lsp",
  "command": "claude-code-config-lsp",
  "args": [],
  "extensionToLanguage": {
    ".json": "claude-config-json",
    ".md": "claude-config-markdown"
  }
}
```

`.json` → `claude-config-json`, `.md` → `claude-config-markdown`
(source: `hooks/session-start.sh`, aligned to `src/file_types.rs`).

## Dogfood

`scripts/dogfood.mjs` drives the real compiled binary over the actual
Content-Length-framed JSON-RPC wire protocol against this repo's own
config fixtures (CLAUDE.md "Testing"). Run:

```bash
cargo build && node scripts/dogfood.mjs
# or: cargo build && node scripts/dogfood.mjs 2>&1 | tee /tmp/dogfood.log
```

## Test

```bash
cargo test
```

~184 tests across `tests/chicago_tdd.rs`,
`tests/chicago_tdd_inventory.rs`, and inline `#[cfg(test)]` modules
(CLAUDE.md "Testing").

## Standing notes

- **FM-GEN-007**: `ggen sync` fails on this repo with
  `[FM-GEN-007] TemplateSource::Pack (pack "lsp-max") is not implemented yet`
  — a ggen core bug, reproduced against a throwaway pack. Ontology
  regeneration is blocked until fixed.
- **capabilities.rs hand-extended**: pending that fix,
  `src/capabilities.rs` is hand-edited as a workaround (CLAUDE.md,
  "Known limitation").

## Packaging & compositor deployment

`scripts/build-release.sh` is the single packaging gate. It solves the
plugin-cache binary problem — the Claude plugin cache copies plugin
*source* only and never compiles, so the wired LSP was depending on a
stale prior build. The script: `cargo build --release`, a fail-closed
`cargo clippy --release --all-targets -- -D warnings` gate, a
version-stamped binary + sha256 checksum into `dist/`, and — if the
plugin cache dir exists — regenerates the binary copy under
`~/.claude/plugins/cache/claude-code-config-lsp/<...>/<version>/dist/`.
`~/.claude/settings.json` is never touched.

```bash
./scripts/build-release.sh
```

### Compositor deployment (lsp-max)

`lsp-max.toml` at the repo root declares this server as a child of the
lsp-max compositor: the compositor fronts one stdio session, fans out
per extension, and merges diagnostics. Fleet evidence: a single
front-door stdio session fans out per-extension child servers and merges
diagnostics, so this server contributes config diagnostics without
displacing general JSON LSPs.

The manifest routes `.json` and `.md` to `claude-code-config-lsp` at
`priority = "diagnostics-only"`, so a full-service JSON language server
keeps primary service on non-config `.json` while this server adds
config-surface diagnostics on top:

```toml
[[server]]
id = "claude-code-config-lsp"
primary_extensions = [".json", ".md"]
secondary_extensions = []
priority = "diagnostics-only"
command = "claude-code-config-lsp"
args = ["serve", "--stdio"]
```

Full deployment reference — architecture, every `[[server]]` manifest key,
the first-wins registration law, priority/tier semantics, child prebuilt
binary requirements, and crash/restart behavior — lives in
[`docs/reference/compositor-deployment.md`](docs/reference/compositor-deployment.md).

## P/H wave lanes (v26.11.3)

Appended at wave close 2026-11-03. Full record:
[`docs/jira/v26.11.3/closure/ph-wave-close.md`](docs/jira/v26.11.3/closure/ph-wave-close.md);
receipts in `docs/jira/v26.11.3/receipts/`. Standing vocabulary: ALIVE =
ran this session with command + exit; PARTIAL = green at landing with a
named post-landing defect.

| Lane | Surface | Standing | Gate |
|---|---|---|---|
| P1 | navigation | ALIVE | navigation 22/0 |
| P2 | workspace symbols | ALIVE | workspace_symbols 5/0 |
| P3 | rename + inlay hints | ALIVE | rename_inlay 13/0 |
| P4 | code lens + signature help | ALIVE | lens_signature 7/0 |
| P5 | formatting | ALIVE | formatting 15/0 |
| P6 | selection + highlight | ALIVE | selection 15/0 |
| P7 | hierarchy + moniker | ALIVE | hierarchy_moniker 13/0 |
| P8 | semantic tokens delta + color | ALIVE | tokens_delta 5/0, tokens_color 11/0 |
| P9 | commands analyzer | ALIVE | commands_analyzer 5/0 |
| P10 | TTL reconciliation court | PARTIAL (G1 anchor rot) | ttl_reconciliation 10/1 |
| P12 | provider wiring (12 modules) | ALIVE | provider_registry 13/0, all targets green |
| H1 | harness docs (virtual docs) | ALIVE | harness_docs 11/0 |
| H2 | harness ops (9-op matrix) | ALIVE | harness_ops 25/0 |
| H3 | packaging (build-release) | ALIVE | build-release.sh exit 0 |
| H4 | standing vocabulary | ALIVE | standing 10/0 |
| H5 | settings overlay | ALIVE | overlay 9/0 |
| X2 | lsp-max pin 26.11.3 (path dep) | ALIVE | build + dogfood + dry-run per receipt |

Wave verdict: ALIVE with 2 named residuals — G1 (P10 court anchor rot,
locator-only, semantics intact) and G2 (dogfood allow-list gap, the F4 flip
consequence: publishDiagnostics now reaches the compositor client).
