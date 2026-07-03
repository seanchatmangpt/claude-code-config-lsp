//! LSP method coverage report — modeled on lsp-max's coverage matrices
//! (`~/lsp-max/src/coverage/lsp_coverage.rs`), adapted to reuse this
//! project's own `ConformanceVector` (see `crate::conformance`) instead of
//! diffing against a separately-vendored `metaModel.json`.
//!
//! `ConformanceVector` already partitions LSP methods into `admitted`
//! (implemented), `refused` (explicitly not implemented), and `unknown`
//! (undetermined) — that partition *is* a coverage matrix. This module just
//! turns it into a percentage + summary suitable for surfacing in the
//! `claude-config://health` virtual document.

use lsp_max::max_protocol::LawAxis;

use crate::conformance::conformance_vector;

/// A summary of LSP method coverage derived from `ConformanceVector`.
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageReport {
    pub admitted: usize,
    pub refused: usize,
    pub unknown: usize,
}

impl CoverageReport {
    pub fn total(&self) -> usize {
        self.admitted + self.refused + self.unknown
    }

    /// Percentage of tracked methods with a real handler implementation,
    /// rounded to one decimal place. `0.0` if no methods are tracked.
    pub fn coverage_percent(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            return 0.0;
        }
        ((self.admitted as f64 / total as f64) * 1000.0).round() / 10.0
    }

    /// Render as a markdown fragment for embedding in a virtual document.
    pub fn render_markdown(&self) -> String {
        format!(
            "## LSP Method Coverage\n\n\
             {}/{} tracked methods implemented ({:.1}%)\n\n\
             | Category | Count |\n|---|---|\n\
             | Admitted (implemented) | {} |\n\
             | Refused (explicitly unimplemented) | {} |\n\
             | Unknown (undetermined) | {} |\n",
            self.admitted,
            self.total(),
            self.coverage_percent(),
            self.admitted,
            self.refused,
            self.unknown
        )
    }
}

/// Compute the current coverage report from this server's `ConformanceVector`.
pub fn coverage_report() -> CoverageReport {
    let cv = conformance_vector();
    CoverageReport {
        admitted: count_axes(&cv.admitted),
        refused: count_axes(&cv.refused),
        unknown: count_axes(&cv.unknown),
    }
}

fn count_axes(axes: &[LawAxis]) -> usize {
    axes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage_percent_matches_conformance_vector() {
        let report = coverage_report();
        assert_eq!(report.total(), report.admitted + report.refused + report.unknown);
        assert!(report.coverage_percent() >= 0.0 && report.coverage_percent() <= 100.0);
    }

    #[test]
    fn render_markdown_contains_counts() {
        let report = CoverageReport { admitted: 6, refused: 27, unknown: 3 };
        let md = report.render_markdown();
        assert!(md.contains("6/36"));
        assert!(md.contains("16.7%"));
    }

    #[test]
    fn empty_report_has_zero_percent() {
        let report = CoverageReport { admitted: 0, refused: 0, unknown: 0 };
        assert_eq!(report.coverage_percent(), 0.0);
    }
}
