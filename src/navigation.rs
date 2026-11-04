//! Navigation for the Claude-config wikilink grammar.
//!
//! Pure functions over `(text, position, uri)` — no backend state, no
//! tree-sitter dependency. Resolves:
//!
//! 1. `[[stem]]` wikilinks (including composite `[[a]]/[[b]]` and
//!    parenthetical annotations like `[[stem]] (SPR)`) to files named
//!    `stem.md` under the corpus roots (`~/.claude/rules/`,
//!    `~/.claude/skills/`, project `.claude/`).
//! 2. Hook path references ("hooks/foo.sh"-style relative paths in
//!    settings/CLAUDE.md) to file locations.
//!
//! Tolerance law: unresolvable links return `None` / are skipped — never
//! panic, never crash the server.

use lsp_types_max::{Location, Position, Range, Uri};
use std::path::{Path, PathBuf};

/// A parsed `[[stem]]` occurrence in a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wikilink {
    /// Link text between `[[` and `]]`.
    pub stem: String,
    /// Range of the stem text (excluding the brackets), UTF-16 LSP coords.
    pub range: Range,
}

// ---------------------------------------------------------------------------
// Position <-> offset helpers (UTF-16 LSP positions over UTF-8 text)
// ---------------------------------------------------------------------------

/// Convert an LSP position to a byte offset; clamps to a char boundary.
/// `None` when the position is past the end of the document.
pub fn position_to_offset(text: &str, position: Position) -> Option<usize> {
    let mut line_start = 0usize;
    let mut line_idx = 0u32;
    let mut bytes = text.as_bytes();

    while line_idx < position.line {
        {
            let nl = bytes.iter().position(|&b| b == b'\n')?;
            line_start += nl + 1;
            bytes = &bytes[nl + 1..];
            line_idx += 1;
        }
    }

    let line_end = bytes
        .iter()
        .position(|&b| b == b'\n')
        .map(|p| line_start + p)
        .unwrap_or(text.len());

    // Walk the line as chars, counting UTF-16 code units.
    let line = text.get(line_start..line_end)?;
    let mut units = 0u32;
    for (byte_idx, ch) in line.char_indices() {
        if units >= position.character {
            return Some(line_start + byte_idx);
        }
        units += ch.len_utf16() as u32;
    }
    Some(line_end)
}

/// Convert a byte offset to an LSP position.
pub fn offset_to_position(text: &str, offset: usize) -> Position {
    let offset = offset.min(text.len());
    let offset = floor_to_char_boundary(text, offset);
    let head = &text[..offset];
    let line = head.matches('\n').count() as u32;
    let line_start = head.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let units: usize = text[line_start..offset].chars().map(char::len_utf16).sum();
    Position {
        line,
        character: units as u32,
    }
}

fn floor_to_char_boundary(text: &str, mut i: usize) -> usize {
    while i > 0 && !text.is_char_boundary(i) {
        i -= 1;
    }
    i
}

// ---------------------------------------------------------------------------
// Wikilink scanning
// ---------------------------------------------------------------------------

/// Find every `[[stem]]` occurrence in `text`.
pub fn find_wikilinks(text: &str) -> Vec<Wikilink> {
    let bytes = text.as_bytes();
    let mut links = Vec::new();
    let mut i = 0usize;
    while i + 3 < bytes.len() {
        if bytes[i] == b'[' && bytes[i + 1] == b'[' {
            if let Some(close) = find_close(text, i + 2) {
                let stem = &text[i + 2..close];
                let stem = stem.trim();
                if !stem.is_empty() && !stem.contains(['[', ']']) {
                    let stem_start = i + 2;
                    let stem_end = close;
                    links.push(Wikilink {
                        stem: stem.to_string(),
                        range: Range {
                            start: offset_to_position(text, stem_start),
                            end: offset_to_position(text, stem_end),
                        },
                    });
                    i = close + 2;
                    continue;
                }
            }
        }
        i += 1;
    }
    links
}

