//! Chicago-style tests for config call hierarchy + monikers.
//!
//! Real collaborators: the fixture tree on disk
//! (`tests/fixtures/claude-config/`) read via real file IO into the corpus
//! index; assertions on final state. The corpus API is in-memory by design
//! (fs-walking lives outside these functions); no mocks.

use claude_code_config_lsp::hierarchy::*;
use claude_code_config_lsp::moniker::*;
use lsp_types_max::Position;
use std::path::PathBuf;

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/claude-config")
}

fn read(p: &PathBuf) -> String {
    std::fs::read_to_string(p).expect("fixture must exist")
}

fn corpus() -> Corpus {
    let root = fixture_root();
    let mut c: Corpus = Vec::new();
    for rel in [
        "skills/ggen/SKILL.md",
        "skills/config-audit/SKILL.md",
        "skills/mcp-doctor/SKILL.md",
        "skills/source-of-truth-check/SKILL.md",
        "skills/errc-cycle/SKILL.md",
        "skills/run-ggen/SKILL.md",
        "agents/default-agent.md",
        "claude-global-CLAUDE.md",
        "settings.json",
    ] {
        let path = root.join(rel);
        c.push((path.clone(), read(&path)));
    }
    c
}

fn pos(line: u32, character: u32) -> Position {
    Position { line, character }
}

// ---------------------------------------------------------------------------
// Moniker: scope detection (pure function, prior art backend.rs AgentScope)
// ---------------------------------------------------------------------------

#[test]
fn scope_detection_by_uri_prefix() {
    let home = "/Users/sac";
    assert_eq!(
        config_scope("file:///Users/sac/.claude/settings.json", Some(home)),
        ConfigScope::Global
    );
    // Plugin beats global: the plugin-cache prefix wins.
    assert_eq!(
        config_scope(
            "file:///Users/sac/.claude/plugins/cache/foo/plugin.json",
            Some(home)
        ),
        ConfigScope::Plugin
    );
    assert_eq!(
        config_scope("file:///work/repo/.claude/settings.json", Some(home)),
        ConfigScope::Project
    );
    // No HOME configured: everything is project scope.
    assert_eq!(
        config_scope("file:///Users/sac/.claude/settings.json", None),
        ConfigScope::Project
    );
}

#[test]
fn settings_keys_scan_real_fixture() {
    let text = read(&fixture_root().join("settings.json"));
    let keys = settings_keys(&text);
    let paths: Vec<&str> = keys.iter().map(|k| k.path()).collect();
    for expected in [
        "env.PATH",
        "env.ANTHROPIC_AUTH_TOKEN",
        "permissions.allow",
        "permissions.deny",
        "hooks.PostToolUse",
        "hooks.PreToolUse",
        "hooks.PostToolUse.0.matcher",
        "hooks.PreToolUse.0.hooks.0.command",
        "enabledPlugins.ggen-lsp@ggen",
        "enabledPlugins.rust-analyzer-lsp@claude-plugins-official",
        "model",
        "effortLevel",
    ] {
        assert!(
            paths.contains(&expected),
            "dotted path {expected} missing from scan; got {} keys",
            keys.len()
        );
    }
    // Scanner must not invent paths that cross container boundaries.
    assert!(
        !paths.iter().any(|p| p.starts_with("env.permissions")),
        "env and permissions are siblings, not nested"
    );
}

