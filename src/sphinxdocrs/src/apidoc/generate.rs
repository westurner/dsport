//! Core apidoc logic: `recurse_tree`, `create_package_file`,
//! `create_module_file`, `create_modules_toc_file`.
//!
//! Mirrors `sphinx.ext.apidoc._generate`.
//!
//! Python-specific suffixes (`.pyx`, C-extension `.so`/`.pyd`) are listed
//! for completeness but we only need them for the `is_initpy` check —
//! the actual import machinery stays Python.

use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::apidoc::settings::{ApidocOptions, SourceMode};
use crate::apidoc::templates::ApidocTemplates;

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum ApidocError {
    Io(io::Error),
    Template(jinja2rs::errors::Jinja2Error),
    InvalidSource(String),
}

impl std::fmt::Display for ApidocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApidocError::Io(e) => write!(f, "I/O error: {e}"),
            ApidocError::Template(e) => write!(f, "Template error: {e}"),
            ApidocError::InvalidSource(message) => write!(f, "invalid source tree: {message}"),
        }
    }
}
impl std::error::Error for ApidocError {}
impl From<io::Error> for ApidocError {
    fn from(e: io::Error) -> Self {
        ApidocError::Io(e)
    }
}
impl From<jinja2rs::errors::Jinja2Error> for ApidocError {
    fn from(e: jinja2rs::errors::Jinja2Error) -> Self {
        ApidocError::Template(e)
    }
}

// ── Python suffix list ────────────────────────────────────────────────────────

/// Python source + extension suffixes (mirrors `_generate.PY_SUFFIXES`).
pub const PY_SUFFIXES: &[&str] = &[".py", ".pyx", ".so", ".pyd"];

// ── Pure helper fns ───────────────────────────────────────────────────────────

/// Return `true` if `filename` is an `__init__.<suffix>` file.
pub fn is_initpy(filename: &Path) -> bool {
    let name = filename
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_default();
    PY_SUFFIXES.iter().any(|s| name == format!("__init__{s}"))
}

