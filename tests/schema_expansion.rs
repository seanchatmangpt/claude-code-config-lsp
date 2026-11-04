//! Schema expansion court — C9 settings.json key inventory expansion.
//! Evidence ground truth: /Users/sac/.claude/settings.json + settings.local.json
//! (2026-11-03). Asserts src/schema.rs carries the real-corpus grammar.

use std::fs;

const SCHEMA_RS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/schema.rs");
const TTL: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/schema/claude-code-config.ttl");

fn schema_rs() -> String {
    fs::read_to_string(SCHEMA_RS).expect("src/schema.rs readable")
}

/// Whitespace-free view of generated source: rustfmt may lay out a logical
/// `settings.fields.push(("key".to_string(), ...))` row on one line or across
/// several; removing all whitespace makes source-greps layout-insensitive.
/// Enum members (`enum: normal | vim`) contain no significant whitespace, so
/// contains-checks on this view are exact.
fn normalized(src: &str) -> String {
    src.split_whitespace().collect::<String>()
}

fn ttl() -> String {
    fs::read_to_string(TTL).expect("schema/claude-code-config.ttl readable")
}

#[test]
fn schema_rs_carries_every_observed_top_level_key() {
    let src = normalized(&schema_rs());
    for key in [
        "env",
        "attribution",
        "permissions.allow",
        "permissions.deny",
        "permissions.defaultMode",
        "enabledMcpjsonServers",
        "disabledMcpjsonServers",
        "hooks.matcher",
        "hooks.hooks",
        "hooks.type",
        "hooks.timeout",
        "hooks.statusMessage",
        "statusLine.type",
        "statusLine.command",
        "extraKnownMarketplaces",
        "modelSettings",
        "askUserQuestionTimeout",
        "voice",
        "voiceEnabled",
        "skipDangerousModePermissionPrompt",
        "skipWorkflowUsageWarning",
        "skipAutoPermissionPrompt",
        "agentPushNotifEnabled",
        "editorMode",
        "verbose",
        "crossSessionInbound",
    ] {
        assert!(
            src.contains(&format!("settings.fields.push((\"{}\"", key)),
            "src/schema.rs missing settings key: {}",
            key
        );
    }
}

#[test]
fn schema_rs_carries_enum_facts_for_enum_keys() {
    let src = normalized(&schema_rs());
    for (key, enum_member) in [
        ("permissions.defaultMode", "bypassPermissions"),
        ("hooks.type", "command"),
        ("statusLine.type", "command"),
        ("editorMode", "vim"),
        ("tui", "fullscreen"),
        ("crossSessionInbound", "accept"),
    ] {
        // Locate the field row for the key, then bound the enum-member check
        // to that row's window (next ~400 whitespace-free chars) so a member
        // belonging to a *different* field can't satisfy this assertion.
        let needle = format!("settings.fields.push((\"{}\"", key);
        let idx = src
            .find(&needle)
            .unwrap_or_else(|| panic!("no schema.rs field row for {}", key));
        let window = &src[idx..(idx + 400).min(src.len())];
        assert!(
            window.contains(enum_member),
            "enum fact missing: {} should list {}",
            key,
            enum_member
        );
    }
}

#[test]
fn ttl_carries_schemafield_individuals_for_expanded_keys() {
    let ttl = ttl();
    for ind in [
        "ccc:settings_env",
        "ccc:settings_attribution",
        "ccc:settings_permissions_allow",
        "ccc:settings_permissions_deny",
        "ccc:settings_permissions_defaultMode",
        "ccc:settings_enabledMcpjsonServers",
        "ccc:settings_disabledMcpjsonServers",
        "ccc:settings_hooks_matcher",
        "ccc:settings_hooks_hooks",
        "ccc:settings_hooks_type",
        "ccc:settings_hooks_timeout",
        "ccc:settings_hooks_statusMessage",
        "ccc:settings_statusLine",
        "ccc:settings_statusLine_type",
        "ccc:settings_statusLine_command",
        "ccc:settings_enabledPlugins",
        "ccc:settings_extraKnownMarketplaces",
        "ccc:settings_modelSettings",
        "ccc:settings_mcpServers",
        "ccc:settings_askUserQuestionTimeout",
        "ccc:settings_tui",
        "ccc:settings_voice",
        "ccc:settings_voiceEnabled",
        "ccc:settings_skipDangerousModePermissionPrompt",
        "ccc:settings_skipWorkflowUsageWarning",
        "ccc:settings_skipAutoPermissionPrompt",
        "ccc:settings_agentPushNotifEnabled",
        "ccc:settings_editorMode",
        "ccc:settings_verbose",
        "ccc:settings_crossSessionInbound",
    ] {
        assert!(ttl.contains(ind), "ttl missing individual {}", ind);
    }
}

#[test]
fn ttl_marks_hand_extension_and_evidence_source() {
    let ttl = ttl();
    assert!(
        ttl.contains("hand-extended 2026-11-03, pending FM-GEN-007"),
        "hand-extension marker missing from ttl"
    );
    assert!(
        ttl.contains("/Users/sac/.claude/settings.json"),
        "evidence source path missing from ttl"
    );
}

#[test]
fn schema_rs_marks_hand_extension() {
    let src = schema_rs();
    assert!(
        src.contains("hand-extended 2026-11-03, pending FM-GEN-007"),
        "hand-extension marker missing from src/schema.rs"
    );
}

#[test]
fn registry_exposes_expanded_fields_at_runtime() {
    let schema = claude_code_config_lsp::schema::build_schema();
    let settings = schema
        .types
        .get("Claude Code settings.json")
        .expect("settings node exists");
    for key in [
        "env",
        "attribution",
        "permissions.defaultMode",
        "extraKnownMarketplaces",
        "modelSettings",
        "askUserQuestionTimeout",
        "crossSessionInbound",
    ] {
        assert!(
            settings.fields.iter().any(|(n, _)| n == key),
            "registry missing field {}",
            key
        );
    }
    let default_mode = settings
        .fields
        .iter()
        .find(|(n, _)| n == "permissions.defaultMode")
        .unwrap();
    assert!(
        default_mode.1.name.contains("bypassPermissions"),
        "defaultMode enum lost bypassPermissions"
    );
}
