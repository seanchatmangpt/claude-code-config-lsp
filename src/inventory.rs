//! Config inventory + usage audit — the data behind the `export` and `unused`
//! CLI verbs.
//!
//! `build_tree` walks the workspace and serializes every Claude Code config
//! file (surface, parsed content, diagnostics). `usage_report` audits how much
//! config is inert/unreferenced across four signals: declared-but-missing,
//! orphaned-on-disk, orphaned hooks, and unknown settings keys.
//!
//! Everything here reuses the single classify/dispatch source of truth in
//! `crate::scan`, and returns `serde::Serialize` types so the `clap-noun-verb`
//! framework renders them as JSON/YAML/table automatically.

use serde::Serialize;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::scan::{self, ScanFinding};

/// Recognized top-level keys in `settings.json` / `settings.local.json`. Any
/// top-level key outside this set is reported as an unknown (dead/no-op) key by
/// [`usage_report`]. Covers both stock Claude Code keys and this project's own.
pub const VALID_SETTINGS_TOP_LEVEL_KEYS: &[&str] = &[
    "$schema",
    "model",
    "env",
    "permissions",
    "hooks",
    "mcpServers",
    "enableAllProjectMcpServers",
    "enabledMcpjsonServers",
    "disabledMcpjsonServers",
    "apiKeyHelper",
    "cleanupPeriodDays",
    "includeCoAuthoredBy",
    "forceLoginMethod",
    "statusLine",
    "outputStyle",
    "preferredNotifChannel",
    "lspServers",
    "worktree",
    "skillOverrides",
    "parentSettingsBehavior",
    "tui",
    "effortLevel",
    "permissionMode",
    "enabledPlugins",
];

/// Refine [`scan::classify`]'s coarse `"json"`/`"claude_md"` kinds into the
/// specific config surface, by filename suffix.
pub fn surface_of(path: &str) -> &'static str {
    let lower = path.to_lowercase();
    match scan::classify(path) {
        "json" => {
            if lower.ends_with("settings.local.json") {
                "settings.local.json"
            } else if lower.ends_with("settings.json") {
                "settings.json"
            } else if lower.ends_with("mcp.json") {
                "mcp.json"
            } else if lower.ends_with("plugin.json") {
                "plugin.json"
            } else if lower.ends_with("marketplace.json") {
                "marketplace.json"
            } else if lower.ends_with("keybindings.json") {
                "keybindings.json"
            } else {
                "json"
            }
        }
        "claude_md" => {
            if lower.ends_with("agents.md") {
                "agents_md"
            } else {
                "claude_md"
            }
        }
        other => other,
    }
}

// ---------------------------------------------------------------------------
// scan: flat diagnostics report
// ---------------------------------------------------------------------------

/// A finding tagged with the file it came from — the serialized shape of the
/// `scan` verb's output.
#[derive(Debug, Clone, Serialize)]
pub struct ScanFileFinding {
    pub file: String,
    pub code: String,
    pub message: String,
    pub category: &'static str,
}

/// The `scan` verb's structured result: every diagnostic across the walked tree.
#[derive(Debug, Clone, Serialize)]
pub struct ScanReport {
    pub target: String,
    pub count: usize,
    pub findings: Vec<ScanFileFinding>,
}

/// Walk `root` and collect all diagnostics into a [`ScanReport`].
pub fn scan_report(root: &str) -> ScanReport {
    let mut findings: Vec<ScanFileFinding> = Vec::new();
    for path in scan::discover(Path::new(root)) {
        let path_str = path.to_string_lossy().to_string();
        if scan::classify(&path_str) == "unknown" {
            continue;
        }
        let Ok(raw) = std::fs::read_to_string(&path) else {
            continue;
        };
        for f in scan::analyze_document(&path_str, &raw) {
            findings.push(ScanFileFinding {
                file: path_str.clone(),
                code: f.code,
                message: f.message,
                category: f.category,
            });
        }
    }
    findings.sort_by(|a, b| a.file.cmp(&b.file).then(a.code.cmp(&b.code)));
    ScanReport { target: root.to_string(), count: findings.len(), findings }
}

// ---------------------------------------------------------------------------
// export: config tree
// ---------------------------------------------------------------------------

