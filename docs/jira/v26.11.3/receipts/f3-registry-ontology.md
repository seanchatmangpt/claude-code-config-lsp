# F3 — registry ontology-first generation receipt

Lane: F3 IMPLEMENTATION. Repos: /Users/sac/lsp-max (writer),
/Users/sac/claude-code-config-lsp (read-only reference + receipt location).
No git run by this lane.

## Deliverables (all files re-read on disk this session)

- `crates/lsp-max-compositor/compositor.ttl` — ADDITIVE: `lspmax:andonPrefix`
  datatype property + 4 server individuals (claude-code-config-lsp
  DiagnosticsOnly on .json/.md prefix CCC-; vscode-json-lsp Primary on .json
  prefix VSJ-; markdown-lsp Primary on .md prefix MDL-; justfile-lsp Primary
  on .just prefix JUST-). A sibling lane later added ggen-lsp + wasm4pm-lsp
  individuals; both are consumers of the same rule, disclosed, not mine.
- `crates/lsp-max-compositor/queries/registry_servers.rq` — NEW. SELECT ?id
  ?tier ?ext over lspmax:ChildServer individuals, ORDER BY ?id ?ext
  (determinism law). Part of the one-rule addition.
- `crates/lsp-max-compositor/registry_init.tera` — REWRITTEN (disclosed
  template update, contract item 1): emits the row tier via ChildTier path
  (Primary/Secondary/Lsif, default DiagnosticsOnly) instead of the old
  hard-coded Primary/Secondary-only shape; nested loop rebuilds each server's
  full extension vec from the ORDER BY'd rows.
- `crates/lsp-max-compositor/ggen.toml` — additive edits, disclosed:
  (a) `[ontology] imports = ["compositor.ttl"]` (rule needs the individuals
  in the graph); (b) ONE new `[[generation.rules]]` entry
  `registry-init-from-ontology` (query/template file refs, output
  src/registry_init.rs, mode Overwrite, skip_empty true, `when` ASK guard
  `?s a lspmax:ChildServer`). Also: dropped pre-existing
  `language_id` ([project]) and two `description` keys (rules) — the
  committed manifest did NOT parse under installed ggen 26.10.11
  ([FM-CONFIG-003] unknown-field errors before my edits; disclosed minimal
  unblocks, same file, in place).
- `crates/lsp-max-compositor/src/registry_init.rs` — regenerated artifact
  (Overwrite): 11 registrations from the current 6 individuals, ASK guard
  fired, double-run byte-identical.
- `crates/lsp-max-compositor/tests/registry_ontology_test.rs` — NEW court,
  3 tests, 3 passed.

## Execution receipts (commands + exits, 2026-11-03)

- `ggen sync run --dry-run` (single-rule manifest staged in crate dir;
  ggen 26.10.11 has no per-rule selection): clean — one planned write
  `src/registry_init.rs`, closure hashes for ttl/rq/tera, ASK guard passed.
- `ggen sync run` (real): `written: [src/registry_init.rs]`; immediate
  second real run: `skipped: unchanged: content identical` — determinism
  double-run byte-identical, receipted on stdout JSON.
- Court: `cargo test -p lsp-max-compositor --test registry_ontology_test`
  → 3 passed; 0 failed. Tests: (1) generated_registry_knows_all_four_servers
  (all 4 contract ids + tier tokens present in src/registry_init.rs);
  (2) deleting_an_individual_drops_it_from_regeneration — real scratch copy,
  real ggen subprocess: deleting the claude-code-config-lsp block drops it
  from regeneration; 3 survivors remain; Primary tier token present;
  (3) determinism_double_run_byte_identical — two real ggen runs, byte-eq.
  Court non-vacuity witnessed live: first version of the deletion mutation
  (line-filter) orphaned Turtle continuation lines → parse error; second
  version consumed the whole file (inline ` .` terminators) → ASK guard
  correctly false → loud failure, not a fake pass. The court kills.
- `cargo check -p lsp-max-compositor` → Finished, exit 0. (One 2-minute
  pause: src/client/builder.rs in the root crate was transiently broken by
  ANOTHER lane's in-flight edit; cleared by its owner; not my file.)

## Contract premise divergence — disclosed, not silent

Contract said "the committed registry_init.rs knows all 4 servers." On disk
it held only wasm4pm-lsp (DiagnosticsOnly, .ocel/.receipt/.rs/.ts/.json) and
ggen-lsp (Primary, .ttl/.rq/.tera) — a stale 2-server hand artifact. The
contract's own server list (4 servers) is the authority; regeneration
therefore REPLACES the 2-server set with the contract's 4-server set. A
sibling lane has since added ggen-lsp + wasm4pm-lsp individuals to
compositor.ttl, so the current generated file carries all 6 (11
registrations). Material divergence from the committed file is real and
intended; ACCEPT with this disclosure rather than BLOCK, per the contract's
own 4-server specification.

## Standing

- Generation rule + individuals + court: ALIVE (ran this session; commands
  and exits above).
- Two pre-existing manifest keys dropped to make ggen 26.10.11 parse the
  file at all — disclosed unblocks, not silent repairs.
- Pre-existing cc-scaffold-stubs / cc-chicago-tests pack rules in ggen.toml
  still fail under installed ggen (FM-GEN-008 scenario_slug missing) —
  pre-existing, not introduced by this lane; the full-manifest `ggen sync
  run` aborts on them; my rule executes via single-rule staging (the
  mechanism the contract's "run ggen in lsp-max root" step requires,
  disclosed).

result: F3 landed ontology-first registry generation — 6 ChildServer
individuals in compositor.ttl, new SPARQL+Tera rule in ggen.toml with ASK
guard, regenerated registry_init.rs (11 registrations), court 3/3 green,
cargo check exit 0, receipt at
/Users/sac/claude-code-config-lsp/docs/jira/v26.11.3/receipts/f3-registry-ontology.md
