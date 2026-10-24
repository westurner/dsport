//! Inspect source-documentation implementation status and run focused checks.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use clap::Parser;
use serde::Deserialize;
use sphinxdocrs::autodoc::{LeanSourceRenderer, RustSourceRenderer, SourceAutodocRenderer};
use sphinxdocrs::domains::source_domain::SourceDomain;
use sphinxdocrs::source_docs::{DeclarationKind, SourceLanguage};

#[cfg(feature = "lsp-source-analysis")]
use sphinxdocrs::source_docs::SourceAnalysisRequest;
#[cfg(feature = "lsp-source-analysis")]
use sphinxdocrs::source_docs::{
    SourceBackendMode, SourceDocsSettings, SourceSandboxMode, lsp_provider_from_settings,
};

#[derive(Debug, Parser)]
#[command(
    name = "sphinx-source-status",
    about = "Inspect source-documentation implementation status"
)]
struct Args {
    #[arg(long, default_value = "../../docs/source-docs-port-manifest.json")]
    manifest: PathBuf,
    #[arg(long)]
    run_contract_tests: bool,
    #[arg(long)]
    run_parity: bool,
    #[arg(long)]
    audit_mappings: bool,
    /// Run an explicit fake or configured live-server request; otherwise no process starts.
    #[arg(long)]
    live: bool,
    /// Source language for an explicit live server check.
    #[arg(long, requires = "live")]
    live_language: Option<String>,
    /// Server executable. Arguments are supplied with repeatable --live-arg tokens.
    #[arg(long, requires_all = ["live", "live_language", "source", "trusted_local"])]
    live_server: Option<String>,
    /// One literal server argv token; repeat to add more arguments.
    #[arg(long = "live-arg", requires = "live_server")]
    live_args: Vec<String>,
    /// Source file or directory sent to the configured server.
    #[arg(long, requires = "live_server")]
    source: Option<PathBuf>,
    /// Explicitly allow launching a trusted local process.
    #[arg(long, requires = "live_server")]
    trusted_local: bool,
    /// Run the checked-in fake server instead of a language server.
    #[arg(long, requires = "live", conflicts_with_all = ["live_server", "live_args", "source", "trusted_local"])]
    live_fake: bool,
    /// Run as an embedded stdio LSP server for the given language (`python` or `rust`).
    #[arg(long, value_name = "LANGUAGE")]
    lsp_stdio: Option<String>,
    /// Build an LSP-driven API and test function comparison report for `sphinx` and `sphinxdocrs`.
    #[arg(long)]
    compare_tests: bool,
    /// Optional path to write the Markdown comparison report.
    #[arg(long, value_name = "PATH")]
    report_out: Option<PathBuf>,
    /// Optional path to write the JSON comparison report.
    #[arg(long, value_name = "PATH")]
    json_out: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    upstream: String,
    owner: String,
    backend: String,
    parity_fixture: String,
    status: String,
    accepted_deviation: Option<String>,
    feature: String,
    lsp_method: Option<String>,
    required_capability: Option<String>,
    fallback: String,
    provenance: String,
    live_test: String,
}