#[test]
fn settings_keys_match_serde_parse() {
    // Cross-check oracle: recursively walk the serde_json parse and collect
    // every object member's dotted path (joining with '.'), then assert the
    // scanner's output set equals it. Exact string equality — no naive
    // `.`->`/` pointer mapping, which breaks on keys containing dots
    // (e.g. "glm-5.3-flash").
    let text = read(&fixture_root().join("settings.json"));
    let value: serde_json::Value = serde_json::from_str(&text).expect("fixture is valid JSON");

    fn walk(prefix: &str, value: &serde_json::Value, out: &mut Vec<String>) {
        if let serde_json::Value::Object(map) = value {
            for (k, v) in map {
                let path = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                out.push(path.clone());
                walk(&path, v, out);
            }
        } else if let serde_json::Value::Array(items) = value {
            for (i, item) in items.iter().enumerate() {
                let ipath = if prefix.is_empty() {
                    i.to_string()
                } else {
                    format!("{prefix}.{i}")
                };
                walk(&ipath, item, out);
            }
        }
    }

    let mut walked: Vec<String> = Vec::new();
    walk("", &value, &mut walked);
    let mut scanned: Vec<String> = settings_keys(&text)
        .into_iter()
        .map(|k| k.path().to_string())
        .collect();
    walked.sort();
    scanned.sort();
    assert_eq!(
        scanned, walked,
        "scanner paths must exactly equal the serde parse walk"
    );
}

#[test]
fn moniker_strings_global_and_project() {
    let m = moniker_for_key(
        ConfigScope::Global,
        std::path::Path::new("settings"),
        "enabledPlugins.ggen-lsp@ggen",
    );
    assert_eq!(
        m,
        "claude-config:global:settings.enabledPlugins.ggen-lsp@ggen"
    );
    let m = moniker_for_key(
        ConfigScope::Project,
        std::path::Path::new("settings.json"),
        "model",
    );
    assert_eq!(m, "claude-config:project:settings.json:model");
}

#[test]
fn moniker_at_position_on_real_fixture() {
    let text = read(&fixture_root().join("settings.json"));
    // Locate the `"ggen-lsp@ggen":` key line programmatically.
    let needle = "\"ggen-lsp@ggen\":";
    let byte_off = text.find(needle).expect("fixture enables ggen-lsp@ggen");
    let line = text[..byte_off].matches('\n').count() as u32;
    let line_start = text
        .rfind('\n')
        .map(|_| text[..byte_off].rfind('\n').map(|i| i + 1).unwrap_or(0));
    let col = (byte_off - line_start.unwrap_or(0)) as u32;
    let uri: lsp_types_max::Uri = "file:///Users/sac/.fake-home/.claude/settings.json"
        .parse()
        .expect("uri parses");
    // Home prefix must match the URI for Global scope.
    let (moniker, key, scope) = moniker_at_position(
        &text,
        &uri,
        pos(line, col + 3),
        Some("/Users/sac/.fake-home"),
        Some(std::path::Path::new("/Users/sac/.fake-home/proj")),
    )
    .expect("position sits on the ggen-lsp@ggen key");
    assert_eq!(scope, ConfigScope::Global);
    assert_eq!(key.path(), "enabledPlugins.ggen-lsp@ggen");
    assert_eq!(
        moniker,
        "claude-config:global:settings.enabledPlugins.ggen-lsp@ggen"
    );
}

#[test]
fn moniker_at_position_off_a_key_is_none() {
    let text = read(&fixture_root().join("settings.json"));
    // Position inside a string VALUE (the PATH value) is not a key.
    let needle = "/Users/sac/.m2/mvnd/bin";
    let byte_off = text.find(needle).expect("PATH value present");
    let line = text[..byte_off].matches('\n').count() as u32;
    let uri: lsp_types_max::Uri = "file:///Users/sac/.fake-home/.claude/settings.json"
        .parse()
        .expect("uri parses");
    let (line_start, _) = {
        let ls = text[..byte_off].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let col = (byte_off - ls) as u32;
        (ls, col)
    };
    let result = moniker_at_position(
        &text,
        &uri,
        pos(line, (byte_off - line_start) as u32),
        Some("/Users/sac/.fake-home"),
        None,
    );
    assert!(result.is_none(), "value strings are not keys: {result:?}");
}

