//! Native source-aware autodoc entry point.

use std::path::PathBuf;

use clap::{Parser, ValueEnum};
#[cfg(feature = "lean-source-analysis")]
use sphinxdocrs::autodoc::LeanSourceRenderer;
#[cfg(feature = "rust-source-analysis")]
use sphinxdocrs::autodoc::RustSourceRenderer;
#[cfg(any(feature = "rust-source-analysis", feature = "lean-source-analysis"))]
use sphinxdocrs::autodoc::SourceAutodocRequest;
#[cfg(any(feature = "rust-source-analysis", feature = "lean-source-analysis"))]
use sphinxdocrs::autodoc::{document_source, render_source_rst};
#[cfg(any(feature = "rust-source-analysis", feature = "lean-source-analysis"))]
use sphinxdocrs::source_analysis::SourceLanguage;
#[cfg(all(
    feature = "lsp-source-analysis",
    any(feature = "rust-source-analysis", feature = "lean-source-analysis")
))]
use sphinxdocrs::source_docs::{
    SourceAnalysisSession, SourceBackendMode, SourceDocsSettings, SourceSandboxMode,
    lsp_provider_from_settings,
};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SourceKind {
    Auto,
    Lean,
    Rust,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum BackendMode {
    Auto,
    Static,
    Lsp,
    Hybrid,
}

#[cfg(all(
    feature = "lsp-source-analysis",
    any(feature = "rust-source-analysis", feature = "lean-source-analysis")
))]
impl From<BackendMode> for SourceBackendMode {
    fn from(value: BackendMode) -> Self {
        match value {
            BackendMode::Auto => Self::Auto,
            BackendMode::Static => Self::Static,
            BackendMode::Lsp => Self::Lsp,
            BackendMode::Hybrid => Self::Hybrid,
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "sphinx-autodoc-rs",
    about = "Render Rust or Lean source documentation as RST"
)]
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
    /// Select static analysis (default), required LSP, or static-base hybrid mode.
    #[arg(long, value_enum, default_value_t = BackendMode::Static)]
    source_backend: BackendMode,
    /// Explicit argv for the selected language server; repeat to append arguments.
    #[arg(long = "source-lsp-server")]
    source_lsp_server: Option<String>,
    /// One literal argument token for the configured server; repeat as needed.
    #[arg(long = "source-lsp-arg", action = clap::ArgAction::Append)]
    source_lsp_args: Vec<String>,
    /// Request timeout in milliseconds.
    #[arg(long = "source-lsp-timeout", default_value_t = 30_000)]
    source_lsp_timeout: u64,
    /// Require LSP success instead of falling back in hybrid mode.
    #[arg(long = "source-lsp-no-fallback")]
    source_lsp_no_fallback: bool,
    /// Explicitly permit launching a trusted local language server.
    #[arg(long = "source-lsp-sandbox", value_enum, default_value_t = CliSandbox::Off)]
    source_lsp_sandbox: CliSandbox,
    /// Additional absolute roots mounted read-only for protected LSP.
    #[arg(long = "source-lsp-read-only-root", action = clap::ArgAction::Append)]
    source_lsp_read_only_roots: Vec<PathBuf>,
    /// Explicit environment allowlist entry for protected LSP, in KEY=VALUE form.
    #[arg(long = "source-lsp-sandbox-env", action = clap::ArgAction::Append)]
    source_lsp_sandbox_environment: Vec<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
enum CliSandbox {
    #[default]
    Off,
    TrustedLocal,
    ProtectedLsp,
}

fn main() {
    let args = Args::parse();
    let source_kind = resolve_source_kind(&args);
    let result = source_kind.and_then(|kind| render(&args, kind));
    match result {
        Ok(rst) => {
            if let Some(path) = args.output {
                if let Err(error) = std::fs::write(&path, rst) {
                    eprintln!(
                        "sphinx-autodoc-rs: failed to write {}: {error}",
                        path.display()
                    );
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
    if args
        .source_root
        .extension()
        .is_some_and(|extension| extension == "lean")
    {
        return Ok(SourceKind::Lean);
    }
    if args.source_root.is_file()
        && args
            .source_root
            .extension()
            .is_some_and(|extension| extension == "json")
    {
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
        let static_provider: Box<dyn sphinxdocrs::source_docs::SourceSnapshotProvider> =
            Box::new(sphinxdocrs::source_analysis::rust::RustdocJsonAnalyzer::default());
        let descriptions = if matches!(args.source_backend, BackendMode::Static | BackendMode::Auto) {
            document_source(&request, static_provider.as_ref(), &RustSourceRenderer)
        } else {
            #[cfg(feature = "lsp-source-analysis")]
            {
                let session = source_session(args, SourceLanguage::Rust, static_provider)?;
                document_source(&request, &session, &RustSourceRenderer)
            }
            #[cfg(not(feature = "lsp-source-analysis"))]
            {
                return Err("LSP source analysis is not enabled; rebuild with --features lsp-source-analysis".into());
            }
        }
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
        let static_provider: Box<dyn sphinxdocrs::source_docs::SourceSnapshotProvider> =
            Box::new(sphinxdocrs::source_analysis::lean::ArboriumLeanAnalyzer);
        let descriptions = if matches!(args.source_backend, BackendMode::Static | BackendMode::Auto) {
            document_source(&request, static_provider.as_ref(), &LeanSourceRenderer)
        } else {
            #[cfg(feature = "lsp-source-analysis")]
            {
                let session = source_session(args, SourceLanguage::Lean, static_provider)?;
                document_source(&request, &session, &LeanSourceRenderer)
            }
            #[cfg(not(feature = "lsp-source-analysis"))]
            {
                return Err("LSP source analysis is not enabled; rebuild with --features lsp-source-analysis".into());
            }
        }
            .map_err(|error| format!("{}: {}", args.source_root.display(), error))?;
        return Ok(render_source_rst(&descriptions));
    }
    #[cfg(not(feature = "lean-source-analysis"))]
    {
        let _ = args;
        Err("Lean source analysis backend is not enabled; rebuild with --features lean".to_string())
    }
}

#[cfg(all(
    feature = "lsp-source-analysis",
    any(feature = "rust-source-analysis", feature = "lean-source-analysis")
))]
fn source_session(
    args: &Args,
    language: SourceLanguage,
    static_provider: Box<dyn sphinxdocrs::source_docs::SourceSnapshotProvider>,
) -> Result<SourceAnalysisSession, String> {
    let Some(executable) = &args.source_lsp_server else {
        return Err("--source-lsp-server is required for lsp/hybrid mode".into());
    };
    let mut settings = SourceDocsSettings::default();
    settings.backend = match args.source_backend {
        BackendMode::Lsp => SourceBackendMode::Lsp,
        other => other.into(),
    };
    settings.lsp_timeout_ms = args.source_lsp_timeout;
    settings.lsp_allow_fallback = !args.source_lsp_no_fallback;
    settings.lsp_sandbox = sandbox_mode_from_cli(args.source_lsp_sandbox);
    settings.lsp_read_only_roots = args.source_lsp_read_only_roots.clone();
    for (key, value) in parse_sandbox_environment(&args.source_lsp_sandbox_environment)? {
        settings.lsp_sandbox_environment.insert(key, value);
    }
    let mut command = vec![executable.clone()];
    command.extend(args.source_lsp_args.iter().cloned());
    settings.lsp_servers.insert(language.to_string(), command);
    let lsp = lsp_provider_from_settings(&settings, language, &args.source_root)
        .map_err(|error| error.to_string())?;
    let static_provider = if matches!(args.source_backend, BackendMode::Lsp) {
        None
    } else {
        Some(static_provider)
    };
    Ok(SourceAnalysisSession::from_settings(
        &settings,
        &language.to_string(),
        static_provider,
        Some(lsp),
    ))
}

#[cfg(all(
    feature = "lsp-source-analysis",
    any(feature = "rust-source-analysis", feature = "lean-source-analysis")
))]
fn sandbox_mode_from_cli(value: CliSandbox) -> SourceSandboxMode {
    match value {
        CliSandbox::Off => SourceSandboxMode::Off,
        CliSandbox::TrustedLocal => SourceSandboxMode::TrustedLocal,
        CliSandbox::ProtectedLsp => SourceSandboxMode::ProtectedLsp,
    }
}