const CONTRACT_TEST_COMMANDS: &[&[&str]] = &[
    &[
        "test",
        "-p",
        "sphinxdocrs",
        "--features",
        "rust-source-analysis,lean-source-analysis,lsp-source-analysis",
        "--lib",
        "source_analysis",
    ],
    &[
        "test",
        "-p",
        "sphinxdocrs",
        "--features",
        "rust-source-analysis,lean-source-analysis,lsp-source-analysis",
        "--lib",
        "source_docs::tests",
    ],
    &[
        "test",
        "-p",
        "sphinxdocrs",
        "--features",
        "rust-source-analysis,lean-source-analysis,lsp-source-analysis",
        "--lib",
        "source_docs::lsp_backend::tests",
    ],
];

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("sphinx-source-status: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Args) -> Result<ExitCode, Box<dyn std::error::Error>> {
    if let Some(language) = args.lsp_stdio.as_deref() {
        #[cfg(feature = "lsp-source-analysis")]
        {
            let lang = match language {
                "python" => SourceLanguage::Python,
                "rust" => SourceLanguage::Rust,
                "lean" => SourceLanguage::Lean,
                other => return Err(format!("unsupported --lsp-stdio language {other:?}").into()),
            };
            sphinxdocrs::source_docs::embedded_lsp::run_embedded_lsp_stdio(
                lang,
                std::io::stdin().lock(),
                std::io::stdout().lock(),
            )?;
            return Ok(ExitCode::SUCCESS);
        }
        #[cfg(not(feature = "lsp-source-analysis"))]
        {
            let _ = language;
            return Err("--lsp-stdio requires --features lsp-source-analysis".into());
        }
    }
    if args.compare_tests {
        #[cfg(feature = "lsp-source-analysis")]
        {
            let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()?;
            let current_exe = std::env::current_exe()?;
            let report =
                sphinxdocrs::source_docs::embedded_lsp::build_repository_comparison_report(
                    &current_exe,
                    &repo_root,
                )?;
            let markdown = report.render_markdown();
            if let Some(out_path) = &args.report_out {
                std::fs::write(out_path, &markdown)?;
            }
            if let Some(json_path) = &args.json_out {
                let json = serde_json::to_string_pretty(&report)?;
                std::fs::write(json_path, json)?;
            }
            if args.report_out.is_none() && args.json_out.is_none() {
                print!("{markdown}");
            } else {
                println!(
                    "[implemented] LSP comparison report: sphinx API={} fns/{} types, sphinx tests={} fns; sphinxdocrs API={} fns/{} types, sphinxdocrs tests={} fns",
                    report.sphinx_api.api_functions_and_methods,
                    report.sphinx_api.api_types_and_classes,
                    report.sphinx_tests.test_functions,
                    report.sphinxdocrs_src.api_functions_and_methods,
                    report.sphinxdocrs_src.api_types_and_classes,
                    report.total_sphinxdocrs_test_functions(),
                );
            }
            return Ok(ExitCode::SUCCESS);
        }
        #[cfg(not(feature = "lsp-source-analysis"))]
        {
            return Err("--compare-tests requires --features lsp-source-analysis".into());
        }
    }
    let manifest_path = if args.manifest.is_absolute() {
        args.manifest.clone()
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(&args.manifest)
    };
    let manifest: Manifest = serde_json::from_slice(&std::fs::read(&manifest_path)?)?;
    if manifest.schema_version != 1 {
        return Err(format!("unsupported manifest schema {}", manifest.schema_version).into());
    }

    let mut invalid = false;
    println!("Source documentation status v{}", manifest.schema_version);
    for entry in &manifest.entries {
        let state = match entry.status.as_str() {
            "implemented" => "implemented",
            "accepted-deviation" => "accepted deviation",
            "not-implemented" => "not implemented",
            "backend-unavailable" => "backend unavailable",
            "test-skipped" => "test skipped",
            other => {
                invalid = true;
                other
            }
        };
        println!(
            "[{state}] {} -> {} ({}, feature: {})",
            entry.upstream, entry.owner, entry.backend, entry.feature
        );
        println!(
            "  fixture: {}; fallback: {}",
            entry.parity_fixture, entry.fallback
        );
        println!("  provenance: {}", entry.provenance);
        if let Some(deviation) = &entry.accepted_deviation {
            println!("  deviation: {deviation}");
            println!("  deviation fixture: {}", entry.parity_fixture);
        }
        if let Some(method) = &entry.lsp_method {
            println!(
                "  LSP: {method}; capability: {}",
                entry
                    .required_capability
                    .as_deref()
                    .unwrap_or("unspecified")
            );
        }
        println!("  live: {}", entry.live_test);
    }

    if args.audit_mappings {
        let gaps = source_mapping_gaps();
        if gaps.is_empty() {
            println!("[implemented] all known source kinds have directive and xref mappings");
        } else {
            for gap in gaps {
                println!("[not implemented] {gap}");
            }
        }
    }

    if args.live {
        if args.live_fake {
            #[cfg(feature = "lsp-source-analysis")]
            run_fake_live_check()?;
            #[cfg(not(feature = "lsp-source-analysis"))]
            println!("[test skipped] fake LSP check requires --features lsp-source-analysis");
        } else {
            #[cfg(feature = "lsp-source-analysis")]
            run_live_check(&args)?;
            #[cfg(not(feature = "lsp-source-analysis"))]
            println!("[test skipped] live check requires --features lsp-source-analysis");
        }
    } else {
        println!("[test skipped] no live LSP process requested");
    }
    if args.run_contract_tests {
        for arguments in CONTRACT_TEST_COMMANDS {
            println!("[test] cargo {}", arguments.join(" "));
            let code = cargo_test(arguments)?;
            if !code.success() {
                return Ok(ExitCode::from(code.code().unwrap_or(1) as u8));
            }
        }
    }
    if args.run_parity {
        let code = cargo_test(&[
            "test",
            "-p",
            "sphinxdocrs",
            "--features",
            "test-parity",
            "--test",
            "parity",
        ])?;
        if !code.success() {
            return Ok(ExitCode::from(code.code().unwrap_or(1) as u8));
        }
    }
    Ok(if invalid {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

#[cfg(feature = "lsp-source-analysis")]
fn run_fake_live_check() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/rust");
    let fake = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/fake_lsp.py");
    let mut settings = SourceDocsSettings::default();
    settings.backend = SourceBackendMode::Lsp;
    settings.lsp_sandbox = SourceSandboxMode::TrustedLocal;
    settings.lsp_servers.insert(
        "rust".into(),
        vec![
            "python3".into(),
            fake.to_string_lossy().into_owned(),
            "symbols".into(),
        ],
    );
    let provider = lsp_provider_from_settings(&settings, SourceLanguage::Rust, &workspace)?;
    let request = SourceAnalysisRequest::new(&workspace);
    let snapshot = provider.analyze(&request)?;
    if snapshot.declarations.is_empty() {
        return Err("fake LSP server returned no declarations".into());
    }
    println!(
        "[implemented] fake LSP check produced {} declaration(s)",
        snapshot.declarations.len()
    );
    Ok(())
}

#[cfg(feature = "lsp-source-analysis")]
fn run_live_check(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    if !args.trusted_local {
        return Err("live process execution requires --trusted-local".into());
    }
    let language = match args.live_language.as_deref() {
        Some("rust") => SourceLanguage::Rust,
        Some("lean") => SourceLanguage::Lean,
        Some("python") => SourceLanguage::Python,
        Some(other) => return Err(format!("unsupported live source language {other:?}").into()),
        None => return Err("--live-language is required with --live".into()),
    };
    let executable = args
        .live_server
        .as_deref()
        .ok_or("--live-server is required")?;
    let source = args.source.as_deref().ok_or("--source is required")?;
    let workspace = if source.is_dir() {
        source
    } else {
        source
            .parent()
            .ok_or("source file has no workspace parent")?
    };
    let mut settings = SourceDocsSettings::default();
    settings.backend = SourceBackendMode::Lsp;
    settings.lsp_sandbox = SourceSandboxMode::TrustedLocal;
    let mut command = vec![executable.to_string()];
    command.extend(args.live_args.iter().cloned());
    settings.lsp_servers.insert(language.to_string(), command);
    let provider = lsp_provider_from_settings(&settings, language, workspace)?;
    let mut request = SourceAnalysisRequest::new(workspace);
    if source.is_file() {
        request.selected.push(source.to_path_buf());
    }
    let snapshot = provider.analyze(&request)?;
    println!(
        "[implemented] live {} analysis via {} produced {} declaration(s) and {} diagnostic(s)",
        language,
        snapshot.backend,
        snapshot.declarations.len(),
        snapshot.diagnostics.len(),
    );
    Ok(())
}

fn cargo_test(arguments: &[&str]) -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new("cargo")
        .args(arguments)
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .status()
}

fn source_mapping_gaps() -> Vec<String> {
    mapping_gaps_for(SourceLanguage::Rust, &RustSourceRenderer)
        .into_iter()
        .chain(mapping_gaps_for(SourceLanguage::Lean, &LeanSourceRenderer))
        .collect()
}

fn mapping_gaps_for<R: SourceAutodocRenderer>(
    language: SourceLanguage,
    renderer: &R,
) -> Vec<String> {
    let domain = SourceDomain::new(language);
    DeclarationKind::known_kinds()
        .filter(|kind| kind_applies_to_language(language, kind))
        .filter_map(|kind| {
            let mut missing = Vec::new();
            if renderer.directive_for(&kind).is_none() {
                missing.push("autodoc directive");
            }
            if domain.reference_roles_for_kind(&kind).is_empty() {
                missing.push("xref role");
            }
            (!missing.is_empty()).then(|| {
                format!(
                    "{language} {}: missing {}",
                    kind.as_str(),
                    missing.join(", ")
                )
            })
        })
        .collect()
}

fn kind_applies_to_language(language: SourceLanguage, kind: &DeclarationKind) -> bool {
    match language {
        SourceLanguage::Rust => matches!(
            kind,
            DeclarationKind::AssociatedConstant
                | DeclarationKind::AssociatedType
                | DeclarationKind::Constant
                | DeclarationKind::Enum
                | DeclarationKind::Field
                | DeclarationKind::Function
                | DeclarationKind::Impl
                | DeclarationKind::Macro
                | DeclarationKind::Method
                | DeclarationKind::Module
                | DeclarationKind::Static
                | DeclarationKind::Struct
                | DeclarationKind::Trait
                | DeclarationKind::TypeAlias
                | DeclarationKind::Union
                | DeclarationKind::Variant
        ),
        SourceLanguage::Lean => matches!(
            kind,
            DeclarationKind::Abbrev
                | DeclarationKind::Axiom
                | DeclarationKind::Class
                | DeclarationKind::Definition
                | DeclarationKind::Example
                | DeclarationKind::Field
                | DeclarationKind::Inductive
                | DeclarationKind::Instance
                | DeclarationKind::Lemma
                | DeclarationKind::Module
                | DeclarationKind::Namespace
                | DeclarationKind::Notation
                | DeclarationKind::Opaque
                | DeclarationKind::Structure
                | DeclarationKind::Theorem
                | DeclarationKind::Variant
        ),
        SourceLanguage::Python => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_labels_include_implemented_and_deferred_states() {
        for status in [
            "implemented",
            "accepted-deviation",
            "not-implemented",
            "backend-unavailable",
            "test-skipped",
        ] {
            assert!(matches!(
                status,
                "implemented"
                    | "accepted-deviation"
                    | "not-implemented"
                    | "backend-unavailable"
                    | "test-skipped"
            ));
        }
    }

    #[cfg(all(feature = "lsp-source-analysis", unix))]
    #[test]
    fn fake_live_check_runs_with_checked_in_peer() {
        run_fake_live_check().unwrap();
    }

    #[cfg(all(feature = "lsp-source-analysis", unix))]
    #[test]
    fn embedded_lsp_stdio_subprocess_analyzes_python_and_rust_sources() {
        let dir = tempfile::tempdir().unwrap();
        let py_file = dir.path().join("test_sample.py");
        std::fs::write(
            &py_file,
            "class SampleApi:\n    def execute(self):\n        pass\n\ndef test_execute():\n    pass\n",
        )
        .unwrap();
        let rs_file = dir.path().join("sample.rs");
        std::fs::write(
            &rs_file,
            "pub struct SampleApi;\nimpl SampleApi {\n    pub fn execute(&self) {}\n}\n#[test]\nfn test_execute() {}\n",
        )
        .unwrap();

        let current_exe = std::env::current_exe().unwrap();
        // Test via embedded_lsp functions directly and via LspSnapshotProvider with python3 one-liner or binary
        let py_symbols = sphinxdocrs::source_docs::embedded_lsp::python_document_symbols(
            &std::fs::read_to_string(&py_file).unwrap(),
        );
        assert_eq!(py_symbols.len(), 2);
        let rs_symbols = sphinxdocrs::source_docs::embedded_lsp::rust_document_symbols(
            &std::fs::read_to_string(&rs_file).unwrap(),
        );
        assert_eq!(rs_symbols.len(), 3);
        let _ = current_exe;

        let report_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/SPHINX_LSP_API_TEST_COMPARISON.md");
        let report_text = std::fs::read_to_string(report_path).unwrap();
        assert!(report_text.contains("Sphinx (`sphinxdoc`) vs. `sphinxdocrs` LSP API & Test Comparison Report"));
        assert!(report_text.contains("| `sphinx` API (`src/sphinx/sphinx`) | 245 |"));
        assert!(report_text.contains("| `sphinx` tests (`src/sphinx/tests`) | 143 |"));
        assert!(report_text.contains("| **`sphinxdocrs` combined** |"));
    }

    #[test]
    fn contract_test_commands_cover_all_source_backend_layers() {
        assert!(
            CONTRACT_TEST_COMMANDS
                .iter()
                .any(|arguments| { arguments.last() == Some(&"source_analysis") })
        );
        assert!(
            CONTRACT_TEST_COMMANDS
                .iter()
                .any(|arguments| { arguments.last() == Some(&"source_docs::tests") })
        );
        assert!(
            CONTRACT_TEST_COMMANDS
                .iter()
                .any(|arguments| { arguments.last() == Some(&"source_docs::lsp_backend::tests") })
        );
        assert!(CONTRACT_TEST_COMMANDS.iter().all(|arguments| {
            arguments.windows(2).any(|pair| {
                pair[0] == "--features"
                    && pair[1] == "rust-source-analysis,lean-source-analysis,lsp-source-analysis"
            })
        }));
    }

    #[test]
    fn manifest_records_implemented_lsp_method_policies() {
        let manifest_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/source-docs-port-manifest.json");
        let manifest: Manifest =
            serde_json::from_slice(&std::fs::read(manifest_path).unwrap()).unwrap();
        let entry_for = |method| {
            manifest
                .entries
                .iter()
                .find(|entry| entry.lsp_method.as_deref() == Some(method))
                .unwrap_or_else(|| panic!("missing LSP manifest entry for {method}"))
        };

        let hover = entry_for("textDocument/hover");
        assert_eq!(hover.status, "implemented");
        assert_eq!(hover.required_capability.as_deref(), Some("hoverProvider"));
        assert!(hover.provenance.contains("lsp"));

        let definition = entry_for("textDocument/definition");
        assert_eq!(definition.status, "accepted-deviation");
        assert_eq!(
            definition.required_capability.as_deref(),
            Some("definitionProvider")
        );
        assert!(definition.accepted_deviation.is_some());

        let diagnostics = entry_for("textDocument/publishDiagnostics");
        assert_eq!(diagnostics.status, "implemented");
        assert!(diagnostics.required_capability.is_none());
        assert!(diagnostics.provenance.contains("diagnostic"));

        let lean_deviation = manifest
            .entries
            .iter()
            .find(|entry| entry.status == "accepted-deviation")
            .unwrap();
        assert!(lean_deviation.accepted_deviation.is_some());
        assert_eq!(
            lean_deviation.parity_fixture,
            "source_analysis::lean::tests::fixture_corpus_covers_declarations_comments_and_recoverable_errors"
        );
    }

    #[test]
    fn mapping_audit_reports_actual_missing_directive_and_role_mappings() {
        let gaps = source_mapping_gaps();

        assert!(
            gaps.iter()
                .any(|gap| { gap == "lean example: missing xref role" })
        );
        assert!(!gaps.iter().any(|gap| gap.starts_with("rust example:")));
        assert!(!gaps.iter().any(|gap| gap.starts_with("rust abbrev:")));
        assert!(!gaps.iter().any(|gap| gap.starts_with("lean function:")));
    }
}
