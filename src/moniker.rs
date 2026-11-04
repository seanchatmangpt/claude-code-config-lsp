//! Stable config-key monikers: `textDocument/moniker` semantics for the
//! claude-code-config LSP.
//!
//! Moniker scheme (stable key identity, not file identity):
//!
//! - global settings: `claude-config:global:settings.<dotted.path>`
//! - project:         `claude-config:project:<relative-path>:<key>`
//!
//! Scope detection is a local re-implementation of the URI-prefix logic in
//! `backend.rs` (`AgentScope`, Global/Plugin/Project by `~/.claude/` and
//! `/.claude/plugins/` prefixes) — cited as prior art, deliberately not
//! imported, so backend.rs stays untouched (lane contract: no backend.rs or
//! capabilities.rs edits).

use lsp_types_max::{Position, Range, Uri};
use std::path::{Path, PathBuf};

/// Which of the three layers a config file resolves from. Local mirror of
/// `backend.rs::AgentScope` (prior art, testable pure function).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigScope {
    /// `~/.claude/...` — applies across all of the user's projects.
    Global,
    /// The current project's `.claude/...` or repo-local config.
    Project,
    /// Installed under a plugin-cache path (`~/.claude/plugins/cache/...`).
    Plugin,
}

/// Local re-implementation of `backend.rs::agent_scope` (prior art cited;
/// not imported — backend.rs is outside this lane's file ownership).
pub fn config_scope(uri: &str, home_dir: Option<&str>) -> ConfigScope {
    let path = uri.strip_prefix("file://").unwrap_or(uri);
    if path.contains("/.claude/plugins/") {
        return ConfigScope::Plugin;
    }
    if let Some(home) = home_dir {
        let home_claude = format!("{home}/.claude/");
        if path.starts_with(&home_claude) {
            return ConfigScope::Global;
        }
    }
    ConfigScope::Project
}

/// A settings.json key occurrence: dotted JSON path plus its source range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsKey {
    /// Dotted path from the document root, e.g. `enabledPlugins.ggen-lsp@ggen`.
    path: String,
    /// Range of the key token (opening quote through closing quote,
    /// UTF-16 LSP coords).
    range: Range,
}

impl SettingsKey {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn range(&self) -> Range {
        self.range
    }

    /// True when `position` lies within the key token's range.
    pub fn contains(&self, position: Position) -> bool {
        range_contains(&self.range, position)
    }
}

/// `start <= position <= end` over (line, utf16-character) coordinates.
fn range_contains(range: &Range, position: Position) -> bool {
    (range.start.line, range.start.character) <= (position.line, position.character)
        && (position.line, position.character) <= (range.end.line, range.end.character)
}

