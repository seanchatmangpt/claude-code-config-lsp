# M13 — compositor documentation receipt

Lane: M13 IMPLEMENTATION (docs). Repo: /Users/sac/claude-code-config-lsp
(shared canonical checkout; no git run by this lane, per contract).

## Deliverables (all on disk, verified by wc/tail this session)

- `docs/reference/compositor-deployment.md` — NEW, 189 lines. Full
  deployment guide: text architecture diagram (Claude Code stdio ->
  compositor front-door -> per-extension child fan-out -> merged
  diagnostics), complete `[[server]]` manifest reference (all 7 keys of
  `ServerEntry`), the first-wins registration order law, priority/tier
  semantics with the method routing table, child prebuilt-binary
  requirements (cache-copies-source-not-binaries law +
  `scripts/build-release.sh` gate), crash/restart behavior, and the three
  Claude Code constraints that make a compositor necessary. Closes with a
  source-receipts section listing every file cited.
- `README.md` — appended 5-line deployment pointer (lines 156-160) after
  the C10-landed compositor section; no existing section altered.
- `docs/jira/v26.11.3/receipts/m13-docs.md` — this receipt.

## Grounding (files read this session, 2026-11-03)

- `/Users/sac/lsp-max/crates/lsp-max-compositor/src/config.rs` (159 lines,
  full read) — authoritative manifest schema: `CompositorConfig`,
  `ServerEntry` (id, primary_extensions, secondary_extensions, priority,
  andon_code_prefixes, command, args), upward-walk discovery, first-wins
  `merge()`, `probe()` (CC-002), `effective_args()` default
  `["serve", "--stdio"]`.
- `/Users/sac/lsp-max/crates/lsp-max-compositor/src/registry.rs` —
  ChildTier {Primary, Secondary, DiagnosticsOnly, Lsif}; priority-string
  mapping ("full"|"semantic" -> Primary, "secondary" -> Secondary, "lsif"
  -> Lsif, anything else -> DiagnosticsOnly); diagnostics-only pins BOTH
  extension lists to DiagnosticsOnly (registry.rs:111-121).
- `/Users/sac/lsp-max/crates/lsp-max-compositor/src/child_process.rs` —
  spawn returns (ChildProcess, exit_future); `spawn_and_register` DROPS the
  exit future (child_process.rs:121-131). No auto-respawn exists.
- `/Users/sac/lsp-max/crates/lsp-max-compositor/CLAUDE.md` — routing table
  (FirstSuccess/FanAll/Notify/PrimaryOnly), module map, law status.
- `/Users/sac/claude-code-config-lsp/lsp-max.toml` — deployed manifest.
- `/Users/sac/claude-code-config-lsp/README.md` (lines 100-155 pre-edit) —
  packaging gate + landed compositor stanza.
- Sibling receipts read: `p12-wiring.md` (20 provider overrides landed,
  cargo check exit 0, registered limitations), `h4-standing.md`
  (ALIVE/PARTIAL/BLOCKED/UNVERIFIED vocabulary, 10 tests green).

## Note on M1 coordination

The contract asked me to coordinate the manifest reference with M1's
receipt. No m1-m12 receipts exist on disk at my run time
(`ls docs/jira/v26.11.3/receipts/` shows only c*/h*/k4/p* lanes). I
grounded the manifest reference directly in the compositor's
deserialization source (config.rs) instead — the schema authority M1 would
itself cite. If an M1 receipt lands later with a divergent schema claim,
config.rs wins; reconcile then.

## Fleet multiplexing story (summary)

One stdio front door per Claude Code extension slot. Per-extension fan-out
via ExtensionRouter: `claude-code-config-lsp` claims .json/.md at
`diagnostics-only` (config-surface diagnostics layered on top), general
JSON LSPs hold Primary on non-config `.json` (full/semantic — win
hover/completion/definition via FirstSuccess), markdown children serve
`.md` interactivity. Diagnostics fan to ALL tiers and merge with
REFUSED_BY_LAW surviving dedup; didOpen/didChange/didClose notify all
children. Net: config diagnostics without displacing general service —
this repo's README fleet-evidence claim (lines 137-145), now mechanistically
documented against registry.rs/config.rs.

## Open items (from sources read this session)

From compositor CLAUDE.md law status (all CANDIDATE):
- Per-server receipt chain: persistent prev_hash chain head, persistent
  compositor Keystore, child-side receipt publication — all OPEN.
- OCEL accumulation (RFC C): CANDIDATE; Declare/DFG inline conformance:
  CANDIDATE.
From child_process.rs:
- No automatic child restart; crash cleanup is caller-wired. Documented in
  the guide as not-deployed.
From p12-wiring.md (this repo):
- semantic_tokens_full_delta is stateless full recompute; moniker passes
  project_root: None; corpus rebuilt per request; single-root workspace
  symbols; code_lens resolve_provider None.
From README (this repo):
- FM-GEN-007 ggen pack bug still blocks ontology regeneration;
  capabilities.rs hand-extended as workaround.
UNVERIFIED (not read this session): h2/h5/k4/c*/p* receipts other than
p12 and h4; sibling M-lane work products in other repos.

## Ship/no-ship verdict

**SHIP the docs.** Standing: PARTIAL_ALIVE.
- Guide claims are grounded in full reads of the compositor source this
  session; no claim rests on memory or prose.
- The deployment path itself is CANDIDATE-to-ALIVE on the child side
  (manifest deployed, packaging gate landed per README, provider wiring
  cargo-checks clean per p12) and PARTIAL on the compositor side (no
  restart, receipt-chain CANDIDATE items open). Shipping the documentation
  does not depend on the open compositor items because the guide states
  them as open.
- Falsifier: any manifest key, tier mapping, or routing behavior in
  `docs/reference/compositor-deployment.md` contradicted by the current
  lsp-max source falsifies the guide; re-run the source reads in the
  "Source receipts" section to check.
