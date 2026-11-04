//! Chicago tests for the settings overlay merge view. Real JSON files in a
//! real tempdir, parsed and merged through the public API — no mocks. The
//! partial-overlay law is exercised with genuinely disjoint key subsets.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use claude_code_config_lsp::overlay::{
    effective_config, layers_from_paths, overlay_diagnostics, parse_layer, Severity,
};

/// Unique per-test temp workspace under the OS temp dir (no tempfile dep).
fn temp_workspace(tag: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "overlay-test-{}-{}-{}",
        tag,
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    std::fs::create_dir_all(&dir).expect("create temp workspace");
    dir
}

fn write_json(dir: &std::path::Path, rel: &str, text: &str) -> PathBuf {
    let path = dir.join(rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(&path, text).expect("write json");
    path
}

// ---------------------------------------------------------------------------
// Conflicting key across layers
// ---------------------------------------------------------------------------

#[test]
fn conflicting_key_across_layers_yields_override_conflict_naming_both() {
    let ws = temp_workspace("conflict");
    let user = write_json(&ws, "user.json", r#"{"model": "sonnet"}"#);
    let local = write_json(&ws, "local.json", r#"{"model": "opus"}"#);

    let layers = layers_from_paths(&[("user", &user), ("local", &local)]);
    let diags = overlay_diagnostics(&layers);

    let conflicts: Vec<_> = diags
        .iter()
        .filter(|d| d.code == "OVERRIDE_CONFLICT")
        .collect();
    assert_eq!(conflicts.len(), 1, "exactly one conflict: {diags:?}");
    let d = conflicts[0];
    assert_eq!(d.severity, Severity::Warning);
    assert_eq!(d.key, "model");
    // Message names both layers AND both values.
    assert!(
        d.message.contains("user"),
        "names losing layer: {}",
        d.message
    );
    assert!(
        d.message.contains("local"),
        "names winning layer: {}",
        d.message
    );
    assert!(
        d.message.contains("sonnet"),
        "names old value: {}",
        d.message
    );
    assert!(d.message.contains("opus"), "names new value: {}", d.message);

    // Effective value is the winner.
    let (eff, prov) = effective_config(&layers);
    assert_eq!(eff.get("model").and_then(|v| v.as_str()), Some("opus"));
    assert_eq!(prov.get("model").and_then(|v| v.as_str()), Some("local"));
}

#[test]
fn same_key_same_value_across_layers_is_not_a_conflict() {
    let ws = temp_workspace("agree");
    let user = write_json(&ws, "user.json", r#"{"model": "sonnet"}"#);
    let project = write_json(&ws, "project.json", r#"{"model": "sonnet"}"#);

    let layers = layers_from_paths(&[("user", &user), ("project", &project)]);
    let diags = overlay_diagnostics(&layers);
    assert!(
        diags.iter().all(|d| d.code != "OVERRIDE_CONFLICT"),
        "identical values must not conflict: {diags:?}"
    );
    // Provenance still resolves to the later (higher-precedence) layer.
    let (_eff, prov) = effective_config(&layers);
    assert_eq!(prov.get("model").and_then(|v| v.as_str()), Some("project"));
}

// ---------------------------------------------------------------------------
// Disjoint overlays (partial-overlay law)
// ---------------------------------------------------------------------------

#[test]
fn disjoint_overlays_merge_without_diagnostics() {
    let ws = temp_workspace("disjoint");
    let user = write_json(&ws, "user.json", r#"{"model": "sonnet", "theme": "dark"}"#);
    let project = write_json(
        &ws,
        "project.json",
        r#"{"permissions": {"allow": ["Bash"]}}"#,
    );
    let local = write_json(&ws, "local.json", r#"{"statusLine": {"type": "command"}}"#);

    let layers = layers_from_paths(&[("user", &user), ("project", &project), ("local", &local)]);
    let diags = overlay_diagnostics(&layers);
    assert!(
        diags.is_empty(),
        "disjoint keys are never diagnosed: {diags:?}"
    );

    let (eff, prov) = effective_config(&layers);
    assert_eq!(eff.len(), 4);
    assert_eq!(prov.get("model").and_then(|v| v.as_str()), Some("user"));
    assert_eq!(
        prov.get("permissions").and_then(|v| v.as_str()),
        Some("project")
    );
    assert_eq!(
        prov.get("statusLine").and_then(|v| v.as_str()),
        Some("local")
    );
}

#[test]
fn missing_layers_and_missing_keys_are_normal() {
    // Only the local layer exists — a legal minimal install.
    let ws = temp_workspace("only-local");
    let local = write_json(&ws, "local.json", r#"{"model": "opus"}"#);
    let layers = layers_from_paths(&[("user", &ws.join("does-not-exist.json")), ("local", &local)]);
    assert_eq!(layers.len(), 1, "missing layers are skipped, not errors");
    let (eff, prov) = effective_config(&layers);
    assert_eq!(eff.get("model").and_then(|v| v.as_str()), Some("opus"));
    assert_eq!(prov.get("model").and_then(|v| v.as_str()), Some("local"));
    assert!(overlay_diagnostics(&layers).is_empty());
}

#[test]
fn malformed_layer_file_is_tolerated_not_poisoning() {
    let ws = temp_workspace("broken");
    let good = write_json(&ws, "good.json", r#"{"model": "sonnet"}"#);
    let bad = write_json(&ws, "bad.json", "{ truncated");
    let layers = layers_from_paths(&[("user", &good), ("local", &bad)]);
    let (eff, prov) = effective_config(&layers);
    assert_eq!(eff.get("model").and_then(|v| v.as_str()), Some("sonnet"));
    assert_eq!(prov.get("model").and_then(|v| v.as_str()), Some("user"));
}

// ---------------------------------------------------------------------------
// Three-layer precedence
// ---------------------------------------------------------------------------

#[test]
fn three_layer_precedence_project_beats_user_local_beats_project() {
    let ws = temp_workspace("precedence");
    let managed = write_json(&ws, "managed.json", r#"{"model": "haiku"}"#);
    let user = write_json(&ws, "user.json", r#"{"model": "sonnet"}"#);
    let project = write_json(&ws, "project.json", r#"{"model": "opus"}"#);
    let local = write_json(&ws, "local.json", r#"{"model": "fable"}"#);

    let layers = layers_from_paths(&[
        ("managed", &managed),
        ("user", &user),
        ("project", &project),
        ("local", &local),
    ]);
    let (eff, prov) = effective_config(&layers);
    assert_eq!(eff.get("model").and_then(|v| v.as_str()), Some("fable"));
    assert_eq!(prov.get("model").and_then(|v| v.as_str()), Some("local"));

    // One conflict spans the whole chain: lowest-precedence holder vs winner.
    let conflicts: Vec<_> = overlay_diagnostics(&layers)
        .into_iter()
        .filter(|d| d.code == "OVERRIDE_CONFLICT")
        .collect();
    assert_eq!(conflicts.len(), 1);
    assert!(conflicts[0].message.contains("managed"));
    assert!(conflicts[0].message.contains("local"));
}

// ---------------------------------------------------------------------------
// LOCAL_SHADOW: local overrides project settings.json
// ---------------------------------------------------------------------------

#[test]
fn local_shadow_info_when_local_overrides_project_settings() {
    let ws = temp_workspace("shadow");
    let project = write_json(&ws, "project.json", r#"{"model": "opus"}"#);
    let local = write_json(&ws, "local.json", r#"{"model": "opus"}"#);

    let layers = layers_from_paths(&[("project", &project), ("local", &local)]);
    let diags = overlay_diagnostics(&layers);
    let shadows: Vec<_> = diags.iter().filter(|d| d.code == "LOCAL_SHADOW").collect();
    assert_eq!(
        shadows.len(),
        1,
        "shadow fires even when values agree: {diags:?}"
    );
    assert_eq!(shadows[0].severity, Severity::Information);
    assert_eq!(shadows[0].key, "model");

    // Identical values → shadow info but NO conflict warning.
    assert!(diags.iter().all(|d| d.code != "OVERRIDE_CONFLICT"));
}

// ---------------------------------------------------------------------------
// .zcode parallel-root tolerance
// ---------------------------------------------------------------------------

#[test]
fn zcode_parallel_root_uses_same_grammar() {
    // A parallel .zcode root: same file grammar, different directory.
    let ws = temp_workspace("zcode");
    let user = write_json(&ws, ".zcode/settings.json", r#"{"model": "sonnet"}"#);
    let project = write_json(&ws, "proj/.zcode/settings.json", r#"{"model": "fable"}"#);

    let layers = layers_from_paths(&[("user", &user), ("project", &project)]);
    let (eff, prov) = effective_config(&layers);
    assert_eq!(eff.get("model").and_then(|v| v.as_str()), Some("fable"));
    assert_eq!(prov.get("model").and_then(|v| v.as_str()), Some("project"));

    let diags = overlay_diagnostics(&layers);
    assert_eq!(
        diags
            .iter()
            .filter(|d| d.code == "OVERRIDE_CONFLICT")
            .count(),
        1,
        "same grammar detects conflicts under parallel roots: {diags:?}"
    );
    assert!(diags[0].message.contains("fable"));
}

// ---------------------------------------------------------------------------
// Layer identity survives parsing (provenance grounding)
// ---------------------------------------------------------------------------

#[test]
fn parse_layer_records_name_and_path() {
    let layer = parse_layer(
        "local",
        Some("/tmp/x/settings.local.json".into()),
        r#"{"a":1}"#,
    );
    assert_eq!(layer.name, "local");
    assert_eq!(layer.path.as_deref(), Some("/tmp/x/settings.local.json"));
    assert_eq!(layer.settings.get("a"), Some(&serde_json::Value::from(1)));
}
