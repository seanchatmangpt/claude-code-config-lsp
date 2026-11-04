//! Chicago-style tests for the admission layer.
//!
//! Real collaborators: the fixture tree on disk (`tests/fixtures/claude-config/`)
//! read via real file IO into the corpus index; the real semantic-token
//! delta machinery; real blake3 hashes. No mocks. Assertions on final state.

use claude_code_config_lsp::admission::*;
use claude_code_config_lsp::hierarchy::Corpus;
use claude_code_config_lsp::tokens_delta::{self, SLOTS_PER_TOKEN};
use lsp_max::lsp_types::{SemanticToken, SemanticTokens, SemanticTokensDelta, SemanticTokensEdit};
use lsp_types_max::{Position, Range};
use std::path::{Path, PathBuf};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/claude-config")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(fixture_root().join(rel)).expect("fixture must exist")
}

fn corpus() -> Corpus {
    let root = fixture_root();
    [
        "skills/ggen/SKILL.md",
        "skills/config-audit/SKILL.md",
        "skills/mcp-doctor/SKILL.md",
        "skills/source-of-truth-check/SKILL.md",
        "skills/errc-cycle/SKILL.md",
        "skills/run-ggen/SKILL.md",
        "agents/default-agent.md",
        "claude-global-CLAUDE.md",
        "settings.json",
    ]
    .iter()
    .map(|rel| (root.join(rel), read(rel)))
    .collect()
}

fn tok(tt: u32, mods: u32) -> SemanticToken {
    SemanticToken {
        delta_line: 0,
        delta_start: 0,
        length: 3,
        token_type: tt,
        token_modifiers_bitset: mods,
    }
}

fn span(line: u32) -> Range {
    Range {
        start: Position { line, character: 0 },
        end: Position { line, character: 1 },
    }
}

// ---------------------------------------------------------------------------
// Blast radius over real fixture call chains
// ---------------------------------------------------------------------------

/// Corpus plus a real referencing doc (repo convention: the corpus API is
/// in-memory; the pushed pair carries real fixture-derived text).
fn corpus_with_reference() -> Corpus {
    let mut c = corpus();
    c.push((
        PathBuf::from("/tmp/proj/notes.md"),
        "---\nname: notes\ndescription: x\nskill: ggen\n---\n\nSee [[ggen]] and [[other]].\n"
            .to_string(),
    ));
    c
}

#[test]
fn radius_captures_incoming_and_outgoing_slices() {
    let c = corpus_with_reference();
    let skill = fixture_root().join("skills/ggen/SKILL.md");
    let r = blast_radius(&c, &skill);
    assert_eq!(r.root_symbol, "ggen");
    assert!(
        !r.incoming.is_empty(),
        "notes.md references ggen — incoming slice must be non-empty"
    );
    // Every slice entry carries >=1 range and a display symbol.
    for entry in r.incoming.iter().chain(r.outgoing.iter()) {
        assert!(
            !entry.ranges.is_empty(),
            "slice entry without ranges: {:?}",
            entry.path
        );
        assert!(!entry.symbol.is_empty());
    }
    // The incoming edge names the referencing doc and its exact ranges.
    let notes = r
        .incoming
        .iter()
        .find(|e| e.symbol == "notes")
        .expect("notes in incoming slice");
    assert!(notes.path.display().to_string().ends_with("notes.md"));
    assert!(
        notes.ranges.len() >= 2,
        "frontmatter `skill: ggen` + [[ggen]] = two ranges; got {:?}",
        notes.ranges
    );
    // Digest is deterministic across recomputation.
    let r2 = blast_radius(&c, &skill);
    assert_eq!(
        r.digest(),
        r2.digest(),
        "radius digest must be deterministic"
    );
}

#[test]
fn radius_of_out_of_corpus_doc_is_empty() {
    let c = corpus();
    let ghost = fixture_root().join("skills/does-not-exist/SKILL.md");
    let r = blast_radius(&c, &ghost);
    assert!(
        r.is_empty(),
        "doc outside the corpus must have an empty radius"
    );
    assert_eq!(r.len(), 0);
}

