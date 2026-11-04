//! Provider registry — single registration module mapping each Phase-1
//! provider module (navigation, workspace_symbols, rename, inlay, code_lens,
//! signature_help, formatting, selection, hierarchy, moniker, tokens_delta,
//! color) to the handler shape `backend.rs`'s `LanguageServer` impl calls.
//! Every backend handler is a one-line delegation into this module's pure
//! functions; nothing here touches transport, so the mapping is unit-testable
//! without a server.
//!
//! Wiring receipt: `docs/jira/v26.11.3/receipts/p12-wiring.md`.
//!
//! See Also: `backend.rs` (`impl LanguageServer for ClaudeCodeConfigBackend`),
//! `capabilities.rs` (`server_capabilities`)

use lsp_types_max::*;

// ---------------------------------------------------------------------------
// Roots and corpus helpers
// ---------------------------------------------------------------------------

/// The corpus roots this server serves.
pub fn default_roots() -> Vec<std::path::PathBuf> {
    crate::navigation::default_corpus_roots()
}

/// Directories never descended into (same list as `workspace_symbols`).
const PRUNED_DIRS: &[&str] = &[".git", "target", "node_modules", "_build"];

/// Maximum depth below any root when building a corpus.
const MAX_DEPTH: u32 = 4;

/// Build a hierarchy/code-lens corpus by walking `roots` for `.md` config
/// documents, depth-capped with the prune list above. Read failures are
/// skipped, never crash.
pub fn corpus_from_roots(roots: &[std::path::PathBuf]) -> crate::hierarchy::Corpus {
    let mut out: crate::hierarchy::Corpus = Vec::new();
    let mut stack: Vec<(std::path::PathBuf, u32)> = roots.iter().cloned().map(|r| (r, 0)).collect();
    while let Some((dir, depth)) = stack.pop() {
        if depth > MAX_DEPTH {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let pruned = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| PRUNED_DIRS.contains(&n))
                .unwrap_or(false);
            if path.is_dir() {
                if !pruned {
                    stack.push((path, depth + 1));
                }
            } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    out.push((path, text));
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Markdown-vs-JSON handler discriminator: a document is markdown-class when
/// its path ends in `.md` (SKILL.md, CLAUDE.md, agents/*.md).
pub fn is_markdown_uri(uri: &str) -> bool {
    let Some(path) = uri.strip_prefix("file://") else {
        return false; // only file-scheme documents are markdown-class
    };
    path.ends_with(".md")
}

// ---------------------------------------------------------------------------
// Navigation (definition, references, highlight)
// ---------------------------------------------------------------------------

/// `textDocument/definition`: wikilink `[[stem]]` definition first, hook
/// path reference second; `None` when the cursor is on neither or the
/// target does not resolve.
pub fn goto_definition(
    uri: &str,
    text: &str,
    position: Position,
    roots: &[std::path::PathBuf],
) -> Option<Location> {
    let base_dir = doc_dir(uri);
    crate::navigation::wikilink_definition(uri, text, position, roots)
        .or_else(|| crate::navigation::hook_path_definition(uri, text, position, &base_dir))
}

/// Directory of a `file://` uri, empty string when unparseable.
fn doc_dir(uri: &str) -> std::path::PathBuf {
    let path = uri.strip_prefix("file://").unwrap_or(uri);
    std::path::Path::new(path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default()
}

/// `textDocument/references`: resolve the link under `position`, then find
/// every `[[stem]]` mention across `docs` whose stem resolves to that same
/// target.
pub fn references(
    text: &str,
    position: Position,
    docs: &[(String, String)],
    roots: &[std::path::PathBuf],
) -> Vec<Location> {
    let link = crate::navigation::wikilink_at_position(text, position);
    let link = match link {
        Some(l) => l,
        None => return Vec::new(),
    };
    let Some(target) = crate::navigation::resolve_stem(&link.stem, roots) else {
        return Vec::new();
    };
    crate::navigation::wikilink_references(docs, &target, roots)
}

/// `textDocument/documentHighlight`: every wikilink in `text` resolving to
/// the same target as the link under `position`, as `DocumentHighlight`s.
pub fn document_highlight(
    uri: &str,
    text: &str,
    position: Position,
    roots: &[std::path::PathBuf],
) -> Vec<DocumentHighlight> {
    crate::navigation::wikilink_highlight(uri, text, position, roots)
        .into_iter()
        .map(|loc| DocumentHighlight {
            range: loc.range,
            kind: None,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Rename
// ---------------------------------------------------------------------------

/// `textDocument/prepareRename`: the key range at `position`, when the
/// document is rename-capable.
pub fn prepare_rename(text: &str, position: Position) -> Option<Range> {
    crate::rename::prepare_rename(text, position)
}

/// `PrepareRenameResponse` for [`prepare_rename`].
pub fn prepare_rename_response(text: &str, position: Position) -> Option<PrepareRenameResponse> {
    prepare_rename(text, position).map(PrepareRenameResponse::Range)
}

/// `textDocument/rename`.
pub fn rename(text: &str, position: Position, new_name: &str, uri: &Uri) -> Option<WorkspaceEdit> {
    crate::rename::rename(text, position, new_name, uri)
}

// ---------------------------------------------------------------------------
// Inlay hints, code lens, signature help
// ---------------------------------------------------------------------------

/// `textDocument/inlayHint`.
pub fn inlay_hints(text: &str, range: Range, uri: &Uri) -> Vec<InlayHint> {
    crate::inlay::inlay_hints(text, range, uri)
}

/// `textDocument/codeLens`.
pub fn code_lens(uri: &str, content: &str, corpus: &[String]) -> Vec<CodeLens> {
    crate::code_lens::code_lens(uri, content, corpus)
}

/// `textDocument/signatureHelp`.
pub fn signature_help(text: &str, position: Position) -> Option<SignatureHelp> {
    crate::signature_help::signature_help_at_position(text, position)
}

// ---------------------------------------------------------------------------
// Formatting
// ---------------------------------------------------------------------------

/// Full-document range for [`format_document_full`].
fn full_range(text: &str) -> Range {
    let end = crate::navigation::offset_to_position(text, text.len());
    Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end,
    }
}

/// `textDocument/formatting`.
pub fn format_document_full(text: &str) -> Option<Vec<TextEdit>> {
    crate::formatting::format_document(text, full_range(text))
}

/// `textDocument/rangeFormatting`.
pub fn range_formatting(
    text: &str,
    range: Range,
    options: &FormattingOptions,
) -> Option<Vec<TextEdit>> {
    crate::formatting::format_range(text, range, options)
}

/// `textDocument/onTypeFormatting`.
pub fn on_type_formatting(text: &str, position: Position, trigger: char) -> Option<Vec<TextEdit>> {
    crate::formatting::on_type_format(text, position, trigger)
}

// ---------------------------------------------------------------------------
// Selection ranges + document highlight (JSON and markdown dispatch)
// ---------------------------------------------------------------------------

/// `textDocument/selectionRange`: markdown vs JSON dispatch by uri.
pub fn selection_range(text: &str, position: Position, uri: &str) -> Option<SelectionRange> {
    crate::selection::selection_range(text, position, is_markdown_uri(uri))
}

/// `textDocument/documentHighlight` for non-navigation surfaces: JSON and
/// markdown bracket/key highlighting. Navigation-class documents (wikilink
/// markdown) should prefer [`document_highlight`]; this variant never fails,
/// returning an empty vec off-keyword.
pub fn selection_highlight(text: &str, position: Position, uri: &str) -> Vec<DocumentHighlight> {
    crate::selection::highlight(text, position, is_markdown_uri(uri))
}

// ---------------------------------------------------------------------------
// Moniker
// ---------------------------------------------------------------------------

/// `textDocument/moniker`: returns the moniker string for the settings key
/// at `position`, `None` when off-key.
pub fn moniker(
    text: &str,
    uri: &str,
    position: Position,
    home_dir: Option<&str>,
    project_root: Option<&std::path::Path>,
) -> Option<(String, Range)> {
    let parsed: Uri = uri.parse().ok()?;
    let (moniker, key, _scope) =
        crate::moniker::moniker_at_position(text, &parsed, position, home_dir, project_root)?;
    Some((moniker, key.range()))
}

// ---------------------------------------------------------------------------
// Call hierarchy + workspace symbols
// ---------------------------------------------------------------------------

/// `textDocument/prepareCallHierarchy`: the skill/agent item when `position`
/// sits on the document's frontmatter `name`.
pub fn prepare_call_hierarchy(
    corpus: &crate::hierarchy::Corpus,
    doc_path: &std::path::Path,
    position: Position,
) -> Option<CallHierarchyItem> {
    crate::hierarchy::prepare_call_hierarchy(corpus, doc_path, position)
}

/// `callHierarchy/incomingCalls`.
pub fn incoming_calls(
    corpus: &crate::hierarchy::Corpus,
    name: &str,
) -> Vec<CallHierarchyIncomingCall> {
    crate::hierarchy::incoming_calls_typed(corpus, name)
}

/// `callHierarchy/outgoingCalls`.
pub fn outgoing_calls(
    corpus: &crate::hierarchy::Corpus,
    doc_path: &std::path::Path,
) -> Vec<CallHierarchyOutgoingCall> {
    crate::hierarchy::outgoing_calls_typed(corpus, doc_path)
}

/// `workspace/symbol`.
pub fn workspace_symbols(root: &std::path::Path, query: &str) -> Vec<SymbolInformation> {
    crate::workspace_symbols::workspace_symbols(root, query)
}

// ---------------------------------------------------------------------------
// Color
// ---------------------------------------------------------------------------

/// `textDocument/documentColor`.
pub fn document_color(text: &str) -> Vec<ColorInformation> {
    crate::color::detect_colors(text)
        .into_iter()
        .map(|d| crate::color::color_information(&d))
        .collect()
}

/// `textDocument/colorPresentation`.
pub fn color_presentation(text: &str, color: &Color, range: &Range) -> Vec<ColorPresentation> {
    let detected = crate::color::detect_colors(text);
    crate::color::color_presentations(&detected, color, range)
}

// ---------------------------------------------------------------------------
// Semantic tokens delta
// ---------------------------------------------------------------------------

/// `textDocument/semanticTokens/full/delta`. With a `prev` snapshot this
/// computes a true delta edit set; without one it returns the full token
/// set (a legal in-place response — a client treats a full reply to a
/// delta request as a re-sync).
pub fn semantic_tokens_full_delta(
    prev: Option<&SemanticTokens>,
    next: SemanticTokens,
    result_id: u64,
) -> SemanticTokensFullDeltaResult {
    match prev {
        Some(prev) => SemanticTokensFullDeltaResult::TokensDelta(
            crate::tokens_delta::semantic_tokens_delta(prev, &next, result_id),
        ),
        None => SemanticTokensFullDeltaResult::Tokens(next),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MD: &str = "See [[other-note]] and [[other-note]] again.\n";
    const JSON: &str = "{\n  \"model\": \"opus\"\n}\n";

    fn pos(line: u32, character: u32) -> Position {
        Position { line, character }
    }

    #[test]
    fn is_markdown_uri_dispatches_by_extension() {
        assert!(is_markdown_uri("file:///x/CLAUDE.md"));
        assert!(!is_markdown_uri("file:///x/settings.json"));
        assert!(!is_markdown_uri("https://example.com/a.md"));
    }

    #[test]
    fn goto_definition_resolves_wikilink_stem_to_h1() {
        let dir = std::env::temp_dir().join("p12-registry-def");
        std::fs::create_dir_all(&dir).unwrap();
        let target = dir.join("other-note.md");
        std::fs::write(&target, "# Other Note\n").unwrap();
        let roots = vec![dir.clone()];
        let loc = goto_definition("file:///p12-registry-def/CLAUDE.md", MD, pos(0, 6), &roots);
        assert!(loc.is_some(), "wikilink at cursor must resolve");
        assert_eq!(loc.unwrap().range.start.line, 0);
        assert!(
            goto_definition("file:///p12-registry-def/CLAUDE.md", MD, pos(0, 0), &roots).is_none(),
            "cursor off any link must yield None"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn document_highlight_maps_locations_to_highlights() {
        let dir = std::env::temp_dir().join("p12-registry-hl");
        std::fs::create_dir_all(&dir).unwrap();
        let target = dir.join("other-note.md");
        std::fs::write(&target, "# Other Note\n").unwrap();
        let roots = vec![dir.clone()];
        let highlights =
            document_highlight("file:///p12-registry-hl/CLAUDE.md", MD, pos(0, 6), &roots);
        assert_eq!(
            highlights.len(),
            2,
            "both [[other-note]] mentions highlight"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn references_requires_resolvable_link_under_cursor() {
        let docs = vec![(
            "file:///a/CLAUDE.md".to_string(),
            "[[other-note]]".to_string(),
        )];
        let roots = vec![std::env::temp_dir()];
        // No link under cursor -> empty, never panic.
        assert!(references(JSON, pos(0, 2), &docs, &roots).is_empty());
    }

    #[test]
    fn rename_round_trip_delegates_to_rename_module() {
        let text = "{\n  \"model\": \"opus\"\n}\n";
        let uri: Uri = "file:///x/settings.json".parse().unwrap();
        let range = prepare_rename_response(text, pos(1, 4));
        assert!(range.is_some(), "key position must be renameable");
        let edit = rename(text, pos(1, 4), "reasoning", &uri);
        assert!(edit.is_some());
        let none = rename(text, pos(1, 4), "not a valid key name!", &uri);
        assert!(none.is_none(), "invalid key name must refuse");
    }

    #[test]
    fn inlay_and_code_lens_delegate_return_values() {
        let uri: Uri = "file:///x/settings.json".parse().unwrap();
        let hints = inlay_hints(JSON, full_range(JSON), &uri);
        assert!(!hints.is_empty(), "a model key must yield an inlay hint");
        let lens = code_lens(
            "file:///x/SKILL.md",
            "---\nname: demo\n---\nbody",
            &["demo".to_string()],
        );
        assert!(!lens.is_empty(), "SKILL.md with name yields lenses");
    }

    #[test]
    fn formatting_full_document_returns_edits() {
        let edits = format_document_full(JSON);
        assert!(
            edits.is_some(),
            "formatter must produce edits for valid JSON"
        );
    }

    #[test]
    fn selection_range_dispatches_markdown_vs_json() {
        let json_sel = selection_range(JSON, pos(2, 6), "file:///x/settings.json");
        assert!(json_sel.is_some());
        let md_sel = selection_range(MD, pos(0, 6), "file:///x/CLAUDE.md");
        assert!(md_sel.is_some());
    }

    #[test]
    fn moniker_builds_dotted_scheme_for_settings_key() {
        let (m, range) = moniker(
            JSON,
            "file:///x/settings.json",
            pos(1, 4),
            None,
            Some(std::path::Path::new("/x")),
        )
        .expect("key at (1,4) must produce a moniker");
        assert!(m.starts_with("claude-config:"));
        assert_eq!(range.start.line, 1);
        assert!(moniker(JSON, "file:///x/settings.json", pos(0, 0), None, None).is_none());
    }

    #[test]
    fn workspace_symbols_finds_symbols_in_real_tree() {
        let dir = std::env::temp_dir().join("p12-registry-ws");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("SKILL.md"), "---\nname: p12probe\n---\n").unwrap();
        let symbols = workspace_symbols(&dir, "p12probe");
        assert!(!symbols.is_empty(), "name symbol must be found in real dir");
        assert!(workspace_symbols(&dir, "zzz-no-such").is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn hierarchy_corpus_walk_prunes_and_depth_caps() {
        let root = std::env::temp_dir().join("p12-registry-corpus");
        let nested = root.join("a").join("b").join("c").join("d").join("e");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(root.join("top.md"), "# top\n").unwrap();
        std::fs::write(nested.join("deep.md"), "# deep\n").unwrap();
        let corpus = corpus_from_roots(std::slice::from_ref(&root));
        assert!(corpus.iter().any(|(p, _)| p.ends_with("top.md")));
        assert!(
            !corpus.iter().any(|(p, _)| p.ends_with("deep.md")),
            "depth 5 doc must be pruned"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn document_color_and_presentations_on_hex() {
        let text = "{ \"color\": \"#ff0000\" }";
        let colors = document_color(text);
        assert!(!colors.is_empty(), "hex literal must be detected");
        let info = &colors[0];
        let pres = color_presentation(text, &info.color, &info.range);
        assert!(!pres.is_empty(), "presentations exist for a detected color");
    }

    #[test]
    fn tokens_delta_returns_delta_with_prev_and_full_without() {
        let tok = |dl: u32| SemanticToken {
            delta_line: dl,
            delta_start: 0,
            length: 3,
            token_type: 0,
            token_modifiers_bitset: 0,
        };
        let a = SemanticTokens {
            result_id: None,
            data: vec![tok(0), tok(1)],
        };
        let b = SemanticTokens {
            result_id: None,
            data: vec![tok(0), tok(1), tok(2)],
        };
        match semantic_tokens_full_delta(Some(&a), b.clone(), 7) {
            SemanticTokensFullDeltaResult::TokensDelta(delta) => {
                assert_eq!(delta.result_id, Some("7".to_string()));
                assert!(!delta.edits.is_empty(), "append must yield one edit");
            }
            other => panic!("expected TokensDelta, got {other:?}"),
        }
        assert!(matches!(
            semantic_tokens_full_delta(None, b, 7),
            SemanticTokensFullDeltaResult::Tokens(_)
        ));
    }
}
