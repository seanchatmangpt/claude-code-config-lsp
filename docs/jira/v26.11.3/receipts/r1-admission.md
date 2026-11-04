# R1 — admission-controller lane receipt

Lane: R1 IMPLEMENTATION (src/admission.rs + tests/admission.rs).
Repo: /Users/sac/claude-code-config-lsp (shared canonical checkout; no git run by this lane).

## Deliverables (all on disk, verified)

- `src/admission.rs` — NEW, ~490 lines. The compiler-backed admission layer over the
  fleet's landed modules, importing only pub APIs (no sibling edits):
  - **BlastRadius**: `blast_radius(corpus, doc_path)` /
    `blast_radius_capped(..., max_depth)` — transitive incoming closure via
    `hierarchy::incoming_calls` recursion (depth-capped `MAX_DEPTH=8`, cycle-guarded by
    a BTreeSet on canonical paths) + outgoing closure via `hierarchy::outgoing_calls`.
    Returns `BlastRadius{root, root_symbol, incoming, outgoing}` of `SliceEntry{path,
    symbol, ranges}` — the exact file/symbol slice set. `digest()` = blake3 hex of the
    sorted canonical radius form.
  - **Gate**: `admit_edit(proposal{uri,span,description}, radius, config) ->
    Admission{verdict, slices, receipt}` — REFUSED when any slice path starts with a
    `config.protected_prefixes` critical-path prefix (reason names the intersected
    prefixes), PARTIAL when the radius is non-empty (full backward+forward slice set
    injected as the context payload — the zero-token graph-slicing context claim made
    real), ALIVE when the radius is empty. REFUSED > PARTIAL > ALIVE precedence.
  - **Transition receipts**: `TransitionReceipt` shaped `ggen.admission-transition/1`
    (schema, decision, blake3 radius digest, RFC3339 timestamp, subject=uri#span,
    reason, prev_hash, hash) appended to `AdmissionJournal` with prev-hash chaining
    (`append` stamps prev_hash from the tail and chain-hashes the full payload);
    `append_unchecked` replays captured chains; `verify_chain` recomputes every hash —
    any post-hoc field mutation breaks it.
  - **Delta grounding**: `ground(&SemanticTokensDelta) -> GroundedObservation` maps P8
    edits into compact `<slot>:<legend-type>[+deprecated][+mutating]` classifications
    using `semantic_tokens::token_type_strings()`; modifier bits decoded per
    `admission::modifiers` (LSP standard deprecated bit 2; mutating bit 5 local
    convention — the repo legend declares no modifiers, noted honestly). Grounding
    asserts output 10x smaller than the raw 5-int-per-token payload.
- `tests/admission.rs` — NEW, 12 Chicago tests over the real fixture corpus
  (`tests/fixtures/claude-config/`, real file IO): radius over fixture call chains
  (incoming via a pushed referencing doc, repo convention), out-of-corpus empty radius,
  cycle-guard termination over mutual wikilink references, depth-cap traversal
  a->b->c->d (depth 8 reaches d, depth 1 stops at b), critical-path REFUSED witnessed
  with reason receipt, non-matching protection stays PARTIAL, empty radius ALIVE with
  empty payload, PARTIAL injects the full slice set + digest equality, journal chaining
  (genesis prev_hash empty, record 2 chains to record 1's hash), tampered record with
  forged hash detected, forged prev_hash detected, grounding compactness (10x), grounding
  deprecated modifier surfacing.
- `src/lib.rs` — one-line `pub mod admission;` PLUS an SLA minimal unblock (see below).

## Commands + exits (real run this session)

- `cargo check --lib` — exit 0.
- `cargo test --test admission` — **12 passed / 0 failed**.
- `cargo test --lib admission` — witness `ground_compacts_a_delta`: 1 passed / 0 failed.
- `cargo clippy --lib` — zero warnings originating in admission.rs (grep count 0).
- `cargo test` (full, --no-fail-fast, mid-session tree state) — lib 264→173 passed
  depending on which lib.rs generation was on disk (foreign lane rewrote lib.rs
  mid-session, see below); admission target 12/12 throughout.

## Pre-existing vs session-introduced failures

Session-introduced: none. Two classifications from the FINAL tree state (full lib.rs
restored by the foreign lane, `pub mod admission;` present at src/lib.rs:62):

- `cargo test --lib` — 259 passed / **2 failed**: `hover::tests::hover_on_key_returns_just_that_entry`
  and `hover_on_value_shows_field_entry` (src/hover.rs:145) — FOREIGN drift in the
  generated hover tables (hover.rs untouched by this lane; the foreign lane regenerated
  lib.rs/hover content mid-session). Not introduced by this lane.
- Integration suites (`compositor_e2e` timeout, `schema_expansion`, `tokens_color`
  integration_delta, `ttl_reconciliation`) also failed mid-session under the truncated
  lib.rs window — foreign-lane regeneration state, again not this lane's diff.

This lane's targets, final state: `cargo test --test admission` 12/12, lib admission
witness 1/1, clippy clean (0 admission warnings), `cargo check --lib` exit 0.

## Compile-freeze SLA events (disclosed)

1. Mid-session a foreign lane regenerated `src/lib.rs` down to 8 module declarations
   (dropping hierarchy/tokens_delta/navigation among ~20 others). `tests/admission.rs`
   stopped compiling (E0432 unresolved imports). Per the 10-minute SLA this lane applied
   the minimal unblock: re-added exactly `navigation`, `hierarchy`, `tokens_delta`,
   `admission` (semantic_tokens was still declared), with a `// SLA minimal unblock`
   comment naming the lane and scope in src/lib.rs:27-33. The owning lane supersedes
   on landing.
2. Also landed: `pub mod admission;` (this lane's own required declaration).

## Wiring points for the integration wave (no sibling edits made)

- backend.rs / provider_registry.rs: expose `admission::blast_radius` +
  `admit_edit` behind a `ccc/advertise`-class surface — natural seam is
  `provider_registry::admit_edit`-style delegate taking `provider_corpus()`; the
  protected prefixes should come from a config read (e.g. a `ccc.admission.protected`
  setting or AdmissionConfig::new with workspace-root-relative critical paths).
- Journal persistence: `AdmissionJournal` is in-memory only; persistence (file-backed
  journal alongside receipt.rs's receipts dir) is the next transition, not landed here.
- ground(): reachable from the `semanticTokens/full/delta` handler
  (provider_registry::semantic_tokens_full_delta) by wrapping the response delta with
  ground(&delta) for agent-readable output.

## Standing

- admission.rs + tests/admission.rs: **ALIVE** (ran + passed this session; commands +
  exits above).
- Journal persistence, backend wiring: **UNVERIFIED / not landed** (out of lane scope).
