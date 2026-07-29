//! Logging/status stream setup for `sphinx-build-rs`.
//!
//! Mirrors `_parse_logging(parser, quiet, really_quiet, warnfile)`.
//! [`finish_build`] is the **H8c** "`TeeStripANSI`" counterpart: it
//! prints/suppresses `SphinxApp::warnings` per [`LoggingConfig`], writes
//! an ANSI-stripped copy to the `-w FILE` warn file when requested, and
//! decides the process exit code for `-W`/`--fail-on-warning`. Called
//! from both CLI entry points
//! ([`crate::bin`]-equivalent `sphinx_build.rs` and
//! [`crate::build::native_runner`]) after a build completes.

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

/// Finalize a build: print collected `warnings` to stderr (unless
/// [`LoggingConfig::suppress_warnings`]), write an ANSI-stripped copy to
/// [`LoggingConfig::warnfile`] if one was requested (mirrors upstream's
/// `TeeStripANSI` writer -- coloured text to the terminal, escape
/// sequences stripped in the file), and decide the process exit code for
/// `-W`/`--fail-on-warning`.
///
/// **Accepted deviation:** upstream's bare `-W` (without `--keep-going`)
/// raises `SphinxWarning` and aborts the build at the *first* warning
/// encountered, whereas `-W --keep-going` collects every warning and
/// only fails at the end. This port's build pipeline has no
/// abort-on-first-warning hook -- warnings are appended to
/// `SphinxApp::warnings` as they're discovered, not raised as
/// exceptions -- so both `-W` and `-W --keep-going` behave like
/// upstream's `--keep-going` here: the build always runs to completion,
/// and the process exits nonzero afterward if any warnings were
/// recorded. This never silently swallows a warning-as-error; it just
/// doesn't abort as early as bare `-W` would upstream.
///
/// Returns `1` when `warningiserror` is set and `warnings` is non-empty,
/// else `0`.
pub fn finish_build(warnings: &[String], config: &LoggingConfig, warningiserror: bool) -> i32 {
    if !config.suppress_warnings {
        for w in warnings {
            eprintln!("WARNING: {w}");
        }
    }
    if let Some(path) = &config.warnfile {
        let mut stripped = String::new();
        for w in warnings {
            stripped.push_str(&crate::util_console::strip_escape_sequences(w));
            stripped.push('\n');
        }
        if let Err(e) = std::fs::write(path, stripped) {
            eprintln!(
                "sphinxdocrs: failed to write warning file {}: {e}",
                path.display()
            );
        }
    }
    if warningiserror && !warnings.is_empty() {
        1
    } else {
        0
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

    #[test]
    fn finish_build_exits_zero_without_warningiserror() {
        let config = parse_logging(false, false, None);
        let code = finish_build(&["a warning".to_string()], &config, false);
        assert_eq!(code, 0);
    }

    #[test]
    fn finish_build_exits_nonzero_with_warningiserror_and_warnings() {
        let config = parse_logging(false, false, None);
        let code = finish_build(&["a warning".to_string()], &config, true);
        assert_eq!(code, 1);
    }

    #[test]
    fn finish_build_exits_zero_with_warningiserror_and_no_warnings() {
        let config = parse_logging(false, false, None);
        let code = finish_build(&[], &config, true);
        assert_eq!(code, 0);
    }

    #[test]
    fn finish_build_writes_stripped_warnfile() {
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("warnings.txt");
        let config = parse_logging(false, false, Some(path.clone()));
        finish_build(
            &["\x1b[91mred warning\x1b[39;49;00m".to_string()],
            &config,
            false,
        );
        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "red warning\n");
    }
}
