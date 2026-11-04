//! Chicago-TDD over the untested evidence layer: receipt chain, conformance
//! vector, coverage census, Declare model. Real files in a real tempdir,
//! real hashes — no mocks. All access via public APIs; the four src modules
//! are generated code owned by another wave and are not edited here.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use claude_code_config_lsp::conformance::conformance_vector;
use claude_code_config_lsp::coverage::{coverage_report, CoverageReport};
use claude_code_config_lsp::declare_model::{declare_model, ConstraintType};
use claude_code_config_lsp::receipt::{
    build_chain, compute_receipt_hash, find_latest_receipt, get_receipts_dir, read_all_receipts,
    verify_receipt_chain, FileChecksum, Receipt,
};

/// Unique per-test temp workspace under the OS temp dir (no tempfile dep).
fn temp_workspace(tag: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "c6-receipt-layer-{}-{}-{}",
        tag,
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    std::fs::create_dir_all(&dir).expect("create temp workspace");
    dir
}

/// Write a receipt file named `<hash>.json` with hash over the given fields.
fn write_receipt(
    dir: &std::path::Path,
    timestamp: chrono::DateTime<chrono::Utc>,
    score: f64,
    checksums: Vec<FileChecksum>,
    prev: Option<String>,
) -> Receipt {
    let hash = compute_receipt_hash(&timestamp, score, &checksums, &prev);
    let receipt = Receipt {
        timestamp,
        conformance_score: score,
        file_checksums: checksums,
        previous_receipt_hash: prev,
        hash: hash.clone(),
    };
    let path = dir.join(format!("{hash}.json"));
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&receipt).expect("serialize"),
    )
    .expect("write receipt");
    receipt
}

// ---------------------------------------------------------------------------
// Receipt chain: genesis + append + verify
// ---------------------------------------------------------------------------

#[test]
fn genesis_receipt_round_trips_through_disk() {
    let ws = temp_workspace("genesis");
    let dir = get_receipts_dir(&ws);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let t0 = chrono::Utc::now();
    write_receipt(&dir, t0, 1.0, vec![], None);

    let receipts = read_all_receipts(&ws).expect("read receipts");
    assert_eq!(receipts.len(), 1);
    assert_eq!(receipts[0].conformance_score, 1.0);
    assert_eq!(receipts[0].previous_receipt_hash, None);
    std::fs::remove_dir_all(&ws).ok();
}

#[test]
fn chain_build_appends_second_receipt_and_orders_genesis_first() {
    let ws = temp_workspace("chain");
    let dir = get_receipts_dir(&ws);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let t0 = chrono::Utc::now();
    let genesis = write_receipt(&dir, t0, 1.0, vec![], None);
    let child = write_receipt(&dir, t0, 0.8, vec![], Some(genesis.hash.clone()));

    let receipts = read_all_receipts(&ws).expect("read receipts");
    assert_eq!(receipts.len(), 2);

    let chain = build_chain(&receipts).expect("build chain");
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0].hash, genesis.hash, "genesis must come first");
    assert_eq!(chain[1].hash, child.hash, "child must come last");

    // The child is the head: nothing references it.
    let latest = find_latest_receipt(&receipts).expect("latest");
    assert_eq!(latest.hash, child.hash);
    std::fs::remove_dir_all(&ws).ok();
}

#[test]
fn tampered_prior_receipt_fails_read() {
    let ws = temp_workspace("tamper-content");
    let dir = get_receipts_dir(&ws);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let t0 = chrono::Utc::now();
    let genesis = write_receipt(&dir, t0, 1.0, vec![], None);
    write_receipt(&dir, t0, 0.8, vec![], Some(genesis.hash.clone()));

    // Flip a byte in the PRIOR (genesis) receipt's stored score: 1.0 -> 9.0.
    let genesis_path = dir.join(format!("{}.json", genesis.hash));
    let content = std::fs::read_to_string(&genesis_path).expect("read genesis");
    let tampered = content.replace("\"conformance_score\": 1.0", "\"conformance_score\": 9.0");
    assert_ne!(tampered, content, "tamper must actually change the file");
    std::fs::write(&genesis_path, tampered).expect("write tampered");

    let err = read_all_receipts(&ws).expect_err("tampering must be detected");
    assert!(err.contains("Tampering detected"), "got: {err}");
    std::fs::remove_dir_all(&ws).ok();
}

