//! Native source-aware autodoc entry point.

use std::path::PathBuf;

use clap::{Parser, ValueEnum};
#[cfg(any(feature = "rust-source-analysis", feature = "lean-source-analysis"))]
use sphinxdocrs::autodoc::SourceAutodocRequest;
#[cfg(any(feature = "rust-source-analysis", feature = "lean-source-analysis"))]
use sphinxdocrs::autodoc::{document_source, render_source_rst};
#[cfg(feature = "lean-source-analysis")]
use sphinxdocrs::autodoc::LeanSourceRenderer;
#[cfg(feature = "rust-source-analysis")]
use sphinxdocrs::autodoc::RustSourceRenderer;
#[cfg(any(feature = "rust-source-analysis", feature = "lean-source-analysis"))]
use sphinxdocrs::source_analysis::SourceLanguage;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SourceKind {
    Auto,
    Lean,
    Rust,
}

#[derive(Debug, Parser)]
#[command(name = "sphinx-autodoc-rs", about = "Render Rust or Lean source documentation as RST")]
struct Args {
    /// Rust source root, Lean project/module root, or a rustdoc JSON file.
    source_root: PathBuf,
    /// Select the source language; auto infers Lean from .lean input and Rust otherwise.
    #[arg(long = "source-kind", value_enum, default_value_t = SourceKind::Auto)]
    source_kind: SourceKind,
    /// Rustdoc JSON produced by the matching rustdoc toolchain.
    #[arg(long)]
    rustdoc_json: Option<PathBuf>,
    /// Include private declarations in the generated descriptions.
    #[arg(long)]
    include_private: bool,
    /// Include declarations marked hidden or no-index.
    #[arg(long)]
    include_hidden: bool,
    /// Exclude declarations marked deprecated.
    #[arg(long)]
    exclude_deprecated: bool,
    /// Write RST to this path instead of stdout.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let source_kind = resolve_source_kind(&args);
    let result = source_kind.and_then(|kind| render(&args, kind));
    match result {
        Ok(rst) => {
            if let Some(path) = args.output {
                if let Err(error) = std::fs::write(&path, rst) {
                    eprintln!("sphinx-autodoc-rs: failed to write {}: {error}", path.display());
                    std::process::exit(1);
                }
            } else {
                print!("{rst}");
            }
        }
        Err(error) => {
            eprintln!("sphinx-autodoc-rs: {error}");
            std::process::exit(1);
        }
    }
}

fn resolve_source_kind(args: &Args) -> Result<SourceKind, String> {
    if !matches!(args.source_kind, SourceKind::Auto) {
        return Ok(args.source_kind);
    }
    if args.source_root.extension().is_some_and(|extension| extension == "lean") {
        return Ok(SourceKind::Lean);
    }
    if args.source_root.is_file() && args.source_root.extension().is_some_and(|extension| extension == "json") {
        return Ok(SourceKind::Rust);
    }
    if args.source_root.join("Cargo.toml").is_file()
        || args.source_root.join("src").is_dir()
        || args.rustdoc_json.is_some()
    {
        return Ok(SourceKind::Rust);
    }
    Err("cannot infer source kind; pass --source-kind rust or --source-kind lean".to_string())
}

fn render(args: &Args, kind: SourceKind) -> Result<String, String> {
    match kind {
        SourceKind::Auto => unreachable!("auto source kind is resolved before rendering"),
        SourceKind::Lean => render_lean(args),
        SourceKind::Rust => render_rust(args),
    }
}

#[cfg(any(feature = "rust-source-analysis", feature = "lean-source-analysis"))]
fn request(args: &Args, language: SourceLanguage) -> SourceAutodocRequest {
    let mut request = SourceAutodocRequest::new(args.source_root.clone(), language);
    request.include_private = args.include_private;
    request.analysis.include_private = args.include_private;
    request.include_hidden = args.include_hidden;
    request.include_deprecated = !args.exclude_deprecated;
    request
}

fn render_rust(args: &Args) -> Result<String, String> {
    #[cfg(feature = "rust-source-analysis")]
    {
        let mut request = request(args, SourceLanguage::Rust);
        if let Some(path) = &args.rustdoc_json {
            request.analysis.selected.push(path.clone());
        }
        let descriptions = document_source(&request, &sphinxdocrs::source_analysis::rust::RustdocJsonAnalyzer::default(), &RustSourceRenderer)
            .map_err(|error| format!("{}: {}", args.source_root.display(), error))?;
        return Ok(render_source_rst(&descriptions));
    }
    #[cfg(not(feature = "rust-source-analysis"))]
    {
        let _ = args;
        Err("Rust source analysis backend is not enabled; rebuild with --features rust".to_string())
    }
}

fn render_lean(args: &Args) -> Result<String, String> {
    #[cfg(feature = "lean-source-analysis")]
    {
        let request = request(args, SourceLanguage::Lean);
        let descriptions = document_source(&request, &sphinxdocrs::source_analysis::lean::ArboriumLeanAnalyzer, &LeanSourceRenderer)
            .map_err(|error| format!("{}: {}", args.source_root.display(), error))?;
        return Ok(render_source_rst(&descriptions));
    }
    #[cfg(not(feature = "lean-source-analysis"))]
    {
        let _ = args;
        Err("Lean source analysis backend is not enabled; rebuild with --features lean".to_string())
    }
}
