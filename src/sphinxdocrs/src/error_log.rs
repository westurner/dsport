//! SQLite-backed Sphinx build diagnostics and an interactive review workflow.

use std::io;
use std::path::Path;

use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, params};
use serde_json::Value;
use thiserror::Error;

use crate::cli::io::Terminal;
use crate::util_strypes::strip_escape_sequences;

pub const STATUS_EDITED: &str = "edited";
pub const STATUS_SKIPPED: &str = "skipped";

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS SphinxBuild (
    id INTEGER PRIMARY KEY,
    dateCreated TEXT NOT NULL,
    dateUpdated TEXT NOT NULL,
    command TEXT NOT NULL,
    logpath TEXT NOT NULL DEFAULT '',
    logcontent TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS ErrorMessage (
    id INTEGER PRIMARY KEY,
    build_id INTEGER NOT NULL REFERENCES SphinxBuild(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT '',
    path TEXT NOT NULL DEFAULT '',
    line INTEGER,
    char INTEGER,
    msg TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sphinx_build_date_created
    ON SphinxBuild(dateCreated);
CREATE INDEX IF NOT EXISTS idx_sphinx_build_date_updated
    ON SphinxBuild(dateUpdated);
CREATE INDEX IF NOT EXISTS idx_error_message_build_id
    ON ErrorMessage(build_id);
CREATE INDEX IF NOT EXISTS idx_error_message_status
    ON ErrorMessage(status);
CREATE INDEX IF NOT EXISTS idx_error_message_path
    ON ErrorMessage(path);
CREATE INDEX IF NOT EXISTS idx_error_message_build_status
    ON ErrorMessage(build_id, status);
"#;

#[derive(Debug, Error)]
pub enum ErrorLogError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported diagnostic JSON record")]
    UnsupportedJson,
}

pub type Result<T> = std::result::Result<T, ErrorLogError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SphinxBuild {
    pub id: i64,
    pub date_created: String,
    pub date_updated: String,
    pub command: String,
    pub logpath: String,
    pub logcontent: String,
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

impl ErrorMessage {
    pub fn new(
        path: impl Into<String>,
        line: Option<i64>,
        char: Option<i64>,
        msg: impl Into<String>,
    ) -> Self {
        Self {
            id: 0,
            build_id: 0,
            status: String::new(),
            path: path.into(),
            line,
            char,
            msg: msg.into(),
        }
    }
}

pub struct ErrorDatabase {
    connection: Connection,
}

impl ErrorDatabase {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let connection = Connection::open(path)?;
        connection.execute_batch(SCHEMA)?;
        connection.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(Self { connection })
    }

    pub fn open_in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        connection.execute_batch(SCHEMA)?;
        connection.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(Self { connection })
    }

    pub fn schema() -> &'static str {
        SCHEMA
    }

    pub fn insert_build(
        &self,
        command: &str,
        logpath: Option<&Path>,
        logcontent: &str,
    ) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        self.connection.execute(
            "INSERT INTO SphinxBuild (dateCreated, dateUpdated, command, logpath, logcontent)
             VALUES (?1, ?1, ?2, ?3, ?4)",
            params![
                now,
                command,
                logpath.map(path_string).unwrap_or_default(),
                logcontent
            ],
        )?;
        Ok(self.connection.last_insert_rowid())
    }

    pub fn insert_errors(&mut self, build_id: i64, errors: &[ErrorMessage]) -> Result<usize> {
        let transaction = self.connection.transaction()?;
        let mut statement = transaction.prepare(
            "INSERT INTO ErrorMessage (build_id, status, path, line, char, msg)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;
        for error in errors {
            statement.execute(params![
                build_id,
                error.status,
                error.path,
                error.line,
                error.char,
                error.msg,
            ])?;
        }
        drop(statement);
        transaction.commit()?;
        Ok(errors.len())
    }

    pub fn record_log(
        &mut self,
        command: &str,
        logpath: Option<&Path>,
        logcontent: &str,
        errors: &[ErrorMessage],
    ) -> Result<i64> {
        let build_id = self.insert_build(command, logpath, logcontent)?;
        let mut errors = errors.to_vec();
        for error in &mut errors {
            error.build_id = build_id;
        }
        self.insert_errors(build_id, &errors)?;
        Ok(build_id)
    }

    pub fn build(&self, id: i64) -> Result<Option<SphinxBuild>> {
        self.connection
            .query_row(
                "SELECT id, dateCreated, dateUpdated, command, logpath, logcontent
                 FROM SphinxBuild WHERE id = ?1",
                [id],
                |row| {
                    Ok(SphinxBuild {
                        id: row.get(0)?,
                        date_created: row.get(1)?,
                        date_updated: row.get(2)?,
                        command: row.get(3)?,
                        logpath: row.get(4)?,
                        logcontent: row.get(5)?,
                    })
                },
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn list_errors(
        &self,
        build_id: Option<i64>,
        pending_only: bool,
    ) -> Result<Vec<ErrorMessage>> {
        let mut errors = Vec::new();
        let mut query =
            String::from("SELECT id, build_id, status, path, line, char, msg FROM ErrorMessage");
        let mut conditions = Vec::new();
        if build_id.is_some() {
            conditions.push("build_id = ?1");
        }
        if pending_only {
            conditions.push("status = ''");
        }
        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }
        query.push_str(" ORDER BY build_id, id");
        let mut statement = self.connection.prepare(&query)?;
        let rows = if let Some(build_id) = build_id {
            statement.query_map([build_id], error_from_row)?
        } else {
            statement.query_map([], error_from_row)?
        };
        for row in rows {
            errors.push(row?);
        }
        Ok(errors)
    }

    pub fn update_status(&self, error_id: i64, status: &str) -> Result<bool> {
        let now = Utc::now().to_rfc3339();
        let changed = self.connection.execute(
            "UPDATE ErrorMessage SET status = ?1 WHERE id = ?2",
            params![status, error_id],
        )?;
        if changed > 0 {
            self.connection.execute(
                "UPDATE SphinxBuild SET dateUpdated = ?1
                 WHERE id = (SELECT build_id FROM ErrorMessage WHERE id = ?2)",
                params![now, error_id],
            )?;
        }
        Ok(changed > 0)
    }
}

fn error_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ErrorMessage> {
    Ok(ErrorMessage {
        id: row.get(0)?,
        build_id: row.get(1)?,
        status: row.get(2)?,
        path: row.get(3)?,
        line: row.get(4)?,
        char: row.get(5)?,
        msg: row.get(6)?,
    })
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

pub fn parse_text(log: &str) -> Vec<ErrorMessage> {
    let mut result = Vec::new();
    for raw_line in log.lines() {
        let line = strip_escape_sequences(raw_line).trim().to_owned();
        if let Some(error) = parse_location_line(&line) {
            result.push(error);
        } else if let Some(error) = parse_native_line(&line) {
            result.push(error);
        }
    }
    result
}

fn parse_location_line(line: &str) -> Option<ErrorMessage> {
    let levels = ["WARNING", "ERROR", "CRITICAL", "SEVERE"];
    let (prefix, msg) = levels.iter().find_map(|level| {
        let marker = format!(": {level}:");
        let index = line.find(&marker)?;
        Some((&line[..index], line[index + marker.len()..].trim()))
    })?;
    let (path, line_number, char_number) = match prefix.rsplit_once(':') {
        Some((before_last, last)) if last.parse::<i64>().is_ok() => {
            let last = last.parse::<i64>().ok()?;
            match before_last.rsplit_once(':') {
                Some((path, line)) if line.parse::<i64>().is_ok() => {
                    (path.to_owned(), Some(line.parse().ok()?), Some(last))
                }
                _ => (before_last.to_owned(), Some(last), None),
            }
        }
        _ => (prefix.to_owned(), None, None),
    };
    Some(ErrorMessage::new(path, line_number, char_number, msg))
}

fn parse_native_line(line: &str) -> Option<ErrorMessage> {
    let (level, rest) = line.split_once(':')?;
    if !matches!(level.trim(), "WARNING" | "ERROR" | "CRITICAL" | "SEVERE") {
        return None;
    }
    let rest = rest.trim();
    if let Some((path, msg)) = rest.split_once(": ") {
        Some(ErrorMessage::new(path.trim(), None, None, msg.trim()))
    } else {
        Some(ErrorMessage::new("", None, None, rest))
    }
}

pub fn parse_json(input: &str) -> Result<Vec<ErrorMessage>> {
    let value: Value = match serde_json::from_str(input) {
        Ok(value) => value,
        Err(error) => {
            let mut records = Vec::new();
            for line in input.lines().filter(|line| !line.trim().is_empty()) {
                match serde_json::from_str(line) {
                    Ok(value) => records.push(value),
                    Err(_) => return Err(error.into()),
                }
            }
            return parse_json_records(records);
        }
    };
    let records = match value {
        Value::Array(records) => records,
        Value::Object(mut object) => {
            for key in ["errors", "warnings", "messages", "diagnostics"] {
                if let Some(Value::Array(records)) = object.remove(key) {
                    return parse_json_records(records);
                }
            }
            vec![Value::Object(object)]
        }
        _ => return Err(ErrorLogError::UnsupportedJson),
    };
    parse_json_records(records)
}

fn parse_json_records(records: Vec<Value>) -> Result<Vec<ErrorMessage>> {
    records.iter().map(parse_json_record).collect()
}

fn parse_json_record(value: &Value) -> Result<ErrorMessage> {
    let object = value.as_object().ok_or(ErrorLogError::UnsupportedJson)?;
    let message = ["msg", "message", "text", "description"]
        .iter()
        .find_map(|key| object.get(*key).and_then(value_string))
        .ok_or(ErrorLogError::UnsupportedJson)?;

    let location = object.get("location");
    let path = ["path", "filepath", "file", "filename"]
        .iter()
        .find_map(|key| object.get(*key).and_then(value_string))
        .or_else(|| {
            location
                .and_then(|value| value.get("path"))
                .and_then(value_string)
        })
        .or_else(|| {
            location
                .and_then(|value| value.get("filepath"))
                .and_then(value_string)
        })
        .or_else(|| {
            location
                .and_then(|value| value.get("uri"))
                .and_then(value_string)
        })
        .unwrap_or_default();
    let line = ["line", "lineno", "line_number"]
        .iter()
        .find_map(|key| object.get(*key).and_then(value_i64))
        .or_else(|| {
            location
                .and_then(|value| value.get("line"))
                .and_then(value_i64)
        })
        .or_else(|| {
            location
                .and_then(|value| value.get("lineno"))
                .and_then(value_i64)
        });
    let char_number = ["char", "column", "col", "charno"]
        .iter()
        .find_map(|key| object.get(*key).and_then(value_i64))
        .or_else(|| {
            location
                .and_then(|value| value.get("column"))
                .and_then(value_i64)
        })
        .or_else(|| {
            location
                .and_then(|value| value.get("char"))
                .and_then(value_i64)
        });

    Ok(ErrorMessage::new(path, line, char_number, message))
}

fn value_string(value: &Value) -> Option<String> {
    value.as_str().map(str::to_owned)
}

fn value_i64(value: &Value) -> Option<i64> {
    value.as_i64().or_else(|| value.as_str()?.parse().ok())
}

pub fn parse_log(input: &str, format: InputFormat) -> Result<Vec<ErrorMessage>> {
    match format {
        InputFormat::Text => Ok(parse_text(input)),
        InputFormat::Json => parse_json(input),
        InputFormat::Auto => {
            if matches!(input.trim_start().chars().next(), Some('{') | Some('[')) {
                parse_json(input).or_else(|_| Ok(parse_text(input)))
            } else {
                Ok(parse_text(input))
            }
        }
        InputFormat::Sqlite => Err(ErrorLogError::UnsupportedJson),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Auto,
    Text,
    Json,
    Sqlite,
}

pub fn import_file(
    database_path: impl AsRef<Path>,
    input_path: impl AsRef<Path>,
    format: InputFormat,
    command: &str,
) -> Result<i64> {
    let input_path = input_path.as_ref();
    let content = std::fs::read_to_string(input_path)?;
    let errors = parse_log(&content, format)?;
    let mut database = ErrorDatabase::open(database_path)?;
    database.record_log(command, Some(input_path), &content, &errors)
}

pub fn record_native_build(
    database_path: impl AsRef<Path>,
    command: &str,
    logpath: Option<&Path>,
    warnings: &[String],
    build_error: Option<&str>,
) -> Result<i64> {
    let mut lines: Vec<String> = warnings
        .iter()
        .map(|warning| format!("WARNING: {warning}"))
        .collect();
    let mut errors = parse_text(&lines.join("\n"));
    if let Some(error) = build_error {
        lines.push(format!("ERROR: {error}"));
        errors.push(ErrorMessage::new("", None, None, error));
    }
    let logcontent = lines.join("\n");
    let mut database = ErrorDatabase::open(database_path)?;
    database.record_log(command, logpath, &logcontent, &errors)
}

pub trait Editor: Send + Sync {
    fn open(&self, error: &ErrorMessage) -> io::Result<()>;
}

pub struct RealEditor;

impl Editor for RealEditor {
    fn open(&self, error: &ErrorMessage) -> io::Result<()> {
        if error.path.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "diagnostic has no source path",
            ));
        }
        let command = std::env::var("VISUAL")
            .or_else(|_| std::env::var("EDITOR"))
            .unwrap_or_else(|_| "vi".to_owned());
        let mut parts = command.split_whitespace();
        let executable = parts.next().unwrap_or("vi");
        let mut process = std::process::Command::new(executable);
        process.args(parts);
        let executable_name = Path::new(executable)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(executable);
        if matches!(executable_name, "code" | "code-insiders" | "codium") {
            let location = format!(
                "{}:{}:{}",
                error.path,
                error.line.unwrap_or(1),
                error.char.unwrap_or(1)
            );
            process.args(["--goto", &location]);
        } else if matches!(executable_name, "vi" | "vim" | "nvim") {
            if let Some(line) = error.line {
                process.arg(format!("+{line}"));
            }
            process.arg(&error.path);
        } else {
            process.arg(&error.path);
        }
        let status = process.status()?;
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::other(format!("editor exited with {status}")))
        }
    }
}