/// One config file in the exported [`ConfigTree`].
#[derive(Debug, Clone, Serialize)]
pub struct ConfigFileNode {
    pub path: String,
    pub surface: &'static str,
    /// Parsed JSON for json surfaces; extracted frontmatter (string) for
    /// agent/skill; raw text for toml/hook/CLAUDE.md.
    pub content: serde_json::Value,
    pub diagnostics: Vec<ScanFinding>,
}

/// The serialized inventory of all config files under a root.
#[derive(Debug, Clone, Serialize)]
pub struct ConfigTree {
    pub root: String,
    pub file_count: usize,
    pub files: Vec<ConfigFileNode>,
}

fn content_value(kind: &str, raw: &str) -> serde_json::Value {
    match kind {
        "json" => serde_json::from_str(raw).unwrap_or(serde_json::Value::Null),
        "agent" | "skill" => match crate::analyzers::claude_md::extract_frontmatter(raw) {
            Some(fm) => serde_json::Value::String(fm.to_string()),
            None => serde_json::Value::Null,
        },
        _ => serde_json::Value::String(raw.to_string()),
    }
}

/// Walk `root` and build the config tree (every known config surface, with its
/// parsed content and diagnostics). Files that aren't a config surface are skipped.
pub fn build_tree(root: &str) -> ConfigTree {
    let mut files: Vec<ConfigFileNode> = Vec::new();
    for path in scan::discover(Path::new(root)) {
        let path_str = path.to_string_lossy().to_string();
        let kind = scan::classify(&path_str);
        if kind == "unknown" {
            continue;
        }
        let Ok(raw) = std::fs::read_to_string(&path) else {
            continue;
        };
        files.push(ConfigFileNode {
            path: path_str.clone(),
            surface: surface_of(&path_str),
            content: content_value(kind, &raw),
            diagnostics: scan::analyze_document(&path_str, &raw),
        });
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    ConfigTree { root: root.to_string(), file_count: files.len(), files }
}

// ---------------------------------------------------------------------------
// unused: config usage audit
// ---------------------------------------------------------------------------

/// One inert/unreferenced config item.
#[derive(Debug, Clone, Serialize)]
pub struct UsageItem {
    /// Which bucket produced this item.
    pub kind: &'static str,
    /// The offending path (for files) or key name (for settings keys).
    pub path_or_key: String,
    pub surface: &'static str,
    pub reason: String,
}

/// "How much config is not being used" — four signals plus a utilization %.
#[derive(Debug, Clone, Serialize)]
pub struct ConfigUsageReport {
    pub root: String,
    pub total_items: usize,
    pub unused_items: usize,
    /// `unused_items / total_items * 100`, rounded to 0.1.
    pub unused_percent: f64,
    pub declared_but_missing: Vec<UsageItem>,
    pub orphaned_on_disk: Vec<UsageItem>,
    pub orphaned_hooks: Vec<UsageItem>,
    pub unknown_settings_keys: Vec<UsageItem>,
}

fn read_json(path: &Path) -> Option<serde_json::Value> {
    let raw = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

fn canonical_or_self(p: &Path) -> PathBuf {
    std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf())
}

/// Resolve a plugin-declared relative path against the candidate base dirs
/// Claude Code may use (the dir containing plugin.json, and its parent /
/// plugin root). Returns the first existing canonical path, else `None`.
fn resolve_declared(plugin_dir: &Path, rel: &str) -> Option<PathBuf> {
    let mut bases: Vec<PathBuf> = vec![plugin_dir.to_path_buf()];
    if let Some(parent) = plugin_dir.parent() {
        bases.push(parent.to_path_buf());
    }
    for base in bases {
        let candidate = base.join(rel);
        if candidate.is_file() {
            return Some(canonical_or_self(&candidate));
        }
    }
    None
}

/// Collect the referenced hook-script paths from a settings.json `hooks` block,
/// resolving `${CLAUDE_PROJECT_DIR}` against `root`.
fn referenced_hook_paths(settings: &serde_json::Value, root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Some(hooks) = settings.get("hooks").and_then(|h| h.as_object()) else {
        return out;
    };
    for (_event, matchers) in hooks {
        let Some(arr) = matchers.as_array() else { continue };
        for matcher in arr {
            let Some(inner) = matcher.get("hooks").and_then(|h| h.as_array()) else {
                continue;
            };
            for entry in inner {
                if let Some(cmd) = entry.get("command").and_then(|c| c.as_str()) {
                    let replaced = cmd
                        .replace("${CLAUDE_PROJECT_DIR}", &root.to_string_lossy())
                        .replace("$CLAUDE_PROJECT_DIR", &root.to_string_lossy());
                    // Only tokens ending in `.sh` are script references. Inline
                    // commands (e.g. `echo '...' >&2`) are not script paths and
                    // must not be reported as missing hook scripts.
                    for tok in replaced.split_whitespace() {
                        if tok.ends_with(".sh") {
                            out.push(PathBuf::from(tok));
                        }
                    }
                }
            }
        }
    }
    out
}

/// Audit config usage under `root`.
pub fn usage_report(root: &str) -> ConfigUsageReport {
    let root_path = Path::new(root);
    let mut declared_but_missing = Vec::new();
    let mut orphaned_on_disk = Vec::new();
    let mut orphaned_hooks = Vec::new();
    let mut unknown_settings_keys = Vec::new();
    let mut total_items = 0usize;

    // Discover on-disk config files once.
    let all = scan::discover(root_path);
    let mut disk_skills: Vec<PathBuf> = Vec::new();
    let mut disk_agents: Vec<PathBuf> = Vec::new();
    let mut disk_hooks: Vec<PathBuf> = Vec::new();
    let mut settings_files: Vec<PathBuf> = Vec::new();
    let mut mcp_files: Vec<PathBuf> = Vec::new();
    let mut plugin_files: Vec<PathBuf> = Vec::new();
    for p in &all {
        let s = p.to_string_lossy();
        match scan::classify(&s) {
            "skill" => disk_skills.push(p.clone()),
            "agent" => disk_agents.push(p.clone()),
            "hook" => disk_hooks.push(p.clone()),
            "json" => {
                let surf = surface_of(&s);
                if surf == "settings.json" || surf == "settings.local.json" {
                    settings_files.push(p.clone());
                } else if surf == "mcp.json" {
                    mcp_files.push(p.clone());
                } else if surf == "plugin.json" {
                    plugin_files.push(p.clone());
                }
            }
            _ => {}
        }
    }

    // --- plugin.json declarations: resolved set + declared-but-missing ---
    let mut declared_resolved: BTreeSet<PathBuf> = BTreeSet::new();
    for plugin in &plugin_files {
        let plugin_dir = plugin.parent().unwrap_or(root_path);
        let Some(json) = read_json(plugin) else { continue };
        if let Some(skills) = json.get("skills").and_then(|s| s.as_object()) {
            for (name, decl) in skills {
                if let Some(rel) = decl.get("path").and_then(|p| p.as_str()) {
                    total_items += 1;
                    match resolve_declared(plugin_dir, rel) {
                        Some(resolved) => {
                            declared_resolved.insert(resolved);
                        }
                        None => declared_but_missing.push(UsageItem {
                            kind: "declared_but_missing",
                            path_or_key: rel.to_string(),
                            surface: "skill",
                            reason: format!(
                                "plugin.json declares skill '{name}' at '{rel}', but no such file exists"
                            ),
                        }),
                    }
                }
            }
        }
    }

    // --- orphaned on disk: skills/agents/mcp not declared ---
    for skill in &disk_skills {
        total_items += 1;
        if !declared_resolved.contains(&canonical_or_self(skill)) {
            orphaned_on_disk.push(UsageItem {
                kind: "orphaned_on_disk",
                path_or_key: skill.to_string_lossy().to_string(),
                surface: "skill",
                reason: "SKILL.md present on disk but not declared in any plugin.json".to_string(),
            });
        }
    }
    for agent in &disk_agents {
        total_items += 1;
        orphaned_on_disk.push(UsageItem {
            kind: "orphaned_on_disk",
            path_or_key: agent.to_string_lossy().to_string(),
            surface: "agent",
            reason: "agent present on disk, undeclared in plugin.json (Claude Code auto-discovers \
                     project-local agents, so this is an inventory signal, not proof of dead config)"
                .to_string(),
        });
    }
    for mcp in &mcp_files {
        if let Some(json) = read_json(mcp) {
            if let Some(servers) = json.get("mcpServers").and_then(|s| s.as_object()) {
                for name in servers.keys() {
                    total_items += 1;
                    orphaned_on_disk.push(UsageItem {
                        kind: "orphaned_on_disk",
                        path_or_key: format!("{}#{name}", mcp.to_string_lossy()),
                        surface: "mcp.json",
                        reason: "MCP server defined in .mcp.json, not declared in plugin.json"
                            .to_string(),
                    });
                }
            }
        }
    }

    // --- orphaned hooks (both directions) ---
    let mut referenced: BTreeSet<PathBuf> = BTreeSet::new();
    for settings in &settings_files {
        if let Some(json) = read_json(settings) {
            for refp in referenced_hook_paths(&json, root_path) {
                total_items += 1;
                let canon = canonical_or_self(&refp);
                referenced.insert(canon.clone());
                if !refp.is_file() {
                    orphaned_hooks.push(UsageItem {
                        kind: "orphaned_hook",
                        path_or_key: refp.to_string_lossy().to_string(),
                        surface: "hook",
                        reason: format!(
                            "referenced by {} but no such script exists on disk",
                            settings.to_string_lossy()
                        ),
                    });
                }
            }
        }
    }
    for hook in &disk_hooks {
        total_items += 1;
        if !referenced.contains(&canonical_or_self(hook)) {
            orphaned_hooks.push(UsageItem {
                kind: "orphaned_hook",
                path_or_key: hook.to_string_lossy().to_string(),
                surface: "hook",
                reason: "hook script on disk not referenced by any settings.json hook entry"
                    .to_string(),
            });
        }
    }

    // --- unknown settings keys ---
    for settings in &settings_files {
        if let Some(json) = read_json(settings) {
            if let Some(obj) = json.as_object() {
                for key in obj.keys() {
                    total_items += 1;
                    if !VALID_SETTINGS_TOP_LEVEL_KEYS.contains(&key.as_str()) {
                        unknown_settings_keys.push(UsageItem {
                            kind: "unknown_settings_key",
                            path_or_key: key.clone(),
                            surface: surface_of(&settings.to_string_lossy()),
                            reason: format!(
                                "'{key}' is not a recognized top-level settings key (dead/no-op config)"
                            ),
                        });
                    }
                }
            }
        }
    }

    let unused_items = declared_but_missing.len()
        + orphaned_on_disk.len()
        + orphaned_hooks.len()
        + unknown_settings_keys.len();
    let unused_percent = if total_items == 0 {
        0.0
    } else {
        ((unused_items as f64 / total_items as f64) * 1000.0).round() / 10.0
    };

    ConfigUsageReport {
        root: root.to_string(),
        total_items,
        unused_items,
        unused_percent,
        declared_but_missing,
        orphaned_on_disk,
        orphaned_hooks,
        unknown_settings_keys,
    }
}

/// The conformance report of all config surfaces in a workspace.
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct WorkspaceConformance {
    pub score: f64,
    pub surfaces_checked: usize,
    pub surfaces_admitted: usize,
    pub violations: Vec<String>,
}

/// Map a finding code to its severity: "error", "warning", or "info".
pub fn classify_severity(code: &str, message: &str) -> &'static str {
    match code {
        // JSON
        "CCC-JSON-001" | "CCC-JSON-002" | "CCC-JSON-003" | "CCC-JSON-004" |
        "CCC-JSON-005" | "CCC-JSON-007" | "CCC-JSON-009" | "CCC-JSON-010" |
        "CCC-JSON-011" | "CCC-JSON-012" | "CCC-JSON-013" => "error",
        "CCC-JSON-006" | "CCC-JSON-008" => "warning",

        // Markdown
        "CCC-MD-001" | "CCC-MD-006" => "error",
        "CCC-MD-002" | "CCC-MD-003" => "warning",

        // Frontmatter & Agent & Skill
        "CCC-FM-001" | "CCC-FM-002" => "error",
        "CCC-FM-003" => "warning",
        "CCC-FM-004" => "warning",
        "CCC-FM-005" => "info",

        // Skill Frontmatter specifically
        "CCC-SKILL-001" | "CCC-SKILL-002" | "CCC-SKILL-003" | "CCC-SKILL-004" |
        "CCC-SKILL-006" | "CCC-SKILL-007" | "CCC-SKILL-008" | "CCC-SKILL-009" |
        "CCC-SKILL-010" => "error",
        "CCC-SKILL-005" => "warning",

        // Agent Frontmatter specifically
        "CCC-AGENT-001" | "CCC-AGENT-002" | "CCC-AGENT-003" | "CCC-AGENT-004" |
        "CCC-AGENT-007" | "CCC-AGENT-008" | "CCC-AGENT-009" | "CCC-AGENT-010" |
        "CCC-AGENT-011" | "CCC-AGENT-012" => "error",
        "CCC-AGENT-005" => {
            if message.contains("exceeds 1024") {
                "warning"
            } else {
                "error"
            }
        }
        "CCC-AGENT-006" => "warning",

        // Hook
        "CCC-HOOK-001" => "error",
        "CCC-HOOK-002" | "CCC-HOOK-003" => "warning",

        // TOML
        "CCC-TOML-001" => "error",
        "CCC-TOML-002" => "warning",

        // Fallbacks based on category patterns
        _ if code.starts_with("CCC-JSON-") => "error",
        _ if code.starts_with("CCC-FM-") || code.starts_with("CCC-AGENT-") || code.starts_with("CCC-SKILL-") => "error",
        _ if code.starts_with("CCC-MD-") => "warning",
        _ if code.starts_with("CCC-HOOK-") => "warning",
        _ if code.starts_with("CCC-TOML-") => "warning",
        _ => "warning",
    }
}

