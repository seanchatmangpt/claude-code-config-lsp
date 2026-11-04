//! selection — textDocument/selectionRange + textDocument/documentHighlight
//! for JSON config docs and markdown docs (CLAUDE.md / SKILL.md).
//!
//! Pure functions over document content, same tree-sitter JSON setup as
//! `hover.rs` / `rename.rs` / `completion.rs`. Positions are LSP (line,
//! UTF-16 character) throughout; byte offsets never leak across a function
//! boundary — every span is converted per the backend span convention
//! (`backend.rs::offset_to_position`, UTF-16 code units per line).
//!
//! JSON: the selection range chain is the ancestor walk of the innermost
//! node containing the position (string_content → string → pair → object →
//! array → document), each link converted to UTF-16. Highlight: same-word
//! occurrences across string_content leaves; a key occurrence is
//! `DocumentHighlightKind::WRITE`, a value occurrence is `READ`.
//!
//! Markdown: line-based ranges — word → line → heading section (heading line
//! through the line before the next heading, any level) → whole document.
//! Highlight: `[[wikilink]]` occurrences whose target matches the word at the
//! position are `READ` (references); the heading whose text matches the
//! target is the definition and is `WRITE`.

use lsp_types_max::{DocumentHighlight, DocumentHighlightKind, Position, Range, SelectionRange};
use tree_sitter::Node;

// ---------------------------------------------------------------------------
// Position <-> byte offset conversion (UTF-16, per the backend convention)
// ---------------------------------------------------------------------------

/// Byte offset of an LSP `Position` (line, UTF-16 character). Out-of-range
/// positions clamp to the document end, mirroring `offset_to_position`'s
/// clamp in backend.rs.
fn offset_of(content: &str, pos: Position) -> usize {
    let mut offset = 0usize;
    for (i, line) in content.split('\n').enumerate() {
        if (i as u32) < pos.line {
            offset += line.len() + 1;
            continue;
        }
        // Inside the target line: walk chars, consuming UTF-16 units.
        let mut units = 0u32;
        for ch in line.chars() {
            if units >= pos.character {
                break;
            }
            units += ch.len_utf16() as u32;
            offset += ch.len_utf8();
        }
        return offset.min(content.len());
    }
    content.len()
}

/// LSP `Position` of a byte offset (line, UTF-16 character) — the same
/// conversion as `backend.rs::offset_to_position`.
fn pos_of(content: &str, offset: usize) -> Position {
    let offset = offset.min(content.len());
    let mut line = 0u32;
    let mut last_line_start = 0usize;
    for (i, b) in content.as_bytes()[..offset].iter().enumerate() {
        if *b == b'\n' {
            line += 1;
            last_line_start = i + 1;
        }
    }
    let character = content[last_line_start..offset].encode_utf16().count() as u32;
    Position::new(line, character)
}

// ---------------------------------------------------------------------------
// tree-sitter JSON parse
// ---------------------------------------------------------------------------

fn parse(content: &str) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .ok()?;
    parser.parse(content, None)
}

// ---------------------------------------------------------------------------
// textDocument/selectionRange — JSON
// ---------------------------------------------------------------------------

/// Smallest named node containing byte `offset` (start <= offset < end, or
/// start == end for an empty node at the offset).
fn node_at_offset<'t>(_content: &str, root: Node<'t>, offset: usize) -> Option<Node<'t>> {
    let mut current = root.child(0)?;
    loop {
        if !current.is_named() {
            // Anonymous tokens (braces, quotes, colons) are not selection
            // links; skip to the covering named node via the parent chain,
            // which always exists because we only ever entered named children.
            let parent = current.parent()?;
            // find the named ancestor's child that covers the offset
            let mut cursor = parent.walk();
            let covering = parent
                .children(&mut cursor)
                .filter(|c| c.is_named())
                .find(|c| {
                    (c.start_byte() <= offset && offset < c.end_byte())
                        || (c.start_byte() == c.end_byte() && c.start_byte() == offset)
                });
            current = covering?;
            continue;
        }
        let mut cursor = current.walk();
        let covering = current
            .children(&mut cursor)
            .filter(|c| c.is_named())
            .find(|c| {
                (c.start_byte() <= offset && offset < c.end_byte())
                    || (c.start_byte() == c.end_byte() && c.start_byte() == offset)
            });
        match covering {
            Some(child) => current = child,
            None => return Some(current),
        }
    }
}

