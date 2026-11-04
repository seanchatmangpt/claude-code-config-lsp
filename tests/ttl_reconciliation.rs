//! TTL reconciliation court (P10) — bidirectional text-level anti-drift fence.
//!
//! Precedent: ggen's refusal_crosswalk_court — the crosswalk between two
//! artifacts is enforced as a text-level court: every analyzer-side literal
//! must appear in the TTL, and every TTL-encoded individual literal must
//! appear in the analyzer source. Both directions. Drift in either file
//! fails the court.
//!
//! Chicago-style: no mocks, no fixtures — the collaborators are the REAL
//! files under fence, read from disk at test time.

use std::fs;
use std::path::Path;

const TTL_REL: &str = "schema/claude-code-config.ttl";
const JSON_RS: &str = "src/analyzers/json.rs";
const FRONTMATTER_RS: &str = "src/analyzers/frontmatter.rs";
const COMMANDS_RS: &str = "src/analyzers/commands.rs";
const FILE_TYPES_RS: &str = "src/file_types.rs";

fn read(rel: &str) -> String {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
    fs::read_to_string(&p).unwrap_or_else(|e| panic!("court cannot read {rel}: {e}"))
}

fn literals(src: &str) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    let mut rest = src;
    while let Some(i) = rest.find('"') {
        let after = &rest[i + 1..];
        match after.find('"') {
            Some(j) => {
                out.push(&after[..j]);
                rest = &after[j + 1..];
            }
            None => break,
        }
    }
    out
}

fn rust_str_array_block<'a>(src: &'a str, name: &str) -> &'a str {
    let marker = format!("{}: &[&str] = &[", name);
    let i = src
        .find(&marker)
        .unwrap_or_else(|| panic!("court cannot find const {name} in analyzer source"));
    let start = i + marker.len();
    let end = src[start..]
        .find("];")
        .unwrap_or_else(|| panic!("court cannot find end of const {name}"));
    &src[start..start + end]
}

fn src_end(src: &str, name: &str) -> usize {
    src.find("];")
        .unwrap_or_else(|| panic!("court cannot find end of const {name}"))
}

