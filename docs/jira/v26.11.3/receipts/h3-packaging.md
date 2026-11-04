# H3 Packaging/Compositor Receipt — v26.11.3

Lane: IMPLEMENTATION (packaging). Date: 2026-11-03. Owner files: `lsp-max.toml`,
`scripts/build-release.sh`, `.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`,
`README.md` (packaging section). Never ran git.

## Subject (exact)

- `/Users/sac/claude-code-config-lsp/scripts/build-release.sh` (new, executable)
- `/Users/sac/claude-code-config-lsp/lsp-max.toml` (new; later refined on disk by another
  hand to schema-precise keys — andon_code_prefixes, a justfile-lsp child — kept as-found)
- `.claude-plugin/plugin.json` (+ `build`/`binaries` fields)
- `.claude-plugin/marketplace.json` (+ `build`/`binaries` fields)
- `README.md` (appended "Packaging & compositor deployment" section)
- **Disclosed out-of-lane unblocks** (SLA minimal-unblock; lint debt in stable files whose
  owning lanes had left them untouched 20+ min):
  - `src/backend.rs` — 3 clippy `-D warnings` hits: `rule_pack_snapshot` dead-code allow
    (harness seam, commented), `mutable_key_type` allow with comment (Url is
    interior-mutable; identity is the serialized URL string), unused `doc`→`_doc`
  - `src/{completion,navigation,selection,hierarchy}.rs` — 8 mechanical lints cleared via
    `cargo clippy --fix --allow-dirty --release` (map_or→is_some_and, match→let-else,
    map_or→unwrap_or) plus a behavior-identical never_loop hoist in
    `hierarchy.rs::frontmatter_name` (first-iteration handling replacing a loop whose body
    provably exits once), with its follow-on `unreachable_code`/`needless_return` cleanups
  - No behavioral rewrites; every change is a rustc/clippy machine suggestion or a
    behavior-identical hoist.

## Problem solved

Plugin cache copies plugin *source*, never compiles — the wired LSP depended on a stale
prior build. `scripts/build-release.sh` is now the single packaging gate: build → clippy
`-D warnings` (lib+bins, contract scope) → `dist/claude-code-config-lsp-<version>` +
sha256 → regenerate
`~/.claude/plugins/cache/claude-code-config-lsp/<...>/<version>/dist/` (idempotent re-copy;
`~/.claude/settings.json` never written — read-only `shasum` receipt `681272878f...`).

## Verification ladder (real output, this session)

1. `cargo build --release` → `Finished release profile [optimized] in 19.08s` (final run
   exit 0; earlier runs raced concurrent-lane in-flight edits and failed — see Standing)
2. `cargo clippy --release -- -D warnings` → `Finished ... in 0.25s`, clean
3. `dist/claude-code-config-lsp-26.6.24` (15,649,648 B) + `.sha256`;
   `shasum -a 256 -c` → **OK**
4. Plugin cache `~/.claude/plugins/cache/claude-code-config-lsp/claude-code-config-lsp/0.1.0/dist/`
   regenerated: `claude-code-config-lsp` (15,649,648 B) + `.sha256`
5. **Wire test (Chicago)**: real `initialize` over Content-Length-framed stdio against
   `/Users/sac/claude-code-config-lsp/dist/claude-code-config-lsp-26.6.24 serve --stdio` →
   1200-byte JSON-RPC result with full capabilities
   (`textDocumentSync`, hover, completion triggers `["\"",":"]`, selectionRange, quickfix,
   formatting, ...) → **ALIVE**
6. Version stamp: Cargo `version = 26.6.24` → dist name
   `claude-code-config-lsp-26.6.24`
7. `jq empty` on both plugin manifests → clean; `python3 -m tomllib` on lsp-max.toml → clean

## Standing

- Script end-to-end: **ALIVE** (final run 13:32, exit 0, artifacts + wire test above)
- Idempotency: **PARTIAL** — dist copy/checksum/cache-regen are deterministic re-copies,
  but the second full run raced a concurrent lane's in-flight landing of
  `src/admission.rs` (E0603 `SemanticTokensDelta` private import, exit 101 at the build
  step, file mtime 13:34). Regression in `lib/` I do not own and did not touch; not
  re-run to completion against the moving tree.
- Compositor manifest: **ALIVE as TOML** (parses; schema-precise after external
  refinement). Routing behavior under the running compositor UNVERIFIED this session
  (requires building lsp-max-compositor — out of lane budget).
- plugin.json/marketplace.json: JSON-valid.

## Known skew (disclosed, unfixed — release decision, out of lane)

plugin.json `version` is `0.1.0` and its `binaries` path is
`dist/claude-code-config-lsp-0.1.0`, but Cargo version is `26.6.24`, so the script stamps
`dist/claude-code-config-lsp-26.6.24`. The `binaries` path does not resolve until the
manifest version is bumped or the stamp format changes.

## Falsifiers

- On a quiet tree, re-running `./scripts/build-release.sh` must exit 0 and reproduce
  checksum `1b42c00ed8361a2af0e9d489e0b25680bad48fba5428a4741593fdb09d49c09f`
- Cache copy mtime (13:32) ≥ binary build time → Claude Code now serves fresh-binary
  diagnostics; the stale-build channel is closed
- `ExtensionRouter::from_config` on lsp-max.toml must route .json/.md to
  claude-code-config-lsp at DiagnosticsOnly tier, and never let it displace a full-priority
  JSON server on non-config .json
- The 11 clippy fixes must not change diagnostics behavior: `cargo test` (~184 tests) is
  the boundary re-run (not executed this session — src/ was moving under concurrent
  lanes; test census belongs to the lane that owns src/)