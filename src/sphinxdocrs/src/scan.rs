//! `--scan-requirements` — detect Python packages needed to build a Sphinx
//! docs directory.
//!
//! The scan examines `conf.py` in the given source directory and collects
//! candidate package names from two sources:
//!
//! 1. The `extensions = [...]` list.
//! 2. Top-level `import X` / `from X import Y` statements, excluding stdlib
//!    and `sphinx` / `docutils`.
//!
//! For each candidate the module is probed with `python3 -c "import <name>"`.
//!
//! In addition, common dependency files (`requirements.txt`, `pyproject.toml`,
//! `setup.py`, …) are located by walking from the docs root up toward the
//! supplied project root.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

// ── stdlib module set ─────────────────────────────────────────────────────────

/// Candidate requirements file names (relative to a search root).
pub const REQUIREMENTS_CANDIDATES: &[&str] = &[
    "requirements.txt",
    "requirements-docs.txt",
    "requirements_docs.txt",
    "docs/requirements.txt",
    "doc/requirements.txt",
    "pyproject.toml",
    "setup.py",
    "setup.cfg",
];

/// Return the set of stdlib top-level module names for the running interpreter.
///
/// Populated once per process via `sys.stdlib_module_names` (Python ≥ 3.10),
/// supplemented by a hardcoded fallback list for older runtimes.
pub fn stdlib_modules() -> &'static HashSet<String> {
    static STDLIB: OnceLock<HashSet<String>> = OnceLock::new();
    STDLIB.get_or_init(|| {
        let live = Command::new("python3")
            .args([
                "-c",
                "import sys; \
                 names = getattr(sys, 'stdlib_module_names', None); \
                 print('\\n'.join(sorted(names)) if names else '')",
            ])
            .output()
            .ok();
        let mut set = HashSet::new();
        if let Some(o) = live {
            for line in String::from_utf8_lossy(&o.stdout).lines() {
                let l = line.trim();
                if !l.is_empty() {
                    set.insert(l.to_string());
                }
            }
        }
        // Fallback / supplement (Python < 3.10 or interpreter not found).
        for m in &[
            "_thread", "__future__", "abc", "array", "ast", "asyncio", "atexit",
            "base64", "bisect", "builtins", "bz2", "codecs", "collections",
            "contextlib", "copy", "copyreg", "csv", "ctypes", "dataclasses",
            "datetime", "decimal", "difflib", "dis", "email", "enum", "errno",
            "fileinput", "fnmatch", "fractions", "ftplib", "functools", "gc",
            "glob", "gzip", "hashlib", "heapq", "hmac", "html", "http",
            "importlib", "inspect", "io", "ipaddress", "itertools", "json",
            "keyword", "linecache", "locale", "logging", "lzma", "math",
            "mimetypes", "mmap", "multiprocessing", "numbers", "operator", "os",
            "pathlib", "pickle", "pkgutil", "platform", "pprint", "profile",
            "queue", "random", "re", "shutil", "signal", "socket", "sqlite3",
            "ssl", "stat", "statistics", "string", "struct", "subprocess",
            "sys", "tarfile", "tempfile", "textwrap", "threading", "time",
            "timeit", "token", "tokenize", "traceback", "types", "typing",
            "unittest", "urllib", "uuid", "warnings", "weakref", "xml",
            "xmlrpc", "zipfile", "zipimport", "zlib",
        ] {
            set.insert(m.to_string());
        }
        set
    })
}

// ── conf.py parsers ───────────────────────────────────────────────────────────

/// Parse the `extensions = [...]` list from `conf_py`.
///
/// Uses a line-oriented scan that handles multi-line lists.  Returns each
/// quoted string found inside the list.
pub fn parse_conf_extensions(conf_py: &Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(conf_py) else {
        return Vec::new();
    };
    let mut in_extensions = false;
    let mut exts = Vec::new();
    for line in text.lines() {
        let stripped = line.trim();
        if stripped.starts_with("extensions") && stripped.contains('=') {
            in_extensions = true;
        }
        if in_extensions {
            let mut rest = stripped;
            loop {
                // Pick whichever quote character appears first.
                let dq = rest.find('"');
                let sq = rest.find('\'');
                let (start, quote) = match (dq, sq) {
                    (Some(d), Some(s)) => {
                        if d < s { (d, '"') } else { (s, '\'') }
                    }
                    (Some(d), None) => (d, '"'),
                    (None, Some(s)) => (s, '\''),
                    (None, None) => break,
                };
                rest = &rest[start + 1..];
                if let Some(end) = rest.find(quote) {
                    exts.push(rest[..end].to_string());
                    rest = &rest[end + 1..];
                } else {
                    break;
                }
            }
            if stripped.contains(']') {
                in_extensions = false;
            }
        }
    }
    exts
}

