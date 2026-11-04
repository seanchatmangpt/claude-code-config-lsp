//! Handler: textDocument/signatureHelp
//!
//! For hook objects inside settings.json: a position inside a hook entry
//! resolves to the hook-object grammar signature
//! `matcher: string, hooks: [{type, command, timeout?}]`, with
//! `activeParameter` derived from which key the cursor is inside, resolved
//! via the tree-sitter JSON parse (same import pattern as completion.rs).

use lsp_types_max::{ParameterInformation, ParameterLabel, SignatureHelp, SignatureInformation};
use tree_sitter::Node;

use crate::completion::position_to_offset;

/// Parameter names of the matcher-object (hook entry) grammar, in order.
const HOOK_OBJECT_PARAMS: [&str; 2] = ["matcher", "hooks"];
/// Parameter names of a command entry inside `hooks: [...]`, in order.
const COMMAND_OBJECT_PARAMS: [&str; 3] = ["type", "command", "timeout"];

/// Grammar label for the hook-object signature (the contract's wire form).
pub const HOOK_SIGNATURE_LABEL: &str =
    "matcher: string, hooks: [{type: \"command\", command: string, timeout?: number}]";

/// Text of a `pair` node's key, unquoted. Local copy of the same traversal
/// completion.rs uses (its `key_text` is private).
fn key_text(pair: Node<'_>, content: &str) -> Option<String> {
    let key_node = pair.child_by_field_name("key")?;
    if let Some(sc) = key_node.named_child(0) {
        if sc.kind() == "string_content" {
            return Some(content[sc.start_byte()..sc.end_byte()].to_string());
        }
    }
    let raw = &content[key_node.start_byte()..key_node.end_byte()];
    Some(raw.trim_matches('"').to_string())
}

/// Nearest `pair` ancestor of `node`.
fn enclosing_pair(node: Node<'_>) -> Option<Node<'_>> {
    let mut cur = Some(node);
    while let Some(n) = cur {
        if n.kind() == "pair" {
            return Some(n);
        }
        cur = n.parent();
    }
    None
}

/// Nearest `object` ancestor at-or-above `node`.
fn enclosing_object(node: Node<'_>) -> Option<Node<'_>> {
    let mut cur = Some(node);
    while let Some(n) = cur {
        if n.kind() == "object" {
            return Some(n);
        }
        cur = n.parent();
    }
    None
}

/// The key of the pair whose value is `array`'s parent pair, if `array` is
/// the value of a pair.
fn array_pair_key(array: Node<'_>, content: &str) -> Option<String> {
    let pair = array.parent().filter(|p| p.kind() == "pair")?;
    key_text(pair, content)
}

/// True when `obj` is a command entry: an element of the array that is the
/// value of a pair keyed `hooks`.
fn is_command_object(obj: Node<'_>, content: &str) -> Option<bool> {
    let array = obj.parent().filter(|a| a.kind() == "array")?;
    Some(array_pair_key(array, content).as_deref() == Some("hooks"))
}

/// True when `obj` is a hook (matcher) entry: an element of the array that
/// is the value of a pair keyed with a hook event name, where that pair's
/// owning object is itself the value of the top-level `hooks` pair.
fn is_hook_object(obj: Node<'_>, content: &str) -> Option<bool> {
    let array = obj.parent().filter(|a| a.kind() == "array")?;
    let event_pair = array.parent().filter(|p| p.kind() == "pair")?;
    let key = key_text(event_pair, content)?;
    if key == "hooks" {
        // The array under the `hooks` key holds command objects, not
        // matcher objects.
        return Some(false);
    }
    // The event entry object must sit under the `hooks` key, whose value
    // may be an object (`"hooks": {"Event": [...]}`) or — defensively —
    // an array of event entries.
    let entry_obj = event_pair.parent()?;
    let under_hooks = match entry_obj.parent().filter(|p| p.kind() == "pair") {
        Some(hooks_pair) => key_text(hooks_pair, content).as_deref() == Some("hooks"),
        None => {
            let entry_array = entry_obj.parent().filter(|a| a.kind() == "array")?;
            array_pair_key(entry_array, content).as_deref() == Some("hooks")
        }
    };
    if !under_hooks {
        return Some(false);
    }
    // A matcher object must actually carry hook-entry keys.
    let mut cursor = obj.walk();
    let has_entry_keys = obj.children(&mut cursor).any(|c| {
        c.kind() == "pair"
            && matches!(
                key_text(c, content).as_deref(),
                Some("matcher") | Some("hooks")
            )
    });
    Some(has_entry_keys)
}

/// Which named key of `params` the cursor's enclosing pair carries.
fn active_param_for(node: Node<'_>, content: &str, params: &[&str]) -> Option<usize> {
    let pair = enclosing_pair(node)?;
    let key = key_text(pair, content)?;
    Some(params.iter().position(|p| *p == key).unwrap_or(0))
}

/// Resolve the signature help at a byte offset. Returns `None` when the
/// position is not inside a hook entry (matcher object or its command
/// objects).
pub fn signature_help_at_offset(
    root: Node<'_>,
    content: &str,
    offset: usize,
) -> Option<SignatureHelp> {
    let offset = offset.min(content.len());
    let node = root.descendant_for_byte_range(offset, offset)?;

    let obj = enclosing_object(node)?;

    let (params, active) = if is_command_object(obj, content) == Some(true) {
        (
            &COMMAND_OBJECT_PARAMS[..],
            active_param_for(node, content, &COMMAND_OBJECT_PARAMS).unwrap_or(0),
        )
    } else if is_hook_object(obj, content) == Some(true) {
        (
            &HOOK_OBJECT_PARAMS[..],
            active_param_for(node, content, &HOOK_OBJECT_PARAMS).unwrap_or(0),
        )
    } else {
        return None;
    };

    let signatures = vec![SignatureInformation {
        label: HOOK_SIGNATURE_LABEL.to_string(),
        documentation: None,
        parameters: Some(
            params
                .iter()
                .map(|p| ParameterInformation {
                    label: ParameterLabel::Simple((*p).to_string()),
                    documentation: None,
                })
                .collect(),
        ),
        active_parameter: None,
    }];

    Some(SignatureHelp {
        signatures,
        active_signature: Some(0),
        active_parameter: Some(active as u32),
    })
}

/// Parse `content` as JSON and resolve signature help at an LSP `Position`
/// (converted through `crate::completion::position_to_offset`, the same
/// UTF-16 conversion every other position-aware handler uses).
pub fn signature_help_at_position(
    content: &str,
    position: lsp_types_max::Position,
) -> Option<SignatureHelp> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .ok()?;
    let tree = parser.parse(content, None)?;
    let offset = position_to_offset(content, position)?;
    signature_help_at_offset(tree.root_node(), content, offset)
}
