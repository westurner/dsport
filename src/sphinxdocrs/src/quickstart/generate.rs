//! Core quickstart logic: `valid_dir`, `ask_user`, `generate`.
//!
//! All I/O goes through the injected [`Fs`], [`Clock`], and [`Terminal`]
//! traits so every branch is unit-testable without touching disk or a
//! real TTY.
//!
//! Parity targets from `sphinx.cmd.quickstart`:
//! - `valid_dir(d)` — reserved-name collision check.
//! - `ask_user(d)` — drives interactive prompts in the same order as upstream.
//! - `generate(d, ...)` — creates the directory tree + renders all files.

use std::path::{Path, PathBuf};

use serde::Serialize;
use unicode_width::UnicodeWidthStr;

use crate::cli::io::{Clock, Fs, Terminal};
use crate::quickstart::settings::{EXTENSIONS, QuickstartSettings};
use crate::quickstart::templates::QuickstartTemplates;
use crate::quickstart::validate;

// ── Error ────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum GenerateError {
    Io(std::io::Error),
    Template(jinja2rs::errors::Jinja2Error),
    /// User asked to exit during `ask_user` (pressed Enter on an empty new path).
    UserAbort,
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::Io(e) => write!(f, "I/O error: {e}"),
            GenerateError::Template(e) => write!(f, "Template error: {e}"),
            GenerateError::UserAbort => write!(f, "Aborted by user."),
        }
    }
}
impl std::error::Error for GenerateError {}

impl From<std::io::Error> for GenerateError {
    fn from(e: std::io::Error) -> Self {
        GenerateError::Io(e)
    }
}
impl From<jinja2rs::errors::Jinja2Error> for GenerateError {
    fn from(e: jinja2rs::errors::Jinja2Error) -> Self {
        GenerateError::Template(e)
    }
}

// ── valid_dir ────────────────────────────────────────────────────────────────

/// Mirrors `valid_dir(d)` from upstream:
/// - `True` if the directory does not exist (safe to create).
/// - `False` if it already contains a `Makefile`, `make.bat`, or
///   sphinx-reserved files matching `dot`.
pub fn valid_dir(
    path: &Path,
    sep: bool,
    dot: &str,
    master: &str,
    suffix: &str,
    fs: &dyn Fs,
) -> bool {
    if !fs.exists(path) {
        return true;
    }
    if !fs.is_dir(path) {
        return false;
    }

    let names = fs.read_dir_names(path).unwrap_or_default();
    let name_set: std::collections::HashSet<_> = names.iter().map(|s| s.as_str()).collect();

    if name_set.contains("Makefile") || name_set.contains("make.bat") {
        return false;
    }

    let check_dir = if sep {
        let src = path.join("source");
        if !fs.exists(&src) {
            return true;
        }
        if !fs.is_dir(&src) {
            return false;
        }
        fs.read_dir_names(&src).unwrap_or_default()
    } else {
        names
    };

    let reserved: &[String] = &[
        "conf.py".to_owned(),
        format!("{dot}static"),
        format!("{dot}templates"),
        format!("{master}{suffix}"),
    ];
    let check_set: std::collections::HashSet<_> = check_dir.iter().map(|s| s.as_str()).collect();
    for r in reserved {
        if check_set.contains(r.as_str()) {
            return false;
        }
    }
    true
}

// ── ask_user ─────────────────────────────────────────────────────────────────

/// Prompt helper — print prompt, read response, apply validator; retry on error.
///
/// Mirrors `do_prompt(text, default, validator)`. Empty response → use
/// `default` when provided.
fn do_prompt<T, E, V>(
    term: &dyn Terminal,
    text: &str,
    default: Option<&str>,
    validator: V,
) -> Result<T, GenerateError>
where
    V: Fn(&str) -> Result<T, E>,
    E: std::fmt::Display,
{
    loop {
        let prompt = match default {
            Some(d) => format!("> {text} [{d}]: "),
            None => format!("> {text}: "),
        };
        let raw = term.prompt(&prompt)?;
        let input = if raw.is_empty() {
            default.unwrap_or("").to_owned()
        } else {
            raw
        };
        match validator(&input) {
            Ok(val) => return Ok(val),
            Err(e) => term.print(&format!("* {e}")),
        }
    }
}

