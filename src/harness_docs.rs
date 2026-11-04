//! Harness assistant out-of-band inspection surface — H1 lane.
//!
//! Expands the virtual document system beyond `claude-config://health` with
//! three markdown-rendered inspection documents over a workspace root:
//!
//! | URI | Content |
//! |---|---|
//! | `claude-config://inventory` | every discovered config surface, its scope, finding counts by code, conformance score |
//! | `claude-config://receipts` | gate receipts (`.claude/receipts/latest.json` fleet convention, falling back to hash-named chain receipts) with age/staleness vs the newest tracked source file (the ggen-igniter require-gate-receipt law) |
//! | `claude-config://standing` | per-surface standing in the fleet vocabulary: ALIVE (clean + fresh receipt), PARTIAL (findings), UNVERIFIED (surface with no files discovered — never scanned) |
//!
//! All renders are pure functions of `(uri, root)` — no server state, no
//! file mutation. Scope classification mirrors backend.rs's private
//! `AgentScope` (global / project / plugin) by path shape; P12 owns the
//! unification at the backend dispatch point.

use crate::inventory::{build_tree, classify_severity};
use std::collections::BTreeMap;
use std::path::Path;
use std::time::SystemTime;

/// The config surfaces this server knows. A surface absent from a workspace
/// has never been scanned there → standing UNVERIFIED.
pub const KNOWN_SURFACES: &[&str] = &[
    "settings.json",
    "settings.local.json",
    "toml",
    "claude_md",
    "agents_md",
    "agent",
    "skill",
    "hook",
    "mcp.json",
    "plugin.json",
    "marketplace.json",
    "keybindings.json",
    "json",
];

/// Fleet standing vocabulary values used by [`render_standing`].
pub const STANDING_ALIVE: &str = "ALIVE";
pub const STANDING_PARTIAL: &str = "PARTIAL";
pub const STANDING_UNVERIFIED: &str = "UNVERIFIED";

/// Which layer a config surface resolves from — mirrors backend.rs
/// `AgentScope` semantics by path shape (backend's enum is private).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessScope {
    Global,
    Project,
    Plugin,
}

impl HarnessScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            HarnessScope::Global => "global",
            HarnessScope::Project => "project",
            HarnessScope::Plugin => "plugin",
        }
    }
}

/// Classify a file path to global/project/plugin scope by the same path-shape
/// rules backend.rs `agent_scope()` applies to URIs. A fixture root under
/// `$HOME` but outside `~/.claude/` classifies as project, matching
/// backend.rs behavior for project-relative paths.
pub fn agent_scope_of(path: &str) -> HarnessScope {
    if path.contains("/.claude/plugins/") {
        return HarnessScope::Plugin;
    }
    if let Ok(home) = std::env::var("HOME") {
        let home_claude = format!("{home}/.claude/");
        if path.starts_with(&home_claude) {
            return HarnessScope::Global;
        }
    }
    HarnessScope::Project
}

// ---------------------------------------------------------------------------
// inventory
// ---------------------------------------------------------------------------

/// Per-surface inventory row.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SurfaceRow {
    pub surface: &'static str,
    pub files: usize,
    pub scopes: Vec<&'static str>,
    /// finding code → count
    pub findings: BTreeMap<String, usize>,
}

/// Aggregate the config tree into per-surface inventory rows.
pub fn surface_rows(root: &str) -> Vec<SurfaceRow> {
    let tree = build_tree(root);
    let mut rows: BTreeMap<&'static str, SurfaceRow> = BTreeMap::new();
    for file in &tree.files {
        let row = rows.entry(file.surface).or_insert_with(|| SurfaceRow {
            surface: file.surface,
            files: 0,
            scopes: Vec::new(),
            findings: BTreeMap::new(),
        });
        row.files += 1;
        let scope = agent_scope_of(&file.path).as_str();
        if !row.scopes.contains(&scope) {
            row.scopes.push(scope);
        }
        for f in &file.diagnostics {
            *row.findings.entry(f.code.clone()).or_insert(0) += 1;
        }
    }
    let mut out: Vec<SurfaceRow> = rows.into_values().collect();
    out.sort_by(|a, b| a.surface.cmp(b.surface));
    out
}

