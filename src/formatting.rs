//! JSON document/range formatting for claude-code config surfaces.
//!
//! # Laws
//!
//! - **Key order is PRESERVED as the user wrote it.** `settings.local.json`
//!   overlays depend on the author's grouping; reordering keys is a refusal.
//!   Enforced by `serde_json`'s `preserve_order` feature (IndexMap-backed
//!   `Map`), never by sorting.
//! - **Invalid JSON is never reformatted.** If the text does not parse as
//!   JSON, formatting returns `None` — a formatter must not mangle a file it
//!   cannot understand (trailing commas, comments in jsonc-style edits, or a
//!   mid-keystroke buffer all arrive here).
//! - **Idempotence**: `format(format(x)) == format(x)`. Enforced by test.
//!
//! Indentation is 2 spaces (Claude Code config house style).

use lsp_types_max::{FormattingOptions, Range, TextEdit};
use serde_json::Value;

/// 2-space indent unit; the only indentation this module emits.
const INDENT: &str = "  ";

/// Pretty-print a parsed JSON `Value` at the given base indentation depth,
/// preserving object key insertion order.
fn render(value: &Value, depth: usize) -> String {
    let pad = INDENT.repeat(depth);
    let pad_inner = INDENT.repeat(depth + 1);
    match value {
        Value::Object(map) => {
            if map.is_empty() {
                return "{}".to_string();
            }
            let items: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}{}: {}",
                        pad_inner,
                        serde_json::Value::String(k.clone()),
                        render(v, depth + 1)
                    )
                })
                .collect();
            format!("{{\n{}\n{}}}", items.join(",\n"), pad)
        }
        Value::Array(items) => {
            if items.is_empty() {
                return "[]".to_string();
            }
            let rendered: Vec<String> = items
                .iter()
                .map(|v| format!("{}{}", pad_inner, render(v, depth + 1)))
                .collect();
            format!("[\n{}\n{}]", rendered.join(",\n"), pad)
        }
        other => other.to_string(),
    }
}

/// Format a full JSON document. Returns `None` for invalid JSON (law: never
/// reformat what we cannot parse). The whole document is returned as one
/// `TextEdit` replacing the full given range (LSP convention: the server
/// passes the document's full range; the edit text is the formatted output).
pub fn format_document(text: &str, range: Range) -> Option<Vec<TextEdit>> {
    let value: Value = serde_json::from_str(text).ok()?;
    let formatted = render(&value, 0);
    Some(vec![TextEdit {
        range,
        new_text: formatted,
    }])
}

/// Format only the region covered by `range`: parse the whole document
/// (tolerance law applies to the whole text), then slice the affected region
/// out of the fully-formatted output. Lines outside `range` are untouched,
/// so the client applies a minimal edit.
///
/// Returns `None` for invalid JSON or if the range is not line-bounded
/// within the formatted output.
pub fn format_range(
    text: &str,
    range: Range,
    _options: &FormattingOptions,
) -> Option<Vec<TextEdit>> {
    let value: Value = serde_json::from_str(text).ok()?;
    let formatted = render(&value, 0);
    let lines: Vec<&str> = formatted.lines().collect();
    let start = range.start.line as usize;
    let end = range.end.line as usize;
    if start > end || end >= lines.len() {
        return None;
    }
    let new_text = lines[start..=end].join("\n");
    Some(vec![TextEdit { range, new_text }])
}

/// Best-effort on-type formatting. `trigger` is the character just typed.
/// Only `"` and `:` trigger. On `:` after a key, we do nothing structural
/// (serde ordering is already correct); on `"` closing a key or value we
/// reformat the enclosing line's indentation if the document is currently
/// valid JSON. Returns an empty vec when no correction is needed.
pub fn on_type_format(
    text: &str,
    position: lsp_types_max::Position,
    trigger: char,
) -> Option<Vec<TextEdit>> {
    if trigger != '"' && trigger != ':' {
        return None;
    }
    // Tolerance law: only act on valid JSON.
    let _value: Value = serde_json::from_str(text).ok()?;
    let lines: Vec<&str> = text.lines().collect();
    let line_idx = position.line as usize;
    let line = *lines.get(line_idx)?;
    let want_indent = compute_indent(line_idx, &lines);
    let current_indent = line.len() - line.trim_start().len();
    let want_len = want_indent.len();
    if current_indent == want_len {
        return None;
    }
    Some(vec![TextEdit {
        range: Range {
            start: lsp_types_max::Position {
                line: position.line,
                character: 0,
            },
            end: lsp_types_max::Position {
                line: position.line,
                character: current_indent as u32,
            },
        },
        new_text: want_indent,
    }])
}

/// Compute the expected indent (in spaces) for line `idx` by scanning the
/// raw (unformatted) text: depth increases on lines ending in `{`/`[`,
/// decreases on lines starting with `}`/`]`.
fn compute_indent(idx: usize, lines: &[&str]) -> String {
    let mut depth: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if i == idx {
            break;
        }
        if trimmed.starts_with('}') || trimmed.starts_with(']') {
            depth = depth.saturating_sub(1);
        }
        if trimmed.ends_with('{') || trimmed.ends_with('[') {
            depth += 1;
        }
    }
    INDENT.repeat(depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_range(text: &str) -> Range {
        let lines = text.lines().count() as u32;
        Range {
            start: lsp_types_max::Position {
                line: 0,
                character: 0,
            },
            end: lsp_types_max::Position {
                line: lines,
                character: 0,
            },
        }
    }

    #[test]
    fn preserves_key_order() {
        let src = r#"{"zebra": 1, "alpha": {"y": 2, "b": 3}, "middle": 4}"#;
        let edits = format_document(src, full_range(src)).expect("valid json formats");
        let out = &edits[0].new_text;
        let z = out.find("\"zebra\"").unwrap();
        let a = out.find("\"alpha\"").unwrap();
        let m = out.find("\"middle\"").unwrap();
        let y = out.find("\"y\"").unwrap();
        let b = out.find("\"b\"").unwrap();
        assert!(z < a && a < m, "top-level order preserved");
        assert!(y < b, "nested order preserved");
    }

    #[test]
    fn invalid_json_returns_none() {
        let src = "{\"a\": 1,,}"; // tolerant-refusal case
        assert!(format_document(src, full_range(src)).is_none());
    }

    #[test]
    fn idempotent() {
        let src = r#"{"b":1,"a":{"d":[1,2],"c":true}}"#;
        let once = format_document(src, full_range(src)).unwrap()[0]
            .new_text
            .clone();
        let twice = format_document(&once, full_range(&once)).unwrap()[0]
            .new_text
            .clone();
        assert_eq!(once, twice);
    }
}