/// Mirrors `ask_user(d)` from upstream: fills in any missing fields on
/// `settings` via interactive terminal prompts.
pub fn ask_user(settings: &mut QuickstartSettings, term: &dyn Terminal, fs: &dyn Fs) {
    term.print(
        "Welcome to the Sphinx quickstart utility.\n\nPlease enter values for the following settings \
         (just press Enter to accept a default value, if one is given in brackets).",
    );

    // path
    term.print("\nEnter the root path for documentation.");
    loop {
        // Use the already-parsed path (from the positional CLI arg, or ".") as
        // the prompt default so the user sees their choice reflected.
        let path_default = settings.path.to_string_lossy().into_owned();
        let new_path: String = do_prompt(
            term,
            "Root path for the documentation",
            Some(&path_default),
            validate::is_path_or_new,
        )
        .unwrap_or_else(|_| path_default.clone());
        settings.path = PathBuf::from(&new_path);

        // Conflict: existing conf.py
        let has_conf = fs.is_file(&settings.path.join("conf.py"))
            || fs.is_file(&settings.path.join("source").join("conf.py"));
        if !has_conf {
            break;
        }
        term.print("\nError: an existing conf.py has been found in the selected root path.");
        term.print("sphinx-quickstart will not overwrite existing Sphinx projects.\n");
        let alt: String = do_prompt(
            term,
            "Please enter a new root path (or just Enter to exit)",
            Some(""),
            validate::is_path_or_empty,
        )
        .unwrap_or_default();
        if alt.is_empty() {
            // user pressed Enter on empty → abort (caller handles exit)
            std::process::exit(1);
        }
        settings.path = PathBuf::from(alt);
    }

    // sep
    term.print(
        "\nYou have two options for placing the build directory for Sphinx output.\n\
         Either, you use a directory \"_build\" within the root path, or you separate\n\
         \"source\" and \"build\" directories within the root path.",
    );
    settings.sep = do_prompt(
        term,
        "Separate source and build directories (y/n)",
        Some("n"),
        validate::boolean,
    )
    .unwrap_or(false);

    // dot
    term.print(
        "\nInside the root directory, two more directories will be created; \"_templates\"\n\
         for custom HTML templates and \"_static\" for custom stylesheets and other static\n\
         files. You can enter another prefix (such as \".\") to replace the underscore.",
    );
    settings.dot = do_prompt(
        term,
        "Name prefix for templates and static dir",
        Some("_"),
        validate::ok,
    )
    .unwrap_or_else(|_| "_".into());

    // project
    term.print("\nThe project name will occur in several places in the built documentation.");
    settings.project =
        do_prompt(term, "Project name", None, validate::nonempty).unwrap_or_default();

    // author
    settings.author =
        do_prompt(term, "Author name(s)", None, validate::nonempty).unwrap_or_default();

    // version / release
    term.print(
        "\nSphinx has the notion of a \"version\" and a \"release\" for the\n\
         software. Each version can have multiple releases. For example, for\n\
         Python the version is something like 2.5 or 3.0, while the release is\n\
         something like 2.5.1 or 3.0a1. If you don't need this dual structure,\n\
         just set both to the same value.",
    );
    settings.version = do_prompt(term, "Project version", Some(""), |x| {
        validate::allow_empty(x)
    })
    .unwrap_or_default();
    let ver = settings.version.clone();
    settings.release = do_prompt(term, "Project release", Some(&ver), |x| {
        validate::allow_empty(x)
    })
    .unwrap_or_else(|_| ver.clone());

    // language
    term.print(
        "\nIf the documents are to be written in a language other than English,\n\
         you can select a language here by its language code. Sphinx will then\n\
         translate text that it generates into that language.\n\n\
         For a list of supported codes, see\n\
         https://www.sphinx-doc.org/en/master/usage/configuration.html#confval-language.",
    );
    let lang: String = do_prompt(term, "Project language", Some("en"), validate::ok)
        .unwrap_or_else(|_| "en".into());
    settings.language = if lang == "en" { None } else { Some(lang) };

    // suffix
    term.print(
        "\nThe file name suffix for source files. Commonly, this is either \".txt\"\n\
         or \".rst\". Only files with this suffix are considered documents.",
    );
    settings.suffix = do_prompt(term, "Source file suffix", Some(".rst"), |x| {
        validate::suffix(x)
    })
    .unwrap_or_else(|_| ".rst".into());

    // master
    term.print(
        "\nOne document is special in that it is considered the top node of the\n\
         \"contents tree\", that is, it is the root of the hierarchical structure\n\
         of the documents. Normally, this is \"index\", but if your \"index\"\n\
         document is a custom template, you can also set this to another filename.",
    );
    loop {
        let master: String = do_prompt(
            term,
            "Name of your master document (without suffix)",
            Some("index"),
            validate::nonempty,
        )
        .unwrap_or_else(|_| "index".into());
        settings.master = master.clone();

        let master_file = master.clone() + &settings.suffix;
        let path = &settings.path;
        let has_master = fs.is_file(&path.join(&master_file))
            || fs.is_file(&path.join("source").join(&master_file));
        if !has_master {
            break;
        }
        term.print(&format!(
            "\nError: the master file {} has already been found in the selected root path.",
            master_file
        ));
        term.print("sphinx-quickstart will not overwrite the existing file.");
    }

    // extensions
    term.print("Indicate which of the following Sphinx extensions should be enabled:");
    let mut selected: Vec<String> = Vec::new();
    for (name, description) in EXTENSIONS {
        let enabled: bool = do_prompt(
            term,
            &format!("{name}: {description} (y/n)"),
            Some("n"),
            validate::boolean,
        )
        .unwrap_or(false);
        if enabled {
            selected.push(format!("sphinx.ext.{name}"));
        }
    }
    // conflict: imgmath + mathjax
    if selected.contains(&"sphinx.ext.imgmath".to_owned())
        && selected.contains(&"sphinx.ext.mathjax".to_owned())
    {
        term.print(
            "Note: imgmath and mathjax cannot be enabled at the same time. imgmath has been deselected.",
        );
        selected.retain(|e| e != "sphinx.ext.imgmath");
    }
    settings.extensions = selected;

    // makefile
    term.print(
        "\nA Makefile and a Windows command file can be generated for you so that you\n\
         only have to run e.g. `make html' instead of invoking sphinx-build\n\
         directly.",
    );
    settings.makefile = do_prompt(term, "Create Makefile? (y/n)", Some("y"), |x| {
        validate::boolean(x)
    })
    .unwrap_or(true);

    settings.batchfile = do_prompt(term, "Create Windows command file? (y/n)", Some("y"), |x| {
        validate::boolean(x)
    })
    .unwrap_or(true);
}

