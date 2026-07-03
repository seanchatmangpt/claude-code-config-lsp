# v26.7.3 Priority Matrix & Dependency Graph

**Release:** v26.7.3  
**Planning Date:** 2026-07-03  
**Release Target:** 2026-09-20

---

## Priority Levels

### P0: Ecosystem Maturity (CRITICAL)

**Rationale:** Foundation features that enable all downstream work. All P0 features must complete before any P1/P2/P3 features can begin.

**Features:**
1. OCEL Event Log Persistence (CCC-2601) — Event logging infrastructure
2. Receipt Chain Foundation (CCC-2604) — Conformance audit trail
3. Wasm4PM Constraint Integration (CCC-2607) — Constraint verification

**Completion Target:** 2026-07-30 (Milestone M1)

**Success Metrics:**
- All 3 features fully implemented and tested
- No dependencies on P1/P2/P3
- Ready to unblock M2

---

### P1: Conformance Completeness (HIGH)

**Rationale:** Expands diagnostic capabilities and scoring accuracy. Unblocked after P0.

**Features:**
4. Expanded Diagnostic Coverage (CCC-2611) — 40+ diagnostic rules
5. Cross-File Reference Validation (CCC-2615) — Dangling link detection
6. Conformance Scoring Refinement (CCC-2618) — Weighted scoring + trends

**Completion Target:** 2026-08-15 (Milestone M2)

**Dependencies:** CCC-2601, CCC-2604, CCC-2607 (all P0 features)

**Success Metrics:**
- All 40+ diagnostic rules live and tested
- Cross-file validation working correctly
- Weighted scoring algorithm validated

---

### P2: Developer Experience (HIGH)

**Rationale:** Improves usability and marketplace readiness. Can partially overlap with P1.

**Features:**
7. Marketplace Plugin Release (CCC-2621) — Plugin distribution
8. Validate-Config Skill Polish (CCC-2625) — Skill automation
9. Virtual Documentation & Discoverability (CCC-2628) — Schema browsing

**Completion Target:** 2026-08-30 (Milestone M3)

**Dependencies:** 
- CCC-2600 (Epic, for context)
- CCC-2611 (P1, for diagnostic codes)
- Strong recommendation: CCC-2604, CCC-2607 for full feature set

**Overlap Opportunity:** Can start marketplace prep (CCC-2622) during M2

**Success Metrics:**
- Plugin ready for marketplace
- Skill fully automated
- Virtual docs complete and discoverable

---

### P3: Testing & Quality (MUST COMPLETE)

**Rationale:** Ensures code quality and release readiness. Can run in parallel with other milestones; execution accelerates in M4.

**Features:**
10. Test Suite Expansion (CCC-2631) — 70%+ coverage
11. Conformance Receipt Chain Testing (CCC-2636) — Receipt chain tests

**Completion Target:** 2026-09-10 (Milestone M4)

**Dependencies:** 
- CCC-2601 (OCEL events, for trend testing)
- CCC-2604 (Receipt chain, for receipt testing)
- All other features (for integration testing)

**Parallel Work:** Unit tests can start as features land (no wait for P0)

**Success Metrics:**
- Coverage > 70%
- Mutation score > 85%
- All tests passing and deterministic

---

## Dependency Graph

### Hierarchical View

