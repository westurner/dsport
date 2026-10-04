//! Backend-neutral facade for Rust and Lean source documentation.
//!
//! Existing implementation modules remain in place during the migration;
//! consumers can depend on this facade without taking a dependency on a
//! parser-specific adapter.

pub mod model {
    pub use crate::source_analysis::{
        AnalysisDiagnostic, AnalysisError, AnalysisSnapshot, BackendPolicy, DeclarationKind,
        DiagnosticSeverity, SOURCE_ANALYSIS_SCHEMA_VERSION, SourceAnalysisRequest,
        SourceBackendKind, SourceDeclaration, SourceLanguage, SourcePosition, SourceProvenance,
        SourceSpan, Visibility, normalize_name, normalize_path, normalize_text, parent_name,
        separator_for, short_name, source_input_hash, stable_declaration_id,
    };
}

pub mod provider {
    pub use crate::source_analysis::{
        AnalysisError, AnalysisSnapshot, SourceAnalysisRequest, SourceAnalyzer, SourceBackendKind,
        SourceSnapshotProvider,
    };

    use super::model::{AnalysisDiagnostic, DiagnosticSeverity};

    /// Explicit source provider mode. `Auto` preserves the static default.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum SourceBackendMode {
        #[default]
        Auto,
        Static,
        Lsp,
        Hybrid,
    }

    /// Per-build provider configuration. Providers are owned by the session
    /// and are never part of persisted environment state.
    pub struct SourceAnalysisSession {
        pub mode: SourceBackendMode,
        pub static_provider: Option<Box<dyn SourceSnapshotProvider>>,
        pub lsp_provider: Option<Box<dyn SourceSnapshotProvider>>,
        pub allow_unmatched_lsp_declarations: bool,
        pub configuration_identity: Option<String>,
        pub allow_fallback: bool,
    }

    impl SourceAnalysisSession {
        pub fn new(
            mode: SourceBackendMode,
            static_provider: Option<Box<dyn SourceSnapshotProvider>>,
            lsp_provider: Option<Box<dyn SourceSnapshotProvider>>,
        ) -> Self {
            Self {
                mode,
                static_provider,
                lsp_provider,
                allow_unmatched_lsp_declarations: false,
                configuration_identity: None,
                allow_fallback: true,
            }
        }

        pub fn from_settings(
            settings: &super::config::SourceDocsSettings,
            language: &str,
            static_provider: Option<Box<dyn SourceSnapshotProvider>>,
            lsp_provider: Option<Box<dyn SourceSnapshotProvider>>,
        ) -> Self {
            let mode = settings.backend;
            let mut session = Self::new(mode, static_provider, lsp_provider);
            session.allow_fallback = settings.lsp_allow_fallback;
            if matches!(mode, SourceBackendMode::Lsp | SourceBackendMode::Hybrid) {
                session.configuration_identity =
                    Some(settings.lsp_configuration_identity(language));
            }
            session
        }

        /// Allow hybrid mode to append declarations absent from the static
        /// snapshot. Disabled by default because static analysis owns API
        /// visibility and membership policy.
        pub fn with_unmatched_lsp_declarations(mut self, allow: bool) -> Self {
            self.allow_unmatched_lsp_declarations = allow;
            self
        }

        pub fn analyze(
            &self,
            request: &SourceAnalysisRequest,
        ) -> Result<AnalysisSnapshot, AnalysisError> {
            match self.mode {
                SourceBackendMode::Auto | SourceBackendMode::Static => self.run_static(request),
                SourceBackendMode::Lsp => {
                    let provider = self
                        .lsp_provider
                        .as_ref()
                        .ok_or_else(|| unavailable("lsp", "no LSP provider is configured"))?;
                    let mut snapshot = self.run_provider(provider.as_ref(), request)?;
                    snapshot.backend_kind = SourceBackendKind::Lsp;
                    snapshot.request_identity = self.cache_identity(request);
                    Ok(snapshot)
                }
                SourceBackendMode::Hybrid => self.run_hybrid(request),
            }
        }

        fn run_static(
            &self,
            request: &SourceAnalysisRequest,
        ) -> Result<AnalysisSnapshot, AnalysisError> {
            let provider = self
                .static_provider
                .as_ref()
                .ok_or_else(|| unavailable("static", "no static provider is configured"))?;
            self.run_provider(provider.as_ref(), request)
        }

        fn run_provider(
            &self,
            provider: &dyn SourceSnapshotProvider,
            request: &SourceAnalysisRequest,
        ) -> Result<AnalysisSnapshot, AnalysisError> {
            let mut snapshot = provider.analyze(request)?;
            snapshot.backend_kind = provider.backend_kind();
            snapshot.request_identity = provider.cache_identity(request);
            let provenance = match provider.backend_kind() {
                SourceBackendKind::Static => super::model::SourceProvenance::Static,
                SourceBackendKind::Lsp => super::model::SourceProvenance::Lsp,
                SourceBackendKind::Hybrid => super::model::SourceProvenance::MergedStaticLsp,
            };
            let declaration_ids = snapshot
                .declarations
                .iter()
                .map(|declaration| declaration.id.clone())
                .collect::<Vec<_>>();
            for declaration_id in declaration_ids {
                for field in [
                    "documentation",
                    "signature",
                    "type_text",
                    "visibility",
                    "deprecated",
                    "noindex",
                    "aliases",
                    "id",
                ] {
                    snapshot.set_provenance(&declaration_id, field, provenance);
                }
            }
            snapshot.normalize_and_sort();
            Ok(snapshot)
        }

        fn run_hybrid(
            &self,
            request: &SourceAnalysisRequest,
        ) -> Result<AnalysisSnapshot, AnalysisError> {
            let mut snapshot = self.run_static(request)?;
            snapshot.backend_kind = SourceBackendKind::Hybrid;
            let Some(provider) = self.lsp_provider.as_ref() else {
                if !self.allow_fallback {
                    return Err(unavailable(
                        "lsp",
                        "hybrid mode requires an LSP provider when fallback is disabled",
                    ));
                }
                snapshot.diagnostics.push(AnalysisDiagnostic {
                    severity: DiagnosticSeverity::Info,
                    backend: "hybrid".to_string(),
                    message: "LSP enrichment unavailable; using static snapshot".to_string(),
                    source: None,
                    declaration: None,
                });
                snapshot.request_identity = self.cache_identity(request);
                snapshot.provenance.clear();
                let declaration_ids = snapshot
                    .declarations
                    .iter()
                    .map(|declaration| declaration.id.clone())
                    .collect::<Vec<_>>();
                for declaration_id in declaration_ids {
                    for field in [
                        "documentation",
                        "signature",
                        "type_text",
                        "visibility",
                        "deprecated",
                        "noindex",
                        "aliases",
                        "id",
                    ] {
                        snapshot.set_provenance(
                            &declaration_id,
                            field,
                            super::model::SourceProvenance::Static,
                        );
                    }
                }
                snapshot.normalize_and_sort();
                return Ok(snapshot);
            };

            match provider.analyze(request) {
                Ok(mut enriched) => {
                    enriched.normalize_and_sort();
                    merge_enrichment(
                        &mut snapshot,
                        enriched,
                        self.allow_unmatched_lsp_declarations,
                    );
                    if let Some(static_provider) = self.static_provider.as_ref() {
                        snapshot.source_hash =
                            static_provider.source_hash(request, snapshot.language)?;
                    }
                }
                Err(error) if !self.allow_fallback => return Err(error),
                Err(error) => {
                    snapshot.diagnostics.push(AnalysisDiagnostic {
                        severity: DiagnosticSeverity::Warning,
                        backend: "hybrid".to_string(),
                        message: format!("LSP enrichment failed; using static snapshot: {error}"),
                        source: None,
                        declaration: None,
                    });
                    let declaration_ids = snapshot
                        .declarations
                        .iter()
                        .map(|declaration| declaration.id.clone())
                        .collect::<Vec<_>>();
                    for declaration_id in declaration_ids {
                        for field in [
                            "documentation",
                            "signature",
                            "type_text",
                            "visibility",
                            "deprecated",
                            "noindex",
                            "aliases",
                            "id",
                        ] {
                            snapshot.set_provenance(
                                &declaration_id,
                                field,
                                super::model::SourceProvenance::Static,
                            );
                        }
                    }
                }
            }
            snapshot.request_identity = self.cache_identity(request);
            snapshot.normalize_and_sort();
            Ok(snapshot)
        }
    }

    impl SourceSnapshotProvider for SourceAnalysisSession {
        fn analyze(
            &self,
            request: &SourceAnalysisRequest,
        ) -> Result<AnalysisSnapshot, AnalysisError> {
            SourceAnalysisSession::analyze(self, request)
        }

        fn backend_kind(&self) -> SourceBackendKind {
            match self.mode {
                SourceBackendMode::Auto | SourceBackendMode::Static => SourceBackendKind::Static,
                SourceBackendMode::Lsp => SourceBackendKind::Lsp,
                SourceBackendMode::Hybrid => SourceBackendKind::Hybrid,
            }
        }

        fn cache_identity(&self, request: &SourceAnalysisRequest) -> String {
            let provider_identity = |provider: &Option<Box<dyn SourceSnapshotProvider>>| {
                provider.as_ref().map_or_else(
                    || "missing".to_string(),
                    |provider| provider.cache_identity(request),
                )
            };
            match self.mode {
                SourceBackendMode::Auto | SourceBackendMode::Static => {
                    provider_identity(&self.static_provider)
                }
                SourceBackendMode::Lsp => {
                    format!(
                        "lsp:{}:{}",
                        provider_identity(&self.lsp_provider),
                        self.configuration_identity.as_deref().unwrap_or_default(),
                    )
                }
                SourceBackendMode::Hybrid => format!(
                    "hybrid:{}:{}:{}:fallback={}",
                    provider_identity(&self.static_provider),
                    provider_identity(&self.lsp_provider),
                    self.configuration_identity.as_deref().unwrap_or_default(),
                    self.allow_fallback,
                ),
            }
        }

        fn source_hash(
            &self,
            request: &SourceAnalysisRequest,
            language: super::model::SourceLanguage,
        ) -> Result<String, AnalysisError> {
            match self.mode {
                SourceBackendMode::Auto | SourceBackendMode::Static => self
                    .static_provider
                    .as_ref()
                    .ok_or_else(|| unavailable("static", "no static provider is configured"))?
                    .source_hash(request, language),
                SourceBackendMode::Lsp => self
                    .lsp_provider
                    .as_ref()
                    .ok_or_else(|| unavailable("lsp", "no LSP provider is configured"))?
                    .source_hash(request, language),
                SourceBackendMode::Hybrid => self.static_provider.as_ref().map_or_else(
                    || Ok(request.cache_identity()),
                    |provider| provider.source_hash(request, language),
                ),
            }
        }
    }

    fn unavailable(backend: &str, message: &str) -> AnalysisError {
        AnalysisError::BackendUnavailable {
            backend: backend.to_string(),
            message: message.to_string(),
        }
    }

    fn merge_enrichment(
        static_snapshot: &mut AnalysisSnapshot,
        lsp_snapshot: AnalysisSnapshot,
        allow_unmatched: bool,
    ) {
        for lsp_declaration in lsp_snapshot.declarations {
            let declaration_index = static_snapshot
                .declarations
                .iter()
                .position(|item| {
                    item.source.path == lsp_declaration.source.path
                        && item.source.start == lsp_declaration.source.start
                })
                .or_else(|| {
                    static_snapshot
                        .declarations
                        .iter()
                        .position(|item| item.qualified_name == lsp_declaration.qualified_name)
                });
            let Some(declaration_index) = declaration_index else {
                if allow_unmatched {
                    let declaration_id = lsp_declaration.id.clone();
                    static_snapshot.declarations.push(lsp_declaration);
                    for field in [
                        "documentation",
                        "signature",
                        "type_text",
                        "visibility",
                        "deprecated",
                        "noindex",
                        "aliases",
                        "id",
                    ] {
                        static_snapshot.set_provenance(
                            &declaration_id,
                            field,
                            super::model::SourceProvenance::Lsp,
                        );
                    }
                }
                continue;
            };
            let mut conflicts = Vec::new();
            let mut lsp_fields = Vec::new();
            let static_declaration = &mut static_snapshot.declarations[declaration_index];

            if !static_declaration.documentation.is_empty()
                && !lsp_declaration.documentation.is_empty()
                && static_declaration.documentation != lsp_declaration.documentation
            {
                conflicts.push("documentation");
            }
            if static_declaration.signature.is_some()
                && lsp_declaration.signature.is_some()
                && static_declaration.signature != lsp_declaration.signature
            {
                conflicts.push("signature");
            }
            if static_declaration.type_text.is_some()
                && lsp_declaration.type_text.is_some()
                && static_declaration.type_text != lsp_declaration.type_text
            {
                conflicts.push("type_text");
            }

            if static_declaration.documentation.is_empty()
                && !lsp_declaration.documentation.is_empty()
            {
                static_declaration.documentation = lsp_declaration.documentation;
                lsp_fields.push("documentation");
            }
            if static_declaration.signature.is_none() {
                static_declaration.signature = lsp_declaration.signature;
                if static_declaration.signature.is_some() {
                    lsp_fields.push("signature");
                }
            }
            if static_declaration.type_text.is_none() {
                static_declaration.type_text = lsp_declaration.type_text;
                if static_declaration.type_text.is_some() {
                    lsp_fields.push("type_text");
                }
            }
            for (key, value) in lsp_declaration.attributes {
                static_declaration
                    .attributes
                    .entry(format!("lsp:{key}"))
                    .or_insert(value);
            }
            if static_declaration.visibility != lsp_declaration.visibility {
                conflicts.push("visibility");
            }
            if static_declaration.deprecated != lsp_declaration.deprecated {
                conflicts.push("deprecated");
            }
            if static_declaration.noindex != lsp_declaration.noindex {
                conflicts.push("noindex");
            }
            if static_declaration.aliases != lsp_declaration.aliases {
                conflicts.push("aliases");
            }
            if static_declaration.id != lsp_declaration.id {
                conflicts.push("id");
            }
            let declaration_id = static_declaration.id.clone();
            let declaration_name = static_declaration.qualified_name.clone();
            let declaration_source = static_declaration.source.clone();
            for field in lsp_fields {
                static_snapshot.set_provenance(
                    &declaration_id,
                    field,
                    super::model::SourceProvenance::Lsp,
                );
            }
            for field in conflicts {
                static_snapshot.diagnostics.push(AnalysisDiagnostic {
                    severity: DiagnosticSeverity::Warning,
                    backend: "hybrid".to_string(),
                    message: format!(
                        "static and LSP providers disagree on {field} for {}; retained static value",
                        declaration_name
                    ),
                    source: Some(declaration_source.clone()),
                    declaration: Some(declaration_id.clone()),
                });
            }
        }
        static_snapshot.diagnostics.extend(lsp_snapshot.diagnostics);
    }
}

