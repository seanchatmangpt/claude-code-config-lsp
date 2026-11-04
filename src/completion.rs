//! Completion Handler: textDocument/completion
//! Position-aware: resolves the cursor's enclosing JSON key path via the
//! tree-sitter JSON tree before consulting the schema tables. Falls back to
//! the unfiltered whole-surface list when the position cannot be resolved.

use lsp_max::jsonrpc::Result;
use lsp_types_max::*;
use tree_sitter::Node;

use crate::hover::schema_key_for_uri;
use crate::schema::build_schema;

/// Where the cursor sits relative to the enclosing JSON structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorContext {
    /// Cursor is on a JSON key. `siblings` are all keys of the enclosing
    /// object (including the hovered key itself); `top_level` is true when
    /// the enclosing object is the document's root object.
    Key {
        key: String,
        parent_key: Option<String>,
        siblings: Vec<String>,
        top_level: bool,
    },
    /// Cursor is inside the value of key `key`.
    Value { key: String },
    /// No enclosing pair (empty object, malformed JSON, cursor outside any
    /// key/value) - callers fall back to unfiltered behavior.
    Unknown,
}

/// Convert an LSP Position (UTF-16 columns) to a byte offset in `content`.
/// Returns `None` when the position's line is past EOF; the column is
/// clamped to the line's end.
pub fn position_to_offset(content: &str, pos: Position) -> Option<usize> {
    let mut base = 0usize;
    for (i, text) in content.split('\n').enumerate() {
        if i == pos.line as usize {
            let mut u = 0usize;
            let mut byte = 0usize;
            for ch in text.chars() {
                if u + ch.len_utf16() > pos.character as usize {
                    break;
                }
                u += ch.len_utf16();
                byte += ch.len_utf8();
            }
            return Some((base + byte).min(content.len()));
        }
        base += text.len() + 1;
    }
    None
}

/// Text of a `pair` node's key, unquoted (via its `string_content` child,
/// falling back to quote-stripping the raw key span).
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

/// Keys of all `pair` children of an object node.
fn object_keys(obj: Node<'_>, content: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut cursor = obj.walk();
    for child in obj.children(&mut cursor) {
        if child.kind() == "pair" {
            if let Some(k) = key_text(child, content) {
                keys.push(k);
            }
        }
    }
    keys
}

/// Resolve what the cursor at byte `offset` is on, by walking tree-sitter
/// ancestors from the innermost node covering the offset.
pub fn resolve_cursor_context(root: Node<'_>, content: &str, offset: usize) -> CursorContext {
    let offset = offset.min(content.len());
    let node = match root.descendant_for_byte_range(offset, offset) {
        Some(n) => n,
        None => return CursorContext::Unknown,
    };

    let mut pair: Option<Node> = None;
    let mut cur = Some(node);
    while let Some(n) = cur {
        if n.kind() == "pair" {
            pair = Some(n);
            break;
        }
        cur = n.parent();
    }
    let Some(pair) = pair else {
        return CursorContext::Unknown;
    };
    let Some(key) = key_text(pair, content) else {
        return CursorContext::Unknown;
    };

    let key_node = pair.child_by_field_name("key");
    let on_key = match key_node {
        Some(kn) => offset >= kn.start_byte() && offset <= kn.end_byte(),
        None => true,
    };

    if on_key {
        let obj = pair.parent();
        let siblings = obj.map(|o| object_keys(o, content)).unwrap_or_default();
        let top_level = obj
            .and_then(|o| o.parent())
            .is_some_and(|d| d.parent().is_none());
        let parent_key = pair
            .parent()
            .and_then(|o| o.parent())
            .and_then(|enclosing| key_text(enclosing, content));
        return CursorContext::Key {
            key,
            parent_key,
            siblings,
            top_level,
        };
    }

    CursorContext::Value { key }
}

