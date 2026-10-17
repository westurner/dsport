//! `sphinx-apidoc-rs` — native Rust entry point.
//!
//! Falls back to upstream Python `sphinx-apidoc` when:
//! - `--use-python-impl` flag is present, or
//! - `SPHINXDOCRS_PY_FALLBACK=1` env var is set.

use sphinxdocrs::apidoc::generate::remove_old_files;
use sphinxdocrs::apidoc::{
    ApidocTemplates, SourceBackend, SourceMode, create_modules_toc_file, parse_args, recurse_tree,
};
use sphinxdocrs::cli::io::{RealFs, SystemClock, py_fallback_requested, run_python_impl};
use sphinxdocrs::quickstart::generate::generate as qs_generate;
use sphinxdocrs::quickstart::settings::QuickstartSettings;
use sphinxdocrs::quickstart::templates::QuickstartTemplates;

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    if py_fallback_requested(&argv) {
        run_python_impl("sphinx.ext.apidoc", &argv);
    }

    let opts = match parse_args(&argv) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    // Validate module_path
    if !opts.module_path.is_dir() {
        eprintln!("Error: {} is not a directory.", opts.module_path.display());
        std::process::exit(1);
    }

    if opts.source_backend == SourceBackend::Lsp {
        let mode =
            match sphinxdocrs::apidoc::generate::effective_source_mode(&opts.module_path, &opts) {
                Ok(mode) => mode,
                Err(error) => {
                    eprintln!("sphinx-apidoc-rs: {error}");
                    std::process::exit(1);
                }
            };
        if !matches!(mode, SourceMode::Rust | SourceMode::Lean) {
            eprintln!(
                "sphinx-apidoc-rs: --source-backend lsp requires Rust or Lean; Python apidoc remains unchanged"
            );
            std::process::exit(1);
        }
        #[cfg(not(all(
            feature = "lsp-source-analysis",
            any(feature = "rust-source-analysis", feature = "lean-source-analysis")
        )))]
        {
            eprintln!(
                "sphinx-apidoc-rs: LSP apidoc requires lsp-source-analysis and a Rust or Lean source feature"
            );
            std::process::exit(1);
        }
    }

    // Ensure output dir exists
    if !opts.dry_run {
        if let Err(e) = std::fs::create_dir_all(&opts.dest_dir) {
            eprintln!("Error: cannot create {}: {e}", opts.dest_dir.display());
            std::process::exit(1);
        }
    }

    // Compile exclude patterns
    let excludes: Vec<regex::Regex> = opts
        .exclude_pattern
        .iter()
        .filter_map(|pat| {
            let resolved = std::path::Path::new(pat)
                .canonicalize()
                .unwrap_or_else(|_| std::path::PathBuf::from(pat));
            let fnmatch_re = fnmatch_to_regex(&resolved.to_string_lossy());
            regex::Regex::new(&fnmatch_re).ok()
        })
        .collect();

    let templates = ApidocTemplates::with_templatedir(opts.template_dir.as_deref());

    let (mut written_files, mut modules) =
        match recurse_tree(&opts.module_path, &excludes, &opts, &templates) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        };

    if opts.source_backend == SourceBackend::Lsp {
        match generate_lsp_source_page(&opts) {
            Ok((path, docname)) => {
                written_files.push(path);
                modules.push(docname);
                modules.sort();
                modules.dedup();
            }
            Err(error) => {
                eprintln!("sphinx-apidoc-rs: {error}");
                std::process::exit(1);
            }
        }
    }

    if opts.full {
        run_full_quickstart(&opts, &modules);
    } else if !opts.toc_file.is_empty() {
        match create_modules_toc_file(&modules, &opts, &opts.toc_file, &templates) {
            Ok(path) => written_files.push(path),
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }

    if opts.remove_old && !opts.dry_run {
        if let Err(e) = remove_old_files(&written_files, &opts.dest_dir, &opts.suffix) {
            eprintln!("Warning during remove-old: {e}");
        }
    }
}