#[test]
fn renamed_receipt_file_fails_filename_check() {
    let ws = temp_workspace("tamper-filename");
    let dir = get_receipts_dir(&ws);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let t0 = chrono::Utc::now();
    let genesis = write_receipt(&dir, t0, 1.0, vec![], None);

    // Rename genesis file so the stem no longer matches its hash.
    let from = dir.join(format!("{}.json", genesis.hash));
    let to = dir.join("deadbeef.json");
    std::fs::rename(&from, &to).expect("rename");
    let err = read_all_receipts(&ws).expect_err("filename mismatch must be detected");
    assert!(err.contains("Filename mismatch"), "got: {err}");
    std::fs::remove_dir_all(&ws).ok();
}

#[test]
fn chain_with_dangling_previous_hash_is_broken() {
    let ws = temp_workspace("dangling");
    let dir = get_receipts_dir(&ws);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let t0 = chrono::Utc::now();
    write_receipt(
        &dir,
        t0,
        0.5,
        vec![],
        Some("nonexistent-parent".to_string()),
    );

    let receipts = read_all_receipts(&ws).expect("read receipts");
    let err = build_chain(&receipts).expect_err("dangling parent must break the chain");
    assert!(err.contains("Broken chain"), "got: {err}");
    std::fs::remove_dir_all(&ws).ok();
}

#[test]
fn empty_workspace_has_no_receipts_and_empty_chain() {
    let ws = temp_workspace("empty");
    let dir = get_receipts_dir(&ws);
    std::fs::create_dir_all(&dir).expect("mkdir");
    assert!(read_all_receipts(&ws).expect("read").is_empty());
    assert!(build_chain(&[]).expect("chain").is_empty());
    std::fs::remove_dir_all(&ws).ok();
}

/// End-to-end against the real scanner: issue genesis via the public issuer,
/// verify passes on an unchanged workspace, then modifying a config file
/// breaks verification.
#[test]
fn verify_receipt_chain_detects_workspace_drift() {
    let ws = temp_workspace("drift");
    let dir = get_receipts_dir(&ws);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let config = ws.join("CLAUDE.md");
    std::fs::write(&config, "# root\n").expect("seed config");

    // Genesis receipt over the current workspace.
    let issued = claude_code_config_lsp::receipt::maybe_issue_receipt(&ws, 1.0)
        .expect("issue genesis")
        .expect("genesis must be issued on empty chain");
    verify_receipt_chain(&ws).expect("unchanged workspace must verify");

    // Drift: mutate the tracked config file.
    std::fs::write(&config, "# root\n\n## changed\n").expect("mutate config");
    let err = verify_receipt_chain(&ws).expect_err("workspace drift must fail verification");
    assert!(
        err.contains("do not match the latest receipt"),
        "got: {err}"
    );
    std::fs::remove_dir_all(&ws).ok();
    drop(issued);
}

// ---------------------------------------------------------------------------
// ConformanceVector admit/refuse tracking
// ---------------------------------------------------------------------------

#[test]
fn conformance_vector_admits_core_lsp_methods() {
    let cv = conformance_vector();
    fn label(a: &lsp_max::max_protocol::LawAxis) -> String {
        match a {
            lsp_max::max_protocol::LawAxis::Custom(s) => s.clone(),
            other => format!("{other:?}"),
        }
    }
    let admitted: Vec<String> = cv.admitted.iter().map(label).collect();
    for m in [
        "textDocument/didOpen",
        "textDocument/didChange",
        "textDocument/hover",
        "textDocument/completion",
    ] {
        assert!(admitted.contains(&m.to_string()), "{m} must be admitted");
    }
    // Refused explicitly lists unimplemented methods, and the sets are disjoint.
    let refused: Vec<String> = cv.refused.iter().map(label).collect();
    assert!(refused.contains(&"textDocument/codeAction".to_string()));
    for m in &admitted {
        assert!(
            !refused.contains(m),
            "{m} cannot be both admitted and refused"
        );
    }
    assert!(!cv.unknown.iter().any(|u| admitted.contains(&label(u))));
}