// ── generate ─────────────────────────────────────────────────────────────────

/// Template context — must be `Serialize` for minijinja.
#[derive(Serialize)]
struct Context {
    project: String,
    copyright: String,
    author: String,
    version: String,
    release: String,
    language: Option<String>,
    suffix: String,
    root_doc: String,
    dot: String,
    sep: bool,
    extensions: Vec<String>,
    exclude_patterns: String,
    mastertoctree: String,
    mastertocmaxdepth: u32,
    now: String,
    project_underline: String,
    makefile: bool,
    batchfile: bool,
    rsrcdir: String,
    rbuilddir: String,
    // Optional path-setup block (apidoc --full)
    append_syspath: bool,
    module_path: Option<String>,
}

/// Mirrors `generate(d, overwrite, silent, templatedir)`.
///
/// Creates the full quickstart tree in `settings.path`.
pub fn generate(
    settings: &QuickstartSettings,
    templates: &QuickstartTemplates,
    fs: &dyn Fs,
    clock: &dyn Clock,
) -> Result<(), GenerateError> {
    let path = settings
        .path
        .canonicalize()
        .unwrap_or_else(|_| settings.path.clone());
    fs.ensure_dir(&path)?;

    let srcdir = if settings.sep {
        path.join("source")
    } else {
        path.clone()
    };
    fs.ensure_dir(&srcdir)?;

    let builddir = if settings.sep {
        path.join("build")
    } else {
        srcdir.join(format!("{}build", settings.dot))
    };
    fs.ensure_dir(&builddir)?;

    fs.ensure_dir(&srcdir.join(format!("{}templates", settings.dot)))?;
    fs.ensure_dir(&srcdir.join(format!("{}static", settings.dot)))?;

    // Build exclude_patterns
    let exclude_patterns = if settings.sep {
        String::new()
    } else {
        let parts: Vec<String> = [
            format!("'{}build'", settings.dot),
            "'Thumbs.db'".to_owned(),
            "'.DS_Store'".to_owned(),
        ]
        .into();
        parts.join(", ")
    };

    let rsrcdir = if settings.sep {
        "source".to_owned()
    } else {
        ".".to_owned()
    };
    let rbuilddir = if settings.sep {
        "build".to_owned()
    } else {
        format!("{}build", settings.dot)
    };

    // project_underline: use display width of the project title
    let title = format!("{} documentation", settings.project);
    let width = UnicodeWidthStr::width(title.as_str());
    let project_underline = "=".repeat(width);

    let year = clock.year();
    let copyright = format!("{year}, {}", settings.author);

    let ctx = Context {
        project: settings.project.clone(),
        copyright,
        author: settings.author.clone(),
        version: settings.version.clone(),
        release: settings.release.clone(),
        language: settings.language.clone(),
        suffix: settings.suffix.clone(),
        root_doc: settings.master.clone(),
        dot: settings.dot.clone(),
        sep: settings.sep,
        extensions: settings.extensions.clone(),
        exclude_patterns,
        mastertoctree: String::new(),
        mastertocmaxdepth: 2,
        now: clock.asctime(),
        project_underline,
        makefile: settings.makefile,
        batchfile: settings.batchfile,
        rsrcdir,
        rbuilddir,
        append_syspath: false,
        module_path: None,
    };

    // conf.py
    let conf_src = include_str!("../../assets/quickstart/conf.py.jinja");
    let conf_content = templates.render_str(conf_src, &ctx)?;
    write_file(
        &srcdir.join("conf.py"),
        conf_content.as_bytes(),
        &[], // default newline
        settings.quiet,
        fs,
    )?;

    // root_doc (index.rst)
    let master_file = srcdir.join(format!("{}{}", settings.master, settings.suffix));
    let root_content = templates.render("root_doc.rst.jinja", &ctx)?;
    write_file(
        &master_file,
        root_content.as_bytes(),
        &[],
        settings.quiet,
        fs,
    )?;

    // Makefile — written with LF (\n) — pass the bytes directly
    if settings.makefile {
        let makefile_content = templates.render("Makefile.new.jinja", &ctx)?;
        // Normalise to LF
        let lf_bytes = to_lf(makefile_content.as_bytes());
        write_file(&path.join("Makefile"), &lf_bytes, &[], settings.quiet, fs)?;
    }

    // make.bat — written with CRLF (\r\n)
    if settings.batchfile {
        let bat_content = templates.render("make.bat.new.jinja", &ctx)?;
        let crlf_bytes = to_crlf(bat_content.as_bytes());
        write_file(&path.join("make.bat"), &crlf_bytes, &[], settings.quiet, fs)?;
    }

    if !settings.quiet {
        print_finish_message(settings, &srcdir, &builddir);
    }

    Ok(())
}

