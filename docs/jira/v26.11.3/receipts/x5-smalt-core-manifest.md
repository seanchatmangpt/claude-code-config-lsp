# X5: smalt-core-pack manifest (MANIFEST_MISSING unblock)

- **Lane**: I6 (implementation), 2026-11-03
- **Repo**: /Users/sac/ggen-marketplace
- **Owned files**: `packs/smalt-core-pack/pack.toml` (NEW), this receipt
- **Trigger**: K5's filed blocker + identity catalog known violation —
  smalt-core-pack had content (9 ontologies, 1 shapes file, vendor/ tree)
  but no pack.toml, so `require_admitted()` (scripts/marketplace.py:408) refused
  every whole-corpus load with `MANIFEST_MISSING`.

## Manufactured

`packs/smalt-core-pack/pack.toml` (NEW, 0.1.0):

- `[pack] name/version/description` — description carries the vendoring-lane
  truth (official verbatim vendored: dcterms, ORG, WAC, Security, TOSCA/SODALITE,
  OCCI; hand-derived, declared not-official in-file: KMIP v1.4, CIMI/DSP0263,
  NIST RBAC; shared shapes; PARTIAL_ALIVE per PACK-NOTES.md — no consuming
  ggen projection yet, templates/ empty).
- `[capabilities] provides = ["urn:ggen:pack:smalt-core-pack"]`, no `requires`
  (leaf pack).
- No `[dependencies]`, no targets table (TARGETS_IN_MANIFEST is a refusal class),
  no unknown [pack] keys (PACK_KEY_UNADMITTED admits only name/version/description/
  deprecated/superseded_by).

## Verification ladder (real gates, real output)

1. **BEFORE** — `python3 scripts/marketplace.py validate` (whole corpus, pre-manifest):
   exit 2, stderr exactly:
   ```
   REFUSED:MANIFEST_MISSING:smalt-core-pack
   REFUSED:ONTOLOGY_SOURCE_MISSING:smalt-core-pack
   ```
   (whole-corpus stderr was exactly these 2 lines — no other pack had refusals)
2. **Manifest schema check**: name==dirname (PACK_DIRECTORY_IDENTITY clean),
   version matches SEMVER, description non-empty. Cleared by the AFTER run below.
   Note: `[capabilities]` is not validated by scripts/marketplace.py (only
   ggen-engine reads it; sibling packs smalt-substrate/semconv use the same shape).
3. **AFTER** — same command, manifest present: exit 2, stderr exactly:
   ```
   REFUSED:ONTOLOGY_SOURCE_MISSING:smalt-core-pack
   ```
   `MANIFEST_MISSING:smalt-core-pack` **gone** — the contracted unblock is
   witnessed on the real whole-corpus load path.
4. **Full qualification NOT run** — K5's scoping decision stands.

## Remaining blocker (NOT closed by this lane)

`REFUSED:ONTOLOGY_SOURCE_MISSING:smalt-core-pack` persists. Cause is structural,
not manifest-shaped: `ontology_files()` (scripts/marketplace.py:255) globs only
pack-root `*.ttl` and a singular `ontology/` dir — this pack stores its TTLs
under `ontologies/` (plural) and `vendor/`. Closing it requires moving/renaming
files I do not own (ontologies/ -> ontology/ or root-level TTLs) — hand-off to
the file-owning lane. The manifest itself is schema-lawful; this is a layout
convention violation, a second and distinct refusal class.

## Standing

PARTIAL_ALIVE. Manifest refusal closed (witnessed before/after on the real
require_admitted load path); whole-corpus load still refuses on the
layout-convention refusal, now precisely scoped with file:line.
