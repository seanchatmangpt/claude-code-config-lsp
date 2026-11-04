//! Harness operations — the 9-operation LSP surface, given real config
//! semantics so every operation the Claude Code LSP tool exposes is
//! meaningful on Claude Code config files.
//!
//! Entry point: [`harness_operation`] — takes the agent-facing operation
//! name, a 1-based line/character, the current document text, and a corpus
//! index, performs the 1-based -> 0-based conversion internally, and
//! returns a typed [`OperationResult`].
//!
//! ## Operations-coverage matrix
//!
//! | op | config semantics | backing module | status |
//! |---|---|---|---|
//! | goToDefinition | wikilink [[stem]] -> target H1; hook/script path token -> file on disk; JSON key -> ontology property in schema/claude-code-config.ttl | navigation.rs, schema.rs, completion.rs | ALIVE |
//! | goToImplementation | hook command value / path token -> resolved script on disk; inline shell -> binary via $PATH; missing target -> typed NotOnDisk | navigation.rs, completion.rs | ALIVE |
//! | findReferences | skill/agent name under cursor (or the doc frontmatter name) -> corpus references | hierarchy.rs incoming_calls | ALIVE |
//! | hover | hovered JSON key -> keyed schema entry; surface dump fallback | hover.rs, completion.rs, schema.rs | ALIVE |
//! | documentSymbol | markdown -> heading outline; JSON -> key tree as typed symbols | analyzers/markdown_nav.rs; local JSON key-tree builder | ALIVE |
//! | workspaceSymbol | corpus-root scan for config symbols; query = word under cursor (empty = all) | workspace_symbols.rs | ALIVE |
//! | prepareCallHierarchy | frontmatter name of a SKILL.md/agent doc -> callable unit (position clamped to the name range) | hierarchy.rs | ALIVE |
//! | incomingCalls | callers of this doc skill/agent name across the corpus | hierarchy.rs | ALIVE |
//! | outgoingCalls | this doc wikilinks + path-like references, resolved against the corpus | hierarchy.rs | ALIVE |
//!
//! Typed misses: OperationResult::NotOnDisk (the reference names a concrete
//! target that does not exist on disk / $PATH) and OperationResult::Unresolved
//! (cursor on nothing with config semantics, or unknown op name — typed
//! refusal, never a silent empty).

use crate::completion::{position_to_offset, resolve_cursor_context, CursorContext};
use crate::hierarchy::{self, Corpus};
use crate::navigation;
use crate::schema::build_schema;
use lsp_types_max::{
    CallHierarchyIncomingCall, CallHierarchyItem, CallHierarchyOutgoingCall, DocumentSymbol, Hover,
    Location, Position, Range, SymbolInformation, SymbolKind, Uri,
};
use std::path::{Path, PathBuf};

/// The exact operation names the Claude Code LSP tool exposes.
pub const OPERATIONS: [&str; 9] = [
    "goToDefinition",
    "findReferences",
    "hover",
    "documentSymbol",
    "workspaceSymbol",
    "goToImplementation",
    "prepareCallHierarchy",
    "incomingCalls",
    "outgoingCalls",
];