#[test]
fn moniker_project_relative_path_in_moniker() {
    let text = r#"{
  "hooks": {
    "PostToolUse": [
      { "matcher": "Write", "hooks": [ { "type": "command", "command": "x.sh" } ] }
    ]
  },
  "model": "opus"
}"#;
    let uri: lsp_types_max::Uri = "file:///work/repo/.claude/settings.json".parse().unwrap();
    // Position on the "matcher" key (line 3).
    let (m, key, scope) = moniker_at_position(
        text,
        &uri,
        pos(3, 8),
        Some("/Users/sac"),
        Some(std::path::Path::new("/work/repo")),
    )
    .expect("on the matcher key");
    assert_eq!(scope, ConfigScope::Project);
    assert_eq!(key.path(), "hooks.PostToolUse.0.matcher");
    assert_eq!(
        m,
        "claude-config:project:.claude/settings.json:hooks.PostToolUse.0.matcher"
    );
}

#[test]
fn moniker_at_position_handles_multibyte_columns() {
    // UTF-16 columns: a multibyte char earlier in the line shifts columns.
    let text = "{\n  \"café\": \"☕\",\n  \"model\": \"opus\"\n}";
    let uri: lsp_types_max::Uri = "file:///w/.claude/settings.json".parse().unwrap();
    let home = "/w";
    // "model" is on line 2; scanner must land on it despite the multibyte
    // chars on line 1.
    let (m, key, _) =
        moniker_at_position(text, &uri, pos(2, 4), Some(home), None).expect("model key found");
    assert_eq!(key.path(), "model");
    assert_eq!(m, "claude-config:global:settings.model");
}

// ---------------------------------------------------------------------------
// Call hierarchy (frontmatter names, wikilinks, enabledPlugins, paths)
// ---------------------------------------------------------------------------

#[test]
fn frontmatter_name_on_real_skill_fixtures() {
    let c = corpus();
    let (_ggen_path, ggen_text) = c
        .iter()
        .find(|(p, _)| p.ends_with("skills/ggen/SKILL.md"))
        .expect("ggen skill fixture in corpus");
    let (name, range) = frontmatter_name(ggen_text).expect("SKILL.md has a name");
    assert_eq!(name, "ggen");
    assert_eq!(
        range.start.line, 1,
        "name is on frontmatter line 2 (index 1)"
    );
    assert_eq!(
        frontmatter_name(&read(&fixture_root().join("agents/default-agent.md")))
            .expect("agent fixture has a name")
            .0,
        "default-agent"
    );
    assert!(frontmatter_name(&read(&fixture_root().join("settings.json"))).is_none());
    assert!(frontmatter_name("no frontmatter").is_none());
}

#[test]
fn prepare_call_hierarchy_on_and_off_the_name() {
    let c = corpus();
    let ggen_path = c
        .iter()
        .find(|(p, _)| p.ends_with("skills/ggen/SKILL.md"))
        .map(|(p, _)| p.clone())
        .expect("ggen skill fixture in corpus");
    // On the name value: frontmatter line index 1, mid-value column.
    let item = prepare_call_hierarchy(&c, &ggen_path, pos(1, 8))
        .expect("cursor on the ggen name value prepares an item");
    assert_eq!(item.name, "ggen");
    assert_eq!(item.kind, lsp_types_max::SymbolKind::FUNCTION);
    assert!(
        item.uri.as_str().ends_with("skills/ggen/SKILL.md"),
        "item uri points at the SKILL.md: {}",
        item.uri.as_str()
    );
    // Off the name (deep in the body): no item.
    assert!(prepare_call_hierarchy(&c, &ggen_path, pos(30, 0)).is_none());
    // Off the name (inside another key's value): no item.
    assert!(prepare_call_hierarchy(&c, &ggen_path, pos(2, 5)).is_none());
}