#[test]
fn cycle_guard_terminates_mutual_reference() {
    let mut c: Corpus = corpus();
    let root = fixture_root();
    c.push((
        root.join("cyc-a.md"),
        "---\nname: cyc-a\nsee: [[cyc-b]]\n---\nbody".into(),
    ));
    c.push((
        root.join("cyc-b.md"),
        "---\nname: cyc-b\nsee: [[cyc-a]]\n---\nbody".into(),
    ));
    // Test completing at all is the termination witness; both directions
    // must still find the neighbor.
    let r = blast_radius_capped(&c, &root.join("cyc-a.md"), 8);
    assert!(
        r.outgoing.iter().any(|e| e.path.ends_with("cyc-b.md")),
        "outgoing slice must reach cyc-b"
    );
    assert!(
        r.incoming.iter().any(|e| e.path.ends_with("cyc-b.md")),
        "incoming slice must reach cyc-b"
    );
}

#[test]
fn depth_cap_limits_closure() {
    let mut c: Corpus = corpus();
    let root = fixture_root();
    for (name, next) in [
        ("chain-a", Some("chain-b")),
        ("chain-b", Some("chain-c")),
        ("chain-c", Some("chain-d")),
        ("chain-d", None),
    ] {
        let body = match next {
            Some(n) => format!("---\nname: {name}\n---\nsee [[{n}]]"),
            None => format!("---\nname: {name}\n---\nterminal"),
        };
        c.push((root.join(format!("{name}.md")), body));
    }
    let full = blast_radius_capped(&c, &root.join("chain-a.md"), 8);
    let capped = blast_radius_capped(&c, &root.join("chain-a.md"), 1);
    assert!(
        full.outgoing.iter().any(|e| e.path.ends_with("chain-d.md")),
        "depth-8 closure must traverse a->b->c->d"
    );
    assert!(
        !capped
            .outgoing
            .iter()
            .any(|e| e.path.ends_with("chain-d.md")),
        "depth-1 closure must stop before chain-d"
    );
    assert!(
        capped
            .outgoing
            .iter()
            .any(|e| e.path.ends_with("chain-b.md")),
        "depth-1 closure must still reach chain-b"
    );
}

// ---------------------------------------------------------------------------
// Admission gate
// ---------------------------------------------------------------------------

fn proposal_for(path: &Path, line: u32) -> EditProposal {
    EditProposal {
        uri: format!("file://{}", path.display()),
        span: span(line),
        description: "test edit".into(),
    }
}

#[test]
fn critical_path_intersection_refuses() {
    let c = corpus_with_reference();
    let skill = fixture_root().join("skills/ggen/SKILL.md");
    let radius = blast_radius(&c, &skill);
    let proposal = proposal_for(&skill, 0);

    // notes.md (/tmp/proj/notes.md) sits in the radius; protecting the
    // /tmp/proj prefix must refuse.
    let cfg = AdmissionConfig::new(vec!["/tmp/proj".to_string()]);
    let decision = admit_edit(&proposal, &radius, &cfg);
    assert_eq!(
        decision.verdict,
        Verdict::Refused,
        "radius intersects /tmp/proj (notes.md) => REFUSED"
    );
    assert!(
        decision.receipt.reason.contains("critical-path"),
        "refusal receipt must name the reason; got {:?}",
        decision.receipt.reason
    );
    assert!(decision.receipt.reason.contains("/tmp/proj"));

    // The same radius under a non-matching protection is PARTIAL, not
    // REFUSED — refusal tracks the actual intersection.
    let cfg2 = AdmissionConfig::new(vec!["/definitely/not/real".into()]);
    let d2 = admit_edit(&proposal, &radius, &cfg2);
    assert_eq!(d2.verdict, Verdict::Partial);

    // An empty radius under the strict config is ALIVE — refusal is
    // radius-dependent, not unconditional.
    let empty = BlastRadius {
        root: PathBuf::from("/nowhere/skill.md"),
        root_symbol: String::new(),
        incoming: vec![],
        outgoing: vec![],
    };
    let d3 = admit_edit(&proposal, &empty, &cfg);
    assert_eq!(d3.verdict, Verdict::Alive);
}