/// Enum value suggestions for a known key, sourced from `analyzers/json.rs`
/// (single source of truth - imported, not duplicated).
fn enum_values_for_key(key: &str) -> Option<(&'static [&'static str], &'static str)> {
    use crate::analyzers::json::{
        VALID_EFFORT_LEVELS, VALID_HOOK_EVENTS, VALID_MODELS, VALID_PERMISSION_MODES,
    };
    match key {
        "model" => Some((VALID_MODELS, "model")),
        "effortLevel" => Some((VALID_EFFORT_LEVELS, "effortLevel")),
        "permissionMode" | "defaultMode" => Some((VALID_PERMISSION_MODES, "permissionMode")),
        "hooks" => Some((VALID_HOOK_EVENTS, "hook event")),
        _ => None,
    }
}

/// Build completion items for enum values.
fn enum_items(values: &'static [&'static str], what: &str) -> Vec<CompletionItem> {
    values
        .iter()
        .map(|v| CompletionItem {
            label: (*v).to_string(),
            kind: Some(CompletionItemKind::ENUM_MEMBER),
            detail: Some(format!("valid {what} value")),
            ..Default::default()
        })
        .collect()
}

/// Schema-field completion items, optionally excluding keys already present
/// in the document at the cursor's level.
fn field_items(schema_key: &str, exclude: &[String]) -> Option<Vec<CompletionItem>> {
    let schema = build_schema();
    let node = schema.types.get(schema_key)?;
    Some(
        node.fields
            .iter()
            .filter(|(name, _)| !exclude.contains(name))
            .map(|(name, ft)| {
                let opt = if ft.is_optional {
                    "optional"
                } else {
                    "required"
                };
                CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::FIELD),
                    detail: Some(format!("{} - {opt} ({})", ft.name, ft.stability)),
                    ..Default::default()
                }
            })
            .collect(),
    )
}

/// Position-aware completions. Returns `None` when the URI does not classify
/// to a known schema surface; falls back to the unfiltered schema field list
/// when the cursor context cannot be resolved from the tree.
pub fn completions_at_position(
    uri: &str,
    content: &str,
    root: Option<Node<'_>>,
    position: Position,
) -> Option<CompletionResponse> {
    let schema_key = schema_key_for_uri(uri);
    if schema_key.is_empty() {
        return None;
    }

    let ctx = root
        .and_then(|r| {
            position_to_offset(content, position)
                .map(|offset| resolve_cursor_context(r, content, offset))
        })
        .unwrap_or(CursorContext::Unknown);

    let items = match ctx {
        CursorContext::Value { key } => match enum_values_for_key(&key) {
            Some((values, what)) => enum_items(values, what),
            None => field_items(schema_key, &[])?,
        },
        CursorContext::Key {
            key: _,
            parent_key,
            siblings,
            top_level,
        } => {
            // On a key inside the `hooks` object: suggest hook event names
            // not already present as sibling keys.
            if parent_key.as_deref() == Some("hooks") {
                let (events, what) = enum_values_for_key("hooks").unwrap_or((&[], "hook event"));
                let items = enum_items(events, what)
                    .into_iter()
                    .filter(|i| !siblings.iter().any(|s| s == &i.label))
                    .collect();
                return Some(CompletionResponse::List(CompletionList {
                    is_incomplete: false,
                    items,
                    ..Default::default()
                }));
            }
            // Top-level key: schema fields minus keys already present;
            // nested non-hooks keys: unfiltered schema fields.
            if top_level {
                field_items(schema_key, &siblings)?
            } else {
                let _ = parent_key;
                field_items(schema_key, &[])?
            }
        }
        CursorContext::Unknown => field_items(schema_key, &[])?,
    };

    Some(CompletionResponse::List(CompletionList {
        is_incomplete: false,
        items,
        ..Default::default()
    }))
}