/// Join module name parts with `.`, filtering empty components.
pub fn module_join(parts: &[&str]) -> String {
    parts
        .iter()
        .filter(|p| !p.is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join(".")
}

/// Return `true` if `dir_path` contains an `__init__.*` file.
pub fn is_package_dir(dir_path: &Path) -> bool {
    dir_path
        .read_dir()
        .ok()
        .map(|rd| rd.flatten().any(|e| is_initpy(&e.path())))
        .unwrap_or(false)
}

/// Return `true` if `path` matches any of the compiled exclude patterns.
pub fn is_excluded(path: &Path, excludes: &[regex::Regex]) -> bool {
    let s = path.to_string_lossy();
    excludes.iter().any(|re| re.is_match(&s))
}

/// Return `true` if we should skip this package directory.
pub fn is_skipped_package(dir: &Path, opts: &ApidocOptions, excludes: &[regex::Regex]) -> bool {
    if !dir.is_dir() {
        return false;
    }
    let files: Vec<PathBuf> = dir
        .read_dir()
        .ok()
        .map(|rd| rd.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    let regular_package = files.iter().any(|f| is_initpy(f));
    if !regular_package && !opts.implicit_namespaces {
        return true;
    }
    files
        .iter()
        .filter(|f| {
            f.extension()
                .and_then(|e| e.to_str())
                .map(|e| PY_SUFFIXES.iter().any(|s| s.trim_start_matches('.') == e))
                .unwrap_or(false)
        })
        .all(|f| is_excluded(f, excludes))
}

/// Return `true` if we should skip this module file.
pub fn is_skipped_module(path: &Path, opts: &ApidocOptions) -> bool {
    if !path.exists() {
        return true;
    }
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_default();
    name.starts_with('_') && !opts.include_private
}

/// Return `true` if at least one Python module exists under `root_path`.
pub fn has_child_module(root_path: &Path, excludes: &[regex::Regex], opts: &ApidocOptions) -> bool {
    for (root, _subs, files) in walk(root_path, excludes, opts) {
        if !files.is_empty() {
            return true;
        }
        drop(root);
    }
    false
}

// ── Walk ──────────────────────────────────────────────────────────────────────

/// One directory-walk step: `(root, subdirs, python_files)`.
pub type WalkStep = (PathBuf, Vec<String>, Vec<String>);

/// Walk the directory tree, yielding `(root, sorted_subdirs, sorted_py_files)`.
///
/// Mirrors `_generate.walk`.
pub fn walk(root_path: &Path, excludes: &[regex::Regex], opts: &ApidocOptions) -> Vec<WalkStep> {
    let mut results = Vec::new();
    walk_inner(root_path, root_path, excludes, opts, &mut results);
    results
}

fn walk_inner(
    _root_path: &Path,
    current: &Path,
    excludes: &[regex::Regex],
    opts: &ApidocOptions,
    out: &mut Vec<WalkStep>,
) {
    let rd = match current.read_dir() {
        Ok(rd) => rd,
        Err(_) => return,
    };
    let mut files: Vec<String> = Vec::new();
    let mut subs: Vec<String> = Vec::new();

    for entry in rd.flatten() {
        let p = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if p.is_dir() {
            // skip hidden/private dirs and excluded dirs
            let skip = name.starts_with('.') || (!opts.include_private && name.starts_with('_'));
            if !skip && !is_excluded(&p, excludes) {
                subs.push(name);
            }
        } else {
            let is_py = PY_SUFFIXES.iter().any(|s| name.ends_with(s));
            if is_py && !is_excluded(&p, excludes) {
                files.push(name);
            }
        }
    }
    files.sort();
    subs.sort();

    out.push((current.to_path_buf(), subs.clone(), files));

    for sub in &subs {
        walk_inner(_root_path, &current.join(sub), excludes, opts, out);
    }
}

// ── Template contexts ─────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ModuleContext {
    show_headings: bool,
    basename: String,
    qualname: String,
    automodule_options: Vec<String>,
}

#[derive(Serialize)]
struct PackageContext {
    pkgname: String,
    subpackages: Vec<String>,
    submodules: Vec<String>,
    is_namespace: bool,
    modulefirst: bool,
    separatemodules: bool,
    automodule_options: Vec<String>,
    show_headings: bool,
    maxdepth: u32,
}

#[derive(Serialize)]
struct TocContext {
    header: String,
    maxdepth: u32,
    docnames: Vec<String>,
}

// ── File writers ──────────────────────────────────────────────────────────────

/// Write `text` to `<dest_dir>/<name>.<suffix>` unless dry-run or already
/// exists without --force.
///
/// Returns the path of the file (written or would-be-written).
pub fn write_file(name: &str, text: &str, opts: &ApidocOptions) -> Result<PathBuf, ApidocError> {
    let fname = opts.dest_dir.join(format!("{name}.{}", opts.suffix));
    if opts.dry_run {
        if !opts.quiet {
            println!("Would create file {}.", fname.display());
        }
        return Ok(fname);
    }
    if !opts.force && fname.is_file() {
        if !opts.quiet {
            println!("File {} already exists, skipping.", fname.display());
        }
    } else {
        if !opts.quiet {
            println!("Creating file {}.", fname.display());
        }
        std::fs::write(&fname, text)?;
    }
    Ok(fname)
}

/// Create a `.rst` file for a single module.
pub fn create_module_file(
    package: Option<&str>,
    basename: &str,
    opts: &ApidocOptions,
    templates: &ApidocTemplates,
) -> Result<PathBuf, ApidocError> {
    let mut options = opts.effective_automodule_options();
    if opts.include_private {
        options.insert("private-members".to_owned());
    }
    let qualname = module_join(&[package.unwrap_or(""), basename]);
    let ctx = ModuleContext {
        show_headings: !opts.no_headings,
        basename: basename.to_owned(),
        qualname: qualname.clone(),
        automodule_options: options.into_iter().collect(),
    };
    let text = templates.render("module.rst.jinja", &ctx)?;
    write_file(&qualname, &text, opts)
}

/// Create a `.rst` file for a package (and optionally its submodules).
#[allow(clippy::too_many_arguments)]
pub fn create_package_file(
    root: &Path,
    master_package: Option<&str>,
    subroot: &str,
    py_files: &[String],
    opts: &ApidocOptions,
    subs: &[String],
    is_namespace: bool,
    excludes: &[regex::Regex],
    templates: &ApidocTemplates,
) -> Result<Vec<PathBuf>, ApidocError> {
    // sub-packages
    let subpackages: Vec<String> = subs
        .iter()
        .filter(|pkgname| !is_skipped_package(&root.join(pkgname), opts, excludes))
        .map(|pkgname| module_join(&[master_package.unwrap_or(""), subroot, pkgname]))
        .collect();

    // sub-modules
    let submodule_basenames: BTreeSet<String> = py_files
        .iter()
        .filter(|f| !is_skipped_module(&root.join(f), opts) && !is_initpy(Path::new(f)))
        .map(|f| f.split('.').next().unwrap_or(f).to_owned())
        .collect();
    let submodules: Vec<String> = submodule_basenames
        .iter()
        .map(|m| module_join(&[master_package.unwrap_or(""), subroot, m]))
        .collect();

    let mut options = opts.effective_automodule_options();
    if opts.include_private {
        options.insert("private-members".to_owned());
    }
    let pkgname = module_join(&[master_package.unwrap_or(""), subroot]);
    let ctx = PackageContext {
        pkgname: pkgname.clone(),
        subpackages,
        submodules: submodules.clone(),
        is_namespace,
        modulefirst: opts.module_first,
        separatemodules: opts.separate_modules,
        automodule_options: options.into_iter().collect(),
        show_headings: !opts.no_headings,
        maxdepth: opts.max_depth,
    };
    let text = templates.render("package.rst.jinja", &ctx)?;
    let mut written = vec![write_file(&pkgname, &text, opts)?];

    // With --separate, also write a file per submodule
    if opts.separate_modules {
        for submodule in &submodules {
            let _basename = submodule.rsplit('.').next().unwrap_or(submodule);
            written.push(create_module_file(None, submodule, opts, templates)?);
        }
    }

    Ok(written)
}

/// Create the top-level `modules.<suffix>` TOC file.
pub fn create_modules_toc_file(
    modules: &[String],
    opts: &ApidocOptions,
    name: &str,
    templates: &ApidocTemplates,
) -> Result<PathBuf, ApidocError> {
    let mut sorted = modules.to_vec();
    sorted.sort();
    // de-duplicate nested packages (keep only top-level)
    let mut prev = String::new();
    let docnames: Vec<String> = sorted
        .into_iter()
        .filter(|m| {
            if m.starts_with(&format!("{prev}.")) {
                false
            } else {
                prev = m.clone();
                true
            }
        })
        .collect();

    let ctx = TocContext {
        header: opts.header.clone(),
        maxdepth: opts.max_depth,
        docnames,
    };
    let text = templates.render("toc.rst.jinja", &ctx)?;
    write_file(name, &text, opts)
}

/// Remove output files that were not generated in this run.
pub fn remove_old_files(
    written_files: &[PathBuf],
    dest_dir: &Path,
    suffix: &str,
) -> Result<(), io::Error> {
    let keep: std::collections::HashSet<_> = written_files.iter().collect();
    let pattern = dest_dir.join(format!("*.{suffix}"));
    for entry in glob::glob(&pattern.to_string_lossy())
        .unwrap_or_else(|_| glob::glob("").unwrap())
        .flatten()
    {
        if !keep.contains(&entry) {
            if let Err(e) = std::fs::remove_file(&entry) {
                eprintln!("Warning: failed to remove {}: {e}", entry.display());
            }
        }
    }
    Ok(())
}

fn effective_source_mode(root_path: &Path, opts: &ApidocOptions) -> Result<SourceMode, ApidocError> {
    match opts.source_mode {
        SourceMode::Auto => {
            let has_rust = opts.cargo_manifest.is_some()
                || root_path.join("Cargo.toml").is_file()
                || !source_files(root_path, "rs", opts, &[]).is_empty();
            let has_lean = (root_path.is_file()
                && root_path.extension().is_some_and(|ext| ext == "lean"))
                || !source_files(root_path, "lean", opts, &[]).is_empty();
            match (has_rust, has_lean) {
                (true, true) => Err(ApidocError::InvalidSource(
                    "mixed Rust and Lean roots require an explicit --source-mode".to_owned(),
                )),
                (true, false) => Ok(SourceMode::Rust),
                (false, true) => Ok(SourceMode::Lean),
                (false, false) => Ok(SourceMode::Python),
            }
        }
        mode => Ok(mode),
    }
}

fn source_files(
    root_path: &Path,
    extension: &str,
    opts: &ApidocOptions,
    excludes: &[regex::Regex],
) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if root_path.is_file() {
        if root_path.extension().is_some_and(|ext| ext == extension)
            && !is_excluded(root_path, excludes)
        {
            files.push(root_path.to_path_buf());
        }
        return files;
    }

    fn visit(
        current: &Path,
        extension: &str,
        opts: &ApidocOptions,
        excludes: &[regex::Regex],
        files: &mut Vec<PathBuf>,
    ) {
        let Ok(entries) = current.read_dir() else { return };
        let mut entries = entries.flatten().collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            if is_excluded(&path, excludes) {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(_) => continue,
            };
            if file_type.is_symlink() && !opts.follow_links {
                continue;
            }
            if file_type.is_dir() {
                if name.starts_with('.') || (!opts.include_private && name.starts_with('_')) {
                    continue;
                }
                visit(&path, extension, opts, excludes, files);
            } else if file_type.is_file()
                && path.extension().is_some_and(|ext| ext == extension)
                && (opts.include_private || !name.starts_with('_'))
            {
                files.push(path);
            }
        }
    }

    visit(root_path, extension, opts, excludes, &mut files);
    files.sort();
    files
}

