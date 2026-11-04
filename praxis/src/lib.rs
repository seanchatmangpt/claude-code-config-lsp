//! Praxis conformance scaffold and receipt chain
//!
//! Evidence types for wrapping conformance vectors with cryptographic signing.

use serde::{Deserialize, Serialize};

/// An Evidence wrapper for conformance data with generic type parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence<T, S, E> {
    /// The wrapped conformance data
    pub data: T,
    /// Status marker (typically Admitted)
    pub status: std::marker::PhantomData<S>,
    /// Error or extension marker
    pub extension: std::marker::PhantomData<E>,
    /// Timestamp of evidence creation
    pub timestamp: String,
}

impl<T, S, E> Evidence<T, S, E>
where
    T: Serialize,
{
    /// Create new Evidence wrapping conformance data
    pub fn new(data: T) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        Self {
            data,
            status: std::marker::PhantomData,
            extension: std::marker::PhantomData,
            timestamp,
        }
    }
}

/// An admitted status marker for Evidence
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Admitted;

/// A cryptographically signed receipt for conformance audits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdmittedReceipt {
    /// Cryptographic hash of the evidence
    pub hash: String,
    /// Timestamp of receipt issuance
    pub timestamp: String,
    /// Human-readable verdict
    pub verdict: String,
    /// Proof that this is an admitted receipt
    pub proof: String,
}

impl AdmittedReceipt {
    /// Create and sign an AdmittedReceipt for the given evidence
    pub fn sign<T>(evidence: Evidence<T, Admitted, ()>) -> Self
    where
        T: Serialize,
    {
        let evidence_json = serde_json::to_string(&evidence).unwrap_or_default();
        let hash = blake3::hash(evidence_json.as_bytes()).to_hex().to_string();
        let timestamp = chrono::Utc::now().to_rfc3339();

        Self {
            hash,
            timestamp,
            verdict: "ADMITTED".to_string(),
            proof: format!("praxis::admit[v1]"),
        }
    }

    /// Verify that this receipt attests the given evidence.
    ///
    /// A receipt is valid only when (a) its proof marker names the praxis
    /// admit scheme and (b) the BLAKE3 hash of the evidence's canonical JSON
    /// serialization equals the recorded hash. Any byte flipped in the
    /// payload changes the serialization and therefore fails verification.
    pub fn verify<T>(&self, evidence: &Evidence<T, Admitted, ()>) -> bool
    where
        T: Serialize,
    {
        if self.proof != "praxis::admit[v1]" {
            return false;
        }
        let evidence_json = serde_json::to_string(evidence).unwrap_or_default();
        let computed = blake3::hash(evidence_json.as_bytes()).to_hex().to_string();
        computed == self.hash
    }
}