/// Walk `root` and audit all config surfaces, returning a conformance scorecard.
pub fn conformance_report(root: &str) -> WorkspaceConformance {
    let root_path = Path::new(root);
    let mut surfaces_checked = 0;
    let mut surfaces_admitted = 0;
    let mut violations = Vec::new();
    let mut errors = 0;
    let mut warnings = 0;

    for path in scan::discover(root_path) {
        let path_str = path.to_string_lossy().to_string();
        if scan::classify(&path_str) == "unknown" {
            continue;
        }
        let Ok(raw) = std::fs::read_to_string(&path) else {
            continue;
        };
        surfaces_checked += 1;

        let relative_path = path.strip_prefix(root_path)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        let findings = scan::analyze_document(&path_str, &raw);
        let mut file_has_issues = false;

        for f in findings {
            let severity = classify_severity(&f.code, &f.message);
            match severity {
                "error" => {
                    errors += 1;
                    file_has_issues = true;
                    violations.push(format!("{}: {} {}", f.code, relative_path, f.message));
                }
                "warning" => {
                    warnings += 1;
                    file_has_issues = true;
                    violations.push(format!("{}: {} {}", f.code, relative_path, f.message));
                }
                _ => {}
            }
        }

        if !file_has_issues {
            surfaces_admitted += 1;
        }
    }

    violations.sort();

    let raw_score = (100.0 - 10.0 * (errors as f64) - 3.0 * (warnings as f64)) / 100.0;
    let score = raw_score.clamp(0.0, 1.0);

    let _ = crate::receipt::maybe_issue_receipt(root_path, score);

    WorkspaceConformance {
        score,
        surfaces_checked,
        surfaces_admitted,
        violations,
    }
}