/// Score summary computed WITHOUT `conformance_report`'s side effect: that
/// function calls `receipt::maybe_issue_receipt`, which writes a receipt
/// into the workspace. Virtual documents must never mutate the workspace.
fn score_summary(rows: &[SurfaceRow]) -> (f64, usize, usize) {
    let mut errors = 0usize;
    let mut warnings = 0usize;
    let mut checked = 0usize;
    for r in rows {
        checked += r.files;
        for (code, count) in &r.findings {
            match classify_severity(code, "") {
                "error" => errors += count,
                "warning" => warnings += count,
                _ => {}
            }
        }
    }
    let raw = (100.0 - 10.0 * errors as f64 - 3.0 * warnings as f64) / 100.0;
    let score = raw.clamp(0.0, 1.0);
    (score, checked, checked.saturating_sub(errors + warnings))
}

/// Render `claude-config://inventory` as markdown with stable sections.
pub fn render_inventory(root: &str) -> String {
    let rows = surface_rows(root);
    let (score, checked, admitted) = score_summary(&rows);
    let mut out = String::from(
        "# Workspace Config Inventory\n\n\
         *Virtual document — read-only. Computed on demand from the workspace root.*\n\n\
         ## Summary\n\n",
    );
    out.push_str(&format!(
        "- Root: `{root}`\n- Surfaces discovered: {}\n- Conformance score: {:.3} ({} of {} surfaces admitted)\n",
        rows.len(),
        score,
        admitted,
        checked,
    ));
    out.push_str("\n## Surfaces\n\n");
    if rows.is_empty() {
        out.push_str("_No config surfaces discovered._\n");
        return out;
    }
    out.push_str("| Surface | Scope | Files | Findings |\n|---|---|---:|---:|\n");
    for r in &rows {
        let total: usize = r.findings.values().sum();
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            r.surface,
            r.scopes.join(", "),
            r.files,
            total,
        ));
    }
    out.push_str("\n## Findings by Code\n\n");
    let mut any = false;
    for r in &rows {
        for (code, count) in &r.findings {
            any = true;
            out.push_str(&format!("- `{code}` x{count} ({})\n", r.surface));
        }
    }
    if !any {
        out.push_str("_No findings._\n");
    }
    out
}

// ---------------------------------------------------------------------------
// receipts
// ---------------------------------------------------------------------------

/// One rendered receipt row.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReceiptRow {
    /// "fleet-latest" or "chain"
    pub source: &'static str,
    pub status: String,
    pub timestamp: Option<String>,
    pub age_seconds: Option<u64>,
    /// "fresh" | "stale" | "unknown"
    pub staleness: &'static str,
    pub commands: Vec<String>,
    pub conformance_score: Option<f64>,
}

/// Age of a file's mtime relative to now, in seconds.
fn mtime_age(path: &Path) -> Option<u64> {
    let m = std::fs::metadata(path).ok()?.modified().ok()?;
    SystemTime::now()
        .duration_since(m)
        .ok()
        .map(|d| d.as_secs())
}

/// Newest mtime age (seconds since the most recently modified config source
/// file) among all discovered config surfaces under `root`.
fn newest_source_age(root: &str) -> Option<u64> {
    let mut newest: Option<SystemTime> = None;
    for path in crate::scan::discover(Path::new(root)) {
        if let Ok(m) = std::fs::metadata(&path) {
            if let Ok(mtime) = m.modified() {
                newest = Some(match newest {
                    Some(n) if n >= mtime => n,
                    _ => mtime,
                });
            }
        }
    }
    newest.map(|t| {
        SystemTime::now()
            .duration_since(t)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    })
}

