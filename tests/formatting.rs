//! Chicago-style tests for src/formatting.rs: real serde_json parses, real
//! string state asserted — no mocks exist in this surface.

use claude_code_config_lsp::formatting;
use lsp_types_max::{Position, Range, TextEdit};

fn range_over(text: &str) -> Range {
    Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: text.lines().count() as u32,
            character: 0,
        },
    }
}

fn settings_like() -> String {
    // Claude Code settings.local.json shape with deliberate key order the
    // author grouped by hand.
    let src = r#"{
  "permissions": {"deny": ["Bash(git *)"], "allow": ["Read"]},
  "hooks": {"PreToolUse": [{"matcher": "Bash"}]},
  "env": {"Z": "1", "A": "2"}
}"#;
    src.to_string()
}

#[test]
fn formats_valid_document_with_2space_indent() {
    let src = r#"{"a":1,"b":[2,3]}"#;
    let edits = formatting::format_document(src, range_over(src)).expect("valid json");
    let out = &edits[0].new_text;
    assert_eq!(out, "{\n  \"a\": 1,\n  \"b\": [\n    2,\n    3\n  ]\n}");
}

#[test]
fn key_order_preserved_as_written() {
    let src = settings_like();
    let edits = formatting::format_document(&src, range_over(&src)).expect("valid json");
    let out = &edits[0].new_text;
    let pos = |k: &str| out.find(k).unwrap_or_else(|| panic!("key {} present", k));
    assert!(pos("\"permissions\"") < pos("\"hooks\"") && pos("\"hooks\"") < pos("\"env\""));
    assert!(pos("\"deny\"") < pos("\"allow\""));
    assert!(pos("\"Z\"") < pos("\"A\""));
}

#[test]
fn invalid_json_is_never_reformatted() {
    let cases = [
        "{\"a\": 1,,}",
        "{\"a\": 1",
        "{ trailing_comma: 1 }",
        "{\"a\": 1} extra",
        "",
        "// jsonc comment\n{\"a\":1}",
    ];
    for src in cases {
        assert!(
            formatting::format_document(src, range_over(src)).is_none(),
            "invalid json must return None: {:?}",
            src
        );
    }
}

#[test]
fn idempotence_format_of_format_is_fixed_point() {
    let src = settings_like();
    let once = formatting::format_document(&src, range_over(&src)).unwrap()[0]
        .new_text
        .clone();
    let twice = formatting::format_document(&once, range_over(&once)).unwrap()[0]
        .new_text
        .clone();
    let thrice = formatting::format_document(&twice, range_over(&twice)).unwrap()[0]
        .new_text
        .clone();
    assert_eq!(once, twice);
    assert_eq!(twice, thrice);
}

#[test]
fn idempotence_over_fuzzed_valid_shapes() {
    //Chicago: real varied inputs, assert fixed point on each.
    let srcs = [
        r#"{}"#,
        r#"[]"#,
        r#"{"a":{}}"#,
        r#"{"a":[]}"#,
        r#"[[1,{"b":null}],false,3.5,"s"]"#,
        r#"{"x":1e10,"y":-0.5,"z":"tab\tquote\""}"#,
    ];
    for src in srcs {
        let once = formatting::format_document(src, range_over(src)).unwrap()[0]
            .new_text
            .clone();
        let twice = formatting::format_document(&once, range_over(&once)).unwrap()[0]
            .new_text
            .clone();
        assert_eq!(once, twice, "not idempotent for {:?}", src);
    }
}

#[test]
fn range_format_slices_affected_lines_only() {
    let src = settings_like();
    let edits = formatting::format_document(&src, range_over(&src)).unwrap();
    let formatted = edits[0].new_text.clone();
    let lines: Vec<&str> = formatted.lines().collect();
    let hook_idx = lines.iter().position(|l| l.contains("\"hooks\"")).unwrap();

    let range = Range {
        start: Position {
            line: hook_idx as u32,
            character: 0,
        },
        end: Position {
            line: (hook_idx + 1) as u32,
            character: 0,
        },
    };
    let opts = lsp_types_max::FormattingOptions::default();
    let slice = formatting::format_range(&src, range, &opts).expect("in-bounds range");
    let edit = &slice[0];
    assert_eq!(edit.range, range, "edit range matches requested range");
    assert_eq!(edit.new_text, lines[hook_idx..=hook_idx + 1].join("\n"));
    assert!(
        !edit.new_text.contains("\"permissions\""),
        "slice excludes out-of-range lines"
    );
}

