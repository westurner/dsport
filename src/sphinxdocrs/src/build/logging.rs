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

use std::path::{Path, PathBuf};

use crate::util_strypes::strip_escape_sequences;

/// Resolved logging configuration.
#[derive(Debug)]
pub struct LoggingConfig {
    /// Whether status output to stdout is suppressed (`-q`).
    pub suppress_status: bool,
    /// Whether warning output to stderr is suppressed (`-Q`).
    pub suppress_warnings: bool,
    /// Path to the warning file (`-w FILE`), if requested.
    pub warnfile: Option<PathBuf>,
    /// Verbosity level (`-v` repetitions), retained for status formatting.
    pub verbosity: u8,
    /// Whether terminal warning output should use ANSI colors.
    pub color: bool,
}

/// Mirrors `_parse_logging`.
pub fn parse_logging(
    quiet: bool,
    really_quiet: bool,
    warnfile: Option<PathBuf>,
    verbosity: u8,
    color: &str,
) -> LoggingConfig {
    LoggingConfig {
        suppress_status: quiet || really_quiet,
        suppress_warnings: really_quiet,
        warnfile,
        verbosity,
        color: match color {
            "yes" => true,
            "no" => false,
            _ => crate::util_console::terminal_supports_colour_from_env(|key| {
                std::env::var(key).ok()
            }),
        },
    }
}

/// Format one warning exactly once for terminal and warning-file consumers.
pub fn format_warning(warning: &str, color: bool) -> String {
    let line = if warning.starts_with("WARNING:") || warning.contains(": WARNING:") {
        warning.to_owned()
    } else {
        format!("WARNING: {warning}")
    };
    if color {
        crate::util_console::wrap("91", &line)
    } else {
        line
    }
}

/// Emit Sphinx-style debug messages when status output is verbose.
pub fn emit_debug_messages(messages: &[String], config: &LoggingConfig) {
    if debug_messages_enabled(config) {
        for message in messages {
            println!("{message}");
        }
    }
}

/// Emit Sphinx-style info messages unless status output is suppressed.
pub fn emit_info_messages(messages: &[String], config: &LoggingConfig) {
    if info_messages_enabled(config) {
        for message in messages {
            println!("{}", format_status_message(message, config.color));
        }
    }
}

/// Apply the same Sphinx bold and progress colors used by its CLI logger.
pub fn format_status_message(message: &str, color: bool) -> String {
    if !color {
        return message.to_owned();
    }

    let running_prefix = translated_template_prefix("Running Sphinx v%s");
    if message.starts_with(&running_prefix) {
        return crate::util_console::wrap("01", message);
    }

    let summary_prefix = crate::locale::tr_console("build succeeded");
    if message.starts_with(&summary_prefix) {
        return crate::util_console::wrap("01", message);
    }

    if let Some(prefix_len) = translated_dynamic_prefix_len(message, "building [%s]: ") {
        return bold_prefix(message, prefix_len);
    }

    for msgid in [
        "making output directory... ",
        "building [mo]: ",
        "updating environment: ",
        "looking for now-outdated files... ",
        "pickling environment... ",
        "checking consistency... ",
        "preparing documents... ",
        "copying assets...",
        "copying static files...",
        "copying static files: ",
        "copying extra files...",
        "copying extra files: ",
        "copying assets: ",
        "generating indices... ",
        "writing additional pages... ",
        "dumping object inventory... ",
    ] {
        let prefix = crate::locale::tr_console(msgid);
        if message.starts_with(&prefix) {
            return bold_prefix(message, prefix.len());
        }
    }

    for (msgid, item_color) in [
        ("reading sources... ", "35"),
        ("writing output... ", "32"),
    ] {
        let prefix = crate::locale::tr_console(msgid);
        if let Some(rest) = message.strip_prefix(&prefix)
            && let Some((percent, item)) = rest.split_once("] ")
        {
            return format!(
                "{}{}] {}",
                crate::util_console::wrap("01", &prefix),
                percent,
                crate::util_console::wrap(item_color, item)
            );
        }
    }

    let search_index_prefix = translated_template_prefix("dumping search index in %s");
    if message.starts_with(&search_index_prefix)
        && let Some(end) = message.find("... ")
    {
        return bold_prefix(message, end + 4);
    }

    message.to_owned()
}

fn bold_prefix(message: &str, prefix_len: usize) -> String {
    format!(
        "{}{}",
        crate::util_console::wrap("01", &message[..prefix_len]),
        &message[prefix_len..]
    )
}

fn translated_template_prefix(template: &str) -> String {
    let translated = crate::locale::tr_console(template);
    let end = ["%s", "%d", "%("]
        .iter()
        .filter_map(|placeholder| translated.find(placeholder))
        .min()
        .unwrap_or(translated.len());
    translated[..end].to_owned()
}

fn translated_dynamic_prefix_len(message: &str, template: &str) -> Option<usize> {
    let translated = crate::locale::tr_console(template);
    let (before, after) = translated.split_once("%s")?;
    let rest = message.strip_prefix(before)?;
    Some(before.len() + rest.find(after)? + after.len())
}

