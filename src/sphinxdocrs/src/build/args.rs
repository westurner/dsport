//! Parsed build arguments and the `_parse_*` helper functions.
//!
//! All functions are pure (no I/O) so they can be unit-tested without
//! touching the filesystem.

use std::collections::HashMap;
use std::path::PathBuf;

/// Parsed and validated arguments for a `sphinx-build` invocation.
#[derive(Debug, Clone)]
pub struct BuildArgs {
    pub sourcedir: PathBuf,
    pub outputdir: PathBuf,
    pub filenames: Vec<String>,
    pub builder: String,
    pub jobs: usize,
    pub force_all: bool,
    pub freshenv: bool,
    pub doctreedir: PathBuf,
    pub confdir: Option<PathBuf>,
    pub noconfig: bool,
    pub confoverrides: HashMap<String, ConfValue>,
    pub tags: Vec<String>,
    pub verbosity: u8,
    pub quiet: bool,
    pub really_quiet: bool,
    /// "yes" | "no" | "auto"
    pub color: String,
    pub warnfile: Option<PathBuf>,
    pub warningiserror: bool,
    pub keep_going: bool,
    pub traceback: bool,
    pub pdb: bool,
    pub exception_on_warning: bool,
    /// When `true`, scan `conf.py` for required packages and exit.
    pub scan_requirements: bool,
}

/// A config override value: either a string or an int (from `-A name=value`
/// where value parses as int).
#[derive(Debug, Clone, PartialEq)]
pub enum ConfValue {
    Str(String),
    Int(i64),
}

impl std::fmt::Display for ConfValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfValue::Str(s) => f.write_str(s),
            ConfValue::Int(i) => write!(f, "{i}"),
        }
    }
}

/// Parse `argv` (excluding the program name) into a [`BuildArgs`].
///
/// Calls the clap parser and then the `_parse_*` helpers.
pub fn parse_args(argv: &[String]) -> Result<BuildArgs, ParseError> {
    let mut full_argv = vec!["sphinx-build".to_owned()];
    full_argv.extend_from_slice(argv);

    let cmd = super::parser::build_parser();
    let m = cmd
        .try_get_matches_from(&full_argv)
        .map_err(|e| ParseError::Clap(e.to_string()))?;

    let sourcedir = PathBuf::from(m.get_one::<String>("sourcedir").unwrap());
    let outputdir = PathBuf::from(m.get_one::<String>("outputdir").unwrap());
    let filenames: Vec<String> = m
        .get_many::<String>("filenames")
        .unwrap_or_default()
        .cloned()
        .collect();
    let builder = m.get_one::<String>("builder").unwrap().clone();
    let jobs_str = m.get_one::<String>("jobs").unwrap().clone();
    let jobs = jobs_argument(&jobs_str).map_err(ParseError::Validation)?;
    let force_all = m.get_flag("force_all");
    let freshenv = m.get_flag("freshenv");
    let noconfig = m.get_flag("noconfig");

    let doctreedir = parse_doctreedir(
        m.get_one::<String>("doctreedir").map(|s| s.as_str()),
        &outputdir,
    );
    let confdir_raw = m.get_one::<String>("confdir").map(|s| s.as_str());
    let confdir = parse_confdir(noconfig, confdir_raw, &sourcedir);

    validate_filenames(force_all, &filenames)?;

    let color = parse_color(m.get_flag("color"), m.get_flag("no_color"));
    let warnfile = m.get_one::<String>("warnfile").map(PathBuf::from);

    let define: Vec<String> = m
        .get_many::<String>("define")
        .unwrap_or_default()
        .cloned()
        .collect();
    let htmldefine: Vec<String> = m
        .get_many::<String>("htmldefine")
        .unwrap_or_default()
        .cloned()
        .collect();
    let nitpicky = m.get_flag("nitpicky");
    let confoverrides = parse_confoverrides(&define, &htmldefine, nitpicky)?;

    let tags: Vec<String> = m
        .get_many::<String>("tags")
        .unwrap_or_default()
        .cloned()
        .collect();
    let verbosity = m.get_count("verbosity");

    Ok(BuildArgs {
        sourcedir,
        outputdir,
        filenames,
        builder,
        jobs,
        force_all,
        freshenv,
        doctreedir,
        confdir,
        noconfig,
        confoverrides,
        tags,
        verbosity,
        quiet: m.get_flag("quiet"),
        really_quiet: m.get_flag("really_quiet"),
        color,
        warnfile,
        warningiserror: m.get_flag("warningiserror"),
        keep_going: m.get_flag("keep_going"),
        traceback: m.get_flag("traceback"),
        pdb: m.get_flag("pdb"),
        exception_on_warning: m.get_flag("exception_on_warning"),
        scan_requirements: m.get_flag("scan_requirements"),
    })
}

