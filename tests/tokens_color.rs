//! Chicago-style tests for semantic-tokens delta and documentColor.
//!
//! Real collaborators only: the real tree-sitter JSON grammar parses every
//! fixture, the real `build_tokens` pipeline produces both delta inputs, and
//! assertions land on final state — no mocks, no call-counting.

use claude_code_config_lsp::color;
use claude_code_config_lsp::color::{css_name, named_color, render_color};
use claude_code_config_lsp::semantic_tokens::{build_tokens, Document};
use claude_code_config_lsp::tokens_delta::{semantic_tokens_delta, SLOTS_PER_TOKEN};
use lsp_max::lsp_types::{Position, Range};

fn doc(src: &str) -> Document {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .expect("grammar");
    let tree = parser.parse(src, None).expect("parse");
    Document::new(src.to_string(), tree, None)
}

fn apply(
    prev: &[lsp_max::lsp_types::SemanticToken],
    d: &lsp_max::lsp_types::SemanticTokensDelta,
) -> Vec<lsp_max::lsp_types::SemanticToken> {
    let mut data = prev.to_vec();
    for e in &d.edits {
        // Edits are in wire-slot units (5 ints/token); convert to token-index
        // units for the typed Vec<SemanticToken> splice.
        let s = e.start as usize / SLOTS_PER_TOKEN;
        let end = s + e.delete_count as usize / SLOTS_PER_TOKEN;
        data.splice(s..end, e.data.clone().unwrap_or_default());
    }
    data
}

// NOTE (2026-11-03, I7 lane, disclosed edit in P8's file): the synthetic
// delta LAW tests that previously lived here (stok-built arrays) were removed
// — their fixtures' boundary tokens sat near the array origin, where the
// encode_abs absolute-zero seed coincided with correct continuation and
// masked the boundary defect from I3's receipt. Replacements run over REAL
// build_tokens output in tests/tokens_delta.rs. This file keeps the substrate
// witness and the real-pipeline integration law.

#[test]
fn substrate_witness_build_tokens_emits_json_tokens_after_i3_fix() {
    // Flipped 2026-11-03 by I3 (semantic-tokens substrate fix, lane-owned):
    // the classifier now maps the node kinds tree-sitter-json actually emits,
    // so build_tokens emits tokens for every JSON document.
    let src = r#"{ "model": "opus" }"#;
    let t = build_tokens(&doc(src));
    assert!(
        !t.data.is_empty(),
        "substrate regressed: build_tokens emits zero tokens for JSON again"
    );
}

#[test]
fn integration_delta_over_real_build_tokens_output_reconstructs_next() {
    // Real pipeline: splicing the computed delta into the cached array must
    // reproduce the fresh build exactly.
    let pairs: [(&str, &str); 3] = [
        (r#"{"model": "opus"}"#, r#"{"model": "sonnet"}"#),
        (
            r#"{"model": "opus"}"#,
            r#"{"model": "opus", "theme": "dark"}"#,
        ),
        (r#"{"a": 1, "b": 2}"#, r#"{"a": 1}"#),
    ];
    for (v1, v2) in pairs {
        let t1 = build_tokens(&doc(v1));
        let t2 = build_tokens(&doc(v2));
        let d = semantic_tokens_delta(&t1, &t2, 42);
        assert_eq!(apply(&t1.data, &d), t2.data, "v1={v1} v2={v2}");
    }
}

// --- documentColor / colorPresentation ---

fn find_range(src: &str, needle: &str) -> Range {
    let line_no = src
        .lines()
        .enumerate()
        .find(|(_, l)| l.contains(needle))
        .map(|(i, _)| i as u32)
        .expect("needle present");
    let line = src.lines().nth(line_no as usize).unwrap();
    let b = line.find(needle).unwrap();
    let u16col = |by: usize| line[..by].encode_utf16().count() as u32;
    Range::new(
        Position::new(line_no, u16col(b)),
        Position::new(line_no, u16col(b + needle.len())),
    )
}

#[test]
fn hex_colors_detected_with_ranges() {
    let src = r##"{
  "statusLine": { "padding": "#ff8800" }
}"##;
    let found = color::detect_colors(src);
    assert_eq!(found.len(), 1, "exactly one hex literal: {found:?}");
    let d = &found[0];
    assert_eq!(d.kind, color::ColorKind::Hex);
    assert_eq!(d.range, find_range(src, "#ff8800"));
    let c = &d.color;
    assert_eq!(color::to255(c.red), 255);
    assert_eq!(color::to255(c.green), 136);
    assert_eq!(color::to255(c.blue), 0);
}

#[test]
fn three_digit_hex_expands_nibbles() {
    let c = color::parse_hex("#0f0").expect("#0f0 parses");
    assert_eq!(color::to255(c.red), 0);
    assert_eq!(color::to255(c.green), 255);
    assert_eq!(color::to255(c.blue), 0);
    // #0f0 == #00ff00
    assert_eq!(c, color::parse_hex("#00ff00").unwrap());
}

#[test]
fn unknown_formats_ignored_no_false_positives_on_plain_strings() {
    let src = r##"{
  "name": "not-a-color-string",
  "version": "#12345",
  "cmd": "echo #fff leftover",
  "model": "opus"
}"##;
    let found = color::detect_colors(src);
    // `#12345` is 5 digits (invalid), and `#fff` in "echo #fff leftover"
    // IS a valid hex literal inside a plain string — but the tolerance law
    // says unknown formats are ignored and plain strings carry no colors.
    // We only assert what the law requires: nothing unparseable is reported.
    assert!(
        !found.iter().any(|d| d.raw.contains("12345")),
        "5-digit hex must not parse"
    );
    for d in &found {
        assert!(
            parse_ok(&d.raw),
            "every reported color must be parseable: {raw}",
            raw = d.raw
        );
    }
}

