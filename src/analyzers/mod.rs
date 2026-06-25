//! Analyzer modules for each ConfigSurface
//!
//! Each analyzer is responsible for validating files matching a specific ConfigSurface pattern.
//! Analyzers emit CCC-* diagnostics corresponding to their respective law axes.

pub mod frontmatter;
pub mod hook;

pub use frontmatter::{
    DefaultFrontmatterAnalyzer, FrontmatterFinding, FrontmatterRule, ParsedFrontmatter,
    ReplayableFrontmatterAnalyzer, extract_frontmatter,
};
pub use hook::{HookDiagnosticCode, HookDiagnostic, HookScriptAnalyzer};
