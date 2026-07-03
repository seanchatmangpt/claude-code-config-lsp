# v26.7.3 Milestones — Detailed Breakdown

**Release Epic:** CCC-2600  
**Release Target:** 2026-09-20

---

## Milestone Overview

v26.7.3 is divided into 5 sequential milestones plus 1 umbrella epic:

| Milestone | Target Date | Focus | Issues | Status |
|-----------|-------------|-------|--------|--------|
| **M1: Ecosystem Foundation** | 2026-07-30 | OCEL, Receipt Chain, Wasm4PM | 9 (P0) | ACTIVE |
| **M2: Conformance Expansion** | 2026-08-15 | Rules, References, Scoring | 15 (P1) | PLANNED |
| **M3: Developer Experience** | 2026-08-30 | Marketplace, Skill, Docs | 12 (P2) | PLANNED |
| **M4: Testing & QA** | 2026-09-10 | Coverage, Mutation, Receipts | 6 (P3) | PLANNED |
| **M5: Release** | 2026-09-20 | Changelog, Final QA, Approval | 4 | PLANNED |

---

## M1: Ecosystem Foundation (2026-07-30)

**Goal:** Complete core infrastructure integrations and enable event logging.

**Priority:** P0 (CRITICAL — blocks all downstream work)

### Features

#### 1. OCEL Event Log Persistence (CCC-2601)

**Owner:** sean.chatman

**Description:** Implement persistent OCEL 2.0 event logging to disk.

**Subtasks:**
- CCC-2602: Implement OcelPersister struct (2026-07-25)
- CCC-2603: Add feature flag and configuration (2026-07-27)

**Acceptance Criteria:**
- Events serialized to `~/.claude/logs/claude-code-config-lsp/ocel-YYYY-MM.jsonl`
- ≥50 events per session
- Monthly rotation enabled
- Feature flag: `ocel-persistence` (default: off)
- OCEL 2.0 schema validation passing
- File size < 1 MB/month (avg workspace)

**Dependencies:** None (chrono, serde_json already in deps)

**Risks:**
- Logging overhead could slow diagnostics → mitigated by feature flag (default off)
- File I/O errors on network mounts → handled gracefully, events dropped (non-blocking)

**Success Metrics:**
- ≥1000 events logged per typical agent session
- No performance regression (< 5ms overhead per event)
- File rotation working on month boundary

---

#### 2. Receipt Chain Foundation (CCC-2604)

**Owner:** sean.chatman

**Description:** Create immutable Merkle-chain receipt system for conformance audit trail.

**Subtasks:**
- CCC-2605: Receipt struct and hash computation (2026-07-20)
- CCC-2606: Receipt issuance and chain validation (2026-07-28)

**Acceptance Criteria:**
- Receipt struct: workspace_id, timestamp (UTC), score, checksums, signer, prior_receipt_hash
- Hash: blake3(workspace_id + timestamp + score + checksums)
- Receipts stored in `~/.claude/receipts/` (immutable)
- Virtual URI: `claude-config://receipts/<hash>`
- Chain validation on startup (detect tampering)
- Receipt available via URI < 100ms
- Merkle chain property verified in tests

**Dependencies:** None (blake3 already in deps)

**Risks:**
- Tampering detection complexity → tested thoroughly in M4
- Disk space on CI systems → receipts pruned after 90 days

**Success Metrics:**
- 100% of scans issue receipts (no failures)
- Chain integrity verified on 100+ operations
- Zero tampering detected in honest workflows

---

#### 3. Wasm4PM Constraint Integration (CCC-2607)

**Owner:** sean.chatman

**Description:** Wire wasm4pm-compat to constraint registry and enable verification.

**Subtasks:**
- CCC-2608: Integrate wasm4pm-compat feature (2026-07-22)
- CCC-2609: Implement constraint verification (2026-07-28)
- CCC-2610: Export PNML and virtual docs (2026-07-29)

**Acceptance Criteria:**
- Feature flag: `wasm4pm-compat` (optional)
- 8+ Declare constraints from ontology exported to PNML
- Constraint violations detectable from OCEL < 50ms latency
- Violations published as INFO diagnostics (non-blocking)
- Virtual URIs:
  - `claude-config://declare/model.xml` (PNML)
  - `claude-config://declare/report` (markdown summary)
- No false positives
- PNML valid against Declare schema

**Dependencies:** wasm4pm-compat 26.6, CCC-2601 (OCEL events)

