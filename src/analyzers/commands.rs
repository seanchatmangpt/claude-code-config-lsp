//! commands analyzer — slash-command markdown surface (`**/.claude/commands/*.md`)
//! Hand-written lane P9 (domain-grammar gap #3): sibling analyzers are ggen-generated;
//! this module follows the same RawFinding/Rule shapes so a future `ggen sync` can adopt it.

use std::path::Path;

use serde::Serialize;

/// A single finding from the analyzer, with span relative to input.
/// Shape-identical to `analyzers/frontmatter.rs::RawFinding`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RawFinding {
    pub code: String,
    pub message: String,
    pub span: (usize, usize),
}

/// Detection rule: code + patterns + message prefix (shape-identical to siblings).
#[derive(Debug, Clone)]
pub struct Rule {
    pub code: &'static str,
    pub patterns: Vec<String>,
    pub message_prefix: &'static str,
}

impl Rule {
    pub fn new(code: &'static str, patterns: Vec<String>, message_prefix: &'static str) -> Self {
        Self {
            code,
            patterns,
            message_prefix,
        }
    }
}

// ── Diagnostic codes (declared in schema/claude-code-config.ttl) ──────────────

/// CCC-CMD-001: command has no description surface (frontmatter `description:` or H1).
pub const CMD_001: &str = "CCC-CMD-001";
/// CCC-CMD-002: argument-hint implies arguments but body has no `$ARGUMENTS`.
pub const CMD_002: &str = "CCC-CMD-002";
/// CCC-CMD-003: `@file` reference does not resolve relative to the command file.
pub const CMD_003: &str = "CCC-CMD-003";
/// CCC-CMD-004: `allowed-tools` entry does not match `Tool(pattern)` grammar.
pub const CMD_004: &str = "CCC-CMD-004";
/// CCC-CMD-005: frontmatter field present with invalid value/type.
pub const CMD_005: &str = "CCC-CMD-005";
/// CCC-CMD-006: `` !`cmd` `` shell-prefixed span not closed on its line.
pub const CMD_006: &str = "CCC-CMD-006";

#[allow(dead_code)]
fn analyze_rule_naive(input: &str, rule: &Rule, findings: &mut Vec<RawFinding>) {
    for pattern in &rule.patterns {
        if pattern.is_empty() {
            continue;
        }
        let mut start = 0;
        while let Some(pos) = input[start..].find(pattern.as_str()) {
            let abs = start + pos;
            findings.push(RawFinding {
                code: rule.code.to_string(),
                message: format!("{}: {pattern}", rule.message_prefix),
                span: (abs, abs + pattern.len()),
            });
            start = abs + pattern.len();
        }
    }
}

/// Pure, deterministic validation of one slash-command markdown file.
/// `base_dir` is the directory containing the command file; `@file` references
/// resolve against it. `None` skips filesystem resolution (no CCC-CMD-003 emitted).
pub fn validate_command_file(content: &str, base_dir: Option<&Path>) -> Vec<RawFinding> {
    let mut findings = Vec::new();

    let (frontmatter, fm_range, body, body_offset) = split_frontmatter(content);

    let mut has_description = false;
    let mut argument_hint: Option<String> = None;

    if let Some(fm) = frontmatter {
        let mut in_tools_block = false;
        let mut offset = fm_range.0;
        for raw_line in fm.lines() {
            let line = raw_line.trim();
            let line_start = offset;
            offset += raw_line.len() + 1;

            if line.starts_with("description:") {
                in_tools_block = false;
                let val = line
                    .trim_start_matches("description:")
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'');
                has_description = !val.is_empty();
                if val.is_empty() {
                    findings.push(RawFinding {
                        code: CMD_001.into(),
                        message: "Command description is present but empty".into(),
                        span: (line_start, line_start + raw_line.len()),
                    });
                }
            } else if line.starts_with("allowed-tools:") {
                in_tools_block = true;
                let after = line.trim_start_matches("allowed-tools:").trim();
                if after.starts_with('[') {
                    in_tools_block = false;
                    let inner = after.trim_start_matches('[').trim_end_matches(']');
                    for entry in inner.split(',') {
                        validate_tools_entry(entry.trim(), line_start, &mut findings);
                    }
                }
            } else if in_tools_block && line.starts_with("- ") {
                validate_tools_entry(
                    line.trim_start_matches('-').trim(),
                    line_start,
                    &mut findings,
                );
            } else if !line.starts_with('-') && !line.starts_with(' ') {
                in_tools_block = false;
            }

            if let Some(val) = line.strip_prefix("argument-hint:") {
                let val = val.trim().trim_matches('"').trim_matches('\'');
                if val.is_empty() {
                    findings.push(RawFinding {
                        code: CMD_005.into(),
                        message: "argument-hint is present but empty".into(),
                        span: (line_start, line_start + raw_line.len()),
                    });
                } else {
                    argument_hint = Some(val.to_string());
                }
            }

            if let Some(val) = line.strip_prefix("disable-model-invocation:") {
                let val = val.trim();
                if val != "true" && val != "false" {
                    findings.push(RawFinding {
                        code: CMD_005.into(),
                        message: format!(
                            "disable-model-invocation must be true or false, got '{val}'"
                        ),
                        span: (line_start, line_start + raw_line.len()),
                    });
                }
            }
        }

        if !has_description {
            findings.push(RawFinding {
                code: CMD_001.into(),
                message: "Command frontmatter is missing a description".into(),
                span: fm_range,
            });
        }
    }

    // Frontmatter-or-H1: a command with neither frontmatter description nor an H1
    // title has no description surface at all.
    let has_h1 = body
        .lines()
        .any(|l| l.trim().starts_with("# ") && !l.trim().starts_with("## "));
    if frontmatter.is_none() && !has_h1 {
        findings.push(RawFinding {
            code: CMD_001.into(),
            message: "Command file has neither YAML frontmatter nor an H1 title".into(),
            span: (0, 0),
        });
    }

    // CCC-CMD-002: argument-hint implies arguments; body must carry $ARGUMENTS.
    let args_count = body.matches("$ARGUMENTS").count();
    if argument_hint.is_some() && args_count == 0 {
        findings.push(RawFinding {
            code: CMD_002.into(),
            message: "argument-hint is set but body has no $ARGUMENTS placeholder".into(),
            span: (body_offset, body_offset + body.len().min(64)),
        });
    }

    // CCC-CMD-006: `` !`cmd` `` spans must close on their line.
    check_shell_spans(body, body_offset, &mut findings);

    // CCC-CMD-003: @file references must resolve relative to the command file.
    if let Some(dir) = base_dir {
        check_at_refs(content, dir, &mut findings);
    }

    findings.sort_by(|a, b| a.span.0.cmp(&b.span.0).then(a.code.cmp(&b.code)));
    findings
}

