//! bcinr SIMD vs naive parity tests. Run under both:
//! `cargo test` (naive) and `cargo test --features bcinr` (SIMD).
//! The private `analyze_rule_naive` is the spec; this file re-implements it
//! as a reference and asserts the real analyzer matches it exactly.

use claude_code_config_lsp::analyzers::hook::{
    self, hook_event_rules, HookAnalyzer, ReplayableAnalyzer as HookReplay,
};
use claude_code_config_lsp::analyzers::toml::{
    self, toml_calver_rules, ReplayableAnalyzer as TomlReplay, TomlAnalyzer,
};

type Triple = (String, String, (usize, usize));

fn naive(input: &str, rules: &[(&'static str, Vec<String>, &'static str)]) -> Vec<Triple> {
    let mut out = Vec::new();
    for (code, patterns, prefix) in rules {
        for pattern in patterns {
            if pattern.is_empty() {
                continue;
            }
            let mut start = 0;
            while let Some(pos) = input[start..].find(pattern.as_str()) {
                let abs = start + pos;
                out.push((
                    (*code).to_string(),
                    format!("{}: {pattern}", prefix),
                    (abs, abs + pattern.len()),
                ));
                start = abs + pattern.len();
            }
        }
    }
    out.sort_by(|a, b| a.2 .0.cmp(&b.2 .0).then(a.0.cmp(&b.0)));
    out
}

fn triples(r: &[toml::Rule]) -> Vec<(&'static str, Vec<String>, &'static str)> {
    r.iter()
        .map(|x| (x.code, x.patterns.clone(), x.message_prefix))
        .collect()
}

fn triples_h(r: &[hook::Rule]) -> Vec<(&'static str, Vec<String>, &'static str)> {
    r.iter()
        .map(|x| (x.code, x.patterns.clone(), x.message_prefix))
        .collect()
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        ("valid_calver", "[package]\nversion = \"26.11.23\"\nedition = \"2021\"\n"),
        ("broken_calver", "[package]\nversion = \"1.2.3\"\nedition = \"2018\"\nversion = \"2.0.0\"\nedition = \"2015\"\n"),
        ("prose_hits", "comment version = \"0.1.0\" inside prose\nedition = \"2015\"\n"),
        ("no_hits", "plain prose, no rules match.\n"),
        ("adjacent", "version = \"1.x\" edition = \"2015\" version = \"3.9.9\"\n"),
        ("repeated", "version = \"1.0.0\"\nversion = \"1.0.0\"\nversion = \"6.1.1\"\n"),
    ]
}

fn hook_fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "misspelled",
            "\"PreTooluse\" \"posttoolusue\" \"PreToolUse\"\n",
        ),
        (
            "invalid_names",
            "on_tool_use BeforeToolUse AfterToolUse on_tool_result\n",
        ),
        ("no_hits", "#!/bin/sh\necho PreToolUse PostToolUse ok\n"),
        ("mixed", "x PreTooluse y on_tool_use z BeforeToolUse\n"),
    ]
}

fn check(input: &str, exp: Vec<Triple>, act: &[toml::RawFinding]) {
    assert_eq!(act.len(), exp.len(), "count on {input:?}");
    for (a, e) in act.iter().zip(exp.iter()) {
        assert_eq!(a.code, e.0, "code on {input:?}");
        assert_eq!(a.message, e.1, "message on {input:?}");
        assert_eq!(a.span, e.2, "span on {input:?}");
    }
}

fn check_h(input: &str, exp: Vec<Triple>, act: &[hook::RawFinding]) {
    assert_eq!(act.len(), exp.len(), "count on {input:?}");
    for (a, e) in act.iter().zip(exp.iter()) {
        assert_eq!(a.code, e.0, "code on {input:?}");
        assert_eq!(a.message, e.1, "message on {input:?}");
        assert_eq!(a.span, e.2, "span on {input:?}");
    }
}

#[test]
fn toml_parity() {
    for (name, input) in fixtures() {
        let rules = toml_calver_rules();
        let exp = naive(input, &triples(&rules));
        let analyzer = TomlAnalyzer::with_rules(name, rules.clone());
        check(input, exp, &analyzer.analyze(input));
    }
}

#[test]
fn toml_parity_default_analyzer() {
    let analyzer = TomlAnalyzer::new();
    for (_n, input) in fixtures() {
        let rules = toml_calver_rules();
        let exp = naive(input, &triples(&rules));
        check(input, exp, &analyzer.analyze(input));
    }
}

#[test]
fn hook_parity() {
    for (name, input) in hook_fixtures() {
        let rules = hook_event_rules();
        let exp = naive(input, &triples_h(&rules));
        let analyzer = HookAnalyzer::with_rules(name, rules.clone());
        check_h(input, exp, &analyzer.analyze(input));
    }
}

#[test]
fn parity_single_byte_pattern() {
    let rules = vec![toml::Rule::new(
        "SB-1",
        vec!["v".to_string()],
        "single byte",
    )];
    let input = "v".repeat(5000) + "version = \"1.0.0\"";
    let exp = naive(&input, &triples(&rules));
    let analyzer = TomlAnalyzer::with_rules("test", rules);
    check(&input, exp, &analyzer.analyze(&input));
}