/// Write a file, printing status unless `quiet`.
fn write_file(
    p: &Path,
    bytes: &[u8],
    _extra_newline: &[u8],
    quiet: bool,
    fs: &dyn Fs,
) -> Result<(), GenerateError> {
    if !quiet {
        println!("Creating file {}.", p.display());
    }
    fs.write(p, bytes)?;
    Ok(())
}

/// Normalise line endings to LF (for Makefile).
fn to_lf(src: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len());
    let mut i = 0;
    while i < src.len() {
        if src[i] == b'\r' && i + 1 < src.len() && src[i + 1] == b'\n' {
            out.push(b'\n');
            i += 2;
            continue;
        }
        out.push(src[i]);
        i += 1;
    }
    out
}

/// Normalise line endings to CRLF (for make.bat).
fn to_crlf(src: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len() * 2);
    let mut i = 0;
    while i < src.len() {
        if src[i] == b'\n' && (i == 0 || src[i - 1] != b'\r') {
            out.push(b'\r');
        }
        out.push(src[i]);
        i += 1;
    }
    out
}

fn print_finish_message(settings: &QuickstartSettings, srcdir: &Path, builddir: &Path) {
    println!("\nFinished: An initial directory structure has been created.\n");
    println!(
        "You should now populate your master file {}/{}{} and create other documentation\nsource files.",
        srcdir.display(),
        settings.master,
        settings.suffix
    );
    if settings.makefile || settings.batchfile {
        println!("Use the Makefile to build the docs, like so:\n   make builder");
    } else {
        println!(
            "Use the sphinx-build command to build the docs, like so:\n   sphinx-build -b builder {} {}",
            srcdir.display(),
            builddir.display()
        );
    }
    println!(
        "where \"builder\" is one of the supported builders, e.g. html, latex or linkcheck.\n"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_lf_strips_crlf() {
        let input = b"line1\r\nline2\r\nline3\n";
        let out = to_lf(input);
        assert_eq!(out, b"line1\nline2\nline3\n");
    }

    #[test]
    fn to_crlf_adds_cr() {
        let input = b"line1\nline2\n";
        let out = to_crlf(input);
        assert_eq!(out, b"line1\r\nline2\r\n");
    }

    #[test]
    fn to_crlf_idempotent_on_crlf() {
        let input = b"line1\r\nline2\r\n";
        let out = to_crlf(input);
        assert_eq!(out, b"line1\r\nline2\r\n");
    }

    #[test]
    fn project_underline_ascii() {
        // "My Project documentation" = 24 chars = 24 wide
        let title = "My Project documentation";
        let w = UnicodeWidthStr::width(title);
        let ul = "=".repeat(w);
        assert_eq!(ul.len(), title.len());
    }

    #[test]
    fn project_underline_wide_chars() {
        // CJK full-width char counts as width 2
        let title = "日本語プロジェクト documentation";
        let w = UnicodeWidthStr::width(title);
        assert!(w > title.chars().count()); // width > len for CJK
    }
}
