# Risk Register — claude-code-config-lsp v26.7.3

**Release**: v26.7.3  
**Date**: 2026-07-03  
**Owner**: Sean Chatman (xpointsh@gmail.com)

---

## Risk Summary

| Risk ID | Risk | Impact | Likelihood | Mitigation | Owner | Epic Link |
|---------|------|--------|------------|-----------|-------|-----------|
| R-001 | OCEL log disk bloat in high-volume agent sessions | HIGH (>1 GB/month, disk space exhaustion) | MEDIUM | Implement log rotation policy (monthly), compression, and disk quota enforcement. Feature flag default: disabled. | Sean Chatman | Ecosystem Maturity |
| R-002 | Receipt chain integrity violation (hash tampering, fork detection) | HIGH (audit trail corruption, compliance violation) | LOW | Use blake3 immutable hashing, validate Merkle chain on startup, store receipts in read-only directory with OS-level protections. | Sean Chatman | Receipt Chain Foundation |
| R-003 | Wasm4PM constraint verification performance degradation | MEDIUM (latency spike in IDE, user frustration) | MEDIUM | Lazy-load wasm4pm-compat, cache constraint model, verify in background thread with <50ms SLA. Monitor via telemetry. | Sean Chatman | Wasm4PM Integration |
| R-004 | False positive diagnostics in expanded rule set (40+ rules) | MEDIUM (user confusion, disable LSP diagnostics) | MEDIUM-HIGH | Implement multi-level severity (ERROR/WARN/INFO), community feedback loop, rule versioning for deprecation. Test coverage >90%. | Sean Chatman | Diagnostic Coverage |
| R-005 | LSP server crash on malformed config files in marketplace plugins | HIGH (plugin unavailability, user churn) | MEDIUM | Add panic recovery, graceful error handling, structured logging for crash analysis. Ship with error reporting hook. | Sean Chatman | Marketplace Release |
| R-006 | Backward compatibility breakage with v26.6.x configs | HIGH (migration friction, adoption blockers) | LOW | Validate against v26.6.x test corpus before release. Implement schema versioning. Publish migration guide. | Sean Chatman | Marketplace Release |
| R-007 | Slow semantic token calculation on large CLAUDE.md files | MEDIUM (IDE lag, poor UX) | MEDIUM | Profile token generation, implement incremental token updates, cache results per file. Target <100ms latency. | Sean Chatman | Conformance Completeness |
| R-008 | Incomplete receipt chain persistence during crash/power loss | MEDIUM (orphaned receipts, audit gaps) | LOW | Implement atomic writes, fsync on critical sections, write-ahead logging (WAL) pattern. | Sean Chatman | Receipt Chain Foundation |
| R-009 | Multi-workspace diagnostics conflict (cross-project scope confusion) | MEDIUM (incorrect diagnostics in global vs. project scope) | MEDIUM | Add explicit workspace boundary detection, test global vs. project scopes thoroughly, document scope precedence rules. | Sean Chatman | Conformance Completeness |
| R-010 | Marketplace distribution delays (plugin review, store infrastructure) | MEDIUM (delayed market release, competitive lag) | MEDIUM-HIGH | Submit to marketplace early (beta), establish review SLA with marketplace team, prepare rollback plan. | Sean Chatman | Marketplace Release |

---

## Risk Mitigation Tracking

### High-Impact Risks (R-001, R-002, R-005, R-006)

- **R-001** (OCEL bloat): Feature flag to disable persistence by default. Log rotation enabled. Test with 10-session workloads.
- **R-002** (Receipt tampering): Implement chain validation test. Add OS-level ACLs in documentation.
- **R-005** (LSP crash): Add panic hook, structured error logging. Ship with telemetry opt-in.
- **R-006** (Compatibility): Maintain v26.6.x test cases. Publish breaking-change guide before release.

### Medium-Impact Risks (R-003, R-004, R-007, R-008, R-009)

- **R-003** (Wasm4PM perf): Benchmark constraint verification. Use background thread.
- **R-004** (False positives): Beta feedback, rule severity calibration, gradual rollout (50% → 100%).
- **R-007** (Token latency): Incremental updates, caching layer, profile with tree-sitter.
- **R-008** (Crash recovery): WAL pattern, atomic writes, test power-loss scenarios.
- **R-009** (Scope confusion): Comprehensive scope detection tests, documentation clarification.

### Release Blockers

None identified. R-005 and R-006 must be resolved before marketplace submission; all others are post-release refinements.

---

## Risk Owner Responsibilities

- **Weekly Risk Review**: Update likelihood/impact based on testing progress
- **Incident Escalation**: Immediately escalate any materialized risk (observed in QA or production)
- **Mitigation Verification**: Confirm each mitigation is implemented and tested before release
- **Release Gate**: Sign off on risk register before GA announcement