/// Typed result of a harness operation. One variant per LSP response shape
/// the 9 operations produce, plus typed misses.
#[derive(Debug, Clone)]
pub enum OperationResult {
    /// Single-target navigation (goToDefinition / goToImplementation hit).
    Definition(Location),
    /// goToDefinition when more than one channel resolves (kept for
    /// multi-channel hits; current dispatch returns Definition).
    Definitions(Vec<Location>),
    /// findReferences.
    References(Vec<Location>),
    /// hover.
    Hover(Option<Hover>),
    /// documentSymbol.
    DocumentSymbols(Vec<DocumentSymbol>),
    /// workspaceSymbol.
    WorkspaceSymbols(Vec<SymbolInformation>),
    /// prepareCallHierarchy.
    CallHierarchyItem(Option<CallHierarchyItem>),
    /// incomingCalls.
    IncomingCalls(Vec<CallHierarchyIncomingCall>),
    /// outgoingCalls.
    OutgoingCalls(Vec<CallHierarchyOutgoingCall>),
    /// A reference resolved to a concrete target that does not exist on
    /// disk (or on $PATH). reason values: "not-on-disk", "not-on-path".
    NotOnDisk { path: String, reason: &'static str },
    /// The cursor/op has no config semantics at this position. reason is a
    /// stable machine-readable token.
    Unresolved {
        reason: &'static str,
        detail: String,
    },
}

/// The single entry point: dispatch `op` over the document at 1-based
/// (line, character). The 1-based -> 0-based conversion happens here.
pub fn harness_operation(
    op: &str,
    uri: &str,
    line_1based: u32,
    char_1based: u32,
    doc_text: &str,
    corpus_index: &Corpus,
) -> OperationResult {
    if !OPERATIONS.contains(&op) {
        return OperationResult::Unresolved {
            reason: "unknown-operation",
            detail: format!("unknown harness operation: {op}"),
        };
    }
    // 1-based -> 0-based; saturating so out-of-range input degrades to a
    // typed miss downstream, never an underflow panic.
    let position = Position {
        line: line_1based.saturating_sub(1),
        character: char_1based.saturating_sub(1),
    };
    let corpus = corpus_with_current_doc(corpus_index, uri, doc_text);
    match op {
        "goToDefinition" => definition(uri, doc_text, position, &corpus),
        "goToImplementation" => implementation(uri, doc_text, position, &corpus),
        "findReferences" => references(uri, doc_text, position, &corpus),
        "hover" => OperationResult::Hover(hover(uri, doc_text, position)),
        "documentSymbol" => OperationResult::DocumentSymbols(document_symbols(uri, doc_text)),
        "workspaceSymbol" => workspace_symbols(&corpus, doc_text, position),
        "prepareCallHierarchy" => prepare_call_hierarchy(&corpus, uri, position),
        "incomingCalls" => incoming_calls(&corpus, uri, doc_text, position),
        "outgoingCalls" => outgoing_calls(&corpus, uri),
        _ => unreachable!("OPERATIONS gate above"),
    }
}

/// The corpus with the current document present under its own path
/// (hierarchy functions require the doc itself to be indexed). When the
/// caller's corpus already contains the path, its text is replaced with the
/// in-editor `doc_text` so dirty buffers behave like real editors.
fn corpus_with_current_doc(corpus_index: &Corpus, uri: &str, doc_text: &str) -> Corpus {
    let path = uri_to_path(uri);
    let mut corpus: Corpus = corpus_index
        .iter()
        .filter(|(p, _)| p != &path)
        .cloned()
        .collect();
    corpus.push((path, doc_text.to_string()));
    corpus
}

/// `file://` URI -> filesystem path (best effort; non-file URIs pass
/// through as a relative-ish path so they still index by suffix).
fn uri_to_path(uri: &str) -> PathBuf {
    let stripped = uri.strip_prefix("file://").unwrap_or(uri);
    PathBuf::from(stripped)
}

/// Unique parent directories of the corpus paths — the wikilink resolution
/// roots for this corpus.
fn corpus_roots(corpus: &Corpus) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    for (p, _) in corpus {
        if let Some(dir) = p.parent() {
            if !roots.iter().any(|r| r == dir) {
                roots.push(dir.to_path_buf());
            }
        }
    }
    roots
}

// ---------------------------------------------------------------------------
// goToDefinition
// ---------------------------------------------------------------------------

/// Channel order: wikilink under cursor, hook/script path token under
/// cursor, then JSON config key -> ontology property.
fn definition(uri: &str, text: &str, position: Position, corpus: &Corpus) -> OperationResult {
    let roots = corpus_roots(corpus);
    // 1. Wikilink.
    if navigation::wikilink_at_position(text, position).is_some() {
        return match navigation::wikilink_definition(uri, text, position, &roots) {
            Some(loc) => OperationResult::Definition(loc),
            None => {
                let stem = navigation::wikilink_at_position(text, position)
                    .map(|l| l.stem)
                    .unwrap_or_default();
                OperationResult::Unresolved {
                    reason: "wikilink-unresolvable",
                    detail: format!("[[{stem}]] resolves to no file in the corpus roots"),
                }
            }
        };
    }
    // 2. Hook/script path token.
    if let Some(loc) = navigation::hook_path_definition(uri, text, position, &base_dir(uri)) {
        return OperationResult::Definition(loc);
    }
    if let Some(result) = path_token_at(uri, text, position) {
        return result;
    }
    // 3. JSON config key -> ontology definition.
    if let Some(key) = json_key_at(text, position) {
        return schema_definition(&key, uri);
    }
    OperationResult::Unresolved {
        reason: "cursor-on-nothing-navigable",
        detail: "no wikilink, path token, or config key under the cursor".to_string(),
    }
}

