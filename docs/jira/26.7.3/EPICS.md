# v26.7.3 Epics

**Release**: claude-code-config-lsp 26.7.3  
**Target Date**: 2026-09-20  
**Status**: CANDIDATE (Receipt Chain OPEN)

---

## Epic Summary

| Epic Key | Title | Description | Success Criteria | Related Features |
|----------|-------|-------------|------------------|------------------|
| CCCLSP-1 | OCEL Event Log Persistence | Implement persistent OCEL 2.0 event logging to disk for process mining and audit trails | ≥50 events/session recorded; valid OCEL 2.0 JSON schema; <1 MB/month; monthly log rotation | CCCLSP-1.1 (OcelPersister), CCCLSP-1.2 (Feature flag) |
| CCCLSP-2 | Receipt Chain Foundation | Create immutable receipt chain for workspace conformance snapshots with Merkle chain integrity validation | Receipts issued after each scan; chain verified on startup; <100ms query latency; tampering detection | CCCLSP-2.1 (Receipt struct), CCCLSP-2.2 (Chain validation), CCCLSP-2.3 (Merkle hashing) |
| CCCLSP-3 | Wasm4PM Constraint Integration | Wire wasm4pm-compat to Declare constraint registry; enable verification and visualization of constraints | All 8+ constraints exportable to PNML; <50ms violation detection; zero false positives; PNML model URI available | CCCLSP-3.1 (Constraint registry), CCCLSP-3.2 (verify_constraints), CCCLSP-3.3 (Virtual PNML doc) |
| CCCLSP-4 | Expanded Diagnostic Coverage | Grow diagnostic rule set from 15 to 40+ rules across all five config surfaces with comprehensive coverage | 40+ rules implemented; all surfaces covered (settings 8, CLAUDE.md 6, plugin 8, marketplace 6, frontmatter 7, hooks 5); zero rule conflicts; test case per rule | CCCLSP-4.1 (settings rules), CCCLSP-4.2 (CLAUDE.md rules), CCCLSP-4.3 (plugin rules), CCCLSP-4.4 (marketplace rules), CCCLSP-4.5 (frontmatter rules), CCCLSP-4.6 (hook rules) |
| CCCLSP-5 | Cross-File Reference Validation | Detect dangling references across config surfaces (plugin paths, CLAUDE.md links, hook wiring) | Symbol table built in single pass; workspace-relative & absolute paths handled; <100ms latency for 50-file workspace; broken refs reported as CCC-REF-001..005 | CCCLSP-5.1 (Symbol table), CCCLSP-5.2 (Reference scanner), CCCLSP-5.3 (Diagnostic emission) |
| CCCLSP-6 | Conformance Scoring Refinement | Implement nuanced, weighted conformance scoring with per-category impact and trend tracking | Per-category weights applied (critical 15pts, important 5pts, minor 1pt); workspace vs. project scoring distinguished; conformance vector published; 7-day trend data available | CCCLSP-6.1 (ConformanceVector), CCCLSP-6.2 (Weighted scoring), CCCLSP-6.3 (Trend tracking) |
| CCCLSP-7 | Marketplace Plugin Release | Publish official plugin to Claude Code marketplace with full distribution pipeline | Plugin installable from Claude Code UI; auto-updates on version bump; >100 installs in 30 days; binary artifacts for macOS/Linux/Windows; CI/CD auto-publish on tag | CCCLSP-7.1 (plugin.json update), CCCLSP-7.2 (marketplace.json finalization), CCCLSP-7.3 (GitHub release), CCCLSP-7.4 (CI/CD pipeline), CCCLSP-7.5 (README install guide) |
| CCCLSP-8 | Validate-Config Skill Polish | Make skill the canonical validation entry point with full automation and CLI interface | Skill uses LSP conformance scan (no duplication); --workspace, --file, --fix, --json args supported; --json output parseable by CI/CD; <2s latency for 50-file workspace; /verify integration complete | CCCLSP-8.1 (Skill CLI args), CCCLSP-8.2 (LSP invocation), CCCLSP-8.3 (JSON output), CCCLSP-8.4 (Result caching) |
| CCCLSP-9 | Virtual Documentation & Discoverability | Expose config schema and diagnostics as searchable virtual URIs within Claude Code UI | Virtual URIs implemented (schema/settings, schema/claude-md, health, declare/model, diagnostics/index); <50ms load time; all docs discoverable via command palette; hover integration working | CCCLSP-9.1 (schema/settings doc), CCCLSP-9.2 (schema/claude-md doc), CCCLSP-9.3 (Health report), CCCLSP-9.4 (PNML model), CCCLSP-9.5 (Diagnostics index) |
| CCCLSP-10 | Test Suite Expansion | Achieve 70%+ code coverage with unit, property-based, and integration tests; 85%+ mutation score | Unit tests: 5 files, 50+ assertions, >70% coverage for main modules; Property tests: 10+ properties via chicago-tdd-tools; Mutation score >85%; `cargo test` <10s; Mutation testing kills 90%+ of mutants | CCCLSP-10.1 (Analyzer units), CCCLSP-10.2 (Property tests), CCCLSP-10.3 (Conformance tests), CCCLSP-10.4 (Mutation suite), CCCLSP-10.5 (Integration LSP) |
| CCCLSP-11 | Conformance Receipt Chain Testing | Comprehensive test coverage for receipt chain with tampering detection and Merkle properties | Receipt issuance and chaining tested; hash consistency validated; tampering detection verified; persistence and replay tested; 100+ receipt operations validated | CCCLSP-11.1 (Issuance tests), CCCLSP-11.2 (Chain tests), CCCLSP-11.3 (Tampering detection), CCCLSP-11.4 (Persistence tests), CCCLSP-11.5 (Replay tests) |