fn validate_tools_entry(entry: &str, line_start: usize, findings: &mut Vec<RawFinding>) {
    if entry.is_empty() {
        return;
    }
    let bad = |findings: &mut Vec<RawFinding>| {
        findings.push(RawFinding {
            code: CMD_004.into(),
            message: format!("allowed-tools entry '{entry}' does not match Tool(pattern) grammar"),
            span: (line_start, line_start + entry.len()),
        });
    };
    let Some(open) = entry.find('(') else {
        bad(findings);
        return;
    };
    let name = &entry[..open];
    let name_ok = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':');
    if !name_ok || !entry.ends_with(')') {
        bad(findings);
    }
}

/// `` !`cmd` `` shell-prefixed spans must be closed-backtick on the same line.
fn check_shell_spans(body: &str, body_offset: usize, findings: &mut Vec<RawFinding>) {
    for raw_line in body.lines() {
        let trim_lead = raw_line.len() - raw_line.trim_start().len();
        let line = raw_line.trim();
        let mut search = 0usize;
        while let Some(rel) = line[search..].find("!`") {
            let open_abs = body_offset + trim_lead + search + rel;
            let span_tail = &line[search + rel + 2..];
            match span_tail.find('`') {
                Some(close_rel) => {
                    search = search + rel + 2 + close_rel + 1;
                }
                None => {
                    findings.push(RawFinding {
                        code: CMD_006.into(),
                        message:
                            "Shell-prefix span !`...` is not closed with a backtick on the same line"
                                .into(),
                        span: (open_abs, open_abs + 2),
                    });
                    search = search + rel + 2;
                }
            }
        }
    }
}

/// `@path` references must resolve relative to the command file's directory.
/// Skips references containing `$` (runtime-substituted paths).
fn check_at_refs(content: &str, dir: &Path, findings: &mut Vec<RawFinding>) {
    let bytes = content.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'@' && (i == 0 || !bytes[i - 1].is_ascii_alphanumeric()) {
            let start = i + 1;
            let mut end = start;
            while end < bytes.len()
                && (bytes[end].is_ascii_alphanumeric()
                    || matches!(bytes[end], b'.' | b'/' | b'-' | b'_'))
            {
                end += 1;
            }
            let path_str = &content[start..end];
            if !path_str.is_empty() && !path_str.contains('$') {
                let candidate = dir.join(path_str);
                if !candidate.is_file() {
                    findings.push(RawFinding {
                        code: CMD_003.into(),
                        message: format!(
                            "@file reference '{path_str}' does not resolve relative to the command file"
                        ),
                        span: (i, end),
                    });
                }
            }
            i = end;
        } else {
            i += 1;
        }
    }
}