/// A path-like token under the cursor (markdown bodies, command strings)
/// that navigation.rs's script-ext filter does not cover. Returns Some only
/// when a token containing '/' is under the cursor.
fn path_token_at(uri: &str, text: &str, position: Position) -> Option<OperationResult> {
    let offset = navigation::position_to_offset(text, position)?;
    let line_start = text[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line_end = text[offset..]
        .find('\n')
        .map(|i| offset + i)
        .unwrap_or(text.len());
    let line = &text[line_start..line_end];
    let col = offset - line_start;
    // Walk token spans; return the token whose span contains the column.
    let mut scan = 0usize;
    for tok in line.split([' ', '`', '"', '(', ')', ',']) {
        let tok_start = scan;
        let tok_end = scan + tok.len();
        scan = tok_end + 1; // +1 for the (single) separator char
        if col < tok_start || col > tok_end {
            continue;
        }
        let trimmed = tok.trim_matches(|c| c == ',' || c == '.' || c == '"');
        if trimmed.contains('/') {
            return Some(resolve_path_target(uri, trimmed));
        }
    }
    None
}

/// Resolve a path-ish token to a Location or a typed NotOnDisk.
fn resolve_path_target(uri: &str, token: &str) -> OperationResult {
    if token.is_empty() || !token.contains('/') {
        return OperationResult::Unresolved {
            reason: "not-a-path",
            detail: token.to_string(),
        };
    }
    let expanded = expand_tilde(token);
    let path = if Path::new(&expanded).is_absolute() {
        PathBuf::from(&expanded)
    } else {
        base_dir(uri).join(&expanded)
    };
    if path.is_file() {
        let target_text = std::fs::read_to_string(&path).unwrap_or_default();
        match format!("file://{}", path.display()).parse::<Uri>() {
            Ok(u) => OperationResult::Definition(Location {
                uri: u,
                range: navigation::h1_range(&target_text),
            }),
            Err(e) => OperationResult::Unresolved {
                reason: "uri-parse-failed",
                detail: e.to_string(),
            },
        }
    } else {
        OperationResult::NotOnDisk {
            path: path.display().to_string(),
            reason: "not-on-disk",
        }
    }
}

fn expand_tilde(token: &str) -> String {
    if let Some(rest) = token.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{home}/{rest}");
        }
    }
    token.to_string()
}

fn base_dir(uri: &str) -> PathBuf {
    uri_to_path(uri)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// goToImplementation
// ---------------------------------------------------------------------------

/// goToImplementation for config: the "implementation" of a hook entry is
/// the script it runs. Cursor on a `command` string value or a path token:
///  - path-like value -> resolved file on disk (Location) or typed NotOnDisk
///  - inline shell string -> first word resolved via $PATH (Location on the
///    binary) or typed NotOnDisk(reason: "not-on-path")
fn implementation(uri: &str, text: &str, position: Position, _corpus: &Corpus) -> OperationResult {
    // JSON `command` (or any string value) under the cursor.
    if let Some(ctx) = json_ctx_at(text, position) {
        if let CursorContext::Value { key } = &ctx {
            let value = json_value_text_at(text, position);
            let is_pathish = value.contains('/') || is_script_ext(&value);
            if *key == "command" || is_pathish {
                if is_pathish {
                    return resolve_path_target(uri, &value);
                }
                // Inline shell: resolve the first word via $PATH.
                let word = value.split_whitespace().next().unwrap_or("");
                return resolve_on_path(uri, word);
            }
        }
        // On a key: its implementation is the ontology property.
        if let CursorContext::Key { key, .. } = &ctx {
            return schema_definition(key, uri);
        }
    }
    // Markdown / plain-text path token.
    if let Some(result) = path_token_at(uri, text, position) {
        return result;
    }
    OperationResult::Unresolved {
        reason: "cursor-on-nothing-implementable",
        detail: "no command value or path token under the cursor".to_string(),
    }
}

fn is_script_ext(token: &str) -> bool {
    [".sh", ".py", ".mjs", ".js", ".rb", ".pl"]
        .iter()
        .any(|e| token.ends_with(e))
}

/// Resolve an inline-shell first word against $PATH; Location on the
/// binary when found, typed NotOnDisk otherwise.
fn resolve_on_path(uri: &str, word: &str) -> OperationResult {
    if word.is_empty() {
        return OperationResult::Unresolved {
            reason: "empty-command",
            detail: String::new(),
        };
    }
    if word.contains('/') {
        return resolve_path_target(uri, word);
    }
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let candidate = dir.join(word);
            if candidate.is_file() {
                return match format!("file://{}", candidate.display()).parse::<Uri>() {
                    Ok(u) => OperationResult::Definition(Location {
                        uri: u,
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 0,
                            },
                            end: Position {
                                line: 0,
                                character: 0,
                            },
                        },
                    }),
                    Err(e) => OperationResult::Unresolved {
                        reason: "uri-parse-failed",
                        detail: e.to_string(),
                    },
                };
            }
        }
    }
    OperationResult::NotOnDisk {
        path: word.to_string(),
        reason: "not-on-path",
    }
}

