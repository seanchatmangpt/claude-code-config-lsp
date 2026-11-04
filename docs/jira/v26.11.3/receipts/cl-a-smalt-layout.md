# CL-A: smalt-core-pack layout (ONTOLOGY_SOURCE_MISSING unblock)

- **Lane**: CL-A (implementation), 2026-11-03
- **Repo**: /Users/sac/ggen-marketplace
- **Owned**: `packs/smalt-core-pack/` (all moves inside it), this receipt
- **Closes**: X5's scoped residual — `REFUSED:ONTOLOGY_SOURCE_MISSING:smalt-core-pack`
  (x5-smalt-core-manifest.md), cause `ontology_files()` (scripts/marketplace.py:255)
  discovering only pack-root `*.ttl` + singular `ontology/`; the pack used
  `ontologies/` (plural) + `shapes/`. Layout fix, not a validator change;
  scripts/marketplace.py untouched.

## Manufactured (pure moves + prose path updates, no content edits)

- `ontologies/` → `ontology/` (9 authored TTLs: kmip, cimi, dcterms, org, wac,
  tosca, security, rbac, occi — byte-identical contents, `mv` only).
- `shapes/smalt-core-shapes.ttl` → `ontology/smalt-core-shapes.ttl` (authored
  shared shapes folded into the discovered dir); empty `shapes/` removed.
- `pack.toml` + `PACK-NOTES.md`: prose path refs `ontologies/` → `ontology/`,
  shapes path updated. No `owl:imports` or machine-consumed relative paths
  existed (grep-verified pre-move); no TTL content touched.
- `vendor/` untouched — duplicates stay vendored/excluded from discovery.

## Verification ladder (real gates, real output)

1. **BEFORE** — `python3 scripts/marketplace.py validate`: exit 2, stderr
   exactly `REFUSED:ONTOLOGY_SOURCE_MISSING:smalt-core-pack` (sole line;
   MANIFEST_MISSING already cleared by X5).
2. **AFTER** — same command: exit 0,
   `validated packs=344 manifests=344 ontologies=571 templates=1948
   native_gates=1995 verifier_gates=21 profiles={project:126,projection:171,
   semantic:47} diataxis=20`. No refusal lines on stderr. Whole-corpus clean —
   no remaining refusals of any class for any pack.
3. **Parse gate post-move** — rdflib parse exit 0: smalt-core-shapes.ttl 131
   triples, dcterms.ttl 700, tosca.ttl 1434 (contents unchanged by the move).
4. **Stale-path sweep** — grep for `ontologies/` / `shapes/` across the pack:
   remaining hits are historical prose in PACK-NOTES receipts and in-file
   PROVENANCE headers, not live path references.

## Standing

ALIVE for the contracted scope: both X5 refusal classes
(MANIFEST_MISSING, ONTOLOGY_SOURCE_MISSING) are gone for smalt-core-pack on
the real whole-corpus load path, witnessed before/after. Whole-corpus residual
refusals: none (exit 0 corpus-wide). Full qualification run (qualify-marketplace.sh)
NOT run — outside this lane's scope.
