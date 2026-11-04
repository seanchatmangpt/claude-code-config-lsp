# P10 — TTL Reconciliation Receipt

Lane: P10 (IMPLEMENTATION). Date: 2026-11-03.
Repo: /Users/sac/claude-code-config-lsp (one canonical checkout; no git run by this lane).

## Subject

- `schema/claude-code-config.ttl` — APPEND-ONLY after marker
  `# --- ontology reconciliation (P10): analyzers richer than TTL, encode the delta ---`
  (no existing individuals modified; C9/C9-sibling individuals untouched)
- `tests/ttl_reconciliation.rs` — new, the permanent anti-drift court.

## What was encoded (TTL catches up to analyzers)

Authorities cited per block inside the TTL:
- `src/analyzers/json.rs`: VALID_HOOK_EVENTS (26 individuals: PreToolUse … ElicitationResult,
  incl. TeammateIdle, TaskCompleted, Setup, Elicitation, ElicitationResult),
  VALID_HOOK_TYPES, VALID_WORKTREE_BASE_REFS, VALID_SKILL_OVERRIDES,
  VALID_PARENT_SETTINGS_BEHAVIOR, VALID_MCP_SERVER_TYPES, VALID_MODELS,
  VALID_EFFORT_LEVELS, VALID_PERMISSION_MODES (as ccc:EnumConstraint individuals),
  deprecated keys CCC-JSON-001/002/013 (9 ccc:DeprecatedKey individuals with
  replacementKey + diagCode, DiagnosticRule-adjacent).
- `src/analyzers/commands.rs`: allowed-tools, disable-model-invocation (ccc:SchemaField).
- `src/analyzers/frontmatter.rs`: context (fork), effort (low/medium/high/xhigh/max),
  1024-char description limit as ccc:DiagnosticRule individual ccc:constraint_description_max_length
  (CCC-AGENT-005).
- `src/file_types.rs`: 4 ccc:LanguageId individuals citing SUPPORTED_FILE_TYPES as
  authority (grounds the C10 languageId reconciliation).

## Court: tests/ttl_reconciliation.rs (11 tests)

Text-level bidirectional fence, ggen refusal_crosswalk_court precedent:
- Direction A (analyzer → TTL): every VALID_* literal, every deprecated-key literal
  (rule-slice scoped, raw-string-safe), every languageId must appear in the TTL.
- Direction B (TTL → analyzer): every ccc:eventName (26), ccc:enumValues /
  fieldEnumValues token, ccc:deprecatedKey, ccc:replacementKey, ccc:languageIdValue,
  plus the 4 skill frontmatter fields + 1024 limit must appear in the analyzer sources.
- Census tests pin counts (26 events, 9 deprecated keys, 4 languageIds) — corpus
  growth forces a conscious court update, never silent drift.

## Non-vacuity (shim-court protocol)

Mutation: `ccc:eventName "TeammateIdle"` → `"TeammateIdleX"` on the real file.
Result: court FLIPPED to FAILED (10 passed / 1 failed). Restore verified byte-exact
(sha256 `bb72f6ff7f5817b006b0be058125d69f4b3b7d62cf26506480c6a6de53fa9b01`).
A court that cannot flip is FAIL; this one flips.

## Commands / exits (ALIVE this session)

```
cargo test --test ttl_reconciliation          # 11 passed; 0 failed
cargo test --test schema_expansion            # 16 passed; 0 failed
cargo test --test diagnostic_goldens          # 6 passed; 6 passed (TTL consumers unaffected)
python3 rdflib parse schema/claude-code-config.ttl  # TURTLE-OK 933 triples
```

Compile contention from other lanes' in-flight edits (src/nouns/config.rs,
src/moniker.rs, src/selection.rs, src/backend.rs) was waited out per SLA; none of
those files are owned by this lane and none of the failures were caused by P10 files.

## Verification ladder

narrow (court tests) → TTL consumers (schema_expansion, diagnostic_goldens) →
Turtle well-formedness (933 triples, rdflib). Pre-existing vs session: all failures
observed during the run were transient compile churn from OTHER lanes' files; zero
failures attributed to P10-owned files in any passing run.

## Standing

ALIVE for the P10 surface. `ggen sync` remains blocked repo-wide by upstream
FM-GEN-007 (pre-existing, documented in CLAUDE.md) — the TTL delta is therefore
hand-encoded and court-fenced, which is exactly the inversion this lane closes:
the analyzers are the law; the TTL now states what the analyzers enforce, and
tests/ttl_reconciliation.rs refuses drift in either direction.
