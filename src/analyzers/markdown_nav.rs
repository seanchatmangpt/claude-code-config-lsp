//! Markdown navigation for Claude Code config docs (CLAUDE.md, SKILL.md,
//! agents/*.md).
//!
//! Pure, line-wise markdown structuring - no new dependencies, no tree-sitter:
//! - `parse_headings` walks ATX `#` headings (skipping fenced code blocks)
//! - `document_symbols` builds hierarchical LSP `DocumentSymbol`s
//! - `folding_ranges` turns each heading section into a `FoldingRange`
//! - `document_links` resolves relative-path tokens into `file://` targets
//!
//! Hand-written (like scan.rs/quickfix.rs): testable pure functions the
//! generator does not model.

use lsp_types_max::{
    DocumentLink, DocumentSymbol, FoldingRange, FoldingRangeKind, Range, SymbolKind, Uri,
};

/// One parsed ATX heading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    /// Number of leading `#` characters (1..=6).
    pub level: usize,
    /// Heading text, prefix/trailing hashes and whitespace stripped.
    pub text: String,
    /// 0-based line index.
    pub line: u32,
    /// UTF-16 column of the first text character.
    pub text_col: u32,
    /// UTF-16 column one past the last text character.
    pub text_end_col: u32,
}

/// Parse a heading line: 1-6 leading `#`s, then a space/tab (or nothing),
/// then non-empty text. Returns `(level, text, text_col, text_end_col)`.
fn match_heading_line(line: &str) -> Option<(usize, String, u32, u32)> {
    let hashes = line.len() - line.trim_start_matches('#').len();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    let rest = &line[hashes..];
    if !rest.is_empty() && !rest.starts_with(' ') && !rest.starts_with('\t') {
        return None;
    }
    let text = rest.trim().trim_end_matches('#').trim();
    if text.is_empty() {
        return None;
    }
    let prefix_len = hashes + (line.len() - rest.len()) + (rest.len() - rest.trim_start().len());
    let text_col = line[..prefix_len].encode_utf16().count() as u32;
    let text_end_col = text_col + text.encode_utf16().count() as u32;
    Some((hashes, text.to_string(), text_col, text_end_col))
}

/// Iterate lines with fenced-code state: yields `(index, line, in_code)`
/// where `in_code` is true for fence delimiter lines and everything between
/// them. Headings and links inside fences are never surfaced.
pub fn fenced_lines(content: &str) -> Vec<(u32, &str, bool)> {
    let mut out = Vec::new();
    let mut in_fence = false;
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            out.push((i as u32, line, true));
        } else {
            out.push((i as u32, line, in_fence));
        }
    }
    out
}

/// Parse all ATX headings outside fenced code blocks.
pub fn parse_headings(content: &str) -> Vec<Heading> {
    let mut out = Vec::new();
    for (i, line, in_code) in fenced_lines(content) {
        if in_code {
            continue;
        }
        if let Some((level, text, text_col, text_end_col)) = match_heading_line(line) {
            out.push(Heading {
                level,
                text,
                line: i,
                text_col,
                text_end_col,
            });
        }
    }
    out
}

/// Build a symbol for one heading. `range` spans from the heading line through
/// the last line of its section; `selection_range` covers the heading text.
fn heading_symbol(
    heading: &Heading,
    section_end: u32,
    children: Vec<DocumentSymbol>,
) -> DocumentSymbol {
    DocumentSymbol {
        name: heading.text.clone(),
        detail: None,
        kind: SymbolKind::STRING,
        tags: None,
        #[allow(deprecated)]
        deprecated: None,
        range: Range::new(
            lsp_types_max::Position::new(heading.line, 0),
            lsp_types_max::Position::new(section_end, 0),
        ),
        selection_range: Range::new(
            lsp_types_max::Position::new(heading.line, heading.text_col),
            lsp_types_max::Position::new(heading.line, heading.text_end_col),
        ),
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
    }
}

