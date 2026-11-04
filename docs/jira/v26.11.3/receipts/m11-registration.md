# Receipt: M11 — compositor registered as Claude Code front door

- **Lane**: IMPLEMENTATION (M11) — repo `/Users/sac/claude-code-config-lsp`
- **Owned files touched**: `hooks/session-start.sh` (extended, idempotent),
  `.claude/settings.json` (modified via running the hook — the hook owns
  that file's lspServers section by design), this receipt (new).
- **Not touched**: `~/.claude/settings.json` (user's live global config).
- **Date**: 2026-11-03

## Status: ALIVE — hook run twice (idempotent), settings jq-valid, both
## registered binary paths confirmed executable on disk after build-release

## What landed

### 1. hooks/session-start.sh — extended, idempotent

Registers TWO entries in a fixed order. ORDER LAW (from the compositor
deployment model): first registration wins exclusive authority over its
claimed extensions — order is load-bearing, so the chosen order is
documented here and in the hook header:

1. **`lsp-max-compositor` — FIRST (exclusive authority over the routed set).**
   The compositor is the front door: one stdio session fans out
   per-extension to child servers and merges diagnostics (see
   `docs/reference/compositor-deployment.md`). Binary: release build of
   sibling repo `/Users/sac/lsp-max` `crates/lsp-max-compositor` v26.7.3,
   built this session (`cargo build --release -p lsp-max-compositor`,
   finished clean in 1m40s; 15,748,256 bytes, executable; smoke-run prints
   `no lsp-max.toml found, starting with empty router` outside a repo and
   reads the repo's `lsp-max.toml` from cwd when present — this repo's root
   has one). Registered with `args: []` so it discovers `./lsp-max.toml`
   from the session cwd.
2. **`claude-code-config-lsp` — SECOND (fallback).** Same extension map but
   receives nothing from the routed set while the compositor entry is
   first. Direct entry is never deleted.

**Routed set**: `.json` and `.md` — read from this repo's `lsp-max.toml`
(`claude-code-config-lsp`, `primary_extensions = [".json", ".md"]`,
`priority = "diagnostics-only"`).

**Binary-absent fallback law**: if the compositor binary is not executable
on disk, the hook removes any stale compositor entry so the direct entry
regains sole authority — settings never reference a missing binary. If
present but stale (wrong position or wrong `extensionToLanguage`), repaired
in place. Canonical map: `.json→claude-config-json`,
`.md→claude-config-markdown` (matches `src/file_types.rs` authority).

### 2. `.claude/settings.json` (repo)

Registered by running the hook. Final `lspServers` state (jq-valid):

```json
[
  {
    "name": "lsp-max-compositor",
    "command": "/Users/sac/lsp-max/target/release/lsp-max-compositor",
    "args": [],
    "extensionToLanguage": {
      ".json": "claude-config-json",
      ".md": "claude-config-markdown"
    }
  },
  {
    "name": "claude-code-config-lsp",
    "command": "claude-code-config-lsp",
    "args": ["serve", "--stdio"],
    "extensionToLanguage": {
      ".json": "claude-config-json",
      ".md": "claude-config-markdown"
    },
    "startupTimeout": 5000
  }
]
```

### 3. DEPLOY step — user's global migration (NOT performed)

The live global registration surface is **not** `~/.claude/settings.json`
(read-only check this session: that file has no `lspServers` key). It is
the plugin cache manifest
`~/.claude/plugins/cache/claude-code-config-lsp/claude-code-config-lsp/0.1.0/.claude-plugin/plugin.json`,
whose `lspServers["claude-code-config"].command` is the bare
`claude-code-config-lsp` resolved via PATH. One-line swap to the compositor
front door, left to the user as a deployment decision (note: the cache
manifest is regenerated on plugin reinstall, so re-apply after updates, or
promote the change to the repo's `.claude-plugin/plugin.json` source):

```bash
P="$HOME/.claude/plugins/cache/claude-code-config-lsp/claude-code-config-lsp/0.1.0/.claude-plugin/plugin.json"
jq --arg c "/Users/sac/lsp-max/target/release/lsp-max-compositor" \
   '.lspServers["claude-code-config"].command = $c' "$P" > "$P.tmp" \
   && mv "$P.tmp" "$P"
```

## Verification transcript (2026-11-03, this session)

1. **Hook run 1** (repo settings had the direct entry, pre-existing from C10):
   ```
   session-start.sh: claude-code-config-lsp already registered in /Users/sac/claude-code-config-lsp/.claude/settings.json
   session-start.sh: registered/refreshed lsp-max-compositor as first entry (exclusive authority) in /Users/sac/claude-code-config-lsp/.claude/settings.json
   ```
2. **Hook run 2** (idempotency):
   ```
   session-start.sh: claude-code-config-lsp already registered in /Users/sac/claude-code-config-lsp/.claude/settings.json
   session-start.sh: lsp-max-compositor already registered first in /Users/sac/claude-code-config-lsp/.claude/settings.json
   ```
3. **jq validation**: `jq empty .claude/settings.json` → exit 0 (`JQ_VALID`);
   `jq '.lspServers'` shows compositor at index 0, direct at index 1.
4. **Absent-binary fallback branch** (temp copy of hook + settings, path
   sed'd to `/nonexistent/lsp-max-compositor`): run 1 removed the stale
   compositor entry (`compositor binary missing — removed lsp-max-compositor
   entry; claude-code-config-lsp is sole authority`), run 2 was a no-op
   (`... direct entry remains sole authority`); resulting `lspServers`
   names: `["claude-code-config-lsp"]` only.
5. **build-release gate**: `./scripts/build-release.sh` →
   `== DONE: /Users/sac/claude-code-config-lsp/dist/claude-code-config-lsp-26.6.24 (+ .sha256); plugin cache refreshed ==`
   (cargo build --release + clippy `-D warnings` both finished clean).
6. **Registered paths exist on disk**:
   - `/Users/sac/lsp-max/target/release/lsp-max-compositor` → EXISTS (executable, 15,748,256 bytes)
   - `claude-code-config-lsp` → EXISTS on PATH (`/Users/sac/.local/bin/claude-code-config-lsp`)
   - dist artifact `dist/claude-code-config-lsp-26.6.24` + `.sha256` written by the gate.

## Standing

ALIVE for the repo-local registration (both branches of the hook exercised
for real this session, both target binaries observed executable). The
global DEPLOY step is UNVERIFIED by design — it is the user's one-line
change and was deliberately not performed.
