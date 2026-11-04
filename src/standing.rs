//! Standing vocabulary for the fleet receipt layer — editor-visible standing.
//!
//! Parses the fleet receipt at `.claude/receipts/latest.json`
//! (`{"status":"green","commands":[...]}` shape), tracks mtime staleness
//! (receipt newer-than-newest-source check below), and maps non-ALIVE
//! standings to editor diagnostics whose message carries the receipt's own
//! command list as remediation (house law: remediation per refusal).

use std::path::Path;
use std::time::SystemTime;

use lsp_types_max::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Uri};
use serde::{Deserialize, Serialize};

/// Diagnostic `source` field for every standing diagnostic.
pub const STANDING_SOURCE: &str = "standing";
/// Diagnostic code: red or stale gate receipt.
pub const CODE_BLOCKED: &str = "standing-blocked";
/// Diagnostic code: green+fresh but findings present on the surface.
pub const CODE_PARTIAL: &str = "standing-partial";
/// Diagnostic code: never scanned (no receipt).
pub const CODE_UNVERIFIED: &str = "standing-unverified";

/// The closed standing vocabulary.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Standing {
    /// Receipt green + fresh + zero diagnostics for the surface.
    ALIVE,
    /// Green + fresh but diagnostic findings present on the surface.
    PARTIAL,
    /// Receipt red or stale (older than newest tracked source file).
    BLOCKED,
    /// Never scanned — no receipt exists.
    UNVERIFIED,
}

impl Standing {
    /// The vocabulary word, per the fleet status vocabulary.
    pub fn as_str(&self) -> &'static str {
        match self {
            Standing::ALIVE => "ALIVE",
            Standing::PARTIAL => "PARTIAL",
            Standing::BLOCKED => "BLOCKED",
            Standing::UNVERIFIED => "UNVERIFIED",
        }
    }
}

/// Fleet receipt shape: `{"status":"green","commands":[...]}`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FleetReceipt {
    /// "green" = gate passed; anything else is treated as red.
    pub status: String,
    /// The gate commands that produced the receipt — surfaced as remediation.
    #[serde(default)]
    pub commands: Vec<String>,
    /// Disk-witnessed file mtime — never serialized; staleness is observed,
    /// not assumed. `None` = parsed without disk witness (never fresh).
    #[serde(skip)]
    pub mtime: Option<SystemTime>,
}

impl FleetReceipt {
    /// True only when the receipt's status is exactly "green".
    /// Never-round-up law: anything non-green is not green.
    pub fn is_green(&self) -> bool {
        self.status == "green"
    }

    /// True when the receipt is not older than the newest tracked source
    /// file. A receipt with unknown mtime (parsed without disk) is never
    /// fresh — staleness is witnessed, not assumed.
    pub fn is_fresh(&self, newest_source_mtime: Option<SystemTime>) -> bool {
        match (self.mtime, newest_source_mtime) {
            (Some(receipt_mtime), Some(source_mtime)) => receipt_mtime >= source_mtime,
            // No source files tracked: nothing newer to invalidate the receipt.
            (Some(_), None) => true,
            // No receipt mtime witnessed -> cannot claim fresh.
            (None, _) => false,
        }
    }

    /// Receipt file mtime, witnessed at load time.
    pub fn mtime(&self) -> Option<SystemTime> {
        self.mtime
    }
}

// mtime is carried alongside the parsed JSON (not serialized): the receipt
// file's mtime is disk-witnessed, never part of the wire shape.
impl FleetReceipt {
    /// Attach a disk-witnessed mtime to a parsed receipt.
    pub fn with_mtime(mut self, t: Option<SystemTime>) -> Self {
        self.mtime = t;
        self
    }
}

/// Path of the fleet receipt within the workspace root.
pub fn latest_receipt_path(workspace_root: &Path) -> std::path::PathBuf {
    workspace_root
        .join(".claude")
        .join("receipts")
        .join("latest.json")
}

/// Parse a receipt from its JSON text (mtime = None; attach one via
/// `with_mtime` if witnessed on disk).
pub fn parse_receipt(text: &str) -> Result<FleetReceipt, String> {
    let r: FleetReceipt =
        serde_json::from_str(text).map_err(|e| format!("invalid fleet receipt JSON: {e}"))?;
    Ok(r.with_mtime(None))
}

/// Load `.claude/receipts/latest.json` under `workspace_root`, attaching the
/// file's real mtime. Returns Ok(None) when the receipt does not exist
/// (UNVERIFIED territory, not an error).
pub fn load_receipt(workspace_root: &Path) -> Result<Option<FleetReceipt>, String> {
    let path = latest_receipt_path(workspace_root);
    let md = match std::fs::metadata(&path) {
        Ok(md) => md,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("failed to stat {}: {e}", path.display())),
    };
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let receipt = parse_receipt(&text)?;
    Ok(Some(receipt.with_mtime(md.modified().ok())))
}

/// Resolve standing for one surface (a document URI) given the fleet
/// receipt, the newest tracked source file mtime, and the diagnostic count
/// for that surface.
///
/// Never-round-up law: ALIVE requires the green+fresh+clean triple
/// witnessed; any missing leg rounds DOWN, never up.
pub fn standing_for_surface(
    surface: &Uri,
    receipt: Option<&FleetReceipt>,
    newest_source_mtime: Option<SystemTime>,
    diagnostic_count: usize,
) -> Standing {
    let _ = surface;
    match receipt {
        None => Standing::UNVERIFIED,
        Some(r) if !r.is_green() || !r.is_fresh(newest_source_mtime) => Standing::BLOCKED,
        Some(_) if diagnostic_count > 0 => Standing::PARTIAL,
        Some(_) => Standing::ALIVE,
    }
}

/// Map a non-ALIVE standing to editor diagnostics for `surface`.
/// ALIVE emits nothing. BLOCKED carries the receipt's own command list as
/// remediation in the diagnostic message; PARTIAL and UNVERIFIED point at
/// the gate receipt.
pub fn standing_diagnostics(
    surface: &Uri,
    standing: Standing,
    receipt: Option<&FleetReceipt>,
) -> Vec<Diagnostic> {
    let range = Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 0,
            character: 0,
        },
    };
    let (code, severity, message) = match standing {
        Standing::ALIVE => return Vec::new(),
        Standing::BLOCKED => {
            let remediation = match receipt {
                Some(r) if !r.commands.is_empty() => {
                    format!("; run the gate commands: {}", r.commands.join("; "))
                }
                _ => "; run the gate commands recorded in .claude/receipts/latest.json".to_string(),
            };
            (
                CODE_BLOCKED,
                DiagnosticSeverity::WARNING,
                format!(
                    "gate receipt stale/red for {}; run the gate commands{}",
                    surface.as_str(),
                    remediation
                ),
            )
        }
        Standing::PARTIAL => (
            CODE_PARTIAL,
            DiagnosticSeverity::WARNING,
            format!(
                "gate receipt green but diagnostic findings present for {}; clear the findings, then re-run the gate",
                surface.as_str()
            ),
        ),
        Standing::UNVERIFIED => (
            CODE_UNVERIFIED,
            DiagnosticSeverity::INFORMATION,
            format!(
                "never scanned: no gate receipt at .claude/receipts/latest.json for {}; run the gate commands to establish standing",
                surface.as_str()
            ),
        ),
    };
    vec![Diagnostic {
        range,
        severity: Some(severity),
        code: Some(NumberOrString::String(code.to_string())),
        source: Some(STANDING_SOURCE.into()),
        message,
        ..Default::default()
    }]
}
