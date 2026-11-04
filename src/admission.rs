//! Compiler-backed admission controller — the "reverse 80/20" blueprint.
//!
//! Three moving parts, all importing the fleet's landed modules (no sibling
//! edits):
//!
//! 1. [`blast_radius`] — transitive incoming-call closure (via
//!    [`crate::hierarchy::incoming_calls`], depth-capped, cycle-guarded) plus
//!    outgoing closure (via [`crate::hierarchy::outgoing_calls`]) over a
//!    config corpus. Returns the exact file/symbol slice set an edit would
//!    touch.
//! 2. [`admit_edit`] — the gate: REFUSED when the radius intersects a
//!    declared critical path (protected path prefixes), PARTIAL when the
//!    radius is non-empty (the minimal backward/forward slices ride along as
//!    the context payload), ALIVE when the radius is empty.
//! 3. [`AdmissionJournal`] — every decision emits a `ggen.admission-transition/1`
//!    record (schema, decision, blake3 blast-radius digest, timestamp) into an
//!    in-memory journal with prev-hash chaining; [`AdmissionJournal::verify_chain`]
//!    detects any post-hoc tampering.
//!
//! Plus [`ground`] — delta grounding: maps a P8 [`SemanticTokensDelta`] into
//! compact agent-readable classifications using the semantic-token legend.
//!
//! All functions are pure over an in-memory [`Corpus`] — the real fixture
//! tree on disk is the Chicago-test collaborator, per repo convention.

use crate::hierarchy::{self, Corpus};
use crate::semantic_tokens;
use lsp_max::lsp_types::SemanticTokensDelta;
use lsp_types_max::Range;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Blast radius
// ---------------------------------------------------------------------------

/// One file/symbol slice: a document in the radius, its display symbol, and
/// the exact reference ranges that put it in the radius.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceEntry {
    pub path: PathBuf,
    pub symbol: String,
    pub ranges: Vec<Range>,
}

impl SliceEntry {
    /// Compact one-line digest for agent context payloads.
    pub fn compact(&self) -> String {
        let mut lines: Vec<String> = self
            .ranges
            .iter()
            .map(|r| {
                format!(
                    "{}:{}-{}:{}",
                    self.path.display(),
                    r.start.line,
                    r.start.character,
                    r.end.character
                )
            })
            .collect();
        lines.sort();
        lines.dedup();
        format!("{} :: {}", self.symbol, lines.join(", "))
    }
}

/// The transitive incoming + outgoing closure of a document under edit.
#[derive(Debug, Clone, PartialEq)]
pub struct BlastRadius {
    /// The document under edit.
    pub root: PathBuf,
    /// Display symbol of the root (frontmatter name or file stem).
    pub root_symbol: String,
    /// Backward slice: documents that transitively reference the root.
    pub incoming: Vec<SliceEntry>,
    /// Forward slice: documents the root transitively references.
    pub outgoing: Vec<SliceEntry>,
}

impl BlastRadius {
    /// True when no document lies in either slice.
    pub fn is_empty(&self) -> bool {
        self.incoming.is_empty() && self.outgoing.is_empty()
    }

    /// Total number of slice entries (both directions).
    pub fn len(&self) -> usize {
        self.incoming.len() + self.outgoing.len()
    }

    /// Deterministic digest input: sorted compact lines of every slice entry.
    fn canonical_form(&self) -> String {
        let mut lines: Vec<String> = self
            .incoming
            .iter()
            .chain(self.outgoing.iter())
            .map(SliceEntry::compact)
            .collect();
        lines.sort();
        lines.dedup();
        format!("root={};{}", self.root.display(), lines.join("\n"))
    }