// ---------------------------------------------------------------------------
// fix: auto-repair repairable config issues
// ---------------------------------------------------------------------------

/// A single proposed auto-fix — the file, a human-readable description of
/// what would change, and the replacement content.
#[derive(Debug, Clone, Serialize)]
pub struct FixProposal {
    /// The file that would be modified.
    pub file: String,
    /// Short description of the fix (shown in dry-run output).
    pub description: String,
    /// The full new content that would be written to `file`.
    #[serde(skip)]
    pub new_content: String,
}

/// Result of a `fix` run.
#[derive(Debug, Clone, Serialize)]
pub struct FixReport {
    pub target: String,
    pub dry_run: bool,
    pub proposals: Vec<FixProposal>,
    pub applied: usize,
}

/// Well-known `$schema` URLs for each JSON config surface.
fn schema_url_for(surface: &str) -> Option<&'static str> {
    match surface {
        "settings.json" | "settings.local.json" => {
            Some("https://raw.githubusercontent.com/anthropics/claude-code/refs/heads/main/.claude/settings.schema.json")
        }
        "plugin.json" => {
            Some("https://raw.githubusercontent.com/anthropics/claude-code/refs/heads/main/.claude-plugin/plugin.schema.json")
        }
        "marketplace.json" => {
            Some("https://raw.githubusercontent.com/anthropics/claude-code/refs/heads/main/.claude-plugin/marketplace.schema.json")
        }
        _ => None,
    }
}

