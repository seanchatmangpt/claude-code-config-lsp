//! Chicago-TDD over the praxis evidence layer: build -> sign -> verify
//! round-trip, tamper detection, and blake3 digest stability. Real data,
//! real hashing, no mocks.

use serde::{Deserialize, Serialize};

use praxis::{Admitted, AdmittedReceipt, Evidence};

/// A plain serializable payload standing in for conformance data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ConformancePayload {
    method: String,
    score: f64,
    violations: Vec<String>,
}

fn payload() -> ConformancePayload {
    ConformancePayload {
        method: "textDocument/hover".to_string(),
        score: 0.9,
        violations: vec![],
    }
}

fn evidence() -> Evidence<ConformancePayload, Admitted, ()> {
    // Fixed timestamp: digest stability requires byte-identical input.
    Evidence {
        data: payload(),
        status: std::marker::PhantomData,
        extension: std::marker::PhantomData,
        timestamp: "2026-11-03T00:00:00Z".to_string(),
    }
}

#[test]
fn build_sign_verify_round_trip() {
    let ev = evidence();
    let receipt = AdmittedReceipt::sign(ev.clone());
    assert_eq!(receipt.verdict, "ADMITTED");
    assert_eq!(receipt.proof, "praxis::admit[v1]");
    assert!(receipt.verify(&ev), "receipt must verify against its own evidence");
}

#[test]
fn tampered_payload_fails_verification() {
    let ev = evidence();
    let receipt = AdmittedReceipt::sign(ev.clone());

    // Flip a byte in the signed payload (score 0.9 -> 9.9).
    let mut tampered = ev.clone();
    tampered.data.score = 9.9;
    assert!(!receipt.verify(&tampered), "flipped byte in signed payload must fail verification");

    // Tampering the timestamp is equally detected.
    let mut tampered_ts = ev.clone();
    tampered_ts.timestamp = "2026-11-03T00:00:01Z".to_string();
    assert!(!receipt.verify(&tampered_ts));
}

#[test]
fn tampered_receipt_hash_fails_verification() {
    let ev = evidence();
    let mut receipt = AdmittedReceipt::sign(ev.clone());
    receipt.hash.truncate(receipt.hash.len() - 1);
    receipt.hash.push(if receipt.hash.ends_with('0') { '1' } else { '0' });
    assert!(!receipt.verify(&ev), "a mutated receipt hash must fail verification");
}

#[test]
fn wrong_proof_marker_fails_verification() {
    let ev = evidence();
    let mut receipt = AdmittedReceipt::sign(ev.clone());
    receipt.proof = "praxis::admit[v2]".to_string();
    assert!(!receipt.verify(&ev), "receipt with foreign proof marker must be refused");
}

#[test]
fn blake3_digest_is_stable_for_identical_input() {
    let a = AdmittedReceipt::sign(evidence());
    let b = AdmittedReceipt::sign(evidence());
    assert_eq!(a.hash, b.hash, "identical serialized evidence must hash identically");
}

#[test]
fn blake3_digest_distinguishes_differing_payloads() {
    let mut other = evidence();
    other.data.method = "textDocument/completion".to_string();
    let a = AdmittedReceipt::sign(evidence());
    let b = AdmittedReceipt::sign(other);
    assert_ne!(a.hash, b.hash);
}

#[test]
fn evidence_serialization_is_reproducible_json() {
    // The hash is over the canonical serde_json string; pin the exact bytes.
    let ev = evidence();
    let json = serde_json::to_string(&ev).expect("serialize");
    let expected = blake3::hash(json.as_bytes()).to_hex().to_string();
    let receipt = AdmittedReceipt::sign(ev.clone());
    assert_eq!(receipt.hash, expected);
    // And it round-trips through JSON without changing the digest.
    let decoded: Evidence<ConformancePayload, Admitted, ()> =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(serde_json::to_string(&decoded).expect("reserialize"), json);
}
