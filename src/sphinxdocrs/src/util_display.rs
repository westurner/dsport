//! Pure formatting primitives for Sphinx status and progress output.
//!
//! The functions in this module deliberately return strings instead of writing
//! to a process-global stream. That keeps the upstream `util.display` contract
//! deterministic in tests and lets the CLI layer choose stdout/stderr later.

/// Format a display chunk like Sphinx's `display_chunk`.
pub fn display_chunk(items: &[&str]) -> String {
    match items {
        [] => String::new(),
        [item] => (*item).to_owned(),
        [first, .., last] => format!("{first} .. {last}"),
    }
}

/// Render the status lines emitted for one status-iterator run.
///
/// `length = None` is the upstream unknown-length mode. With a known length,
/// verbosity zero uses carriage-return updates and verbosity one uses newline
/// updates. The final line receives the corresponding completion terminator.
pub fn status_iterator_lines(
    items: &[&str],
    prefix: &str,
    length: Option<usize>,
    verbosity: u8,
) -> Vec<String> {
    match length {
        None => vec![format!("{prefix}{} \n", items.join(" "))],
        Some(length) if length == 0 => vec![format!("{prefix}{} \n", items.join(" "))],
        Some(length) => items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let percent = ((index + 1) * 100) / length;
                let terminator = if verbosity == 0 { "\r" } else { "\n" };
                let final_terminator = if index + 1 == items.len() {
                    if verbosity == 0 { "\r\n" } else { "\n\n" }
                } else {
                    terminator
                };
                format!("{prefix}[{percent:>3}%] {item}{final_terminator}")
            })
            .collect(),
    }
}

/// Outcome of a progress-message context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgressOutcome<'a> {
    /// The operation completed normally; `body` is the text emitted inside it.
    Done { body: &'a str },
    /// The operation was intentionally skipped with a reason.
    Skipped { reason: &'a str },
    /// The operation failed and the caller is re-raising its error.
    Failed,
}

/// Render the visible lines for Sphinx's `progress_message` context manager.
pub fn progress_message_lines(name: &str, outcome: ProgressOutcome<'_>) -> Vec<String> {
    match outcome {
        ProgressOutcome::Done { body } => vec![format!("{name}... {body}done\n")],
        ProgressOutcome::Skipped { reason } => {
            vec![format!("{name}... skipped\n"), format!("{reason}\n")]
        }
        ProgressOutcome::Failed => vec![format!("{name}... failed\n")],
    }
}

/// Format the native build completion line shared by direct and make mode.
pub fn build_succeeded(written: usize) -> String {
    format!("Build succeeded: {written} file(s) written.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_chunk_matches_upstream_shapes() {
        assert_eq!(display_chunk(&[]), "");
        assert_eq!(display_chunk(&["hello"]), "hello");
        assert_eq!(
            display_chunk(&["hello", "sphinx", "world"]),
            "hello .. world"
        );
    }

    #[test]
    fn status_iterator_unknown_length_joins_items() {
        assert_eq!(
            status_iterator_lines(&["hello", "sphinx", "world"], "testing ... ", None, 0),
            vec!["testing ... hello sphinx world \n"]
        );
    }

    #[test]
    fn status_iterator_zero_length_uses_unknown_length_format() {
        assert_eq!(
            status_iterator_lines(&["hello"], "testing ... ", Some(0), 0),
            vec!["testing ... hello \n"]
        );
    }

    #[test]
    fn status_iterator_verbosity_zero_uses_carriage_returns() {
        assert_eq!(
            status_iterator_lines(&["hello", "sphinx", "world"], "testing ... ", Some(3), 0),
            vec![
                "testing ... [ 33%] hello\r",
                "testing ... [ 66%] sphinx\r",
                "testing ... [100%] world\r\n",
            ]
        );
    }

    #[test]
    fn status_iterator_verbosity_one_uses_newlines() {
        assert_eq!(
            status_iterator_lines(&["hello", "sphinx", "world"], "testing ... ", Some(3), 1),
            vec![
                "testing ... [ 33%] hello\n",
                "testing ... [ 66%] sphinx\n",
                "testing ... [100%] world\n\n",
            ]
        );
    }

    #[test]
    fn progress_message_formats_done_skip_and_failure() {
        assert_eq!(
            progress_message_lines("testing", ProgressOutcome::Done { body: "blah " }),
            vec!["testing... blah done\n"]
        );
        assert_eq!(
            progress_message_lines(
                "testing",
                ProgressOutcome::Skipped {
                    reason: "Reason: error"
                }
            ),
            vec!["testing... skipped\n", "Reason: error\n"]
        );
        assert_eq!(
            progress_message_lines("testing", ProgressOutcome::Failed),
            vec!["testing... failed\n"]
        );
    }

    #[test]
    fn build_success_includes_file_count() {
        assert_eq!(build_succeeded(2), "Build succeeded: 2 file(s) written.");
    }
}
