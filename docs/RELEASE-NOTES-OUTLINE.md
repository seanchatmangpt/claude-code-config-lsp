# Release Notes Outline — v26.7.3

**Version:** 26.7.3  
**Release Date:** 2026-07-03  
**Status:** CANDIDATE  

---

## Overview

OCEL persistence layer, receipt chain integration, ecosystem tower support, 40+ diagnostic rules, cross-file validation, marketplace plugin distribution.

---

## New Features

### OCEL Persistence & Event Accumulation
- **Story Keys:** CCC-OCEL-001, CCC-OCEL-002, CCC-OCEL-003
- **Description:** Event log (OCEL) recording and persistence for workspace audit trails; process mining integration; accumulator trait with multi-backend support (file, database, stream)
- **Impact:** Enables compliance tracking, workflow analysis, and forensic diagnostics

### Receipt Chain Integration
- **Story Keys:** CCC-RECEIPT-001, CCC-RECEIPT-002
- **Description:** LSP conformance receipt generation; immutable hash chain for workspace state snapshots; linked to OCEL events
- **Impact:** Workspace integrity verification; audit trail closure for compliance

### Ecosystem Tower Support
- **Story Keys:** CCC-ECO-001, CCC-ECO-002, CCC-ECO-003
- **Description:** Integration with wasm4pm-compat, bcinr, chicago-tdd-tools, cargo-cicd; cross-project configuration sharing; ecosystem-wide constraint model
- **Impact:** Multi-project conformance scoring; shared compliance rules across workspace ecosystem

### Diagnostic Rules Expansion (40+ Rules)
- **Story Keys:** CCC-DIAG-001 through CCC-DIAG-040+
- **Description:** Extended diagnostic coverage for settings.json, CLAUDE.md, plugin.json, marketplace.json, hooks, agents, skills; semantic token classification; field-level validation
- **Categories:**
  - Permission validation (10+ rules)
  - Scope detection (5 rules)
  - Constraint compliance (15+ rules)
  - Cross-file consistency (10+ rules)

### Cross-File Validation
- **Story Keys:** CCC-CROSS-001, CCC-CROSS-002
- **Description:** Validator that checks consistency across settings, plugins, agents, skills, and marketplace; detects orphaned references; validates hook wiring
- **Impact:** Prevents configuration drift; ensures plugin/agent interdependencies are satisfied

### Marketplace Plugin Distribution
- **Story Keys:** CCC-MARKET-001, CCC-MARKET-002
- **Description:** Automated marketplace.json generation; plugin discovery; versioning and dependency resolution; GitHub Actions CI/CD integration
- **Impact:** Simplified distribution; automated version management; community plugin ecosystem

### Semantic Token Classification
- **Story Keys:** CCC-TOKENS-001, CCC-TOKENS-002
- **Description:** Identifiers reclassified as enumerations for config values; namespace detection; symbol highlighting; LSP semantic tokens protocol support
- **Impact:** Enhanced IDE syntax highlighting; improved code navigation

---

## Breaking Changes

