# M4 Routing Dispatch — Receipt

Status: ALIVE (all commands run this session, exit 0 on final run)

## Subject

New test file: `/Users/sac/lsp-max/crates/lsp-max-compositor/tests/routing_dispatch.rs`
(6 tests). No compositor src modified. No git operations run.

## Construction paths exercised

- `ExtensionRouter::new()` + `register()` (manual)
- `ExtensionRouter::new_from_manifest()` (generated registry init,
  `src/registry_init.rs`, ggen-projected from lsp-max.toml)
- `ExtensionRouter::from_config()` (TOML parse → router)
- Observable output: `fanout::servers_for_uri(&router, uri)` — the real dispatch
  decision the fanout pipeline consumes; asserted on final state, no mocks of
  the unit under test.

## Test matrix

1. `mapped_ttl_routes_to_child_a` — mapped extension → child-a exactly.
2. `manifest_path_ttl_routes_to_ggen_lsp` — same contract via generated
   manifest router (startup path).
3. `unmapped_extension_falls_back_to_empty` — documented fallback: empty
   candidate list, no cross-server leakage.
4. `disjoint_children_each_receive_their_own_extension` — two children,
   disjoint extension sets, isolation both directions.
5. `from_config_routes_mapped_and_rejects_unmapped` — full TOML → router path,
   mapped + unmapped asserted.
6. `hover_first_success_never_lands_on_diagnostics_only_child` — FirstSuccess
   (hover) dispatch is Primary-only; DiagnosticsOnly child mapped for .rs is
   never a hover target.

## Commands / exits

```
cargo test -p lsp-max-compositor --test routing_dispatch
# run 1: compile error (2× E0716 temporary-freed borrow in test 5) — fixed by
#        binding servers_for_uri results before borrowing ids
# run 2: 5 passed / 1 failed — fixture TOML missing required
#        `secondary_extensions` field for child-b — fixture fixed
# run 3 (final): test result: ok. 6 passed; 0 failed — exit 0
```

## Defects found in compositor src

None filed: `ExtensionRouter` pub API was sufficient for all three construction
paths (new_from_manifest exists on the generated impl in `registry_init.rs`).

## Notes

- `CompositorConfig` TOML schema requires `secondary_extensions` on every
  `[[server]]` entry (no serde default) — fixture-level constraint, not a defect;
  documented here for future test authors.
- Pre-existing sibling coverage: `tests/routing_integration.rs` (21 tests) uses
  the same observable; this file adds the manifest-init dispatch path and the
  strategy-vs-tier consistency check not present there.