#[cfg(all(
    feature = "lsp-source-analysis",
    any(feature = "rust-source-analysis", feature = "lean-source-analysis")
))]
fn generate_lsp_source_page(
    opts: &sphinxdocrs::apidoc::ApidocOptions,
) -> Result<(std::path::PathBuf, String), String> {
    use std::collections::HashMap;

    use sphinxdocrs::autodoc::{
        LeanSourceRenderer, RustSourceRenderer, SourceAutodocRequest, document_source,
        render_source_rst_with_definition_url_template,
    };
    use sphinxdocrs::config::{SphinxConfig, raw_config_from_conf_py};
    use sphinxdocrs::source_docs::model::SourceLanguage;
    use sphinxdocrs::source_docs::provider::SourceBackendMode;
    use sphinxdocrs::source_docs::{SourceDocsSettings, lsp_provider_from_settings};

    if opts.dry_run {
        let path = sphinxdocrs::apidoc::generate::write_file("lsp-api", "", opts)
            .map_err(|error| error.to_string())?;
        return Ok((path, "lsp-api".into()));
    }
    let existing = opts.dest_dir.join(format!("lsp-api.{}", opts.suffix));
    if existing.is_file() && !opts.force {
        if !opts.quiet {
            println!(
                "File {} already exists, skipping LSP analysis.",
                existing.display()
            );
        }
        return Ok((existing, "lsp-api".into()));
    }

    let mode = sphinxdocrs::apidoc::generate::effective_source_mode(&opts.module_path, opts)
        .map_err(|error| error.to_string())?;
    let language = match mode {
        SourceMode::Rust => SourceLanguage::Rust,
        SourceMode::Lean => SourceLanguage::Lean,
        SourceMode::Python | SourceMode::Auto => {
            return Err(
                "--source-backend lsp requires --source-mode rust or lean; Python apidoc remains unchanged".into(),
            );
        }
    };

    let conf_file = opts
        .conf_file
        .clone()
        .or_else(|| {
            opts.module_path
                .ancestors()
                .map(|directory| directory.join("conf.py"))
                .find(|path| path.is_file())
        })
        .ok_or_else(|| {
            "LSP apidoc needs --conf-file or a conf.py in the source tree to supply server/MXC policy".to_string()
        })?;
    let raw_config = raw_config_from_conf_py(&conf_file).map_err(|error| {
        format!(
            "failed to load LSP configuration from {}: {error}",
            conf_file.display()
        )
    })?;
    let sphinx_config = SphinxConfig::new(raw_config, HashMap::new());
    let definition_url_template = sphinx_config
        .get("source_lsp_definition_url_template")
        .and_then(|value| value.as_str().map(str::to_owned))
        .filter(|value| !value.trim().is_empty());
    let mut settings = SourceDocsSettings::from_sphinx_config(&sphinx_config).map_err(|error| {
        format!(
            "invalid LSP configuration in {}: {error}",
            conf_file.display()
        )
    })?;
    settings.backend = SourceBackendMode::Lsp;

    let workspace = match mode {
        SourceMode::Rust => opts
            .cargo_manifest
            .as_ref()
            .map(|manifest| manifest.parent().unwrap_or(&opts.module_path).to_path_buf())
            .or_else(|| {
                opts.module_path
                    .join("Cargo.toml")
                    .is_file()
                    .then(|| opts.module_path.clone())
            })
            .or_else(|| {
                opts.module_path
                    .parent()
                    .filter(|parent| parent.join("Cargo.toml").is_file())
                    .map(std::path::Path::to_path_buf)
            })
            .unwrap_or_else(|| opts.module_path.clone()),
        SourceMode::Lean => opts.module_path.clone(),
        SourceMode::Python | SourceMode::Auto => unreachable!(),
    };
    let provider = lsp_provider_from_settings(&settings, language, &workspace)
        .map_err(|error| error.to_string())?;
    let mut request = SourceAutodocRequest::new(&workspace, language);
    request.include_private = opts.include_private;
    request.analysis.include_private = opts.include_private;
    let rst = match language {
        SourceLanguage::Rust => {
            let descriptions = document_source(&request, provider.as_ref(), &RustSourceRenderer)
                .map_err(|error| error.to_string())?;
            render_source_rst_with_definition_url_template(
                &descriptions,
                definition_url_template.as_deref(),
            )
        }
        SourceLanguage::Lean => {
            let descriptions = document_source(&request, provider.as_ref(), &LeanSourceRenderer)
                .map_err(|error| error.to_string())?;
            render_source_rst_with_definition_url_template(
                &descriptions,
                definition_url_template.as_deref(),
            )
        }
        SourceLanguage::Python => unreachable!(),
    };
    let title = format!("{} (LSP API)", opts.header);
    let mut page = format!("{title}\n{}\n\n", "=".repeat(title.chars().count()));
    if rst.trim().is_empty() {
        page.push_str(
            "No supported declarations were returned by the configured language server.\n",
        );
    } else {
        page.push_str(&rst);
    }
    let path = sphinxdocrs::apidoc::generate::write_file("lsp-api", &page, opts)
        .map_err(|error| error.to_string())?;
    eprintln!(
        "{}",
        sphinxdocrs::source_docs::config::sandboxing_config_summary(&sphinx_config)
    );
    Ok((path, "lsp-api".into()))
}

