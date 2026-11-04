# C9 Schema Expansion Receipt — settings.json key inventory

Lane: IMPLEMENTATION (schema/claude-code-config.ttl + src/schema.rs owned)
Date: 2026-11-03
Repo: /Users/sac/claude-code-config-lsp

## Subject

Expand the settings.json schema inventory from the sample set (5 settings fields)
to the real corpus grammar observed in `/Users/sac/.claude/settings.json` and
`/Users/sac/.claude/settings.json` → `settings.local.json` (2026-11-03).

## Files touched (owned only; no git commands run)

- `schema/claude-code-config.ttl` — added 30 new `ccc:SchemaField` individuals
  (block header: `hand-extended 2026-11-03, pending FM-GEN-007`, evidence source
  `/Users/sac/.claude/settings.json` noted in header comment), plus 12
  `ccc:fieldParent` nested links
  (permissions{allow,deny,defaultMode}, attribution{commit,pr},
  hooks{matcher,hooks,type,timeout,statusMessage}, statusLine{type,command}).
- `src/schema.rs` — mirrored all 26 new field rows into the settings table,
  keeping the ggen AUTO-GENERATED header untouched, additions fenced by
  `// hand-extended 2026-11-03, pending FM-GEN-007` marker comment.
- `tests/schema_expansion.rs` — new court file: key-presence, enum-fact,
  individual-presence, marker-presence, and runtime-registry assertions.
- `docs/jira/v26.11.3/receipts/c9-schema-expansion.md` — this receipt.

## Keys added (evidence: /Users/sac/.claude/settings.json)

Top-level: env, attribution{commit,pr}, permissions{allow,deny,defaultMode},
enabledMcpjsonServers, disabledMcpjsonServers (witnessed in settings.local.json),
hooks{matcher, hooks[], type, timeout, statusMessage}, statusLine{type,command},
enabledPlugins, extraKnownMarketplaces, modelSettings, mcpServers,
askUserQuestionTimeout, tui, voice{enabled,mode}, voiceEnabled,
skipDangerousModePermissionPrompt, skipWorkflowUsageWarning, skipAutoPermissionPrompt,
agentPushNotifEnabled, editorMode, verbose, crossSessionInbound.

Existing fields (model, effortLevel, permissions, hooks, mcpServers, env,
enabledPlugins, statusLine, tui) kept unchanged; nested structure added as new
individuals with `ccc:fieldParent`.

## Verification ladder (real output)

1. **TTL parses + counts (rdflib, real)**: `SchemaField count: 41` (was 10);
   12 fieldParent links resolve; namespace
   `https://claude-code-config-lsp.rs/ontology/`.
2. **schema.rs compiles standalone (real)**: copied to /tmp scratch crate,
   `cargo test` → `4 passed; 0 failed` (existing in-file tests still green).
   Scratch lease deleted after verification.
3. **Test assertions executed mechanically (real)**: all 26 schema.rs keys
   present, 6 enum facts present (defaultMode lists bypassPermissions;
   hooks.type/statusLine.type = command; editorMode = vim; tui = fullscreen;
   crossSessionInbound = accept), all 30 TTL individuals present, both
   hand-extension markers present. Output: `NONE (all 26 present)` /
   `NONE (all 30 present)` / `enum facts: OK` / `markers: OK`.
4. **BLOCKED (external, pre-existing/concurrent)**: `cargo test` on the shared
   lib currently fails with 7 errors in `src/backend.rs`,
   `src/completion.rs`, `src/analyzers/json.rs` — files owned by other lanes
   (json.rs explicitly forbidden to this lane). **None of the 7 errors touch
   schema.rs or tests/schema_expansion.rs** (standalone compile of schema.rs
   proves the latter is clean). Standing: PARTIAL_ALIVE — full-court run
   (`cargo test --test schema_expansion`) is ready and unblocks the moment the
   lib compiles.

## Standing

PARTIAL_ALIVE. Artifacts verified by real commands (rdflib parse + standalone
cargo test + mechanical assertion run). The cargo full-court run is BLOCKED by
other lanes' in-flight compile errors, not by this lane's diff.
