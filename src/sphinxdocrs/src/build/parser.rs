//! Argument parser for `sphinx-build-rs`.
//!
//! Mirrors `sphinx.cmd.build.get_parser()` flag-for-flag using [`clap`].

use clap::{Arg, ArgAction, Command};

/// Build the clap [`Command`] mirroring upstream `get_parser()`.
pub fn build_parser() -> Command {
    Command::new("sphinx-build")
        .about("Generate documentation from source files.")
        .arg(
            Arg::new("sourcedir")
                .value_name("SOURCE_DIR")
                .required(true)
                .help("path to documentation source files"),
        )
        .arg(
            Arg::new("outputdir")
                .value_name("OUTPUT_DIR")
                .required(true)
                .help("path to output directory"),
        )
        .arg(
            Arg::new("filenames")
                .value_name("FILENAMES")
                .num_args(0..)
                .help(
                    "(optional) a list of specific files to rebuild. \
                     Ignored if --write-all is specified",
                ),
        )
        // General
        .arg(
            Arg::new("builder")
                .short('b')
                .long("builder")
                .value_name("BUILDER")
                .default_value("html")
                .help("builder to use (default: 'html')"),
        )
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .value_name("N")
                .default_value("1")
                .help(
                    "run in parallel with N processes, when possible. \
                     'auto' uses the number of CPU cores",
                ),
        )
        .arg(
            Arg::new("force_all")
                .short('a')
                .long("write-all")
                .action(ArgAction::SetTrue)
                .help("write all files (default: only write new and changed files)"),
        )
        .arg(
            Arg::new("freshenv")
                .short('E')
                .long("fresh-env")
                .action(ArgAction::SetTrue)
                .help("don't use a saved environment, always read all files"),
        )
        // Path opts
        .arg(
            Arg::new("doctreedir")
                .short('d')
                .long("doctree-dir")
                .value_name("PATH")
                .help(
                    "directory for doctree and environment files \
                     (default: OUTPUT_DIR/.doctrees)",
                ),
        )
        .arg(
            Arg::new("confdir")
                .short('c')
                .long("conf-dir")
                .value_name("PATH")
                .help("directory for the configuration file (conf.py) (default: SOURCE_DIR)"),
        )
        // Build config
        .arg(
            Arg::new("noconfig")
                .short('C')
                .long("isolated")
                .action(ArgAction::SetTrue)
                .help("use no configuration file, only use settings from -D options"),
        )
        .arg(
            Arg::new("define")
                .short('D')
                .long("define")
                .value_name("setting=value")
                .action(ArgAction::Append)
                .help("override a setting in configuration file"),
        )
        .arg(
            Arg::new("htmldefine")
                .short('A')
                .long("html-define")
                .value_name("name=value")
                .action(ArgAction::Append)
                .help("pass a value into HTML templates"),
        )
        .arg(
            Arg::new("tags")
                .short('t')
                .long("tag")
                .value_name("TAG")
                .action(ArgAction::Append)
                .help("define tag: include \"only\" blocks with TAG"),
        )
        .arg(
            Arg::new("nitpicky")
                .short('n')
                .long("nitpicky")
                .action(ArgAction::SetTrue)
                .help("nitpicky mode: warn about all missing references"),
        )
        // Console output
        .arg(
            Arg::new("verbosity")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .help("increase verbosity (can be repeated)"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("no output on stdout, just warnings on stderr"),
        )
        .arg(
            Arg::new("really_quiet")
                .short('Q')
                .long("silent")
                .action(ArgAction::SetTrue)
                .help("no output at all, not even warnings"),
        )
        .arg(
            Arg::new("color")
                .long("color")
                .action(ArgAction::SetTrue)
                .conflicts_with("no_color")
                .help("do emit colored output (default: auto-detect)"),
        )
        .arg(
            Arg::new("no_color")
                .short('N')
                .long("no-color")
                .action(ArgAction::SetTrue)
                .help("do not emit colored output"),
        )
        // Warning control
        .arg(
            Arg::new("warnfile")
                .short('w')
                .long("warning-file")
                .value_name("FILE")
                .help("write warnings (and errors) to given file"),
        )
        .arg(
            Arg::new("warningiserror")
                .short('W')
                .long("fail-on-warning")
                .action(ArgAction::SetTrue)
                .help("turn warnings into errors"),
        )
        .arg(
            Arg::new("keep_going")
                .long("keep-going")
                .action(ArgAction::SetTrue)
                .hide(true),
        )
        .arg(
            Arg::new("traceback")
                .short('T')
                .long("show-traceback")
                .action(ArgAction::SetTrue)
                .help("show full traceback on exception"),
        )
        .arg(
            Arg::new("pdb")
                .short('P')
                .long("pdb")
                .action(ArgAction::SetTrue)
                .help("run Pdb on exception"),
        )
        .arg(
            Arg::new("exception_on_warning")
                .long("exception-on-warning")
                .action(ArgAction::SetTrue)
                .help("raise an exception on warnings"),
        )
        .arg(
            Arg::new("use_python_impl")
                .long("use-python-impl")
                .action(ArgAction::SetTrue)
                .help("delegate to the upstream Python sphinx-build"),
        )
        .arg(
            Arg::new("scan_requirements")
                .long("scan-requirements")
                .action(ArgAction::SetTrue)
                .help(
                    "scan conf.py and more for required Python packages",
                ),
        )
}
