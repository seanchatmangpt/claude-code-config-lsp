# v26.7.3 Milestone Planning — JIRA Import Guide

**Target Release:** 2026-09-20  
**Epic:** CCC-2600  
**Project:** claude-code-config-lsp  
**Status:** CANDIDATE (Receipt Chain OPEN)

---

## Overview

This directory contains JIRA import artifacts for the v26.7.3 release of claude-code-config-lsp. The v26.7.3 milestone advances the LSP validator from a CANDIDATE implementation to a **production-ready configuration law enforcement and conformance oracle** for Claude Code.

### Three Strategic Pillars

1. **Ecosystem Maturation** — Complete integration with wasm4pm-compat, OCEL event logging, and Chicago TDD patterns
2. **Conformance Completeness** — Expand Declare constraint coverage, add receipt chain persistence, and enable workspace health scoring
3. **Developer Experience** — Ship marketplace-ready plugin, skill-based validation API, and multi-workspace diagnostics

---

## Directory Structure

```
docs/jira/26.7.3/
├── README.md                  ← You are here
├── bulk-import.json           ← JIRA import file with all 42 issues
├── MILESTONES.md              ← Detailed milestone breakdown
└── PRIORITY_MATRIX.md         ← Priority and dependency mapping
```

### File Descriptions

#### `bulk-import.json`
**Purpose:** JIRA bulk import file containing all 42 issues for v26.7.3 release.

**Contents:**
- 1 Epic (CCC-2600): v26.7.3 umbrella
- 11 Stories (CCC-2601 through CCC-2635): Feature stories grouped by P0/P1/P2/P3
- 30 Subtasks (CCC-2602 through CCC-2641): Granular work items

**Issue Breakdown by Priority:**

| Priority | Count | Range | Features |
|----------|-------|-------|----------|
| **P0: Ecosystem Maturity** | 9 | CCC-2601 to CCC-2610 | OCEL persistence, Receipt chain, Wasm4PM |
| **P1: Conformance Completeness** | 15 | CCC-2611 to CCC-2620 | 40+ diagnostic rules, Cross-file refs, Scoring |
| **P2: Developer Experience** | 12 | CCC-2621 to CCC-2630 | Marketplace, Skill polish, Virtual docs |
| **P3: Testing & Quality** | 6 | CCC-2631 to CCC-2637 | Test coverage, Mutation testing, Receipt chain tests |
| **M5: Release** | 4 | CCC-2638 to CCC-2641 | Changelog, Docs, Final QA, Sign-off |
| **Epic** | 1 | CCC-2600 | Umbrella epic |
| **TOTAL** | **42** | — | — |

**Format:** JSON with JIRA issue objects. Each issue includes:
- `key`: JIRA issue key (e.g., "CCC-2601")
- `summary`: Issue title
- `description`: Detailed requirements with acceptance criteria
- `issuetype`: Epic, Story, or Subtask
- `assignee`: Always "sean.chatman" (adjust as needed)
- `priority`: Highest, High, Medium (maps to P0, P1, P2)
- `dueDate`: ISO 8601 format (YYYY-MM-DD)
- `labels`: Tag-based categorization (e.g., "p0-ecosystem", "m1-foundation")
- `components`: LSP module assignment (e.g., "OCEL", "Conformance")
- `parent`: Parent issue key (for subtasks)

**Size:** ~150 KB (JSON)

---

#### `MILESTONES.md` (Future)
**Purpose:** Detailed breakdown of 5 milestones and their deliverables.

**Expected Contents:**
- M1: Ecosystem Foundation (2026-07-30) — OCEL, receipts, wasm4pm
- M2: Conformance Expansion (2026-08-15) — 40+ rules, cross-file refs, scoring
- M3: Developer Experience (2026-08-30) — Marketplace, skill polish, virtual docs
- M4: Testing & QA (2026-09-10) — 70% coverage, mutation testing
- M5: Release (2026-09-20) — Final docs, GitHub release, marketplace approval

**Status:** Not yet created (optional reference)