fn find_close(text: &str, from: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut j = from;
    while j + 1 < bytes.len() {
        if bytes[j] == b']' && bytes[j + 1] == b']' {
            return Some(j);
        }
        j += 1;
    }
    None
}

/// The wikilink whose stem range contains `position`, if any.
pub fn wikilink_at_position(text: &str, position: Position) -> Option<Wikilink> {
    let offset = position_to_offset(text, position)?;
    for link in find_wikilinks(text) {
        let start = position_to_offset(text, link.range.start)?;
        let end = position_to_offset(text, link.range.end)?;
        if offset >= start && offset <= end {
            return Some(link);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Corpus resolution
// ---------------------------------------------------------------------------

/// Default corpus roots: `~/.claude/rules/`, `~/.claude/skills/`.
pub fn default_corpus_roots() -> Vec<PathBuf> {
    let home = std::env::var("HOME").unwrap_or_default();
    let home = PathBuf::from(home);
    vec![home.join(".claude/rules"), home.join(".claude/skills")]
}

/// Resolve a stem to a file named `stem.md` under one of `roots`
/// (recursive, first match wins in root order). Returns the canonical-ish
/// absolute path, or `None` when unresolvable.
pub fn resolve_stem(stem: &str, roots: &[PathBuf]) -> Option<PathBuf> {
    let want = format!("{stem}.md");
    for root in roots {
        if !root.is_dir() {
            continue;
        }
        if let Some(hit) = walk_for(root, &want, 0) {
            return Some(hit);
        }
    }
    None
}

fn walk_for(dir: &Path, want: &str, depth: usize) -> Option<PathBuf> {
    if depth > 8 {
        return None;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return None, // unreadable dir: tolerance, not crash
    };
    // First pass: direct children named want.
    let mut subdirs = Vec::new();
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_file() && p.file_name().and_then(|n| n.to_str()) == Some(want) {
            return Some(p);
        }
        if p.is_dir() {
            subdirs.push(p);
        }
    }
    for sub in subdirs {
        if let Some(hit) = walk_for(&sub, want, depth + 1) {
            return Some(hit);
        }
    }
    None
}

/// Range of the H1 heading (`# ...`) in `text`, or the whole first line
/// when there is no H1.
pub fn h1_range(text: &str) -> Range {
    for (idx, line) in text.lines().enumerate() {
        if line.starts_with("# ") {
            return Range {
                start: Position {
                    line: idx as u32,
                    character: 0,
                },
                end: Position {
                    line: idx as u32,
                    character: line.chars().map(char::len_utf16).sum::<usize>() as u32,
                },
            };
        }
    }
    Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 0,
            character: 0,
        },
    }
}

fn file_uri(path: &Path) -> Option<Uri> {
    format!("file://{}", path.display()).parse::<Uri>().ok()
}

/// Goto-definition for a `[[stem]]` wikilink: resolve the stem against
/// `roots` and return a `Location` pointing at the target's H1.
/// `None` when the cursor is not on a link or the stem is unresolvable.
pub fn wikilink_definition(
    _uri: &str,
    text: &str,
    position: Position,
    roots: &[PathBuf],
) -> Option<Location> {
    let link = wikilink_at_position(text, position)?;
    let path = resolve_stem(&link.stem, roots)?;
    let target_text = std::fs::read_to_string(&path).ok()?;
    Some(Location {
        uri: file_uri(&path)?,
        range: h1_range(&target_text),
    })
}

/// Reverse map: for a target file, find every `[[stem]]` mention across the
/// open document set whose stem resolves to that target. Returns a Location
/// per mention (document URI + stem range).
pub fn wikilink_references(
    docs: &[(String, String)], // (uri, text)
    target: &Path,
    roots: &[PathBuf],
) -> Vec<Location> {
    let mut out = Vec::new();
    for (uri, text) in docs {
        let Ok(doc_uri) = uri.parse::<Uri>() else {
            continue; // unparseable URI: skip, never crash
        };
        for link in find_wikilinks(text) {
            if let Some(resolved) = resolve_stem(&link.stem, roots) {
                if resolved == target {
                    out.push(Location {
                        uri: doc_uri.clone(),
                        range: link.range,
                    });
                }
            }
        }
    }
    out
}

