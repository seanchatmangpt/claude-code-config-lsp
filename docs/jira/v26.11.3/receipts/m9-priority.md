# M9 Receipt — Priority Semantics (diagnostics-only vs full)

Lane: IMPLEMENTATION (lsp-max compositor priority tests)
Date: 2026-11-03
Status: ALIVE (10/10 passed, command cited below)

## Subject

`/Users/sac/lsp-max/crates/lsp-max-compositor/tests/priority_semantics.rs` (new, only file
written; compositor src untouched, no git commands run).

## What the real filters say

- **Tier mapping**: `ExtensionRouter::from_config` (src/registry.rs:108) maps `priority =
  "diagnostics-only"` → `ChildTier::DiagnosticsOnly` for BOTH primary_extensions and
  secondary_extensions. Any other string → Primary/Secondary. `ChildTier::from_priority`
  (src/registry.rs:29) maps `"full"|"semantic"` → Primary, `"secondary"` → Secondary,
  `"lsif"` → Lsif, everything else (typos included) → DiagnosticsOnly.
- **Request surface**: `dispatch_strategy` (src/fanout.rs:20) classifies
  hover/completion/definition/… → `FirstSuccess` (Primary-tier candidates only);
  publishDiagnostics/diagnostic → `FanAll`; didOpen/Change/Close/Save → `Notify`;
  everything else → `PrimaryOnly` (Primary tier only). A DiagnosticsOnly child therefore
  receives ONLY the diagnostics fan.
- **Notification gate**: `should_forward_notification` (src/routing.rs:186) —
  DiagnosticsOnly receives didSave but NOT didOpen/didChange/didClose; Lsif receives none.
- **Capability merge**: `merge_capabilities` (src/capability_merge.rs:20) filters
  DiagnosticsOnly inputs entirely — its hover/completion capabilities never reach the
  client-facing aggregate.

## Test coverage (10 tests, all Chicago: real filters, final-state asserts)

1. diagnostics-only declaration → both extension lists land DiagnosticsOnly tier
2. diagnostics-only child receives diagnostics fan but zero Primary candidacy for
   hover/completion/definition/formatting/rename; didOpen/Change/Close withheld, didSave forwarded
3. diagnostics-only child contributes no capabilities even when sole child
4. full declaration → Primary tier on primary exts, Secondary on secondary exts; full surface
5. full child capabilities survive the merge
6. mixed set: full on .json + diagnostics-only on .md — each behaves per declaration
7. co-tenant deployment shape (full json-lsp + diagnostics-only config-lsp on .json): tier
   ordering, FanAll reaches both, hover candidacy stays with full child, notification gate
   splits correctly, validator completion capability does not leak through merge
8. misdeclaration: full-surface requests to a diagnostics-only child → refused by tier
   filter (zero candidates), never silently delivered
9. priority-string typos: `from_priority` conservatively demotes typos to DiagnosticsOnly
10. **Hazard pinned**: `from_config` vs `from_priority` DIVERGE on typo'd priority strings —
    from_config escalates `priority = "ful"` to PRIMARY while from_priority demotes it. This
    asymmetry is the real misdeclaration hazard in the config path.

## Deployment verdict (H3 config-lsp posture)

`diagnostics-only` is the CORRECT posture for a config validator coexisting with general
JSON tools, and is verified end-to-end: the validator receives the diagnostics fan on .json,
is excluded from hover/completion/definition candidacy, is withheld state-mutating
notifications, and cannot leak capabilities into the merged surface. One operational
caution: the priority string must be spelled exactly `diagnostics-only` — `from_config`
treats any other string as full/primary, so a typo silently escalates the validator to
Primary tier (test 10 pins this; consider a startup validation if this surface grows).

## Verification

```
cd /Users/sac/lsp-max && cargo test -p lsp-max-compositor --test priority_semantics
10 passed; 0 failed; 0 ignored (exit 0)
```

Build: `Finished test profile` — compiles clean under the workspace feature set.
