//! Small public API used to exercise the SphinxDocRS LSP report pipeline.

/// Summary counts returned by a source analysis run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildReport {
    /// Number of declarations returned by the language server.
    pub declarations: usize,
    /// Number of diagnostics returned by the language server.
    pub diagnostics: usize,
}

/// Create a report from the declaration and diagnostic totals.
pub fn build_report(declarations: usize, diagnostics: usize) -> BuildReport {
    BuildReport {
        declarations,
        diagnostics,
    }
}

/// Format a short human-readable summary of a build report.
pub fn render_report(report: &BuildReport) -> String {
    format!(
        "LSP returned {} declaration(s) and {} diagnostic(s).",
        report.declarations, report.diagnostics
    )
}