#[test]
fn conformance_vector_partition_is_exhaustive_and_disjoint() {
    let cv = conformance_vector();
    let keys = |v: &[lsp_max::max_protocol::LawAxis]| -> Vec<String> {
        v.iter()
            .map(|a| match a {
                lsp_max::max_protocol::LawAxis::Custom(s) => s.clone(),
                other => format!("{other:?}"),
            })
            .collect()
    };
    let (a, r, u) = (keys(&cv.admitted), keys(&cv.refused), keys(&cv.unknown));
    for x in &a {
        assert!(!r.contains(x) && !u.contains(x));
    }
    for x in &r {
        assert!(!u.contains(x));
    }
    // No duplicates within a category.
    for v in [&a, &r, &u] {
        let mut s = v.clone();
        s.sort();
        s.dedup();
        assert_eq!(s.len(), v.len(), "duplicate law axis within one category");
    }
}

// ---------------------------------------------------------------------------
// Coverage census
// ---------------------------------------------------------------------------

#[test]
fn coverage_census_counts_the_full_partition() {
    let report = coverage_report();
    let cv = conformance_vector();
    assert_eq!(report.admitted, cv.admitted.len());
    assert_eq!(report.refused, cv.refused.len());
    assert_eq!(report.unknown, cv.unknown.len());
    assert_eq!(
        report.total(),
        report.admitted + report.refused + report.unknown
    );
}

#[test]
fn coverage_percent_and_markdown_render() {
    let report = coverage_report();
    let expected = ((report.admitted as f64 / report.total() as f64) * 1000.0).round() / 10.0;
    assert_eq!(report.coverage_percent(), expected);
    assert!(report.coverage_percent() > 0.0 && report.coverage_percent() < 100.0);
    let md = report.render_markdown();
    assert!(md.contains(&format!("{}/{}", report.admitted, report.total())));
    assert!(md.contains("Admitted (implemented)"));
    assert!(md.contains("Refused (explicitly unimplemented)"));
    assert!(md.contains("Unknown (undetermined)"));
}

#[test]
fn empty_coverage_report_is_zero() {
    let r = CoverageReport {
        admitted: 0,
        refused: 0,
        unknown: 0,
    };
    assert_eq!(r.coverage_percent(), 0.0);
    assert_eq!(r.total(), 0);
}

// ---------------------------------------------------------------------------
// Declare model
// ---------------------------------------------------------------------------

#[test]
fn declare_model_holds_four_well_formed_constraints() {
    let model = declare_model();
    assert_eq!(model.name, "server_process_model");
    assert_eq!(model.constraints.len(), 4);
    // Response/Precedence constraints need both antecedent and consequent.
    for c in &model.constraints {
        assert!(!c.antecedent.is_empty());
        assert!(
            c.consequent.is_some(),
            "{:?} constraint {:?} lacks a consequent",
            c.constraint_type,
            c.antecedent
        );
    }
    assert!(model
        .constraints
        .iter()
        .any(|c| c.constraint_type == ConstraintType::Precedence));
    assert_eq!(
        model
            .constraints
            .iter()
            .filter(|c| c.constraint_type == ConstraintType::Response)
            .count(),
        3
    );
}

#[test]
fn constraint_type_parses_known_and_unknown_names() {
    for (s, expected) in [
        ("Init", ConstraintType::Init),
        ("End", ConstraintType::End),
        ("Response", ConstraintType::Response),
        ("Precedence", ConstraintType::Precedence),
        ("ExactlyOne", ConstraintType::ExactlyOne),
        ("NotCoExistence", ConstraintType::NotCoExistence),
        ("RespondedExistence", ConstraintType::RespondedExistence),
        ("Absence", ConstraintType::Absence),
        ("ChainResponse", ConstraintType::ChainResponse),
    ] {
        assert_eq!(ConstraintType::from(s), expected);
    }
    assert_eq!(
        ConstraintType::from("SomethingElse"),
        ConstraintType::Other("SomethingElse".to_string())
    );
}