```
CCC-2600 (Epic: v26.7.3)
  │
  ├─ P0 (Ecosystem Foundation) — M1
  │   ├─ CCC-2601 (OCEL persistence) — CRITICAL PATH
  │   │   ├─ CCC-2602 (OcelPersister)
  │   │   └─ CCC-2603 (Feature flag)
  │   │
  │   ├─ CCC-2604 (Receipt chain) — CRITICAL PATH
  │   │   ├─ CCC-2605 (Receipt struct)
  │   │   └─ CCC-2606 (Issuance & validation)
  │   │
  │   └─ CCC-2607 (Wasm4PM) — CRITICAL PATH
  │       ├─ CCC-2608 (Feature integration)
  │       ├─ CCC-2609 (Verification logic)
  │       └─ CCC-2610 (PNML export)
  │
  ├─ P1 (Conformance Completion) — M2
  │   ├─ CCC-2611 (40+ rules)
  │   │   ├─ CCC-2612 (Add to ontology)
  │   │   ├─ CCC-2613 (ggen sync)
  │   │   └─ CCC-2614 (Test fixtures)
  │   │
  │   ├─ CCC-2615 (Cross-file refs)
  │   │   ├─ CCC-2616 (Workspace scanner)
  │   │   └─ CCC-2617 (Validation)
  │   │
  │   └─ CCC-2618 (Scoring refinement)
  │       ├─ CCC-2619 (ConformanceVector)
  │       └─ CCC-2620 (Trends & health)
  │
  ├─ P2 (Developer Experience) — M3
  │   ├─ CCC-2621 (Marketplace)
  │   │   ├─ CCC-2622 (Update manifests) [can start week of 2026-08-15]
  │   │   ├─ CCC-2623 (GitHub release)
  │   │   └─ CCC-2624 (Submit to marketplace)
  │   │
  │   ├─ CCC-2625 (Skill polish)
  │   │   ├─ CCC-2626 (Argument parsing)
  │   │   └─ CCC-2627 (Caching & formatting)
  │   │
  │   └─ CCC-2628 (Virtual docs)
  │       ├─ CCC-2629 (URI routing)
  │       └─ CCC-2630 (Schema rendering)
  │
  ├─ P3 (Testing & Quality) — M4 (starts in parallel, executes M4)
  │   ├─ CCC-2631 (Test suite)
  │   │   ├─ CCC-2632 (Unit tests) [can start week of 2026-07-30]
  │   │   ├─ CCC-2633 (Property tests)
  │   │   ├─ CCC-2634 (Integration tests)
  │   │   └─ CCC-2635 (Coverage & mutation)
  │   │
  │   └─ CCC-2636 (Receipt chain testing)
  │       └─ CCC-2637 (Test suite)
  │
  └─ M5 (Release) — M5
      └─ CCC-2638 (Release umbrella)
          ├─ CCC-2639 (CHANGELOG & notes)
          ├─ CCC-2640 (README & docs)
          └─ CCC-2641 (Final QA & sign-off)
```

### Cross-Milestone Dependencies

```
M1 (P0) ──→ M2 (P1) ──→ M3 (P2) ──→ M4 (P3) ──→ M5 (Release)
  │           │          │
  └─ CCC-2632 (unit tests start here, finish M4)
  └─ CCC-2633 (property tests start here, finish M4)
              └─ CCC-2622 (marketplace prep can start)
                         └─ CCC-2639 (changelog draft)
```

---

## Critical Path

**Definition:** The longest sequence of dependent tasks that determines overall project duration.

### Critical Path Timeline

```
2026-07-03:  Project kickoff
              ↓
2026-07-20:  M1 P0-1: CCC-2605 (Receipt struct) due
              ↓
2026-07-25:  M1 P0-1: CCC-2602 (OcelPersister) due
              ↓
2026-07-30:  M1 Complete: OCEL + Receipt + Wasm4PM ready
              ↓
2026-08-10:  M2 P0-1: CCC-2613 (ggen sync & analyzers) due
2026-08-10:  M2 P1-2: CCC-2619 (ConformanceVector) due
              ↓
2026-08-15:  M2 Complete: 40+ rules + cross-file refs + scoring ready
2026-08-15:  P2 Overlap: CCC-2622 (update manifests) starts
              ↓
2026-08-30:  M3 Complete: Marketplace + skill + virtual docs ready
              ↓
2026-09-01:  M4 P3-1: CCC-2632 (unit tests) due
              ↓
2026-09-10:  M4 Complete: 70% coverage + mutation testing done
              ↓
2026-09-15:  M5 P3: CCC-2639 (CHANGELOG) due
              ↓
2026-09-20:  M5 Complete: Release signed off, marketplace live
```

**Critical Path Length:** 4.3 months (July 3 → Sept 20)

**Critical Path Items (cannot slip):**
1. CCC-2601 (OCEL) — enables M2
2. CCC-2604 (Receipt) — enables M2 and M4
3. CCC-2607 (Wasm4PM) — enables M2
4. CCC-2611 (Rules) — enables M2 completion
5. CCC-2631 (Testing) — enables M4

**Slack Time (can slip without delaying release):**
- Virtual docs (CCC-2628) — low priority for release (ship docs later if needed)
- Some unit tests (CCC-2632) — can reduce scope if time constrained

---

## Parallelization Opportunities

### Week of 2026-07-30 (M1 wraps, M2 starts)

**Parallel Streams:**
1. **P1 Features (CCC-2611 to CCC-2620):**
   - Teams: Rules team works on CCC-2612, CCC-2613, CCC-2614
   
2. **P3 Testing (CCC-2631 to CCC-2636):**
   - Team: QA writes unit tests for existing features (no wait for P1)
   - Can start testing P0 features immediately

3. **P2 Marketplace (prep only):**
   - Team: Update plugin manifests (CCC-2622) — lightweight, fast

### Week of 2026-08-15 (M2 wraps, M3 starts)

**Parallel Streams:**
1. **P2 Features (CCC-2625, CCC-2628):**
   - Skills team: Implement skill arguments and modes (CCC-2626)
   - Docs team: Render schema documentation (CCC-2630)