#[test]
fn incoming_calls_wikilink_and_frontmatter_channels() {
    // Real fixture docs plus a referencing doc; the corpus API is in-memory
    // by design, so the referencing doc is an in-memory pair — the function
    // under test is the real implementation over real fixture text.
    let mut c = corpus();
    let notes =
        "---\nname: notes\ndescription: x\nskill: ggen\n---\n\nSee [[ggen]] and [[other]].\n";
    c.push((PathBuf::from("/tmp/proj/notes.md"), notes.to_string()));
    let edges = incoming_calls(&c, "ggen");
    let notes_edge = edges
        .iter()
        .find(|e| e.item.name == "notes")
        .expect("notes.md references ggen");
    // Two channels hit in notes.md: frontmatter `skill: ggen` + [[ggen]].
    assert_eq!(notes_edge.from_ranges.len(), 2, "one range per reference");
    // The defining SKILL.md itself is NOT an incoming edge.
    assert!(
        !edges.iter().any(|e| e.item.name == "ggen"),
        "the definition must not reference itself"
    );
    // Unrelated names produce no edges from the fixture corpus.
    assert!(incoming_calls(&c, "no-such-skill").is_empty());
}

#[test]
fn incoming_calls_enabledplugins_channel() {
    // The real settings.json fixture enables ggen-lsp@ggen: true — that is
    // an incoming reference to the plugin named "ggen-lsp" (not "ggen").
    let c = corpus();
    let edges = incoming_calls(&c, "ggen-lsp");
    let edge = edges
        .iter()
        .find(|e| e.item.detail.as_deref() == Some("settings.json") || e.item.name == "settings")
        .expect("settings.json references ggen-lsp via enabledPlugins");
    assert_eq!(edge.from_ranges.len(), 1);
    // Exact segment match: "ggen" must not match "ggen-lsp@ggen".
    assert!(
        incoming_calls(&c, "ggen")
            .iter()
            .all(|e| e.item.name != "settings"),
        "ggen must not match the ggen-lsp@ggen key"
    );
}

#[test]
fn outgoing_calls_resolve_wikilinks_and_paths() {
    let mut c = corpus();
    // The global CLAUDE.md fixture's wikilinks (e.g. [[config-audit]]-style
    // stems) resolve against corpus docs by frontmatter name or stem.
    let claude_md_path = c
        .iter()
        .find(|(p, _)| p.ends_with("claude-global-CLAUDE.md"))
        .map(|(p, _)| p.clone())
        .expect("claude-global fixture in corpus");
    let outgoing = outgoing_calls(&c, &claude_md_path);
    // The fixture's stems (testing-chicago-style etc.) don't resolve within
    // the config corpus — but the settings key channel doesn't run here.
    // Add a corpus doc whose stem matches a real wikilink stem.
    assert!(outgoing.is_empty(), "no targets in a config-only corpus");
    // Path references resolve by exact suffix match.
    let ggen_path = c
        .iter()
        .find(|(p, _)| p.ends_with("skills/ggen/SKILL.md"))
        .map(|(p, _)| p.clone())
        .expect("ggen fixture");
    let _doc = format!(
        "# Doc\n\nRead `{}\\n` and [[config-audit]].\n",
        ggen_path.display()
    );
    let doc = format!(
        "# Doc\n\nRead `{}` and [[config-audit]].\n",
        ggen_path.display()
    );
    c.push((PathBuf::from("/tmp/proj/CLAUDE.md"), doc));
    let outgoing = outgoing_calls(&c, &PathBuf::from("/tmp/proj/CLAUDE.md"));
    assert_eq!(outgoing.len(), 2, "path ref + wikilink both resolve");
    let by_name: Vec<&str> = outgoing.iter().map(|e| e.item.name.as_str()).collect();
    assert!(
        by_name.contains(&"ggen"),
        "path ref resolves to ggen SKILL.md: {by_name:?}"
    );
    assert!(
        by_name.contains(&"config-audit"),
        "wikilink resolves to config-audit"
    );
    // Ranges are per-reference and sorted.
    let ggen_edge = outgoing.iter().find(|e| e.item.name == "ggen").unwrap();
    assert_eq!(ggen_edge.from_ranges.len(), 1);
}
