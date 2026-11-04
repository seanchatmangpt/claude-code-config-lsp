//! Handler: textDocument/codeLens
//!
//! Lens set per document family:
//! - settings.json  -> "Run conformance audit" lens whose command is
//!   `claude-code-config/conformanceAudit` with the doc URI as the sole
//!   argument (the executeCommand handler already exists in backend.rs).
//! - SKILL.md       -> "invocation count" lens: occurrences of the skill
//!   name across a corpus passed in as an index parameter.
//! - CLAUDE.md      -> "wikilink count" lens: `[[...]]` links in the doc.
//!
//! All lens ranges cover the first line of the document.

use lsp_types_max::{CodeLens, Command, Range};

/// True when the URI classifies as a Claude Code settings.json surface
/// (`.claude/settings*.json`, same glob family as file_types.rs).
pub fn is_settings_json(uri: &str) -> bool {
    // Strip scheme so the query-string of virtual docs does not break the
    // suffix check.
    let path = uri.strip_prefix("file://").unwrap_or(uri);
    let file_name = path.rsplit('/').next().unwrap_or(path);
    file_name == "settings.json" || file_name == "settings.local.json"
}

/// True when the URI points at a SKILL.md document.
pub fn is_skill_md(uri: &str) -> bool {
    let path = uri.strip_prefix("file://").unwrap_or(uri);
    path.rsplit('/').next() == Some("SKILL.md")
}

/// True when the URI points at a CLAUDE.md document.
pub fn is_claude_md(uri: &str) -> bool {
    let path = uri.strip_prefix("file://").unwrap_or(uri);
    let file_name = path.rsplit('/').next().unwrap_or(path);
    file_name == "CLAUDE.md"
}

/// First-line range shared by every lens this module produces.
fn first_line_range(content: &str) -> Range {
    // Columns are UTF-16 per LSP; the lens only needs the line extent.
    let u16_len: usize = content
        .lines()
        .next()
        .map_or(0, |l| l.chars().map(|c| c.len_utf16()).sum());
    Range {
        start: lsp_types_max::Position {
            line: 0,
            character: 0,
        },
        end: lsp_types_max::Position {
            line: 0,
            character: u16_len as u32,
        },
    }
}

/// Skill name for a SKILL.md document: frontmatter `name:` field if
/// present, otherwise the parent directory name.
pub fn skill_name(uri: &str, content: &str) -> String {
    for line in content.lines() {
        let Some(stripped) = line.strip_prefix("name:") else {
            continue;
        };
        let name = stripped.trim();
        if !name.is_empty() {
            return name.to_string();
        }
    }
    // Fall back to the parent directory of SKILL.md.
    let path = uri.strip_prefix("file://").unwrap_or(uri);
    let dir = path.rsplit_once('/').map_or(path, |(d, _)| d);
    dir.rsplit('/').next().unwrap_or(dir).to_string()
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut n = 0;
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(needle) {
        n += 1;
        start += pos + needle.len();
    }
    n
}

/// Total occurrences of the skill name across the corpus docs.
pub fn skill_invocation_count(skill: &str, corpus: &[String]) -> usize {
    corpus.iter().map(|doc| count_occurrences(doc, skill)).sum()
}

/// Count `[[wikilink]]` targets in a markdown doc.
pub fn wikilink_count(content: &str) -> usize {
    count_occurrences(content, "[[")
}

/// Compute the code lenses for a document. Returns an empty vec when the
/// URI does not classify to a lens-bearing family.
pub fn code_lens(uri: &str, content: &str, corpus: &[String]) -> Vec<CodeLens> {
    let range = first_line_range(content);
    let mut lenses = Vec::new();

    if is_settings_json(uri) {
        lenses.push(CodeLens {
            range,
            command: Some(Command {
                title: "Run conformance audit".to_string(),
                command: "claude-code-config/conformanceAudit".to_string(),
                arguments: Some(vec![serde_json::Value::String(uri.to_string())]),
            }),
            data: None,
        });
        return lenses;
    }

    if is_skill_md(uri) {
        let skill = skill_name(uri, content);
        let count = skill_invocation_count(&skill, corpus);
        lenses.push(CodeLens {
            range,
            command: Some(Command {
                title: format!("{count} invocations of `{skill}` in corpus"),
                // Resolved/refreshed by the client via codeLens/resolve;
                // no server-side executeCommand exists for this surface.
                command: "codeLens/resolve".to_string(),
                arguments: Some(vec![serde_json::Value::String(uri.to_string())]),
            }),
            data: None,
        });
        return lenses;
    }

    if is_claude_md(uri) {
        let count = wikilink_count(content);
        lenses.push(CodeLens {
            range,
            command: Some(Command {
                title: format!("{count} wikilinks"),
                command: "codeLens/resolve".to_string(),
                arguments: Some(vec![serde_json::Value::String(uri.to_string())]),
            }),
            data: None,
        });
    }

    lenses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_uri_families() {
        assert!(is_settings_json("file:///repo/.claude/settings.json"));
        assert!(is_settings_json("file:///repo/.claude/settings.local.json"));
        assert!(!is_settings_json("file:///repo/package.json"));
    }

    #[test]
    fn skill_name_prefers_frontmatter() {
        let content = "---\nname: testing-chicago-style\n---\nbody";
        assert_eq!(
            skill_name("file:///skills/testing-chicago-style/SKILL.md", content),
            "testing-chicago-style"
        );
        assert_eq!(
            skill_name("file:///skills/my-skill/SKILL.md", "no frontmatter"),
            "my-skill"
        );
    }

    #[test]
    fn lens_counts_are_real_counts() {
        let corpus = vec![
            "use the ggen skill then ggen again".to_string(),
            "ggen packs".to_string(),
            "unrelated".to_string(),
        ];
        assert_eq!(skill_invocation_count("ggen", &corpus), 3);
        assert_eq!(wikilink_count("a [[b]] and [[c]][[d]]"), 3);
        assert_eq!(wikilink_count("no links"), 0);
    }
}