/// Classify receipt staleness: fresh iff the receipt is at least as new as
/// the newest tracked config source file (ggen-igniter require-gate-receipt
/// law — a receipt older than the code it gates is stale).
fn staleness_of(receipt_age: Option<u64>, newest_source: Option<u64>) -> &'static str {
    match (receipt_age, newest_source) {
        (Some(r), Some(s)) if r <= s => "fresh",
        (Some(_), Some(_)) => "stale",
        _ => "unknown",
    }
}

/// Collect gate receipts for `root`: the fleet-convention
/// `.claude/receipts/latest.json` (`{"status":"green","commands":[...]}`)
/// first, then the hash-named chain receipts from [`crate::receipt`].
pub fn collect_receipt_rows(root: &str) -> Vec<ReceiptRow> {
    let mut rows = Vec::new();
    let newest_source = newest_source_age(root);

    // Fleet-convention latest.json, if present.
    let latest = Path::new(root).join(".claude/receipts/latest.json");
    if let Ok(raw) = std::fs::read_to_string(&latest) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) {
            let status = v
                .get("status")
                .and_then(|s| s.as_str())
                .unwrap_or("unknown")
                .to_string();
            let commands = v
                .get("commands")
                .and_then(|c| c.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|c| c.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            let age = mtime_age(&latest);
            rows.push(ReceiptRow {
                source: "fleet-latest",
                status,
                timestamp: None,
                age_seconds: age,
                staleness: staleness_of(age, newest_source),
                commands,
                conformance_score: None,
            });
        }
    }

    // Hash-named chain receipts (newest first).
    if let Ok(receipts) = crate::receipt::read_all_receipts(Path::new(root)) {
        let mut receipts = receipts;
        receipts.sort_by_key(|a| std::cmp::Reverse(a.timestamp));
        for r in receipts.iter().take(3) {
            // Chain receipts carry their own timestamp; derive age from it.
            let age_secs = chrono::Utc::now()
                .signed_duration_since(r.timestamp)
                .num_seconds()
                .max(0) as u64;
            rows.push(ReceiptRow {
                source: "chain",
                status: format!("conformance {:.3}", r.conformance_score),
                timestamp: Some(r.timestamp.to_rfc3339()),
                age_seconds: Some(age_secs),
                staleness: staleness_of(Some(age_secs), newest_source),
                commands: Vec::new(),
                conformance_score: Some(r.conformance_score),
            });
        }
    }
    rows
}

/// Render `claude-config://receipts` as markdown with stable sections.
pub fn render_receipts(root: &str) -> String {
    let rows = collect_receipt_rows(root);
    let mut out = String::from(
        "# Gate Receipts\n\n\
         *Virtual document — read-only. Rendered from `.claude/receipts/`.*\n\n\
         ## Status\n\n",
    );
    if rows.is_empty() {
        out.push_str(
            "_No receipts found. A workspace with no gate receipt is UNVERIFIED \
             (require-gate-receipt law)._\n",
        );
        return out;
    }
    let head = &rows[0];
    out.push_str(&format!(
        "- Latest: `{}` ({})\n- Staleness: **{}**\n",
        head.status, head.source, head.staleness,
    ));
    if let Some(age) = head.age_seconds {
        out.push_str(&format!("- Age: {age}s\n"));
    }
    out.push_str(
        "\n## Receipts\n\n| Source | Status | Age (s) | Staleness |\n|---|---|---:|---|\n",
    );
    for r in &rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            r.source,
            r.status,
            r.age_seconds
                .map(|a| a.to_string())
                .unwrap_or_else(|| "-".to_string()),
            r.staleness,
        ));
    }
    if !head.commands.is_empty() {
        out.push_str("\n## Commands\n\n```sh\n");
        for c in &head.commands {
            out.push_str(c);
            out.push('\n');
        }
        out.push_str("```\n");
    }
    out
}

// ---------------------------------------------------------------------------
// standing
// ---------------------------------------------------------------------------

/// Per-surface standing summary row.
#[derive(Debug, Clone, serde::Serialize)]
pub struct StandingRow {
    pub surface: &'static str,
    pub standing: &'static str,
    pub files: usize,
    pub finding_count: usize,
    pub reason: String,
}

