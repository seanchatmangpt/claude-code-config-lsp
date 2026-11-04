# L2 Docs Version Bump — Receipt (v26.11.3)

Status: ALIVE (all greps/edits run this session, 2026-11-03)
Lane: L2 (docs/**, README.md, GETTING_STARTED.md, FORWARD_DEPLOYMENT.md). Did not run git, did not touch Cargo.toml / CHANGELOG.md / src / tests.

## Dependency on L1
- L1's workspace version HAD LANDED before this lane started: `Cargo.toml:142`
  `version = "26.11.3"` (verified on disk). No polling was needed.
- Open dependency: `Cargo.toml` still pins `lsp-types-max = { version = "26.6.24" }`
  (lines 47, 154). Docs now cite 26.11.3+ for the sibling; L1's dep-pin bump should
  close this. NOT touched (Cargo.toml is L1-owned).

## Files edited (before -> after version refs)
- README.md:39: `lsp-max = "26.7"` -> `"26.11.3"`. Also added
  `## What's New in 26.11.3` block (compositor courts M4-M9, F3 ontology-first
  registry generation, M13/H3 packaging gate, CalVer note), sourced from
  receipts m4/m5/m7/m9, f3, m13, h3.
- GETTING_STARTED.md:358,371: error-message example `=26.6.24` -> `=26.11.3`;
  `e.g., 26.6.24` -> `e.g., 26.11.3`.
- docs/EXAMPLES.md:288-289: lsp-types-max "26.6.24+" -> "26.11.3+";
  clap-noun-verb "26.6.2+" -> "26.11.3+" (sibling repos on shared CalVer).
- docs/TUTORIAL_getting_started.md:21: expected output `26.6.9` -> `26.11.3`.
- docs/tpot2-autonomic-mesh.md:12: `workspace 26.6.x` -> `26.11.x`.
- docs/tpot2-pipeline.md:286: JSON output `"version": "26.6.9"` -> `"26.11.3"`.
- docs/how-to/release.md: Current Version `26.7.3` -> `26.11.3`; "Current version
  is 26.7.1 (July 1, 2026)" -> "26.11.3 (November 3, 2026)"; illustrative bump
  examples `26.7.15` -> `26.11.15`; publish/checksum/yank/add examples
  `26.7.1` -> `26.11.3`; out-of-sync troubleshooting template `26.6.24` ->
  `26.10.24` / `"26.6"` -> `"26.10"` (kept as coherent illustrative scenario).
- docs/reference/WASM4PM_COMPAT_REVIEW_SPEC.md:147: `26.6.9 or similar` ->
  `26.11.3 or similar`.
- docs/tutorials/getting-started.md:327: LSIF metaData example
  `"version":"26.6.21"` -> `"26.11.3"`.

## Deliberately NOT edited (records / honesty)
- docs/archive/** — historical audit trail, untouched.
- docs/jira/** — historical receipts, untouched.
- docs/automatic-configuration.md:89-97 — recorded verification of a specific
  fixed manifest bug (observed `lsp-types-max v26.6.5` compile lines and the
  `26.6.24`-vs-`26.6.5` incident description); rewriting would falsify the record.
- docs/rfcs/0005-calver-versioning-over-semver.md — RFC's own examples of the
  CalVer convention (26.6.24 etc.); illustrative, left as written.
- docs/how-to/release.md incident/post-mortem TEMPLATE dates (2026-06-14) — sample
  content, out of version scope.
- PROJECT.md — does not exist in /Users/sac/lsp-max.
- FORWARD_DEPLOYMENT.md — contains no version references (verified by grep).

## Verification (this session)
- `grep -rn "26\.6\.\|26\.7\.\|\"26\.6\"\|\"26\.7\"..."` over README.md,
  GETTING_STARTED.md, FORWARD_DEPLOYMENT.md, docs/EXAMPLES.md,
  docs/TUTORIAL_getting_started.md, docs/tpot2-*.md, docs/how-to/**,
  docs/reference/**, docs/tutorials/**, docs/explanation/** -> exit 1
  (zero matches).
- Historical trees (docs/archive/**, docs/jira/**) intentionally excluded from
  the zero-match gate; their version strings are records, not stale claims.