pub fn run_interactive<T: Terminal, E: Editor>(
    database: &ErrorDatabase,
    terminal: &T,
    editor: &E,
    build_id: Option<i64>,
    pending_only: bool,
) -> Result<usize> {
    let mut current = 0usize;
    let mut changed = 0usize;
    loop {
        let errors = database.list_errors(build_id, pending_only)?;
        if errors.is_empty() {
            terminal.print("No error messages to review.");
            return Ok(changed);
        }
        current = current.min(errors.len() - 1);
        let error = &errors[current];
        let location = format_location(error);
        let prompt = format!(
            "[{}/{}] {}{}\n[e]dit [s]kip [n]ext [p]rev [q]uit: ",
            current + 1,
            errors.len(),
            location,
            error.msg
        );
        match terminal
            .prompt(&prompt)?
            .trim()
            .to_ascii_lowercase()
            .as_str()
        {
            "e" | "edit" => {
                if let Err(error) = editor.open(error) {
                    terminal.print(&format!("Editor failed: {error}"));
                } else {
                    database.update_status(error.id, STATUS_EDITED)?;
                    changed += 1;
                }
            }
            "s" | "skip" => {
                database.update_status(error.id, STATUS_SKIPPED)?;
                changed += 1;
            }
            "n" | "next" => current = (current + 1).min(errors.len() - 1),
            "p" | "prev" | "previous" => current = current.saturating_sub(1),
            "q" | "quit" | "" => return Ok(changed),
            _ => terminal.print("Choose e, s, n, p, or q."),
        }
    }
}

