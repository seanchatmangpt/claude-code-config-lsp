//! Chicago-style integration tests for code_lens + signature_help.
//! Real tree-sitter JSON parses, real corpus slices, no mocks.

use claude_code_config_lsp::{code_lens, signature_help};

const SETTINGS: &str = r#"{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "echo hi",
            "timeout": 30
          }
        ]
      }
    ]
  }
}"#;

const SKILL_DOC: &str = "---\nname: testing-chicago-style\n---\nUse real collaborators.";

fn corpus() -> Vec<String> {
    vec![
        "invoked testing-chicago-style earlier".to_string(),
        "no skill here".to_string(),
        "testing-chicago-style twice: testing-chicago-style".to_string(),
    ]
}

fn help(content: &str, line: u32, character: u32) -> Option<lsp_types_max::SignatureHelp> {
    signature_help::signature_help_at_position(content, lsp_types_max::Position { line, character })
}

#[test]
fn settings_json_gets_conformance_lens_with_doc_uri_arg() {
    let uri = "file:///repo/.claude/settings.json";
    let lenses = code_lens::code_lens(uri, SETTINGS, &[]);
    assert_eq!(lenses.len(), 1);
    let cmd = lenses[0].command.as_ref().expect("lens carries a command");
    assert_eq!(cmd.command, "claude-code-config/conformanceAudit");
    assert_eq!(
        cmd.arguments.as_ref().and_then(|a| a.first()),
        Some(&serde_json::json!(uri))
    );
    // Lens range is the first line of the document.
    assert_eq!(lenses[0].range.start.line, 0);
    assert_eq!(lenses[0].range.end.line, 0);
    assert!(lenses[0].range.end.character > 0);
}

#[test]
fn skill_md_lens_counts_corpus_invocations() {
    let uri = "file:///repo/skills/testing-chicago-style/SKILL.md";
    let lenses = code_lens::code_lens(uri, SKILL_DOC, &corpus());
    assert_eq!(lenses.len(), 1);
    let title = lenses[0]
        .command
        .as_ref()
        .map(|c| c.title.clone())
        .expect("command");
    assert!(
        title.contains("testing-chicago-style") && title.contains('3'),
        "title must carry the real count (3): got {title}"
    );
    // Range is line 0.
    assert_eq!(lenses[0].range.start.line, 0);
}

#[test]
fn claude_md_lens_counts_wikilinks() {
    let content = "# Index\n\nSee [[operating-doctrine]] and [[dfcm-composition]] plus [[operating-doctrine]] again.";
    let lenses = code_lens::code_lens("file:///repo/CLAUDE.md", content, &[]);
    assert_eq!(lenses.len(), 1);
    let title = lenses[0]
        .command
        .as_ref()
        .map(|c| c.title.clone())
        .expect("command");
    assert!(
        title.contains('3'),
        "wikilink count lens must say 3, got {title}"
    );
}

#[test]
fn unrelated_doc_gets_no_lens() {
    let lenses = code_lens::code_lens("file:///repo/src/main.rs", "fn main() {}", &corpus());
    assert!(lenses.is_empty());
}

#[test]
fn signature_help_on_matcher_key_of_hook_entry() {
    // Line 4 (0-based) is `        "matcher": "Bash",`
    let h = help(SETTINGS, 4, 10).expect("signature help inside a hook entry");
    assert_eq!(h.signatures.len(), 1);
    assert_eq!(h.signatures[0].label, signature_help::HOOK_SIGNATURE_LABEL);
    assert_eq!(h.active_parameter, Some(0), "matcher is param 0");
}

#[test]
fn signature_help_on_command_key_of_command_entry() {
    // Line 8 (0-based) is `            "command": "echo hi",`
    let h = help(SETTINGS, 8, 16).expect("signature help inside a command entry");
    assert_eq!(h.signatures[0].label, signature_help::HOOK_SIGNATURE_LABEL);
    assert_eq!(
        h.active_parameter,
        Some(1),
        "command is param 1 of command grammar"
    );
}

#[test]
fn signature_help_none_outside_hooks() {
    // Line 1 is `"hooks": {` — cursor on the top-level `hooks` key itself is the
    // event-map level, not a hook entry.
    let h = help(SETTINGS, 1, 4);
    assert!(
        h.is_none(),
        "cursor outside a hook entry must return None, got {h:?}"
    );
}