/// Format Sphinx's translated build completion message.
pub fn format_build_summary(warning_count: usize, warningiserror: bool) -> String {
    match (warning_count, warningiserror) {
        (0, _) => crate::locale::tr_console("build succeeded."),
        (1, true) => crate::locale::tr_console(
            "build succeeded, 1 warning (with warnings treated as errors).",
        ),
        (1, false) => crate::locale::tr_console("build succeeded, 1 warning."),
        (count, true) => crate::locale::tr_console(
            "build succeeded, %s warnings (with warnings treated as errors).",
        )
        .replacen("%s", &count.to_string(), 1),
        (count, false) => crate::locale::tr_console("build succeeded, %s warnings.").replacen(
            "%s",
            &count.to_string(),
            1,
        ),
    }
}

/// Format the translated builder epilog for HTML builders.
pub fn format_build_epilog(builder: &str, outdir: &Path) -> Option<String> {
    matches!(builder, "html" | "dirhtml" | "singlehtml").then(|| {
        crate::locale::tr_console("The HTML pages are in %(outdir)s.")
            .replace("%(outdir)s", &outdir.display().to_string())
    })
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
            eprintln!("{}", format_warning(w, config.color));
        }
    }
    if let Some(path) = &config.warnfile {
        let mut stripped = String::new();
        for w in warnings {
            stripped.push_str(&strip_escape_sequences(&format_warning(w, config.color)));
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
        let l = parse_logging(true, false, None, 0, "auto");
        assert!(l.suppress_status);
        assert!(!l.suppress_warnings);
    }

    #[test]
    fn really_quiet_suppresses_both() {
        let l = parse_logging(true, true, None, 0, "auto");
        assert!(l.suppress_status);
        assert!(l.suppress_warnings);
    }

    #[test]
    fn default_no_suppression() {
        let l = parse_logging(false, false, None, 0, "no");
        assert!(!l.suppress_status);
        assert!(!l.suppress_warnings);
    }

    #[test]
    fn explicit_yes_color_mode_enables_color() {
        let l = parse_logging(false, false, None, 0, "yes");
        assert!(l.color);
    }

    #[test]
    fn debug_messages_require_verbose_non_quiet_output() {
        let normal = parse_logging(false, false, None, 0, "no");
        let verbose = parse_logging(false, false, None, 1, "no");
        let quiet_verbose = parse_logging(true, false, None, 1, "no");

        assert!(!debug_messages_enabled(&normal));
        assert!(debug_messages_enabled(&verbose));
        assert!(!debug_messages_enabled(&quiet_verbose));
    }

    #[test]
    fn info_messages_are_visible_by_default_and_suppressed_by_quiet() {
        let normal = parse_logging(false, false, None, 0, "no");
        let quiet = parse_logging(true, false, None, 0, "no");

        assert!(info_messages_enabled(&normal));
        assert!(!info_messages_enabled(&quiet));
    }

    #[test]
    fn finish_build_exits_zero_without_warningiserror() {
        let config = parse_logging(false, false, None, 0, "no");
        let code = finish_build(&["a warning".to_string()], &config, false);
        assert_eq!(code, 0);
    }

    #[test]
    fn finish_build_exits_nonzero_with_warningiserror_and_warnings() {
        let config = parse_logging(false, false, None, 0, "no");
        let code = finish_build(&["a warning".to_string()], &config, true);
        assert_eq!(code, 1);
    }

    #[test]
    fn finish_build_exits_zero_with_warningiserror_and_no_warnings() {
        let config = parse_logging(false, false, None, 0, "no");
        let code = finish_build(&[], &config, true);
        assert_eq!(code, 0);
    }

    #[test]
    fn finish_build_writes_stripped_warnfile() {
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("warnings.txt");
        let config = parse_logging(false, false, Some(path.clone()), 0, "yes");
        finish_build(
            &["\x1b[91mred warning\x1b[39;49;00m".to_string()],
            &config,
            false,
        );
        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "WARNING: red warning\n");
    }

    #[test]
    fn finish_build_suppresses_terminal_warnings_and_reports_write_failure() {
        let tmp = tempfile::TempDir::new().unwrap();
        let missing_parent = tmp.path().join("missing").join("warnings.txt");
        let config = parse_logging(true, true, Some(missing_parent), 0, "no");
        assert_eq!(finish_build(&["hidden warning".to_string()], &config, true), 1);
    }

    #[test]
    fn warning_format_respects_color_mode() {
        assert_eq!(format_warning("message", false), "WARNING: message");
        assert_eq!(
            format_warning("message", true),
            "\x1b[91mWARNING: message\x1b[39;49;00m"
        );
    }

    #[test]
    fn warning_format_preserves_existing_warning_prefixes() {
        assert_eq!(format_warning("WARNING: already formatted", false), "WARNING: already formatted");
        assert_eq!(
            format_warning("index.rst: WARNING: located warning", false),
            "index.rst: WARNING: located warning"
        );
    }
}

fn debug_messages_enabled(config: &LoggingConfig) -> bool {
    config.verbosity > 0 && !config.suppress_status
}

fn info_messages_enabled(config: &LoggingConfig) -> bool {
    !config.suppress_status
}