/// The innermost named node containing `pos` (UTF-16 → byte → tree walk).
fn node_at<'t>(content: &str, root: Node<'t>, pos: Position) -> Option<Node<'t>> {
    node_at_offset(content, root, offset_of(content, pos))
}

/// Collapse a byte span to a UTF-16 `Range` against `content`.
fn range_of(content: &str, start: usize, end: usize) -> Range {
    Range::new(pos_of(content, start), pos_of(content, end))
}

/// textDocument/selectionRange for a JSON document: the nested ancestor chain
/// of the innermost node at `pos` — innermost link is the node itself, each
/// `parent` widens to the enclosing named node until the grammar root. Nodes
/// that share a byte range with their child are skipped so every link
/// strictly widens. Returns `None` when the parse fails or `pos` selects
/// nothing.
pub fn selection_range_json(content: &str, pos: Position) -> Option<SelectionRange> {
    let tree = parse(content)?;
    let root = tree.root_node();
    let start = node_at(content, root, pos)?;
    let mut chain: Vec<(usize, usize)> = Vec::new();
    let mut current = Some(start);
    while let Some(node) = current {
        let span = (node.start_byte(), node.end_byte());
        if chain.last() != Some(&span) {
            chain.push(span);
        }
        current = node.parent();
    }
    // Build parent-linked chain innermost → outermost.
    build_chain(content, chain)
}

/// Build a parent-linked SelectionRange chain innermost → outermost from
/// (start, end) byte spans, innermost-first order in the input vec. Spans
/// equal to their neighbor (outer link in the input) are dropped so every
/// link strictly widens.
fn build_chain(content: &str, innermost_first: Vec<(usize, usize)>) -> Option<SelectionRange> {
    let mut deduped: Vec<(usize, usize)> = Vec::new();
    for span in innermost_first {
        if deduped.last() != Some(&span) {
            deduped.push(span);
        }
    }
    let mut acc: Option<SelectionRange> = None;
    for (s, e) in deduped.into_iter().rev() {
        let mut link = SelectionRange {
            range: range_of(content, s, e),
            parent: None,
        };
        link.parent = acc.map(Box::new);
        acc = Some(link);
    }
    acc
}

// ---------------------------------------------------------------------------
// textDocument/selectionRange — markdown (line-based)
// ---------------------------------------------------------------------------

/// Contiguous word span `[start, end)` (byte offsets) around `offset`:
/// alphanumeric + `_` + `-`, expanded in both directions.
fn word_span_at(content: &str, offset: usize) -> Option<(usize, usize)> {
    let bytes = content.as_bytes();
    let offset = offset.min(content.len());
    let is_word = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b'-';
    if offset >= content.len() || !is_word(bytes[offset]) {
        // Cursor on punctuation: try the character just before.
        let before = offset.checked_sub(1)?;
        if !is_word(bytes[before]) {
            return None;
        }
        return word_span_left_then_right(content, before);
    }
    let start = (0..offset)
        .rev()
        .take_while(|&i| is_word(bytes[i]))
        .last()
        .unwrap_or(offset); // .last() IS the start index (smallest word byte)
    let end = (offset..bytes.len())
        .find(|&i| !is_word(bytes[i]))
        .unwrap_or(bytes.len());
    Some((start, end))
}

/// Expand a word span left then right from a known word byte.
fn word_span_left_then_right(content: &str, at: usize) -> Option<(usize, usize)> {
    let bytes = content.as_bytes();
    let is_word = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b'-';
    let start = (0..at)
        .rev()
        .take_while(|&i| is_word(bytes[i]))
        .last()
        .unwrap_or(at);
    let end = (at..bytes.len())
        .find(|&i| !is_word(bytes[i]))
        .unwrap_or(bytes.len());
    if start == end {
        None
    } else {
        Some((start, end))
    }
}