#[cfg(not(all(
    feature = "lsp-source-analysis",
    any(feature = "rust-source-analysis", feature = "lean-source-analysis")
)))]
fn generate_lsp_source_page(
    _opts: &sphinxdocrs::apidoc::ApidocOptions,
) -> Result<(std::path::PathBuf, String), String> {
    Err(
        "LSP apidoc requires the lsp-source-analysis feature and a Rust or Lean source feature"
            .into(),
    )
}

/// Native implementation of `_full_quickstart`: mirrors
/// `sphinx.ext.apidoc._cli._full_quickstart`.
///
/// Builds a `QuickstartSettings` from the apidoc options and calls
/// `quickstart::generate`. Falls back to Python only if the native
/// quickstart path itself fails.
fn run_full_quickstart(opts: &sphinxdocrs::apidoc::ApidocOptions, modules: &[String]) {
    // Build the sorted mastertoctree text (matches upstream logic).
    let mut sorted_mods = modules.to_vec();
    sorted_mods.sort();
    let mut prev = String::new();
    let mut text = String::new();
    for m in &sorted_mods {
        if m.starts_with(&format!("{prev}.")) {
            continue;
        }
        prev = m.clone();
        text.push_str(&format!("   {m}\n"));
    }

    let release = opts
        .release
        .clone()
        .or_else(|| opts.version.clone())
        .unwrap_or_default();

    let mut extensions = vec![
        "sphinx.ext.autodoc".to_owned(),
        "sphinx.ext.viewcode".to_owned(),
        "sphinx.ext.todo".to_owned(),
    ];
    extensions.extend(opts.extensions.clone());

    let qs = QuickstartSettings {
        path: opts.dest_dir.clone(),
        sep: false,
        dot: "_".to_owned(),
        project: opts.header.clone(),
        author: opts.author.clone().unwrap_or_else(|| "Author".to_owned()),
        version: opts.version.clone().unwrap_or_default(),
        release,
        suffix: format!(".{}", opts.suffix),
        master: "index".to_owned(),
        extensions,
        makefile: true,
        batchfile: true,
        quiet: opts.quiet,
        language: Some("en".to_owned()),
        templatedir: opts.template_dir.clone(),
    };

    if let Err(e) = std::fs::create_dir_all(&qs.path) {
        eprintln!("Error creating {}: {e}", qs.path.display());
        std::process::exit(1);
    }

    let qs_templates = QuickstartTemplates::with_templatedir(opts.template_dir.as_deref());
    let clock = SystemClock;

    if let Err(e) = qs_generate(&qs, &qs_templates, &RealFs, &clock) {
        eprintln!("Error in --full quickstart: {e}");
        std::process::exit(1);
    }

    // Also write the mastertoctree into the index file if non-empty.
    if !text.is_empty() {
        let index = qs.path.join(format!("index.{}", opts.suffix));
        if let Ok(existing) = std::fs::read_to_string(&index) {
            // Append toctree entries to the existing index (best effort).
            let updated = existing.replace("{{ mastertoctree }}", &text);
            let _ = std::fs::write(&index, updated);
        }
    }
}

/// Translate an fnmatch pattern to a regex string (simplified).
fn fnmatch_to_regex(pat: &str) -> String {
    let mut re = String::from("(?s)");
    for ch in pat.chars() {
        match ch {
            '*' => re.push_str(".*"),
            '?' => re.push('.'),
            '.' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '+' | '\\' => {
                re.push('\\');
                re.push(ch);
            }
            c => re.push(c),
        }
    }
    re
}
