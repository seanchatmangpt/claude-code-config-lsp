//! Acceptance tests for the ORIGINAL_REQUEST conformance/receipt/fix CLI verbs,
//! driven against the REAL compiled binary over real temp workspaces.
//! Chicago-style: real collaborators, assert on final on-disk state and real
//! exit codes — no mocks.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

static TMP_COUNTER: AtomicU32 = AtomicU32::new(0);

/// A real temp workspace: unique dir under std::env::temp_dir(), deleted on drop.
struct TmpWorkspace {
    root: PathBuf,
}

impl TmpWorkspace {
    fn new(label: &str) -> Self {
        let n = TMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let root = std::env::temp_dir().join(format!(
            "ccc-acceptance-{}-{}-{}",
            label,
            std::process::id(),
            n
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join(".claude")).expect("create workspace dir");
        TmpWorkspace { root }
    }

    fn path(&self) -> String {
        self.root.to_string_lossy().to_string()
    }

    fn bin(&self) -> Command {
        Command::new(env!("CARGO_BIN_EXE_claude-code-config-lsp"))
    }
}

impl Drop for TmpWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// Write a minimal clean settings.json (with $schema) into the workspace.
fn write_clean_settings(ws: &TmpWorkspace) {
    fs::write(
        ws.root.join(".claude/settings.json"),
        concat!(
            "{\n",
            "  \"$schema\": \"https://raw.githubusercontent.com/anthropics/claude-code/refs/heads/main/.claude/settings.schema.json\",\n",
            "  \"permissions\": {\"allow\": [\"Bash(ls:*)\"]}\n",
            "}\n"
        ),
    )
    .expect("write settings.json");
}

/// Write a settings.json missing $schema (the repairable defect `fix` targets).
fn write_settings_missing_schema(ws: &TmpWorkspace) {
    fs::write(
        ws.root.join(".claude/settings.json"),
        "{\n  \"permissions\": {\"allow\": [\"Bash(ls:*)\"]}\n}\n",
    )
    .expect("write settings.json");
}

/// Run `conformance <ws>` so `maybe_issue_receipt` writes a genesis receipt.
fn issue_genesis_receipt(ws: &TmpWorkspace) {
    let out = ws
        .bin()
        .args(["conformance", &ws.path()])
        .output()
        .expect("run conformance");
    assert!(
        out.status.success(),
        "conformance failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let count = ws
        .root
        .join(".claude/receipts")
        .read_dir()
        .map(|d| d.count())
        .unwrap_or(0);
    assert!(count > 0, "no receipt issued on first conformance run");
}

// ---------------------------------------------------------------------------
// R1: conformance --format json|yaml|table
// ---------------------------------------------------------------------------

#[test]
fn conformance_json_format_outputs_health_vector_shape() {
    let ws = TmpWorkspace::new("fmt-json");
    write_clean_settings(&ws);
    let out = ws
        .bin()
        .args(["conformance", &ws.path(), "--format", "json"])
        .output()
        .expect("run conformance");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("stdout must be valid JSON");
    assert!(parsed["score"].is_number(), "score missing in: {}", stdout);
    assert!(parsed["surfaces_checked"].is_number());
    assert!(parsed["surfaces_admitted"].is_number());
    assert!(parsed["violations"].is_array());
}

#[test]
fn conformance_yaml_format_outputs_key_lines_with_quoted_violations() {
    let ws = TmpWorkspace::new("fmt-yaml");
    write_clean_settings(&ws);
    // Also add an agent file so the violations array is exercised non-empty.
    let agents = ws.root.join(".claude/agents");
    fs::create_dir_all(&agents).expect("create agents dir");
    fs::write(
        agents.join("worker.md"),
        "---\nmodel: gpt-4o\nname: worker\n---\n\nbody\n",
    )
    .expect("write agent fixture");
    let out = ws
        .bin()
        .args(["conformance", &ws.path(), "--format", "yaml"])
        .output()
        .expect("run conformance");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.starts_with("score: "), "yaml shape: {}", stdout);
    assert!(stdout.contains("surfaces_checked: "));
    assert!(stdout.contains("violations:\n"));
    assert!(
        stdout.contains("\n  - \"CCC-"),
        "quoted violation list: {}",
        stdout
    );
}

#[test]
fn conformance_table_format_outputs_aligned_columns() {
    let ws = TmpWorkspace::new("fmt-table");
    write_clean_settings(&ws);
    let out = ws
        .bin()
        .args(["conformance", &ws.path(), "--format", "table"])
        .output()
        .expect("run conformance");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    for key in [
        "score",
        "surfaces_checked",
        "surfaces_admitted",
        "violations",
    ] {
        assert!(
            stdout
                .lines()
                .any(|l| l.starts_with(key) && !l.contains(':')),
            "table summary row for {} missing in:\n{}",
            key,
            stdout
        );
    }
}

#[test]
fn conformance_table_violation_rows_are_column_aligned() {
    let ws = TmpWorkspace::new("fmt-table-align");
    write_clean_settings(&ws);
    let agents = ws.root.join(".claude/agents");
    fs::create_dir_all(&agents).expect("create agents dir");
    fs::write(
        agents.join("worker.md"),
        "---\nmodel: gpt-4o\nname: worker\n---\n\nbody\n",
    )
    .expect("write agent fixture");
    let out = ws
        .bin()
        .args(["conformance", &ws.path(), "--format", "table"])
        .output()
        .expect("run conformance");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let rows: Vec<&str> = stdout.lines().filter(|l| l.starts_with("CCC-")).collect();
    assert!(!rows.is_empty(), "expected violation rows:\n{}", stdout);
    // Column alignment: every data row's file path begins at the same offset.
    let offsets: Vec<usize> = rows
        .iter()
        .map(|r| r.len() - r.trim_start().len())
        .collect();
    assert!(
        offsets.iter().all(|&o| o == offsets[0]),
        "violation rows not column-aligned:\n{}",
        stdout
    );
}

#[test]
fn conformance_unknown_format_fails_loudly() {
    let ws = TmpWorkspace::new("fmt-bad");
    write_clean_settings(&ws);
    let out = ws
        .bin()
        .args(["conformance", &ws.path(), "--format", "csv"])
        .output()
        .expect("run conformance");
    assert!(!out.status.success(), "csv format must be refused");
    assert!(String::from_utf8_lossy(&out.stderr).contains("unsupported --format"));
}

// ---------------------------------------------------------------------------
// R2: receipt chain | verify
// ---------------------------------------------------------------------------

#[test]
fn receipt_verify_succeeds_on_valid_chain() {
    let ws = TmpWorkspace::new("receipt-valid");
    write_clean_settings(&ws);
    issue_genesis_receipt(&ws);
    let out = ws
        .bin()
        .args(["receipt", "verify", &ws.path()])
        .output()
        .expect("run receipt verify");
    assert!(
        out.status.success(),
        "verify failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stdout).contains("Verification succeeded"));
}

#[test]
fn receipt_verify_fails_when_workspace_config_is_tampered() {
    let ws = TmpWorkspace::new("receipt-tamper-config");
    write_clean_settings(&ws);
    issue_genesis_receipt(&ws);
    // Mutate one byte of the workspace config: latest-receipt checksums no
    // longer match the workspace on disk.
    let settings = ws.root.join(".claude/settings.json");
    let raw = fs::read_to_string(&settings).expect("read settings");
    fs::write(&settings, raw.replace("Bash(ls:*)", "Bash(rm:*)")).expect("tamper settings");
    let out = ws
        .bin()
        .args(["receipt", "verify", &ws.path()])
        .output()
        .expect("run receipt verify");
    assert!(
        !out.status.success(),
        "verify must fail on tampered workspace config"
    );
}

#[test]
fn receipt_verify_fails_when_receipt_byte_is_mutated() {
    let ws = TmpWorkspace::new("receipt-tamper-receipt");
    write_clean_settings(&ws);
    issue_genesis_receipt(&ws);
    // Mutate one byte inside the receipt file itself: its stored hash no longer
    // matches the recomputed BLAKE3 diagnostic hash.
    let dir = ws.root.join(".claude/receipts");
    let receipt_path = fs::read_dir(&dir)
        .expect("read receipts dir")
        .next()
        .expect("genesis receipt exists")
        .expect("dir entry")
        .path();
    let raw = fs::read_to_string(&receipt_path).expect("read receipt");
    let mutated = raw.replace("conformance_score", "CONFORMANCE_SCORE");
    assert_ne!(raw, mutated, "mutation must change the receipt bytes");
    fs::write(&receipt_path, mutated).expect("write mutated receipt");
    let out = ws
        .bin()
        .args(["receipt", "verify", &ws.path()])
        .output()
        .expect("run receipt verify");
    assert!(
        !out.status.success(),
        "verify must fail on mutated receipt file"
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("Verification failed"));
}

#[test]
fn receipt_chain_lists_snapshots_genesis_to_latest() {
    let ws = TmpWorkspace::new("receipt-chain");
    write_clean_settings(&ws);
    issue_genesis_receipt(&ws);
    let out = ws
        .bin()
        .args(["receipt", "chain", &ws.path()])
        .output()
        .expect("run receipt chain");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("chain stdout must be JSON");
    let arr = parsed.as_array().expect("chain is an array");
    assert!(
        !arr.is_empty(),
        "chain must list at least the genesis snapshot"
    );
    let genesis = &arr[0];
    assert!(
        genesis["previous_receipt_hash"].is_null(),
        "genesis has no prior hash"
    );
    assert!(!genesis["hash"].as_str().unwrap_or("").is_empty());
}

// ---------------------------------------------------------------------------
// R3: fix (dry-run default) + --apply / -y
// ---------------------------------------------------------------------------

#[test]
fn fix_dry_run_shows_proposals_without_writing() {
    let ws = TmpWorkspace::new("fix-dry");
    write_settings_missing_schema(&ws);
    let before = fs::read_to_string(ws.root.join(".claude/settings.json")).expect("read before");
    let out = ws
        .bin()
        .args(["fix", &ws.path()])
        .output()
        .expect("run fix dry-run");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("dry_run"), "report shape:\n{}", stdout);
    assert!(
        stdout.contains("$schema"),
        "proposal mentions the fix:\n{}",
        stdout
    );
    let after = fs::read_to_string(ws.root.join(".claude/settings.json")).expect("read after");
    assert_eq!(before, after, "dry-run must NOT modify the file");
}

#[test]
fn fix_apply_writes_schema_key_to_disk() {
    let ws = TmpWorkspace::new("fix-apply");
    write_settings_missing_schema(&ws);
    let out = ws
        .bin()
        .args(["fix", &ws.path(), "--apply"])
        .output()
        .expect("run fix --apply");
    assert!(out.status.success());
    let after = fs::read_to_string(ws.root.join(".claude/settings.json")).expect("read after");
    assert!(
        after.contains("$schema"),
        "--apply must write the fix:\n{}",
        after
    );
    // The written file must still be valid JSON.
    serde_json::from_str::<serde_json::Value>(&after).expect("applied file is valid JSON");
}

#[test]
fn fix_short_y_flag_applies_like_apply() {
    let ws = TmpWorkspace::new("fix-y");
    write_settings_missing_schema(&ws);
    let out = ws
        .bin()
        .args(["fix", &ws.path(), "-y"])
        .output()
        .expect("run fix -y");
    assert!(
        out.status.success(),
        "-y must be accepted: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let after = fs::read_to_string(ws.root.join(".claude/settings.json")).expect("read after");
    assert!(
        after.contains("$schema"),
        "-y must write the fix:\n{}",
        after
    );
}

#[test]
fn fix_on_clean_workspace_proposes_nothing() {
    let ws = TmpWorkspace::new("fix-clean");
    write_clean_settings(&ws);
    let out = ws
        .bin()
        .args(["fix", &ws.path()])
        .output()
        .expect("run fix");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("fix report must be JSON");
    assert_eq!(parsed["proposals"].as_array().map(|a| a.len()), Some(0));
    assert_eq!(parsed["applied"], 0);
}