/// Byte span of the line containing `offset` (excluding the newline).
fn line_span(content: &str, offset: usize) -> (usize, usize) {
    let start = content.as_bytes()[..offset.min(content.len())]
        .iter()
        .rposition(|&b| b == b'\n')
        .map_or(0, |i| i + 1);
    let end = content.as_bytes()[start..]
        .iter()
        .position(|&b| b == b'\n')
        .map_or(content.len(), |i| start + i);
    (start, end)
}

/// Is line `idx` (0-based) an ATX heading (a `#` run followed by space/tab)?
fn is_heading(lines: &[&str], idx: usize) -> bool {
    lines.get(idx).is_some_and(|l| {
        let trimmed = l.trim_start();
        let hash_run = trimmed.chars().take_while(|&c| c == '#').count();
        hash_run > 0
            && trimmed[hash_run..]
                .chars()
                .next()
                .is_some_and(|c| c == ' ' || c == '\t')
    })
}

/// Markdown selection ranges: word → line → section → whole document, all
/// line/UTF-16 based. Returns `None` only for an empty document.
pub fn selection_range_markdown(content: &str, pos: Position) -> Option<SelectionRange> {
    if content.is_empty() {
        return None;
    }
    let offset = offset_of(content, pos);
    let (line_start, line_end) = line_span(content, offset);
    let lines: Vec<&str> = content.split('\n').collect();
    let line_idx = content.as_bytes()[..offset]
        .iter()
        .filter(|&&b| b == b'\n')
        .count();
    let section = section_span(&lines, line_idx);
    // Chain: word (if any) → line → section → document.
    let word = word_span_at(content, offset);
    let doc = (0, content.len());
    // Innermost-first ordering (word, line, section, doc) — build_chain
    // treats element 0 as the innermost link.
    let mut chain: Vec<(usize, usize)> = Vec::new();
    if let Some(w) = word {
        chain.push(w);
    }
    chain.push((line_start, line_end));
    chain.push(section);
    chain.push(doc);
    build_chain(content, chain)
}

/// Heading-section byte span: from the start of the nearest heading at or
/// above `line_idx` through the line before the next heading (any level).
/// Above the first heading, the section is the leading preamble (doc start
/// through the line before the first heading).
fn section_span(lines: &[&str], line_idx: usize) -> (usize, usize) {
    let mut start_line = 0usize;
    for i in (0..=line_idx.min(lines.len().saturating_sub(1))).rev() {
        if is_heading(lines, i) {
            start_line = i;
            break;
        }
    }
    let mut end_line = lines.len();
    for (i, _) in lines.iter().enumerate().skip(start_line + 1) {
        if is_heading(lines, i) {
            end_line = i;
            break;
        }
    }
    // Byte offsets: start of start_line through end of the line before end_line.
    let mut start = 0usize;
    for line in &lines[..start_line] {
        start += line.len() + 1;
    }
    let mut end = start;
    for line in &lines[start_line..end_line.min(lines.len())] {
        end += line.len() + 1;
    }
    // End = start position of the next heading line (section boundary is the
    // heading line itself, exclusive). For the last section, end = doc end.
    (start, end)
}

// ---------------------------------------------------------------------------
// textDocument/documentHighlight — JSON
// ---------------------------------------------------------------------------

/// Inner text of the innermost named node at `pos`, when it is a word-ish
/// leaf (`string_content`, or a bare identifier/number token).
fn word_at_json<'a>(content: &'a str, root: Node<'_>, pos: Position) -> Option<&'a str> {
    let node = node_at(content, root, pos)?;
    let kind = node.kind();
    let text = &content[node.start_byte()..node.end_byte()];
    if kind == "string_content" || kind == "number" {
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    } else {
        None
    }
}

/// Is this `string_content` node a pair's KEY (vs a value)?
fn is_pair_key(node: Node<'_>) -> bool {
    let mut current = Some(node);
    while let Some(n) = current {
        if n.kind() == "pair" {
            let Some(key) = n.child_by_field_name("key") else {
                return false;
            };
            // The key chain: pair.key -> string -> string_content
            let mut cursor = key.walk();
            return key
                .children(&mut cursor)
                .any(|c| c.start_byte() == node.start_byte() && c.kind() == "string_content");
        }
        current = n.parent();
    }
    false
}