pub mod config {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use crate::config::{ConfigVal, SphinxConfig};

    use super::provider::SourceBackendMode;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum SourceSandboxMode {
        #[default]
        Off,
        TrustedLocal,
        ProtectedLsp,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum SourceBuildSandboxMode {
        #[default]
        Off,
        ProtectedBuild,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SourceDocsSettings {
        pub backend: SourceBackendMode,
        pub lsp_servers: BTreeMap<String, Vec<String>>,
        pub lsp_timeout_ms: u64,
        pub lsp_allow_fallback: bool,
        pub lsp_workspace_root: Option<PathBuf>,
        pub lsp_sandbox: SourceSandboxMode,
        pub build_sandbox: SourceBuildSandboxMode,
    }

    impl Default for SourceDocsSettings {
        fn default() -> Self {
            Self {
                backend: SourceBackendMode::Static,
                lsp_servers: BTreeMap::new(),
                lsp_timeout_ms: 30_000,
                lsp_allow_fallback: true,
                lsp_workspace_root: None,
                lsp_sandbox: SourceSandboxMode::Off,
                build_sandbox: SourceBuildSandboxMode::Off,
            }
        }
    }

    impl SourceDocsSettings {
        pub fn from_sphinx_config(config: &SphinxConfig) -> Result<Self, String> {
            let mut settings = Self::default();
            if let Some(value) = config.get("source_backend") {
                settings.backend = match value.as_str().unwrap_or("static") {
                    "static" => SourceBackendMode::Static,
                    "auto" => SourceBackendMode::Auto,
                    "lsp" => SourceBackendMode::Lsp,
                    "hybrid" => SourceBackendMode::Hybrid,
                    other => return Err(format!("invalid source_backend {other:?}")),
                };
            }
            if let Some(value) = config.get("source_lsp_servers") {
                let ConfigVal::Map(entries) = value else {
                    return Err("source_lsp_servers must be a mapping".into());
                };
                for (language, value) in entries {
                    let ConfigVal::List(args) = value else {
                        return Err(format!("source_lsp_servers[{language:?}] must be a list"));
                    };
                    let args = args
                        .iter()
                        .map(|arg| {
                            arg.as_str().map(str::to_string).ok_or_else(|| {
                                format!(
                                    "source_lsp_servers[{language:?}] arguments must be strings"
                                )
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    if args.is_empty() || args[0].is_empty() {
                        return Err(format!("source_lsp_servers[{language:?}] command is empty"));
                    }
                    settings.lsp_servers.insert(language, args);
                }
            }
            if let Some(value) = config.get("source_lsp_timeout") {
                let timeout = value
                    .as_int()
                    .and_then(|value| u64::try_from(value).ok())
                    .filter(|value| *value > 0)
                    .ok_or_else(|| "source_lsp_timeout must be a positive integer".to_string())?;
                settings.lsp_timeout_ms = timeout;
            }
            if let Some(value) = config.get("source_lsp_allow_fallback") {
                settings.lsp_allow_fallback = value
                    .as_bool()
                    .ok_or_else(|| "source_lsp_allow_fallback must be boolean".to_string())?;
            }
            if let Some(value) = config.get("source_lsp_workspace_root") {
                settings.lsp_workspace_root = match value {
                    ConfigVal::Null => None,
                    ConfigVal::Str(path) => Some(PathBuf::from(path)),
                    _ => {
                        return Err(
                            "source_lsp_workspace_root must be a path string or None".into()
                        );
                    }
                };
            }
            if let Some(value) = config.get("source_lsp_sandbox") {
                settings.lsp_sandbox = match value.as_str().unwrap_or("off") {
                    "off" => SourceSandboxMode::Off,
                    "trusted-local" => SourceSandboxMode::TrustedLocal,
                    "protected-lsp" => SourceSandboxMode::ProtectedLsp,
                    other => return Err(format!("invalid source_lsp_sandbox {other:?}")),
                };
            }
            if let Some(value) = config.get("source_build_sandbox") {
                settings.build_sandbox = match value.as_str().unwrap_or("off") {
                    "off" => SourceBuildSandboxMode::Off,
                    "protected-build" => SourceBuildSandboxMode::ProtectedBuild,
                    other => return Err(format!("invalid source_build_sandbox {other:?}")),
                };
            }
            if settings.lsp_sandbox == SourceSandboxMode::ProtectedLsp {
                return Err(
                    "protected-lsp is unavailable: no audited sandbox provider is enabled".into(),
                );
            }
            if settings.build_sandbox == SourceBuildSandboxMode::ProtectedBuild {
                return Err(
                    "protected-build is unavailable: no audited sandbox provider is enabled".into(),
                );
            }
            Ok(settings)
        }

        /// A stable cache component that does not reveal command arguments.
        pub fn lsp_configuration_identity(&self, language: &str) -> String {
            let mut value = format!(
                "backend={:?};timeout={};fallback={};root={};sandbox={:?};build_sandbox={:?}",
                self.backend,
                self.lsp_timeout_ms,
                self.lsp_allow_fallback,
                self.lsp_workspace_root
                    .as_ref()
                    .map_or_else(String::new, |path| path.to_string_lossy().into_owned()),
                self.lsp_sandbox,
                self.build_sandbox,
            );
            if let Some(command) = self.lsp_servers.get(language) {
                for arg in command {
                    for byte in arg.as_bytes() {
                        value.push(char::from(*byte));
                    }
                    value.push('\0');
                }
            }
            let hash = value
                .as_bytes()
                .iter()
                .fold(0xcbf29ce484222325u64, |hash, byte| {
                    (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
                });
            format!("source-lsp-v1-{hash:016x}")
        }

        pub fn backend_mode(&self) -> SourceBackendMode {
            self.backend
        }
    }
}

#[cfg(feature = "lsp-source-analysis")]
pub fn lsp_provider_from_settings(
    settings: &SourceDocsSettings,
    language: SourceLanguage,
    source_root: &std::path::Path,
) -> Result<Box<dyn SourceSnapshotProvider>, AnalysisError> {
    if language == SourceLanguage::Python {
        return Err(AnalysisError::InvalidRequest(
            "source LSP analysis supports Rust and Lean only".into(),
        ));
    }
    let provider =
        lsp_backend::LspSnapshotProvider::from_settings(settings, language, source_root)?;
    Ok(Box::new(provider))
}

#[cfg(feature = "lsp-source-analysis")]
#[path = "source_docs/lsp_backend.rs"]
pub mod lsp_backend;

pub mod static_backend {
    #[cfg(feature = "rust-source-analysis")]
    pub mod rust {
        pub use crate::source_analysis::rust::*;
    }

    #[cfg(feature = "lean-source-analysis")]
    pub mod lean {
        pub use crate::source_analysis::lean::*;
    }
}

pub mod domain {
    pub use crate::domains::source_domain::*;
}

pub mod autodoc {
    pub use crate::autodoc::{LeanSourceRenderer, RustSourceRenderer, SourceAutodocRenderer};
}

pub mod apidoc {
    pub use crate::apidoc::settings::SourceMode;
}

pub use config::{SourceBuildSandboxMode, SourceDocsSettings, SourceSandboxMode};
pub use model::{
    AnalysisDiagnostic, AnalysisError, AnalysisSnapshot, BackendPolicy, DeclarationKind,
    DiagnosticSeverity, SOURCE_ANALYSIS_SCHEMA_VERSION, SourceAnalysisRequest, SourceBackendKind,
    SourceDeclaration, SourceLanguage, SourcePosition, SourceProvenance, SourceSpan, Visibility,
    normalize_name, normalize_path, normalize_text, parent_name, separator_for, short_name,
    source_input_hash, stable_declaration_id,
};
pub use provider::{
    SourceAnalysisSession, SourceAnalyzer, SourceBackendMode, SourceSnapshotProvider,
};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;

    use crate::config::{ConfigVal, SphinxConfig};

    use super::provider::{SourceAnalysisSession, SourceBackendMode, SourceSnapshotProvider};
    use super::{
        AnalysisDiagnostic, AnalysisError, AnalysisSnapshot, DeclarationKind, DiagnosticSeverity,
        SourceAnalysisRequest, SourceBackendKind, SourceDeclaration, SourceLanguage,
        SourcePosition, SourceSpan, Visibility,
    };

    struct FixtureProvider {
        backend: &'static str,
        kind: SourceBackendKind,
        qualified_name: &'static str,
        source_path: &'static str,
        documentation: &'static str,
        visibility: Visibility,
        fail: bool,
    }

    impl SourceSnapshotProvider for FixtureProvider {
        fn analyze(
            &self,
            request: &SourceAnalysisRequest,
        ) -> Result<AnalysisSnapshot, AnalysisError> {
            if self.fail {
                return Err(AnalysisError::BackendUnavailable {
                    backend: self.backend.to_string(),
                    message: "fixture failure".to_string(),
                });
            }
            let declaration = SourceDeclaration::new(
                SourceLanguage::Rust,
                self.backend,
                self.qualified_name,
                "::",
                DeclarationKind::Struct,
                None,
                None,
                self.documentation,
                self.visibility.clone(),
                SourceSpan::new(
                    Path::new(self.source_path),
                    SourcePosition {
                        byte: 0,
                        line: 1,
                        column: 0,
                    },
                    None,
                ),
            );
            Ok(AnalysisSnapshot::new(
                SourceLanguage::Rust,
                self.backend,
                "test",
                &request.source_root,
                "input-hash",
                vec![declaration],
                Vec::new(),
            ))
        }

        fn backend_kind(&self) -> SourceBackendKind {
            self.kind
        }

        fn cache_identity(&self, request: &SourceAnalysisRequest) -> String {
            format!("{}:{}", self.backend, request.cache_identity())
        }

        fn source_hash(
            &self,
            request: &SourceAnalysisRequest,
            _language: SourceLanguage,
        ) -> Result<String, AnalysisError> {
            Ok(format!("fixture:{}", request.cache_identity()))
        }
    }

    fn static_provider() -> Box<dyn SourceSnapshotProvider> {
        Box::new(FixtureProvider {
            backend: "fixture-static",
            kind: SourceBackendKind::Static,
            qualified_name: "demo::Thing",
            source_path: "src/lib.rs",
            documentation: "static docs",
            visibility: Visibility::Public,
            fail: false,
        })
    }

    fn lsp_provider() -> Box<dyn SourceSnapshotProvider> {
        Box::new(FixtureProvider {
            backend: "fixture-lsp",
            kind: SourceBackendKind::Lsp,
            qualified_name: "demo::Thing",
            source_path: "src/lib.rs",
            documentation: "LSP docs",
            visibility: Visibility::Private,
            fail: false,
        })
    }

    #[test]
    fn static_mode_never_requires_or_uses_lsp_provider() {
        let session = SourceAnalysisSession::new(
            SourceBackendMode::Static,
            Some(static_provider()),
            Some(Box::new(FixtureProvider {
                backend: "fixture-lsp",
                kind: SourceBackendKind::Lsp,
                qualified_name: "demo::Thing",
                source_path: "src/lib.rs",
                documentation: "unused",
                visibility: Visibility::Public,
                fail: true,
            })),
        );
        let snapshot = session.analyze(&SourceAnalysisRequest::new("src")).unwrap();
        assert_eq!(snapshot.backend_kind, SourceBackendKind::Static);
        assert_eq!(snapshot.backend, "fixture-static");
    }

    #[test]
    fn required_lsp_mode_errors_when_provider_is_missing() {
        let session = SourceAnalysisSession::new(SourceBackendMode::Lsp, None, None);
        let error = session
            .analyze(&SourceAnalysisRequest::new("src"))
            .unwrap_err();
        assert!(
            matches!(error, AnalysisError::BackendUnavailable { backend, .. } if backend == "lsp")
        );
    }

    #[test]
    fn hybrid_enriches_text_but_retains_static_policy_metadata() {
        let session = SourceAnalysisSession::new(
            SourceBackendMode::Hybrid,
            Some(static_provider()),
            Some(lsp_provider()),
        );
        let snapshot = session.analyze(&SourceAnalysisRequest::new("src")).unwrap();
        let declaration = &snapshot.declarations[0];
        assert_eq!(snapshot.backend_kind, SourceBackendKind::Hybrid);
        assert_eq!(declaration.documentation, "static docs");
        assert_eq!(declaration.visibility, Visibility::Public);
        assert!(snapshot.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Warning
                && diagnostic.message.contains("visibility")
        }));
        let declaration_id = &snapshot.declarations[0].id;
        assert_eq!(
            snapshot
                .provenance
                .get(&format!("{declaration_id}.documentation")),
            Some(&super::model::SourceProvenance::Static)
        );
        assert_eq!(
            snapshot
                .provenance
                .get(&format!("{declaration_id}.visibility")),
            Some(&super::model::SourceProvenance::Static)
        );
    }

    #[test]
    fn hybrid_missing_lsp_provider_returns_static_snapshot_with_diagnostic() {
        let session =
            SourceAnalysisSession::new(SourceBackendMode::Hybrid, Some(static_provider()), None);
        let snapshot = session.analyze(&SourceAnalysisRequest::new("src")).unwrap();
        assert_eq!(snapshot.backend_kind, SourceBackendKind::Hybrid);
        assert_eq!(snapshot.declarations[0].documentation, "static docs");
        assert!(
            snapshot
                .diagnostics
                .iter()
                .any(|diagnostic: &AnalysisDiagnostic| {
                    diagnostic.message.contains("LSP enrichment unavailable")
                })
        );
    }

    #[test]
    fn hybrid_disabled_fallback_requires_lsp_provider() {
        let mut session =
            SourceAnalysisSession::new(SourceBackendMode::Hybrid, Some(static_provider()), None);
        session.allow_fallback = false;
        let error = session
            .analyze(&SourceAnalysisRequest::new("src"))
            .unwrap_err();
        assert!(error.to_string().contains("requires an LSP provider"));
    }

    #[test]
    fn hybrid_provider_error_falls_back_with_static_provenance() {
        let session = SourceAnalysisSession::new(
            SourceBackendMode::Hybrid,
            Some(static_provider()),
            Some(Box::new(FixtureProvider {
                backend: "fixture-lsp",
                kind: SourceBackendKind::Lsp,
                qualified_name: "demo::Thing",
                source_path: "src/lib.rs",
                documentation: "unused",
                visibility: Visibility::Public,
                fail: true,
            })),
        );
        let snapshot = session.analyze(&SourceAnalysisRequest::new("src")).unwrap();
        assert_eq!(snapshot.backend_kind, SourceBackendKind::Hybrid);
        assert_eq!(snapshot.declarations[0].documentation, "static docs");
        assert!(
            snapshot
                .provenance
                .values()
                .all(|value| { *value == super::model::SourceProvenance::Static })
        );
    }

    #[test]
    fn hybrid_unmatched_lsp_declarations_require_explicit_opt_in() {
        let session = SourceAnalysisSession::new(
            SourceBackendMode::Hybrid,
            Some(static_provider()),
            Some(Box::new(FixtureProvider {
                backend: "fixture-lsp",
                kind: SourceBackendKind::Lsp,
                qualified_name: "demo::Other",
                source_path: "src/other.rs",
                documentation: "LSP-only declaration",
                visibility: Visibility::Public,
                fail: false,
            })),
        );
        let snapshot = session.analyze(&SourceAnalysisRequest::new("src")).unwrap();
        assert_eq!(snapshot.declarations.len(), 1);

        let session = session.with_unmatched_lsp_declarations(true);
        let snapshot = session.analyze(&SourceAnalysisRequest::new("src")).unwrap();
        assert_eq!(snapshot.declarations.len(), 2);
    }

    #[test]
    fn source_docs_settings_default_to_static_and_hash_server_arguments() {
        let settings =
            super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new_defaults())
                .unwrap();
        assert_eq!(settings.backend, SourceBackendMode::Static);
        assert_eq!(settings.lsp_timeout_ms, 30_000);
        assert!(settings.lsp_allow_fallback);

        let mut raw = HashMap::new();
        raw.insert("source_backend".into(), ConfigVal::Str("hybrid".into()));
        raw.insert(
            "source_lsp_servers".into(),
            ConfigVal::Map(vec![(
                "rust".into(),
                ConfigVal::List(vec![
                    ConfigVal::Str("rust-analyzer".into()),
                    ConfigVal::Str("--stdio".into()),
                ]),
            )]),
        );
        let config = SphinxConfig::new(raw, HashMap::new());
        let settings = super::config::SourceDocsSettings::from_sphinx_config(&config).unwrap();
        assert_eq!(settings.backend, SourceBackendMode::Hybrid);
        assert_eq!(settings.lsp_servers["rust"], ["rust-analyzer", "--stdio"]);
        let identity = settings.lsp_configuration_identity("rust");
        assert!(identity.starts_with("source-lsp-v1-"));
        assert!(!identity.contains("rust-analyzer"));

        let mut changed = HashMap::new();
        changed.insert("source_backend".into(), ConfigVal::Str("hybrid".into()));
        changed.insert(
            "source_lsp_servers".into(),
            ConfigVal::Map(vec![(
                "rust".into(),
                ConfigVal::List(vec![
                    ConfigVal::Str("rust-analyzer".into()),
                    ConfigVal::Str("--other".into()),
                ]),
            )]),
        );
        let changed = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            changed,
            HashMap::new(),
        ))
        .unwrap();
        assert_ne!(identity, changed.lsp_configuration_identity("rust"));

        let session =
            SourceAnalysisSession::from_settings(&settings, "rust", Some(static_provider()), None);
        let request = SourceAnalysisRequest::new("src");
        assert!(session.cache_identity(&request).contains(&identity));
    }

    #[test]
    fn protected_source_modes_fail_closed_and_invalid_timeout_is_rejected() {
        let mut raw = HashMap::new();
        raw.insert(
            "source_lsp_sandbox".into(),
            ConfigVal::Str("protected-lsp".into()),
        );
        let error = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            raw,
            HashMap::new(),
        ))
        .unwrap_err();
        assert!(error.contains("no audited sandbox provider"));

        let mut raw = HashMap::new();
        raw.insert("source_lsp_timeout".into(), ConfigVal::Int(0));
        let error = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            raw,
            HashMap::new(),
        ))
        .unwrap_err();
        assert!(error.contains("positive integer"));
    }
}
