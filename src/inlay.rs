//! inlay — textDocument/inlayHint for settings.json.
//!
//! Pure functions over parsed JSON content: for hook config entries an inlay
//! after the matcher string shows the valid event-name enum; for `model` keys
//! an inlay after the value shows the accepted forms
//! (alias | full-id | account:-routed).

use lsp_types_max::{InlayHint, InlayHintLabel, Position, Range, Url};
use tree_sitter::Node;

/// Valid hook event names (mirrors analyzers/json.rs VALID_HOOK_EVENTS).
const HOOK_EVENTS: &[&str] = &[
    "PreToolUse",
    "PostToolUse",
    "PostToolUseFailure",
    "Stop",
    "StopFailure",
    "SubagentStart",
    "SubagentStop",
    "Notification",
    "SessionStart",
    "SessionEnd",
    "PreCompact",
    "PostCompact",
    "UserPromptSubmit",
    "PermissionRequest",
    "PermissionDenied",
    "ConfigChange",
    "CwdChanged",
    "FileChanged",
    "InstructionsLoaded",
    "MessageDisplay",
    "TeammateIdle",
    "TaskCompleted",
    "TaskCreated",
    "Setup",
    "Elicitation",
    "ElicitationResult",
];

/// Accepted forms for a settings.json `model` value.
const MODEL_FORMS: &str = "alias | full-id | account:-routed";

/// Parse `content` with the tree-sitter JSON grammar.
fn parse(content: &str) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .ok()?;
    parser.parse(content, None)
}

/// Byte offset of a (line, character) position (ASCII assumption: character
/// == byte column).
fn offset_of(content: &str, pos: Position) -> usize {
    let mut offset = 0usize;
    for (i, line) in content.split('\n').enumerate() {
        if (i as u32) < pos.line {
            offset += line.len() + 1;
        } else {
            return offset + pos.character as usize;
        }
    }
    offset + pos.character as usize
}

/// (line, character) of a byte offset.
fn pos_of(content: &str, offset: usize) -> Position {
    let mut line = 0u32;
    let mut line_start = 0usize;
    for (i, b) in content.bytes().enumerate() {
        if i >= offset {
            break;
        }
        if b == b'\n' {
            line += 1;
            line_start = i + 1;
        }
    }
    Position {
        line,
        character: (offset - line_start) as u32,
    }
}

/// Full text of a node.
fn node_text<'c>(node: Node<'_>, content: &'c str) -> &'c str {
    &content[node.start_byte()..node.end_byte()]
}

/// Unquoted text of a node if it is a string.
fn string_value<'c>(node: Node<'_>, content: &'c str) -> Option<&'c str> {
    let text = node_text(node, content);
    text.strip_prefix('"')?.strip_suffix('"')
}

/// Inlay hints for `content`, restricted to `range`.
///
/// - Hook config entries: after each hook entry's matcher string, an inlay
///   naming the valid event-name enum.
/// - `model` keys: after the value, an inlay naming the accepted forms.
pub fn inlay_hints(content: &str, range: Range, _uri: &Url) -> Vec<InlayHint> {
    let _ = _uri;
    let tree = match parse(content) {
        Some(t) => t,
        None => return Vec::new(),
    };
    let range_end = offset_of(content, range.end);
    let range_start = offset_of(content, range.start);

    let mut hints = Vec::new();
    collect_hints(
        tree.root_node(),
        content,
        range_start,
        range_end,
        &mut hints,
    );
    hints
}

/// Walk the tree, emitting hints for hook matcher entries and model keys.
fn collect_hints(
    node: Node<'_>,
    content: &str,
    range_start: usize,
    range_end: usize,
    out: &mut Vec<InlayHint>,
) {
    // Hook config entry: an object containing a "matcher" pair.
    if node.kind() == "object" && pair_value(node, content, "matcher").is_some() {
        emit_matcher_hint(node, content, range_start, range_end, out);
    }
    // Model key: a pair with key "model".
    if node.kind() == "pair" {
        if let Some(key) = node.child_by_field_name("key") {
            if string_value(key, content) == Some("model") {
                emit_model_hint(node, content, range_start, range_end, out);
            }
        }
    }
    // Recurse.
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();
    drop(cursor);
    for child in children {
        collect_hints(child, content, range_start, range_end, out);
    }
}

/// Inlay after the matcher string of a hook entry: the valid event enum.
fn emit_matcher_hint(
    entry: Node<'_>,
    content: &str,
    range_start: usize,
    range_end: usize,
    out: &mut Vec<InlayHint>,
) {
    let matcher = match pair_value(entry, content, "matcher") {
        Some(v) => v,
        None => return,
    };
    let end = matcher.end_byte();
    if end < range_start || end > range_end {
        return;
    }
    out.push(InlayHint {
        position: pos_of(content, end),
        label: InlayHintLabel::String(format!(" events: {}", HOOK_EVENTS.join("|"))),
        kind: None,
        text_edits: None,
        tooltip: None,
        padding_left: None,
        padding_right: None,
        data: None,
    });
}

/// Inlay after the value of a `model` pair: the accepted forms.
fn emit_model_hint(
    pair: Node<'_>,
    content: &str,
    range_start: usize,
    range_end: usize,
    out: &mut Vec<InlayHint>,
) {
    let value = match pair.child_by_field_name("value") {
        Some(v) => v,
        None => return,
    };
    let end = value.end_byte();
    if end < range_start || end > range_end {
        return;
    }
    out.push(InlayHint {
        position: pos_of(content, end),
        label: InlayHintLabel::String(format!(" forms: {MODEL_FORMS}")),
        kind: None,
        text_edits: None,
        tooltip: None,
        padding_left: None,
        padding_right: None,
        data: None,
    });
}

/// The value node of the pair with unquoted key `key` in object `node`
/// (first match).
fn pair_value<'t>(node: Node<'t>, content: &str, key: &str) -> Option<Node<'t>> {
    let mut cursor = node.walk();
    let found = node
        .children(&mut cursor)
        .filter(|c| c.kind() == "pair")
        .find(|c| {
            c.child_by_field_name("key")
                .map(|k| string_value(k, content) == Some(key))
                .unwrap_or(false)
        })
        .and_then(|c| c.child_by_field_name("value"));
    found
}