/// Hierarchical `DocumentSymbol`s: a heading's section ends at the next
/// heading of the same or higher level (or EOF). Same-level headings are
/// siblings; deeper headings nest under the enclosing shallower one.
pub fn document_symbols(content: &str) -> Vec<DocumentSymbol> {
    let headings = parse_headings(content);
    if headings.is_empty() {
        return Vec::new();
    }
    let total_lines = content.lines().count() as u32;
    let last_line = if total_lines == 0 { 0 } else { total_lines - 1 };

    // Section end for heading i: line index of the next heading with
    // level <= this level, minus one (or the last line of the document).
    let mut ends = Vec::with_capacity(headings.len());
    for (i, h) in headings.iter().enumerate() {
        let end = headings[i + 1..]
            .iter()
            .find(|n| n.level <= h.level)
            .map(|n| n.line.saturating_sub(1))
            .unwrap_or(last_line);
        ends.push(end.max(h.line));
    }

    // Attach `sym` as the last child of `top`.
    fn attach(top: &mut DocumentSymbol, sym: DocumentSymbol) {
        match &mut top.children {
            Some(c) => c.push(sym),
            slot => *slot = Some(vec![sym]),
        }
    }

    // Standard outline stack: when a heading arrives, first close every
    // stacked heading at the same or deeper level — each popped symbol is
    // attached to its own parent (the new stack top, which is strictly
    // shallower), or to the root list when no shallower parent exists
    // (sibling H1s). Then push the new heading.
    let mut stack: Vec<(usize, DocumentSymbol)> = Vec::new();
    let mut roots: Vec<DocumentSymbol> = Vec::new();
    for (i, h) in headings.iter().enumerate() {
        while let Some(&(top_level, _)) = stack.last() {
            if top_level >= h.level {
                let (_, popped) = stack.pop().unwrap();
                match stack.last_mut() {
                    Some((_, parent)) => attach(parent, popped),
                    None => roots.push(popped),
                }
            } else {
                break;
            }
        }
        let sym = heading_symbol(h, ends[i], Vec::new());
        stack.push((h.level, sym));
    }
    // Final drain with the same parent rule: deeper remainders nest, equal
    // remainders are siblings at root.
    while let Some((level, popped)) = stack.pop() {
        match stack.last_mut() {
            Some((top_level, parent)) if level > *top_level => attach(parent, popped),
            _ => roots.push(popped),
        }
    }
    roots
}

/// One `FoldingRange` per heading section: folds from the heading line to the
/// last line before the next same-or-higher heading (or last content line).
pub fn folding_ranges(content: &str) -> Vec<FoldingRange> {
    let headings = parse_headings(content);
    let total_lines = content.lines().count() as u32;
    let last_line = if total_lines == 0 { 0 } else { total_lines - 1 };
    let mut out = Vec::new();
    for (i, h) in headings.iter().enumerate() {
        let end = headings[i + 1..]
            .iter()
            .find(|n| n.level <= h.level)
            .map(|n| n.line.saturating_sub(1))
            .unwrap_or(last_line);
        let end = end.max(h.line);
        if end == h.line {
            continue;
        }
        out.push(FoldingRange {
            start_line: h.line,
            start_character: None,
            end_line: end,
            end_character: None,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        });
    }
    out
}

/// A relative path reference found in a markdown doc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathRef {
    /// 0-based line index.
    pub line: u32,
    /// UTF-16 start column of the path token.
    pub start_col: u32,
    /// UTF-16 end column (exclusive).
    pub end_col: u32,
    /// The token text, e.g. `hooks/session-start.sh`.
    pub token: String,
}

/// Config-relevant extensions a path token must end with to count as a
/// relative path reference.
const PATH_EXTS: [&str; 8] = ["sh", "json", "md", "toml", "yml", "yaml", "py", "ts"];

/// True if `c` can appear inside a relative path token.
fn is_path_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '/' | '.' | '-' | '_' | '@')
}

/// Scan one line for path-like tokens ending in a config-relevant extension.
/// A token must contain a `/` (relative path into the tree, e.g.
/// `hooks/foo.sh`) or start with `./`. Bare filenames (`README.md`) are not
/// surfaced.
fn scan_line_paths(line: &str) -> Vec<(usize, usize, String)> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    for (idx, ch) in line.char_indices().chain([(line.len(), ' ')]) {
        if is_path_char(ch) {
            if start.is_none() {
                start = Some(idx);
            }
        } else if let Some(s) = start.take() {
            let token = &line[s..idx];
            if let Some(ext) = token.rsplit('.').next() {
                let has_ext = PATH_EXTS.iter().any(|e| ext.eq_ignore_ascii_case(e));
                if has_ext && token.contains('/') && !token.starts_with('/') {
                    out.push((s, idx, token.to_string()));
                }
            }
        }
    }
    let _ = bytes;
    out
}

