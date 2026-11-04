//! Call hierarchy adapted to config semantics: a SKILL.md / agent name is a
//! callable unit; "callers" are the config files that reference it.
//!
//! - `prepare_call_hierarchy` on a SKILL.md or `agents/*.md` frontmatter
//!   name produces a `CallHierarchyItem` for the skill/agent.
//! - `incoming_calls` finds files referencing that skill/agent: frontmatter
//!   value matches, `[[wikilink]]` stems, and `enabledPlugins` entries.
//! - `outgoing_calls` from a CLAUDE.md/SKILL.md = the `[[wikilinks]]` and
//!   path-like references it contains, resolved against the corpus.
//!
//! Everything is computed over a corpus index (`Vec<(PathBuf, String)>` of
//! name→text pairs) so it is testable without any filesystem walking; the
//! real fixture tree is the Chicago-test collaborator.

use crate::navigation::find_wikilinks;
use lsp_types_max::{
    CallHierarchyIncomingCall, CallHierarchyItem, CallHierarchyOutgoingCall, Position, Range,
    SymbolKind, Uri,
};
use std::path::{Path, PathBuf};

/// Corpus index: (path, text) pairs. No fs walking inside these functions.
pub type Corpus = Vec<(PathBuf, String)>;

/// `start <= position <= end` over (line, utf16-character) coordinates.
fn range_contains(range: &Range, position: Position) -> bool {
    (range.start.line, range.start.character) <= (position.line, position.character)
        && (position.line, position.character) <= (range.end.line, range.end.character)
}

/// UTF-16 column width of a &str.
fn width_utf16(s: &str) -> u32 {
    s.chars().map(char::len_utf16).sum::<usize>() as u32
}

/// Parse a `---` frontmatter block and return (name value, range of the
/// value token) for the `name:` field, honoring quotes.
pub fn frontmatter_name(text: &str) -> Option<(String, Range)> {
    let mut lines = text.lines().enumerate();
    if lines.next()?.1.trim_end() != "---" {
        return None;
    }
    // (was: `for ... { ... }` whose body provably exits on the first
    // iteration — clippy::never_loop; hoisted to first-iteration handling,
    // behavior-identical)
    let Some((idx, line)) = lines.next() else {
        return None; // empty frontmatter block; name not found
    };
    {
        let trimmed = line.trim_end();
        if trimmed == "---" {
            return None; // end of frontmatter; name not found
        }
        let after = trimmed.trim().strip_prefix("name:")?;
        let value = after.trim();
        if value.is_empty() {
            return None;
        }
        let lead = after.len() - after.trim_start().len();
        let (indent, trimmed_line) = (trimmed.len() - trimmed.trim_start().len(), trimmed.trim());
        let start_col = indent as u32 + 5u32 + 1u32 + lead as u32; // "name:" + space
        let _ = (after, lead, indent, trimmed_line, start_col);
        let start_col = match trimmed_line.find(value) {
            Some(c) => c as u32,
            None => 0,
        };
        let end_col = start_col + width_utf16(value);
        let unquoted = value
            .trim_start_matches('"')
            .trim_end_matches('"')
            .trim_start_matches('\'')
            .trim_end_matches('\'');
        Some((
            unquoted.to_string(),
            Range {
                start: Position {
                    line: idx as u32,
                    character: start_col,
                },
                end: Position {
                    line: idx as u32,
                    character: end_col,
                },
            },
        ))
    }
}

/// The `CallHierarchyItem` for the skill/agent defined at `doc_path`, when
/// `position` sits on its frontmatter `name` value.
pub fn prepare_call_hierarchy(
    corpus: &Corpus,
    doc_path: &Path,
    position: Position,
) -> Option<CallHierarchyItem> {
    let text = corpus.iter().find(|(p, _)| p == doc_path)?.1.clone();
    let (name, value_range) = frontmatter_name(&text)?;
    if !range_contains(&value_range, position) {
        return None;
    }
    make_item(doc_path, &name, value_range)
}

/// Build a `CallHierarchyItem` for a config document: name = frontmatter
/// name when present, else the file stem; kind = FUNCTION; detail = the
/// path relative to the corpus.
fn make_item(path: &Path, name: &str, selection: Range) -> Option<CallHierarchyItem> {
    let range = selection;

    Some(CallHierarchyItem {
        name: name.to_string(),
        kind: SymbolKind::FUNCTION,
        tags: None,
        detail: Some(path.display().to_string()),
        uri: format!("file://{}", path.display()).parse::<Uri>().ok()?,
        range,
        selection_range: selection,
        data: None,
    })
}

