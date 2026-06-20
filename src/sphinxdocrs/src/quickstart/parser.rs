//! Argument parser for `sphinx-quickstart-rs`.
//!
//! Mirrors `sphinx.cmd.quickstart.get_parser()` flag-for-flag using
//! [`clap`]. Parsed args are then normalised into
//! [`QuickstartSettings`].

use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};

use crate::quickstart::settings::{EXTENSIONS, QuickstartSettings};

/// Build the clap [`Command`] mirroring upstream `get_parser()`.
pub fn build_parser() -> Command {
    let mut cmd = Command::new("sphinx-quickstart")
        .about(
            "Generate required files for a Sphinx project.\n\n\
             sphinx-quickstart is an interactive tool that asks some questions \
             about your project and then generates a complete documentation \
             directory and sample Makefile to be used with sphinx-build.",
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("quiet mode"),
        )
        .arg(
            Arg::new("path")
                .value_name("PROJECT_DIR")
                .default_value(".")
                .help("project root"),
        )
        // Structure options
        .arg(
            Arg::new("sep")
                .long("sep")
                .action(ArgAction::SetTrue)
                .help("if specified, separate source and build dirs"),
        )
        .arg(
            Arg::new("no_sep")
                .long("no-sep")
                .action(ArgAction::SetTrue)
                .conflicts_with("sep")
                .help("if specified, create build dir under source dir"),
        )
        .arg(
            Arg::new("dot")
                .long("dot")
                .value_name("DOT")
                .default_value("_")
                .help("replacement for dot in _templates etc."),
        )
        // Project info
        .arg(
            Arg::new("project")
                .short('p')
                .long("project")
                .value_name("PROJECT")
                .help("project name"),
        )
        .arg(
            Arg::new("author")
                .short('a')
                .long("author")
                .value_name("AUTHOR")
                .help("author names"),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .value_name("VERSION")
                .default_value("")
                .help("version of project"),
        )
        .arg(
            Arg::new("release")
                .short('r')
                .long("release")
                .value_name("RELEASE")
                .help("release of project"),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_name("LANGUAGE")
                .help("document language"),
        )
        .arg(
            Arg::new("suffix")
                .long("suffix")
                .value_name("SUFFIX")
                .default_value(".rst")
                .help("source file suffix"),
        )
        .arg(
            Arg::new("master")
                .long("master")
                .value_name("MASTER")
                .default_value("index")
                .help("master document name"),
        )
        .arg(
            Arg::new("epub")
                .long("epub")
                .action(ArgAction::SetTrue)
                .help("use epub"),
        )
        // Makefile / batchfile
        .arg(
            Arg::new("makefile")
                .long("makefile")
                .action(ArgAction::SetTrue)
                .default_value("true")
                .help("create makefile"),
        )
        .arg(
            Arg::new("no_makefile")
                .long("no-makefile")
                .action(ArgAction::SetTrue)
                .conflicts_with("makefile")
                .help("do not create makefile"),
        )
        .arg(
            Arg::new("batchfile")
                .long("batchfile")
                .action(ArgAction::SetTrue)
                .default_value("true")
                .help("create batchfile"),
        )
        .arg(
            Arg::new("no_batchfile")
                .long("no-batchfile")
                .action(ArgAction::SetTrue)
                .conflicts_with("batchfile")
                .help("do not create batchfile"),
        )
        // Extensions (bulk)
        .arg(
            Arg::new("extensions")
                .long("extensions")
                .value_name("EXTENSIONS")
                .action(ArgAction::Append)
                .help("enable arbitrary extensions"),
        )
        // Fallback escape-hatch
        .arg(
            Arg::new("use_python_impl")
                .long("use-python-impl")
                .action(ArgAction::SetTrue)
                .help("delegate to the upstream Python sphinx-quickstart"),
        );

    // --ext-<name> flags for each known extension
    for (name, desc) in EXTENSIONS {
        // clap requires &'static str; leak the formatted strings.
        let id: &'static str =
            Box::leak(format!("ext_{}", name.replace('-', "_")).into_boxed_str());
        let long: &'static str = Box::leak(format!("ext-{name}").into_boxed_str());
        let help: &'static str =
            Box::leak(format!("enable {name} extension: {desc}").into_boxed_str());
        cmd = cmd.arg(
            Arg::new(id)
                .long(long)
                .action(ArgAction::SetTrue)
                .help(help),
        );
    }

    cmd
}