/// Scan a settings.json document for every object key, computing each
/// key's dotted JSON path. Objects contribute their member names; arrays
/// contribute their live element indices, so every scanned path resolves
/// as a serde_json pointer with `.` → `/` (cross-checked by tests against
/// the real parse of the fixture). serde_json does not preserve key
/// ranges, which is why the scan runs over raw text.
pub fn settings_keys(text: &str) -> Vec<SettingsKey> {
    #[derive(Clone)]
    struct Container {
        is_object: bool,
        member: Option<String>,
        index: usize,
    }

    /// Dotted path from the ancestor containers plus the key.
    fn dotted(containers: &[Container], key: &str) -> String {
        let mut parts: Vec<String> = Vec::new();
        for c in containers {
            // Member name (None for the document root): present for both
            // object and array containers — "PostToolUse": [...] keeps its
            // name in the path.
            if let Some(m) = &c.member {
                parts.push(m.clone());
            }
            // Array containers also contribute their live element index.
            if !c.is_object {
                parts.push(c.index.to_string());
            }
        }
        parts.push(key.to_string());
        parts.join(".")
    }

    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut i = 0usize;
    let mut keys: Vec<SettingsKey> = Vec::new();
    let mut containers: Vec<Container> = Vec::new();
    let mut last_key: Option<String> = None;
    let mut line = 0usize;
    let mut col16 = 0usize;
    let mut in_string = false;
    let mut string_start: (usize, usize) = (0, 0);
    let mut string_buf = String::new();
    let mut escaped = false;

    while i < n {
        let c = chars[i];
        if c == '\n' {
            line += 1;
            col16 = 0;
            i += 1;
            // A raw newline cannot occur inside a valid JSON string; reset
            // any in-progress string (malformed doc) rather than misparse.
            in_string = false;
            escaped = false;
            string_buf.clear();
            continue;
        }
        if in_string {
            if escaped {
                string_buf.push(c);
                escaped = false;
            } else {
                match c {
                    '\\' => escaped = true,
                    '"' => {
                        // Classify by the next non-space char on this line:
                        // `:` → object key; anything else → value string.
                        let mut j = i + 1;
                        while j < n && (chars[j] == ' ' || chars[j] == '\t') {
                            j += 1;
                        }
                        if j < n && chars[j] == ':' {
                            keys.push(SettingsKey {
                                path: dotted(&containers, &string_buf),
                                range: Range {
                                    start: Position {
                                        line: string_start.0 as u32,
                                        character: string_start.1 as u32,
                                    },
                                    end: Position {
                                        line: line as u32,
                                        character: (col16 + 1) as u32,
                                    },
                                },
                            });
                            last_key = Some(string_buf.clone());
                        }
                        in_string = false;
                    }
                    _ => string_buf.push(c),
                }
            }
            i += 1;
            col16 += c.len_utf16();
            continue;
        }
        match c {
            '"' => {
                in_string = true;
                string_start = (line, col16);
                string_buf.clear();
            }
            '{' => {
                containers.push(Container {
                    is_object: true,
                    member: last_key.take(),
                    index: 0,
                });
            }
            '[' => {
                containers.push(Container {
                    is_object: false,
                    member: last_key.take(),
                    index: 0,
                });
            }
            '}' | ']' => {
                containers.pop();
                last_key = None;
            }
            ',' => {
                // Top-level array: advance the element index.
                if let Some(top) = containers.last_mut() {
                    if !top.is_object {
                        top.index += 1;
                    }
                }
            }
            _ => {}
        }
        i += 1;
        col16 += c.len_utf16();
    }
    keys
}

/// Moniker string for a config key, per the scheme:
/// - Global:  `claude-config:global:settings.<dotted.path>`
/// - Project: `claude-config:project:<relative-path>:<key>`
/// - Plugin:  keyed by cache path, like project.
pub fn moniker_for_key(scope: ConfigScope, relative_path: &Path, dotted_path: &str) -> String {
    match scope {
        ConfigScope::Global => format!("claude-config:global:settings.{dotted_path}"),
        ConfigScope::Project | ConfigScope::Plugin => format!(
            "claude-config:project:{}:{}",
            relative_path.display(),
            dotted_path
        ),
    }
}

/// Full moniker computation for a position in a settings document: scope
/// from the URI, key from the text scan, moniker from the scheme. Returns
/// `(moniker, key, scope)`; `None` when the position is not on a settings
/// key.
pub fn moniker_at_position(
    text: &str,
    uri: &Uri,
    position: Position,
    home_dir: Option<&str>,
    project_root: Option<&Path>,
) -> Option<(String, SettingsKey, ConfigScope)> {
    let scope = config_scope(uri.as_str(), home_dir);
    let path = uri.as_str().strip_prefix("file://").unwrap_or(uri.as_str());
    let path = Path::new(path);
    let relative = match (scope, project_root) {
        (ConfigScope::Global, _) => PathBuf::from("settings"),
        (ConfigScope::Project | ConfigScope::Plugin, Some(root)) => path
            .strip_prefix(root)
            .ok()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| {
                path.file_name()
                    .map(|f| PathBuf::from(f.to_string_lossy().as_ref()))
                    .unwrap_or_else(|| PathBuf::from("unknown"))
            }),
        (ConfigScope::Project | ConfigScope::Plugin, None) => match path.file_name() {
            Some(f) => PathBuf::from(f.to_string_lossy().as_ref()),
            None => PathBuf::new(),
        },
    };
    let keys = settings_keys(text);
    let key = keys.into_iter().find(|k| k.contains(position))?;
    let moniker = moniker_for_key(scope, &relative, key.path());
    Some((moniker, key, scope))
}