---

#### `PRIORITY_MATRIX.md` (Future)
**Purpose:** Visual dependency graph and priority ordering.

**Expected Contents:**
- Dependency constraints (P0 must complete before P1, etc.)
- Critical path analysis
- Issue dependencies (parent → child)
- Parallel work opportunities

**Status:** Not yet created (optional reference)

---

## How to Import to JIRA

### Prerequisites

- **JIRA Access:** Admin or Project Manager role in the target JIRA project
- **Project Setup:** Project must exist (e.g., "CCC" or "claude-code-config-lsp")
- **JSON Validity:** `bulk-import.json` must be syntactically valid

### Step-by-Step Import

#### 1. Validate JSON

```bash
jq empty docs/jira/26.7.3/bulk-import.json
```

Expected: No output (valid JSON)

#### 2. Prepare JIRA Project

In JIRA:
1. Navigate to **Projects** → your project (e.g., "claude-code-config-lsp")
2. Note the **Project Key** (e.g., "CCC")
3. Verify project has required **Issue Types**: Epic, Story, Subtask
4. Verify **Components** exist: LSP Server, OCEL, Conformance, Diagnostics, Receipt Chain, etc.

#### 3. Import via JIRA REST API

**Option A: Command Line (Recommended)**

```bash
# Install jq (macOS: brew install jq)

# Authenticate and import
curl -X POST \
  -H "Authorization: Bearer YOUR_API_TOKEN" \
  -H "Content-Type: application/json" \
  -d @docs/jira/26.7.3/bulk-import.json \
  https://your-jira-instance.atlassian.net/rest/api/3/issues/import
```

**Option B: JIRA UI**

1. Go to **Projects** → your project
2. Click **Tools** (⚙️ icon) → **Project Settings**
3. Select **Issue Import** → **Import**
4. Upload `docs/jira/26.7.3/bulk-import.json`
5. Map fields to JIRA schema (if prompted)
6. Review summary and click **Import**

**Option C: Atlassian Migration Tool**

Use the official Atlassian CSV/JSON import tool:
```bash
# https://www.atlassian.com/software/jira/migration-tools
```

#### 4. Post-Import Verification

After import completes:

```bash
# Count imported issues
curl -H "Authorization: Bearer YOUR_API_TOKEN" \
  https://your-jira-instance.atlassian.net/rest/api/3/search \
  -d 'jql=project=CCC AND created >= -1h' | jq '.total'
```

Expected: 42 issues

#### 5. Link Issues in JIRA

Once imported, verify:

```bash
# View epic
curl -H "Authorization: Bearer YOUR_API_TOKEN" \
  https://your-jira-instance.atlassian.net/rest/api/3/issues/CCC-2600

# View epic's child issues
curl -H "Authorization: Bearer YOUR_API_TOKEN" \
  https://your-jira-instance.atlassian.net/rest/api/3/search \
  -d 'jql=parent=CCC-2600' | jq '.issues[].key'
```

---

## Field Mappings

When importing, ensure JIRA field mappings match:

| JSON Field | JIRA Field | Data Type | Required |
|------------|-----------|-----------|----------|
| `key` | Issue Key | Text | ✓ |
| `summary` | Summary | Text | ✓ |
| `description` | Description | Long Text | ✓ |
| `issuetype` | Issue Type | Select | ✓ |
| `assignee` | Assignee | User | ✗ |
| `priority` | Priority | Select | ✓ |
| `dueDate` | Due Date | Date | ✗ |
| `labels` | Labels | Array | ✗ |
| `components` | Components | Array | ✗ |
| `parent` | Parent Issue | Link | ✓ (for Subtasks) |

**Adjustments for Your Instance:**
- Replace `sean.chatman` with your JIRA user ID if different
- Verify component names match your JIRA project setup
- Adjust priority levels if using custom scheme (e.g., map "Highest" → "Critical")

---

## Acceptance Criteria for Each Feature

Each Story and Subtask includes **Acceptance Criteria** in the description. Sample:

```
Acceptance Criteria:
- OcelPersister struct serializes events to ~/.claude/logs/...
- Events include: timestamp (UTC), URI, event type, severity counts
- Each event validates against OCEL 2.0 JSON schema
- Log file size monitoring and rotation working correctly
- Feature flag ocel-persistence controls enabling/disabling
```

Use these to:
1. **Define Definition of Done** per task
2. **Create test cases** (1 test per criterion)
3. **Review PRs** against criteria
4. **Validate completion** before marking issue resolved

---

## Milestone Timeline

| Milestone | Target | Key Deliverables | Issues |
|-----------|--------|------------------|--------|
| **M1: Ecosystem Foundation** | 2026-07-30 | OCEL persistence, Receipt chain, Wasm4PM | CCC-2601 to CCC-2610 |
| **M2: Conformance Expansion** | 2026-08-15 | 40+ rules, Cross-file validation, Scoring | CCC-2611 to CCC-2620 |
| **M3: Developer Experience** | 2026-08-30 | Marketplace, Skill polish, Virtual docs | CCC-2621 to CCC-2630 |
| **M4: Testing & QA** | 2026-09-10 | 70% coverage, Mutation testing, Receipt tests | CCC-2631 to CCC-2637 |
| **M5: Release** | 2026-09-20 | GitHub release, Marketplace approval, Final docs | CCC-2638 to CCC-2641 |

---

## Next Steps for Milestone Planning

### Immediate (Before Import)

1. **Validate JSON:**
   ```bash
   jq empty docs/jira/26.7.3/bulk-import.json
   ```

2. **Review Epic & Stories:**
   - Read through bulk-import.json (Epic CCC-2600)
   - Verify features align with v26.7.3 PRD
   - Check timeline assumptions (2026-07-30 to 2026-09-20)

3. **Customize for Your Team:**
   - Update assignee from "sean.chatman" to your team members
   - Adjust due dates if needed (milestones may shift)
   - Add custom fields if your JIRA instance requires them

### Import Phase

4. **Import to JIRA:**
   - Follow **Step-by-Step Import** section above
   - Run post-import verification
   - Link issues and verify hierarchy

5. **Configure JIRA Workflow:**
   - Set issue transitions: To Do → In Progress → Done
   - Enable notifications (updates on assignment, status change)
   - Create dashboard to track epic progress

### Post-Import

6. **Create Release Plan:**
   - Create 5 JIRA Versions (M1, M2, M3, M4, M5)
   - Assign stories to versions
   - Configure version release dates
   - Link GitHub commits to issues (GitHub integration)

7. **Team Kickoff:**
   - Review epic with team
   - Discuss milestone dependencies
   - Assign owners to each story
   - Establish daily standup cadence

8. **Set Up Tracking:**
   - Create Gantt chart or roadmap view
   - Monitor burndown chart weekly
   - Track velocity (issues completed per sprint)
   - Schedule milestone review meetings

### Execution

9. **Start M1 (2026-07-30 target):**
   - Move CCC-2601, CCC-2604, CCC-2607 to "In Progress"
   - Create GitHub milestone for v26.7.3
   - Link JIRA issues to GitHub PRs
   - Daily standup: report blockers

10. **Monitor Progress:**
    - Weekly milestone status review
    - Adjust due dates if needed
    - Update issue descriptions with findings
    - Escalate blockers immediately

11. **Release Readiness (Week of 2026-09-15):**
    - Verify all M1-M4 issues marked "Done"
    - Run full test suite
    - Finalize marketplace submission
    - Prepare release notes

12. **Go Live (2026-09-20):**
    - Tag v26.7.3 in GitHub
    - Publish to JIRA (mark version as released)
    - Submit plugin to marketplace
    - Monitor adoption and bug reports

---

## Dependencies & Critical Path

### P0 Blocks Everything

The three P0 features **must complete before** P1-P3 can be done:
- CCC-2601: OCEL persistence → enables OCEL event log analysis
- CCC-2604: Receipt chain → enables conformance audit trail
- CCC-2607: Wasm4PM integration → enables constraint verification

