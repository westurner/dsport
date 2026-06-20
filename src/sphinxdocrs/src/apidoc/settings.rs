//! `ApidocOptions` — structured representation of all `sphinx-apidoc` settings.
//!
//! Mirrors `sphinx.ext.apidoc._shared.ApidocOptions`.

use std::collections::BTreeSet;
use std::path::PathBuf;

/// Default automodule options (mirrors `_generate.OPTIONS` default).
pub const DEFAULT_AUTOMODULE_OPTIONS: &[&str] = &["members", "undoc-members", "show-inheritance"];

/// All settings for a `sphinx-apidoc` run.
///
/// Every field mirrors its namesake in upstream's `ApidocOptions` dataclass.
#[derive(Debug, Clone)]
pub struct ApidocOptions {
    /// Path to the module/package to document.
    pub module_path: PathBuf,
    /// Output directory for generated `.rst` files.
    pub dest_dir: PathBuf,

    /// Glob/fnmatch exclude patterns (resolved before use).
    pub exclude_pattern: Vec<String>,
    /// Maximum toctree depth (default 4).
    pub max_depth: u32,
    /// Follow symbolic links.
    pub follow_links: bool,
    /// Put each module on its own page.
    pub separate_modules: bool,
    /// Include `_private` modules.
    pub include_private: bool,
    /// Name of the TOC file (default `"modules"`); empty means no TOC.
    pub toc_file: String,
    /// Suppress headings in generated files.
    pub no_headings: bool,
    /// Put module documentation before submodule documentation.
    pub module_first: bool,
    /// Interpret paths as PEP 420 implicit namespaces.
    pub implicit_namespaces: bool,
    /// Options forwarded to `.. automodule::` directives.
    pub automodule_options: BTreeSet<String>,
    /// Source file suffix without leading dot (default `"rst"`).
    pub suffix: String,

    /// Remove old output files not in this run.
    pub remove_old: bool,
    /// Suppress stdout messages.
    pub quiet: bool,
    /// Dry run — don't write anything.
    pub dry_run: bool,
    /// Overwrite existing files.
    pub force: bool,

    // --full mode
    /// Generate a full Sphinx project (calls quickstart).
    pub full: bool,
    /// Append module_path to sys.path in generated conf.py.
    pub append_syspath: bool,
    /// Project name header (default: root module dir name).
    pub header: String,
    /// Project author (used with --full).
    pub author: Option<String>,
    /// Project version (used with --full).
    pub version: Option<String>,
    /// Project release (used with --full, defaults to version).
    pub release: Option<String>,
    /// Extra extensions (used with --full).
    pub extensions: Vec<String>,
    /// Custom template directory.
    pub template_dir: Option<PathBuf>,
}

impl ApidocOptions {
    /// Construct with sensible defaults matching upstream.
    pub fn new(module_path: PathBuf, dest_dir: PathBuf) -> Self {
        let header = module_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        Self {
            module_path,
            dest_dir,
            exclude_pattern: vec![],
            max_depth: 4,
            follow_links: false,
            separate_modules: false,
            include_private: false,
            toc_file: "modules".to_owned(),
            no_headings: false,
            module_first: false,
            implicit_namespaces: false,
            automodule_options: BTreeSet::new(),
            suffix: "rst".to_owned(),
            remove_old: false,
            quiet: false,
            dry_run: false,
            force: false,
            full: false,
            append_syspath: false,
            header,
            author: None,
            version: None,
            release: None,
            extensions: vec![],
            template_dir: None,
        }
    }

    /// Effective automodule options: user-supplied, or the defaults.
    pub fn effective_automodule_options(&self) -> BTreeSet<String> {
        if self.automodule_options.is_empty() {
            DEFAULT_AUTOMODULE_OPTIONS
                .iter()
                .map(|s| s.to_string())
                .collect()
        } else {
            self.automodule_options.clone()
        }
    }
}