---

## Epic Organization by Priority

### P0: Ecosystem Maturity (REQUIRED)
**Target**: 2026-07-30  
**Epics**: CCCLSP-1, CCCLSP-2, CCCLSP-3

These features unblock marketplace release and enable downstream integrations with OCEL, receipt chains, and Declare constraint verification.

### P1: Conformance Completeness (HIGH)
**Target**: 2026-08-15  
**Epics**: CCCLSP-4, CCCLSP-5, CCCLSP-6

Expand diagnostic coverage, enable cross-file validation, and refine conformance scoring for comprehensive configuration law enforcement.

### P2: Developer Experience (HIGH)
**Target**: 2026-08-30  
**Epics**: CCCLSP-7, CCCLSP-8, CCCLSP-9

Ship marketplace plugin, polish skill interface, and expose schema via virtual documentation for discoverability.

### P3: Testing & Quality (MUST COMPLETE)
**Target**: 2026-09-10  
**Epics**: CCCLSP-10, CCCLSP-11

Achieve comprehensive test coverage, mutation testing, and receipt chain validation for production readiness.

---

## Milestones & Deliverables

| Milestone | Date | Epics | Key Deliverables |
|-----------|------|-------|------------------|
| M1: Ecosystem Foundation | 2026-07-30 | CCCLSP-1, 2, 3 | OCEL persistence, receipt chain, wasm4pm integration |
| M2: Conformance Expansion | 2026-08-15 | CCCLSP-4, 5, 6 | 40+ diagnostic rules, cross-file validation, conformance vector |
| M3: Developer Experience | 2026-08-30 | CCCLSP-7, 8, 9 | Marketplace plugin, skill polish, virtual docs |
| M4: Testing & QA | 2026-09-10 | CCCLSP-10, 11 | 70% coverage, 85% mutation score, receipt chain tests |
| M5: Release | 2026-09-20 | All | GitHub release, marketplace submission, documentation |

---

## Dependency Graph

```
CCCLSP-1 (OCEL)
   └─→ CCCLSP-11 (Receipt Chain Testing)

CCCLSP-2 (Receipt Chain)
   └─→ CCCLSP-11 (Receipt Chain Testing)

CCCLSP-3 (Wasm4PM Integration)
   └─→ CCCLSP-10 (Test Suite)

CCCLSP-4 (Diagnostic Coverage)
   ├─→ CCCLSP-5 (Cross-File Validation)
   └─→ CCCLSP-10 (Test Suite)

CCCLSP-5 (Cross-File Validation)
   └─→ CCCLSP-6 (Conformance Scoring)

CCCLSP-6 (Conformance Scoring)
   └─→ CCCLSP-9 (Virtual Docs)

CCCLSP-7 (Marketplace Plugin)
   ├─→ CCCLSP-8 (Skill Polish)
   └─→ CCCLSP-10 (Test Suite)

CCCLSP-8 (Skill Polish)
   └─→ CCCLSP-9 (Virtual Docs)

CCCLSP-9 (Virtual Docs)
   └─→ All prior epics

CCCLSP-10 (Test Suite)
   ├─→ All prior epics
   └─→ Prerequisite for CCCLSP-11

CCCLSP-11 (Receipt Chain Testing)
   └─→ Final validation gate
```

---

## Success Metrics

### Adoption
- Plugin installs: >100 within 30 days
- Active users: >50 Claude Code sessions daily
- NPS: >40

### Quality
- Test coverage: >70%
- Mutation score: >85%
- Bug rate: <1/week

### Technical
- Diagnostics latency: <200ms (100-file workspace)
- LSP 3.18 compliance: 100%
- Receipt issuance: 100% (no failures)

### Ecosystem
- OCEL events logged: >1,000 per session
- Declare constraints verified: 100% of scans
- Cross-file references validated: 100%

---

## Document Metadata

**Generated**: 2026-07-03  
**Source**: v26.7.3-PRD-ARD.md  
**Epic Count**: 11  
**Feature Count**: 42 (rolled up)  
**Estimated Team-Weeks**: 20 (assuming 1 FTE over 5 weeks)