**Risks:**
- Constraint checking performance on large logs → incremental verification + caching
- PNML export correctness → validated against Declare spec

**Success Metrics:**
- All 8+ constraints exportable to PNML
- < 50ms constraint verification latency
- 0 false positive violations

---

### M1 Success Criteria

- ✅ All 9 issues marked "Done"
- ✅ `cargo test` passes (including new OCEL/receipt/constraint tests)
- ✅ `cargo build --release` succeeds
- ✅ OCEL events logged to disk in valid format
- ✅ Receipt chain validated and Merkle properties held
- ✅ Wasm4PM constraints exported and verified
- ✅ No performance regression (< 10% latency increase for typical scan)

### M1 Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Feature flag overhead | Low | Medium | Design feature flag to be zero-cost when disabled |
| I/O performance | Medium | High | Use async file I/O; write batches to reduce syscalls |
| Constraint timeout | Medium | High | Incremental verification; cache constraint results |

---

## M2: Conformance Expansion (2026-08-15)

**Goal:** Expand diagnostic rules, validate cross-file references, refine scoring.

**Priority:** P1 (HIGH — unblocked after M1)

### Features

#### 4. Expanded Diagnostic Coverage (CCC-2611)

**Owner:** sean.chatman

**Description:** Grow diagnostic rules from ~15 to 40+.

**Subtasks:**
- CCC-2612: Add 25+ rules to ontology (2026-07-31)
- CCC-2613: Run ggen sync and implement analyzer logic (2026-08-10)
- CCC-2614: Add test fixtures and verify coverage (2026-08-14)

**Rules Added (25+):**

**settings.json (8 rules):**
- CCC-JSON-101: Model ID validation
- CCC-JSON-102: Permission cycles detection
- CCC-JSON-103: Hook reference validation
- CCC-JSON-104: Env var syntax
- CCC-JSON-105: Deprecated model detection
- CCC-JSON-106: Duplicate permissions
- CCC-JSON-107: Invalid hook event names
- CCC-JSON-108: Non-existent tool references

**CLAUDE.md (6 rules):**
- CCC-MD-201 to CCC-MD-206: Section structure, link validation, frontmatter syntax

**plugin.json (8 rules):**
- CCC-PLUGIN-301 to CCC-PLUGIN-308: Command validation, paths, dependencies, versions

**marketplace.json (6 rules):**
- CCC-MKT-401 to CCC-MKT-406: Name uniqueness, URL reachability, version schema

**Frontmatter (7 rules):**
- CCC-FM-501 to CCC-FM-507: Required fields, enums, arrays, dates, URLs, markdown

**Hook Scripts (5 rules):**
- CCC-HOOK-601 to CCC-HOOK-605: Bash syntax, shebang, exit codes, permissions, signals

**Acceptance Criteria:**
- All 40+ rules defined in ontology
- Rules non-conflicting (no contradictions)
- Each rule has ≥1 test fixture
- Rules discoverable via `/code-review`
- Coverage: settings (8/8), CLAUDE.md (6/6), plugin (8/8), marketplace (6/6), frontmatter (7/7), hooks (5/5)

**Dependencies:** None (but M1 should be mostly done)

**Success Metrics:**
- 40+ diagnostic codes live and functional
- Zero false positives in common workspaces
- All rules tested and documented

---

#### 5. Cross-File Reference Validation (CCC-2615)

**Owner:** sean.chatman

**Description:** Detect dangling references across config surfaces.

**Subtasks:**
- CCC-2616: Implement workspace scanner (2026-08-08)
- CCC-2617: Validate references and issue diagnostics (2026-08-13)

**Acceptance Criteria:**
- `Backend::scan_workspace_references()` builds symbol table
- Symbol table includes: file paths, skill names, agent names, hook names
- CLAUDE.md "See Also" links validated
- plugin.json skill/agent/MCP paths validated
- settings.json hook references validated
- Diagnostic codes: CCC-REF-001 through CCC-REF-005
- < 100ms latency for 50-file workspace
- No false positives

**Dependencies:** CCC-2611 (diagnostic codes)

**Success Metrics:**
- Symbol table built correctly for various workspace sizes
- All 5 reference types (See Also, skills, agents, hooks, MCPs) validated
- Broken references detected with zero false positives

---

#### 6. Conformance Scoring Refinement (CCC-2618)

**Owner:** sean.chatman

