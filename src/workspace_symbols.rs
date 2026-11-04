//! workspace/symbol — symbol extraction over a Claude-config directory tree.
//!
//! Reuses [`crate::scan::classify`] (read-only import) to decide which files
//! are config surfaces, then extracts one LSP [`SymbolInformation`] per surface
//! symbol:
//!
//! | surface | symbol | kind |
//! |---|---|---|
//! | settings-family JSON (`classify` = "json") | dotted key path (`a.b.c`) | PROPERTY |
//! | `SKILL.md` / `/skills/` ("skill") | frontmatter `name` | CLASS |
//! | `/agents/*.md` ("agent") | frontmatter `name` | CLASS |
//! | `CLAUDE.md` / `AGENTS.md` ("claude_md") | H1 title (or file stem when no H1) | FILE |
//! | `/commands/*.md` | file stem | FUNCTION |
//!
//! Depth cap 4 directories below the root; `.git`/`target`/`node_modules`/`_build`
//! pruned. Query filtering is case-insensitive substring on the symbol name
//! (LSP workspace/symbol semantics; empty query = all symbols). Note:
//! `/commands/*.md` classifies as "unknown" in [`crate::scan::classify`], so it
//! is surfaced by directory convention (parent dir named `commands`), not by
//! classify — commands are a real Claude Code surface even though the
//! diagnostic classifier does not route them.

use lsp_types_max::{Location, Position, Range, SymbolInformation, SymbolKind, Uri};

use std::str::FromStr as _;

/// Directories never descended into.
const PRUNED_DIRS: &[&str] = &[".git", "target", "node_modules", "_build"];

/// Maximum directory depth (levels below `root`) to descend.
const MAX_DEPTH: u32 = 4;

/// Scan `root` for Claude-config symbols, filtered by `query`.
///
/// Empty `query` returns every symbol (LSP workspace/symbol: empty query =
/// all). Matching is case-insensitive substring on the symbol name.
pub fn workspace_symbols(root: &std::path::Path, query: &str) -> Vec<SymbolInformation> {
    let mut out = Vec::new();
    walk(root, root, 0, query, &mut out);
    out
}

fn walk(
    root: &std::path::Path,
    dir: &std::path::Path,
    depth: u32,
    query: &str,
    out: &mut Vec<SymbolInformation>,
) {
    if depth > MAX_DEPTH {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).to_owned();
        if path.is_dir() {
            if name.is_some_and(|n| PRUNED_DIRS.contains(&n)) {
                continue;
            }
            walk(root, &path, depth + 1, query, out);
        } else {
            emit_symbols_for_file(root, &path, query, out);
        }
    }
}

fn emit_symbols_for_file(
    root: &std::path::Path,
    path: &std::path::Path,
    query: &str,
    out: &mut Vec<SymbolInformation>,
) {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();
    let lower = file_name.to_lowercase();
    let path_str = path.to_string_lossy().replace('\\', "/");
    let parent_dir = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Commands: parent directory named `commands`, .md file.
    if parent_dir == "commands" && lower.ends_with(".md") {
        let stem = file_name.trim_end_matches(".md");
        push_symbol(out, root, path, stem, SymbolKind::FUNCTION, query, (0, 0));
        return;
    }

    // Everything else routes through the shared classifier.
    let kind = crate::scan::classify(&path_str);
    match kind {
        "json" => {
            let Ok(content) = std::fs::read_to_string(path) else {
                return;
            };
            for (dotted, line, col) in json_property_symbols(&content) {
                push_symbol(
                    out,
                    root,
                    path,
                    &dotted,
                    SymbolKind::PROPERTY,
                    query,
                    (line, col),
                );
            }
        }
        "skill" | "agent" => {
            let Ok(content) = std::fs::read_to_string(path) else {
                return;
            };
            if let Some(name) = frontmatter_name(&content) {
                push_symbol(out, root, path, &name, SymbolKind::CLASS, query, (0, 0));
            }
        }
        "claude_md" => {
            let Ok(content) = std::fs::read_to_string(path) else {
                return;
            };
            let (title, line) = markdown_h1(&content);
            let name = title.unwrap_or_else(|| file_name.clone());
            push_symbol(out, root, path, &name, SymbolKind::FILE, query, (line, 0));
        }
        _ => {}
    }
}

/// Build and register one symbol if it passes the query filter.
fn push_symbol(
    out: &mut Vec<SymbolInformation>,
    root: &std::path::Path,
    path: &std::path::Path,
    name: &str,
    kind: SymbolKind,
    query: &str,
    pos: (u32, u32),
) {
    if !query.is_empty() && !name.to_lowercase().contains(&query.to_lowercase()) {
        return;
    }
    let Some(uri) = file_uri(path) else {
        return;
    };
    let range = Range::new(Position::new(pos.0, pos.1), Position::new(pos.0, pos.1));
    // container_name: the root-relative directory of the file, so clients can
    // distinguish same-named symbols across surfaces.
    let container = path
        .parent()
        .and_then(|p| p.strip_prefix(root).ok())
        .map(|r| r.to_string_lossy().to_string())
        .filter(|s| !s.is_empty());
    out.push(SymbolInformation {
        name: name.to_string(),
        kind,
        tags: None,
        // `deprecated` is deprecated in LSP 3.16+ in favor of `tags`; the
        // struct requires the field — construct with None under allow.
        #[allow(deprecated)]
        deprecated: None,
        location: Location::new(uri, range),
        container_name: container,
    });
}

