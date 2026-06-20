//! Argument parser for `sphinx-autogen-rs`.
//!
//! Mirrors `sphinx.ext.autosummary.generate.get_parser()`.

use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};

/// Parsed arguments from `sphinx-autogen`.
#[derive(Debug, Clone)]
pub struct AutogenArgs {
    /// Source RST files to scan for `.. autosummary::` directives.
    pub source_files: Vec<PathBuf>,
    /// Output directory for generated stubs.
    pub output_dir: Option<PathBuf>,
    /// File suffix without leading dot (default `"rst"`).
    pub suffix: String,
    /// Custom template directory.
    pub templates: Option<PathBuf>,
    /// Document imported members.
    pub imported_members: bool,
    /// Document exactly the members in `__all__`.
    pub respect_module_all: bool,
    /// Remove old output files not generated in this run.
    pub remove_old: bool,
}

/// Build the clap [`Command`] mirroring upstream `get_parser()`.
pub fn build_parser() -> Command {
    Command::new("sphinx-autogen")
        .about(
            "Generate ReStructuredText using autosummary directives.\n\n\
             sphinx-autogen is a frontend to sphinx.ext.autosummary.generate. \
             It generates the reStructuredText files from the autosummary \
             directives contained in the given input files.",
        )
        .arg(
            Arg::new("source_file")
                .value_name("SOURCE_FILE")
                .num_args(1..)
                .required(true)
                .help("source files to generate rST files for"),
        )
        .arg(
            Arg::new("output_dir")
                .short('o')
                .long("output-dir")
                .value_name("OUTPUT_DIR")
                .help("directory to place all output in"),
        )
        .arg(
            Arg::new("suffix")
                .short('s')
                .long("suffix")
                .value_name("SUFFIX")
                .default_value("rst")
                .help("default suffix for files (default: rst)"),
        )
        .arg(
            Arg::new("templates")
                .short('t')
                .long("templates")
                .value_name("TEMPLATES")
                .help("custom template directory (default: None)"),
        )
        .arg(
            Arg::new("imported_members")
                .short('i')
                .long("imported-members")
                .action(ArgAction::SetTrue)
                .help("document imported members (default: false)"),
        )
        .arg(
            Arg::new("respect_module_all")
                .short('a')
                .long("respect-module-all")
                .action(ArgAction::SetTrue)
                .help("document exactly the members in module __all__ attribute. (default: false)"),
        )
        .arg(
            Arg::new("remove_old")
                .long("remove-old")
                .action(ArgAction::SetTrue)
                .help("Remove existing files in the output directory that were not generated"),
        )
        .arg(
            Arg::new("use_python_impl")
                .long("use-python-impl")
                .action(ArgAction::SetTrue)
                .help("delegate to the upstream Python sphinx-autogen"),
        )
}

/// Parse `argv` into [`AutogenArgs`].
pub fn parse_args(argv: &[String]) -> Result<AutogenArgs, clap::Error> {
    let mut full_argv = vec!["sphinx-autogen".to_owned()];
    full_argv.extend_from_slice(argv);
    let cmd = build_parser();
    let m = cmd.try_get_matches_from(&full_argv)?;

    Ok(AutogenArgs {
        source_files: m
            .get_many::<String>("source_file")
            .unwrap_or_default()
            .map(PathBuf::from)
            .collect(),
        output_dir: m.get_one::<String>("output_dir").map(PathBuf::from),
        suffix: m
            .get_one::<String>("suffix")
            .unwrap()
            .trim_start_matches('.')
            .to_owned(),
        templates: m.get_one::<String>("templates").map(PathBuf::from),
        imported_members: m.get_flag("imported_members"),
        respect_module_all: m.get_flag("respect_module_all"),
        remove_old: m.get_flag("remove_old"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn required_source_file() {
        let a = parse_args(&argv("source.rst")).unwrap();
        assert_eq!(a.source_files, [PathBuf::from("source.rst")]);
        assert_eq!(a.suffix, "rst");
        assert!(a.output_dir.is_none());
    }

    #[test]
    fn output_dir() {
        let a = parse_args(&argv("src.rst -o /out")).unwrap();
        assert_eq!(a.output_dir, Some(PathBuf::from("/out")));
    }

    #[test]
    fn suffix_strips_dot() {
        let a = parse_args(&argv("src.rst -s .txt")).unwrap();
        assert_eq!(a.suffix, "txt");
    }

    #[test]
    fn bool_flags() {
        let a = parse_args(&argv("src.rst -i -a --remove-old")).unwrap();
        assert!(a.imported_members);
        assert!(a.respect_module_all);
        assert!(a.remove_old);
    }

    #[test]
    fn multiple_sources() {
        let a = parse_args(&argv("a.rst b.rst c.rst")).unwrap();
        assert_eq!(a.source_files.len(), 3);
    }
}
