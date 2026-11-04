# Compositor deployment guide

Deployment reference for running `claude-code-config-lsp` as a child of the
`lsp-max` compositor (`crates/lsp-max-compositor`, sibling repo
`/Users/sac/lsp-max`). Every schema and routing claim in this guide is read
from the compositor's real source this session (`src/config.rs`,
`src/registry.rs`, `src/child_process.rs`, compositor `CLAUDE.md`) — not
from prose or memory.

## Why a compositor is necessary

Claude Code enforces **one-server-per-extension exclusivity**: a single
server binding wins an extension, so two servers cannot both claim `.json`
without one displacing the other. The compositor removes the contradiction
by fronting **one** stdio session and fanning out per extension to N child
servers, then merging their diagnostics back over that one session:

```
Claude Code (one server slot per extension)
    |  stdio (LSP over Content-Length-framed JSON-RPC)
    v
lsp-max compositor (front door)
    |  ExtensionRouter routes per file extension
    +--> child: claude-code-config-lsp  (.json, .md -- diagnostics-only)
    +--> child: general JSON LSP        (.json -- priority full/semantic)
    +--> child: markdown LSP            (.md  -- priority full/secondary)
    |
    v
merged diagnostics published back over the single stdio session
```

Without the compositor, registering `claude-code-config-lsp` on `.json`
displaces the general JSON language server (or is displaced — registration
order decides). The `diagnostics-only` declaration is what lets both servers
contribute on the same extension.

## Manifest reference (lsp-max.toml)

The compositor finds `lsp-max.toml` by walking from the current directory
upward (`CompositorConfig::load()`, config.rs:96-112), stopping with `None`
at a directory whose `Cargo.toml` contains `[workspace]` but no manifest was
found. An optional auto-discovery manifest `.claude/lsp-max-auto.toml` is
located by the same upward walk (`find_auto_config`, config.rs:142-158) and
merged with the static one (`load_with_auto`, config.rs:117-128).

Top level: a `server` array of `[[server]]` tables. Each entry deserializes
as `ServerEntry` (config.rs:7-17) with exactly these keys:

| Key | Type | Required | Semantics |
|---|---|---|---|
| `id` | string | yes | Unique server identity; dedup key and per-server ANDON-prefix key |
| `primary_extensions` | string[] | yes | Extensions claimed for primary service |
| `secondary_extensions` | string[] | yes | Extensions covered on top of a primary |
| `priority` | string | yes | Tier selector — see semantics table below |
| `andon_code_prefixes` | string[] | no | Per-server law-axis prefixes; see note below |
| `command` | string | no | Binary to spawn. Absent = declared but NOT auto-spawned |
| `args` | string[] | no | Spawn args; default `["serve", "--stdio"]` (`effective_args`) |

Defaults note: servers without `andon_code_prefixes` fall back to the
legacy defaults `["WASM4PM-", "ANTI-LLM-", "GGEN-"]` (config.rs:40).

The repo's own manifest (`/Users/sac/claude-code-config-lsp/lsp-max.toml`,
read this session) is a working example:

```toml
[[server]]
id = "claude-code-config-lsp"
primary_extensions = [".json", ".md"]
secondary_extensions = []
priority = "diagnostics-only"
command = "claude-code-config-lsp"
args = ["serve", "--stdio"]
```

### Registration order law: first-wins exclusive authority

The compositor loads two manifest sources: the static `lsp-max.toml` and the
auto-discovered `.claude/lsp-max-auto.toml`. Entries are merged by `id` in
`CompositorConfig::merge()` (config.rs:132-140): any auto-discovered entry
whose `id` already exists in the static set is silently dropped — the first
occurrence wins and is never overridden. Static configuration therefore
holds exclusive authority over its server IDs; auto-discovery can only add
new servers, never replace or reconfigure existing ones.

Discovery is also first-wins: the upward walk binds the nearest
`lsp-max.toml`, and a workspace root without a manifest stops the walk, so a
nested manifest cannot be silently overridden by an ancestor's.

### Priority semantics

`priority` maps to a `ChildTier` (`src/registry.rs:26-34`, read this
session):

| `priority` value | ChildTier | Behavior |
|---|---|---|
| `"full"` or `"semantic"` | `Primary` | Wins hover/completion/definition via FirstSuccess routing |
| `"secondary"` | `Secondary` | Adds coverage where no primary responds |
| `"lsif"` | `Lsif` | LSIF-index surface |
| anything else | `DiagnosticsOnly` | Diagnostics only; excluded from capability merge |