/// One incoming or outgoing call edge, ready for the LSP response types.
#[derive(Debug, Clone, PartialEq)]
pub struct CallEdge {
    pub item: CallHierarchyItem,
    pub from_ranges: Vec<Range>,
}

impl CallEdge {
    fn into_incoming(self) -> CallHierarchyIncomingCall {
        CallHierarchyIncomingCall {
            from: self.item,
            from_ranges: self.from_ranges,
        }
    }
    fn into_outgoing(self) -> CallHierarchyOutgoingCall {
        CallHierarchyOutgoingCall {
            to: self.item,
            from_ranges: self.from_ranges,
        }
    }
}

/// Incoming references to `name` across the corpus. Channels:
/// (1) frontmatter `key: name` value matches in other docs' frontmatter,
/// (2) `[[wikilink]]` stems equal to the name,
/// (3) `enabledPlugins` keys `"plugin@marketplace": ...` in settings.json
///     whose `plugin` segment equals the name.
/// The defining document itself is excluded.
pub fn incoming_calls(corpus: &Corpus, name: &str) -> Vec<CallEdge> {
    let mut edges: Vec<CallEdge> = Vec::new();
    for (path, text) in corpus {
        if frontmatter_name(text)
            .map(|(n, _)| n == name)
            .unwrap_or(false)
        {
            continue; // the definition itself
        }
        let mut ranges = find_refs(text, name);
        if ranges.is_empty() {
            continue;
        }
        ranges.sort_by_key(|r| (r.start.line, r.start.character));
        // One item per referencing document, one from_range per reference.
        let item = match frontmatter_name(text) {
            Some((n, sel)) => make_item(path, &n, sel),
            None => {
                let stem = path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                make_item(path, &stem, crate::navigation::h1_range(text))
            }
        };
        if let Some(item) = item {
            edges.push(CallEdge {
                item,
                from_ranges: ranges,
            });
        }
    }
    edges
}

/// Ranges of all references to `name` in `text`, across the channels:
/// frontmatter `key: name` values, `[[name]]` wikilinks, and settings.json
/// `"name@marketplace"` enabledPlugins keys.
pub fn find_refs(text: &str, name: &str) -> Vec<Range> {
    let mut out: Vec<Range> = Vec::new();
    // Channel 2: wikilinks.
    for link in find_wikilinks(text) {
        if link.stem == name {
            out.push(link.range);
        }
    }
    // Channels 1 and 3, line-oriented.
    let mut in_frontmatter = false;
    let mut seen_first = false;
    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed == "---" {
            if !seen_first {
                seen_first = true;
                in_frontmatter = true;
                continue;
            }
            in_frontmatter = false;
            continue;
        }
        if in_frontmatter {
            // Channel 1: any frontmatter `key: name` value match.
            if let Some((k, v)) = trimmed.split_once(':') {
                if k.trim() != "name" {
                    let value = v.trim();
                    let unquoted = value
                        .trim_start_matches('"')
                        .trim_end_matches('"')
                        .trim_start_matches('\'')
                        .trim_end_matches('\'');
                    if unquoted == name {
                        if let Some(off) = trimmed.find(value) {
                            out.push(Range {
                                start: Position {
                                    line: idx as u32,
                                    character: off as u32,
                                },
                                end: Position {
                                    line: idx as u32,
                                    character: off as u32 + width_utf16(value),
                                },
                            });
                        }
                    }
                }
            }
            continue;
        }
        // Channel 3: `"plugin@marketplace":` settings keys.
        let indent = (line.len() - line.trim_start().len()) as u32;
        let t = line.trim_start();
        if let Some(rest) = t.strip_prefix('"') {
            if let Some((key, tail)) = rest.split_once('"') {
                if tail.starts_with(':') {
                    if let Some((plugin, _mkt)) = key.split_once('@') {
                        if plugin == name {
                            let start = indent + 1u32; // past opening quote
                            out.push(Range {
                                start: Position {
                                    line: idx as u32,
                                    character: start,
                                },
                                end: Position {
                                    line: idx as u32,
                                    character: start + width_utf16(name),
                                },
                            });
                        }
                    }
                }
            }
        }
    }
    out
}

