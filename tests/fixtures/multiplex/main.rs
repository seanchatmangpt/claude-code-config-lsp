//! Sanitized excerpt of `ggen/crates/ggen-lsp/src/error.rs` (ggen repo).
//! Fixture for the four-language compositor multiplex witness (R2 lane).
//! A deliberate rustc E0308 type-mismatch violation is appended at the bottom
//! so rust-analyzer's flycheck relays a REAL rustc diagnostic through the
//! compositor.

use std::fmt;

/// Typed error surface for the ggen LSP server. Shape mirrors the real
/// `LspError` enum; the `thiserror` derive is dropped here because this
/// fixture crate has zero dependencies by design.
#[derive(Debug)]
pub enum LspError {
    /// RDF parse failure.
    RdfParseError(String),
    /// RDF query failure.
    RdfQueryError(String),
    /// Tera parse failure.
    TeraParseError(String),
    /// Tera render failure.
    TeraRenderError(String),
    /// TOML parse failure.
    TomlParseError(String),
    /// TOML validation failure.
    TomlValidationError(String),
    /// IO failure.
    IoError(String),
    /// LSP protocol failure.
    LspProtocolError(String),
    /// Unknown failure.
    Unknown(String),
}

impl fmt::Display for LspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LspError::RdfParseError(m) => write!(f, "RDF parse error: {m}"),
            LspError::RdfQueryError(m) => write!(f, "RDF query error: {m}"),
            LspError::TeraParseError(m) => write!(f, "Tera parse error: {m}"),
            LspError::TeraRenderError(m) => write!(f, "Tera render error: {m}"),
            LspError::TomlParseError(m) => write!(f, "TOML parse error: {m}"),
            LspError::TomlValidationError(m) => write!(f, "TOML validation error: {m}"),
            LspError::IoError(m) => write!(f, "IO error: {m}"),
            LspError::LspProtocolError(m) => write!(f, "LSP protocol error: {m}"),
            LspError::Unknown(m) => write!(f, "Unknown error: {m}"),
        }
    }
}

pub type Result<T> = std::result::Result<T, LspError>;

// ── deliberate violation (rustc E0308: mismatched types) ────────────────────
pub fn deliberate_violation() {
    let v: u64 = "not a u64";
    let _ = v;
}

// ── deliberate syntax violation (instant rust-analyzer parse diagnostic; the
//    E0308 above only surfaces via cargo-check flycheck on save) ─────────────
pub fn syntax_violation( {
    let _ = 1;
}