/// Walk `root`, collect repairable issues, and return a [`FixReport`].
/// When `apply` is `true`, the fixes are written to disk immediately.
pub fn fix_report(root: &str, apply: bool) -> FixReport {
    let root_path = Path::new(root);
    let mut proposals: Vec<FixProposal> = Vec::new();

    for path in scan::discover(root_path) {
        let path_str = path.to_string_lossy().to_string();
        let kind = scan::classify(&path_str);
        if kind != "json" {
            continue;
        }
        let surface = surface_of(&path_str);
        let Some(schema_url) = schema_url_for(surface) else {
            continue;
        };

        let Ok(raw) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&raw) else {
            continue;
        };
        let Some(obj) = value.as_object_mut() else {
            continue;
        };

        // Only propose a fix if $schema is absent or empty.
        if obj.contains_key("$schema") {
            continue;
        }

        // Insert $schema as the first key by rebuilding the object.
        let mut new_obj = serde_json::Map::new();
        new_obj.insert("$schema".to_string(), serde_json::Value::String(schema_url.to_string()));
        for (k, v) in obj.iter() {
            new_obj.insert(k.clone(), v.clone());
        }
        let new_value = serde_json::Value::Object(new_obj);
        let Ok(new_content) = serde_json::to_string_pretty(&new_value) else {
            continue;
        };
        let new_content = format!("{}\n", new_content);

        proposals.push(FixProposal {
            file: path_str.clone(),
            description: format!(
                "Add missing \"$schema\": \"{}\" to {}",
                schema_url, surface
            ),
            new_content,
        });
    }

    let mut applied = 0;
    if apply {
        for proposal in &proposals {
            if std::fs::write(&proposal.file, &proposal.new_content).is_ok() {
                applied += 1;
            }
        }
    }

    FixReport {
        target: root.to_string(),
        dry_run: !apply,
        proposals,
        applied,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_refines_json_and_markdown() {
        assert_eq!(surface_of("/x/.claude/settings.json"), "settings.json");
        assert_eq!(surface_of("/x/.claude/settings.local.json"), "settings.local.json");
        assert_eq!(surface_of("/x/.mcp.json"), "mcp.json");
        assert_eq!(surface_of("/x/.claude-plugin/plugin.json"), "plugin.json");
        assert_eq!(surface_of("/x/.claude-plugin/marketplace.json"), "marketplace.json");
        assert_eq!(surface_of("/x/CLAUDE.md"), "claude_md");
        assert_eq!(surface_of("/x/AGENTS.md"), "agents_md");
        assert_eq!(surface_of("/x/.claude/agents/w.md"), "agent");
        assert_eq!(surface_of("/x/.claude/skills/s/SKILL.md"), "skill");
        assert_eq!(surface_of("/x/.claude/lsp-max-auto.toml"), "toml");
        assert_eq!(surface_of("/x/hooks/h.sh"), "hook");
    }

    #[test]
    fn build_tree_on_this_repo_covers_surfaces() {
        let tree = build_tree(".");
        let surfaces: BTreeSet<&str> = tree.files.iter().map(|f| f.surface).collect();
        // Representative surfaces this repo is known to contain.
        for expected in ["settings.json", "plugin.json", "agent", "skill", "toml", "hook", "mcp.json"] {
            assert!(surfaces.contains(expected), "missing surface {expected} in export tree");
        }
        // A json node's content parses to an object.
        let settings = tree
            .files
            .iter()
            .find(|f| f.surface == "settings.json")
            .expect("settings.json node");
        assert!(settings.content.is_object());
    }

    #[test]
    fn usage_report_flags_orphaned_skills_and_agents() {
        let report = usage_report(".");
        // The 5 project-local skills under .claude/skills are undeclared.
        assert!(report.orphaned_on_disk.iter().any(|i| i.surface == "skill"));
        assert!(report.orphaned_on_disk.iter().any(|i| i.surface == "agent"));
        // The declared validate-config skill resolves under .claude-plugin/, so
        // it must NOT appear as declared-but-missing.
        assert!(
            report.declared_but_missing.iter().all(|i| !i.path_or_key.contains("validate-config")),
            "validate-config resolves on disk and should not be reported missing"
        );
        assert!(report.total_items > 0);
        assert_eq!(
            report.unused_items,
            report.declared_but_missing.len()
                + report.orphaned_on_disk.len()
                + report.orphaned_hooks.len()
                + report.unknown_settings_keys.len()
        );
    }
}