Under `"diagnostics-only"` the tier applies to BOTH the entry's
`primary_extensions` and `secondary_extensions` (registry.rs:111-121) — the
server never escalates to Primary regardless of which list its extension
appears in.

Method routing (compositor `CLAUDE.md`, read this session):

```
textDocument/hover | completion | definition   -> FirstSuccess (Primary tier only)
textDocument/publishDiagnostics                -> FanAll (all tiers; REFUSED_BY_LAW survives dedup)
textDocument/didOpen | didChange | didClose    -> Notify (fan all, no response expected)
unknown methods                                -> PrimaryOnly
```

This is why a diagnostics-only child and a full-service JSON LSP coexist on
`.json`: diagnostics fan to every tier, interactive requests go to the
primary.

### Child requirements: prebuilt binaries

The Claude plugin cache copies plugin *source* only and never compiles —
this is the cache-copies-source-not-binaries law. A child `command` that
points at a source tree produces a stale or missing binary at spawn time,
because nothing in the cache path runs `cargo build`. Every compositor
child must therefore ship a **prebuilt binary**, and the manifest `command`
must resolve to it on `PATH` or as an absolute path.

In this repo the packaging gate is `scripts/build-release.sh` (README,
"Packaging & compositor deployment", read this session): `cargo build
--release`, a fail-closed `cargo clippy --release --all-targets -- -D
warnings` gate, a version-stamped binary + sha256 checksum into `dist/`,
and — if the plugin cache dir exists — regeneration of the binary copy
under `~/.claude/plugins/cache/claude-code-config-lsp/<...>/<version>/dist/`.
`~/.claude/settings.json` is never touched. Run it after any source change
that should reach the compositor child:

```bash
./scripts/build-release.sh
```

Spawn probe: `ServerEntry::probe()` (config.rs:29-37) tests reachability by
attempting to spawn the command with `--version`; a successful spawn counts
as reachable even if the process exits immediately (CC-002). A child whose
binary is not prebuilt fails here, at registration time — loud, not silent.

## Crash and restart behavior

Honest status: **there is no automatic child restart today.**
`ChildProcess::spawn` (child_process.rs:22-83, read this session) returns a
`(ChildProcess, exit_future)` pair — the exit future resolves when the child
exits (crash or clean exit), and the doc comment instructs callers to
`tokio::spawn` that future and drive cleanup themselves. The convenience
path `ChildProcessPool::spawn_and_register` (child_process.rs:121-131)
explicitly **drops the exit future** ("callers that need exit detection use
spawn() directly"). Consequences for operators:

- A child crash is observable (exit watcher) but not self-healing; the
  compositor does not currently respawn.
- If you need crash detection, wire the exit future from `spawn()`, not
  `spawn_and_register()`.
- Restart-on-crash is an open compositor item, not a deployed behavior —
  do not document it as working until it lands in lsp-max.

## Claude Code constraints that make the compositor necessary

1. **One server per extension.** Claude Code binds exactly one LSP server
   per file extension; a second registration on `.json` displaces the
   first. Any multi-server story on a shared extension must go through one
   front door.
2. **Plugin cache does not build.** The cache-copies-source-not-binaries
   law (above) means Claude Code will never compile a child server; only
   prebuilt binaries deployed via `scripts/build-release.sh` are viable.
3. **One stdio session.** Claude Code speaks LSP over a single stdio
   transport per server binding; there is no multiplexed multi-server
   transport. The compositor's fan-out is internal to that one session.

## Source receipts for this guide

All line references verified by direct read this session (2026-11-03):

- `/Users/sac/lsp-max/crates/lsp-max-compositor/src/config.rs` (159 lines)
  — manifest schema, load/merge/probe.
- `/Users/sac/lsp-max/crates/lsp-max-compositor/src/registry.rs` —
  ChildTier enum and priority-string mapping.
- `/Users/sac/lsp-max/crates/lsp-max-compositor/src/child_process.rs` —
  spawn/exit-watcher semantics.
- `/Users/sac/lsp-max/crates/lsp-max-compositor/CLAUDE.md` — routing table,
  module map, law status.
- `/Users/sac/claude-code-config-lsp/lsp-max.toml` — this repo's deployed
  manifest.
- `/Users/sac/claude-code-config-lsp/README.md` lines 117-155 — packaging
  gate and landed compositor section.