/// Parse `import X` and `from X import Y` statements in `conf_py` and return
/// top-level module names that are **not** stdlib, `sphinx`, or `docutils`.
pub fn parse_conf_third_party_imports(conf_py: &Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(conf_py) else {
        return Vec::new();
    };
    let stdlib = stdlib_modules();
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for line in text.lines() {
        let s = line.trim();
        if s.starts_with('#') {
            continue;
        }
        // Strip inline comment.
        let s = s.split(" #").next().unwrap_or(s).trim();

        let candidates: Vec<String> = if let Some(rest) = s.strip_prefix("import ") {
            // `import foo.bar, baz.qux` → top-level names
            rest.split(',')
                .filter_map(|part| {
                    let top = part.trim().split('.').next()?.trim();
                    if top.is_empty() { None } else { Some(top.to_string()) }
                })
                .collect()
        } else if let Some(rest) = s.strip_prefix("from ") {
            // `from foo.bar import ...` → top-level name of `foo`
            let top = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .split('.')
                .next()
                .unwrap_or("");
            // Relative imports start with `.`; skip them.
            if top.is_empty() || top.starts_with('.') {
                vec![]
            } else {
                vec![top.to_string()]
            }
        } else {
            vec![]
        };

        for module in candidates {
            if stdlib.contains(&module) {
                continue;
            }
            if matches!(module.as_str(), "sphinx" | "docutils")
                || module.starts_with("sphinx.")
                || module.starts_with("docutils.")
            {
                continue;
            }
            if seen.insert(module.clone()) {
                result.push(module);
            }
        }
    }
    result
}

// ── importability check ───────────────────────────────────────────────────────

/// Return `true` if `module` is importable by the current `python3`.
pub fn python_module_importable(module: &str) -> bool {
    Command::new("python3")
        .args(["-c", &format!("import {module}")])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// ── requirements-file discovery ───────────────────────────────────────────────

/// Return paths of dependency files found between `docs_root` and
/// `project_root` (inclusive).
///
/// Walks up the directory tree, checking each ancestor for files listed in
/// [`REQUIREMENTS_CANDIDATES`].
pub fn find_requirements_files(docs_root: &Path, project_root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut cur = docs_root;
    loop {
        for name in REQUIREMENTS_CANDIDATES {
            let candidate = cur.join(name);
            if candidate.is_file() {
                found.push(candidate);
            }
        }
        if cur == project_root {
            break;
        }
        match cur.parent() {
            Some(p) => cur = p,
            None => break,
        }
    }
    found
}

// ── scan result ───────────────────────────────────────────────────────────────

/// Importability status of a single candidate module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageStatus {
    /// Top-level module / extension name.
    pub name: String,
    /// `true` when `python3 -c "import <name>"` succeeds.
    pub importable: bool,
    /// Where this name was found in `conf.py`.
    pub source: PackageSource,
}

/// How a package name was discovered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageSource {
    /// Listed in the `extensions = [...]` value.
    Extension,
    /// Found in an `import` or `from … import` statement.
    Import,
}

impl std::fmt::Display for PackageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageSource::Extension => write!(f, "extension"),
            PackageSource::Import => write!(f, "import"),
        }
    }
}

/// Full result of scanning a docs directory.
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// The `conf.py` that was scanned.
    pub conf_py: PathBuf,
    /// Packages found (extensions + third-party imports), deduplicated.
    pub packages: Vec<PackageStatus>,
    /// Dependency files found near the docs root.
    pub requirements_files: Vec<PathBuf>,
}

impl ScanResult {
    /// Return only the packages that are **not** importable.
    pub fn missing(&self) -> Vec<&PackageStatus> {
        self.packages.iter().filter(|p| !p.importable).collect()
    }

    /// `true` when every detected package is importable.
    pub fn all_present(&self) -> bool {
        self.packages.iter().all(|p| p.importable)
    }

    /// Render a human-readable report.
    pub fn report(&self) -> String {
        let mut s = format!("Requirements scan: {}\n", self.conf_py.display());

        // Group by source for display.
        let mut by_source: HashMap<String, Vec<&PackageStatus>> = HashMap::new();
        for p in &self.packages {
            by_source.entry(p.source.to_string()).or_default().push(p);
        }
        for source in &["extension", "import"] {
            if let Some(pkgs) = by_source.get(*source) {
                s.push_str(&format!("  {}s:\n", source));
                for p in pkgs {
                    let status = if p.importable { "ok" } else { "MISSING" };
                    s.push_str(&format!("    {:40} [{}]\n", p.name, status));
                }
            }
        }
        if !self.requirements_files.is_empty() {
            s.push_str("  requirements/dependency files:\n");
            for f in &self.requirements_files {
                s.push_str(&format!("    {}\n", f.display()));
            }
        }
        if self.all_present() {
            s.push_str("  status: all packages present\n");
        } else {
            s.push_str("  status: MISSING packages detected\n");
        }
        s
    }
}

// ── public entry point ────────────────────────────────────────────────────────

/// Scan `docs_root` for Python packages required to load `conf.py`.
///
/// `project_root` is used as the upper bound when searching for requirements
/// files.
pub fn scan_requirements(docs_root: &Path, project_root: &Path) -> ScanResult {
    let conf_py = docs_root.join("conf.py");

    let mut seen: HashSet<String> = HashSet::new();
    let mut packages: Vec<PackageStatus> = Vec::new();

    for name in parse_conf_extensions(&conf_py) {
        if seen.insert(name.clone()) {
            let importable = python_module_importable(&name);
            packages.push(PackageStatus { name, importable, source: PackageSource::Extension });
        }
    }
    for name in parse_conf_third_party_imports(&conf_py) {
        if seen.insert(name.clone()) {
            let importable = python_module_importable(&name);
            packages.push(PackageStatus { name, importable, source: PackageSource::Import });
        }
    }

    let requirements_files = find_requirements_files(docs_root, project_root);
    ScanResult { conf_py, packages, requirements_files }
}
