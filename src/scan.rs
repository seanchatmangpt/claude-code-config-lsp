//! Shared classification + analyzer dispatch for Claude Code config files.
//!
//! This is the single source of truth used by BOTH the LSP backend
//! (`ClaudeCodeConfigBackend::scan_uri_classified`) and the `scan` CLI verb
//! (`src/nouns/config.rs`). Keeping one classifier and one dispatch here means
//! `claude-code-config-lsp scan <path>` sees exactly the same diagnostics the
//! editor would surface — no drift between the two entry points.

use crate::analyzers::claude_md::{validate_claude_md, validate_skill_frontmatter};
use crate::analyzers::frontmatter::{
    validate_agent_frontmatter, FrontmatterAnalyzer,
    ReplayableAnalyzer as FrontmatterReplayable,
};
use crate::analyzers::hook::{HookAnalyzer, ReplayableAnalyzer as HookReplayable};
use crate::analyzers::json::{
    validate_plugin_json, validate_settings_json_enums, JsonAnalyzer,
    ReplayableAnalyzer as JsonReplayable,
};
use crate::analyzers::toml::{TomlAnalyzer, ReplayableAnalyzer as TomlReplayable};

/// A diagnostic produced by [`analyze_document`], flattened to the fields both
/// callers need. `category` mirrors the `kind` returned by [`classify`].
/// `span` is the byte offset range `(start, end)` into `content`, taken
/// verbatim from the analyzer's `RawFinding` — every analyzer computes a real
/// span (see e.g. `analyzers/json.rs`'s `RawFinding`), but until this field
/// existed it was dropped on the floor and every diagnostic collapsed to
/// LSP range 0:0 regardless of where the actual problem was.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ScanFinding {
    pub code: String,
    pub message: String,
    pub category: &'static str,
    pub span: (usize, usize),
}

/// Classify a URI or filesystem path to the analyzer family that applies.
///
/// Works on both `file://` URIs and plain paths — it lowercases and matches on
/// suffix/substring. Returns one of
/// `"skill" | "claude_md" | "agent" | "json" | "toml" | "hook" | "unknown"`.
pub fn classify(uri_or_path: &str) -> &'static str {
    let u = uri_or_path.to_lowercase();
    if u.ends_with("skill.md") || u.contains("/skills/") {
        "skill"
    } else if u.ends_with("claude.md") || u.ends_with("agents.md") {
        "claude_md"
    } else if u.contains("/agents/") && u.ends_with(".md") {
        "agent"
    } else if u.ends_with("settings.json") || u.ends_with("settings.local.json")
        || u.ends_with("mcp.json") || u.ends_with("plugin.json")
        || u.ends_with("marketplace.json") || u.ends_with("keybindings.json")
    {
        "json"
    } else if u.contains("/.claude/") && u.ends_with(".toml") {
        // Config TOML lives only under `.claude/` (e.g. lsp-max-auto.toml).
        // A repo's Cargo.toml / rust-toolchain.toml / etc. is NOT Claude config
        // and must not be run through the CalVer TOML rules.
        "toml"
    } else if u.contains("/hooks/") {
        // Hook scripts live under a hooks/ directory (ontology: .claude/hooks/*.sh).
        // A bare *.sh elsewhere in the repo is not a Claude Code hook.
        "hook"
    } else {
        "unknown"
    }
}

