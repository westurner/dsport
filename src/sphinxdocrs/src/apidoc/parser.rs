//! Argument parser for `sphinx-apidoc-rs`.
//!
//! Mirrors `sphinx.ext.apidoc._cli.get_parser()` flag-for-flag.

use std::collections::BTreeSet;
use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};

use crate::apidoc::settings::ApidocOptions;
use crate::quickstart::settings::EXTENSIONS as QS_EXTENSIONS;

/// Build the clap [`Command`] mirroring upstream `get_parser()`.
pub fn build_parser() -> Command {
    let mut cmd = Command::new("sphinx-apidoc")
        .about(
            "Look recursively in <MODULE_PATH> for Python modules and packages \
             and create one reST file with automodule directives per package \
             in the <OUTPUT_PATH>.\n\n\
             The <EXCLUDE_PATTERN>s can be file and/or directory patterns that \
             will be excluded from generation.\n\n\
             Note: By default this script will not overwrite already created files.",
        )
        .arg(
            Arg::new("module_path")
                .value_name("MODULE_PATH")
                .required(true)
                .help("path to module to document"),
        )
        .arg(
            Arg::new("exclude_pattern")
                .value_name("EXCLUDE_PATTERN")
                .num_args(0..)
                .help("fnmatch-style file and/or directory patterns to exclude from generation"),
        )
        .arg(
            Arg::new("dest_dir")
                .short('o')
                .long("output-dir")
                .value_name("OUTPUT_PATH")
                .required(true)
                .help("directory to place all output"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .action(ArgAction::SetTrue)
                .help("no output on stdout, just warnings on stderr"),
        )
        .arg(
            Arg::new("max_depth")
                .short('d')
                .long("maxdepth")
                .value_name("MAXDEPTH")
                .default_value("4")
                .help("maximum depth of submodules to show in the TOC (default: 4)"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("overwrite existing files"),
        )
        .arg(
            Arg::new("follow_links")
                .short('l')
                .long("follow-links")
                .action(ArgAction::SetTrue)
                .help("follow symbolic links"),
        )
        .arg(
            Arg::new("dry_run")
                .short('n')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("run the script without creating files"),
        )
        .arg(
            Arg::new("separate_modules")
                .short('e')
                .long("separate")
                .action(ArgAction::SetTrue)
                .help("put documentation for each module on its own page"),
        )
        .arg(
            Arg::new("include_private")
                .short('P')
                .long("private")
                .action(ArgAction::SetTrue)
                .help("include \"_private\" modules"),
        )
        .arg(
            Arg::new("toc_file")
                .long("tocfile")
                .value_name("TOCFILE")
                .default_value("modules")
                .help("filename of table of contents (default: modules)"),
        )
        .arg(
            Arg::new("no_toc")
                .short('T')
                .long("no-toc")
                .action(ArgAction::SetTrue)
                .conflicts_with("toc_file")
                .help("don't create a table of contents file"),
        )
        .arg(
            Arg::new("no_headings")
                .short('E')
                .long("no-headings")
                .action(ArgAction::SetTrue)
                .help("don't create headings for the module/package packages"),
        )
        .arg(
            Arg::new("module_first")
                .short('M')
                .long("module-first")
                .action(ArgAction::SetTrue)
                .help("put module documentation before submodule documentation"),
        )
        .arg(
            Arg::new("implicit_namespaces")
                .long("implicit-namespaces")
                .action(ArgAction::SetTrue)
                .help("interpret module paths according to PEP-0420 implicit namespaces specification"),
        )
        .arg(
            Arg::new("automodule_options")
                .long("automodule-options")
                .value_name("OPTIONS")
                .default_value("")
                .help("comma-separated list of options to pass to automodule directive"),
        )
        .arg(
            Arg::new("suffix")
                .short('s')
                .long("suffix")
                .value_name("SUFFIX")
                .default_value("rst")
                .help("file suffix (default: rst)"),
        )
        .arg(
            Arg::new("remove_old")
                .long("remove-old")
                .action(ArgAction::SetTrue)
                .conflicts_with("full")
                .help("remove existing files in the output directory that were not generated"),
        )
        .arg(
            Arg::new("full")
                .short('F')
                .long("full")
                .action(ArgAction::SetTrue)
                .conflicts_with("remove_old")
                .help("generate a full project with sphinx-quickstart"),
        )
        .arg(
            Arg::new("append_syspath")
                .short('a')
                .long("append-syspath")
                .action(ArgAction::SetTrue)
                .help("append module_path to sys.path, used when --full is given"),
        )
        .arg(
            Arg::new("header")
                .short('H')
                .long("doc-project")
                .value_name("HEADER")
                .help("project name (default: root module name)"),
        )
        .arg(
            Arg::new("author")
                .short('A')
                .long("doc-author")
                .value_name("AUTHOR")
                .help("project author(s), used when --full is given"),
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("doc-version")
                .value_name("VERSION")
                .help("project version, used when --full is given"),
        )
        .arg(
            Arg::new("release")
                .short('R')
                .long("doc-release")
                .value_name("RELEASE")
                .help("project release, used when --full is given"),
        )
        .arg(
            Arg::new("extensions")
                .long("extensions")
                .value_name("EXTENSIONS")
                .action(ArgAction::Append)
                .help("enable arbitrary extensions, used when --full is given"),
        )
        .arg(
            Arg::new("template_dir")
                .short('t')
                .long("templatedir")
                .value_name("TEMPLATEDIR")
                .help("template directory for template files"),
        )
        .arg(
            Arg::new("use_python_impl")
                .long("use-python-impl")
                .action(ArgAction::SetTrue)
                .help("delegate to the upstream Python sphinx-apidoc"),
        );

    // --ext-<name> flags (same set as quickstart, used with --full)
    for (name, _desc) in QS_EXTENSIONS {
        let id: &'static str =
            Box::leak(format!("ext_{}", name.replace('-', "_")).into_boxed_str());
        let long: &'static str = Box::leak(format!("ext-{name}").into_boxed_str());
        let help: &'static str = Box::leak(
            format!("enable {name} extension, used when --full is given").into_boxed_str(),
        );
        cmd = cmd.arg(
            Arg::new(id)
                .long(long)
                .action(ArgAction::SetTrue)
                .help(help),
        );
    }

    cmd
}

/// Parse `argv` into `ApidocOptions`.
pub fn parse_args(argv: &[String]) -> Result<ApidocOptions, clap::Error> {
    let mut full_argv = vec!["sphinx-apidoc".to_owned()];
    full_argv.extend_from_slice(argv);
    let cmd = build_parser();
    let m = cmd.try_get_matches_from(&full_argv)?;

    let module_path = PathBuf::from(m.get_one::<String>("module_path").unwrap());
    let dest_dir = PathBuf::from(m.get_one::<String>("dest_dir").unwrap());

    let header = m.get_one::<String>("header").cloned().unwrap_or_else(|| {
        module_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default()
    });

    let suffix = m
        .get_one::<String>("suffix")
        .unwrap()
        .trim_start_matches('.')
        .to_owned();

    let toc_file = if m.get_flag("no_toc") {
        String::new()
    } else {
        m.get_one::<String>("toc_file").unwrap().clone()
    };

    let automodule_options: BTreeSet<String> = {
        let raw = m.get_one::<String>("automodule_options").unwrap();
        if raw.is_empty() {
            // also check SPHINX_APIDOC_OPTIONS env var (matches upstream)
            std::env::var("SPHINX_APIDOC_OPTIONS")
                .map(|v| v.split(',').map(str::trim).map(String::from).collect())
                .unwrap_or_default()
        } else {
            raw.split(',').map(str::trim).map(String::from).collect()
        }
    };

    let mut extensions: Vec<String> = m
        .get_many::<String>("extensions")
        .unwrap_or_default()
        .cloned()
        .collect();
    for (name, _) in QS_EXTENSIONS {
        let key = format!("ext_{}", name.replace('-', "_"));
        if m.get_flag(&key) {
            extensions.push(format!("sphinx.ext.{name}"));
        }
    }

    Ok(ApidocOptions {
        module_path,
        dest_dir,
        exclude_pattern: m
            .get_many::<String>("exclude_pattern")
            .unwrap_or_default()
            .cloned()
            .collect(),
        max_depth: m
            .get_one::<String>("max_depth")
            .unwrap()
            .parse()
            .unwrap_or(4),
        follow_links: m.get_flag("follow_links"),
        separate_modules: m.get_flag("separate_modules"),
        include_private: m.get_flag("include_private"),
        toc_file,
        no_headings: m.get_flag("no_headings"),
        module_first: m.get_flag("module_first"),
        implicit_namespaces: m.get_flag("implicit_namespaces"),
        automodule_options,
        suffix,
        remove_old: m.get_flag("remove_old"),
        quiet: m.get_flag("quiet"),
        dry_run: m.get_flag("dry_run"),
        force: m.get_flag("force"),
        full: m.get_flag("full"),
        append_syspath: m.get_flag("append_syspath"),
        header,
        author: m.get_one::<String>("author").cloned(),
        version: m.get_one::<String>("version").cloned(),
        release: m.get_one::<String>("release").cloned(),
        extensions,
        template_dir: m.get_one::<String>("template_dir").map(PathBuf::from),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn required_args() {
        let opts = parse_args(&argv(&["-o", "/out", "/src/mypkg"])).unwrap();
        assert_eq!(opts.module_path, PathBuf::from("/src/mypkg"));
        assert_eq!(opts.dest_dir, PathBuf::from("/out"));
        assert_eq!(opts.header, "mypkg");
    }

    #[test]
    fn suffix_strips_dot() {
        let opts = parse_args(&argv(&["-o", "/out", "/src", "-s", ".rst"])).unwrap();
        assert_eq!(opts.suffix, "rst");
    }

    #[test]
    fn no_toc_clears_toc_file() {
        let opts = parse_args(&argv(&["-o", "/out", "/src", "-T"])).unwrap();
        assert!(opts.toc_file.is_empty());
    }

    #[test]
    fn separate_modules_flag() {
        let opts = parse_args(&argv(&["-o", "/out", "/src", "-e"])).unwrap();
        assert!(opts.separate_modules);
    }

    #[test]
    fn implicit_namespaces_flag() {
        let opts = parse_args(&argv(&["-o", "/out", "/src", "--implicit-namespaces"])).unwrap();
        assert!(opts.implicit_namespaces);
    }

    #[test]
    fn full_flag_sets_full() {
        let opts = parse_args(&argv(&[
            "-o", "/out", "/src", "-F", "-H", "MyProj", "-A", "Me",
        ]))
        .unwrap();
        assert!(opts.full);
        assert_eq!(opts.header, "MyProj");
        assert_eq!(opts.author.as_deref(), Some("Me"));
    }

    #[test]
    fn ext_autodoc_flag() {
        let opts = parse_args(&argv(&["-o", "/out", "/src", "-F", "--ext-autodoc"])).unwrap();
        assert!(opts.extensions.contains(&"sphinx.ext.autodoc".to_owned()));
    }
}
