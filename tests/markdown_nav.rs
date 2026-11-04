//! Chicago-style integration tests for markdown navigation.
//!
//! Real collaborators: the fixture tree on disk
//! (`tests/fixtures/md_nav_proj/`), the real analyzer functions, real file
//! IO. No mocks; assertions are on final state.

use claude_code_config_lsp::analyzers::markdown_nav::{
    document_links, document_symbols, folding_ranges, parse_headings,
};

/// Read the fixture doc from disk; returns (content, doc_dir).
fn fixture_doc() -> (String, String) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/md_nav_proj/CLAUDE.md");
    let content = std::fs::read_to_string(&path).expect("fixture CLAUDE.md must exist");
    let dir = path.parent().unwrap().to_string_lossy().to_string();
    (content, dir)
}

#[test]
fn fixture_headings_parse_from_disk() {
    let (content, _) = fixture_doc();
    let hs = parse_headings(&content);
    let names: Vec<&str> = hs.iter().map(|h| h.text.as_str()).collect();
    assert_eq!(names, vec!["Project Rules", "Setup", "Deep Section"]);
    let levels: Vec<usize> = hs.iter().map(|h| h.level).collect();
    assert_eq!(levels, vec![1, 2, 3]);
}

#[test]
fn fixture_symbols_hierarchical_with_ranges() {
    let (content, _) = fixture_doc();
    let syms = document_symbols(&content);
    assert_eq!(syms.len(), 1, "one top-level section: Project Rules");
    let root = &syms[0];
    assert_eq!(root.name, "Project Rules");
    assert_eq!(root.range.start.line, 0);
    assert_eq!(
        root.range.end.line, 7,
        "section runs to last line (index 7)"
    );
    let kids = root.children.as_ref().expect("Setup must nest under root");
    assert_eq!(kids.len(), 1);
    assert_eq!(kids[0].name, "Setup");
    let deep = kids[0].children.as_ref().expect("Deep nests under Setup");
    assert_eq!(deep[0].name, "Deep Section");
    assert_eq!(deep[0].range.start.line, 6);
}

#[test]
fn fixture_folds_cover_all_sections() {
    let (content, _) = fixture_doc();
    let folds = folding_ranges(&content);
    assert_eq!(folds.len(), 3, "one fold per heading");
    assert_eq!(folds[0].start_line, 0);
    assert_eq!(folds[0].end_line, 7);
    assert_eq!(folds[2].start_line, 6);
}

#[test]
fn fixture_links_resolve_against_doc_dir_with_real_targets() {
    let (content, dir) = fixture_doc();
    let links = document_links(&content, &dir);
    assert_eq!(
        links.len(),
        2,
        "hooks/session-start.sh and docs/reference.md"
    );

    let hook = links
        .iter()
        .find(|l| {
            l.target
                .as_ref()
                .unwrap()
                .as_str()
                .ends_with("hooks/session-start.sh")
        })
        .expect("link to hooks/session-start.sh");
    assert_eq!(hook.range.start.line, 1);

    let ref_link = links
        .iter()
        .find(|l| {
            l.target
                .as_ref()
                .unwrap()
                .as_str()
                .ends_with("docs/reference.md")
        })
        .expect("link to docs/reference.md");
    assert_eq!(ref_link.range.start.line, 4);

    // Chicago: the targets are real files on disk next to the fixture doc.
    for l in &links {
        let target = l.target.as_ref().unwrap().as_str();
        let path = target.strip_prefix("file://").expect("file URI");
        let decoded = path.replace("%20", " ");
        assert!(
            std::path::Path::new(&decoded).is_file(),
            "target {target} must be a real file"
        );
    }
}

#[test]
fn non_markdown_content_yields_no_nav() {
    let syms = document_symbols("");
    assert!(syms.is_empty());
    assert!(folding_ranges("").is_empty());
    assert!(document_links("", "/tmp").is_empty());
}
