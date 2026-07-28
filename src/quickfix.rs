//! Pure text-fix logic for `textDocument/codeAction`. Deliberately separated
//! from `backend.rs` so the mapping from a diagnostic's exact message text to
//! its replacement is unit-testable without spinning up an LSP session.
//!
//! Scope note: only diagnostics that carry a REAL byte span can be safely
//! turned into a `TextEdit` that replaces the offending text — `CCC-JSON-001`
//! and `CCC-JSON-002` come from the pattern-matched `ReplayableAnalyzer`
//! (real spans, see `analyzers/json.rs`'s `analyze_rule_naive`). The
//! `serde_json::Value`-based enum validators (`CCC-JSON-004/005/006`, model/
//! effortLevel/permissionMode) still emit a `(0, 0)` span (documented in
//! `scan.rs`'s `ScanFinding`), so they are NOT wired here — offering a
//! "quick fix" that replaces line 0, column 0 regardless of where the
//! invalid value actually is would silently corrupt an unrelated part of the
//! document. Closing that gap requires tracking real offsets through
//! `serde_json::Value` first (Phase 2 of the roadmap), not a code-action
//! trick.

/// Exact `message_prefix` strings from `analyzers::json::settings_json_rules()`
/// — MUST stay byte-for-byte identical to what produces the diagnostic
/// message, since matching works by stripping this prefix off.
const DEPRECATED_KEY_PREFIX: &str = "Deprecated settings.json key — use mcpServers";
const HOOK_KEY_PREFIX: &str = "Wrong hook key format — use hooks.PreToolUse / hooks.PostToolUse";

/// For a `CCC-JSON-001` diagnostic message, the replacement key name — always
/// `mcpServers`, the one correct spelling all three deprecated keys map to.
pub fn deprecated_key_replacement(message: &str) -> Option<&'static str> {
    let matched = message.strip_prefix(DEPRECATED_KEY_PREFIX)?.strip_prefix(": ")?;
    match matched {
        "enabledMcpjsonServers" | "disabledMcpjsonServers" | "mcpJsonServers" => Some("mcpServers"),
        _ => None,
    }
}

/// For a `CCC-JSON-002` diagnostic message, the replacement key name — depends
/// on which of the four wrong-cased spellings matched.
pub fn hook_key_replacement(message: &str) -> Option<&'static str> {
    let matched = message.strip_prefix(HOOK_KEY_PREFIX)?.strip_prefix(": ")?;
    match matched {
        "hookEvents" | "hook_events" => Some("hooks"),
        "pre_tool_use" => Some("PreToolUse"),
        "post_tool_use" => Some("PostToolUse"),
        _ => None,
    }
}

/// Byte offset just after the first `{` in `content`, i.e. a safe insertion
/// point for a new leading JSON object member. `None` if there's no `{` at
/// all (malformed/empty document — nothing to insert into).
pub fn first_object_member_insert_offset(content: &str) -> Option<usize> {
    content.find('{').map(|i| i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deprecated_key_replacement_matches_all_three_patterns() {
        for key in ["enabledMcpjsonServers", "disabledMcpjsonServers", "mcpJsonServers"] {
            let msg = format!("{DEPRECATED_KEY_PREFIX}: {key}");
            assert_eq!(deprecated_key_replacement(&msg), Some("mcpServers"));
        }
    }

    #[test]
    fn deprecated_key_replacement_rejects_unrelated_message() {
        assert_eq!(deprecated_key_replacement("some other diagnostic"), None);
    }

    #[test]
    fn hook_key_replacement_matches_all_four_patterns() {
        assert_eq!(hook_key_replacement(&format!("{HOOK_KEY_PREFIX}: hookEvents")), Some("hooks"));
        assert_eq!(hook_key_replacement(&format!("{HOOK_KEY_PREFIX}: hook_events")), Some("hooks"));
        assert_eq!(hook_key_replacement(&format!("{HOOK_KEY_PREFIX}: pre_tool_use")), Some("PreToolUse"));
        assert_eq!(hook_key_replacement(&format!("{HOOK_KEY_PREFIX}: post_tool_use")), Some("PostToolUse"));
    }

    #[test]
    fn first_object_member_insert_offset_finds_position_after_brace() {
        assert_eq!(first_object_member_insert_offset(r#"{"name": "x"}"#), Some(1));
        assert_eq!(first_object_member_insert_offset("   {\n  \"a\": 1\n}"), Some(4));
    }

    #[test]
    fn first_object_member_insert_offset_none_without_brace() {
        assert_eq!(first_object_member_insert_offset(""), None);
        assert_eq!(first_object_member_insert_offset("not json"), None);
    }

    /// End-to-end proof these two functions produce output that actually
    /// round-trips through the real analyzer that generates the diagnostic
    /// message in the first place — not just that the prefix constants
    /// happen to match today.
    #[test]
    fn replacement_functions_agree_with_the_real_analyzer_output() {
        use crate::analyzers::json::{settings_json_rules, ReplayableAnalyzer, JsonAnalyzer};
        let analyzer = JsonAnalyzer::new();
        let findings = analyzer.analyze(r#"{"enabledMcpjsonServers": [], "hookEvents": {}, "pre_tool_use": []}"#);
        let mut saw_001 = false;
        let mut saw_002 = false;
        for f in findings {
            match f.code.as_str() {
                "CCC-JSON-001" => {
                    saw_001 = true;
                    assert_eq!(deprecated_key_replacement(&f.message), Some("mcpServers"));
                }
                "CCC-JSON-002" => {
                    saw_002 = true;
                    assert!(hook_key_replacement(&f.message).is_some());
                }
                _ => {}
            }
        }
        assert!(saw_001 && saw_002, "fixture must exercise both rule codes");
        // settings_json_rules() itself must still define the exact prefixes
        // this module's constants assume — pins the two together so if the
        // rule wording changes, this test (not a silent quick-fix bug) fails.
        let rules = settings_json_rules();
        assert!(rules.iter().any(|r| r.code == "CCC-JSON-001" && r.message_prefix == DEPRECATED_KEY_PREFIX));
        assert!(rules.iter().any(|r| r.code == "CCC-JSON-002" && r.message_prefix == HOOK_KEY_PREFIX));
    }
}