/// Document highlight: all ranges in `text` whose wikilink stem resolves to
/// the same target as the link under `position`.
pub fn wikilink_highlight(
    _uri: &str,
    text: &str,
    position: Position,
    roots: &[PathBuf],
) -> Vec<Location> {
    let Some(link) = wikilink_at_position(text, position) else {
        return Vec::new();
    };
    let Some(target) = resolve_stem(&link.stem, roots) else {
        return Vec::new();
    };
    let Some(uri) = file_uri(target.as_path()) else {
        return Vec::new();
    };
    find_wikilinks(text)
        .into_iter()
        .filter(|l| {
            resolve_stem(&l.stem, roots)
                .map(|r| r == target)
                .unwrap_or(false)
        })
        .map(|l| Location {
            uri: uri.clone(),
            range: l.range,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Hook path references ("hooks/foo.sh"-style)
// ---------------------------------------------------------------------------

/// Script extensions recognized as hook path references.
const SCRIPT_EXTS: &[&str] = &["sh", "py", "js", "ts", "rb", "zsh", "bash"];

/// Find hook-path-like tokens: relative paths containing `/`, ending in a
/// script extension. Skips absolute (`/…`) and tilde (`~/…`) forms only for
/// the *reference scan* — `hook_path_definition` still resolves those.
pub fn find_hook_path_refs(text: &str) -> Vec<(String, Range)> {
    let mut out = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut byte_map = Vec::with_capacity(chars.len() + 1);
    let mut b = 0usize;
    for ch in &chars {
        byte_map.push(b);
        b += ch.len_utf8();
    }
    byte_map.push(b);

    let mut i = 0usize;
    while i < chars.len() {
        let is_token_char =
            |c: char| c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '/';
        if is_token_char(chars[i]) && (i == 0 || !is_token_char(chars[i - 1])) {
            // Token start; consume the token.
            let start = i;
            let mut end = i;
            while end < chars.len() && is_token_char(chars[end]) {
                end += 1;
            }
            let token: String = chars[start..end].iter().collect();
            let looks_relative = !token.starts_with('/') && !token.starts_with("~/");
            if looks_relative
                && token.contains('/')
                && SCRIPT_EXTS
                    .iter()
                    .any(|e| token.ends_with(&format!(".{e}")))
            {
                out.push((
                    token,
                    Range {
                        start: offset_to_position(text, byte_map[start]),
                        end: offset_to_position(text, byte_map[end]),
                    },
                ));
            }
            i = end;
        } else {
            i += 1;
        }
    }
    out
}

/// Goto-definition for a hook path reference: resolve relative to
/// `base_dir` (the doc's directory); tilde/absolute paths resolve as-is.
/// Returns `None` when the cursor is not on a path token or the file does
/// not exist on disk.
pub fn hook_path_definition(
    _uri: &str,
    text: &str,
    position: Position,
    base_dir: &Path,
) -> Option<Location> {
    let offset = position_to_offset(text, position)?;
    for (token, range) in find_hook_path_refs(text) {
        let start = position_to_offset(text, range.start)?;
        let end = position_to_offset(text, range.end)?;
        if offset < start || offset > end {
            continue;
        }
        let expanded = expand_tilde(&token);
        let path = if Path::new(&expanded).is_absolute() {
            PathBuf::from(&expanded)
        } else {
            base_dir.join(&token)
        };
        if path.is_file() {
            let target_text = std::fs::read_to_string(&path).unwrap_or_default();
            return Some(Location {
                uri: file_uri(&path)?,
                range: h1_range(&target_text),
            });
        }
        return None;
    }
    None
}

fn expand_tilde(token: &str) -> String {
    if let Some(rest) = token.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{home}/{rest}");
        }
    }
    token.to_string()
}