/// Incoming calls for the skill/agent named `name`: one edge per
/// referencing document, `from_ranges` sorted by position.
pub fn incoming_calls_typed(corpus: &Corpus, name: &str) -> Vec<CallHierarchyIncomingCall> {
    incoming_calls(corpus, name)
        .into_iter()
        .map(CallEdge::into_incoming)
        .collect()
}

// ---------------------------------------------------------------------------
// Outgoing calls
// ---------------------------------------------------------------------------

/// Outgoing calls from the document at `doc_path`: its `[[wikilinks]]` and
/// path-like references (`skills/foo/SKILL.md`, `agents/bar.md`), each
/// resolved against the corpus by frontmatter name or file stem.
pub fn outgoing_calls(corpus: &Corpus, doc_path: &Path) -> Vec<CallEdge> {
    let text = match corpus.iter().find(|(p, _)| p == doc_path) {
        Some((_, t)) => t.clone(),
        None => return Vec::new(),
    };
    let mut stems: Vec<(String, Range)> = Vec::new();
    for link in find_wikilinks(&text) {
        stems.push((link.stem.clone(), link.range));
    }
    // Path-like references inside backticks or plain text.
    for (idx, line) in text.lines().enumerate() {
        for token in line.split([' ', '`', '(', ')']) {
            let tok = token.trim_matches(|c| c == ',' || c == '.' || c == '"');
            if !tok.contains('/') {
                continue;
            }
            // Keep the full token: slash-ful paths resolve by path suffix,
            // bare stems by stem/frontmatter name (see resolve_stem).
            let stem = tok.to_string();
            {
                if let Some(off) = line.find(tok) {
                    stems.push((
                        stem,
                        Range {
                            start: Position {
                                line: idx as u32,
                                character: off as u32,
                            },
                            end: Position {
                                line: idx as u32,
                                character: off as u32 + width_utf16(tok),
                            },
                        },
                    ));
                }
            }
        }
    }
    // Resolve each stem to a corpus doc; merge ranges per target.
    let mut edges: Vec<CallEdge> = Vec::new();
    let mut order: Vec<PathBuf> = Vec::new();
    let mut by_path: std::collections::HashMap<PathBuf, Vec<Range>> =
        std::collections::HashMap::new();
    for (stem, range) in stems {
        if let Some(target) = resolve_stem(corpus, &stem) {
            let ranges = by_path.entry(target.clone()).or_insert_with(|| {
                order.push(target.clone());
                Vec::new()
            });
            ranges.push(range);
        }
    }
    for path in order {
        let ranges = by_path.remove(&path).unwrap_or_default();
        let mut ranges = ranges;
        ranges.sort_by_key(|r| (r.start.line, r.start.character));
        ranges.dedup();
        let item = match corpus.iter().find(|(p, _)| p == &path) {
            Some((_, t)) => match frontmatter_name(t) {
                Some((n, sel)) => make_item(&path, &n, sel),
                None => make_item(
                    &path,
                    stem_of(&path).as_str(),
                    crate::navigation::h1_range(t),
                ),
            },
            None => continue,
        };
        if let Some(item) = item {
            edges.push(CallEdge {
                item,
                from_ranges: ranges,
            });
        }
    }
    edges
}

/// File stem of a path, as String ("" when none).
fn stem_of(path: &Path) -> String {
    path.file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Resolve a reference to a corpus doc. Slash-ful tokens (e.g.
/// `skills/ggen/SKILL.md`) must suffix-match a corpus path exactly —
/// never by bare file stem, which would collide across every `SKILL.md`.
/// Bare stems match file stem or frontmatter name; first match wins.
fn resolve_stem(corpus: &Corpus, token: &str) -> Option<PathBuf> {
    let token = token.strip_prefix("~/").unwrap_or(token);
    if token.contains('/') {
        return corpus
            .iter()
            .map(|(p, _)| p.display().to_string())
            .find(|p| p.ends_with(token))
            .map(PathBuf::from);
    }
    for (path, text) in corpus {
        if path
            .file_stem()
            .map(|s| s.to_string_lossy() == *token)
            .unwrap_or(false)
        {
            return Some(path.clone());
        }
        if frontmatter_name(text)
            .map(|(n, _)| n == token)
            .unwrap_or(false)
        {
            return Some(path.clone());
        }
    }
    None
}

/// Outgoing calls as LSP response types.
pub fn outgoing_calls_typed(corpus: &Corpus, doc_path: &Path) -> Vec<CallHierarchyOutgoingCall> {
    outgoing_calls(corpus, doc_path)
        .into_iter()
        .map(CallEdge::into_outgoing)
        .collect()
}