/// Compute per-surface standing over `root`.
///
/// A surface is ALIVE iff diagnostics are clean AND the gate receipt is
/// fresh; PARTIAL if it has findings; UNVERIFIED if no files of that surface
/// were discovered (never scanned). If no receipt exists at all, every
/// scanned surface is capped at PARTIAL — clean diagnostics without a fresh
/// gate receipt do not reach ALIVE.
pub fn standing_rows(root: &str) -> Vec<StandingRow> {
    let rows = surface_rows(root);
    let by_surface: BTreeMap<&str, &SurfaceRow> = rows.iter().map(|r| (r.surface, r)).collect();
    let receipt_fresh = collect_receipt_rows(root)
        .first()
        .map(|r| r.staleness == "fresh")
        .unwrap_or(false);

    let mut out = Vec::new();
    for surface in KNOWN_SURFACES {
        let scanned = by_surface.get(surface);
        let (standing, files, findings, reason) = match scanned {
            None => (
                STANDING_UNVERIFIED,
                0,
                0,
                "no files of this surface discovered — never scanned".to_string(),
            ),
            Some(r) => {
                let count: usize = r.findings.values().sum();
                if count == 0 && receipt_fresh {
                    (
                        STANDING_ALIVE,
                        r.files,
                        0,
                        "diagnostics clean, gate receipt fresh".to_string(),
                    )
                } else if count > 0 {
                    (
                        STANDING_PARTIAL,
                        r.files,
                        count,
                        format!("{count} findings across {} file(s)", r.files),
                    )
                } else {
                    (
                        STANDING_PARTIAL,
                        r.files,
                        0,
                        "diagnostics clean but no fresh gate receipt".to_string(),
                    )
                }
            }
        };
        out.push(StandingRow {
            surface,
            standing,
            files,
            finding_count: findings,
            reason,
        });
    }
    out
}

/// Render `claude-config://standing` as markdown with stable sections.
pub fn render_standing(root: &str) -> String {
    let rows = standing_rows(root);
    let mut out = String::from(
        "# Surface Standing\n\n\
         *Virtual document — read-only. Fleet standing vocabulary: ALIVE / PARTIAL / \
         UNVERIFIED (REFUSED is a transition verdict, never a standing here).*\n\n\
         ## Summary\n\n",
    );
    let alive = rows.iter().filter(|r| r.standing == STANDING_ALIVE).count();
    let partial = rows
        .iter()
        .filter(|r| r.standing == STANDING_PARTIAL)
        .count();
    let unverified = rows
        .iter()
        .filter(|r| r.standing == STANDING_UNVERIFIED)
        .count();
    out.push_str(&format!(
        "- ALIVE: {alive}\n- PARTIAL: {partial}\n- UNVERIFIED: {unverified}\n\n\
         ## Surfaces\n\n\
         | Surface | Standing | Files | Findings | Reason |\n|---|---|---:|---:|---|\n"
    ));
    for r in &rows {
        out.push_str(&format!(
            "| {} | **{}** | {} | {} | {} |\n",
            r.surface, r.standing, r.files, r.finding_count, r.reason,
        ));
    }
    out
}

// ---------------------------------------------------------------------------
// dispatch
// ---------------------------------------------------------------------------

/// Dispatch a `claude-config://` URI to its renderer. Unknown virtual URIs
/// return `None` (backend maps that to method_not_found).
pub fn render_uri(uri: &str, root: &str) -> Option<String> {
    match uri {
        "claude-config://inventory" => Some(render_inventory(root)),
        "claude-config://receipts" => Some(render_receipts(root)),
        "claude-config://standing" => Some(render_standing(root)),
        _ => None,
    }
}

