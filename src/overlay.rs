//! Settings overlay intelligence: merge-view over ordered settings layers.
//!
//! Claude Code config is a stack of partial overlays — managed/enterprise
//! policy (`managed-settings.json`), user (`~/.claude/settings.json`),
//! project (`.claude/settings.json`), and project-local
//! (`.claude/settings.local.json`, gitignored). Each layer is an arbitrary
//! key subset; later layers win per Claude Code precedence (managed < user <
//! project < local).
//!
//! Partial-overlay law: missing keys are normal, never diagnosed.
//!
//! Diagnostics:
//! - `OVERRIDE_CONFLICT` (warning): the same key appears in >= 2 layers with
//!   different values — message names both layers and values; remediation is
//!   to pick the intended layer.
//! - `LOCAL_SHADOW` (info): `settings.local.json` overrides a
//!   `settings.json` value — the expected gitignore pattern, not an error.
//!
//! `effective_config` is a pure fn usable by other providers — a hover
//! provider on an overlaid key can later call it to show the effective value
//! plus source layer (wiring point: `hover.rs` would map a hovered key at a
//! document position to a layer name + key, then call `effective_config`).

use serde_json::{Map, Value};

/// Canonical layer order, lowest precedence first.
pub const LAYER_ORDER: [&str; 4] = ["managed", "user", "project", "local"];

/// Well-known file names for each canonical layer.
pub const LAYER_FILES: [(&str, &str); 4] = [
    ("managed", "managed-settings.json"),
    ("user", "settings.json"),
    ("project", "settings.json"),
    ("local", "settings.local.json"),
];

/// A single settings layer: parsed top-level key map plus its identity.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    /// Layer identity, e.g. `"managed"`, `"user"`, `"project"`, `"local"`.
    /// Any name is accepted; order in the slice is precedence order.
    pub name: String,
    /// File the layer was parsed from, when known.
    pub path: Option<String>,
    /// Top-level settings keys (partial overlay — arbitrary key subset).
    pub settings: Map<String, Value>,
}

/// Diagnostic severity, mirroring the LSP diagnostic severities we emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Information,
}

/// One overlay diagnostic.
#[derive(Debug, Clone, PartialEq)]
pub struct OverlayDiagnostic {
    /// `OVERRIDE_CONFLICT` or `LOCAL_SHADOW`.
    pub code: String,
    pub severity: Severity,
    pub message: String,
    /// The conflicted/shadowed key.
    pub key: String,
}

/// Merge an ordered slice of layers into the effective config.
///
/// Returns `(effective, provenance)`: the effective top-level key map, and a
/// map from every effective key to the layer name that supplied it. Later
/// layers win; an empty layer contributes nothing; keys absent from all
/// layers are absent from the view (partial-overlay law).
pub fn effective_config(layers: &[Layer]) -> (Map<String, Value>, Map<String, Value>) {
    let mut effective = Map::new();
    let mut provenance = Map::new();
    for layer in layers {
        for (key, value) in &layer.settings {
            effective.insert(key.clone(), value.clone());
            provenance.insert(key.clone(), Value::String(layer.name.clone()));
        }
    }
    (effective, provenance)
}

/// Compute overlay diagnostics for an ordered layer slice (lowest first).
///
/// - Keys present in >= 2 layers with different values → one
///   `OVERRIDE_CONFLICT` naming the losing (lowest-precedence) layer, the
///   winning (highest-precedence) layer, and both values.
/// - `local` overriding `project` (same key present in both) → `LOCAL_SHADOW`
///   info, regardless of whether the values differ.
pub fn overlay_diagnostics(layers: &[Layer]) -> Vec<OverlayDiagnostic> {
    let mut diags = Vec::new();
    let by_name = |name: &str| layers.iter().find(|l| l.name == name);

    for key in collect_keys(layers) {
        // Collect (layer index, value) pairs for this key across layers.
        let hits: Vec<(usize, &Value)> = layers
            .iter()
            .enumerate()
            .filter_map(|(i, l)| l.settings.get(&key).map(|v| (i, v)))
            .collect();

        // OVERRIDE_CONFLICT: >= 2 hits with at least one differing value.
        // Winner = highest index (later layer), loser = lowest index layer
        // that disagrees with the winner.
        if hits.len() >= 2 {
            let (win_i, win_v) = hits[hits.len() - 1];
            let first = hits[0];
            if first.1 != win_v {
                let loser = layers[first.0].name.clone();
                let winner = layers[win_i].name.clone();
                diags.push(OverlayDiagnostic {
                    code: "OVERRIDE_CONFLICT".into(),
                    severity: Severity::Warning,
                    message: format!(
                        "key '{key}' is set in '{loser}' ({}) and overridden by '{winner}' ({}) with a different value; pick the intended layer",
                        first.1, win_v
                    ),
                    key: key.clone(),
                });
            }
        }

        // LOCAL_SHADOW: local overrides project settings.json.
        if let (Some(project), Some(local)) = (by_name("project"), by_name("local")) {
            if project.settings.contains_key(&key) && local.settings.contains_key(&key) {
                diags.push(OverlayDiagnostic {
                    code: "LOCAL_SHADOW".into(),
                    severity: Severity::Information,
                    message: format!(
                        "'{key}' in settings.local.json shadows the settings.json value (expected pattern; edit settings.local.json to change the effective value)"
                    ),
                    key: key.clone(),
                });
            }
        }
    }
    diags
}

