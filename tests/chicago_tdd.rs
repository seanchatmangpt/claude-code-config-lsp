//! Chicago TDD Test Suite for claude-code-config-lsp analyzers
//!
//! This test module integrates chicago-tdd-tools to validate analyzer behavior
//! using AAA (Arrange-Act-Assert) pattern with test fixtures.
//!
//! Tests validate four public analyzer functions:
//! - analyzers::json::validate_settings_json_enums
//! - analyzers::json::validate_plugin_json
//! - analyzers::claude_md::validate_claude_md
//! - analyzers::frontmatter::validate_agent_frontmatter

use chicago_tdd_tools::prelude::*;
use claude_code_config_lsp::analyzers::{frontmatter, json, claude_md};

// ────────────────────────────────────────────────────────────────────────────
// Test Fixture for JSON Analyzers
// ────────────────────────────────────────────────────────────────────────────

struct SettingsJsonFixture {
    valid_basic: String,
    valid_with_model: String,
    valid_with_effort: String,
    valid_with_hooks: String,
    invalid_model_gpt: String,
    invalid_effort_extreme: String,
    invalid_permission_unsafe: String,
    invalid_hook_on_tool_use: String,
}

impl SettingsJsonFixture {
    fn new() -> Self {
        Self {
            valid_basic: r#"{"hooks": {}, "mcpServers": {}}"#.to_string(),
            valid_with_model: r#"{"model": "sonnet", "mcpServers": {}}"#.to_string(),
            valid_with_effort: r#"{"effortLevel": "max", "mcpServers": {}}"#.to_string(),
            valid_with_hooks: r#"{"hooks": {"PreToolUse": [], "PostToolUse": [], "SessionStart": []}}"#.to_string(),
            invalid_model_gpt: r#"{"model": "gpt-4"}"#.to_string(),
            invalid_effort_extreme: r#"{"effortLevel": "extreme"}"#.to_string(),
            invalid_permission_unsafe: r#"{"permissionMode": "unsafe"}"#.to_string(),
            invalid_hook_on_tool_use: r#"{"hooks": {"on_tool_use": []}}"#.to_string(),
        }
    }
}

struct PluginJsonFixture {
    valid_with_schema: String,
    invalid_missing_schema: String,
    valid_full_structure: String,
}

