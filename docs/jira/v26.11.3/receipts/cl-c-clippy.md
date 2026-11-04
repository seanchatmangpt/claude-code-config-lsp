# cl-c-clippy — clippy -D warnings gate (lane cl-c, v26.11.3)

Standing: **ALIVE** — both gates re-run this session against the exact subject on disk.

## Gates

- `cargo clippy --all-targets -- -D warnings` → **exit 0** (re-run after final edit, "Finished dev profile", 0 errors).
- `cargo test` full suite → **438 passed / 1 failed**, byte-identical to the pre-fix
  baseline run (zero-delta). The 1 failure is pre-existing and environmental:
  `multiplex_real_lsps::multiplex_real_language_servers_end_to_end`
  (panics at `tests/multiplex_real_lsps.rs:599`), failed identically at baseline
  before any cl-c edit.

## Findings fixed (behavior-identical only)

Clippy enumeration was refreshed at dispatch, not taken from LC3's residual list —
the live list was ~35 findings across 20 files. All fixed:

### src/
- `src/analyzers/hook.rs`, `src/analyzers/toml.rs`, `src/analyzers/json.rs`,
  `src/nouns/config.rs`: items_after_test_module — trailing items moved above
  the `#[cfg(test)] mod tests {` / `mod render_tests {` line (json.rs block ~470 lines).
- `src/analyzers/json.rs` additionally: the moved block's `#[cfg(test)]` attr re-attached
  to the mod line after the move (self-inflicted during the move, repaired in-session).
- `src/analyzers/commands.rs:368` useless_format — `format!("...")` → `.to_string()`.
- `src/provider_registry.rs:462` cloned_ref_to_slice_refs — `&[root.clone()]` →
  `std::slice::from_ref(&root)`.
- `src/nouns/config.rs:328` manual_pattern_char_comparison —
  `ends_with(|c: char| c == ' ')` → `ends_with(' ')`.

### tests/
- `tests/stdout_is_lsp_frames_only.rs` zombie_processes — `let _ = child.wait();`
  added after `kill()` (spawn+await ordering, per contract).
- `tests/compositor_e2e.rs:11` doc_lazy_continuation — blank `//!` line inserted.
- `tests/harness_ops.rs` unused imports removed; dead `corpus_path` helper deleted.
- `tests/ttl_reconciliation.rs` needless_lifetimes elided; map_or → is_some_and.
- `tests/standing.rs` unused lsp_types_max imports removed; `&PathBuf` → `&Path` ×3
  (+ `{Path, PathBuf}` import).
- `tests/admission.rs` `&PathBuf` → `&Path` (+ `{Path, PathBuf}` import).
- `tests/hierarchy_moniker.rs` two unused variables underscore-prefixed
  (`_ggen_path`, `_doc` — second `let doc` in the file left intact and is used).
- `tests/bcinr_parity.rs` needless_borrow `&input` → `input` at 98/100/109/110/118/120;
  sites at 132/134 deliberately keep `&input` (input there is `String` passed as `&str`).
- `tests/formatting.rs` expect_fun_call → `unwrap_or_else(|| panic!(...))`.
- `tests/navigation.rs` needless_borrow `&text` → `text` ×11.
- `tests/semantic_tokens.rs` dead `apply` helper deleted (zero call sites, verified by grep).
- Both deletions were verified against real usage before deletion
  (grep zero-call-site) — no `#[allow]` sprawl.

## Environmental note (not a code delta)

Mid-session, `lsp-max-compositor` (foreign repo `/Users/sac/lsp-max`) and this repo's
own release binary were missing when compositor_e2e ran (external deletion; this lane's
only compositor_e2e edit is a doc-comment blank line). Fix-forward: rebuilt both
(`cargo build --release` in each repo, 2m21s + 3m16s). After rebuild the suite matches
baseline exactly. Release binaries are artifacts; no repo file affected.

## Disclosed files touched (18)

src/analyzers/hook.rs · src/analyzers/toml.rs · src/analyzers/json.rs ·
src/analyzers/commands.rs · src/nouns/config.rs · src/provider_registry.rs ·
tests/stdout_is_lsp_frames_only.rs · tests/compositor_e2e.rs · tests/harness_ops.rs ·
tests/ttl_reconciliation.rs · tests/standing.rs · tests/admission.rs ·
tests/hierarchy_moniker.rs · tests/bcinr_parity.rs · tests/formatting.rs ·
tests/navigation.rs · tests/semantic_tokens.rs · tests/diagnostic_goldens.rs
