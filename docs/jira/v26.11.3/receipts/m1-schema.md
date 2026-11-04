# M1 — Compositor Manifest Schema Extraction + Manifest Fix

Lane: M1 (read-then-edit). Date: 2026-11-03.
Subject: `/Users/sac/claude-code-config-lsp/lsp-max.toml` (schema authority: `/Users/sac/lsp-max/crates/lsp-max-compositor/src/config.rs`, `registry.rs`, `registry_init.rs`).

## 1. Extracted schema (config.rs, real source)

`CompositorConfig { server: Vec<ServerEntry> }` — top-level key `[[server]]`.

| key | type | required | notes |
|---|---|---| `serde::Deserialize` |
|---|---|---|---|
| `id` | String | yes | child identity; per-server ANDON map key |
| `primary_extensions` | Vec<String> | yes | no serde default — key must be present even when empty |
| `secondary_extensions` | Vec<String> | yes | same; empty `[]` legal |
| `priority` | String | yes | `"full"`/`"semantic"` → Primary; `"secondary"` → Secondary; `"lsif"` → Lsif; anything else → DiagnosticsOnly (`ChildTier::from_priority`) |
| `andon_code_prefixes` | Option<Vec<String>> | no | per-server C_D routing (`per_server_andon_prefixes`); omitted → defaults `["WASM4PM-","ANTI-LLM-","GGEN-"]` |
| `command` | Option<String> | no | `None` = not auto-spawned; probe runs `<cmd> --version`, any spawn status = reachable |
| `args` | Option<Vec<String>> | no | default `["serve", "--stdio"]` (`effective_args`) |

- No `name` key, no `matchers` key. Unknown keys are silently ignored by serde (no
  `deny_unknown_fields`) — an off-schema key never errors, so keep to the 8 schema keys.
- Routing semantics: extension-keyed suffix match (`servers_for_filename`; dotted keys
  `.json`, compound `.ocel.json` both match; bare keys normalised). Multiple servers per
  extension supported and deduplicated by id. Diagnostics = FanAll, requests = FirstSuccess
  (Primary only), notifications = fan, unknown methods → PrimaryOnly.
- Generated registry shape: `ExtensionRouter::new_from_manifest()` (registry_init.rs, ggen
  Tera-generated from the repo's own lsp-max.toml; `from_config` is the runtime equivalent).

## 2. Diff: H3-authored manifest vs real schema

| field | H3 authored | real schema | verdict |
|---|---| fixes |
|---|---|---|---|
| `id` | present | required String | PASS |
| `primary_extensions` [".json",".md"] | present | required | PASS |
| `secondary_extensions = []` | present | required | PASS (required key, correctly present) |
| `priority = "diagnostics-only"` | present | valid | PASS |
| `command = "claude-code-config-lsp"` | present | Option<String>, PATH lookup | **FIXED** |
| `args = ["serve","--stdio"]` | present | valid verb route (src/main.rs) | PASS |
| `andon_code_prefixes` | absent | optional | ADDED `"CCC-"` |

Key content fixes:

1. **`command` fixed to absolute path**: PATH lookup found a stale Jul-28 binary at
   `~/.local/bin/claude-code-config-lsp` (sha `e92834f3…`) vs H3's build-release output
   (sha `1b42c00e…`, identical to `dist/claude-code-config-lsp-26.6.24`). The PATH copy
   would spawn a stale binary. Manifest now pins
   `/Users/sac/claude-code-config-lsp/target/release/claude-code-config-lsp` (the
   build-release.sh output — unversioned path, survives version bumps).
2. **`andon_code_prefixes = ["CCC-"]` added** — config-lsp emits `CCC-*` codes; without it the
   server inherits the WASM4PM-/ANTI-LLM-/GGEN- defaults that never match its diagnostics.
3. **Second child added**: `justfile-lsp` (just a `.just` router, diagnostics-only, `JUST-`
   prefixes). Schema supports N children (`server: Vec<ServerEntry>`; registry dedup test
   proves multi-server-per-extension). General-JSON-to-a-second-child is NOT expressible as
   a distinct entry here honestly because no second general-JSON LSP binary exists on this
   machine — `justfile-lsp` is the only other real child available, and it claims `.just`,
   not `.json`. `justfile-lsp` was built this session:
   `cargo build --release` in `/Users/sac/lsp-max/examples/justfile-lsp` →
   `/Users/sac/lsp-max/target/release/justfile-lsp` (2m25s, success).
4. Multiple-children-per-extension note: routing to a *second* `.json` server would be
   expressible (FanAll merges diagnostics from both), but no such binary exists here — recorded honestly rather than fabricated.

## 3. Validation (the gate)

- `cargo test -p lsp-max-compositor`: **BLOCKED pre-existing** — `tests/andon_prefixes.rs`
  (and others) contain mid-flight placeholder code (`-seq_placeholder_from_config`,
  `expected_placeholder`), compile errors E0599/E0277. Not session-introduced; M3–M6 lanes
  own compositor tests. Not touched.
- **Real parse gate (passed)**: scratch binary at `/tmp/m1-manifest-parse` calls the crate's
  own `CompositorConfig::from_toml_file` on the manifest (built under the pinned toolchain
  `nightly-2026-04-15`; system toolchain fails E0554 on `wasm4pm-compat` — pre-existing).
  Output: `PARSE OK: 2 server entries`, tiers map to DiagnosticsOnly, routes resolve:
  `settings.json`/`CLAUDE.md` → claude-code-config-lsp; `recipe.just` → justfile-lsp.
  Exit 0. Re-run after the command fix: PARSE OK again.

## 4. Standing

- Manifest: schema-exact, real-parser-validated. ALIVE (parse gate ran+passed this session).
- `~/.local/bin/claude-code-config-lsp` stale-binary finding: OPEN (PATH copy is Jul 28,
  sha e92834f3; current build is Nov 3, sha 1b42c00e). Suggest re-running
  scripts/build-release.sh or refreshing the PATH copy — outside this lane's file ownership.
- justfile-lsp panics on bare `--version` (serve().unwrap() on empty stdin,
  examples/justfile-lsp/src/main.rs:138) — harmless to `probe()` (any spawn status counts
  as reachable) and to real stdio serving, recorded for the ledger.

result: lsp-max.toml manifest schema extracted from compositor source, H3's manifest fixed
to schema-exact (absolute command path, CCC- andon prefixes, real second child justfile-lsp
on .just), validated by the compositor's own from_toml_file parser (PARSE OK, exit 0).