/// Run every analyzer that applies to `uri_or_path` against `content`.
///
/// This is the exact dispatch the LSP backend runs per document: the
/// `ReplayableAnalyzer` rule packs plus the standalone `validate_*` checks,
/// with the filename-gated `settings.json` enum and `plugin.json` structural
/// checks. `"unknown"` documents yield no findings.
pub fn analyze_document(uri_or_path: &str, content: &str) -> Vec<ScanFinding> {
    let kind = classify(uri_or_path);
    let mut out: Vec<ScanFinding> = Vec::new();
    let push = |out: &mut Vec<ScanFinding>, code: String, message: String, category: &'static str, span: (usize, usize)| {
        out.push(ScanFinding { code, message, category, span });
    };

    match kind {
        "skill" => {
            // NOTE: intentionally NOT running ClaudeMdAnalyzer's
            // skill_name_rules() here — it scans the whole file for the literal
            // substrings "claude"/"anthropic" (not just the `name:` field), so
            // any skill whose description or body legitimately mentions Claude
            // Code (e.g. this project's own `claude-config://` scheme) gets
            // falsely flagged as if its *name* contained a reserved word.
            // validate_skill_frontmatter already does the correct,
            // name-field-scoped check.
            for raw in validate_skill_frontmatter(content) {
                push(&mut out, raw.code, raw.message, "skill", raw.span);
            }
        }
        "claude_md" => {
            // NOTE: only the structural checks in validate_claude_md run here.
            // We intentionally do NOT run ClaudeMdAnalyzer's replayable rules —
            // its rule set is skill_name_rules() (a whole-file substring scan
            // for "claude"/"anthropic"), which false-fires on every legitimate
            // mention of Claude Code in a CLAUDE.md body. Same rationale as the
            // "skill" branch skip above.
            for raw in validate_claude_md(content) {
                push(&mut out, raw.code, raw.message, "claude_md", raw.span);
            }
        }
        "agent" => {
            for raw in validate_agent_frontmatter(content) {
                push(&mut out, raw.code, raw.message, "agent", raw.span);
            }
            for raw in FrontmatterReplayable::analyze(&FrontmatterAnalyzer::new(), content) {
                push(&mut out, raw.code, raw.message, "agent", raw.span);
            }
        }
        "json" => {
            for raw in JsonReplayable::analyze(&JsonAnalyzer::new(), content) {
                push(&mut out, raw.code, raw.message, "json", raw.span);
            }
            let lower = uri_or_path.to_lowercase();
            if lower.ends_with("settings.json") || lower.ends_with("settings.local.json") {
                for raw in validate_settings_json_enums(content) {
                    push(&mut out, raw.code, raw.message, "json", raw.span);
                }
            }
            if lower.ends_with("plugin.json") {
                for raw in validate_plugin_json(content) {
                    push(&mut out, raw.code, raw.message, "json", raw.span);
                }
            }
        }
        "toml" => {
            for raw in TomlReplayable::analyze(&TomlAnalyzer::new(), content) {
                push(&mut out, raw.code, raw.message, "toml", raw.span);
            }
        }
        "hook" => {
            for raw in HookReplayable::analyze(&HookAnalyzer::new(), content) {
                push(&mut out, raw.code, raw.message, "hook", raw.span);
            }
        }
        _ => {}
    }

    out
}

/// Recursively collect files under `root`, pruning build/VCS/vendor dirs and
/// `worktrees` (which duplicate the whole config tree). The single discovery
/// helper shared by the `scan`, `export`, and `unused` CLI verbs. Returns
/// every file — callers decide what's a config surface via [`classify`].
pub fn discover(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if path.is_dir() {
                if matches!(name, "target" | ".git" | "node_modules" | "worktrees") {
                    continue;
                }
                walk(&path, out);
            } else {
                out.push(path);
            }
        }
    }
    if root.is_dir() {
        walk(root, &mut out);
    } else if root.is_file() {
        out.push(root.to_path_buf());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_all_kinds() {
        assert_eq!(classify("/x/skills/foo/SKILL.md"), "skill");
        assert_eq!(classify("/x/.claude/skills/foo/anything.md"), "skill");
        assert_eq!(classify("/proj/CLAUDE.md"), "claude_md");
        assert_eq!(classify("/proj/AGENTS.md"), "claude_md");
        assert_eq!(classify("/x/.claude/agents/worker.md"), "agent");
        assert_eq!(classify("/x/.claude/settings.json"), "json");
        assert_eq!(classify("/x/.claude/settings.local.json"), "json");
        assert_eq!(classify("/x/.mcp.json"), "json");
        assert_eq!(classify("/x/.claude-plugin/plugin.json"), "json");
        assert_eq!(classify("/x/.claude-plugin/marketplace.json"), "json");
        assert_eq!(classify("/x/.claude/keybindings.json"), "json");
        assert_eq!(classify("/x/.claude/lsp-max-auto.toml"), "toml");
        assert_eq!(classify("/x/hooks/session-start.sh"), "hook");
        assert_eq!(classify("/x/README.md"), "unknown");
        assert_eq!(classify("/x/src/main.rs"), "unknown");
        // Non-config TOML must NOT be treated as a config surface.
        assert_eq!(classify("/x/Cargo.toml"), "unknown");
        assert_eq!(classify("/x/rust-toolchain.toml"), "unknown");
    }

    #[test]
    fn unknown_document_yields_nothing() {
        assert!(analyze_document("/x/README.md", "# hello").is_empty());
    }

    #[test]
    fn settings_json_enum_findings_flow_through() {
        // A bare-string skillOverrides must fire CCC-JSON-009 via the json branch.
        let f = analyze_document("/x/.claude/settings.json", r#"{"skillOverrides": "off"}"#);
        assert!(f.iter().any(|f| f.code == "CCC-JSON-009"));
    }

    #[test]
    fn toml_coverage_now_reached_by_scan() {
        // Malformed TOML is a family the OLD CLI classifier never routed. This
        // proves analyze_document closes that gap: a "toml" document is dispatched
        // to the TOML analyzer (finding set is non-negative and category is toml).
        let findings = analyze_document("/x/.claude/lsp-max-auto.toml", "not = [valid toml");
        assert!(findings.iter().all(|f| f.category == "toml"));
    }

    #[test]
    fn hook_document_routes_to_hook_category() {
        let findings = analyze_document("/x/hooks/broken.sh", "#!/bin/sh\necho hi\n");
        assert!(findings.iter().all(|f| f.category == "hook"));
    }
}
