//! Chicago-style tests for rename + inlay modules: real parsers, real
//! tree-sitter JSON, real content — no mocks. Assert on final state.

use claude_code_config_lsp::inlay::inlay_hints;
use claude_code_config_lsp::rename::{is_valid_key_name, prepare_rename, rename};
use lsp_types_max::{Position, Range};

const SETTINGS: &str = r#"{
  "model": "sonnet",
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [{"type": "command", "command": "echo hi"}]
      }
    ]
  }
}
"#;

fn label_text(h: &lsp_types_max::InlayHint) -> String {
    match &h.label {
        lsp_types_max::InlayHintLabel::String(s) => s.clone(),
        lsp_types_max::InlayHintLabel::LabelParts(parts) => {
            parts.iter().map(|p| p.value.clone()).collect()
        }
    }
}

fn uri() -> lsp_types_max::Url {
    "file:///proj/settings.json".parse().expect("static uri")
}

// ---- rename: prepareRename ----

#[test]
fn prepare_rename_returns_inner_key_range_on_key() {
    // Line 1: `  "model": "sonnet",` — key at columns 2..=8 (inner text).
    let r = prepare_rename(
        SETTINGS,
        Position {
            line: 1,
            character: 4,
        },
    )
    .expect("position is on a key");
    assert_eq!(r.start.line, 1);
    assert_eq!(
        r.start.character, 3,
        "inner range excludes the opening quote"
    );
    assert_eq!(r.end.character, 8, "inner range excludes the closing quote");
}

#[test]
fn prepare_rename_works_at_opening_quote_and_inner_text() {
    let at_quote = prepare_rename(
        SETTINGS,
        Position {
            line: 1,
            character: 2,
        },
    );
    let at_text = prepare_rename(
        SETTINGS,
        Position {
            line: 1,
            character: 4,
        },
    );
    assert_eq!(at_quote, at_text, "quote char and key text map to same key");
}

#[test]
fn prepare_rename_on_value_returns_none() {
    // The value string "sonnet" is not a key.
    let r = prepare_rename(
        SETTINGS,
        Position {
            line: 1,
            character: 13,
        },
    );
    assert_eq!(r, None, "value strings are not renameable keys");
}

#[test]
fn prepare_rename_outside_any_key_returns_none() {
    let r = prepare_rename(
        SETTINGS,
        Position {
            line: 0,
            character: 0,
        },
    );
    assert_eq!(r, None);
}

// ---- rename: validation ----

#[test]
fn valid_names_accept_lowercase_snake_and_dot_notation() {
    for name in [
        "effort_level",
        "a",
        "dot.notation.key",
        "x_1.y_2",
        "hook_z.end",
    ] {
        assert!(is_valid_key_name(name), "{name} must be valid");
    }
}

#[test]
fn invalid_names_rejected() {
    for name in [
        "",
        "Upper",
        "kebab-case",
        "double..dot",
        ".lead",
        "trail.",
        "9lead",
        "sp ace",
    ] {
        assert!(!is_valid_key_name(name), "{name} must be invalid");
    }
}

#[test]
fn rename_to_invalid_name_returns_none() {
    let e = rename(
        SETTINGS,
        Position {
            line: 1,
            character: 4,
        },
        "kebab-case",
        &uri(),
    );
    assert_eq!(e, None, "invalid name must refuse, client shows error");
}

// ---- rename: edit shape ----

#[test]
fn rename_produces_text_document_edit_on_inner_range() {
    let edit = rename(
        SETTINGS,
        Position {
            line: 1,
            character: 4,
        },
        "base_model",
        &uri(),
    )
    .expect("valid rename");
    let changes = match edit.document_changes {
        Some(lsp_types_max::DocumentChanges::Edits(edits)) => edits,
        other => panic!("expected TextDocumentEdit, got {other:?}"),
    };
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes[0].text_document.uri.as_str(),
        "file:///proj/settings.json"
    );
    assert_eq!(changes[0].edits.len(), 1);
    let lsp_types_max::OneOf3::A(te) = &changes[0].edits[0] else {
        panic!("expected a plain TextEdit");
    };
    assert_eq!(
        te.range.start,
        Position {
            line: 1,
            character: 3
        }
    );
    assert_eq!(
        te.range.end,
        Position {
            line: 1,
            character: 8
        }
    );
    assert_eq!(te.new_text, "base_model");
}

#[test]
fn rename_of_nested_key_targets_the_nested_key() {
    // Line 5: `        "matcher": "Bash"` — nested inside hooks.
    let edit = rename(
        SETTINGS,
        Position {
            line: 5,
            character: 10,
        },
        "match_string",
        &uri(),
    )
    .expect("nested key renameable");
    let lsp_types_max::DocumentChanges::Edits(edits) = edit.document_changes.unwrap() else {
        panic!("expected edits");
    };
    let lsp_types_max::OneOf3::A(te) = &edits[0].edits[0] else {
        panic!("expected TextEdit");
    };
    assert_eq!(te.range.start.line, 5);
    assert_eq!(te.new_text, "match_string");
}

// ---- inlay hints ----

#[test]
fn hook_matcher_gets_event_enum_inlay() {
    let full = Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 12,
            character: 0,
        },
    };
    let hints = inlay_hints(SETTINGS, full, &uri());
    let matcher_hint = hints
        .iter()
        .find(|h| label_text(h).contains("PreToolUse"))
        .expect("hook matcher entry must carry the event-name enum inlay");
    let text = match &matcher_hint.label {
        lsp_types_max::InlayHintLabel::String(s) => s.clone(),
        other => panic!("expected string label, got {other:?}"),
    };
    assert!(text.contains("PostToolUse"));
    assert!(text.contains("SessionStart"));
    assert_eq!(
        matcher_hint.position.line, 5,
        "inlay sits after the matcher string"
    );
}

#[test]
fn model_key_gets_accepted_forms_inlay() {
    let full = Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 12,
            character: 0,
        },
    };
    let hints = inlay_hints(SETTINGS, full, &uri());
    let h = hints
        .iter()
        .find(|h| label_text(h).contains("account"))
        .expect("model key must carry accepted-forms inlay");
    assert!(label_text(h).contains("alias"));
    assert_eq!(h.position.line, 1, "inlay sits after the model value");
}

#[test]
fn hints_outside_range_excluded() {
    // Range covering only lines 3-4 (event key + array open): no hint anchors there.
    let r = Range {
        start: Position {
            line: 3,
            character: 0,
        },
        end: Position {
            line: 4,
            character: 0,
        },
    };
    let hints = inlay_hints(SETTINGS, r, &uri());
    assert!(hints.is_empty(), "no hint anchors on lines 3-4: {hints:?}");
}

#[test]
fn invalid_json_yields_no_hints() {
    let full = Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 12,
            character: 0,
        },
    };
    let hints = inlay_hints("not json {", full, &uri());
    assert!(hints.is_empty());
}