#[cfg(any(
    test,
    all(
        feature = "lsp-source-analysis",
        any(feature = "rust-source-analysis", feature = "lean-source-analysis")
    )
))]
fn parse_sandbox_environment(entries: &[String]) -> Result<Vec<(String, String)>, String> {
    entries
        .iter()
        .map(|entry| {
            let (key, value) = entry
                .split_once('=')
                .filter(|(key, value)| !key.is_empty() && !value.contains('\0'))
                .ok_or_else(|| {
                    "--source-lsp-sandbox-env must use KEY=VALUE with a nonempty key".to_string()
                })?;
            if key.contains('\0') {
                return Err("--source-lsp-sandbox-env keys cannot contain NUL".into());
            }
            Ok((key.to_string(), value.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protected_sandbox_args_accept_repeatable_roots_and_environment() {
        let args = Args::try_parse_from([
            "sphinx-autodoc-rs",
            "src",
            "--source-backend",
            "lsp",
            "--source-lsp-server",
            "rust-analyzer",
            "--source-lsp-sandbox",
            "protected-lsp",
            "--source-lsp-read-only-root",
            "/opt/rust-toolchain",
            "--source-lsp-read-only-root",
            "/opt/rust-src",
            "--source-lsp-sandbox-env",
            "PATH=/opt/rust-toolchain/bin",
            "--source-lsp-sandbox-env",
            "RUSTUP_HOME=/opt/rust-toolchain/rustup",
        ])
        .unwrap();

        assert_eq!(args.source_lsp_read_only_roots.len(), 2);
        assert!(matches!(args.source_lsp_sandbox, CliSandbox::ProtectedLsp));
        assert_eq!(
            parse_sandbox_environment(&args.source_lsp_sandbox_environment).unwrap(),
            [
                ("PATH".into(), "/opt/rust-toolchain/bin".into()),
                ("RUSTUP_HOME".into(), "/opt/rust-toolchain/rustup".into()),
            ]
        );
    }

    #[test]
    fn protected_sandbox_environment_rejects_malformed_entries() {
        for entry in [
            "=value",
            "missing-separator",
            "KEY=value\0injected",
            "KEY\0injected=value",
        ] {
            assert!(
                parse_sandbox_environment(&[entry.into()]).is_err(),
                "{entry:?}"
            );
        }
    }

    #[cfg(all(
        feature = "lsp-source-analysis",
        any(feature = "rust-source-analysis", feature = "lean-source-analysis")
    ))]
    #[test]
    fn protected_sandbox_cli_mode_maps_to_protected_policy() {
        assert!(matches!(
            sandbox_mode_from_cli(CliSandbox::ProtectedLsp),
            SourceSandboxMode::ProtectedLsp
        ));
    }
}
