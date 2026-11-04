//! rename — textDocument/prepareRename + textDocument/rename for JSON keys in
//! settings-like docs.
//!
//! Pure functions over document content: parse with tree-sitter JSON (same
//! grammar setup as `analyzers/json.rs` / `hover.rs`), locate the key token
//! at the requested position, and produce a `WorkspaceEdit`
//! (TextDocumentEdit) that replaces the key's inner text (quotes preserved).
//! A rename to a name violating the house key grammar returns `None` — the
//! client surfaces that as an error.
//!
//! House key grammar: `lowercase_snake` segments joined by `.` — each
//! segment matches `[a-z][a-z0-9_]*`.

use lsp_types_max::{
    DocumentChanges, OneOf3, OptionalVersionedTextDocumentIdentifier, Position, Range,
    TextDocumentEdit, TextEdit, Url, WorkspaceEdit,
};
use tree_sitter::Node;

/// Parse `content` with the tree-sitter JSON grammar.
fn parse(content: &str) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .ok()?;
    parser.parse(content, None)
}

/// Byte offset of a (line, character) position. LSP columns are UTF-16 code
/// units; this module assumes ASCII text (settings keys are ASCII in
/// practice), where character == byte column.
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

/// (line, character) of a byte offset (same ASCII assumption as `offset_of`).
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

/// The `pair` key string node at `pos`, if `pos` is inside a pair's key.
/// Descends from the root through document/object/array containers to the
/// `pair` whose key spans the position's byte offset.
fn key_node_at<'t>(root: Node<'t>, content: &str, pos: Position) -> Option<Node<'t>> {
    let offset = offset_of(content, pos);
    let mut current = root.child(0)?;
    loop {
        match current.kind() {
            "pair" => {
                let key = current.child_by_field_name("key")?;
                if key.start_byte() <= offset && offset <= key.end_byte() {
                    return Some(string_child(key).unwrap_or(key));
                }
                let value = current.child_by_field_name("value")?;
                current = value;
            }
            "document" | "object" | "array" => {
                let mut cursor = current.walk();
                let next = current
                    .children(&mut cursor)
                    .find(|c| c.start_byte() <= offset && offset < c.end_byte())?;
                current = next;
            }
            _ => return None,
        }
    }
}

/// Prefer the named `string` child of the key node (tree-sitter-json 0.23
/// wraps key text in a `string` node whose own child is `string_content`).
fn string_child(node: Node<'_>) -> Option<Node<'_>> {
    let mut cursor = node.walk();
    let found = node.children(&mut cursor).find(|c| c.kind() == "string");
    found
}

/// Inner range of a `string` node: excludes the surrounding quote chars.
fn inner_range(key: Node<'_>, content: &str) -> Range {
    let text = &content[key.start_byte()..key.end_byte()];
    let strip = usize::from(text.starts_with('"'));
    let end_strip = usize::from(text.ends_with('"') && text.len() > strip);
    Range {
        start: pos_of(content, key.start_byte() + strip),
        end: pos_of(content, key.end_byte() - end_strip),
    }
}

/// Quote-free range of the key token at `pos`, or `None` if not on a key.
pub fn key_range_at(content: &str, pos: Position) -> Option<Range> {
    let tree = parse(content)?;
    let key = key_node_at(tree.root_node(), content, pos)?;
    Some(inner_range(key, content))
}

/// textDocument/prepareRename: quote-free range of the key token at `pos`.
pub fn prepare_rename(content: &str, pos: Position) -> Option<Range> {
    key_range_at(content, pos)
}

/// Is `name` valid per the house key grammar: dot-joined segments, each
/// `[a-z][a-z0-9_]*` (lowercase_snake or dot.notation).
pub fn is_valid_key_name(name: &str) -> bool {
    !name.is_empty()
        && name.split('.').all(|seg| {
            let mut chars = seg.chars();
            match chars.next() {
                Some(c) if c.is_ascii_lowercase() => {
                    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                }
                _ => false,
            }
        })
}

/// textDocument/rename: replace the key at `pos` with `new_name`. Returns
/// `None` when there is no key at `pos` or `new_name` violates the house
/// grammar (client renders that as an error).
pub fn rename(content: &str, pos: Position, new_name: &str, uri: &Url) -> Option<WorkspaceEdit> {
    if !is_valid_key_name(new_name) {
        return None;
    }
    let range = key_range_at(content, pos)?;
    Some(WorkspaceEdit {
        document_changes: Some(DocumentChanges::Edits(vec![TextDocumentEdit {
            text_document: OptionalVersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: Some(-1), // client-side version check governs
            },
            edits: vec![OneOf3::A(TextEdit {
                range,
                new_text: new_name.to_string(),
            })],
        }])),
        ..Default::default()
    })
}