**Critical Path Timeline:**
```
M1 (2026-07-30)
  └─ P0 features complete
     └─ M2 (2026-08-15)
        └─ P1 features (rules, refs, scoring)
           └─ M3 (2026-08-30)
              └─ P2 features (marketplace, skill, docs)
                 └─ M4 (2026-09-10)
                    └─ P3 features (testing, QA)
                       └─ M5 (2026-09-20)
                          └─ Release
```

### Parallel Work Opportunities

- **P1 and P2 can overlap** (week of 2026-08-15): Marketplace prep while writing diagnostic rules
- **Testing (P3) can start early**: Write unit tests as features land
- **Documentation can be drafted** in M2 (polish in M3/M4)

---

## Tracking & Reporting

### Weekly Status Report Template

Use this to track progress:

```markdown
## v26.7.3 Milestone Status (Week of YYYY-MM-DD)

### Milestone Progress
- M1 (2026-07-30): X/9 issues complete (XX%)
- M2 (2026-08-15): X/15 issues complete (XX%)
- M3 (2026-08-30): X/12 issues complete (XX%)
- M4 (2026-09-10): X/6 issues complete (XX%)
- M5 (2026-09-20): X/4 issues complete (XX%)

### Completed This Week
- CCC-2601: OCEL persistence (Done)
- CCC-2602: OcelPersister struct (Done)
- ...

### In Progress
- CCC-2603: Feature flag configuration (50%)
- ...

### Blockers
- None currently
- Or: CCC-2615 blocked by missing dependency in CCC-2610

### Next Week
- Complete M1 ecosystem foundation
- Begin M2 diagnostic rules
- ...

### Risks
- Timeline risk: if P0 slips by 1 week, entire release slides
- Team availability: ensure coverage for each milestone
```

### Metrics to Track

- **Burndown:** Issues completed vs. projected (per milestone)
- **Velocity:** Stories completed per week (calculate every 2 weeks)
- **Lead Time:** Days from issue creation to completion (target: < 14 days)
- **Defect Rate:** Bugs found vs. features shipped (target: < 5%)

---

## Troubleshooting Import Issues

### JSON Validation Errors

**Error:** `jq: parse error`

**Solution:**
```bash
# Check for syntax errors
python3 -m json.tool docs/jira/26.7.3/bulk-import.json
```

### JIRA API Errors

**Error:** `401 Unauthorized`

**Solution:** Check API token validity and permissions

**Error:** `400 Bad Request: Unknown Issue Type`

**Solution:** Verify "Epic", "Story", "Subtask" exist in your JIRA project

**Error:** `Issue key already exists`

**Solution:** Either update existing issues or rename keys (find/replace in JSON)

### Missing Components or Fields

**Error:** `Component 'OCEL' not found`

**Solution:**
1. Go to JIRA Project Settings → Components
2. Create missing components (OCEL, Conformance, etc.)
3. Re-run import

---

## Reference Materials

- **Full PRD:** `/Users/sac/claude-code-config-lsp/docs/v26.7.3-PRD-ARD.md`
- **Ontology Source:** `schema/claude-code-config.ttl`
- **GitHub Repo:** https://github.com/seanchatmangpt/claude-code-config-lsp
- **LSP Spec:** https://microsoft.github.io/language-server-protocol/specifications/specification-3-18/
- **Declare:** https://www.win.tue.nl/~cgunther/declare/

---

## Support & Questions

For questions about the import process or issue structure:

1. **Check this README** (you're reading it!)
2. **Review PRD:** `v26.7.3-PRD-ARD.md` for full context
3. **Check JIRA Issue:** Click any issue key (CCC-2601, etc.) to view full description
4. **Contact:** Sean Chatman (xpointsh@gmail.com)

---

**Document Version:** 26.7.3-JIRA-Import-v1  
**Last Updated:** 2026-07-03  
**Next Review:** After import completion (2026-07-05)