fn format_location(error: &ErrorMessage) -> String {
    match (error.path.is_empty(), error.line, error.char) {
        (false, Some(line), Some(char_number)) => {
            format!("{}:{}:{}: ", error.path, line, char_number)
        }
        (false, Some(line), None) => format!("{}:{}: ", error.path, line),
        (false, None, _) => format!("{}: ", error.path),
        (true, _, _) => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::io::Terminal;
    use std::sync::Mutex;

    struct FakeTerminal {
        responses: Mutex<Vec<String>>,
        output: Mutex<Vec<String>>,
    }

    impl FakeTerminal {
        fn new(responses: &[&str]) -> Self {
            Self {
                responses: Mutex::new(responses.iter().rev().map(|s| (*s).to_owned()).collect()),
                output: Mutex::new(Vec::new()),
            }
        }
    }

    impl Terminal for FakeTerminal {
        fn print(&self, line: &str) {
            self.output.lock().unwrap().push(line.to_owned());
        }

        fn prompt(&self, _: &str) -> io::Result<String> {
            Ok(self
                .responses
                .lock()
                .unwrap()
                .pop()
                .unwrap_or_else(|| "q".into()))
        }
    }

    struct FakeEditor;

    impl Editor for FakeEditor {
        fn open(&self, _: &ErrorMessage) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn text_parser_accepts_sphinx_and_native_locations() {
        let errors = parse_text(
            "/tmp/index.rst:12:4: WARNING: bad title\n/tmp/guide.rst: WARNING: missing toctree\nWARNING: guide: native warning\n",
        );
        assert_eq!(errors.len(), 3);
        assert_eq!(errors[0].path, "/tmp/index.rst");
        assert_eq!(errors[0].line, Some(12));
        assert_eq!(errors[0].char, Some(4));
        assert_eq!(errors[1].path, "/tmp/guide.rst");
        assert_eq!(errors[1].msg, "missing toctree");
        assert_eq!(errors[2].path, "guide");
        assert_eq!(errors[2].msg, "native warning");
    }

    #[test]
    fn json_parser_accepts_wrapped_locations() {
        let errors = parse_json(
            r#"{"errors":[{"message":"bad title","location":{"filepath":"index.rst","line":3,"column":2}}]}"#,
        )
        .unwrap();
        assert_eq!(errors[0].path, "index.rst");
        assert_eq!(errors[0].line, Some(3));
        assert_eq!(errors[0].char, Some(2));
    }

    #[test]
    fn json_parser_accepts_json_lines() {
        let errors = parse_json(
            "{\"path\":\"one.rst\",\"line\":1,\"message\":\"first\"}\n{\"path\":\"two.rst\",\"line\":2,\"message\":\"second\"}",
        )
        .unwrap();
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[1].path, "two.rst");
    }

    #[test]
    fn database_creates_schema_indexes_and_updates_status() {
        let mut database = ErrorDatabase::open_in_memory().unwrap();
        let error = ErrorMessage::new("index.rst", Some(4), None, "bad title");
        let build_id = database
            .record_log("sphinx-build", None, "log", &[error])
            .unwrap();
        assert_eq!(database.list_errors(Some(build_id), true).unwrap().len(), 1);
        let error_id = database.list_errors(None, true).unwrap()[0].id;
        assert!(database.update_status(error_id, STATUS_SKIPPED).unwrap());
        assert!(
            database
                .list_errors(Some(build_id), true)
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            database.list_errors(Some(build_id), false).unwrap()[0].status,
            STATUS_SKIPPED
        );
        let indexes: Vec<String> = database
            .connection
            .prepare("SELECT name FROM sqlite_master WHERE type = 'index' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(|row| row.unwrap())
            .collect();
        assert!(
            indexes
                .iter()
                .any(|name| name == "idx_error_message_build_status")
        );
    }

    #[test]
    fn workflow_marks_skip_and_edit() {
        let mut database = ErrorDatabase::open_in_memory().unwrap();
        let build_id = database
            .record_log(
                "sphinx-build",
                None,
                "log",
                &[
                    ErrorMessage::new("one.rst", Some(1), None, "first"),
                    ErrorMessage::new("two.rst", Some(2), None, "second"),
                ],
            )
            .unwrap();
        let terminal = FakeTerminal::new(&["s", "e"]);
        run_interactive(&database, &terminal, &FakeEditor, Some(build_id), true).unwrap();
        let errors = database.list_errors(Some(build_id), false).unwrap();
        assert_eq!(errors[0].status, STATUS_SKIPPED);
        assert_eq!(errors[1].status, STATUS_EDITED);
    }
}