/// Parse `argv` into a [`QuickstartSettings`] with all flags applied.
/// Fields that require interactive prompts are left at their defaults;
/// `ask_user` fills them in later.
pub fn parse_args(argv: &[String]) -> Result<QuickstartSettings, clap::Error> {
    let cmd = build_parser();
    let matches = cmd.try_get_matches_from(argv)?;

    let mut s = QuickstartSettings {
        path: PathBuf::from(matches.get_one::<String>("path").unwrap()),
        quiet: matches.get_flag("quiet"),
        dot: matches.get_one::<String>("dot").unwrap().clone(),
        suffix: matches.get_one::<String>("suffix").unwrap().clone(),
        master: matches.get_one::<String>("master").unwrap().clone(),
        ..QuickstartSettings::default()
    };

    // sep / no-sep
    if matches.get_flag("sep") {
        s.sep = true;
    } else if matches.get_flag("no_sep") {
        s.sep = false;
    }

    // project / author / version / release / language
    if let Some(v) = matches.get_one::<String>("project") {
        s.project = v.clone();
    }
    if let Some(v) = matches.get_one::<String>("author") {
        s.author = v.clone();
    }
    if let Some(v) = matches.get_one::<String>("version") {
        s.version = v.clone();
    }
    // release defaults to version when not provided
    s.release = matches
        .get_one::<String>("release")
        .cloned()
        .unwrap_or_else(|| s.version.clone());
    if let Some(v) = matches.get_one::<String>("language") {
        let lang = v.clone();
        s.language = if lang == "en" { None } else { Some(lang) };
    }

    // makefile / batchfile
    if matches.get_flag("no_makefile") {
        s.makefile = false;
    }
    if matches.get_flag("no_batchfile") {
        s.batchfile = false;
    }

    // --ext-<name> flags
    let mut exts: Vec<String> = Vec::new();
    for (name, _) in EXTENSIONS {
        let key = format!("ext_{}", name.replace('-', "_"));
        if matches.get_flag(&key) {
            exts.push(format!("sphinx.ext.{name}"));
        }
    }
    // --extensions bulk
    if let Some(bulk) = matches.get_many::<String>("extensions") {
        for e in bulk {
            exts.push(e.clone());
        }
    }
    if !exts.is_empty() {
        s.extensions = exts;
    }

    Ok(s)
}

/// Return `true` if all required fields (project, author) are already
/// filled in — meaning interactive prompts can be skipped (`-q` mode).
pub fn is_fully_specified(s: &QuickstartSettings) -> bool {
    !s.project.is_empty() && !s.author.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(args: &[&str]) -> Vec<String> {
        std::iter::once("sphinx-quickstart")
            .chain(args.iter().copied())
            .map(String::from)
            .collect()
    }

    #[test]
    fn defaults() {
        let s = parse_args(&argv(&["-q", "-p", "P", "-a", "A"])).unwrap();
        assert_eq!(s.suffix, ".rst");
        assert_eq!(s.master, "index");
        assert_eq!(s.dot, "_");
        assert!(!s.sep);
        assert!(s.makefile);
        assert!(s.batchfile);
    }

    #[test]
    fn sep_flag() {
        let s = parse_args(&argv(&["--sep", "-p", "P", "-a", "A"])).unwrap();
        assert!(s.sep);
    }

    #[test]
    fn no_sep_flag() {
        let s = parse_args(&argv(&["--no-sep", "-p", "P", "-a", "A"])).unwrap();
        assert!(!s.sep);
    }

    #[test]
    fn no_makefile_no_batchfile() {
        let s = parse_args(&argv(&[
            "-q",
            "--no-makefile",
            "--no-batchfile",
            "-p",
            "P",
            "-a",
            "A",
        ]))
        .unwrap();
        assert!(!s.makefile);
        assert!(!s.batchfile);
    }

    #[test]
    fn ext_autodoc() {
        let s = parse_args(&argv(&["-q", "-p", "P", "-a", "A", "--ext-autodoc"])).unwrap();
        assert!(s.extensions.contains(&"sphinx.ext.autodoc".to_owned()));
    }

    #[test]
    fn release_defaults_to_version() {
        let s = parse_args(&argv(&["-q", "-p", "P", "-a", "A", "-v", "1.2"])).unwrap();
        assert_eq!(s.release, "1.2");
    }

    #[test]
    fn explicit_release() {
        let s = parse_args(&argv(&[
            "-q", "-p", "P", "-a", "A", "-v", "1.2", "-r", "1.2.3",
        ]))
        .unwrap();
        assert_eq!(s.release, "1.2.3");
    }

    #[test]
    fn is_fully_specified_ok() {
        let s = parse_args(&argv(&["-p", "P", "-a", "A"])).unwrap();
        assert!(is_fully_specified(&s));
    }

    #[test]
    fn is_fully_specified_missing_author() {
        let s = parse_args(&argv(&["-p", "P"])).unwrap();
        assert!(!is_fully_specified(&s));
    }
}