/// Mirrors `jobs_argument(value)`: `"auto"` → CPU count; positive int → that
/// count; 0 or negative → error.
pub fn jobs_argument(value: &str) -> Result<usize, String> {
    if value == "auto" {
        return Ok(num_cpus::get());
    }
    match value.parse::<i64>() {
        Ok(n) if n > 0 => Ok(n as usize),
        _ => Err("job number should be a positive number".to_owned()),
    }
}

/// Mirrors `_parse_confdir(noconfig, confdir, sourcedir)`.
pub fn parse_confdir(
    noconfig: bool,
    confdir: Option<&str>,
    sourcedir: &std::path::Path,
) -> Option<PathBuf> {
    if noconfig {
        None
    } else if let Some(c) = confdir {
        if c.is_empty() {
            Some(sourcedir.to_path_buf())
        } else {
            Some(PathBuf::from(c))
        }
    } else {
        Some(sourcedir.to_path_buf())
    }
}

/// Mirrors `_parse_doctreedir(doctreedir, outputdir)`.
pub fn parse_doctreedir(doctreedir: Option<&str>, outputdir: &std::path::Path) -> PathBuf {
    match doctreedir {
        Some(d) if !d.is_empty() => PathBuf::from(d),
        _ => outputdir.join(".doctrees"),
    }
}

/// Mirrors `_validate_filenames(parser, force_all, filenames)`.
pub fn validate_filenames(force_all: bool, filenames: &[String]) -> Result<(), ParseError> {
    if force_all && !filenames.is_empty() {
        Err(ParseError::Validation(
            "cannot combine -a option and filenames".to_owned(),
        ))
    } else {
        Ok(())
    }
}

/// Resolve color mode: "yes" | "no" | "auto".
pub fn parse_color(color_flag: bool, no_color_flag: bool) -> String {
    if no_color_flag {
        "no".to_owned()
    } else if color_flag {
        "yes".to_owned()
    } else {
        "auto".to_owned()
    }
}

/// Mirrors `_parse_confoverrides(parser, define, htmldefine, nitpicky)`.
///
/// `-D name=value` → string override.
/// `-A name=value` → int override if `value` parses as int, else string,
///   stored under `html_context.<name>`.
pub fn parse_confoverrides(
    define: &[String],
    htmldefine: &[String],
    nitpicky: bool,
) -> Result<HashMap<String, ConfValue>, ParseError> {
    let mut overrides = HashMap::new();

    for val in define {
        let (key, value) = split_kv(val, "-D")?;
        overrides.insert(key, ConfValue::Str(value));
    }

    for val in htmldefine {
        let (key, value) = split_kv(val, "-A")?;
        let cv = if let Ok(i) = value.parse::<i64>() {
            ConfValue::Int(i)
        } else {
            ConfValue::Str(value)
        };
        overrides.insert(format!("html_context.{key}"), cv);
    }

    if nitpicky {
        overrides.insert("nitpicky".to_owned(), ConfValue::Int(1));
    }

    Ok(overrides)
}

fn split_kv(s: &str, flag: &str) -> Result<(String, String), ParseError> {
    match s.find('=') {
        Some(pos) => Ok((s[..pos].to_owned(), s[pos + 1..].to_owned())),
        None => Err(ParseError::Validation(format!(
            "{flag} option argument must be in the form name=value"
        ))),
    }
}