**Description:** Implement weighted conformance scoring and trend tracking.

**Subtasks:**
- CCC-2619: ConformanceVector and weighted scoring (2026-08-10)
- CCC-2620: Trend tracking and health virtual doc (2026-08-14)

**Acceptance Criteria:**
- ConformanceVector maps (surface_type, severity) → impact
- Weights: critical (15pts), important (5pts), minor (1pt)
- Workspace score = global + min(project scores) (conservative)
- Score always in [0, 100]
- Trend data: 7-day history
- Virtual URI: `claude-config://health` (JSON + markdown)
- < 50ms response time for health query
- v26.6 score 85 → v26.7.3 score 80-90 (verify weighted model)

**Dependencies:** CCC-2601 (OCEL for trend data)

**Success Metrics:**
- Weighted scoring algorithm working correctly
- Trend data accumulated and queryable
- Conformance UI reflects nuanced scoring (not binary)

---

### M2 Success Criteria

- ✅ All 15 P1 issues marked "Done"
- ✅ 40+ diagnostic rules live and tested
- ✅ Cross-file validation working end-to-end
- ✅ Weighted conformance scoring implemented
- ✅ `cargo test` passes (all tests including new rules)
- ✅ No regression in diagnostics latency (still < 200ms for 100-file workspace)

---

## M3: Developer Experience (2026-08-30)

**Goal:** Ship marketplace-ready plugin with improved DX and documentation.

**Priority:** P2 (HIGH)

### Features

#### 7. Marketplace Plugin Release (CCC-2621)

**Owner:** sean.chatman

**Subtasks:**
- CCC-2622: Update plugin.json and marketplace.json (2026-08-20)
- CCC-2623: GitHub release and binaries (2026-08-27)
- CCC-2624: Marketplace submission (2026-09-05)

**Acceptance Criteria:**
- plugin.json version = 26.7.3
- marketplace.json listing complete and accurate
- GitHub release tagged v26.7.3
- Binaries: macOS (x86_64, arm64), Linux (x86_64), Windows (x86_64)
- GitHub CI/CD auto-publishes on tag
- Plugin installable from Claude Code marketplace UI
- > 100 installations within 30 days of release

**Dependencies:** M1 + M2 features should be feature-complete

**Success Metrics:**
- Plugin live in marketplace
- Installation tracking shows growth
- No marketplace approval blockers

---

#### 8. Validate-Config Skill Polish (CCC-2625)

**Owner:** sean.chatman

**Subtasks:**
- CCC-2626: Argument parsing and modes (2026-08-22)
- CCC-2627: Caching and output formatting (2026-08-28)

**Acceptance Criteria:**
- Skill invokes LSP server (no duplicate logic)
- Arguments: `--workspace`, `--file <path>`, `--fix`, `--json`
- Output modes: table (default), JSON (--json)
- Caching: 5-minute TTL
- Performance: < 2s for 50-file workspace
- JSON output parseable by CI/CD
- Integrated with `/verify` command

**Dependencies:** CCC-2611 (all diagnostics available)

**Success Metrics:**
- Skill callable without fallback
- Agents rely on skill for validation
- CI/CD integrates skill into workflows

---

#### 9. Virtual Documentation & Discoverability (CCC-2628)

**Owner:** sean.chatman

**Subtasks:**
- CCC-2629: URI routing and virtual doc handlers (2026-08-24)
- CCC-2630: Schema docs and diagnostic reference (2026-08-28)

**Acceptance Criteria:**
- Virtual URIs implemented:
  - `claude-config://schema/settings`
  - `claude-config://schema/claude-md`
  - `claude-config://health`
  - `claude-config://declare/model`
  - `claude-config://diagnostics/index`
- Each URI loads < 50ms
- Markdown rendering with hover metadata
- Searchable from Claude Code command palette
- Diagnostic codes link to reference

**Dependencies:** All config surfaces must be documented

**Success Metrics:**
- All 5 virtual URIs working
- Schema docs auto-generated from ontology
- Zero broken links

---

### M3 Success Criteria

- ✅ All 12 P2 issues marked "Done"
- ✅ Plugin ready for marketplace
- ✅ Skill fully automated
- ✅ Virtual docs complete and discoverable
- ✅ GitHub release published
- ✅ Installation tracked and counted

---

## M4: Testing & QA (2026-09-10)

**Goal:** Comprehensive test coverage and mutation testing.