fn split_frontmatter(content: &str) -> (Option<&str>, (usize, usize), &str, usize) {
    let trimmed_start = content.trim_start();
    if !trimmed_start.starts_with("---") {
        return (None, (0, 0), content, 0);
    }
    let lead = content.len() - trimmed_start.len();
    let after_open = lead + 3;
    let rest = &content[after_open..];
    let Some(close_rel) = rest.find("\n---") else {
        return (None, (0, 0), content, 0);
    };
    let fm_start = after_open;
    let fm_end = after_open + close_rel;
    let fm = &content[fm_start..fm_end];
    let after_close_rel = close_rel + 4;
    let body_start_rel = rest[after_close_rel..]
        .find('\n')
        .map(|p| after_close_rel + p + 1)
        .unwrap_or(rest.len());
    let body_start = after_open + body_start_rel;
    let body = &content[body_start..];
    (Some(fm), (fm_start, fm_end), body, body_start)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = "---\ndescription: Create the spec.\nargument-hint: [feature]\n---\n\n## User Input\n\n```text\n$ARGUMENTS\n```\n";

    #[test]
    fn valid_command_has_no_findings() {
        assert_eq!(validate_command_file(GOOD, None), vec![]);
    }

    #[test]
    fn ccc_cmd_001_missing_description() {
        let i = "---\nname: x\n---\nbody\n";
        assert!(validate_command_file(i, None)
            .iter()
            .any(|f| f.code == CMD_001));
    }

    #[test]
    fn h1_counts_as_description_surface() {
        let i = "# My Command\n\nDo the thing.\n";
        assert!(validate_command_file(i, None).is_empty());
    }

    #[test]
    fn ccc_cmd_001_neither_frontmatter_nor_h1() {
        let i = "Just some prose.\n";
        assert!(validate_command_file(i, None)
            .iter()
            .any(|f| f.code == CMD_001));
    }

    #[test]
    fn ccc_cmd_002_hint_without_arguments() {
        let i = "---\ndescription: d\nargument-hint: [x]\n---\n\nNo placeholder here.\n";
        assert!(validate_command_file(i, None)
            .iter()
            .any(|f| f.code == CMD_002));
    }

    #[test]
    fn hint_with_arguments_is_clean() {
        let i = "---\ndescription: d\nargument-hint: [x]\n---\n\n$ARGUMENTS\n";
        assert!(validate_command_file(i, None).is_empty());
    }

    #[test]
    fn ccc_cmd_003_unresolvable_at_file() {
        let i = "---\ndescription: d\n---\n\nSee @definitely-missing-file.md\n";
        let dir = std::env::temp_dir();
        assert!(validate_command_file(i, Some(&dir))
            .iter()
            .any(|f| f.code == CMD_003));
    }

    #[test]
    fn resolvable_at_file_is_clean() {
        let dir = std::env::temp_dir();
        let marker = dir.join("ccc-cmd-resolve-marker");
        std::fs::write(&marker, b"x").unwrap();
        let i = "---\ndescription: d\n---\n\nSee @ccc-cmd-resolve-marker\n".to_string();
        let f = validate_command_file(&i, Some(&dir));
        let _ = std::fs::remove_file(&marker);
        assert!(!f.iter().any(|x| x.code == CMD_003), "{f:?}");
    }

    #[test]
    fn at_ref_with_variable_is_skipped() {
        let i = "---\ndescription: d\n---\n\nSee @$ARGS/spec.md\n";
        assert!(validate_command_file(i, None).is_empty());
    }

    #[test]
    fn ccc_cmd_004_bad_tools_grammar() {
        let i = "---\ndescription: d\nallowed-tools:\n  - Bash\n  - Read\n---\n";
        assert!(validate_command_file(i, None)
            .iter()
            .any(|f| f.code == CMD_004));
    }

    #[test]
    fn good_tools_grammar_clean() {
        let i = "---\ndescription: d\nallowed-tools: [Bash(git *), Read(*)]\n---\n";
        assert!(!validate_command_file(i, None)
            .iter()
            .any(|f| f.code == CMD_004));
    }

    #[test]
    fn ccc_cmd_005_bad_bool() {
        let i = "---\ndescription: d\ndisable-model-invocation: maybe\n---\n";
        assert!(validate_command_file(i, None)
            .iter()
            .any(|f| f.code == CMD_005));
    }

    #[test]
    fn good_bool_clean() {
        let i = "---\ndescription: d\ndisable-model-invocation: true\n---\n";
        assert!(validate_command_file(i, None).is_empty());
    }

    #[test]
    fn ccc_cmd_006_unclosed_shell_span() {
        let i = "---\ndescription: d\n---\n\nRun !`git status now\n";
        assert!(validate_command_file(i, None)
            .iter()
            .any(|f| f.code == CMD_006));
    }

    #[test]
    fn closed_shell_span_clean() {
        let i = "---\ndescription: d\n---\n\nRun !`git status` now\n";
        assert!(!validate_command_file(i, None)
            .iter()
            .any(|f| f.code == CMD_006));
    }

    #[test]
    fn findings_are_sorted() {
        let i = "Just prose.\nRun !`git status now\n";
        let f = validate_command_file(i, None);
        assert!(f.windows(2).all(|w| w[0].span.0 <= w[1].span.0));
    }
}