#[test]
fn range_format_out_of_bounds_is_none() {
    let src = r#"{"a":1}"#;
    let opts = lsp_types_max::FormattingOptions::default();
    let range = Range {
        start: Position {
            line: 99,
            character: 0,
        },
        end: Position {
            line: 100,
            character: 0,
        },
    };
    assert!(formatting::format_range(src, range, &opts).is_none());
}

#[test]
fn range_format_invalid_json_is_none() {
    let src = "{\"a\":1,,}";
    let opts = lsp_types_max::FormattingOptions::default();
    let range = Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 1,
            character: 0,
        },
    };
    assert!(formatting::format_range(src, range, &opts).is_none());
}

#[test]
fn on_type_format_ignores_non_trigger_chars() {
    let src = "{\n  \"a\": 1\n}";
    for trigger in ['{', '1', '\n', 'x'] {
        assert!(
            formatting::on_type_format(
                src,
                Position {
                    line: 1,
                    character: 5
                },
                trigger
            )
            .is_none(),
            "trigger {:?} must not fire",
            trigger
        );
    }
}

#[test]
fn on_type_format_on_valid_json_with_correct_indent_is_noop() {
    let src = "{\n  \"a\": 1\n}";
    let pos = Position {
        line: 1,
        character: 8,
    };
    assert!(formatting::on_type_format(src, pos, ':').is_none());
}

#[test]
fn on_type_format_corrects_bad_indent_after_quote_or_colon() {
    // Line 1 over-indented: expects 2 spaces, has 4.
    let src = "{\n      \"a\": 1\n}";
    let pos = Position {
        line: 1,
        character: 10,
    };
    let edits = formatting::on_type_format(src, pos, '"').expect("indent correction fires");
    let edit = &edits[0];
    assert_eq!(edit.range.start.character, 0);
    assert_eq!(edit.range.end.character, 6);
    assert_eq!(edit.new_text, "  ");
    // Same correction fires on ':' trigger.
    let edits2 = formatting::on_type_format(src, pos, ':').expect("colon trigger fires");
    assert_eq!(edits2[0].new_text, "  ");
    // And under-indented line also fires.
    let under = "{\n\"a\": 1\n}";
    let edits3 = formatting::on_type_format(
        under,
        Position {
            line: 1,
            character: 3,
        },
        ':',
    )
    .expect("under-indent correction fires");
    assert_eq!(edits3[0].new_text, "  ");
}

#[test]
fn on_type_format_refuses_invalid_json_even_on_trigger() {
    let src = "{\n      \"a\": 1,,\n}";
    let pos = Position {
        line: 1,
        character: 10,
    };
    assert!(formatting::on_type_format(src, pos, '"').is_none());
}

#[test]
fn nested_depth_indent_is_correct_in_document_output() {
    let src = r#"{"hooks":{"PreToolUse":[{"matcher":"Bash"}]}}"#;
    let out = &formatting::format_document(src, range_over(src)).unwrap()[0].new_text;
    let expected = concat!(
        "{\n",
        "  \"hooks\": {\n",
        "    \"PreToolUse\": [\n",
        "      {\n",
        "        \"matcher\": \"Bash\"\n",
        "      }\n",
        "    ]\n",
        "  }\n",
        "}"
    );
    assert_eq!(out, expected);
}

#[test]
fn empty_object_and_array_stay_inline() {
    let src = r#"{"a":{},"b":[]}"#;
    let out = &formatting::format_document(src, range_over(src)).unwrap()[0].new_text;
    assert_eq!(out, "{\n  \"a\": {},\n  \"b\": []\n}");
}

#[test]
fn text_edit_type_is_public_surface_compatible() {
    // Guards the signature drift of the seam consumed by backend wiring.
    fn assert_edit(_: TextEdit) {}
    let src = "{}";
    let edits = formatting::format_document(src, range_over(src)).unwrap();
    assert_edit(edits[0].clone());
}