fn rust_str_array(src: &str, name: &str) -> Vec<String> {
    rust_str_array_block(src, name)
        .replace('"', "")
        .split_whitespace()
        .map(|t| t.trim_matches(',').to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

/// The P10 appended block: everything after the reconciliation marker.
/// Direction-B census counts scope to this block so pre-existing individuals
/// (owned by C9/C9-siblings) are not attributed to P10.
fn p10_block(ttl: &str) -> &str {
    const MARKER: &str =
        "# --- ontology reconciliation (P10): analyzers richer than TTL, encode the delta ---";
    match ttl.find(MARKER) {
        Some(i) => &ttl[i..],
        None => panic!("P10 reconciliation marker missing from {TTL_REL}"),
    }
}

fn ttl_prop_values(ttl: &str, prop: &str) -> Vec<String> {
    let marker = format!("ccc:{} \"", prop);
    let mut out: Vec<String> = Vec::new();
    let mut rest = ttl;
    while let Some(i) = rest.find(&marker) {
        let after = &rest[i + marker.len()..];
        match after.find('"') {
            Some(j) => {
                out.push(after[..j].to_string());
                rest = &after[j + 1..];
            }
            None => break,
        }
    }
    out
}

fn assert_all_in(haystack: &str, needles: &[String], what: &str, src_name: &str) {
    let missing: Vec<&String> = needles
        .iter()
        .filter(|n| !haystack.contains(n.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "P10 reconciliation drift ({what}): {missing:?} in {src_name} but missing from {ttl}",
        ttl = TTL_REL
    );
}

// ==== Direction A: analyzer source must be covered by the TTL ====

#[test]
fn every_hook_event_in_json_rs_appears_in_ttl() {
    let rust = read(JSON_RS);
    let ttl = read(TTL_REL);
    let events = rust_str_array(&rust, "VALID_HOOK_EVENTS");
    assert_eq!(
        events.len(),
        26,
        "json.rs VALID_HOOK_EVENTS changed size — update TTL individuals + this court"
    );
    assert_all_in(&ttl, &events, "hook events", "VALID_HOOK_EVENTS");
}

#[test]
fn every_json_enum_value_set_appears_in_ttl() {
    let rust = read(JSON_RS);
    let ttl = read(TTL_REL);
    let sets: [(&str, &str); 8] = [
        ("VALID_HOOK_TYPES", "hook types"),
        ("VALID_WORKTREE_BASE_REFS", "worktree baseRefs"),
        ("VALID_SKILL_OVERRIDES", "skill override modes"),
        ("VALID_PARENT_SETTINGS_BEHAVIOR", "parentSettingsBehavior"),
        ("VALID_MCP_SERVER_TYPES", "mcp server types"),
        ("VALID_MODELS", "models"),
        ("VALID_EFFORT_LEVELS", "effort levels"),
        ("VALID_PERMISSION_MODES", "permission modes"),
    ];
    for (name, what) in sets {
        let vals = rust_str_array(&rust, name);
        assert!(
            !vals.is_empty(),
            "{name} parsed empty — court parser rotted"
        );
        assert_all_in(&ttl, &vals, what, name);
    }
}

#[test]
fn every_deprecated_key_literal_appears_in_ttl() {
    let rust = read(JSON_RS);
    let ttl = read(TTL_REL);
    // Scan only the rule-declaration slices — the file-wide scan desyncs on
    // r#"..."# raw strings in test bodies (quote-pairing artifact).
    let anchors: [(&str, Option<&str>); 3] = [
        ("CCC-JSON-001", None),
        ("CCC-JSON-002", None),
        ("CCC-JSON-013", Some("// CCC-JSON-013:")),
    ];
    let mut needles: Vec<String> = Vec::new();
    for (code, anchor) in anchors {
        let start = match anchor {
            Some(a) => rust
                .find(a)
                .unwrap_or_else(|| panic!("court cannot find rule {code} in {JSON_RS}")),
            // Rule-declared codes: formatting-stable locator (backward scan
            // from the code literal to the enclosing Rule::new().
            None => {
                let slice = rule_slice(&rust, code);
                rust.find(slice).unwrap_or_else(|| {
                    panic!("court cannot locate rule slice for {code} in {JSON_RS}")
                })
            }
        };
        let slice = &rust[start..];
        let limit = slice.find("\n    }").unwrap_or(slice.len());
        let slice = &slice[..limit];
        for lit in literals(slice) {
            if !needles.iter().any(|n| n == lit) {
                needles.push(lit.to_string());
            }
        }
    }
    // Keep only plausible key literals (drop codes/messages found in the slices).
    let expected: [&str; 9] = [
        "enabledMcpjsonServers",
        "disabledMcpjsonServers",
        "mcpJsonServers",
        "hookEvents",
        "hook_events",
        "pre_tool_use",
        "post_tool_use",
        "themes",
        "monitors",
    ];
    for e in expected {
        assert!(
            needles.iter().any(|n| n == e),
            "deprecated key {e:?} no longer found in the rule slices of {JSON_RS} — update TTL + this court"
        );
    }
    assert_eq!(needles_census(&needles, &expected), 9);
    // Keep only identifier-like literals (drop rule message strings).
    let keys: Vec<String> = needles
        .into_iter()
        .filter(|n| {
            n.len() < 40
                && n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && n.chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_lowercase() || c == '_')
        })
        .collect();
    assert_all_in(&ttl, &keys, "deprecated keys", JSON_RS);
}

#[test]
fn every_languageid_in_file_types_rs_appears_in_ttl() {
    let rust = read(FILE_TYPES_RS);
    let ttl = read(TTL_REL);
    let block = {
        let marker = "SUPPORTED_FILE_TYPES: &[(&str, &str)] = &[";
        let i = rust.find(marker).unwrap_or_else(|| {
            panic!("court cannot find const SUPPORTED_FILE_TYPES in {FILE_TYPES_RS}")
        });
        let start = i + marker.len();
        let end = src_end(&rust[start..], "SUPPORTED_FILE_TYPES");
        &rust[start..start + end]
    };
    let lits = literals(block);
    let mut langids: Vec<String> = Vec::new();
    for pair in lits.chunks(2) {
        if pair.len() == 2 {
            langids.push(pair[1].to_string());
        }
    }
    assert!(
        langids.contains(&"claude-config-json".to_string()),
        "court parser rotted"
    );
    assert_all_in(&ttl, &langids, "languageIds", FILE_TYPES_RS);
}

#[test]
fn skill_frontmatter_fields_are_grounded_in_analyzers() {
    let ttl = read(TTL_REL);
    // Direction B-side grounding for the four fields: they must exist in the
    // TTL AND in the analyzer that validates them.
    let cmds = read(COMMANDS_RS);
    let fm = read(FRONTMATTER_RS);
    let fields = [
        "allowed-tools",
        "disable-model-invocation",
        "context",
        "effort",
    ];
    for f in fields {
        let in_ttl = ttl.contains(&format!("ccc:fieldName \"{f}\""));
        assert!(
            in_ttl,
            "skill frontmatter field {f} missing from TTL (P10 gap)"
        );
        let in_analyzer = cmds.contains(&format!("{f}:")) || fm.contains(&format!("{f}:"));
        assert!(
            in_analyzer,
            "TTL skill field {f} not grounded in commands.rs/frontmatter.rs"
        );
    }
    let fm = read(FRONTMATTER_RS);
    assert!(
        fm.contains("1024") && ttl.contains("CCC-AGENT-005"),
        "1024-char description limit / CCC-AGENT-005 constraint missing from TTL (P10 gap)"
    );
}

// ==== Direction B: TTL individuals must be grounded in analyzer source ====

#[test]
fn every_ttl_hook_event_appears_in_json_rs() {
    let rust = read(JSON_RS);
    let ttl = read(TTL_REL);
    let events = ttl_prop_values(p10_block(&ttl), "eventName");
    assert_eq!(
        events.len(),
        26,
        "TTL hook-event individual count changed — update this court"
    );
    assert_all_in(&rust, &events, "TTL hook events", "TTL ccc:eventName");
}

#[test]
fn every_ttl_enum_value_appears_in_analyzers() {
    let rust = read(JSON_RS);
    let fm = read(FRONTMATTER_RS);
    let ttl = read(TTL_REL);
    let mut needles: Vec<String> = Vec::new();
    for v in ttl_prop_values(p10_block(&ttl), "enumValues") {
        for tok in v.split_whitespace() {
            needles.push(tok.to_string());
        }
    }
    for v in ttl_prop_values(p10_block(&ttl), "fieldEnumValues") {
        for tok in v.split_whitespace() {
            needles.push(tok.to_string());
        }
    }
    assert!(
        !needles.is_empty(),
        "TTL enum literals parsed empty — court parser rotted"
    );
    // Values from frontmatter enums (fork, effort levels) are grounded in
    // frontmatter.rs; everything else must be in json.rs.
    for n in &needles {
        assert!(
            rust.contains(n.as_str()) || fm.contains(n.as_str()),
            "P10 reconciliation drift: TTL enum value {n:?} absent from both analyzers"
        );
    }
}

#[test]
fn every_ttl_deprecated_key_appears_in_json_rs() {
    let rust = read(JSON_RS);
    let ttl = read(TTL_REL);
    let keys = ttl_prop_values(p10_block(&ttl), "deprecatedKey");
    assert_eq!(
        keys.len(),
        9,
        "TTL deprecatedKey count changed — update this court"
    );
    assert_all_in(&rust, &keys, "TTL deprecated keys", "TTL ccc:deprecatedKey");
}

#[test]
fn every_ttl_replacement_key_appears_in_json_rs() {
    let rust = read(JSON_RS);
    let ttl = read(TTL_REL);
    let repls = ttl_prop_values(p10_block(&ttl), "replacementKey");
    assert!(
        !repls.is_empty(),
        "TTL replacementKey parsed empty — court parser rotted"
    );
    assert_all_in(
        &rust,
        &repls,
        "TTL replacement keys",
        "TTL ccc:replacementKey",
    );
}

#[test]
fn every_ttl_languageid_appears_in_file_types_rs() {
    let rust = read(FILE_TYPES_RS);
    let ttl = read(TTL_REL);
    let ids = ttl_prop_values(p10_block(&ttl), "languageIdValue");
    assert_eq!(
        ids.len(),
        4,
        "TTL languageId individual count changed — update this court"
    );
    assert_all_in(&rust, &ids, "TTL languageIds", "TTL ccc:languageIdValue");
}

#[test]
fn ttl_event_individual_count_matches_json_rs_enum() {
    let rust = read(JSON_RS);
    let ttl = read(TTL_REL);
    let in_rs = rust_str_array(&rust, "VALID_HOOK_EVENTS").len();
    let in_ttl = ttl_prop_values(&ttl, "eventName").len();
    assert_eq!(
        in_rs, in_ttl,
        "hook-event census: json.rs has {in_rs}, TTL has {in_ttl}"
    );
}

/// Locate the `Rule::new(...)` declaration slice for a diagnostic code,
/// formatting-stable: anchor on the code literal, then scan BACKWARD to the
/// enclosing `Rule::new(`. The previous anchor (`Rule::new("CODE"`) broke the
/// court the moment rustfmt split the call across lines.
fn rule_slice<'a>(src: &'a str, code: &str) -> &'a str {
    let code_lit = format!("\"{code}\"");
    let lit_pos = src.find(&code_lit).unwrap_or_else(|| {
        panic!("court cannot find code literal {code_lit} in analyzer source")
    });
    let open = src[..lit_pos]
        .rfind("Rule::new(")
        .unwrap_or_else(|| panic!("court cannot find Rule::new( enclosing {code}"));
    &src[open..]
}

#[test]
fn rule_slice_finds_single_line_rule() {
    // Single-line shape (the historical anchor).
    let src = r#"    vec![
        Rule::new("X-001", vec!["a".to_string()], "msg"),
    ]"#;
    let slice = rule_slice(src, "X-001");
    assert!(slice.starts_with("Rule::new("), "slice: {slice}");
    assert!(slice.contains("\"X-001\""));
}

#[test]
fn rule_slice_finds_rustfmt_multiline_rule() {
    // Multiline shape (what broke the court at json.rs:295-306).
    let src = "    Rule::new(\n        \"MULTI-001\",\n        vec![\n            \"k\".to_string(),\n        ],\n        \"msg\",\n    )";
    let slice = rule_slice(src, "MULTI-001");
    assert!(slice.starts_with("Rule::new("), "slice: {slice}");
    assert!(slice.contains("\"MULTI-001\""));
}

fn needles_census(needles: &[String], expected: &[&str; 9]) -> usize {
    expected
        .iter()
        .filter(|e| needles.iter().any(|n| n == *e))
        .count()
}