// ---------------------------------------------------------------------------
// Schema (ontology) definition for config keys
// ---------------------------------------------------------------------------

/// The ontology file shipped with this checkout; tests and the dogfood run
/// both execute from the repo, so the compile-time manifest path is the
/// canonical location.
const ONTOLOGY_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/schema/claude-code-config.ttl");

/// goToDefinition for a JSON config key: the key's definition is its
/// property individual in schema/claude-code-config.ttl. Keys not in the
/// registry or the ontology get a typed Unresolved, never a guess.
fn schema_definition(key: &str, _uri: &str) -> OperationResult {
    // Registry gate: the key must be a real schema field on some surface.
    let schema = build_schema();
    let in_registry = schema
        .types
        .values()
        .any(|node| node.fields.iter().any(|(n, _)| n == key));
    if !in_registry {
        return OperationResult::Unresolved {
            reason: "key-not-in-schema",
            detail: format!("config key `{key}` is not a schema field"),
        };
    }
    let ttl = match std::fs::read_to_string(ONTOLOGY_PATH) {
        Ok(t) => t,
        Err(e) => {
            return OperationResult::Unresolved {
                reason: "ontology-unreadable",
                detail: e.to_string(),
            }
        }
    };
    // Property individuals are named ccc:settings_<key>, ccc:agent_<key>,
    // ccc:skill_<key>, ccc:md_<key>, ccc:cmd_<key>, ccc:plugin_<key>, ...
    // Find the line declaring any surface's property for this key.
    for (idx, line) in ttl.lines().enumerate() {
        let candidate = line
            .trim()
            .trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '-');
        for prefix in [
            "ccc:settings_",
            "ccc:agent_",
            "ccc:skill_",
            "ccc:md_",
            "ccc:cmd_",
            "ccc:plugin_",
            "ccc:marketplace_",
            "ccc:mcp_",
            "ccc:hook_",
            "ccc:kb_",
        ] {
            let want = format!("{prefix}{key}");
            if line.trim_start().starts_with("ccc:") && candidate == want {
                return match format!("file://{ONTOLOGY_PATH}").parse::<Uri>() {
                    Ok(u) => OperationResult::Definition(Location {
                        uri: u,
                        range: ttl_line_range(&ttl, idx),
                    }),
                    Err(e) => OperationResult::Unresolved {
                        reason: "uri-parse-failed",
                        detail: e.to_string(),
                    },
                };
            }
        }
    }
    OperationResult::Unresolved {
        reason: "key-not-in-ontology",
        detail: format!("config key `{key}` has no property individual in the ontology"),
    }
}

/// Range of the idx-th line of `ttl` (0-based).
fn ttl_line_range(ttl: &str, idx: usize) -> Range {
    let line = ttl.lines().nth(idx).unwrap_or("");
    Range {
        start: Position {
            line: idx as u32,
            character: 0,
        },
        end: Position {
            line: idx as u32,
            character: line.chars().map(char::len_utf16).sum::<usize>() as u32,
        },
    }
}

// ---------------------------------------------------------------------------
// findReferences
// ---------------------------------------------------------------------------