fn rust_root(root_path: &Path, opts: &ApidocOptions) -> PathBuf {
    let manifest = opts
        .cargo_manifest
        .clone()
        .or_else(|| root_path.is_file().then(|| root_path.to_path_buf()))
        .or_else(|| root_path.join("Cargo.toml").is_file().then(|| root_path.join("Cargo.toml")));
    manifest
        .as_deref()
        .and_then(Path::parent)
        .map(|root| root.join("src"))
        .filter(|root| root.is_dir())
        .unwrap_or_else(|| root_path.to_path_buf())
}

fn source_module_name(path: &Path, root: &Path, mode: SourceMode, crate_name: &str) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let mut parts = relative
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let filename = parts.pop().unwrap_or_else(|| "module".to_owned());
    let stem = filename.strip_suffix(".rs").or_else(|| filename.strip_suffix(".lean")).unwrap_or(&filename);
    if mode == SourceMode::Rust {
        if stem == "lib" || stem == "main" {
            parts.clear();
        } else if stem == "mod" {
            // `foo/mod.rs` names the `foo` module itself.
        } else {
            parts.push(stem.to_owned());
        }
        let mut name = parts.join("::");
        if name.is_empty() {
            name = crate_name.to_owned();
        } else {
            name = format!("{crate_name}::{name}");
        }
        name
    } else {
        if stem != "Main" {
            parts.push(stem.to_owned());
        }
        if parts.is_empty() {
            crate_name.to_owned()
        } else {
            parts.join(".")
        }
    }
}

