# How-to Guides

Concrete recipes for specific tasks. Each guide assumes the server bootstraps
via `ggen sync` and the lsp-max `RulePackServer` trait.

---

## Add a diagnostic for SKILL.md frontmatter violations

SKILL.md files must have frontmatter that satisfies the Claude Code Skill naming
contract: `name` ≤ 64 chars, lowercase/hyphens only, no reserved words
(`anthropic`, `claude`); `description` non-empty, ≤ 1024 chars.

**1 — Declare the surface in the ontology**

```turtle
@prefix ccc: <https://claude-code-config-lsp.rs/ontology/> .

ccc:SkillMd a ccc:ConfigSurface ;
    ccc:filePattern      "SKILL.md" ;
    ccc:analyzerModule   "claude_md" ;
    ccc:analyzerModulePascal "ClaudeMd" ;
    ccc:languageId       "markdown" .
```

**2 — Run `ggen sync`**

`src/analyzers/claude_md.rs` appears with a stub `analyze()` function.

**3 — Implement the validation logic**

```rust
// src/analyzers/claude_md.rs  (generated stub — fill in the logic)
use lsp_max::lsp_types::{Diagnostic, DiagnosticSeverity, Range, Position};

pub fn analyze(content: &str) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let Some(fm) = extract_frontmatter(content) else { return diags };

    if let Some(name) = fm.get("name") {
        if name.len() > 64 {
            diags.push(Diagnostic {
                range: Range::new(Position::new(1, 0), Position::new(1, name.len() as u32)),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(lsp_max::lsp_types::NumberOrString::String(
                    "CCC-SKILL-001".into(),
                )),
                message: format!("Skill name exceeds 64 characters ({} chars)", name.len()),
                ..Default::default()
            });
        }
        if name.chars().any(|c| c.is_uppercase()) {
            diags.push(/* CCC-SKILL-002: uppercase chars forbidden */);
        }
        for reserved in &["anthropic", "claude"] {
            if name.contains(reserved) {
                diags.push(/* CCC-SKILL-003: reserved word */);
            }
        }
    }

    if fm.get("description").map(|d| d.is_empty()).unwrap_or(true) {
        diags.push(/* CCC-SKILL-004: description required and non-empty */);
    }

    diags
}
```

**4 — Wire the analyzer into the backend**

Override `scan_uri_classified` in `src/backend.rs`:

```rust
fn scan_uri_classified(&self, uri: &DocumentUri, content: &str) -> ClassifiedFindings {
    if uri.ends_with("SKILL.md") {
        let findings = crate::analyzers::claude_md::analyze(content)
            .into_iter()
            .map(|d| (MaxDiagnostic { lsp: d.clone(), ..Default::default() }, d))
            .collect();
        return (findings, vec![]);
    }
    (vec![], vec![])
}
```

---

## Add hover for SKILL.md frontmatter fields

Hovering on `allowed-tools:` in a SKILL.md should show what values are legal
and whether the field applies in CLI vs SDK mode.

**1 — Extend `src/hover.rs`**

```rust
pub async fn handle_hover(uri: &str, line: u32, content: &str) -> Option<Hover> {
    if !uri.ends_with("SKILL.md") { return None; }

    let text = content.lines().nth(line as usize)?;
    let field = text.split(':').next()?.trim();

    let markdown = match field {
        "name" => "**`name`** *(required)*\n\nMax 64 chars. Lowercase, numbers, \
                   hyphens only. Cannot contain `anthropic` or `claude`.",
        "description" => "**`description`** *(required)*\n\nMax 1024 chars. \
                          Third-person. Must include both what the Skill does \
                          and when to trigger it. Claude selects Skills from \
                          100+ available by matching this text.",
        "allowed-tools" => "**`allowed-tools`** *(optional, CLI only)*\n\nList \
                            of tool names this Skill may invoke. **CLI only** — \
                            ignored when accessed through the Agent SDK. Use \
                            `allowedTools` in SDK `query()` config instead.",
        "disallowed-tools" => "**`disallowed-tools`** *(optional)*\n\nTools \
                               explicitly blocked for this Skill.",
        "context" => "**`context`** *(optional)*\n\nSet to `fork` to run this \
                      Skill in an isolated context branch.",
        "paths" => "**`paths`** *(optional)*\n\nGlob patterns limiting which \
                    files this Skill can access.",
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: markdown.to_string(),
        }),
        range: None,
    })
}
```

---

## Add completion for MCP tool references

When editing `allowed-tools:` in SKILL.md or `allowedTools` in JSON config,
offer completions following the `mcp__<server>__<tool>` naming pattern.

**1 — Extend `src/completion.rs`**

```rust
pub async fn handle_completion(
    uri: &str, line: u32, character: u32, content: &str,
) -> Option<CompletionResponse> {
    let text = content.lines().nth(line as usize)?;

    // Trigger inside allowed-tools list
    if text.trim_start().starts_with("allowed-tools") || text.contains("allowedTools") {
        let partial = word_at(text, character as usize);

        let mut items = vec![
            // Standard Claude Code tools
            completion_item("Bash", "Execute shell commands"),
            completion_item("Read", "Read files"),
            completion_item("Edit", "Edit files"),
            completion_item("Write", "Write files"),
            completion_item("Glob", "Glob file patterns"),
        ];

        // MCP wildcard pattern hint
        if partial.starts_with("mcp__") {
            items.push(CompletionItem {
                label: "mcp__<server>__*".into(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("Allow all tools from an MCP server".into()),
                insert_text: Some("mcp__${1:server-name}__*".into()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        return Some(CompletionResponse::Array(items));
    }

    None
}
```

---

## Add SKILL.md description length diagnostic

The description field has a 1024-character limit enforced by Claude Code at
runtime. Warn early:

```rust
if let Some(desc) = fm.get("description") {
    if desc.len() > 1024 {
        diags.push(Diagnostic {
            range: /* description line range */,
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("CCC-SKILL-005".into())),
            message: format!(
                "Skill description exceeds 1024 characters ({} chars). \
                 Claude may truncate during Skill discovery.",
                desc.len()
            ),
            ..Default::default()
        });
    }
    if desc.is_empty() {
        diags.push(/* CCC-SKILL-004 */);
    }
}
```

---

## Re-generate after schema changes

After editing any `.ttl`, `.rq`, or `ggen.toml`:

```sh
ggen sync          # regenerate
cargo check        # verify it compiles
cargo clippy --all-targets -- -D warnings   # verify no warnings
cargo test         # verify tests pass
```

Never edit generated files in `src/` directly — edit the ontology or the pack
templates in `../lsp-max/templates/lsp-max/` and regenerate.

---

## Wire a new analyzer into the module graph

When a new `ConfigSurface` individual is added to the ontology and `ggen sync`
runs, `src/analyzers/mod.rs` and `src/lib.rs` are both regenerated automatically
— the new module is declared in the right places. The only manual step is
implementing the validation body in the generated analyzer stub.