    /// blake3 hex digest of the canonical radius form — the receipt's
    /// consequence fingerprint.
    pub fn digest(&self) -> String {
        hex(blake3::hash(self.canonical_form().as_bytes()).as_bytes())
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Default recursion cap for both closures.
pub const MAX_DEPTH: usize = 8;

/// Display symbol of a corpus doc: frontmatter name when present, else stem.
fn symbol_of(path: &Path, text: &str) -> String {
    hierarchy::frontmatter_name(text)
        .map(|(n, _)| n)
        .unwrap_or_else(|| {
            path.file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default()
        })
}

/// Transitive incoming closure (backward slice) of `doc_path`'s symbol.
/// Each hop resolves the referencing doc's own symbol and recurses, capped at
/// `max_depth`, cycle-guarded by canonical path string.
fn incoming_closure(corpus: &Corpus, doc_path: &Path, max_depth: usize) -> Vec<SliceEntry> {
    let mut visited: BTreeSet<String> = BTreeSet::new();
    visited.insert(canonical(doc_path));
    let mut out: Vec<SliceEntry> = Vec::new();
    let mut frontier: Vec<(PathBuf, String)> = corpus
        .iter()
        .find(|(p, _)| p == doc_path)
        .map(|(p, t)| (p.clone(), symbol_of(p, t)))
        .into_iter()
        .collect();

    let mut depth = 0usize;
    while !frontier.is_empty() && depth < max_depth {
        let mut next: Vec<(PathBuf, String)> = Vec::new();
        for (_, name) in &frontier {
            for edge in hierarchy::incoming_calls(corpus, name) {
                let target = PathBuf::from(edge.item.detail.as_deref().unwrap_or(""));
                let key = canonical(&target);
                if visited.contains(&key) {
                    continue; // cycle guard: root + already-visited nodes
                }
                visited.insert(key.clone());
                let symbol = edge.item.name.clone();
                let text = corpus
                    .iter()
                    .find(|(p, _)| canonical(p) == key)
                    .map(|(_, t)| t.clone())
                    .unwrap_or_default();
                next.push((PathBuf::from(&key), symbol_of(&target, &text)));
                out.push(SliceEntry {
                    path: PathBuf::from(key),
                    symbol,
                    ranges: edge.from_ranges,
                });
            }
        }
        frontier = next;
        depth += 1;
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

/// Transitive outgoing closure (forward slice) from `doc_path`.
fn outgoing_closure(corpus: &Corpus, doc_path: &Path, max_depth: usize) -> Vec<SliceEntry> {
    let mut visited: BTreeSet<String> = BTreeSet::new();
    visited.insert(canonical(doc_path));
    let mut out: Vec<SliceEntry> = Vec::new();
    let mut frontier: Vec<PathBuf> = vec![doc_path.to_path_buf()];

    let mut depth = 0usize;
    while !frontier.is_empty() && depth < max_depth {
        let mut next: Vec<PathBuf> = Vec::new();
        for path in &frontier {
            for edge in hierarchy::outgoing_calls(corpus, path) {
                let target = PathBuf::from(edge.item.detail.as_deref().unwrap_or(""));
                let key = canonical(&target);
                if visited.contains(&key) {
                    continue;
                }
                visited.insert(key.clone());
                let text = corpus
                    .iter()
                    .find(|(p, _)| canonical(p) == key)
                    .map(|(_, t)| t.clone())
                    .unwrap_or_default();
                next.push(PathBuf::from(&key));
                out.push(SliceEntry {
                    path: PathBuf::from(key),
                    symbol: symbol_of(&target, &text),
                    ranges: edge.from_ranges,
                });
                let _ = &text;
            }
        }
        frontier = next;
        depth += 1;
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

fn canonical(p: &Path) -> String {
    // URIs in `detail` are absolute; normalize separators for path comparison.
    p.display().to_string().replace('\\', "/")
}

/// Compute the full blast radius of editing `doc_path` over `corpus`.
pub fn blast_radius(corpus: &Corpus, doc_path: &Path) -> BlastRadius {
    blast_radius_capped(corpus, doc_path, MAX_DEPTH)
}

/// Depth-capped variant of [`blast_radius`] (tests exercise tighter caps).
pub fn blast_radius_capped(corpus: &Corpus, doc_path: &Path, max_depth: usize) -> BlastRadius {
    let root_symbol = corpus
        .iter()
        .find(|(p, _)| p == doc_path)
        .map(|(p, t)| symbol_of(p, t))
        .unwrap_or_default();
    BlastRadius {
        root: doc_path.to_path_buf(),
        root_symbol,
        incoming: incoming_closure(corpus, doc_path, max_depth),
        outgoing: outgoing_closure(corpus, doc_path, max_depth),
    }
}

// ---------------------------------------------------------------------------
// Admission gate
// ---------------------------------------------------------------------------

/// The edit proposal under consideration.
#[derive(Debug, Clone)]
pub struct EditProposal {
    pub uri: String,
    pub span: Range,
    pub description: String,
}

/// Admission configuration: protected path prefixes are the declared
/// critical paths; an edit whose radius intersects one is REFUSED.
#[derive(Debug, Clone, Default)]
pub struct AdmissionConfig {
    pub protected_prefixes: Vec<String>,
    pub max_depth: usize,
}

impl AdmissionConfig {
    /// Config with the given critical-path prefixes and default depth cap.
    pub fn new(protected_prefixes: Vec<String>) -> Self {
        Self {
            protected_prefixes,
            max_depth: MAX_DEPTH,
        }
    }
}

/// Gate verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Radius intersects a declared critical path — edit must not proceed.
    Refused,
    /// Radius non-empty — edit proceeds with slices injected as context.
    Partial,
    /// Radius empty — no config-surface consequence, edit proceeds.
    Alive,
}

impl Verdict {
    pub fn as_str(&self) -> &'static str {
        match self {
            Verdict::Refused => "REFUSED",
            Verdict::Partial => "PARTIAL",
            Verdict::Alive => "ALIVE",
        }
    }
}

/// Gate output: verdict, the slices (context payload on PARTIAL), and the
/// transition receipt.
#[derive(Debug, Clone)]
pub struct Admission {
    pub verdict: Verdict,
    pub slices: Vec<SliceEntry>,
    pub receipt: TransitionReceipt,
}

/// Decide an edit. REFUSED beats PARTIAL beats ALIVE; the refusal lists the
/// intersected critical prefixes in the receipt's reason.
pub fn admit_edit(
    proposal: &EditProposal,
    radius: &BlastRadius,
    config: &AdmissionConfig,
) -> Admission {
    let (verdict, reason): (Verdict, String) = {
        let hits: Vec<&String> = config
            .protected_prefixes
            .iter()
            .filter(|prefix| {
                radius
                    .incoming
                    .iter()
                    .chain(radius.outgoing.iter())
                    .any(|s| s.path.display().to_string().starts_with(prefix.as_str()))
                    || radius
                        .root
                        .display()
                        .to_string()
                        .starts_with(prefix.as_str())
            })
            .collect();
        if !hits.is_empty() {
            (
                Verdict::Refused,
                format!(
                    "critical-path intersection: {}",
                    hits.iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
        } else if radius.is_empty() {
            (Verdict::Alive, "empty blast radius".to_string())
        } else {
            (
                Verdict::Partial,
                format!("{} slice(s) in radius", radius.len()),
            )
        }
    };

    let slices = if verdict == Verdict::Alive {
        Vec::new()
    } else {
        let mut all: Vec<SliceEntry> = radius.incoming.clone();
        all.extend(radius.outgoing.clone());
        all
    };

    Admission {
        verdict,
        slices,
        receipt: TransitionReceipt::new(verdict, proposal, radius, &reason),
    }
}

// ---------------------------------------------------------------------------
// Transition receipts (ggen.admission-transition/1, prev-hash chained)
// ---------------------------------------------------------------------------

/// One `ggen.admission-transition/1` record.
#[derive(Debug, Clone, PartialEq)]
pub struct TransitionReceipt {
    /// Schema identifier, always `ggen.admission-transition/1`.
    pub schema: String,
    /// Verdict string (REFUSED / PARTIAL / ALIVE).
    pub decision: String,
    /// blake3 digest of the canonical blast-radius form.
    pub radius_digest: String,
    /// RFC 3339 UTC timestamp of the decision.
    pub timestamp: String,
    /// Proposal URI + span, carried for replay.
    pub subject: String,
    /// Human/agent-readable reason for the verdict.
    pub reason: String,
    /// Hash of the previous journal record ("" for the genesis record).
    pub prev_hash: String,
    /// blake3 chain hash of this record.
    pub hash: String,
}

impl TransitionReceipt {
    fn new(verdict: Verdict, proposal: &EditProposal, radius: &BlastRadius, reason: &str) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let mut rec = TransitionReceipt {
            schema: "ggen.admission-transition/1".to_string(),
            decision: verdict.as_str().to_string(),
            radius_digest: radius.digest(),
            timestamp,
            subject: format!(
                "{}#{}:{}-{}:{}",
                proposal.uri,
                proposal.span.start.line,
                proposal.span.start.character,
                proposal.span.end.line,
                proposal.span.end.character
            ),
            reason: reason.to_string(),
            prev_hash: String::new(),
            hash: String::new(),
        };
        rec.prev_hash = String::new(); // stamped by the journal on append
        rec
    }

    fn payload(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}",
            self.schema,
            self.decision,
            self.radius_digest,
            self.timestamp,
            self.subject,
            self.reason,
            self.prev_hash
        )
    }
}

/// In-memory, prev-hash-chained journal of admission transitions.
#[derive(Debug, Default)]
pub struct AdmissionJournal {
    records: Vec<TransitionReceipt>,
}

impl AdmissionJournal {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a record: stamp `prev_hash` from the current tail, chain-hash.
    pub fn append(&mut self, mut rec: TransitionReceipt) -> String {
        rec.prev_hash = self
            .records
            .last()
            .map(|r| r.hash.clone())
            .unwrap_or_default();
        rec.hash = hex(blake3::hash(rec.payload().as_bytes()).as_bytes());
        let h = rec.hash.clone();
        self.records.push(rec);
        h
    }

    /// All records, in append order.
    pub fn records(&self) -> &[TransitionReceipt] {
        &self.records
    }

    /// Insert a record WITHOUT recomputing prev_hash/hash — used to replay a
    /// captured chain (e.g. tamper-detection witnesses). `verify_chain` is
    /// the arbiter of whether the replayed chain is sound.
    pub fn append_unchecked(&mut self, rec: TransitionReceipt) {
        self.records.push(rec);
    }

    /// Verify the chain: every record's stored hash must equal its recomputed
    /// payload hash, and each `prev_hash` must equal the prior record's hash.
    /// Tampering with ANY field of ANY record breaks verification.
    pub fn verify_chain(&self) -> Result<(), String> {
        let mut prev = String::new();
        for (idx, rec) in self.records.iter().enumerate() {
            if rec.prev_hash != prev {
                return Err(format!(
                    "record {}: prev_hash mismatch (expected {}, found {})",
                    idx, prev, rec.prev_hash
                ));
            }
            if rec.hash != hex(blake3::hash(rec.payload().as_bytes()).as_bytes()) {
                return Err(format!("record {}: hash mismatch (record tampered)", idx));
            }
            prev = rec.hash.clone();
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Delta grounding (P8 SemanticTokensDelta -> agent-readable classifications)
// ---------------------------------------------------------------------------

/// LSP standard semantic-token modifier bit positions this grounding
/// recognizes. The local legend declares no modifiers (`token_modifiers`
/// empty in [`semantic_tokens::legend`]), so modifier bits surface only when
/// a legend that declares them feeds the grounding.
pub mod modifiers {
    pub const DEPRECATED: u32 = 1 << 2; // LSP standard bit 2
    pub const MODIFIERS_BIT: u32 = 1 << 5; // mutating/modifiable marker (local convention)
}

/// A compact, agent-readable observation derived from one delta edit.
#[derive(Debug, Clone, PartialEq)]
pub struct GroundedEdit {
    /// Slot offset the edit applies at (token-slot units, 5 ints per token).
    pub slot_start: u32,
    /// Slots deleted.
    pub slots_deleted: u32,
    /// Compact classification lines, one per inserted token:
    /// `<slot>:<legend type>[+deprecated][+mutating]`.
    pub classifications: Vec<String>,
}

/// The grounded observation over a whole delta.
#[derive(Debug, Clone, PartialEq)]
pub struct GroundedObservation {
    pub edits: Vec<GroundedEdit>,
    /// Totals by legend token-type name across inserted tokens.
    pub totals: BTreeSet<String>,
    /// Rendered compact form (the bytes the agent actually reads).
    pub rendered: String,
}

/// Ground a [`SemanticTokensDelta`] into compact classifications. Input is
/// the wire delta; output is a few bytes per token instead of the full
/// 5-int token array.
pub fn ground(delta: &SemanticTokensDelta) -> GroundedObservation {
    let legend = semantic_tokens::token_type_strings();
    let mut edits = Vec::new();
    let mut totals: BTreeSet<String> = BTreeSet::new();
    let mut rendered = String::new();
    for e in &delta.edits {
        let mut classifications = Vec::new();
        if let Some(data) = &e.data {
            let mut slot = e.start;
            for t in data {
                let ty = legend
                    .get(t.token_type as usize)
                    .cloned()
                    .unwrap_or_else(|| format!("unknown#{}", t.token_type));
                let mut cls = ty.clone();
                if t.token_modifiers_bitset & modifiers::DEPRECATED != 0 {
                    cls.push_str("+deprecated");
                }
                if t.token_modifiers_bitset & modifiers::MODIFIERS_BIT != 0 {
                    cls.push_str("+mutating");
                }
                classifications.push(format!("{}:{}", slot, cls));
                totals.insert(cls.clone());
                slot += crate::tokens_delta::SLOTS_PER_TOKEN as u32;
            }
        }
        let ge = GroundedEdit {
            slot_start: e.start,
            slots_deleted: e.delete_count,
            classifications,
        };
        rendered.push_str(&format!(
            "e@{} d{} [{}]\n",
            ge.slot_start,
            ge.slots_deleted,
            ge.classifications.join(" ")
        ));
        edits.push(ge);
    }
    GroundedObservation {
        edits,
        totals,
        rendered,
    }
}

// ---------------------------------------------------------------------------
// Witnesses (in-module, real inputs, no mocks)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod witness {
    use super::*;

    fn tok(line: u32, ch: u32, len: u32, tt: u32, mods: u32) -> lsp_max::lsp_types::SemanticToken {
        lsp_max::lsp_types::SemanticToken {
            delta_line: line,
            delta_start: ch,
            length: len,
            token_type: tt,
            token_modifiers_bitset: mods,
        }
    }

    #[test]
    fn ground_compacts_a_delta() {
        let delta = SemanticTokensDelta {
            result_id: Some("r1".into()),
            edits: vec![lsp_max::lsp_types::SemanticTokensEdit {
                start: 0,
                delete_count: 5,
                data: Some(vec![
                    tok(0, 0, 4, 3, 0),                     // property
                    tok(0, 6, 3, 1, modifiers::DEPRECATED), // keyword+deprecated
                ]),
            }],
        };
        let obs = ground(&delta);
        assert_eq!(obs.edits.len(), 1);
        assert_eq!(obs.edits[0].classifications.len(), 2);
        assert!(obs.classifications_contain("keyword+deprecated"));
        assert!(
            obs.rendered.len() < 120,
            "rendered must be compact, got {} bytes",
            obs.rendered.len()
        );
    }

    impl GroundedObservation {
        fn classifications_contain(&self, needle: &str) -> bool {
            self.edits
                .iter()
                .any(|e| e.classifications.iter().any(|c| c.contains(needle)))
        }
    }
}
