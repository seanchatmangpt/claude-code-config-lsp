# AF-23 — H4 standing-diagnostics publish wiring (IMPLEMENTATION lane receipt)

Date: 2026-11-03. Repo: `/Users/sac/claude-code-config-lsp`. Lane:
IMPLEMENTATION (AF-23, closes AB-18's disclosed H4 wiring gap per PG-4's G3
design). No git commands run.

## Subject

`src/standing.rs` (`standing_for_surface` / `load_receipt` /
`standing_diagnostics`) landed with zero backend callers (AF-2/AB-18). This
lane wires the standing vocabulary into the real did_open/did_change publish
batch.

## Diff (owned files only)

- `src/backend.rs` — standing layer appended to `scan_uri_classified`:
  - `standing_layer()` resolves the workspace root from
    `provider_roots()[0]` (fallback cwd) and delegates to the pure core.
  - `pub fn standing_layer_for_root(root, surface, analyzer_finding_count)`
    — pure-core standing layer: classify gate (`classify == "unknown"` →
    empty; standing is a config-surface property only), real disk
    `load_receipt(root)`, `standing_for_surface(..., newest_source_mtime =
    None, count)` per the never-round-down law (standing.rs:137-143: a
    receipt parsed without a disk mtime witness never claims fresh; a
    disk-witnessed green receipt stays fresh against no source baseline), and
    `standing_diagnostics` mapped into `Finding`s with
    `LawAxis::Custom("standing")`. Exposed `pub` for courts.
  - Both `did_open` and `did_change` reach this via the trait-default
    publish path (`handle_did_open` → `publish_findings_classified` →
    `scan_uri_classified`), so the standing diagnostic rides the SAME
    `publishDiagnostics` notification as the analyzer findings — one merged
    batch.
- `tests/standing.rs` — 6 new courts (Chicago: real tempdir receipts via
  `load_receipt`, real analyzers via `scan::analyze_document`, no mocks):
  1. `receiptless_workspace_emits_unverified_merged_with_analyzer_findings`
     — both code families present in one merged batch (CCC-JSON-* +
     standing-unverified), source tag "standing".
  2. `alive_receipt_emits_no_standing_diagnostic` — green+fresh+clean →
     ALIVE emits nothing (standing.rs:150 law).
  3. `green_receipt_with_findings_emits_standing_partial` — PARTIAL fires
     only on findings-present over a green receipt.
  4. `standing_severities_are_never_error` — across BLOCKED/PARTIAL/
     UNVERIFIED, severity never ERROR (ANDON law: lsp-max auto-injects
     LSPMAX-ANDON-PUSH-MISSING on any ERROR-severity non-andon diagnostic).
  5. `unclassified_surface_emits_no_standing_diagnostic` — README.md in a
     receiptless workspace emits zero standing diagnostics (classify gate).
  6. Helper composition courts: `merged_batch` mirrors the exact
     `scan_uri_classified` composition (analyzers → count → standing layer).
- `tests/protocol_roundtrip.rs` — fix-forward of a stale premise (the wire
  test asserted every published code is `CCC-*`; the batch now legitimately
  carries two source families). Accepts `standing-*` codes only from
  `source == "standing"`. Unassigned file; AB-18's
  `compositor_relay_stress.rs` untouched.
- `docs/jira/v26.11.3/receipts/af-23-h4-wiring.md` — this receipt.

## Wire-level witness

`cargo test --test protocol_roundtrip` failure BEFORE the test fix (kept as
the wiring witness — the standing diagnostic arrived over the real
Content-Length-framed stdio wire on a fixture did_open):

```
panicked at tests/protocol_roundtrip.rs:327:13:
diagnostic code must be a CCC-* code for "agents/default-agent.md", got:
{"range":(0,0)-(0,0),"severity":3,"code":"standing-unverified",
 "source":"standing","message":"never scanned: no gate receipt at
 .claude/receipts/latest.json for file:///fixtures/claude-config/agents/
 default-agent.md; run the gate commands to establish standing"}
```

The publish batch merge is wired end-to-end, not just unit-level.

## Gates (this session, real output)

- `cargo test` × 2: 34/34 targets `test result: ok.`, zero FAILED
  (standing suite 15/15 incl. the 6 new courts; protocol_roundtrip 3/3 after
  the premise fix). Second pass re-ran clean.
- `cargo clippy --all-targets -- -D warnings`: the ONLY error is
  `tests/compositor_relay_stress.rs:12:5 doc list item without indentation`
  — AB-18's owned, in-flight file (compile-abort of one target; census rule:
  compile-abort ≠ failure count). Zero clippy findings in any AF-23 file;
  lib + all other test targets passed clippy before the abort.
- Baseline claim "595+ tests": re-read from this session's runs — the
  suite total across targets exceeds the 595 baseline with the 6 new courts
  added.

## Standing

ALIVE for the lane scope: standing diagnostics now publish on
did_open/did_change of classifyable config surfaces, merged with analyzer
findings; ALIVE standing emits nothing; severities WARNING/INFO only;
unclassified surfaces emit nothing. `publish_cross_file_diagnostics` (the
separate cross-file publish) untouched — AB-18's lane.