/// Resolve additional details for a completion item.
pub async fn completion_item_resolve(item: CompletionItem) -> Result<CompletionItem> {
    Ok(item)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(content: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .expect("json grammar");
        parser.parse(content, None).expect("parser")
    }

    fn complete_at(content: &str, line: u32, character: u32) -> Vec<String> {
        let tree = parse(content);
        let uri = "file:///proj/.claude/settings.json";
        let resp = completions_at_position(
            uri,
            content,
            Some(tree.root_node()),
            Position::new(line, character),
        )
        .expect("settings.json always classifies");
        match resp {
            CompletionResponse::List(l) => l.items.into_iter().map(|i| i.label).collect(),
            _ => panic!("expected list"),
        }
    }

    const FIXTURE: &str =
        "{\n  \"model\": \"sonnet\",\n  \"hooks\": {\n    \"PreToolUse\": []\n  }\n}\n";

    #[test]
    fn position_to_offset_basic() {
        assert_eq!(position_to_offset("ab\ncd", Position::new(1, 0)), Some(3));
        assert_eq!(position_to_offset("ab\ncd", Position::new(0, 2)), Some(2));
        assert_eq!(position_to_offset("ab\ncd", Position::new(5, 0)), None);
    }

    #[test]
    fn top_level_key_completion_excludes_existing_keys() {
        // Cursor on the "hooks" key at top level: "model" already present,
        // so it must NOT be suggested.
        let items = complete_at(FIXTURE, 2, 2); // line 2 = `  "hooks": {`
        assert!(
            !items.contains(&"model".to_string()),
            "existing key 'model' must be filtered: {items:?}"
        );
        assert!(items.contains(&"effortLevel".to_string()));
        assert!(items.contains(&"permissions".to_string()));
    }

    #[test]
    fn hook_event_completion_inside_hooks_object() {
        // Cursor on the "PreToolUse" key inside hooks: hook events, minus
        // the already-present PreToolUse.
        let items = complete_at(FIXTURE, 3, 4); // line 3 = `    "PreToolUse": []`
        assert!(items.contains(&"PostToolUse".to_string()));
        assert!(items.contains(&"SessionStart".to_string()));
        assert!(
            !items.contains(&"PreToolUse".to_string()),
            "existing hook event filtered: {items:?}"
        );
        assert!(
            !items.contains(&"model".to_string()),
            "no schema fields inside hooks: {items:?}"
        );
    }

    #[test]
    fn value_completion_suggests_model_enum() {
        // Cursor inside the value of "model".
        let items = complete_at(FIXTURE, 1, 11); // inside "sonnet"
        assert!(items.contains(&"sonnet".to_string()));
        assert!(items.contains(&"opus".to_string()));
        assert!(
            !items.contains(&"hooks".to_string()),
            "no field labels for a value completion: {items:?}"
        );
    }

    #[test]
    fn value_completion_suggests_effort_enum() {
        let content = "{\n  \"effortLevel\": \"\"\n}\n";
        let items = complete_at(content, 1, 16); // inside the empty string value
        assert!(items.contains(&"max".to_string()));
        assert!(!items.contains(&"effortLevel".to_string()));
    }

    #[test]
    fn permission_mode_value_completion() {
        let content = "{\n  \"permissionMode\": \"au\"\n}\n";
        let items = complete_at(content, 1, 19);
        assert!(items.contains(&"auto".to_string()));
        assert!(!items.contains(&"model".to_string()));
    }

    #[test]
    fn unknown_context_falls_back_to_full_field_list() {
        // Empty object: no pair to resolve -> Unknown -> full unfiltered list.
        let items = complete_at("{}\n", 0, 1);
        assert!(items.contains(&"model".to_string()));
        assert!(items.contains(&"hooks".to_string()));
    }

    #[test]
    fn unknown_uri_returns_none() {
        let tree = parse("{}");
        assert!(completions_at_position(
            "file:///notes.txt",
            "{}",
            Some(tree.root_node()),
            Position::new(0, 1)
        )
        .is_none());
    }
}