/// findReferences: cursor on a wikilink -> every mention of the resolved
/// target across the corpus; otherwise the skill/agent name (frontmatter
/// name of this doc, or the word under the cursor) -> hierarchy incoming
/// calls flattened to Locations.
fn references(_uri: &str, text: &str, position: Position, corpus: &Corpus) -> OperationResult {
    let roots = corpus_roots(corpus);
    if let Some(link) = navigation::wikilink_at_position(text, position) {
        let target = navigation::resolve_stem(&link.stem, &roots);
        let docs: Vec<(String, String)> = corpus
            .iter()
            .map(|(p, t)| (format!("file://{}", p.display()), t.clone()))
            .collect();
        let mut locs = match &target {
            Some(t) => navigation::wikilink_references(&docs, t, &roots),
            None => Vec::new(),
        };
        // Include the definition site of the resolved target.
        if let Some(t) = target {
            if let Ok(u) = format!("file://{}", t.display()).parse::<Uri>() {
                let target_text = std::fs::read_to_string(&t).unwrap_or_default();
                locs.push(Location {
                    uri: u,
                    range: navigation::h1_range(&target_text),
                });
            }
        }
        if locs.is_empty() {
            return OperationResult::Unresolved {
                reason: "wikilink-unresolvable",
                detail: format!("[[{0}]] has no references in the corpus", link.stem),
            };
        }
        return OperationResult::References(locs);
    }
    let name = reference_name(text, position);
    if name.is_empty() {
        return OperationResult::Unresolved {
            reason: "no-referenceable-name",
            detail: "cursor is not on a name token and the doc has no frontmatter name".to_string(),
        };
    }
    let incoming = hierarchy::incoming_calls_typed(corpus, &name);
    if incoming.is_empty() {
        return OperationResult::Unresolved {
            reason: "no-references-found",
            detail: format!("`{name}` is not referenced anywhere in the corpus"),
        };
    }
    OperationResult::References(
        incoming
            .into_iter()
            .flat_map(|call| {
                call.from_ranges.into_iter().map(move |r| Location {
                    uri: call.from.uri.clone(),
                    range: r,
                })
            })
            .collect(),
    )
}

/// The name to search references for: the word under the cursor, or the
/// doc's own frontmatter `name` when the cursor is on nothing named.
fn reference_name(text: &str, position: Position) -> String {
    if let Some(word) = word_at(text, position) {
        return word;
    }
    hierarchy::frontmatter_name(text)
        .map(|(n, _)| n)
        .unwrap_or_default()
}

/// Word (alphanumeric/_/-) under the cursor, if any.
fn word_at(text: &str, position: Position) -> Option<String> {
    let offset = navigation::position_to_offset(text, position)?;
    let bytes = text.as_bytes();
    let is_word = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b'-';
    let mut start = offset.min(bytes.len());
    while start > 0 && is_word(bytes[start - 1]) {
        start -= 1;
    }
    let mut end = offset.min(bytes.len());
    while end < bytes.len() && is_word(bytes[end]) {
        end += 1;
    }
    if start == end {
        None
    } else {
        Some(text[start..end].to_string())
    }
}

// ---------------------------------------------------------------------------
// hover
// ---------------------------------------------------------------------------

/// hover: reuse the landed position-aware hover handler. JSON docs get the
/// tree-sitter root so the keyed (not the fallback) entry wins.
fn hover(uri: &str, text: &str, position: Position) -> Option<Hover> {
    if looks_like_json(text) {
        let mut parser = tree_sitter::Parser::new();
        if parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .is_ok()
        {
            if let Some(tree) = parser.parse(text, None) {
                return crate::hover::hover_at_position(
                    uri,
                    text,
                    Some(tree.root_node()),
                    position,
                );
            }
        }
    }
    crate::hover::hover_at_position(uri, text, None, position)
}

/// Cheap structural sniff: does this text begin like a JSON document?
fn looks_like_json(text: &str) -> bool {
    let t = text.trim_start();
    t.starts_with('{') || t.starts_with('[')
}

// ---------------------------------------------------------------------------
// documentSymbol
// ---------------------------------------------------------------------------

/// documentSymbol: JSON -> key tree as typed symbols; markdown -> heading
/// outline via markdown_nav.
fn document_symbols(uri: &str, text: &str) -> Vec<DocumentSymbol> {
    if looks_like_json(text) {
        if let Some(symbols) = json_document_symbols(text) {
            return symbols;
        }
    }
    if uri.ends_with(".json") {
        return Vec::new();
    }
    crate::analyzers::markdown_nav::document_symbols(text)
}

/// Top-level (recursive) JSON keys as DocumentSymbols. Real tree-sitter
/// parse; malformed JSON yields no symbols (typed empty, not garbage).
fn json_document_symbols(text: &str) -> Option<Vec<DocumentSymbol>> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .ok()?;
    let tree = parser.parse(text, None)?;
    let root = tree.root_node();
    if root.has_error() {
        return None;
    }
    Some(json_symbols_from_node(root, text))
}

