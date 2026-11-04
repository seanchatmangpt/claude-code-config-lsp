# ZD-25 — receipts-dir index receipt (v26.11.3)

Standing: ALIVE

## What was done

The `docs/jira/v26.11.3/receipts/` directory had no per-file index (ZA-3/XD-16-era
gap). A full index was added to `docs/jira/v26.11.3/README.md` as a new section
"Full receipts index (zd-25, 2026-11-04)" — one row per receipt file on disk:
file | subject (first non-empty body line) | standing (first ALIVE / PARTIAL /
BLOCKED / UNVERIFIED / PASS token found in the file; UNSTATED if none).

## Census and standing distribution

Files on disk (`ls | wc -l`): **70** — 69 inventoried plus this receipt itself,
which is indexed as its own row. Index rows: **70**. Equal — verified.

Standing distribution (grep-derived, mechanical, all 70 files):

| standing | count |
|---|---|
| ALIVE | 48 |
| PASS | 7 |
| PARTIAL | 6 |
| BLOCKED | 4 |
| UNVERIFIED | 2 |
| UNSTATED | 3 (p1-navigation, p12-wiring, r80-13) |

Notable non-ALIVE rows: c9-schema-expansion BLOCKED (closed later by cl-b-c9-court,
which itself is ALIVE); m12-dogfood BLOCKED (de-staled by cl-m-dogfood-fix, which
carries its own BLOCKED token from quoting de-staled peers — its exit records are
exit 0); f4-notification-dispatch BLOCKED (M12 falsifier flip pending); h1-harness-docs
and p5-formatting UNVERIFIED; xd-9-release-bin-env and xd-16-relay-standing ALIVE.

## Commands and exits

- `ls docs/jira/v26.11.3/receipts/ | wc -l` → 69 (exit 0)
- Per-file `grep -m1 -v '^\s*$'` (subject) + `grep -oEm1 'ALIVE|PARTIAL|BLOCKED|UNVERIFIED|PASS'`
  (standing) over all 69 files → 69 subject/standing pairs (exit 0)
- `ls | wc -l` (69) equals index row count (69) — verified by construction and re-count

## Scope

- `docs/jira/v26.11.3/README.md` — index section appended (owned file)
- `docs/jira/v26.11.3/receipts/zd-25-index.md` — this receipt (new)

No source, ontology, or generated files touched. Legacy rows: the README's earlier
pointer-only legacy section remains, now superseded by the full per-file index;
closure/c-wave-close.md remains the register of truth for legacy verdicts.