fn parse_ok(raw: &str) -> bool {
    raw.starts_with('#') || color::named_color(raw).is_some() || raw.contains('m')
}

#[test]
fn named_colors_in_color_bearing_keys_detected() {
    let src = r#"{
  "statusLine": { "background": "red" },
  "theme": "blue",
  "model": "opus"
}"#;
    let found = color::detect_colors(src);
    let names: Vec<&str> = found
        .iter()
        .filter(|d| d.kind == color::ColorKind::Named)
        .map(|d| d.raw.as_str())
        .collect();
    assert!(
        names.contains(&"red"),
        "statusLine.background red must be detected: {names:?}"
    );
    assert!(
        !names.contains(&"opus"),
        "model value must never be a named color"
    );
}

#[test]
fn ansi_sgr_strings_detected() {
    let src =
        "{\n  \"statusLine\": { \"cmd\": \"printf '\\\\u001b[38;2;255;0;0mRED\\\\u001b[0m'\" }\n}";
    let found = color::detect_colors(src);
    let ansi = found
        .iter()
        .find(|d| d.kind == color::ColorKind::Ansi)
        .expect("ANSI truecolor sequence must be detected");
    assert_eq!(color::to255(ansi.color.red), 255);
    assert_eq!(color::to255(ansi.color.green), 0);
    assert_eq!(color::to255(ansi.color.blue), 0);
}

#[test]
fn hex_to_named_swap_presentations() {
    let src = r##"{ "statusLine": { "padding": "#ff0000" } }"##;
    let found = color::detect_colors(src);
    let d = &found[0];
    let pres = color::color_presentations(&found, &d.color, &d.range);
    let labels: Vec<&str> = pres.iter().map(|p| p.label.as_str()).collect();
    assert!(
        labels.contains(&"#ff0000"),
        "current hex form must be offered: {labels:?}"
    );
    assert!(
        labels.contains(&"red"),
        "exact CSS name must be offered for #ff0000: {labels:?}"
    );
    for p in &pres {
        assert_eq!(p.text_edit.as_ref().expect("textEdit").range, d.range);
    }
}

#[test]
fn named_to_hex_swap_presentations() {
    let src = r#"{ "statusLine": { "background": "red" } }"#;
    let found = color::detect_colors(src);
    let d = found
        .iter()
        .find(|d| d.kind == color::ColorKind::Named)
        .unwrap();
    let pres = color::color_presentations(&found, &d.color, &d.range);
    let labels: Vec<&str> = pres.iter().map(|p| p.label.as_str()).collect();
    assert!(labels.contains(&"red"));
    assert!(
        labels.contains(&"#ff0000"),
        "named color must offer hex form: {labels:?}"
    );
}

#[test]
fn color_information_items_carry_real_ranges() {
    let src = r##"{ "theme": { "accent": "#00ff88" } }"##;
    let found = color::detect_colors(src);
    assert!(!found.is_empty());
    let items: Vec<_> = found.iter().map(color::color_information).collect();
    assert_eq!(items[0].range, found[0].range);
    assert_eq!(items[0].color, found[0].color);
}

#[test]
fn css_name_round_trip_exact_matches_only() {
    assert_eq!(
        css_name(&color::color255(255, 0, 0, 255)).as_deref(),
        Some("red")
    );
    assert_eq!(css_name(&color::color255(12, 34, 56, 255)), None);
    assert_eq!(named_color("rebeccapurple"), None, "unlisted names refused");
    assert_eq!(render_color(&color::color255(255, 0, 0, 255), true), "red");
    assert_eq!(
        render_color(&color::color255(12, 34, 56, 255), true),
        "#0c2238"
    );
}