**Priority:** P3 (MUST COMPLETE)

### Features

#### 10. Test Suite Expansion (CCC-2631)

**Owner:** sean.chatman

**Subtasks:**
- CCC-2632: Unit tests for analyzers (2026-09-01)
- CCC-2633: Property-based tests (2026-09-04)
- CCC-2634: Integration tests (2026-09-07)
- CCC-2635: Coverage and mutation testing (2026-09-09)

**Acceptance Criteria:**
- Unit tests: 50+ assertions across 5 analyzers
- Property tests: 10+ properties with 100+ test cases each
- Integration tests: LSP protocol, conformance, marketplace
- Coverage: > 70% overall, > 80% analyzers
- Mutation score: > 85%
- `cargo test` completes < 10s
- All tests deterministic (no flakes)

**Dependencies:** None (can run in parallel with other features)

**Success Metrics:**
- High test coverage achieved
- Mutation testing shows most code paths exercised
- Zero known bugs in codebase

---

#### 11. Receipt Chain Testing (CCC-2636)

**Owner:** sean.chatman

**Subtasks:**
- CCC-2637: Receipt chain test suite (2026-09-08)

**Acceptance Criteria:**
- Receipt issuance and chaining tested
- Receipt hash consistency verified
- Tampering detection tested (corrupt receipt → invalid chain)
- Receipt persistence and replay tested
- 100+ receipt operations verified
- Chain integrity properties hold

**Dependencies:** CCC-2604 (Receipt Chain Foundation)

**Success Metrics:**
- All receipt chain properties verified
- Tampering reliably detected
- Chain can be replayed correctly

---

### M4 Success Criteria

- ✅ All 6 P3 issues marked "Done"
- ✅ Coverage > 70%
- ✅ Mutation score > 85%
- ✅ All tests passing and deterministic
- ✅ No regressions from previous milestones
- ✅ Performance verified (< 200ms for 100-file workspace)

---

## M5: Release (2026-09-20)

**Goal:** Final documentation, QA, and marketplace approval.

**Priority:** CRITICAL (final milestone)

### Tasks

#### Release Checklist

- CCC-2638: Umbrella story
  - CCC-2639: CHANGELOG and release notes (2026-09-15)
  - CCC-2640: README and installation docs (2026-09-18)
  - CCC-2641: Final QA and sign-off (2026-09-20)

**Acceptance Criteria (CCC-2641):**
- All M1-M4 issues marked "Done"
- All tests passing
- Code coverage > 70%
- Mutation score > 85%
- LSP 3.18 compliance verified
- Plugin installable from marketplace
- No known critical bugs
- Performance verified
- Documentation complete

### M5 Success Criteria

- ✅ v26.7.3 tagged in GitHub
- ✅ Binaries released and verified
- ✅ Plugin live in marketplace
- ✅ Installation count tracked
- ✅ Release notes published
- ✅ Documentation updated

---

## Critical Path Analysis

**Longest Path (determines overall duration):**

```
Start → M1 (9 issues, 4 weeks) 
      → M2 (15 issues, 3 weeks)
      → M3 (12 issues, 4 weeks)
      → M4 (6 issues, 3 weeks)
      → M5 (4 issues, 1 week)
      → End

Total: ~15 weeks (July 30 → September 20)
```

**Critical Dependencies:**
1. P0 (M1) must complete before P1 (M2)
2. Marketplace prep can overlap with P1 (week of 2026-08-15)
3. Testing (P3) can start early (unit tests as features land)

**Parallel Opportunities:**
- Documentation drafting in M2 (polish in M3/M4)
- GitHub release prep in M3 (finalize in M5)
- QA test design in M3 (execution in M4)

---

## Risk & Mitigation Summary

| Milestone | Risk | Mitigation |
|-----------|------|-----------|
| M1 | OCEL I/O overhead | Feature flag (default off), async I/O |
| M1 | Constraint timeout | Incremental verification, caching |
| M2 | Rule conflicts | Thorough review in CCC-2614 |
| M2 | False positives | Extensive test fixtures |
| M3 | Marketplace approval delay | Pre-submit at M2 completion |
| M3 | Plugin distribution issues | Test binary installs early |
| M4 | Flaky tests | Determinism checks, no time-based tests |
| M5 | Last-minute bugs | Freeze code 1 week before release |

---

**Document Version:** 26.7.3-Milestones-v1  
**Last Updated:** 2026-07-03