fn create_source_file(
    path: &Path,
    root: &Path,
    mode: SourceMode,
    crate_name: &str,
    opts: &ApidocOptions,
) -> Result<(PathBuf, String), ApidocError> {
    let module = source_module_name(path, root, mode, crate_name);
    let directive = match mode {
        SourceMode::Rust => "rust:module",
        SourceMode::Lean => "lean:module",
        SourceMode::Python | SourceMode::Auto => {
            return Err(ApidocError::InvalidSource("Python has no source module mode".to_owned()))
        }
    };
    let mode_name = match mode {
        SourceMode::Rust => "rust",
        SourceMode::Lean => "lean",
        SourceMode::Python | SourceMode::Auto => unreachable!(),
    };
    let mut text = format!(
        ".. Generated by sphinx-apidoc-rs (source-mode: {mode_name}).\n\n.. {directive}:: {module}\n"
    );
    text.push_str(&format!("\n   :source: {}\n", path.display()));
    let docname = module.replace("::", ".");
    let written = write_file(&docname, &text, opts)?;
    Ok((written, docname))
}

/// Discover Rust or Lean source files and generate one native module
/// directive per source file. The returned names are suitable for a toctree.
pub fn recurse_source_tree(
    root_path: &Path,
    excludes: &[regex::Regex],
    opts: &ApidocOptions,
) -> Result<(Vec<PathBuf>, Vec<String>), ApidocError> {
    let mode = effective_source_mode(root_path, opts)?;
    let (root, extension, crate_name) = match mode {
        SourceMode::Rust => {
            let root = rust_root(root_path, opts);
            let crate_name = opts
                .cargo_package
                .clone()
                .or_else(|| root.parent().and_then(Path::file_name).map(|name| name.to_string_lossy().into_owned()))
                .unwrap_or_else(|| "crate".to_owned());
            (root, "rs", crate_name)
        }
        SourceMode::Lean => {
            let root = if root_path.is_file() {
                root_path.parent().unwrap_or(Path::new(".")).to_path_buf()
            } else {
                root_path.to_path_buf()
            };
            let crate_name = root
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Main".to_owned());
            (root, "lean", crate_name)
        }
        SourceMode::Python | SourceMode::Auto => {
            return Err(ApidocError::InvalidSource(
                "recurse_source_tree requires Rust or Lean mode".to_owned(),
            ))
        }
    };
    let files = source_files(&root, extension, opts, excludes);
    if files.is_empty() {
        return Err(ApidocError::InvalidSource(format!(
            "no .{extension} source files found under {}",
            root.display()
        )));
    }
    let mut written = Vec::new();
    let mut modules = Vec::new();
    for path in files {
        let (file, module) = create_source_file(&path, &root, mode, &crate_name, opts)?;
        written.push(file);
        modules.push(module);
    }
    modules.sort();
    modules.dedup();
    Ok((written, modules))
}

