//! Disabled SQLite error database API.
//!
//! The `sphinx-errors-rs` binary remains available in minimal builds, but
//! database operations require the opt-in `sqlite-error-db` feature.

use std::path::Path;

use crate::cli::io::Terminal;

#[derive(Debug)]
pub struct ErrorLogError;

impl std::fmt::Display for ErrorLogError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .write_str("SQLite error logging is disabled; rebuild with --features sqlite-error-db")
    }
}

impl std::error::Error for ErrorLogError {}

impl From<std::io::Error> for ErrorLogError {
    fn from(_error: std::io::Error) -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorMessage {
    pub id: i64,
    pub build_id: i64,
    pub status: String,
    pub path: String,
    pub line: Option<i64>,
    pub char: Option<i64>,
    pub msg: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Auto,
    Text,
    Json,
    Sqlite,
}

pub struct ErrorDatabase;

impl ErrorDatabase {
    pub fn open(_path: impl AsRef<Path>) -> Result<Self, ErrorLogError> {
        Err(ErrorLogError)
    }

    pub fn list_errors(
        &self,
        _build_id: Option<i64>,
        _pending_only: bool,
    ) -> Result<Vec<ErrorMessage>, ErrorLogError> {
        Err(ErrorLogError)
    }
}

pub struct RealEditor;

pub fn import_file(
    _database_path: impl AsRef<Path>,
    _input_path: impl AsRef<Path>,
    _format: InputFormat,
    _command: &str,
) -> Result<i64, ErrorLogError> {
    Err(ErrorLogError)
}

pub fn run_interactive<T: Terminal, E>(
    _database: &ErrorDatabase,
    _terminal: &T,
    _editor: &E,
    _build_id: Option<i64>,
    _pending_only: bool,
) -> Result<usize, ErrorLogError> {
    Err(ErrorLogError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    struct NoopTerminal;

    impl Terminal for NoopTerminal {
        fn print(&self, _line: &str) {}

        fn prompt(&self, _prompt_text: &str) -> io::Result<String> {
            Ok(String::new())
        }
    }

    #[test]
    fn disabled_database_api_returns_the_feature_guidance_error() {
        let error = ErrorLogError;
        assert_eq!(
            error.to_string(),
            "SQLite error logging is disabled; rebuild with --features sqlite-error-db"
        );
        assert!(std::error::Error::source(&error).is_none());
        assert_eq!(
            ErrorLogError::from(io::Error::other("ignored")).to_string(),
            error.to_string()
        );

        assert!(ErrorDatabase::open("ignored.db").is_err());
        assert!(ErrorDatabase.list_errors(None, false).is_err());
        assert!(
            import_file(
                "ignored.db",
                "ignored.log",
                InputFormat::Auto,
                "sphinx-build"
            )
            .is_err()
        );

        let terminal = NoopTerminal;
        terminal.print("ignored");
        assert!(terminal.prompt("ignored").is_ok());
        assert!(run_interactive(&ErrorDatabase, &terminal, &(), None, false).is_err());
    }
}
