//! Handler: textDocument/hover
//! Position-aware: resolves the hovered JSON key via the tree-sitter JSON
//! tree and returns just that key's schema entry. Falls back to the
//! whole-surface dump when the position cannot be resolved.

use lsp_types_max::*;
use tree_sitter::Node;

use crate::completion::{position_to_offset, resolve_cursor_context, CursorContext};
use crate::schema::build_schema;

/// Position-aware hover. Returns `None` when the URI does not classify
/// to a known schema surface; falls back to the whole-surface dump when the
/// cursor does not resolve to a known key.
pub fn hover_at_position(
    uri: &str,
    content: &str,
    root: Option<Node<'_>>,
    position: Position,
) -> Option<Hover> {
    let schema_key = schema_key_for_uri(uri);
    if schema_key.is_empty() {
        return None;
    }
    let schema = build_schema();
    let surface = schema.types.get(schema_key)?;

    let ctx = root
        .and_then(|r| {
            position_to_offset(content, position)
                .map(|offset| resolve_cursor_context(r, content, offset))
        })
        .unwrap_or(CursorContext::Unknown);

    // Position resolved to a key (or a value of a key) with a schema entry:
    // return just that key's entry.
    let hovered_key = match &ctx {
        CursorContext::Key { key, .. } => Some(key.clone()),
        CursorContext::Value { key } => Some(key.clone()),
        CursorContext::Unknown => None,
    };

    if let Some(key) = hovered_key {
        if let Some((name, ft)) = surface.fields.iter().find(|(n, _)| n == &key) {
            let opt = if ft.is_optional {
                "optional"
            } else {
                "required"
            };
            let value = format!(
                "**`{name}`** `{}` - {opt} ({})\n\n*(surface: {schema_key})*",
                ft.name, ft.stability
            );
            return Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                range: None,
            });
        }
    }

    // Fallback: whole-surface dump (previous behavior, kept for unresolved
    // positions / unknown keys / non-JSON surfaces).
    let lines: Vec<String> = surface
        .fields
        .iter()
        .map(|(name, ft)| {
            let opt = if ft.is_optional {
                "optional"
            } else {
                "required"
            };
            format!("**`{name}`** `{}` - {opt} ({})", ft.name, ft.stability)
        })
        .collect();
    if lines.is_empty() {
        return None;
    }
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("## {schema_key}\n\n{}", lines.join("\n\n")),
        }),
        range: None,
    })
}

pub fn schema_key_for_uri(uri: &str) -> &'static str {
    let u = uri.to_lowercase();
    if u.contains("settings") && u.ends_with(".json") {
        "Claude Code settings.json"
    } else if u.contains("skill.md") || u.contains("/skills/") {
        "Skill definition (.claude/skills/*/SKILL.md)"
    } else if u.contains("plugin.json") {
        "Plugin manifest (plugin.json)"
    } else if u.contains("/agents/") && u.ends_with(".md") {
        "Agent definition (.claude/agents/*.md)"
    } else if u.ends_with("mcp.json") {
        "MCP server config (mcp.json)"
    } else if u.ends_with("claude.md") || u.ends_with("agents.md") {
        "Project documentation (CLAUDE.md)"
    } else {
        ""
    }
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

    /// Returns the hover markdown at the given position for a settings.json.
    fn hover_text_at(content: &str, line: u32, character: u32) -> String {
        let tree = parse(content);
        let uri = "file:///proj/.claude/settings.json";
        match hover_at_position(
            uri,
            content,
            Some(tree.root_node()),
            Position::new(line, character),
        )
        .expect("settings.json always classifies")
        .contents
        {
            HoverContents::Markup(m) => m.value,
            other => panic!("expected markup contents, got {other:?}"),
        }
    }

    const FIXTURE: &str = "{\n  \"model\": \"sonnet\",\n  \"effortLevel\": \"high\"\n}\n";

    #[test]
    fn hover_on_key_returns_just_that_entry() {
        let text = hover_text_at(FIXTURE, 2, 4);
        assert!(text.contains("effortLevel"), "{text}");
        assert!(text.contains("low | medium | high | xhigh | max"), "{text}");
        assert!(
            !text.contains("**`model`**"),
            "keyed hover must not dump the whole surface: {text}"
        );
    }

    #[test]
    fn hover_on_value_shows_field_entry() {
        let text = hover_text_at(FIXTURE, 1, 11);
        assert!(text.contains("**`model`**"), "{text}");
        assert!(text.contains("opus | sonnet"), "{text}");
        assert!(
            !text.contains("**`effortLevel`**"),
            "must not dump whole surface: {text}"
        );
    }

    #[test]
    fn hover_on_unknown_key_falls_back_to_surface_dump() {
        let content = "{\n  \"bogusKey\": 1\n}\n";
        let text = hover_text_at(content, 1, 4);
        assert!(text.contains("## Claude Code settings.json"), "{text}");
        assert!(text.contains("**`model`**"), "{text}");
        assert!(text.contains("**`hooks`**"), "{text}");
    }

    #[test]
    fn hover_with_no_tree_falls_back_to_surface_dump() {
        let tree = None;
        let h = hover_at_position(
            "file:///proj/.claude/settings.json",
            FIXTURE,
            tree,
            Position::new(1, 4),
        )
        .expect("classifies");
        let text = match h.contents {
            HoverContents::Markup(m) => m.value,
            other => panic!("expected markup, got {other:?}"),
        };
        assert!(text.contains("## Claude Code settings.json"), "{text}");
        assert!(text.contains("**`model`**"), "{text}");
    }

    #[test]
    fn unknown_uri_returns_none() {
        assert!(hover_at_position("file:///notes.txt", "{}", None, Position::new(0, 0)).is_none());
    }
}