/// True if `uri` is one of the H1 harness virtual documents.
pub fn is_harness_doc_uri(uri: &str) -> bool {
    matches!(
        uri,
        "claude-config://inventory" | "claude-config://receipts" | "claude-config://standing"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_root() -> String {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/claude-config").to_string()
    }

    #[test]
    fn scope_classifies_plugin_and_project() {
        assert_eq!(
            agent_scope_of("/x/.claude/plugins/cache/foo/plugin.json"),
            HarnessScope::Plugin
        );
        assert_eq!(
            agent_scope_of("/some/project/settings.json"),
            HarnessScope::Project
        );
    }

    #[test]
    fn inventory_lists_fixture_surfaces_and_scores() {
        let root = fixture_root();
        let md = render_inventory(&root);
        assert!(md.contains("# Workspace Config Inventory"));
        assert!(md.contains("## Summary"));
        assert!(md.contains("## Surfaces"));
        assert!(md.contains("Conformance score:"));
        // fixture has skill files → skill surface present
        assert!(md.contains("skill"));
    }

    #[test]
    fn inventory_empty_root_renders_placeholder() {
        let md = render_inventory("/nonexistent-root-xyz");
        assert!(md.contains("No config surfaces discovered."));
    }

    #[test]
    fn staleness_law() {
        // receipt newer than (age <=) newest source → fresh
        assert_eq!(staleness_of(Some(10), Some(20)), "fresh");
        // receipt older than newest source → stale
        assert_eq!(staleness_of(Some(30), Some(20)), "stale");
        // missing either side → unknown
        assert_eq!(staleness_of(None, Some(20)), "unknown");
        assert_eq!(staleness_of(Some(20), None), "unknown");
    }

    #[test]
    fn receipts_fixture_has_chain_receipt() {
        let root = fixture_root();
        let md = render_receipts(&root);
        assert!(md.contains("# Gate Receipts"));
        // fixture ships one hash-named chain receipt
        assert!(md.contains("chain"));
        assert!(md.contains("conformance 0.700"));
    }

    #[test]
    fn receipts_missing_root_is_unverified_notice() {
        let md = render_receipts("/nonexistent-root-xyz");
        assert!(md.contains("No receipts found"));
    }

    #[test]
    fn standing_fixture_partition() {
        let root = fixture_root();
        let rows = standing_rows(&root);
        assert_eq!(rows.len(), KNOWN_SURFACES.len());
        // Real fixture partition: findings live on the JSON and agent
        // surfaces; skill (6 clean SKILL.md files) is clean.
        let settings = rows.iter().find(|r| r.surface == "settings.json").unwrap();
        assert_eq!(settings.standing, STANDING_PARTIAL);
        assert!(settings.finding_count > 0);
        let agent = rows.iter().find(|r| r.surface == "agent").unwrap();
        assert_eq!(agent.standing, STANDING_PARTIAL);
        let skill = rows.iter().find(|r| r.surface == "skill").unwrap();
        assert_eq!(skill.finding_count, 0);
        assert_eq!(skill.files, 6);
        // absent surfaces are UNVERIFIED
        let absent = rows.iter().find(|r| r.surface == "plugin.json").unwrap();
        assert_eq!(absent.standing, STANDING_UNVERIFIED);
    }

    #[test]
    fn standing_renders_vocabulary() {
        let md = render_standing(&fixture_root());
        assert!(md.contains("# Surface Standing"));
        assert!(md.contains("ALIVE") || md.contains("PARTIAL"));
        assert!(md.contains("UNVERIFIED"));
    }

    #[test]
    fn dispatch_routes_known_uris_and_rejects_unknown() {
        let root = fixture_root();
        assert!(render_uri("claude-config://inventory", &root).is_some());
        assert!(render_uri("claude-config://receipts", &root).is_some());
        assert!(render_uri("claude-config://standing", &root).is_some());
        assert!(render_uri("claude-config://health", &root).is_none());
        assert!(render_uri("claude-config://bogus", &root).is_none());
        assert!(is_harness_doc_uri("claude-config://standing"));
        assert!(!is_harness_doc_uri("claude-config://health"));
        assert!(!is_harness_doc_uri("file:///x"));
    }
}