/// Convert a filesystem path to a `file://` [`Uri`], percent-encoding
/// characters a URI cannot carry literally.
fn file_uri(path: &std::path::Path) -> Option<Uri> {
    let mut s = String::from("file://");
    for c in path.to_string_lossy().chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '.' | '_' | '~' | '/' | ':' => s.push(c),
            _ => {
                let mut buf = [0u8; 4];
                for b in c.encode_utf8(&mut buf).bytes() {
                    s.push_str(&format!("%{b:02X}"));
                }
            }
        }
    }
    Uri::from_str(&s).ok()
}

/// Extract dotted-path property symbols from JSON text.
///
/// Walks the parsed value recursively collecting `a.b.c` paths (array elements
/// contribute their index as a segment), then locates each leaf's position by a
/// monotonic scan of the text for the leaf key's quoted final segment — JSON
/// object keys appear in document order, so the monotonic scan matches them in
/// symbol order. Keys whose quoted segment cannot be located (e.g. duplicated
/// leaf key names that would break monotonicity) are still emitted, positioned
/// at the last matched location.
fn json_property_symbols(content: &str) -> Vec<(String, u32, u32)> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(content) else {
        return Vec::new();
    };
    let mut paths: Vec<String> = Vec::new();
    collect_json_paths(&value, String::new(), &mut paths);

    let lines: Vec<&str> = content.lines().collect();
    // Prefix-sum of line start byte offsets (content.lines() drops the \n).
    let mut line_starts = Vec::with_capacity(lines.len());
    let mut acc = 0usize;
    for l in &lines {
        line_starts.push(acc);
        acc += l.len() + 1; // +1 for the newline dropped by .lines()
    }

    let mut cursor_line = 0usize;
    let mut cursor_byte = 0usize;
    let mut out = Vec::with_capacity(paths.len());
    for path in &paths {
        let final_seg = path.rsplit('.').next().unwrap_or(path);
        let needle = format!("\"{}\"", final_seg);
        let mut found = None;
        for (i, line) in lines.iter().enumerate().skip(cursor_line) {
            let line_start = line_starts[i];
            let from_col = if i == cursor_line {
                cursor_byte - line_start
            } else {
                0
            };
            let from_col = from_col.min(line.len());
            if let Some(byte_col) = line[from_col..].find(&needle) {
                found = Some((
                    i as u32,
                    (from_col + byte_col + 1) as u32, // +1: past the opening quote
                    line_start + from_col + byte_col + needle.len(),
                ));
                break;
            }
        }
        match found {
            Some((line, col, abs_end)) => {
                cursor_line = line as usize;
                cursor_byte = abs_end;
                out.push((path.clone(), line, col));
            }
            None => {
                // Emit at the last known position rather than dropping the
                // symbol entirely.
                out.push((path.clone(), cursor_line as u32, 0));
            }
        }
    }
    out
}

/// Recursive dotted-path collection over a parsed JSON value.
fn collect_json_paths(value: &serde_json::Value, prefix: String, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let child = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                collect_json_paths(v, child, out);
            }
        }
        serde_json::Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                let child = if prefix.is_empty() {
                    i.to_string()
                } else {
                    format!("{}.{}", prefix, i)
                };
                collect_json_paths(v, child, out);
            }
        }
        // Scalar leaf — one property symbol.
        _ => out.push(prefix),
    }
}

/// Extract the `name:` field from a `---` YAML frontmatter block.
fn frontmatter_name(content: &str) -> Option<String> {
    let mut lines = content.lines();
    if lines.next()?.trim_end() != "---" {
        return None;
    }
    for line in lines {
        let t = line.trim_end();
        if t == "---" {
            break;
        }
        if let Some(rest) = t.strip_prefix("name:") {
            let v = rest.trim();
            let v = v.trim_matches('"').trim_matches('\'');
            return Some(v.to_string());
        }
    }
    None
}

/// Find the first ATX H1 (`# Title`) in markdown; returns (title, line).
fn markdown_h1(content: &str) -> (Option<String>, u32) {
    for (i, line) in content.lines().enumerate() {
        if let Some(rest) = line.strip_prefix("# ") {
            return (Some(rest.trim().to_string()), i as u32);
        }
    }
    (None, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontmatter_name_parses_quoted_and_bare() {
        let md = "---\nname: \"my-agent\"\ndescription: x\n---\nbody\n";
        assert_eq!(frontmatter_name(md).as_deref(), Some("my-agent"));
        let md2 = "---\nname: bare-name\n---\n";
        assert_eq!(frontmatter_name(md2).as_deref(), Some("bare-name"));
        let md3 = "no frontmatter\n";
        assert!(frontmatter_name(md3).is_none());
    }

    #[test]
    fn markdown_h1_extracts_title() {
        assert_eq!(
            markdown_h1("# My Title\nbody"),
            (Some("My Title".into()), 0)
        );
        assert_eq!(
            markdown_h1("intro\n\n# Later H1\n"),
            (Some("Later H1".into()), 2)
        );
        assert_eq!(markdown_h1("## not h1\n"), (None, 0));
    }

    #[test]
    fn dotted_paths_flatten_nested_json() {
        let content = r#"{"a": {"b": {"c": 1}}, "d": [10, 20], "e": "str"}"#;
        let syms = json_property_symbols(content);
        let names: Vec<&str> = syms.iter().map(|(n, _, _)| n.as_str()).collect();
        assert_eq!(names, vec!["a.b.c", "d.0", "d.1", "e"]);
    }

    #[test]
    fn invalid_json_yields_no_symbols() {
        assert!(json_property_symbols("{not json").is_empty());
    }

    #[test]
    fn file_uri_percent_encodes_spaces() {
        let uri = file_uri(std::path::Path::new("/tmp/a b/c.json")).expect("uri");
        assert_eq!(uri.as_str(), "file:///tmp/a%20b/c.json");
    }
}