/// Recursively convert a JSON tree into DocumentSymbols (kind Key/Object/
/// Array/String/Number/Boolean by value shape).
fn json_symbols_from_node(node: tree_sitter::Node<'_>, text: &str) -> Vec<DocumentSymbol> {
    let mut out = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "pair" {
            if let Some(key_node) = child.child_by_field_name("key") {
                let key = unquote(&text[key_node.byte_range()]);
                let value = child.child_by_field_name("value");
                let (kind, children, detail) = match value {
                    Some(v) => match v.kind() {
                        "object" => (
                            SymbolKind::OBJECT,
                            json_symbols_from_node(v, text),
                            "{...}".to_string(),
                        ),
                        "array" => (
                            SymbolKind::ARRAY,
                            json_symbols_from_node(v, text),
                            format!("[{}]", v.child_count()),
                        ),
                        "string" => (
                            SymbolKind::STRING,
                            Vec::new(),
                            unquote(&text[v.byte_range()]),
                        ),
                        "number" => (
                            SymbolKind::NUMBER,
                            Vec::new(),
                            text[v.byte_range()].to_string(),
                        ),
                        "true" | "false" => (SymbolKind::BOOLEAN, Vec::new(), v.kind().to_string()),
                        "null" => (SymbolKind::NULL, Vec::new(), "null".to_string()),
                        _ => (SymbolKind::KEY, Vec::new(), String::new()),
                    },
                    None => (SymbolKind::KEY, Vec::new(), String::new()),
                };
                let range = byte_range_to_range(text, child.byte_range());
                let selection = byte_range_to_range(text, key_node.byte_range());
                out.push(
                    #[allow(deprecated)]
                    DocumentSymbol {
                        name: key,
                        detail: Some(detail),
                        kind,
                        tags: None,
                        range,
                        selection_range: selection,
                        deprecated: None,
                        children: if children.is_empty() {
                            None
                        } else {
                            Some(children)
                        },
                    },
                );
            }
        } else if child.kind() == "object" || child.kind() == "array" {
            out.extend(json_symbols_from_node(child, text));
        }
    }
    out
}

/// Strip one layer of surrounding quotes from a JSON key token.
fn unquote(s: &str) -> String {
    let t = s.trim();
    t.strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .unwrap_or(t)
        .to_string()
}

/// UTF-8 byte range -> UTF-16 LSP range.
fn byte_range_to_range(text: &str, bytes: std::ops::Range<usize>) -> Range {
    Range {
        start: navigation::offset_to_position(text, bytes.start),
        end: navigation::offset_to_position(text, bytes.end),
    }
}

// ---------------------------------------------------------------------------
// workspaceSymbol
// ---------------------------------------------------------------------------

/// workspaceSymbol: scan the corpus root with the landed workspace_symbols
/// walker. Query = the word under the cursor; empty word = all symbols.
fn workspace_symbols(corpus: &Corpus, text: &str, position: Position) -> OperationResult {
    let query = word_at(text, position).unwrap_or_default();
    let root = match corpus_root_dir(corpus) {
        Some(r) => r,
        None => {
            return OperationResult::Unresolved {
                reason: "no-corpus-root",
                detail: "corpus index has no on-disk root to scan".to_string(),
            }
        }
    };
    let symbols = crate::workspace_symbols::workspace_symbols(&root, &query);
    if symbols.is_empty() {
        return OperationResult::Unresolved {
            reason: "no-symbols",
            detail: format!(
                "no config symbols match query `{query}` under {}",
                root.display()
            ),
        };
    }
    OperationResult::WorkspaceSymbols(symbols)
}

/// Deepest directory that contains every corpus path (the common ancestor),
/// for workspace scans.
fn corpus_root_dir(corpus: &Corpus) -> Option<PathBuf> {
    let mut iter = corpus.iter();
    let first = iter.next()?.0.clone();
    let mut prefix = first.parent()?.to_path_buf();
    for (p, _) in iter {
        while !p.starts_with(&prefix) {
            prefix = prefix.parent()?.to_path_buf();
        }
    }
    Some(prefix)
}

// ---------------------------------------------------------------------------
// Call hierarchy trio
// ---------------------------------------------------------------------------