#[test]
fn nonempty_radius_is_partial_with_slices_injected() {
    let c = corpus_with_reference();
    let skill = fixture_root().join("skills/ggen/SKILL.md");
    let radius = blast_radius(&c, &skill);
    let proposal = proposal_for(&skill, 1);
    let cfg = AdmissionConfig::new(vec!["/definitely/not/a/real/prefix".into()]);
    let decision = admit_edit(&proposal, &radius, &cfg);
    assert_eq!(
        decision.verdict,
        Verdict::Partial,
        "non-empty radius, no protected hit => PARTIAL"
    );
    assert_eq!(
        decision.slices.len(),
        radius.len(),
        "PARTIAL injects the full slice set"
    );
    assert_eq!(
        decision.receipt.radius_digest,
        radius.digest(),
        "receipt carries the radius digest"
    );
    assert_eq!(decision.receipt.decision, "PARTIAL");
}

#[test]
fn empty_radius_is_alive_with_empty_payload() {
    let c = corpus();
    let ghost = PathBuf::from("/tmp/nowhere/SKILL.md");
    let radius = blast_radius(&c, &ghost);
    let proposal = proposal_for(&ghost, 0);
    let cfg = AdmissionConfig::new(vec![fixture_root().display().to_string()]);
    let decision = admit_edit(&proposal, &radius, &cfg);
    assert_eq!(decision.verdict, Verdict::Alive);
    assert!(
        decision.slices.is_empty(),
        "ALIVE carries no context payload"
    );
    assert_eq!(decision.receipt.decision, "ALIVE");
}

// ---------------------------------------------------------------------------
// Journal: chain integrity, tamper detection
// ---------------------------------------------------------------------------

fn receipt(decision: &str, digest: &str, ts: &str, subject: &str) -> TransitionReceipt {
    TransitionReceipt {
        schema: "ggen.admission-transition/1".into(),
        decision: decision.into(),
        radius_digest: digest.into(),
        timestamp: ts.into(),
        subject: subject.into(),
        reason: "test".into(),
        prev_hash: String::new(),
        hash: String::new(),
    }
}

#[test]
fn journal_chains_records() {
    let c = corpus();
    let skill = fixture_root().join("skills/ggen/SKILL.md");
    let radius = blast_radius(&c, &skill);
    let cfg = AdmissionConfig::new(vec![fixture_root().display().to_string()]);
    let mut journal = AdmissionJournal::new();

    let d1 = admit_edit(&proposal_for(&skill, 0), &radius, &cfg);
    let d2 = admit_edit(&proposal_for(&skill, 2), &radius, &cfg);
    journal.append(d1.receipt.clone());
    journal.append(d2.receipt.clone());

    let recs = journal.records();
    assert_eq!(recs.len(), 2);
    assert_eq!(recs[0].schema, "ggen.admission-transition/1");
    assert_eq!(recs[0].prev_hash, "", "genesis record has empty prev_hash");
    assert_eq!(
        recs[1].prev_hash, recs[0].hash,
        "second record chains to the first's hash"
    );
    assert!(
        journal.verify_chain().is_ok(),
        "untampered chain must verify"
    );
    assert!(recs.iter().all(|r| r.radius_digest == radius.digest()));
}

#[test]
fn tampered_record_is_detected() {
    // Chain two real decisions, then forge record 0's decision field while
    // keeping its original hash — verification must fail.
    let c = corpus();
    let skill = fixture_root().join("skills/ggen/SKILL.md");
    let radius = blast_radius(&c, &skill);
    let cfg = AdmissionConfig::new(vec![fixture_root().display().to_string()]);
    let d1 = admit_edit(&proposal_for(&skill, 0), &radius, &cfg);
    let d2 = admit_edit(&proposal_for(&skill, 3), &radius, &cfg);

    let mut j = AdmissionJournal::new();
    j.append(d1.receipt.clone());
    j.append(d2.receipt.clone());
    let rec0 = journal_records(&j)[0].clone();
    let rec1 = journal_records(&j)[1].clone();
    let mut forged = rec0.clone();
    forged.decision = "ALIVE".into(); // content changed...
    forged.hash = rec0.hash.clone(); // ...hash forged to the original
    let mut j2 = AdmissionJournal::default();
    j2.append_unchecked(forged);
    j2.append_unchecked(rec1);
    assert!(
        j2.verify_chain().is_err(),
        "tampered content with forged hash must fail verification"
    );
}