/// Parse errors.
#[derive(Debug)]
pub enum ParseError {
    Clap(String),
    Validation(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Clap(s) | ParseError::Validation(s) => f.write_str(s),
        }
    }
}
impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    // ── jobs_argument ──────────────────────────────────────────────────────
    #[test]
    fn jobs_auto_returns_cpu_count() {
        let n = jobs_argument("auto").unwrap();
        assert!(n >= 1);
    }

    #[test]
    fn jobs_positive_int() {
        assert_eq!(jobs_argument("4"), Ok(4));
        assert_eq!(jobs_argument("1"), Ok(1));
    }

    #[test]
    fn jobs_zero_is_error() {
        assert!(jobs_argument("0").is_err());
        let e = jobs_argument("0").unwrap_err();
        assert!(e.contains("positive number"), "got: {e}");
    }

    #[test]
    fn jobs_negative_is_error() {
        assert!(jobs_argument("-1").is_err());
    }

    #[test]
    fn jobs_non_numeric_is_error() {
        assert!(jobs_argument("abc").is_err());
    }

    // ── parse_confdir ──────────────────────────────────────────────────────
    #[test]
    fn confdir_noconfig_returns_none() {
        let src = PathBuf::from("/src");
        assert_eq!(parse_confdir(true, None, &src), None);
    }

    #[test]
    fn confdir_explicit() {
        let src = PathBuf::from("/src");
        assert_eq!(
            parse_confdir(false, Some("/cfg"), &src),
            Some(PathBuf::from("/cfg"))
        );
    }

    #[test]
    fn confdir_default_is_sourcedir() {
        let src = PathBuf::from("/src");
        assert_eq!(parse_confdir(false, None, &src), Some(src.clone()));
    }

    // ── parse_doctreedir ──────────────────────────────────────────────────
    #[test]
    fn doctreedir_default() {
        let out = PathBuf::from("/out");
        assert_eq!(
            parse_doctreedir(None, &out),
            PathBuf::from("/out/.doctrees")
        );
    }

    #[test]
    fn doctreedir_explicit() {
        let out = PathBuf::from("/out");
        assert_eq!(
            parse_doctreedir(Some("/dtree"), &out),
            PathBuf::from("/dtree")
        );
    }

    // ── validate_filenames ────────────────────────────────────────────────
    #[test]
    fn filenames_no_conflict() {
        assert!(validate_filenames(false, &["file.rst".to_owned()]).is_ok());
        assert!(validate_filenames(true, &[]).is_ok());
    }

    #[test]
    fn filenames_force_all_with_files_errors() {
        let e = validate_filenames(true, &["f.rst".to_owned()]).unwrap_err();
        assert!(e.to_string().contains("cannot combine"), "got: {e}");
    }

    // ── parse_confoverrides ───────────────────────────────────────────────
    #[test]
    fn define_key_value() {
        let ovr = parse_confoverrides(&["language=fr".to_owned()], &[], false).unwrap();
        assert_eq!(ovr.get("language"), Some(&ConfValue::Str("fr".to_owned())));
    }

    #[test]
    fn htmldefine_int() {
        let ovr = parse_confoverrides(&[], &["myvar=42".to_owned()], false).unwrap();
        assert_eq!(ovr.get("html_context.myvar"), Some(&ConfValue::Int(42)));
    }

    #[test]
    fn htmldefine_string() {
        let ovr = parse_confoverrides(&[], &["title=Foo Bar".to_owned()], false).unwrap();
        assert_eq!(
            ovr.get("html_context.title"),
            Some(&ConfValue::Str("Foo Bar".to_owned()))
        );
    }

    #[test]
    fn nitpicky_sets_override() {
        let ovr = parse_confoverrides(&[], &[], true).unwrap();
        assert_eq!(ovr.get("nitpicky"), Some(&ConfValue::Int(1)));
    }

    #[test]
    fn define_missing_equals_errors() {
        let r = parse_confoverrides(&["noeqsign".to_owned()], &[], false);
        assert!(r.is_err());
        assert!(r.unwrap_err().to_string().contains("name=value"));
    }

    // ── parse_color ───────────────────────────────────────────────────────
    #[test]
    fn color_auto_by_default() {
        assert_eq!(parse_color(false, false), "auto");
    }

    #[test]
    fn color_no() {
        assert_eq!(parse_color(false, true), "no");
    }

    #[test]
    fn color_yes() {
        assert_eq!(parse_color(true, false), "yes");
    }
}