/// prepareCallHierarchy: the callable unit is the doc's frontmatter name.
/// hierarchy::prepare requires the cursor on the name value; harness
/// semantics clamp any cursor inside a named doc to the name range so
/// "prepare on this skill" is meaningful anywhere in the file. Docs with
/// no frontmatter name are a typed Unresolved, not a fake item.
fn prepare_call_hierarchy(corpus: &Corpus, uri: &str, position: Position) -> OperationResult {
    let doc_path = uri_to_path(uri);
    if let Some(item) = hierarchy::prepare_call_hierarchy(corpus, &doc_path, position) {
        return OperationResult::CallHierarchyItem(Some(item));
    }
    // Clamp: retry with the name value's start position.
    let text = corpus
        .iter()
        .find(|(p, _)| *p == doc_path)
        .map(|(_, t)| t.clone())
        .unwrap_or_default();
    if let Some((_, name_range)) = hierarchy::frontmatter_name(&text) {
        if let Some(item) = hierarchy::prepare_call_hierarchy(corpus, &doc_path, name_range.start) {
            return OperationResult::CallHierarchyItem(Some(item));
        }
    }
    OperationResult::Unresolved {
        reason: "no-callable-unit",
        detail: format!("{} has no frontmatter name to prepare", doc_path.display()),
    }
}

/// incomingCalls for this doc's (or the cursor's) name across the corpus.
fn incoming_calls(corpus: &Corpus, uri: &str, text: &str, position: Position) -> OperationResult {
    let name = reference_name(text, position);
    if name.is_empty() {
        return OperationResult::Unresolved {
            reason: "no-referenceable-name",
            detail: "cursor is not on a name token and the doc has no frontmatter name".to_string(),
        };
    }
    let _ = uri;
    let calls = hierarchy::incoming_calls_typed(corpus, &name);
    if calls.is_empty() {
        return OperationResult::Unresolved {
            reason: "no-references-found",
            detail: format!("`{name}` has no incoming references in the corpus"),
        };
    }
    OperationResult::IncomingCalls(calls)
}

/// outgoingCalls from this doc across the corpus.
fn outgoing_calls(corpus: &Corpus, uri: &str) -> OperationResult {
    let doc_path = uri_to_path(uri);
    let calls = hierarchy::outgoing_calls_typed(corpus, &doc_path);
    if calls.is_empty() {
        return OperationResult::Unresolved {
            reason: "no-references-found",
            detail: format!(
                "{} has no outgoing references in the corpus",
                doc_path.display()
            ),
        };
    }
    OperationResult::OutgoingCalls(calls)
}

// ---------------------------------------------------------------------------
// JSON cursor helpers
// ---------------------------------------------------------------------------

/// Tree-sitter JSON root for cursor-context resolution (None for non-JSON
/// text or parse failure).
fn json_ctx_root(text: &str) -> Option<tree_sitter::Tree> {
    if !looks_like_json(text) {
        return None;
    }
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .ok()?;
    parser.parse(text, None)
}

/// CursorContext at a position over JSON text (None for non-JSON).
fn json_ctx_at(text: &str, position: Position) -> Option<CursorContext> {
    let tree = json_ctx_root(text)?;
    let offset = position_to_offset(text, position)?;
    Some(resolve_cursor_context(tree.root_node(), text, offset))
}

/// The JSON key under the cursor (CursorContext::Key), if any.
fn json_key_at(text: &str, position: Position) -> Option<String> {
    match json_ctx_at(text, position)? {
        CursorContext::Key { key, .. } => Some(key),
        _ => None,
    }
}

/// The raw (unquoted) string value under the cursor when the cursor sits
/// inside a string value; empty string otherwise.
fn json_value_text_at(text: &str, position: Position) -> String {
    let offset = match position_to_offset(text, position) {
        Some(o) => o,
        None => return String::new(),
    };
    // Find the enclosing string token around the offset.
    let bytes = text.as_bytes();
    let mut start = offset.min(bytes.len());
    let mut end = start;
    while start > 0 && bytes[start - 1] != b'"' && bytes[start - 1] != b'\n' {
        start -= 1;
    }
    while end < bytes.len() && bytes[end] != b'"' && bytes[end] != b'\n' {
        end += 1;
    }
    if start == end || start == 0 || end >= bytes.len() {
        return String::new();
    }
    // start-1 must be an opening quote, not an escape.
    text[start..end].to_string()
}
