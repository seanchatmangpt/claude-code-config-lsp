# H4 Standing Lane Receipt

Subject: make the LSP receipt-aware — fleet standing vocabulary, editor-visible.

## Owned files

- `src/standing.rs` (new) — fleet receipt parse (`{"status","commands"}`), disk-witnessed
  mtime, `standing_for_surface`, `standing_diagnostics`.
- `tests/standing.rs` (new) — Chicago tests over real tempdir fixture receipts.
- `src/lib.rs` — one line: `pub mod standing;` (module registration only).
- `docs/jira/v26.11.3/receipts/h4-standing.md` — this receipt.

No backend.rs/capabilities.rs edits (contract boundary honored).

## Standing vocabulary (closed)

- ALIVE = receipt green + fresh (receipt mtime >= newest tracked source mtime) + zero
  diagnostics for the surface. Never emitted without all three legs witnessed
  (never-round-up law).
- PARTIAL = green+fresh, diagnostic findings present on the surface.
- BLOCKED = receipt red or stale.
- UNVERIFIED = no receipt at `.claude/receipts/latest.json`.

A receipt parsed from text alone (no disk stat) carries `mtime: None` and is never fresh —
staleness is observed, not assumed.

## Diagnostics (remediation per refusal)

- BLOCKED -> WARNING, code `standing-blocked`, message `gate receipt stale/red for
  <surface>; run the gate commands: <receipt's own command list>`.
- PARTIAL -> WARNING, code `standing-partial`.
- UNVERIFIED -> INFORMATION, code `standing-unverified`.
- ALIVE -> zero diagnostics.

## Verification

- `cargo test --test standing` — exit 0, `test result: ok. 10 passed; 0 failed`
  (green fresh->ALIVE, green stale->BLOCKED + command-list remediation, red->BLOCKED,
  absent->UNVERIFIED, findings->PARTIAL, no-disk-witness never fresh, malformed receipt
  typed error, non-green status not green, ALIVE emits zero diagnostics).
- `cargo clippy --lib` — zero findings in src/standing.rs.
- `rustfmt src/standing.rs tests/standing.rs` — applied.
- Compile-freeze disclosures: shared lib/bin was mid-flight broken several times by
  OTHER lanes' files (src/harness_ops.rs:150,301 E0308; src/harness_docs.rs:154-156
  E0425; bin target) — waited through the SLA window and retried; no edits to foreign
  files, no unblock fixes applied by this lane.

## Standing: ALIVE