2. **P3 Testing (continued):**
   - QA: Write integration tests for P1 features (CCC-2634)

3. **P2 Marketplace (execution):**
   - Release team: Build GitHub binaries (CCC-2623)
   - Prepare marketplace submission (CCC-2624 groundwork)

### Week of 2026-08-30 (M3 wraps, M4 starts)

**Parallel Streams:**
1. **P3 Testing (execution):**
   - QA: Run full test suite, measure coverage (CCC-2635)
   - QA: Run mutation testing, analyze results

2. **M5 Preparation (early start):**
   - Docs team: Draft CHANGELOG (CCC-2639 outline)
   - Release team: Prepare release notes template

---

## Issue Priority by Work Type

### Frontend Changes (API, UI, CLI)

**High Priority:**
- CCC-2626 (Skill arguments) — user-facing
- CCC-2629, CCC-2630 (Virtual docs) — discoverability

**Medium Priority:**
- CCC-2622 (Marketplace updates) — one-time setup

### Backend Changes (Logic, Storage)

**High Priority (Critical Path):**
- CCC-2602, CCC-2603 (OCEL) — required for everything
- CCC-2605, CCC-2606 (Receipt chain) — required for everything
- CCC-2612, CCC-2613, CCC-2614 (Rules) — core functionality

**High Priority (Feature Completeness):**
- CCC-2616, CCC-2617 (Cross-file refs) — data integrity
- CCC-2619, CCC-2620 (Scoring) — user-facing metric

### Testing

**High Priority (M4 Critical Path):**
- CCC-2632 (Unit tests) — must achieve > 70% coverage
- CCC-2635 (Mutation testing) — must achieve > 85% mutation score
- CCC-2637 (Receipt tests) — foundation feature validation

**Medium Priority:**
- CCC-2633 (Property tests) — nice-to-have if time constrained
- CCC-2634 (Integration tests) — last-mile validation

### Documentation

**High Priority (Release):**
- CCC-2639 (CHANGELOG) — required for GitHub release
- CCC-2640 (README) — required for marketplace

**Medium Priority:**
- CCC-2630 (Virtual docs) — can ship docs in v26.7.4 if needed
- MILESTONES.md, PRIORITY_MATRIX.md — internal planning only

---

## Risk Mitigation by Priority

### P0 Risks (Ecosystem Foundation)

| Risk | Mitigation | Contingency |
|------|-----------|-------------|
| OCEL I/O overhead | Feature flag (default off) | Disable OCEL in release if < 1s latency |
| Receipt chain hash collision | blake3 has negligible collision probability | Use sha256 if blake3 fails |
| Wasm4PM timeout on large logs | Incremental verification + cache | Disable wasm4pm verification for logs > 100MB |

### P1 Risks (Conformance)

| Risk | Mitigation | Contingency |
|------|-----------|-------------|
| Diagnostic rule conflicts | Thorough review (CCC-2614) | Remove conflicting rules post-release |
| False positives | Extensive test fixtures | Add rule exemptions per workspace |
| Performance regression | Benchmark before/after | Optimize hot paths or defer to v26.7.4 |

### P2 Risks (Developer Experience)

| Risk | Mitigation | Contingency |
|------|-----------|-------------|
| Marketplace approval delay | Pre-submit at M2 completion | Ship to GitHub releases only (no marketplace) |
| Skill I/O errors | Graceful error handling + caching | Fallback to LSP direct API |
| Virtual doc rendering bugs | Test all URIs before release | Remove virtual docs, link to reference.md |

### P3 Risks (Testing & Quality)

| Risk | Mitigation | Contingency |
|------|-----------|-------------|
| Low test coverage | Automated coverage checks in CI | Reduce coverage target to 60% if blocked |
| Flaky tests | No time-dependent tests, deterministic fixtures | Exclude flaky tests from CI, manual validation |
| Mutation test timeout | Set timeout per mutation | Run mutation testing post-release (v26.7.1) |

---

## Dependency Matrix

**Format:** Rows are features that depend on columns.