fn journal_records(j: &AdmissionJournal) -> Vec<TransitionReceipt> {
    j.records().to_vec()
}

#[test]
fn prev_hash_mismatch_is_detected() {
    let mut j = AdmissionJournal::new();
    j.append(receipt("ALIVE", "aa", "2026-11-03T00:00:00Z", "file:///x"));
    assert!(j.verify_chain().is_ok());
    // Forged second record: hash recomputed over a payload claiming a
    // prev_hash that is NOT record 0's hash.
    let mut r2 = receipt("PARTIAL", "bb", "2026-11-03T00:00:01Z", "file:///y");
    r2.prev_hash = "not-the-real-prev".into();
    let payload = format!(
        "{}|{}|{}|{}|{}|{}|{}",
        r2.schema, r2.decision, r2.radius_digest, r2.timestamp, r2.subject, r2.reason, r2.prev_hash
    );
    let digest = blake3::hash(payload.as_bytes());
    r2.hash = digest
        .as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    j.append_unchecked(r2);
    assert!(
        j.verify_chain().is_err(),
        "claimed prev_hash not matching record 0 must fail"
    );
}

// ---------------------------------------------------------------------------
// Grounding: compactness + modifier markers
// ---------------------------------------------------------------------------

#[test]
fn grounding_compacts_delta_output() {
    // 40 tokens, one changed in the middle — the real P8 delta path yields
    // one small edit; the grounded rendering must be an order of magnitude
    // smaller than the raw 5-int-per-token payload.
    let mk = |i: usize| SemanticToken {
        delta_line: 0,
        delta_start: if i == 0 { 0 } else { 4 },
        length: 3,
        token_type: (i % 6) as u32,
        token_modifiers_bitset: 0,
    };
    let prev_data: Vec<SemanticToken> = (0..40).map(mk).collect();
    let mut next_data = prev_data.clone();
    next_data[20] = SemanticToken {
        token_type: 4,
        ..next_data[20]
    };
    let prev = SemanticTokens {
        result_id: None,
        data: prev_data.clone(),
    };
    let next = SemanticTokens {
        result_id: None,
        data: next_data.clone(),
    };
    let delta: SemanticTokensDelta = tokens_delta::semantic_tokens_delta(&prev, &next, 1);
    assert!(
        !delta.edits.is_empty(),
        "one changed token among 40 must produce an edit"
    );

    let input_bytes: usize = next_data
        .iter()
        .map(|t| {
            format!(
                "{},{},{},{},{}",
                t.delta_line, t.delta_start, t.length, t.token_type, t.token_modifiers_bitset
            )
            .len()
        })
        .sum();
    let obs = ground(&delta);
    let output_bytes = obs.rendered.len();
    assert!(
        output_bytes * 10 < input_bytes,
        "grounded output ({}B) must be 10x smaller than the raw token payload ({}B)",
        output_bytes,
        input_bytes
    );
    for t in &obs.totals {
        assert!(
            !t.starts_with("unknown#"),
            "legend-known types must classify by name: {}",
            t
        );
    }
}

#[test]
fn grounding_marks_deprecated_modifier() {
    let delta = SemanticTokensDelta {
        result_id: None,
        edits: vec![SemanticTokensEdit {
            start: 0,
            delete_count: SLOTS_PER_TOKEN as u32,
            data: Some(vec![tok(3, modifiers::DEPRECATED)]),
        }],
    };
    let obs = ground(&delta);
    assert!(
        obs.rendered.contains("+deprecated"),
        "deprecated modifier bit must surface in classifications; got: {}",
        obs.rendered
    );
    assert!(
        obs.rendered.contains("property"),
        "token_type 3 = property in the legend"
    );
}
