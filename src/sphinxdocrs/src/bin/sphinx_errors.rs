//! Import and interactively review Sphinx build diagnostics.

use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use sphinxdocrs::cli::io::RealTerminal;
use sphinxdocrs::error_log::{
    import_file, run_interactive, ErrorDatabase, ErrorLogError, ErrorMessage, InputFormat,
    RealEditor,
};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliFormat {
    Auto,
    Text,
    Json,
    Sqlite,
}

impl From<CliFormat> for InputFormat {
    fn from(value: CliFormat) -> Self {
        match value {
            CliFormat::Auto => Self::Auto,
            CliFormat::Text => Self::Text,
            CliFormat::Json => Self::Json,
            CliFormat::Sqlite => Self::Sqlite,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "sphinx-errors-rs", about = "Review Sphinx build diagnostics")]
struct Args {
    /// A Sphinx text/JSON log, or an existing SQLite database.
    source: Option<PathBuf>,
    /// Review pending messages interactively.
    #[arg(short, long)]
    interactive: bool,
    /// Input format. Auto detects SQLite by extension and JSON by content.
    #[arg(long, value_enum, default_value_t = CliFormat::Auto)]
    format: CliFormat,
    /// Destination SQLite database when importing a log.
    #[arg(short, long)]
    db: Option<PathBuf>,
    /// Restrict review/listing to one build.
    #[arg(long)]
    build_id: Option<i64>,
    /// Include messages whose status is already set.
    #[arg(long)]
    all: bool,
    /// Command to store for an imported log.
    #[arg(long, default_value = "sphinx-build")]
    command: String,
}

fn main() {
    if let Err(error) = run(Args::parse()) {
        eprintln!("sphinx-errors-rs: {error}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), ErrorLogError> {
    let source = args.source.as_deref();
    let input_format = match args.format {
        CliFormat::Auto if source.is_some_and(is_sqlite_path) => InputFormat::Sqlite,
        format => format.into(),
    };

    if input_format == InputFormat::Sqlite {
        let database_path = source.or(args.db.as_deref()).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "a SQLite database is required",
            )
        })?;
        let database = ErrorDatabase::open(database_path)?;
        return review_or_list(&database, &args);
    }

    let source = source.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "a log file is required when importing",
        )
    })?;
    let database_path = args.db.as_deref().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "--db is required when importing a log",
        )
    })?;
    let build_id = import_file(database_path, source, input_format, &args.command)?;
    let database = ErrorDatabase::open(database_path)?;
    println!(
        "Imported build {build_id} with {} message(s).",
        database.list_errors(Some(build_id), false)?.len()
    );
    if args.interactive {
        review(&database, Some(build_id), args.all)?;
    }
    Ok(())
}

fn review_or_list(database: &ErrorDatabase, args: &Args) -> Result<(), ErrorLogError> {
    if args.interactive {
        review(database, args.build_id, args.all)
    } else {
        let errors = database.list_errors(args.build_id, !args.all)?;
        for error in errors {
            print_error(&error);
        }
        Ok(())
    }
}

fn review(database: &ErrorDatabase, build_id: Option<i64>, all: bool) -> Result<(), ErrorLogError> {
    let terminal = RealTerminal;
    let editor = RealEditor;
    run_interactive(database, &terminal, &editor, build_id, !all)?;
    Ok(())
}

fn print_error(error: &ErrorMessage) {
    let location = match (error.path.is_empty(), error.line, error.char) {
        (false, Some(line), Some(char_number)) => {
            format!("{}:{}:{}", error.path, line, char_number)
        }
        (false, Some(line), None) => format!("{}:{}", error.path, line),
        (false, None, _) => error.path.clone(),
        (true, _, _) => "<build>".to_owned(),
    };
    println!("{} [{}] {}", location, error.status, error.msg);
}

fn is_sqlite_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("db") | Some("sqlite") | Some("sqlite3")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_formats_convert_to_input_formats() {
        assert_eq!(InputFormat::from(CliFormat::Auto), InputFormat::Auto);
        assert_eq!(InputFormat::from(CliFormat::Text), InputFormat::Text);
        assert_eq!(InputFormat::from(CliFormat::Json), InputFormat::Json);
        assert_eq!(InputFormat::from(CliFormat::Sqlite), InputFormat::Sqlite);
    }

    #[test]
    fn print_error_formats_all_location_shapes() {
        for (path, line, char_number) in [
            ("index.rst", Some(4), Some(2)),
            ("index.rst", Some(4), None),
            ("index.rst", None, Some(2)),
            ("", None, None),
        ] {
            print_error(&ErrorMessage {
                id: 0,
                build_id: 0,
                status: String::new(),
                path: path.to_owned(),
                line,
                char: char_number,
                msg: "diagnostic".to_owned(),
            });
        }
    }

    #[test]
    fn sqlite_path_detection_accepts_supported_extensions_only() {
        assert!(is_sqlite_path(Path::new("errors.db")));
        assert!(is_sqlite_path(Path::new("errors.sqlite")));
        assert!(is_sqlite_path(Path::new("errors.sqlite3")));
        assert!(!is_sqlite_path(Path::new("errors.json")));
        assert!(!is_sqlite_path(Path::new("errors")));
    }
}