/// Union of top-level keys across layers, in first-seen order.
fn collect_keys(layers: &[Layer]) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();
    for layer in layers {
        for key in layer.settings.keys() {
            if !keys.contains(key) {
                keys.push(key.clone());
            }
        }
    }
    keys
}

/// Parse one layer from raw JSON text. Malformed JSON yields an empty layer
/// (tolerance: a broken file must not poison the merge view); report it via
/// `layer_parse_error` if callers want the failure surfaced.
pub fn parse_layer(name: &str, path: Option<String>, text: &str) -> Layer {
    let settings = serde_json::from_str::<Map<String, Value>>(text).unwrap_or_default();
    Layer {
        name: name.to_string(),
        path,
        settings,
    }
}

/// Read an ordered layer set from canonical paths. Missing files are normal
/// (partial-overlay law) and skipped; unreadable-but-present files parse as
/// empty via `parse_layer` tolerance.
///
/// `root` is the config root directory (`~/.claude` or `~/.zcode` — the
/// grammar is identical for parallel roots; only the path differs). When
/// `managed_settings_path` is `Some`, it is used for the managed layer
/// (enterprise policy location varies by platform).
pub fn layers_from_root(
    root: &std::path::Path,
    managed_settings_path: Option<&std::path::Path>,
) -> Vec<Layer> {
    let mut layers = Vec::new();
    if let Some(mp) = managed_settings_path {
        if let Ok(text) = std::fs::read_to_string(mp) {
            layers.push(parse_layer(
                "managed",
                Some(mp.display().to_string()),
                &text,
            ));
        }
    }
    let read = |rel: &str, name: &str| {
        std::fs::read_to_string(root.join(rel))
            .ok()
            .map(|text| parse_layer(name, Some(root.join(rel).display().to_string()), &text))
    };
    // user layer: settings.json directly under the root
    if let Some(l) = read("settings.json", "user") {
        layers.push(l);
    }
    // project layers: under <root-project>/claude/... — callers pass the
    // project settings dir as `root` too when it is the same tree; for the
    // canonical single-root layout the project/local files live alongside.
    if let Some(l) = read(".claude/settings.json", "project") {
        layers.push(l);
    }
    if let Some(l) = read(".claude/settings.local.json", "local") {
        layers.push(l);
    }
    layers
}

/// Convenience: parse all layers from explicit files (missing files skipped).
/// Each entry is `(layer name, path)`. This is the pure entry point callers
/// with non-canonical layouts (parallel `.zcode` roots, test fixtures) use.
pub fn layers_from_paths(entries: &[(&str, &std::path::Path)]) -> Vec<Layer> {
    entries
        .iter()
        .filter_map(|(name, path)| {
            std::fs::read_to_string(path)
                .ok()
                .map(|text| parse_layer(name, Some(path.display().to_string()), &text))
        })
        .collect()
}

#[cfg(test)]
mod unit {
    use super::*;

    fn layer(name: &str, json: &str) -> Layer {
        parse_layer(name, None, json)
    }

    #[test]
    fn later_layer_wins() {
        let layers = [layer("user", r#"{"a": 1}"#), layer("local", r#"{"a": 2}"#)];
        let (eff, prov) = effective_config(&layers);
        assert_eq!(eff.get("a"), Some(&Value::from(2)));
        assert_eq!(prov.get("a"), Some(&Value::String("local".into())));
    }

    #[test]
    fn empty_layers_contribute_nothing() {
        let layers = [layer("user", "{}"), layer("local", "not json")];
        let (eff, prov) = effective_config(&layers);
        assert!(eff.is_empty());
        assert!(prov.is_empty());
    }
}