```
                CCC-2601  CCC-2604  CCC-2607  CCC-2611  CCC-2615  CCC-2618  CCC-2621  CCC-2625  CCC-2628  CCC-2631  CCC-2636
                (OCEL)    (Receipt) (Wasm4PM) (Rules)   (Refs)    (Score)   (Market)  (Skill)   (Docs)    (Tests)   (RcptTst)
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
CCC-2604        │                                                                                                          │
(Receipt)       └─ depends on OCEL for event ordering

CCC-2607        │                                                                                                          │
(Wasm4PM)       └─ depends on OCEL for event stream

CCC-2611        ✓         ✓         ✓                                                                                      │
(Rules)         └─ depends on all P0 (for context)

CCC-2615        ✓         ✓         ✓         ✓                                                                            │
(Refs)          └─ depends on P0 + P1 rules

CCC-2618        ✓         ✓         ✓                                                                                      │
(Score)         └─ depends on P0 (OCEL for trends)

CCC-2625        ✓         ✓         ✓         ✓                                                                            │
(Skill)         └─ depends on P0 + P1 for validation APIs

CCC-2628        ✓         ✓         ✓         ✓                                                                            │
(Docs)          └─ depends on P0 + P1 for schema

CCC-2631        ✓         ✓         ✓         ✓         ✓         ✓         │         │         │                       │
(Tests)         └─ depends on everything (tests entire release)

CCC-2636        ✓         ✓                                                                                      │
(RcptTests)     └─ depends on receipt feature (CCC-2604)

CCC-2639        ✓         ✓         ✓         ✓         ✓         ✓         ✓         ✓         ✓         ✓         ✓
(CHANGELOG)     └─ depends on all features (documents everything)

Legend:
  ✓ = Blocks this feature
  │ = Data dependency (this feature inputs data from feature above)
```

---

## Issue Allocation by Team

### Recommended Team Structure

**Infrastructure Team (2 people):**
- Lead: Sean Chatman
- Responsibilities: P0 features (OCEL, receipt, wasm4pm)
- Issues: CCC-2601 to CCC-2610
- Timeline: M1 (2026-07-30)

**Diagnostics Team (2 people):**
- Lead: TBD
- Responsibilities: P1 features (rules, cross-file refs, scoring)
- Issues: CCC-2611 to CCC-2620
- Timeline: M2 (2026-08-15)

**Developer Tools Team (2 people):**
- Lead: TBD
- Responsibilities: P2 features (marketplace, skill, docs)
- Issues: CCC-2621 to CCC-2630
- Timeline: M3 (2026-08-30)

**QA & Release Team (2 people):**
- Lead: TBD
- Responsibilities: P3 testing, release management
- Issues: CCC-2631 to CCC-2641
- Timeline: M4 (2026-09-10) to M5 (2026-09-20)

---

## Milestone Gate Criteria

### M1 Gate (2026-07-30)

**Must Pass:**
- ✅ CCC-2601 (OCEL persistence) — 100% done
- ✅ CCC-2604 (Receipt chain) — 100% done
- ✅ CCC-2607 (Wasm4PM) — 100% done
- ✅ All tests passing
- ✅ No critical bugs

**If Failed:**
- Extend M1 timeline by 1 week
- Delay M2 start accordingly
- Re-plan M2-M5 dates

---

### M2 Gate (2026-08-15)

**Must Pass:**
- ✅ CCC-2611 (40+ rules) — 100% done
- ✅ CCC-2615 (Cross-file refs) — 100% done
- ✅ CCC-2618 (Scoring) — 100% done
- ✅ All tests passing
- ✅ < 200ms latency for 100-file workspace

**If Failed:**
- Extend M2 timeline by 1 week
- Prioritize rule coverage over edge cases
- Remove lowest-priority rules if needed

---

### M3 Gate (2026-08-30)

**Must Pass:**
- ✅ CCC-2621 (Marketplace) — submission ready
- ✅ CCC-2625 (Skill) — feature-complete
- ✅ CCC-2628 (Virtual docs) — all URIs working
- ✅ GitHub release published

**If Failed:**
- Ship to GitHub releases only (no marketplace)
- Marketplace approval can happen post-release (v26.7.1)

---

### M4 Gate (2026-09-10)

**Must Pass:**
- ✅ Coverage > 70%
- ✅ Mutation score > 85%
- ✅ All tests passing and deterministic
- ✅ CCC-2631 (Test suite) — 100% done
- ✅ CCC-2636 (Receipt tests) — 100% done

**If Failed:**
- Fix coverage gaps in M5 (before release)
- Extend M4 by 1 week if coverage < 60%

---

### M5 Gate (2026-09-20)

**Must Pass:**
- ✅ All M1-M4 issues marked "Done"
- ✅ CHANGELOG and README updated
- ✅ GitHub release live
- ✅ Marketplace submission approved (or pending)
- ✅ Zero critical bugs in release

**If Failed:**
- Hold release (push to 2026-09-27)
- Fix critical bugs in hotfix branch
- Re-test before going live

---

**Document Version:** 26.7.3-Priority-Matrix-v1  
**Last Updated:** 2026-07-03  
**Next Review:** 2026-07-30 (M1 completion)