/// Minimal percent-encoding for path segments inside a `file://` URI (space
/// and a few URI-hostile bytes). Keeps the common case byte-identical.
fn percent_encode_path(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for b in path.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'.' | b'-' | b'_' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Build a `file://` URI string for an absolute path.
fn file_uri(path: &str) -> String {
    format!("file://{}", percent_encode_path(path))
}

/// Find relative-path references and resolve them against `doc_dir` (the
/// directory containing the markdown document). Tokens inside fenced code
/// blocks are skipped. Links whose URI fails to parse are skipped.
pub fn document_links(content: &str, doc_dir: &str) -> Vec<DocumentLink> {
    let mut out = Vec::new();
    for (i, line, in_code) in fenced_lines(content) {
        if in_code {
            continue;
        }
        for (s, e, token) in scan_line_paths(line) {
            let start_col = line[..s].encode_utf16().count() as u32;
            let end_col = start_col + line[s..e].encode_utf16().count() as u32;
            let target = format!("{doc_dir}/{}", token);
            let target = if target.starts_with('/') {
                file_uri(&target)
            } else {
                file_uri(&format!("/{target}"))
            };
            let Ok(uri) = target.parse::<Uri>() else {
                continue;
            };
            out.push(DocumentLink {
                range: Range::new(
                    lsp_types_max::Position::new(i, start_col),
                    lsp_types_max::Position::new(i, end_col),
                ),
                target: Some(uri),
                tooltip: None,
                data: None,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = "\
# Root
intro text referencing hooks/session-start.sh here.
## Sub A
body A
### Deep
deep body
## Sub B
body B with docs/reference.md
```sh
# not a heading: hooks/inside-fence.sh
```
# Tail
tail body
";

    #[test]
    fn headings_parse_levels_and_text() {
        let hs = parse_headings(DOC);
        let lv: Vec<usize> = hs.iter().map(|h| h.level).collect();
        assert_eq!(lv, vec![1, 2, 3, 2, 1]);
        assert_eq!(hs[1].text, "Sub A");
        assert_eq!(hs[4].text, "Tail");
    }

    #[test]
    fn headings_skip_fenced_lines() {
        let hs = parse_headings(DOC);
        assert!(!hs.iter().any(|h| h.text.contains("not a heading")));
    }

    #[test]
    fn symbols_nest_children_under_parents() {
        let syms = document_symbols(DOC);
        assert_eq!(syms.len(), 2, "two top-level sections");
        let a = &syms[0];
        assert_eq!(a.name, "Root");
        let kids = a.children.as_ref().expect("Root must have children");
        assert_eq!(kids.len(), 2);
        assert_eq!(kids[0].name, "Sub A");
        let deep = kids[0].children.as_ref().expect("Sub A nests Deep");
        assert_eq!(deep[0].name, "Deep");
    }

    #[test]
    fn symbol_ranges_span_their_sections() {
        let syms = document_symbols(DOC);
        let a = &syms[0];
        // "Root" section: lines 0..=10 (everything before "# Tail" at line 11).
        assert_eq!(a.range.start.line, 0);
        assert_eq!(a.range.end.line, 10);
        assert_eq!(a.selection_range.start.line, 0);
        // "Tail" section runs to the last line (12).
        let tail = &syms[1];
        assert_eq!(tail.range.end.line, 12);
        assert!(tail.children.is_none());
    }

    #[test]
    fn folds_cover_every_section() {
        let folds = folding_ranges(DOC);
        assert_eq!(folds.len(), 5, "every heading whose section has >1 line");
        assert_eq!(folds[0].start_line, 0);
        assert_eq!(folds[0].end_line, 10);
        assert_eq!(folds.last().unwrap().end_line, 12);
    }

    #[test]
    fn links_resolve_relative_paths_against_doc_dir() {
        let links = document_links(DOC, "/proj");
        let targets: Vec<&str> = links
            .iter()
            .map(|l| l.target.as_ref().unwrap().as_str())
            .collect();
        assert_eq!(
            targets,
            vec![
                "file:///proj/hooks/session-start.sh",
                "file:///proj/docs/reference.md"
            ]
        );
        assert_eq!(links[0].range.start.line, 1);
    }

    #[test]
    fn links_skip_fences_and_absolute_and_bare_names() {
        let doc = "no links\n```json\nhooks/in/fence.json\n```\nsee /abs/path.md and README.md\n";
        let links = document_links(doc, "/proj");
        assert!(links.is_empty(), "got {links:?}");
    }

    #[test]
    fn empty_and_headingless_documents_are_noops() {
        assert!(parse_headings("").is_empty());
        assert_eq!(document_symbols("# only heading\n").len(), 1);
        assert!(folding_ranges("# only heading\n").is_empty());
        assert!(document_links("", "/x").is_empty());
    }
}