impl PluginJsonFixture {
    fn new() -> Self {
        Self {
            valid_with_schema: r#"{"$schema": "https://example.com/schema.json", "name": "x"}"#.to_string(),
            invalid_missing_schema: r#"{"name": "my-plugin"}"#.to_string(),
            valid_full_structure: r#"{"$schema": "https://example.com/plugin-schema.json", "name": "test-plugin", "version": "1.0.0"}"#.to_string(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test Fixture for CLAUDE.md Analyzer
// ────────────────────────────────────────────────────────────────────────────

struct ClaudeMdFixture {
    valid_with_h1: String,
    valid_with_content: String,
    invalid_no_h1: String,
    invalid_empty: String,
}

impl ClaudeMdFixture {
    fn new() -> Self {
        Self {
            valid_with_h1: "# My CLAUDE.md Title\n\nContent here".to_string(),
            valid_with_content: "# Configuration Guide\n\n## Overview\n\nThis is a test".to_string(),
            invalid_no_h1: "## Not a top-level heading\n\nContent here".to_string(),
            invalid_empty: "".to_string(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test Fixture for Agent Frontmatter Analyzer
// ────────────────────────────────────────────────────────────────────────────

struct AgentFrontmatterFixture {
    valid_basic: String,
    valid_with_tools: String,
    valid_with_model: String,
    invalid_missing_name: String,
    invalid_missing_description: String,
    invalid_model: String,
    invalid_tool: String,
}

impl AgentFrontmatterFixture {
    fn new() -> Self {
        Self {
            valid_basic: "---\nname: TestAgent\ndescription: A test agent\n---\n# Agent\n".to_string(),
            valid_with_tools: "---\nname: TestAgent\ndescription: A test agent\ntools: [Read, Write, Edit]\n---\n# Agent\n".to_string(),
            valid_with_model: "---\nname: TestAgent\ndescription: A test agent\nmodel: sonnet\n---\n# Agent\n".to_string(),
            invalid_missing_name: "---\ndescription: A test agent\n---\n# Agent\n".to_string(),
            invalid_missing_description: "---\nname: TestAgent\n---\n# Agent\n".to_string(),
            invalid_model: "---\nname: TestAgent\ndescription: A test agent\nmodel: gpt-4\n---\n# Agent\n".to_string(),
            invalid_tool: "---\nname: TestAgent\ndescription: A test agent\ntools: [InvalidTool]\n---\n# Agent\n".to_string(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// AAA Tests: JSON Analyzer - settings.json enum validation
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_validate_settings_json_enums_valid_basic() {
    // Arrange
    let fixture = SettingsJsonFixture::new();

    // Act
    let findings = json::validate_settings_json_enums(&fixture.valid_basic);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for valid basic settings.json");
}

#[test]
fn test_validate_settings_json_enums_valid_with_model_sonnet() {
    // Arrange
    let fixture = SettingsJsonFixture::new();

    // Act
    let findings = json::validate_settings_json_enums(&fixture.valid_with_model);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for valid model 'sonnet'");
}

#[test]
fn test_validate_settings_json_enums_valid_full_model_id() {
    // Arrange
    let input = r#"{"model": "claude-sonnet-4-6"}"#;

    // Act
    let findings = json::validate_settings_json_enums(input);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for full model ID");
}

#[test]
fn test_validate_settings_json_enums_valid_effort_level() {
    // Arrange
    let fixture = SettingsJsonFixture::new();

    // Act
    let findings = json::validate_settings_json_enums(&fixture.valid_with_effort);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for valid effort level 'max'");
}

#[test]
fn test_validate_settings_json_enums_valid_permission_mode_auto() {
    // Arrange
    let input = r#"{"permissionMode": "auto"}"#;

    // Act
    let findings = json::validate_settings_json_enums(input);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for valid permissionMode 'auto'");
}

#[test]
fn test_validate_settings_json_enums_valid_hook_events() {
    // Arrange
    let fixture = SettingsJsonFixture::new();

    // Act
    let findings = json::validate_settings_json_enums(&fixture.valid_with_hooks);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for valid hook events");
}

#[test]
fn test_validate_settings_json_enums_invalid_model_gpt4() {
    // Arrange
    let fixture = SettingsJsonFixture::new();

    // Act
    let findings = json::validate_settings_json_enums(&fixture.invalid_model_gpt);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for invalid model 'gpt-4'");
    assert!(
        findings.iter().any(|f| f.code == "CCC-JSON-004"),
        "Expected CCC-JSON-004 code for invalid model"
    );
}

#[test]
fn test_validate_settings_json_enums_invalid_effort_level() {
    // Arrange
    let fixture = SettingsJsonFixture::new();

    // Act
    let findings = json::validate_settings_json_enums(&fixture.invalid_effort_extreme);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for invalid effort level");
    assert!(
        findings.iter().any(|f| f.code == "CCC-JSON-005"),
        "Expected CCC-JSON-005 code for invalid effort level"
    );
}

#[test]
fn test_validate_settings_json_enums_invalid_permission_mode() {
    // Arrange
    let fixture = SettingsJsonFixture::new();

    // Act
    let findings = json::validate_settings_json_enums(&fixture.invalid_permission_unsafe);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for invalid permissionMode");
    assert!(
        findings.iter().any(|f| f.code == "CCC-JSON-006"),
        "Expected CCC-JSON-006 code for invalid permissionMode"
    );
}

#[test]
fn test_validate_settings_json_enums_invalid_hook_event() {
    // Arrange
    let fixture = SettingsJsonFixture::new();

    // Act
    let findings = json::validate_settings_json_enums(&fixture.invalid_hook_on_tool_use);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for invalid hook event");
    assert!(
        findings.iter().any(|f| f.code == "CCC-HOOK-001"),
        "Expected CCC-HOOK-001 code for invalid hook event"
    );
}

#[test]
fn test_validate_settings_json_enums_malformed_json() {
    // Arrange
    let input = "not valid json";

    // Act
    let findings = json::validate_settings_json_enums(input);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for malformed JSON (syntax is tree-sitter's domain)");
}

// ────────────────────────────────────────────────────────────────────────────
// AAA Tests: JSON Analyzer - plugin.json validation
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_validate_plugin_json_with_schema() {
    // Arrange
    let fixture = PluginJsonFixture::new();

    // Act
    let findings = json::validate_plugin_json(&fixture.valid_with_schema);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for plugin.json with $schema");
}

#[test]
fn test_validate_plugin_json_full_structure() {
    // Arrange
    let fixture = PluginJsonFixture::new();

    // Act
    let findings = json::validate_plugin_json(&fixture.valid_full_structure);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for complete plugin.json");
}

#[test]
fn test_validate_plugin_json_missing_schema() {
    // Arrange
    let fixture = PluginJsonFixture::new();

    // Act
    let findings = json::validate_plugin_json(&fixture.invalid_missing_schema);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for missing $schema");
    assert!(
        findings.iter().any(|f| f.code == "CCC-JSON-007"),
        "Expected CCC-JSON-007 code for missing $schema"
    );
}

#[test]
fn test_validate_plugin_json_malformed_json() {
    // Arrange
    let input = "{invalid json";

    // Act
    let findings = json::validate_plugin_json(input);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for malformed plugin.json");
}

// ────────────────────────────────────────────────────────────────────────────
// AAA Tests: CLAUDE.md Analyzer
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_validate_claude_md_valid_with_h1() {
    // Arrange
    let fixture = ClaudeMdFixture::new();

    // Act
    let findings = claude_md::validate_claude_md(&fixture.valid_with_h1);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for CLAUDE.md with H1 title");
}

#[test]
fn test_validate_claude_md_valid_with_content() {
    // Arrange
    let fixture = ClaudeMdFixture::new();

    // Act
    let findings = claude_md::validate_claude_md(&fixture.valid_with_content);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for CLAUDE.md with proper H1");
}

#[test]
fn test_validate_claude_md_invalid_no_h1() {
    // Arrange
    let fixture = ClaudeMdFixture::new();

    // Act
    let findings = claude_md::validate_claude_md(&fixture.invalid_no_h1);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for missing H1 title");
    assert!(
        findings.iter().any(|f| f.code == "CCC-MD-006"),
        "Expected CCC-MD-006 code for missing H1 title"
    );
}

#[test]
fn test_validate_claude_md_invalid_empty() {
    // Arrange
    let fixture = ClaudeMdFixture::new();

    // Act
    let findings = claude_md::validate_claude_md(&fixture.invalid_empty);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for empty CLAUDE.md");
    assert!(
        findings.iter().any(|f| f.code == "CCC-MD-006"),
        "Expected CCC-MD-006 code for empty content"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// AAA Tests: Agent Frontmatter Analyzer
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_validate_agent_frontmatter_valid_basic() {
    // Arrange
    let fixture = AgentFrontmatterFixture::new();

    // Act
    let findings = frontmatter::validate_agent_frontmatter(&fixture.valid_basic);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for valid agent frontmatter");
}

#[test]
fn test_validate_agent_frontmatter_valid_with_tools() {
    // Arrange
    let fixture = AgentFrontmatterFixture::new();

    // Act
    let findings = frontmatter::validate_agent_frontmatter(&fixture.valid_with_tools);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for agent with valid tools");
}

#[test]
fn test_validate_agent_frontmatter_valid_with_model() {
    // Arrange
    let fixture = AgentFrontmatterFixture::new();

    // Act
    let findings = frontmatter::validate_agent_frontmatter(&fixture.valid_with_model);

    // Assert
    assert!(findings.is_empty(), "Expected no findings for agent with valid model tier");
}

#[test]
fn test_validate_agent_frontmatter_invalid_missing_name() {
    // Arrange
    let fixture = AgentFrontmatterFixture::new();

    // Act
    let findings = frontmatter::validate_agent_frontmatter(&fixture.invalid_missing_name);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for missing name");
    assert!(
        findings.iter().any(|f| f.code == "CCC-AGENT-001"),
        "Expected CCC-AGENT-001 code for missing name"
    );
}

#[test]
fn test_validate_agent_frontmatter_invalid_missing_description() {
    // Arrange
    let fixture = AgentFrontmatterFixture::new();

    // Act
    let findings = frontmatter::validate_agent_frontmatter(&fixture.invalid_missing_description);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for missing description");
    assert!(
        findings.iter().any(|f| f.code == "CCC-AGENT-005"),
        "Expected CCC-AGENT-005 code for missing description"
    );
}

#[test]
fn test_validate_agent_frontmatter_invalid_model() {
    // Arrange
    let fixture = AgentFrontmatterFixture::new();

    // Act
    let findings = frontmatter::validate_agent_frontmatter(&fixture.invalid_model);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for invalid model");
    assert!(
        findings.iter().any(|f| f.code == "CCC-AGENT-002"),
        "Expected CCC-AGENT-002 code for invalid model tier"
    );
}

#[test]
fn test_validate_agent_frontmatter_invalid_tool() {
    // Arrange
    let fixture = AgentFrontmatterFixture::new();

    // Act
    let findings = frontmatter::validate_agent_frontmatter(&fixture.invalid_tool);

    // Assert
    assert!(!findings.is_empty(), "Expected findings for unknown tool");
    assert!(
        findings.iter().any(|f| f.code == "CCC-AGENT-006"),
        "Expected CCC-AGENT-006 code for unknown tool"
    );
}

#[test]
fn test_validate_agent_frontmatter_no_frontmatter() {
    // Arrange
    let input = "# Agent\n\nNo frontmatter here";

    // Act
    let findings = frontmatter::validate_agent_frontmatter(input);

    // Assert
    // When there's no frontmatter, the validator returns empty findings (no triple-dash delimiter)
    assert!(findings.is_empty(), "Expected no findings when frontmatter is completely absent");
}
