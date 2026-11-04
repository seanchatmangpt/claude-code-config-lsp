# cl-b-c9 — schema_expansion court rustfmt-stability receipt

Lane: cl-b-c9 (IMPLEMENTATION, tests/schema_expansion.rs owner). 2026-11-03.

## Attributed cause (LC3)

`tests/schema_expansion.rs:49` and `:72` source-grepped the single-line form
`settings.fields.push(("key"...` while C9's landed `src/schema.rs` rows are
rustfmt'd multi-line — the court failed 4/2 on locator drift, not missing
capability.

## Diff (file owned: `tests/schema_expansion.rs` only; `src/schema.rs` untouched)

### Before

- Line 49: `src.contains("settings.fields.push((\"{key}\"")` on raw lines —
  fails on rustfmt'd rows (`settings.fields.push((\n "env".to_string(),`).
- Lines 68–77: `src.lines().find(|l| l.contains("\"{key}\""))` — single-line
  window; the enum member (`enum: normal | vim`) lives on a later line after
  rustfmt, so `line.contains(enum_member)` failed.

### After

- New `normalized()` helper: `src.split_whitespace().collect::<String>()` —
  removes ALL whitespace, so the locator is layout-insensitive for both the
  single-line and rustfmt'd multi-line forms of a row. (An intermediate
  space-join variant was tried first and discarded: it required a space
  after `push((`, pinning the multi-line form only. The shipped version is
  the fully whitespace-free one.)
- Test 1: match on normalized source, same key set (26 keys).
- Test 2: locate `settings.fields.push(( "{key}"` in normalized source, bound
  the enum-member contains-check to a 400-normalized-char window from that
  index so a member belonging to a *different* field cannot satisfy the
  assertion. Same 6 (key, member) pairs.

## Verification (ALIVE — commands + exits this session)

```
cd /Users/sac/claude-code-config-lsp
cargo test --test schema_expansion   # run 1: 6 passed; 0 failed — exit 0
cargo test --test schema_expansion   # run 2: 6 passed; 0 failed — exit 0
```

Rustfmt stability: the locator operates on a whitespace-free view of
`src/schema.rs`, so rustfmt reflow between single-line and multi-line row
layouts cannot flip these assertions. `src/schema.rs` was not modified by
this lane. Non-vacuity by construction: each key/member check reduces to a
`contains` on the whitespace-free view — removing the key row or enum member
from `src/schema.rs` removes the needle and the assertion fails. (No shim
mutation was executed against `src/schema.rs` — file out of this lane's
ownership; noted as the one UNVERIFIED edge.)

## Standing

ALIVE. Court green 2×, rustfmt-stable by mechanism, not by suppression: both
tests still fail if a key or enum member is removed (the normalized needle or
window check misses). No skips, no allowlist entries.
