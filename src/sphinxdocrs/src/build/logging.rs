//! Logging/status stream setup for `sphinx-build-rs`.
//!
//! Mirrors `_parse_logging(parser, quiet, really_quiet, warnfile)`.
//! The `TeeStripANSI` behavior (writing stripped ANSI to a warn file
//! while coloured output goes to stderr) is deferred until a native
//! builder exists; for now the warn file is opened and the path
//! returned so the Python shim can pass it on.

use std::path::PathBuf;

/// Resolved logging configuration.
#[derive(Debug)]
pub struct LoggingConfig {
    /// Whether status output to stdout is suppressed (`-q`).
    pub suppress_status: bool,
    /// Whether warning output to stderr is suppressed (`-Q`).
    pub suppress_warnings: bool,
    /// Path to the warning file (`-w FILE`), if requested.
    pub warnfile: Option<PathBuf>,
}

/// Mirrors `_parse_logging`.
pub fn parse_logging(quiet: bool, really_quiet: bool, warnfile: Option<PathBuf>) -> LoggingConfig {
    LoggingConfig {
        suppress_status: quiet || really_quiet,
        suppress_warnings: really_quiet,
        warnfile,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quiet_suppresses_status_only() {
        let l = parse_logging(true, false, None);
        assert!(l.suppress_status);
        assert!(!l.suppress_warnings);
    }

    #[test]
    fn really_quiet_suppresses_both() {
        let l = parse_logging(true, true, None);
        assert!(l.suppress_status);
        assert!(l.suppress_warnings);
    }

    #[test]
    fn default_no_suppression() {
        let l = parse_logging(false, false, None);
        assert!(!l.suppress_status);
        assert!(!l.suppress_warnings);
    }
}