// ── Main entry point ──────────────────────────────────────────────────────────

/// Mirrors `_generate.recurse_tree`.
///
/// Returns `(written_files, top_level_module_names)`.
pub fn recurse_tree(
    root_path: &Path,
    excludes: &[regex::Regex],
    opts: &ApidocOptions,
    templates: &ApidocTemplates,
) -> Result<(Vec<PathBuf>, Vec<String>), ApidocError> {
    if effective_source_mode(root_path, opts)? != SourceMode::Python {
        return recurse_source_tree(root_path, excludes, opts);
    }
    recurse_python_tree(root_path, excludes, opts, templates)
}

fn recurse_python_tree(
    root_path: &Path,
    excludes: &[regex::Regex],
    opts: &ApidocOptions,
    templates: &ApidocTemplates,
) -> Result<(Vec<PathBuf>, Vec<String>), ApidocError> {
    let root_package = if is_package_dir(root_path) || opts.implicit_namespaces {
        Some(
            root_path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default(),
        )
    } else {
        None
    };

    let mut written_files: Vec<PathBuf> = Vec::new();
    let mut toplevels: Vec<String> = Vec::new();

    for (root, subs, mut files) in walk(root_path, excludes, opts) {
        let is_pkg = files.iter().any(|f| is_initpy(Path::new(f)));
        let is_namespace = !is_pkg && opts.implicit_namespaces;

        if is_pkg {
            // __init__ first
            files.sort_by(|a, b| {
                let a_init = is_initpy(Path::new(a));
                let b_init = is_initpy(Path::new(b));
                b_init.cmp(&a_init).then(a.cmp(b))
            });
        } else if root != root_path {
            // non-package below top-level only allowed with implicit namespaces
            if !opts.implicit_namespaces {
                continue;
            }
        }

        if is_pkg || is_namespace {
            if subs.len() + files.len() > 1 || !is_skipped_package(&root, opts, excludes) {
                let subroot = root
                    .strip_prefix(root_path)
                    .unwrap_or(Path::new(""))
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, ".");
                let subroot = subroot.trim_start_matches('.');

                if !is_namespace || has_child_module(&root, excludes, opts) {
                    let wf = create_package_file(
                        &root,
                        root_package.as_deref(),
                        subroot,
                        &files,
                        opts,
                        &subs,
                        is_namespace,
                        excludes,
                        templates,
                    )?;
                    written_files.extend(wf);
                    toplevels.push(module_join(&[
                        root_package.as_deref().unwrap_or(""),
                        subroot,
                    ]));
                }
            }
        } else {
            // top-level directory: document each loose Python file
            assert_eq!(root, root_path, "only root should reach this branch");
            assert!(root_package.is_none());
            for py_file in &files {
                if !is_skipped_module(&root.join(py_file), opts) {
                    let module = py_file.split('.').next().unwrap_or(py_file);
                    written_files.push(create_module_file(
                        root_package.as_deref(),
                        module,
                        opts,
                        templates,
                    )?);
                    toplevels.push(module.to_owned());
                }
            }
        }
    }

    Ok((written_files, toplevels))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn templates() -> ApidocTemplates {
        ApidocTemplates::vendored()
    }

    fn options(tmp: &TempDir) -> ApidocOptions {
        ApidocOptions::new(tmp.path().join("src"), tmp.path().join("out"))
    }

    #[test]
    fn is_initpy_py() {
        assert!(is_initpy(Path::new("__init__.py")));
        assert!(is_initpy(Path::new("__init__.pyx")));
        assert!(!is_initpy(Path::new("utils.py")));
        assert!(!is_initpy(Path::new("__init___.py")));
    }

    #[test]
    fn module_join_filters_empty() {
        assert_eq!(module_join(&["", "mypackage", "utils"]), "mypackage.utils");
        assert_eq!(module_join(&["mypackage", ""]), "mypackage");
        assert_eq!(module_join(&["a", "b", "c"]), "a.b.c");
    }

    #[test]
    fn is_excluded_matches_pattern() {
        let re = regex::Regex::new(".*test.*").unwrap();
        assert!(is_excluded(
            Path::new("/src/tests/foo.py"),
            std::slice::from_ref(&re)
        ));
        assert!(!is_excluded(Path::new("/src/mymod.py"), &[re]));
    }

    #[test]
    fn package_and_module_filters_cover_missing_private_and_namespace_cases() {
        let tmp = TempDir::new().unwrap();
        let pkg = tmp.path().join("pkg");
        std::fs::create_dir_all(&pkg).unwrap();
        std::fs::write(pkg.join("__init__.py"), "").unwrap();
        std::fs::write(pkg.join("public.py"), "").unwrap();
        std::fs::write(pkg.join("_private.py"), "").unwrap();
        let excludes = [regex::Regex::new(".*pkg.*").unwrap()];

        assert!(is_package_dir(&pkg));
        assert!(!is_package_dir(&tmp.path().join("missing")));
        assert!(!is_skipped_package(&pkg, &options(&tmp), &[]));
        let namespace = tmp.path().join("namespace");
        std::fs::create_dir(&namespace).unwrap();
        assert!(is_skipped_package(
            &namespace,
            &options(&tmp),
            &[]
        ));

        let mut namespace_options = options(&tmp);
        namespace_options.implicit_namespaces = true;
        std::fs::write(namespace.join("module.py"), "").unwrap();
        assert!(!is_skipped_package(&namespace, &namespace_options, &[]));
        assert!(is_skipped_package(&pkg, &options(&tmp), &excludes));

        assert!(is_skipped_module(&tmp.path().join("missing.py"), &options(&tmp)));
        assert!(is_skipped_module(&pkg.join("_private.py"), &options(&tmp)));
        let mut private_options = options(&tmp);
        private_options.include_private = true;
        assert!(!is_skipped_module(&pkg.join("_private.py"), &private_options));
        assert!(!is_skipped_module(&pkg.join("public.py"), &options(&tmp)));
    }

    #[test]
    fn walk_filters_hidden_private_excluded_and_non_python_entries() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("pkg");
        std::fs::create_dir_all(root.join("visible")).unwrap();
        std::fs::create_dir_all(root.join("_private")).unwrap();
        std::fs::create_dir_all(root.join(".hidden")).unwrap();
        for path in [
            root.join("z.py"),
            root.join("a.pyx"),
            root.join("visible/b.pyd"),
            root.join("visible/readme.txt"),
            root.join("_private/hidden.py"),
            root.join(".hidden/hidden.py"),
        ] {
            std::fs::write(path, "").unwrap();
        }
        let excludes = [regex::Regex::new("visible").unwrap()];
        let steps = walk(&root, &excludes, &options(&tmp));
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].1, Vec::<String>::new());
        assert_eq!(steps[0].2, vec!["a.pyx", "z.py"]);

        assert!(has_child_module(&root, &[], &options(&tmp)));
        assert!(!has_child_module(&tmp.path().join("missing"), &[], &options(&tmp)));
    }

    #[test]
    fn write_file_handles_dry_run_existing_force_and_quiet() {
        let tmp = TempDir::new().unwrap();
        let mut opts = options(&tmp);
        std::fs::create_dir_all(&opts.dest_dir).unwrap();

        opts.dry_run = true;
        let path = write_file("module", "new", &opts).unwrap();
        assert_eq!(path, opts.dest_dir.join("module.rst"));
        assert!(!path.exists());

        opts.dry_run = false;
        std::fs::write(&path, "old").unwrap();
        write_file("module", "new", &opts).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "old");

        opts.force = true;
        opts.quiet = true;
        write_file("module", "new", &opts).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "new");
    }

    #[test]
    fn generated_package_and_toc_files_cover_separate_modules_and_deduplication() {
        let tmp = TempDir::new().unwrap();
        let mut opts = options(&tmp);
        opts.force = true;
        opts.quiet = true;
        opts.separate_modules = true;
        opts.include_private = true;
        std::fs::create_dir_all(opts.dest_dir.clone()).unwrap();
        let root = tmp.path().join("src");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("__init__.py"), "").unwrap();
        std::fs::write(root.join("a.py"), "").unwrap();
        std::fs::write(root.join("_private.py"), "").unwrap();
        let files = vec!["__init__.py".to_string(), "a.py".to_string(), "_private.py".to_string()];
        let written = create_package_file(
            &root,
            Some("pkg"),
            "",
            &files,
            &opts,
            &[],
            false,
            &[],
            &templates(),
        )
        .unwrap();
        assert!(written.len() >= 3);

        let toc = create_modules_toc_file(
            &["pkg".into(), "pkg.sub".into(), "other".into(), "pkg".into()],
            &opts,
            "modules",
            &templates(),
        )
        .unwrap();
        let content = std::fs::read_to_string(toc).unwrap();
        assert!(content.contains("pkg"));
        assert!(content.contains("other"));
        assert_eq!(content.matches("pkg.sub").count(), 0);
    }

    #[test]
    fn remove_old_files_keeps_written_and_removes_stale() {
        let tmp = TempDir::new().unwrap();
        let keep = tmp.path().join("keep.rst");
        let stale = tmp.path().join("stale.rst");
        std::fs::write(&keep, "keep").unwrap();
        std::fs::write(&stale, "stale").unwrap();
        remove_old_files(std::slice::from_ref(&keep), tmp.path(), "rst").unwrap();
        assert!(keep.exists());
        assert!(!stale.exists());
    }

    #[test]
    fn source_modes_generate_native_module_directives() {
        let tmp = TempDir::new().unwrap();
        let rust_root = tmp.path().join("rust-crate/src");
        let lean_root = tmp.path().join("lean-project");
        let output = tmp.path().join("out");
        std::fs::create_dir_all(&rust_root).unwrap();
        std::fs::create_dir_all(&lean_root).unwrap();
        std::fs::create_dir_all(&output).unwrap();
        std::fs::write(rust_root.join("lib.rs"), "pub mod api;").unwrap();
        std::fs::write(rust_root.join("api.rs"), "pub fn answer() -> i32 { 42 }").unwrap();
        std::fs::write(lean_root.join("Demo.lean"), "def answer : Nat := 42").unwrap();

        let mut rust_options = ApidocOptions::new(rust_root.clone(), output.clone());
        rust_options.source_mode = SourceMode::Rust;
        rust_options.cargo_package = Some("demo".to_owned());
        rust_options.force = true;
        rust_options.quiet = true;
        let (rust_files, rust_modules) = recurse_tree(
            &rust_root,
            &[],
            &rust_options,
            &templates(),
        )
        .unwrap();
        assert_eq!(rust_modules, vec!["demo", "demo.api"]);
        assert_eq!(rust_files.len(), 2);
        assert!(std::fs::read_to_string(output.join("demo.api.rst"))
            .unwrap()
            .contains(".. rust:module:: demo::api"));

        let mut lean_options = ApidocOptions::new(lean_root.clone(), output.clone());
        lean_options.source_mode = SourceMode::Lean;
        lean_options.force = true;
        lean_options.quiet = true;
        let (_, lean_modules) = recurse_tree(&lean_root, &[], &lean_options, &templates()).unwrap();
        assert_eq!(lean_modules, vec!["Demo"]);
        assert!(std::fs::read_to_string(output.join("Demo.rst"))
            .unwrap()
            .contains(".. lean:module:: Demo"));
    }
}