/// All `string_content` leaves in the tree, as (node, text).
fn string_contents<'a>(root: Node<'a>, content: &'a str) -> Vec<(Node<'a>, &'a str)> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if node.kind() == "string_content" {
            out.push((node, &content[node.start_byte()..node.end_byte()]));
        }
        let mut child_cursor = node.walk();
        let children: Vec<_> = node.children(&mut child_cursor).collect();
        for child in children.into_iter().rev() {
            stack.push(child);
        }
    }
    out
}

/// textDocument/documentHighlight for JSON: every `string_content` leaf whose
/// text equals the word at `pos`. The defining key occurrence (a pair key) is
/// `WRITE`; value occurrences are `READ`. When `pos` is on a pair key's word,
/// the key at `pos` is WRITE regardless; all other occurrences follow their
/// own key/value role.
pub fn highlight_json(content: &str, pos: Position) -> Vec<DocumentHighlight> {
    let Some(tree) = parse(content) else {
        return Vec::new();
    };
    let root = tree.root_node();
    let Some(word) = word_at_json(content, root, pos) else {
        return Vec::new();
    };
    string_contents(root, content)
        .into_iter()
        .filter(|(_, text)| *text == word)
        .map(|(node, _)| {
            let kind = if is_pair_key(node) {
                DocumentHighlightKind::WRITE
            } else {
                DocumentHighlightKind::READ
            };
            DocumentHighlight {
                range: range_of(content, node.start_byte(), node.end_byte()),
                kind: Some(kind),
            }
        })
        .collect()
}

/// textDocument/documentHighlight for markdown: all `[[target]]` occurrences
/// matching the word at `pos`, plus the defining heading (text equal to the
/// target, case-insensitive) which is `WRITE`. Wikilink occurrences are
/// `READ`.
pub fn highlight_markdown(content: &str, pos: Position) -> Vec<DocumentHighlight> {
    let offset = offset_of(content, pos);
    let word = word_span_at(content, offset).map(|(s, e)| content[s..e].to_string());
    let Some(word) = word else {
        return Vec::new();
    };
    let lower = word.to_ascii_lowercase();
    let mut out: Vec<DocumentHighlight> = Vec::new();
    // Wikilink occurrences: [[target]] — target text gets READ.
    let bytes = content.as_bytes();
    let mut i = 0usize;
    while i + 1 < bytes.len() {
        if bytes[i] == b'[' && bytes[i + 1] == b'[' {
            if let Some(close) = content[i + 2..].find("]]") {
                let target_start = i + 2;
                let target_end = i + 2 + close;
                let target = &content[target_start..target_end];
                if target.eq_ignore_ascii_case(&lower) {
                    out.push(DocumentHighlight {
                        range: range_of(content, target_start, target_end),
                        kind: Some(DocumentHighlightKind::READ),
                    });
                }
                i = target_end + 2;
                continue;
            }
        }
        i += 1;
    }
    // Defining heading: text equal to the target, case-insensitive → WRITE.
    let lines: Vec<&str> = content.split('\n').collect();
    for (idx, line) in lines.iter().enumerate() {
        if is_heading(&lines, idx)
            && line
                .trim_start()
                .trim_start_matches('#')
                .trim()
                .eq_ignore_ascii_case(&lower)
        {
            let start: usize = lines[..idx].iter().map(|l| l.len() + 1).sum();
            out.push(DocumentHighlight {
                range: range_of(content, start, start + line.len()),
                kind: Some(DocumentHighlightKind::WRITE),
            });
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Dispatchers
// ---------------------------------------------------------------------------

/// selectionRange for either doc class.
pub fn selection_range(content: &str, pos: Position, markdown: bool) -> Option<SelectionRange> {
    if markdown {
        selection_range_markdown(content, pos)
    } else {
        selection_range_json(content, pos)
    }
}

/// documentHighlight for either doc class.
pub fn highlight(content: &str, pos: Position, markdown: bool) -> Vec<DocumentHighlight> {
    if markdown {
        highlight_markdown(content, pos)
    } else {
        highlight_json(content, pos)
    }
}