### Analyzer API Refactor (v26.7.0)
- **Affected:** Custom analyzer implementations
- **Change:** `ReplayableAnalyzer` trait now requires `resume_from_checkpoint()` method
- **Migration:** See [Migration Guide — Analyzer Updates](#migration-guide)

### Ontology File Relocation
- **Affected:** Build scripts, CI/CD pipelines
- **Change:** `schema/claude-code-config.ttl` moved to canonical location; schema versioning now required
- **Migration:** Update build tooling to reference new path; see `ggen sync` documentation

### OCEL Event Format Version
- **Affected:** Event consumers, external audit systems
- **Change:** OCEL schema updated to v1.1; `object_id` field renamed to `object_ref` for clarity
- **Migration:** Update event parsers; backward-compatible reader available in `ocel_compat.rs`

---

## Bug Fixes

### Semantic Tokens — Enumeration Classification
- **Issue:** Identifiers in config values misclassified as `enumMember` instead of generic identifiers
- **Fix:** Revised token classifier to distinguish symbol vs. enum context
- **PR:** #42 (fix(semantic-tokens): identifiers were misclassified as enumMember)

### Scope Detection Edge Cases
- **Issue:** Global scope detection failed for nested `.claude/` directories
- **Fix:** Improved `is_global_scope()` path normalization
- **Affected:** Diagnostics source attribution

### Clippy Linter Warnings
- **Issue:** Dead code and needless borrow warnings in analyzers
- **Fix:** Removed unused code paths; optimized borrow ergonomics
- **PR:** #41 (fix(clippy): resolve dead_code and needless_borrow warnings in analyzers)

### Conformance Scoring Edge Cases
- **Issue:** Declare constraint evaluation incorrect for recursive references
- **Fix:** Constraint solver now handles cyclic rule dependencies correctly
- **Impact:** Accurate workspace health scores

---

## Dependencies

### Updated
- `lsp-max` → 26.7 (receipt chain, conformance v2)
- `tree-sitter` → 0.27 (parser improvements)
- `tokio` → 1.40 (async runtime refinements)
- `serde_json` → 1.1 (streaming event serialization)

### Added
- `blake3` → 1.1 (fast hashing for receipts)
- `uuid` → 1.0 (event correlation IDs)
- `chrono` → 0.4.38 (timestamp precision)

### Removed
- `parking_lot` (no longer needed; tokio sync primitives sufficient)

---

## Migration Guide

### For Custom Analyzer Implementations

If you've extended the analyzer API:

1. **Update trait implementation:**
   ```rust
   impl ReplayableAnalyzer for MyAnalyzer {
       async fn scan(&self, uri: &Url, content: &str) -> Result<Vec<Diagnostic>> { … }
       async fn resume_from_checkpoint(&self, uri: &Url, checkpoint: CheckpointState) 
           -> Result<Vec<Diagnostic>> { … }  // NEW
   }
   ```

2. **Build and test:**
   ```bash
   cargo build
   cargo test analyzers::my_analyzer
   ```

### For OCEL Event Consumers

If you consume OCEL logs:

1. **Update object reference handling:**
   - Old field name: `object_id`
   - New field name: `object_ref`
   - Use `ocel_compat::translate_v10_to_v11()` for backward compatibility

2. **Update parsers:**
   ```rust
   let event: OcelEvent = serde_json::from_str(json_str)?;
   // event.object_ref now used instead of object_id
   ```

### For Build Pipelines

1. **Update ggen command:**
   ```bash
   # Old: ggen sync --schema schema/old-location.ttl
   # New:
   ggen sync --schema schema/claude-code-config.ttl
   ```

2. **Verify generated files:**
   ```bash
   cargo check
   cargo clippy -- -D warnings
   ```

### For Marketplace Publishers

1. **Update marketplace.json schema:**
   - New field: `conformanceModel` (Declare constraint URI)
   - New field: `supportedEcosystems` (array of ecosystem names)

2. **Publish marketplace entry:**
   ```bash
   cargo build --release
   git push origin main  # Triggers marketplace auto-publish via GitHub Actions
   ```

---

## Acknowledgments

**Contributors:**
- Sean Chatman (primary developer)
- LSP community (conformance feedback)
- Ecosystem tower integration team

**Sponsors:**
- Anthropic Claude Code platform

**Special Thanks:**
- `lsp-max` framework maintainers for receipt chain and conformance v2 APIs
- Chicago TDD and cargo-cicd communities for integration patterns

---

## Appendix: Diagnostic Rules Summary

| Category | Count | Examples |
|----------|-------|----------|
| Permission Validation | 12 | tool access denied, invalid scope, credential storage |
| Scope Detection | 5 | global/project mismatch, nested scope confusion |
| Constraint Compliance | 18 | Declare process violations, state conflicts |
| Cross-File Consistency | 10+ | orphaned references, hook wiring, plugin interdependencies |
| Schema Validation | 8+ | field type mismatch, required field missing |
| **Total** | **40+** | |

---

## Document Metadata

- **Outline Version:** 1.0
- **Template Last Updated:** 2026-07-03
- **Next Review:** v26.7.4 (planned)
