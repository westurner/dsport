//! Backend-neutral facade for Rust and Lean source documentation.
//!
//! Existing implementation modules remain in place during the migration;
//! consumers can depend on this facade without taking a dependency on a
//! parser-specific adapter.

pub mod model {
    pub use crate::source_analysis::{
        normalize_name, normalize_path, normalize_text, parent_name, separator_for, short_name,
        source_input_hash, stable_declaration_id, AnalysisDiagnostic, AnalysisError,
        AnalysisSnapshot, BackendPolicy, DeclarationKind, DiagnosticSeverity,
        SourceAnalysisRequest, SourceBackendKind, SourceDeclaration, SourceLanguage,
        SourcePosition, SourceProvenance, SourceSpan, Visibility, SOURCE_ANALYSIS_SCHEMA_VERSION,
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
                    snapshot.provider_identity = self.cache_identity(request);
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
            snapshot.request_identity = request.cache_identity();
            snapshot.provider_identity = provider.cache_identity(request);
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
                snapshot.provider_identity = self.cache_identity(request);
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
            snapshot.provider_identity = self.cache_identity(request);
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

        fn prepare_for_cache(&self) -> Result<(), AnalysisError> {
            match self.mode {
                SourceBackendMode::Auto | SourceBackendMode::Static => {
                    if let Some(provider) = &self.static_provider {
                        provider.prepare_for_cache()?;
                    }
                }
                SourceBackendMode::Lsp => self
                    .lsp_provider
                    .as_ref()
                    .ok_or_else(|| unavailable("lsp", "no LSP provider is configured"))?
                    .prepare_for_cache()?,
                SourceBackendMode::Hybrid => {
                    if let Some(provider) = &self.static_provider {
                        provider.prepare_for_cache()?;
                    }
                    if let Some(provider) = &self.lsp_provider {
                        provider.prepare_for_cache()?;
                    }
                }
            }
            Ok(())
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
                let key = key.strip_prefix("lsp:").unwrap_or(&key);
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum SourceSandboxEngine {
        #[default]
        Mxc,
        AiSandbox,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum MxcLspContainment {
        #[default]
        Process,
        Bubblewrap,
        Seatbelt,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum MxcNetworkAction {
        #[default]
        Deny,
        Allow,
    }

    /// Safe MXC policy defaults with a narrow, explicit `conf.py` override surface.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MxcLspConfig {
        pub containment: MxcLspContainment,
        pub mount_procfs: bool,
        pub readonly_paths: Vec<PathBuf>,
        pub readwrite_paths: Vec<PathBuf>,
        pub denied_paths: Vec<PathBuf>,
        pub egress: MxcNetworkAction,
        pub ingress: MxcNetworkAction,
        pub host_loopback: MxcNetworkAction,
        pub timeout_ms: u32,
    }

    impl Default for MxcLspConfig {
        fn default() -> Self {
            Self {
                containment: MxcLspContainment::Process,
                mount_procfs: false,
                readonly_paths: Vec::new(),
                readwrite_paths: Vec::new(),
                denied_paths: Vec::new(),
                egress: MxcNetworkAction::Deny,
                ingress: MxcNetworkAction::Deny,
                host_loopback: MxcNetworkAction::Deny,
                timeout_ms: 30 * 60 * 1000,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SourceDocsSettings {
        pub backend: SourceBackendMode,
        pub lsp_servers: BTreeMap<String, Vec<String>>,
        pub lsp_server_environment: BTreeMap<String, BTreeMap<String, String>>,
        pub lsp_timeout_ms: u64,
        pub lsp_allow_fallback: bool,
        pub lsp_workspace_root: Option<PathBuf>,
        pub lsp_read_only_roots: Vec<PathBuf>,
        pub lsp_server_read_only_roots: BTreeMap<String, Vec<PathBuf>>,
        pub lsp_sandbox_environment: BTreeMap<String, String>,
        pub lsp_sandbox_engine: SourceSandboxEngine,
        pub lsp_mxc_config: MxcLspConfig,
        pub lsp_sandbox: SourceSandboxMode,
        pub build_sandbox: SourceBuildSandboxMode,
    }

    impl Default for SourceDocsSettings {
        fn default() -> Self {
            Self {
                backend: SourceBackendMode::Static,
                lsp_servers: BTreeMap::new(),
                lsp_server_environment: BTreeMap::new(),
                lsp_timeout_ms: 30_000,
                lsp_allow_fallback: true,
                lsp_workspace_root: None,
                lsp_read_only_roots: Vec::new(),
                lsp_server_read_only_roots: BTreeMap::new(),
                lsp_sandbox_environment: BTreeMap::new(),
                lsp_sandbox_engine: SourceSandboxEngine::Mxc,
                lsp_mxc_config: MxcLspConfig::default(),
                lsp_sandbox: SourceSandboxMode::Off,
                build_sandbox: SourceBuildSandboxMode::Off,
            }
        }
    }

    impl SourceDocsSettings {
        pub fn lsp_server_environment_for(
            &self,
            language: &str,
        ) -> Result<Vec<(String, String)>, String> {
            let environment = self
                .lsp_server_environment
                .get(language)
                .cloned()
                .unwrap_or_default();
            for (key, value) in &environment {
                validate_lsp_environment_entry(
                    key,
                    value,
                    &format!("source_lsp_server_environment[{language:?}]"),
                )?;
            }
            Ok(environment.into_iter().collect())
        }

        pub fn lsp_sandbox_environment_entries(&self) -> Result<Vec<(String, String)>, String> {
            for (key, value) in &self.lsp_sandbox_environment {
                validate_lsp_environment_entry(key, value, "source_lsp_sandbox_environment")?;
            }
            Ok(self
                .lsp_sandbox_environment
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect())
        }

        pub fn lsp_read_only_roots_for(&self, language: &str) -> Result<Vec<PathBuf>, String> {
            let mut roots = self.lsp_read_only_roots.clone();
            roots.extend(self.lsp_mxc_config.readonly_paths.iter().cloned());
            if let Some(server_roots) = self.lsp_server_read_only_roots.get(language) {
                roots.extend(server_roots.iter().cloned());
            }
            for root in &roots {
                validate_lsp_read_only_root(root, "LSP read-only roots")?;
            }
            roots.sort();
            roots.dedup();
            Ok(roots)
        }

        pub fn effective_lsp_sandbox_mode(&self) -> SourceSandboxMode {
            self.effective_lsp_sandbox_mode_with(sandboxing_disabled_by_env())
        }

        pub(super) fn effective_lsp_sandbox_mode_with(
            &self,
            sandboxing_disabled: bool,
        ) -> SourceSandboxMode {
            if sandboxing_disabled && self.lsp_sandbox == SourceSandboxMode::ProtectedLsp {
                SourceSandboxMode::TrustedLocal
            } else {
                self.lsp_sandbox
            }
        }

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
            if let Some(value) = config.get("source_lsp_server_environment") {
                let ConfigVal::Map(languages) = value else {
                    return Err("source_lsp_server_environment must be a mapping".into());
                };
                for (language, value) in languages {
                    let ConfigVal::Map(entries) = value else {
                        return Err(format!(
                            "source_lsp_server_environment[{language:?}] must be a mapping"
                        ));
                    };
                    let mut environment = BTreeMap::new();
                    for (key, value) in entries {
                        let value = value.as_str().ok_or_else(|| {
                            format!(
                                "source_lsp_server_environment[{language:?}][{key:?}] must be a string"
                            )
                        })?;
                        validate_lsp_environment_entry(
                            &key,
                            value,
                            &format!("source_lsp_server_environment[{language:?}]"),
                        )?;
                        environment.insert(key, value.to_string());
                    }
                    settings
                        .lsp_server_environment
                        .insert(language, environment);
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
            if let Some(value) = config.get("source_lsp_read_only_roots") {
                let ConfigVal::List(roots) = value else {
                    return Err(
                        "source_lsp_read_only_roots must be a list of absolute paths".into(),
                    );
                };
                settings.lsp_read_only_roots =
                    parse_lsp_read_only_roots(&roots, "source_lsp_read_only_roots")?;
            }
            if let Some(value) = config.get("source_lsp_server_read_only_roots") {
                let ConfigVal::Map(languages) = value else {
                    return Err("source_lsp_server_read_only_roots must be a mapping".into());
                };
                for (language, value) in languages {
                    let ConfigVal::List(roots) = value else {
                        return Err(format!(
                            "source_lsp_server_read_only_roots[{language:?}] must be a list"
                        ));
                    };
                    settings.lsp_server_read_only_roots.insert(
                        language.clone(),
                        parse_lsp_read_only_roots(
                            &roots,
                            &format!("source_lsp_server_read_only_roots[{language:?}]"),
                        )?,
                    );
                }
            }
            if let Some(value) = config.get("source_lsp_sandbox_environment") {
                let ConfigVal::Map(entries) = value else {
                    return Err("source_lsp_sandbox_environment must be a mapping".into());
                };
                for (key, value) in entries {
                    let value = value.as_str().ok_or_else(|| {
                        format!("source_lsp_sandbox_environment[{key:?}] must be a string")
                    })?;
                    validate_lsp_environment_entry(&key, value, "source_lsp_sandbox_environment")?;
                    settings
                        .lsp_sandbox_environment
                        .insert(key, value.to_string());
                }
            }
            if let Some(value) = config.get("source_lsp_mxc_config") {
                settings.lsp_mxc_config = parse_mxc_lsp_config(value)?;
            }
            if let Some(value) = config.get("source_lsp_sandbox_engine") {
                settings.lsp_sandbox_engine = match value.as_str().unwrap_or("mxc") {
                    "mxc" => SourceSandboxEngine::Mxc,
                    "ai-sandbox" => SourceSandboxEngine::AiSandbox,
                    other => {
                        return Err(format!(
                            "source_lsp_sandbox_engine must be 'mxc' or 'ai-sandbox', got {other:?}"
                        ));
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
            let sandboxing_disabled = sandboxing_disabled_by_env();
            if settings.lsp_sandbox == SourceSandboxMode::ProtectedLsp && !sandboxing_disabled {
                if !cfg!(feature = "source-sandbox") {
                    return Err(
                        "protected-lsp requires the optional source-sandbox feature (MXC)".into(),
                    );
                }
                if !cfg!(any(target_os = "linux", target_os = "macos")) {
                    return Err("protected-lsp with MXC is unsupported on this platform".into());
                }
                if settings.lsp_read_only_roots.is_empty()
                    && settings.lsp_mxc_config.readonly_paths.is_empty()
                    && settings
                        .lsp_server_read_only_roots
                        .values()
                        .all(Vec::is_empty)
                {
                    return Err(
                        "protected-lsp requires source_lsp_read_only_roots for toolchain access"
                            .into(),
                    );
                }
            }
            if settings.build_sandbox == SourceBuildSandboxMode::ProtectedBuild
                && !sandboxing_disabled
            {
                return Err(
                    "protected-build is unavailable: no audited MXC build provider is enabled"
                        .into(),
                );
            }
            Ok(settings)
        }

        /// A stable cache component that does not reveal command arguments.
        pub fn lsp_configuration_identity(&self, language: &str) -> String {
            let procfs_mode = if self.effective_lsp_sandbox_mode()
                == SourceSandboxMode::ProtectedLsp
                && cfg!(target_os = "linux")
            {
                if self.lsp_mxc_config.mount_procfs {
                    "mounted"
                } else {
                    "omitted"
                }
            } else {
                "not-applicable"
            };
            let mut value = format!(
                "backend={:?};timeout={};fallback={};root={};sandbox={:?};sandbox_engine={:?};build_sandbox={:?};procfs={};sandbox_off={};mxc_containment={:?};mxc_timeout={};mxc_egress={:?};mxc_ingress={:?};mxc_loopback={:?};",
                self.backend,
                self.lsp_timeout_ms,
                self.lsp_allow_fallback,
                self.lsp_workspace_root
                    .as_ref()
                    .map_or_else(String::new, |path| path.to_string_lossy().into_owned()),
                self.effective_lsp_sandbox_mode(),
                self.lsp_sandbox_engine,
                self.build_sandbox,
                procfs_mode,
                sandboxing_disabled_by_env(),
                self.lsp_mxc_config.containment,
                self.lsp_mxc_config.timeout_ms,
                self.lsp_mxc_config.egress,
                self.lsp_mxc_config.ingress,
                self.lsp_mxc_config.host_loopback,
            );
            for root in &self.lsp_read_only_roots {
                value.push_str(&format!("ro-root={};", root.to_string_lossy()));
            }
            for root in &self.lsp_mxc_config.readonly_paths {
                value.push_str(&format!("mxc-ro-root={};", root.to_string_lossy()));
            }
            for root in &self.lsp_mxc_config.readwrite_paths {
                value.push_str(&format!("mxc-rw-root={};", root.to_string_lossy()));
            }
            for root in &self.lsp_mxc_config.denied_paths {
                value.push_str(&format!("mxc-denied-root={};", root.to_string_lossy()));
            }
            if let Some(roots) = self.lsp_server_read_only_roots.get(language) {
                for root in roots {
                    value.push_str(&format!("server-ro-root={};", root.to_string_lossy()));
                }
            }
            for (key, environment_value) in &self.lsp_sandbox_environment {
                value.push_str(&format!("sandbox-env={key}={environment_value};"));
            }
            if let Some(environment) = self.lsp_server_environment.get(language) {
                for (key, environment_value) in environment {
                    value.push_str(&format!("server-env={key}={environment_value};"));
                }
            }
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

    fn parse_lsp_read_only_roots(
        roots: &[ConfigVal],
        setting: &str,
    ) -> Result<Vec<PathBuf>, String> {
        roots
            .iter()
            .map(|root| {
                let Some(root) = root.as_str() else {
                    return Err(format!("{setting} entries must be path strings"));
                };
                let root = PathBuf::from(root);
                validate_lsp_read_only_root(&root, setting)?;
                Ok(root)
            })
            .collect()
    }

    pub fn sandboxing_disabled_by_env() -> bool {
        sandboxing_disabled_from_value(std::env::var("SPHINXDOCRS_SANDBOXING").ok().as_deref())
    }

    pub(super) fn sandboxing_disabled_from_value(value: Option<&str>) -> bool {
        value.is_some_and(|value| value.trim().eq_ignore_ascii_case("off"))
    }

    pub fn sandboxing_config_summary(config: &SphinxConfig) -> String {
        sandboxing_config_summary_for(config, sandboxing_disabled_by_env())
    }

    pub fn sandboxing_requested(config: &SphinxConfig) -> bool {
        sandboxing_disabled_by_env()
            || config
                .get("source_lsp_sandbox")
                .and_then(|value| value.as_str().map(str::to_owned))
                .is_some_and(|mode| mode != "off")
            || config
                .get("source_build_sandbox")
                .and_then(|value| value.as_str().map(str::to_owned))
                .is_some_and(|mode| mode != "off")
    }

    fn configured_path_count(value: Option<ConfigVal>) -> usize {
        match value {
            Some(ConfigVal::List(paths)) => paths.len(),
            Some(ConfigVal::Map(languages)) => languages
                .iter()
                .map(|(_, roots)| match roots {
                    ConfigVal::List(paths) => paths.len(),
                    _ => 0,
                })
                .sum(),
            _ => 0,
        }
    }

    pub(super) fn sandboxing_config_summary_for(
        config: &SphinxConfig,
        sandboxing_disabled: bool,
    ) -> String {
        let lsp_mode = config
            .get("source_lsp_sandbox")
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_else(|| "off".to_owned());
        let sandbox_engine = config
            .get("source_lsp_sandbox_engine")
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_else(|| "mxc".to_owned());
        let build_mode = config
            .get("source_build_sandbox")
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_else(|| "off".to_owned());
        if sandboxing_disabled {
            return format!(
                "sphinxdocrs: sandboxing=disabled override=SPHINXDOCRS_SANDBOXING=off configured_lsp={lsp_mode} configured_engine={sandbox_engine} configured_build={build_mode}"
            );
        }
        let mxc_config = config
            .get("source_lsp_mxc_config")
            .map(parse_mxc_lsp_config)
            .unwrap_or_else(|| Ok(MxcLspConfig::default()));
        let shared_read_only_roots =
            configured_path_count(config.get("source_lsp_read_only_roots"));
        let server_read_only_roots =
            configured_path_count(config.get("source_lsp_server_read_only_roots"));
        match (lsp_mode.as_str(), mxc_config) {
            ("protected-lsp", Ok(mxc)) => format!(
                "sphinxdocrs: sandboxing=protected-lsp engine={sandbox_engine} containment={:?} procfs={} filesystem=deny-by-default read-only-roots={} read-write-roots={} network=egress:{:?},ingress:{:?},host-loopback:{:?} build={build_mode}",
                mxc.containment,
                if mxc.mount_procfs {
                    "mounted"
                } else {
                    "omitted"
                },
                mxc.readonly_paths.len() + shared_read_only_roots + server_read_only_roots,
                mxc.readwrite_paths.len(),
                mxc.egress,
                mxc.ingress,
                mxc.host_loopback,
            ),
            ("protected-lsp", Err(error)) => {
                format!(
                    "sphinxdocrs: sandboxing=protected-lsp engine={sandbox_engine} config=invalid ({error})"
                )
            }
            ("trusted-local", _) => format!(
                "sphinxdocrs: sandboxing=trusted-local (LSP has no OS boundary) build={build_mode}"
            ),
            ("off", _) => format!(
                "sphinxdocrs: sandboxing=off (no protected LSP configured) build={build_mode}"
            ),
            (other, _) => format!("sphinxdocrs: sandboxing=invalid source_lsp_sandbox={other:?}"),
        }
    }

    fn parse_mxc_lsp_config(value: ConfigVal) -> Result<MxcLspConfig, String> {
        let ConfigVal::Map(entries) = value else {
            return Err("source_lsp_mxc_config must be a mapping".into());
        };
        let mut settings = MxcLspConfig::default();
        for (key, value) in entries {
            match key.as_str() {
                "containment" => {
                    settings.containment = match value.as_str().unwrap_or_default() {
                        "process" => MxcLspContainment::Process,
                        "bubblewrap" => MxcLspContainment::Bubblewrap,
                        "seatbelt" => MxcLspContainment::Seatbelt,
                        other => {
                            return Err(format!(
                                "source_lsp_mxc_config.containment must be process, bubblewrap, or seatbelt; got {other:?}"
                            ));
                        }
                    };
                }
                "filesystem" => {
                    let ConfigVal::Map(filesystem) = value else {
                        return Err("source_lsp_mxc_config.filesystem must be a mapping".into());
                    };
                    for (field, value) in filesystem {
                        if field == "mount_procfs" {
                            settings.mount_procfs = value.as_bool().ok_or_else(|| {
                                "source_lsp_mxc_config.filesystem.mount_procfs must be boolean"
                                    .to_string()
                            })?;
                            continue;
                        }
                        let paths = parse_mxc_paths(
                            value,
                            &format!("source_lsp_mxc_config.filesystem.{field}"),
                        )?;
                        match field.as_str() {
                            "readonly_paths" => settings.readonly_paths = paths,
                            "readwrite_paths" => settings.readwrite_paths = paths,
                            "denied_paths" => settings.denied_paths = paths,
                            _ => {
                                return Err(format!(
                                    "unsupported source_lsp_mxc_config.filesystem field {field:?}"
                                ));
                            }
                        }
                    }
                }
                "network" => {
                    let ConfigVal::Map(network) = value else {
                        return Err("source_lsp_mxc_config.network must be a mapping".into());
                    };
                    for (direction, value) in network {
                        let ConfigVal::Map(policy) = value else {
                            return Err(format!(
                                "source_lsp_mxc_config.network.{direction} must be a mapping"
                            ));
                        };
                        for (field, action) in policy {
                            let action = parse_mxc_network_action(
                                &action,
                                &format!("source_lsp_mxc_config.network.{direction}.{field}"),
                            )?;
                            match (direction.as_str(), field.as_str()) {
                                ("egress", "default") => settings.egress = action,
                                ("ingress", "default") => settings.ingress = action,
                                ("ingress", "host_loopback") => settings.host_loopback = action,
                                _ => {
                                    return Err(format!(
                                        "unsupported source_lsp_mxc_config.network field {direction}.{field}"
                                    ));
                                }
                            }
                        }
                    }
                }
                "timeout_ms" => {
                    settings.timeout_ms = value
                        .as_int()
                        .and_then(|value| u32::try_from(value).ok())
                        .filter(|value| *value > 0 && *value <= 30 * 60 * 1000)
                        .ok_or_else(|| {
                            "source_lsp_mxc_config.timeout_ms must be between 1 and 1800000"
                                .to_string()
                        })?;
                }
                _ => {
                    return Err(format!("unsupported source_lsp_mxc_config field {key:?}"));
                }
            }
        }
        Ok(settings)
    }

    fn parse_mxc_paths(value: ConfigVal, setting: &str) -> Result<Vec<PathBuf>, String> {
        let ConfigVal::List(paths) = value else {
            return Err(format!("{setting} must be a list of absolute paths"));
        };
        paths
            .iter()
            .map(|path| {
                let Some(path) = path.as_str() else {
                    return Err(format!("{setting} entries must be path strings"));
                };
                let path = PathBuf::from(path);
                validate_lsp_read_only_root(&path, setting)?;
                Ok(path)
            })
            .collect()
    }

    fn parse_mxc_network_action(
        value: &ConfigVal,
        setting: &str,
    ) -> Result<MxcNetworkAction, String> {
        match value.as_str().unwrap_or_default() {
            "deny" => Ok(MxcNetworkAction::Deny),
            "allow" => Ok(MxcNetworkAction::Allow),
            other => Err(format!(
                "{setting} must be 'deny' or 'allow', got {other:?}"
            )),
        }
    }

    fn validate_lsp_read_only_root(root: &std::path::Path, setting: &str) -> Result<(), String> {
        if !root.is_absolute()
            || root == std::path::Path::new("/")
            || root
                .components()
                .any(|component| component == std::path::Component::ParentDir)
        {
            return Err(format!(
                "{setting} entries must be absolute non-root paths without '..'"
            ));
        }
        if ["/dev", "/proc"]
            .iter()
            .any(|reserved| root == std::path::Path::new(reserved) || root.starts_with(reserved))
            || ["/tmp", "/home", "/root"]
                .iter()
                .any(|reserved| root == std::path::Path::new(reserved))
        {
            return Err(format!(
                "{setting} entries must not replace protected sandbox mount points"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_lsp_environment_entry(
        key: &str,
        value: &str,
        setting: &str,
    ) -> Result<(), String> {
        let mut characters = key.chars();
        let valid_first = characters
            .next()
            .is_some_and(|character| character == '_' || character.is_ascii_alphabetic());
        if !valid_first
            || !characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
            || value.contains('\0')
        {
            return Err(format!(
                "{setting}[{key:?}] has an invalid key or NUL value"
            ));
        }
        if key.starts_with("LD_")
            || key.starts_with("DYLD_")
            || matches!(
                key,
                "BASH_ENV"
                    | "ENV"
                    | "GCONV_PATH"
                    | "LOCPATH"
                    | "NODE_OPTIONS"
                    | "NLSPATH"
                    | "PERL5OPT"
                    | "PYTHONHOME"
                    | "PYTHONINSPECT"
                    | "PYTHONPATH"
                    | "RUBYOPT"
            )
        {
            return Err(format!(
                "{setting}[{key:?}] is blocked because it can inject code or alter dynamic loading"
            ));
        }
        if key == "PATH" {
            let paths = std::env::split_paths(std::ffi::OsStr::new(value)).collect::<Vec<_>>();
            if paths.is_empty()
                || paths
                    .iter()
                    .any(|path| path.as_os_str().is_empty() || !path.is_absolute())
            {
                return Err(format!(
                    "{setting}[\"PATH\"] must contain only non-empty absolute paths"
                ));
            }
        }
        Ok(())
    }
}

#[cfg(feature = "lsp-source-analysis")]
pub fn lsp_provider_from_settings(
    settings: &SourceDocsSettings,
    language: SourceLanguage,
    source_root: &std::path::Path,
) -> Result<Box<dyn SourceSnapshotProvider>, AnalysisError> {
    let provider =
        lsp_backend::LspSnapshotProvider::from_settings(settings, language, source_root)?;
    Ok(Box::new(provider))
}

#[cfg(feature = "lsp-source-analysis")]
#[path = "source_docs/lsp_backend.rs"]
pub mod lsp_backend;

#[cfg(feature = "lsp-source-analysis")]
#[path = "source_docs/embedded_lsp.rs"]
pub mod embedded_lsp;

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
    normalize_name, normalize_path, normalize_text, parent_name, separator_for, short_name,
    source_input_hash, stable_declaration_id, AnalysisDiagnostic, AnalysisError, AnalysisSnapshot,
    BackendPolicy, DeclarationKind, DiagnosticSeverity, SourceAnalysisRequest, SourceBackendKind,
    SourceDeclaration, SourceLanguage, SourcePosition, SourceProvenance, SourceSpan, Visibility,
    SOURCE_ANALYSIS_SCHEMA_VERSION,
};
pub use provider::{
    SourceAnalysisSession, SourceAnalyzer, SourceBackendMode, SourceSnapshotProvider,
};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;

    use crate::autodoc::{
        document_source, render_source_rst, RustSourceRenderer, SourceAutodocRequest,
    };
    use crate::config::{ConfigVal, SphinxConfig};
    use crate::domains::{Domain, RustDomain};
    use crate::environment::{BuildEnvironment, EnvProject};
    use crate::search::SearchIndex;

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

    struct ContractProvider {
        backend: &'static str,
        kind: SourceBackendKind,
    }

    impl SourceSnapshotProvider for ContractProvider {
        fn analyze(
            &self,
            request: &SourceAnalysisRequest,
        ) -> Result<AnalysisSnapshot, AnalysisError> {
            let make_declaration = |name: &str, kind, byte| {
                SourceDeclaration::new(
                    SourceLanguage::Rust,
                    self.backend,
                    name,
                    "::",
                    kind,
                    None,
                    None,
                    "fixture docs",
                    if self.kind == SourceBackendKind::Lsp {
                        Visibility::Unknown
                    } else {
                        Visibility::Public
                    },
                    SourceSpan::new(
                        "src/lib.rs",
                        SourcePosition {
                            byte,
                            line: 1,
                            column: byte,
                        },
                        None,
                    ),
                )
            };
            let mut widget = make_declaration("demo::Widget", DeclarationKind::Struct, 5);
            widget.aliases = vec![
                "demo::Zed".into(),
                "demo::Alias".into(),
                "demo::Alias".into(),
            ];
            widget.children = vec![
                "demo::Widget::delete".into(),
                "demo::Widget::create".into(),
                "demo::Widget::create".into(),
            ];
            widget.deprecated = self.kind != SourceBackendKind::Lsp;
            widget.noindex = self.kind != SourceBackendKind::Lsp;
            if self.kind == SourceBackendKind::Lsp {
                widget
                    .attributes
                    .insert("lsp:definition_path".into(), "src/implementation.rs".into());
            }
            let create = make_declaration("demo::Widget::create", DeclarationKind::Method, 10);
            let delete = make_declaration("demo::Widget::delete", DeclarationKind::Method, 15);
            let alpha = make_declaration("demo::Alpha", DeclarationKind::Function, 0);

            let mut snapshot = AnalysisSnapshot::new(
                SourceLanguage::Rust,
                self.backend,
                "contract-fixture-v1",
                &request.source_root,
                "contract-source-hash",
                Vec::new(),
                Vec::new(),
            );
            snapshot.declarations = vec![widget, delete, alpha, create];
            snapshot.diagnostics = vec![
                AnalysisDiagnostic {
                    severity: DiagnosticSeverity::Warning,
                    backend: self.backend.into(),
                    message: "zeta contract diagnostic".into(),
                    source: None,
                    declaration: None,
                },
                AnalysisDiagnostic {
                    severity: DiagnosticSeverity::Info,
                    backend: self.backend.into(),
                    message: "alpha contract diagnostic".into(),
                    source: None,
                    declaration: None,
                },
            ];
            Ok(snapshot)
        }

        fn backend_kind(&self) -> SourceBackendKind {
            self.kind
        }

        fn cache_identity(&self, request: &SourceAnalysisRequest) -> String {
            format!("{}:{}", self.backend, request.cache_identity())
        }

        fn source_hash(
            &self,
            _request: &SourceAnalysisRequest,
            _language: SourceLanguage,
        ) -> Result<String, AnalysisError> {
            Ok("contract-source-hash".into())
        }
    }

    fn contract_provider(
        backend: &'static str,
        kind: SourceBackendKind,
    ) -> Box<dyn SourceSnapshotProvider> {
        Box::new(ContractProvider { backend, kind })
    }

    fn assert_provider_contract(
        provider: &dyn SourceSnapshotProvider,
        request: &SourceAnalysisRequest,
    ) {
        let snapshot = provider.analyze(request).unwrap();
        let repeated = provider.analyze(request).unwrap();
        assert_eq!(snapshot.declarations, repeated.declarations);
        assert_eq!(snapshot.diagnostics, repeated.diagnostics);
        assert_eq!(snapshot.provenance, repeated.provenance);
        assert_eq!(snapshot.backend_kind, provider.backend_kind());
        assert_eq!(snapshot.request_identity, request.cache_identity());
        assert_eq!(snapshot.provider_identity, provider.cache_identity(request));

        let names = snapshot
            .declarations
            .iter()
            .map(|declaration| declaration.qualified_name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                "demo::Alpha",
                "demo::Widget",
                "demo::Widget::create",
                "demo::Widget::delete",
            ]
        );
        for declaration in &snapshot.declarations {
            assert_eq!(
                declaration.id,
                super::model::stable_declaration_id(
                    declaration.language,
                    &declaration.qualified_name,
                    &declaration.kind,
                )
            );
            assert_eq!(declaration.source.path, "src/lib.rs");
            assert!(declaration.source.start.line > 0);
            assert!(declaration.aliases.windows(2).all(|pair| pair[0] < pair[1]));
            assert!(declaration
                .children
                .windows(2)
                .all(|pair| pair[0] < pair[1]));
        }
        let widget = snapshot
            .declarations
            .iter()
            .find(|declaration| declaration.qualified_name == "demo::Widget")
            .unwrap();
        assert_eq!(widget.parent.as_deref(), Some("demo"));
        assert_eq!(widget.aliases, ["demo::Alias", "demo::Zed"]);
        assert_eq!(
            widget.children,
            ["demo::Widget::create", "demo::Widget::delete"]
        );
        match snapshot.backend_kind {
            SourceBackendKind::Static | SourceBackendKind::Hybrid => {
                assert_eq!(widget.visibility, Visibility::Public);
                assert!(widget.deprecated);
                assert!(widget.noindex);
            }
            SourceBackendKind::Lsp => {
                assert_eq!(widget.visibility, Visibility::Unknown);
                assert!(!widget.deprecated);
                assert!(!widget.noindex);
            }
        }
        let mut sorted_diagnostics = snapshot.diagnostics.clone();
        sorted_diagnostics.sort_by(|left, right| {
            left.backend
                .cmp(&right.backend)
                .then(left.message.cmp(&right.message))
        });
        assert_eq!(snapshot.diagnostics, sorted_diagnostics);
        assert!(snapshot
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.message == "alpha contract diagnostic" }));
        assert!(snapshot
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.message == "zeta contract diagnostic" }));

        let encoded = serde_json::to_vec(&snapshot).unwrap();
        let decoded: AnalysisSnapshot = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded.declarations, snapshot.declarations);
        assert_eq!(decoded.diagnostics, snapshot.diagnostics);
        assert_eq!(decoded.request_identity, snapshot.request_identity);
        assert_eq!(decoded.provider_identity, snapshot.provider_identity);
        assert_eq!(decoded.backend_kind, snapshot.backend_kind);
    }

    #[test]
    fn shared_provider_contract_holds_for_static_lsp_and_hybrid_sessions() {
        let request = SourceAnalysisRequest::new("src");
        let static_session = SourceAnalysisSession::new(
            SourceBackendMode::Static,
            Some(contract_provider(
                "contract-static",
                SourceBackendKind::Static,
            )),
            None,
        );
        let lsp_session = SourceAnalysisSession::new(
            SourceBackendMode::Lsp,
            None,
            Some(contract_provider("contract-lsp", SourceBackendKind::Lsp)),
        );
        let hybrid_session = SourceAnalysisSession::new(
            SourceBackendMode::Hybrid,
            Some(contract_provider(
                "contract-static",
                SourceBackendKind::Static,
            )),
            Some(contract_provider("contract-lsp", SourceBackendKind::Lsp)),
        );

        assert_provider_contract(&static_session, &request);
        assert_provider_contract(&lsp_session, &request);
        assert_provider_contract(&hybrid_session, &request);
    }

    #[test]
    fn static_and_hybrid_consumers_match_when_lsp_adds_no_declarations() {
        let request = SourceAnalysisRequest::new("src");
        let static_session = SourceAnalysisSession::new(
            SourceBackendMode::Static,
            Some(contract_provider(
                "contract-static",
                SourceBackendKind::Static,
            )),
            None,
        );
        let hybrid_session = SourceAnalysisSession::new(
            SourceBackendMode::Hybrid,
            Some(contract_provider(
                "contract-static",
                SourceBackendKind::Static,
            )),
            Some(contract_provider("contract-lsp", SourceBackendKind::Lsp)),
        );
        let static_snapshot = static_session.analyze(&request).unwrap();
        let hybrid_snapshot = hybrid_session.analyze(&request).unwrap();

        let mut static_autodoc_request = SourceAutodocRequest::new("src", SourceLanguage::Rust);
        static_autodoc_request.analysis = request.clone();
        let hybrid_autodoc_request = static_autodoc_request.clone();
        let static_descriptions = document_source(
            &static_autodoc_request,
            &static_session,
            &RustSourceRenderer,
        )
        .unwrap();
        let hybrid_descriptions = document_source(
            &hybrid_autodoc_request,
            &hybrid_session,
            &RustSourceRenderer,
        )
        .unwrap();
        assert_eq!(static_descriptions, hybrid_descriptions);
        assert_eq!(
            render_source_rst(&static_descriptions),
            render_source_rst(&hybrid_descriptions)
        );

        let hybrid_widget = hybrid_snapshot
            .declarations
            .iter()
            .find(|declaration| declaration.qualified_name == "demo::Widget")
            .unwrap();
        assert_eq!(
            hybrid_widget
                .attributes
                .get("lsp:definition_path")
                .map(String::as_str),
            Some("src/implementation.rs")
        );
        assert!(!hybrid_widget
            .attributes
            .contains_key("lsp:lsp:definition_path"));

        let mut static_domain = RustDomain::new();
        static_domain.note_snapshot("api", &static_snapshot.declarations);
        let mut hybrid_domain = RustDomain::new();
        hybrid_domain.note_snapshot("api", &hybrid_snapshot.declarations);
        let static_objects = static_domain.source_objects();
        let hybrid_objects = hybrid_domain.source_objects();
        assert_eq!(static_objects, hybrid_objects);
        assert!(!static_objects.is_empty());

        let mut static_search = SearchIndex::new();
        static_search.set_source_objects(static_objects);
        let mut hybrid_search = SearchIndex::new();
        hybrid_search.set_source_objects(hybrid_objects);
        assert_eq!(static_search.to_json(), hybrid_search.to_json());

        let workspace = tempfile::tempdir().unwrap();
        let environment = BuildEnvironment::new(
            SphinxConfig::new_defaults(),
            EnvProject::new(workspace.path(), &[(".rst", "restructuredtext")]),
            workspace.path(),
            workspace.path().join("doctrees"),
        );
        let static_target = static_domain
            .resolve_xref(&environment, "index", "meth", "demo::Widget::create")
            .unwrap();
        let hybrid_target = hybrid_domain
            .resolve_xref(&environment, "index", "meth", "demo::Widget::create")
            .unwrap();
        assert_eq!(static_target, hybrid_target);
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
        assert!(snapshot
            .diagnostics
            .iter()
            .any(|diagnostic: &AnalysisDiagnostic| {
                diagnostic.message.contains("LSP enrichment unavailable")
            }));
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
        assert!(snapshot
            .provenance
            .values()
            .all(|value| { *value == super::model::SourceProvenance::Static }));
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
    fn per_server_lsp_environment_and_roots_are_scoped_and_hashed() {
        let mut raw = HashMap::new();
        raw.insert(
            "source_lsp_servers".into(),
            ConfigVal::Map(vec![
                (
                    "lean".into(),
                    ConfigVal::List(vec![ConfigVal::Str("lake".into())]),
                ),
                (
                    "rust".into(),
                    ConfigVal::List(vec![ConfigVal::Str("rust-analyzer".into())]),
                ),
            ]),
        );
        raw.insert(
            "source_lsp_server_environment".into(),
            ConfigVal::Map(vec![(
                "lean".into(),
                ConfigVal::Map(vec![
                    ("ELAN_HOME".into(), ConfigVal::Str("/opt/elan".into())),
                    (
                        "LEAN_PATH".into(),
                        ConfigVal::Str("/workspace/.lake/build/lib".into()),
                    ),
                ]),
            )]),
        );
        raw.insert(
            "source_lsp_read_only_roots".into(),
            ConfigVal::List(vec![ConfigVal::Str("/opt/shared".into())]),
        );
        raw.insert(
            "source_lsp_server_read_only_roots".into(),
            ConfigVal::Map(vec![
                (
                    "lean".into(),
                    ConfigVal::List(vec![ConfigVal::Str("/opt/elan".into())]),
                ),
                (
                    "rust".into(),
                    ConfigVal::List(vec![ConfigVal::Str("/opt/rust".into())]),
                ),
            ]),
        );
        let settings = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            raw,
            HashMap::new(),
        ))
        .unwrap();

        assert_eq!(
            settings.lsp_server_environment_for("lean").unwrap(),
            [
                ("ELAN_HOME".into(), "/opt/elan".into()),
                ("LEAN_PATH".into(), "/workspace/.lake/build/lib".into()),
            ]
        );
        assert_eq!(
            settings.lsp_read_only_roots_for("lean").unwrap(),
            [Path::new("/opt/elan"), Path::new("/opt/shared")]
        );
        assert_eq!(
            settings.lsp_read_only_roots_for("rust").unwrap(),
            [Path::new("/opt/rust"), Path::new("/opt/shared")]
        );

        let lean_identity = settings.lsp_configuration_identity("lean");
        let rust_identity = settings.lsp_configuration_identity("rust");
        let mut changed = settings.clone();
        changed
            .lsp_server_environment
            .get_mut("lean")
            .unwrap()
            .insert("LEAN_PATH".into(), "/different/lib".into());
        assert_ne!(lean_identity, changed.lsp_configuration_identity("lean"));
        assert_eq!(rust_identity, changed.lsp_configuration_identity("rust"));
        changed
            .lsp_server_read_only_roots
            .get_mut("lean")
            .unwrap()
            .push(Path::new("/opt/lean-extra").to_path_buf());
        assert_ne!(lean_identity, changed.lsp_configuration_identity("lean"));
    }

    #[test]
    fn lsp_environment_and_server_roots_reject_unsafe_values() {
        for (key, value) in [
            ("LD_PRELOAD", "/tmp/inject.so"),
            ("PYTHONPATH", "/tmp/inject"),
            ("PATH", ".:/usr/bin"),
        ] {
            let mut raw = HashMap::new();
            raw.insert(
                "source_lsp_server_environment".into(),
                ConfigVal::Map(vec![(
                    "lean".into(),
                    ConfigVal::Map(vec![(key.into(), ConfigVal::Str(value.into()))]),
                )]),
            );
            let error = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
                raw,
                HashMap::new(),
            ))
            .unwrap_err();
            assert!(error.contains(key), "{error}");
        }

        for path in ["/", "/dev/shm", "/proc/self", "/tmp", "/home", "/root"] {
            let mut raw = HashMap::new();
            raw.insert(
                "source_lsp_server_read_only_roots".into(),
                ConfigVal::Map(vec![(
                    "lean".into(),
                    ConfigVal::List(vec![ConfigVal::Str(path.into())]),
                )]),
            );
            let error = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
                raw,
                HashMap::new(),
            ))
            .unwrap_err();
            assert!(
                error.contains("absolute non-root paths")
                    || error.contains("protected sandbox mount points"),
                "path {path:?}: {error}"
            );
        }

        let mut raw = HashMap::new();
        raw.insert(
            "source_lsp_sandbox_environment".into(),
            ConfigVal::Map(vec![(
                "LD_LIBRARY_PATH".into(),
                ConfigVal::Str("/tmp/lib".into()),
            )]),
        );
        let error = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            raw,
            HashMap::new(),
        ))
        .unwrap_err();
        assert!(error.contains("LD_LIBRARY_PATH"), "{error}");
    }

    #[cfg(feature = "source-sandbox")]
    #[test]
    fn protected_lsp_parses_mxc_policy_from_conf_and_keeps_network_denied_by_default() {
        let mut raw = HashMap::new();
        raw.insert(
            "source_lsp_sandbox".into(),
            ConfigVal::Str("protected-lsp".into()),
        );
        raw.insert(
            "source_lsp_servers".into(),
            ConfigVal::Map(vec![(
                "lean".into(),
                ConfigVal::List(vec![
                    ConfigVal::Str("lake".into()),
                    ConfigVal::Str("serve".into()),
                ]),
            )]),
        );
        raw.insert(
            "source_lsp_mxc_config".into(),
            ConfigVal::Map(vec![
                ("containment".into(), ConfigVal::Str("bubblewrap".into())),
                (
                    "filesystem".into(),
                    ConfigVal::Map(vec![
                        (
                            "readonly_paths".into(),
                            ConfigVal::List(vec![ConfigVal::Str("/opt/elan".into())]),
                        ),
                        (
                            "readwrite_paths".into(),
                            ConfigVal::List(vec![ConfigVal::Str("/workspace/.lake".into())]),
                        ),
                    ]),
                ),
                (
                    "network".into(),
                    ConfigVal::Map(vec![(
                        "egress".into(),
                        ConfigVal::Map(vec![("default".into(), ConfigVal::Str("deny".into()))]),
                    )]),
                ),
                ("timeout_ms".into(), ConfigVal::Int(10_000)),
            ]),
        );

        let settings = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            raw,
            HashMap::new(),
        ))
        .unwrap();

        assert_eq!(
            settings.lsp_mxc_config.containment,
            super::config::MxcLspContainment::Bubblewrap
        );
        assert!(!settings.lsp_mxc_config.mount_procfs);
        assert_eq!(
            settings.lsp_mxc_config.egress,
            super::config::MxcNetworkAction::Deny
        );
        assert_eq!(
            settings.lsp_mxc_config.ingress,
            super::config::MxcNetworkAction::Deny
        );
        assert_eq!(
            settings.lsp_mxc_config.host_loopback,
            super::config::MxcNetworkAction::Deny
        );
        assert_eq!(settings.lsp_mxc_config.timeout_ms, 10_000);
        assert_eq!(
            settings.lsp_read_only_roots_for("lean").unwrap(),
            [Path::new("/opt/elan")]
        );
        assert_eq!(
            settings.lsp_mxc_config.readwrite_paths,
            [Path::new("/workspace/.lake")]
        );
    }

    #[cfg(feature = "source-sandbox")]
    #[test]
    fn sandbox_off_override_changes_effective_mode_and_redacts_paths() {
        let mut raw = HashMap::new();
        raw.insert(
            "source_lsp_sandbox".into(),
            ConfigVal::Str("protected-lsp".into()),
        );
        raw.insert(
            "source_lsp_mxc_config".into(),
            ConfigVal::Map(vec![(
                "filesystem".into(),
                ConfigVal::Map(vec![(
                    "readonly_paths".into(),
                    ConfigVal::List(vec![ConfigVal::Str("/private/toolchain".into())]),
                )]),
            )]),
        );
        let config = SphinxConfig::new(raw, HashMap::new());
        let settings = super::config::SourceDocsSettings::from_sphinx_config(&config).unwrap();

        assert!(super::config::sandboxing_disabled_from_value(Some("off")));
        assert!(super::config::sandboxing_disabled_from_value(Some(" OFF ")));
        assert!(!super::config::sandboxing_disabled_from_value(Some(
            "false"
        )));
        assert!(!super::config::sandboxing_disabled_from_value(None));
        assert_eq!(
            settings.effective_lsp_sandbox_mode_with(false),
            super::config::SourceSandboxMode::ProtectedLsp
        );
        assert_eq!(
            settings.effective_lsp_sandbox_mode_with(true),
            super::config::SourceSandboxMode::TrustedLocal
        );
        let summary = super::config::sandboxing_config_summary_for(&config, true);
        assert!(summary.contains("SPHINXDOCRS_SANDBOXING=off"));
        assert!(summary.contains("configured_lsp=protected-lsp"));
        assert!(!summary.contains("/private/toolchain"));
    }

    #[test]
    fn mxc_lsp_config_uses_safe_defaults_and_explicit_overrides() {
        let defaults = super::config::SourceDocsSettings::default().lsp_mxc_config;
        assert_eq!(
            defaults.containment,
            super::config::MxcLspContainment::Process
        );
        assert!(!defaults.mount_procfs);
        assert!(defaults.readonly_paths.is_empty());
        assert!(defaults.readwrite_paths.is_empty());
        assert_eq!(defaults.egress, super::config::MxcNetworkAction::Deny);
        assert_eq!(defaults.ingress, super::config::MxcNetworkAction::Deny);
        assert_eq!(
            defaults.host_loopback,
            super::config::MxcNetworkAction::Deny
        );

        let mut raw = HashMap::new();
        raw.insert(
            "source_lsp_mxc_config".into(),
            ConfigVal::Map(vec![
                ("containment".into(), ConfigVal::Str("bubblewrap".into())),
                (
                    "filesystem".into(),
                    ConfigVal::Map(vec![
                        ("mount_procfs".into(), ConfigVal::Bool(true)),
                        (
                            "readonly_paths".into(),
                            ConfigVal::List(vec![ConfigVal::Str("/opt/lean".into())]),
                        ),
                        (
                            "readwrite_paths".into(),
                            ConfigVal::List(vec![ConfigVal::Str("/workspace/cache".into())]),
                        ),
                        (
                            "denied_paths".into(),
                            ConfigVal::List(vec![ConfigVal::Str("/workspace/secrets".into())]),
                        ),
                    ]),
                ),
                (
                    "network".into(),
                    ConfigVal::Map(vec![
                        (
                            "egress".into(),
                            ConfigVal::Map(vec![(
                                "default".into(),
                                ConfigVal::Str("allow".into()),
                            )]),
                        ),
                        (
                            "ingress".into(),
                            ConfigVal::Map(vec![
                                ("default".into(), ConfigVal::Str("deny".into())),
                                ("host_loopback".into(), ConfigVal::Str("allow".into())),
                            ]),
                        ),
                    ]),
                ),
                ("timeout_ms".into(), ConfigVal::Int(20_000)),
            ]),
        );
        let settings = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            raw,
            HashMap::new(),
        ))
        .unwrap();
        let mxc = &settings.lsp_mxc_config;
        assert_eq!(
            mxc.containment,
            super::config::MxcLspContainment::Bubblewrap
        );
        assert!(mxc.mount_procfs);
        assert_eq!(mxc.readonly_paths, [Path::new("/opt/lean")]);
        assert_eq!(mxc.readwrite_paths, [Path::new("/workspace/cache")]);
        assert_eq!(mxc.denied_paths, [Path::new("/workspace/secrets")]);
        assert_eq!(mxc.egress, super::config::MxcNetworkAction::Allow);
        assert_eq!(mxc.host_loopback, super::config::MxcNetworkAction::Allow);
        assert_eq!(mxc.timeout_ms, 20_000);
        let mut changed = settings.clone();
        changed.lsp_mxc_config.readwrite_paths.clear();
        assert_ne!(
            settings.lsp_configuration_identity("rust"),
            changed.lsp_configuration_identity("rust")
        );
    }

    #[test]
    fn sandbox_engine_defaults_parses_and_invalidates_cache_identity() {
        let defaults = super::config::SourceDocsSettings::default();
        assert_eq!(
            defaults.lsp_sandbox_engine,
            super::config::SourceSandboxEngine::Mxc
        );

        let parse_engine = |engine: &str| {
            let mut raw = HashMap::new();
            raw.insert(
                "source_lsp_sandbox_engine".into(),
                ConfigVal::Str(engine.into()),
            );
            super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
                raw,
                HashMap::new(),
            ))
        };

        let mxc = parse_engine("mxc").unwrap();
        let ai_sandbox = parse_engine("ai-sandbox").unwrap();
        assert_eq!(
            mxc.lsp_sandbox_engine,
            super::config::SourceSandboxEngine::Mxc
        );
        assert_eq!(
            ai_sandbox.lsp_sandbox_engine,
            super::config::SourceSandboxEngine::AiSandbox
        );
        assert_ne!(
            mxc.lsp_configuration_identity("rust"),
            ai_sandbox.lsp_configuration_identity("rust")
        );
        let mut protected_raw = HashMap::new();
        protected_raw.insert(
            "source_lsp_sandbox".into(),
            ConfigVal::Str("protected-lsp".into()),
        );
        protected_raw.insert(
            "source_lsp_sandbox_engine".into(),
            ConfigVal::Str("ai-sandbox".into()),
        );
        let protected_config = SphinxConfig::new(protected_raw, HashMap::new());
        assert!(
            super::config::sandboxing_config_summary_for(&protected_config, false)
                .contains("sandboxing=protected-lsp engine=ai-sandbox")
        );
        assert!(
            super::config::sandboxing_config_summary_for(&protected_config, true)
                .contains("configured_engine=ai-sandbox")
        );
        assert!(parse_engine("unknown").unwrap_err().contains("mxc"));
    }

    #[cfg(feature = "source-sandbox")]
    #[test]
    fn sandbox_off_override_is_explicit_and_logged_without_paths() {
        assert!(super::config::sandboxing_disabled_from_value(Some("off")));
        assert!(super::config::sandboxing_disabled_from_value(Some(" OFF ")));
        assert!(!super::config::sandboxing_disabled_from_value(Some(
            "false"
        )));
        assert!(!super::config::sandboxing_disabled_from_value(None));

        let mut raw = HashMap::new();
        raw.insert(
            "source_lsp_sandbox".into(),
            ConfigVal::Str("protected-lsp".into()),
        );
        raw.insert(
            "source_lsp_read_only_roots".into(),
            ConfigVal::List(vec![ConfigVal::Str("/opt/secret-toolchain".into())]),
        );
        raw.insert(
            "source_lsp_mxc_config".into(),
            ConfigVal::Map(vec![(
                "filesystem".into(),
                ConfigVal::Map(vec![(
                    "readonly_paths".into(),
                    ConfigVal::List(vec![ConfigVal::Str("/opt/secret-cache".into())]),
                )]),
            )]),
        );
        let config = SphinxConfig::new(raw, HashMap::new());
        let settings = super::config::SourceDocsSettings::from_sphinx_config(&config).unwrap();

        assert_eq!(
            settings.effective_lsp_sandbox_mode_with(false),
            super::config::SourceSandboxMode::ProtectedLsp
        );
        assert_eq!(
            settings.effective_lsp_sandbox_mode_with(true),
            super::config::SourceSandboxMode::TrustedLocal
        );
        let summary = super::config::sandboxing_config_summary_for(&config, true);
        assert!(summary.contains("SPHINXDOCRS_SANDBOXING=off"));
        assert!(summary.contains("configured_lsp=protected-lsp"));
        assert!(!summary.contains("secret-toolchain"));
        assert!(!summary.contains("secret-cache"));
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
        assert!(error.contains("protected-lsp"));

        let mut raw = HashMap::new();
        raw.insert("source_lsp_timeout".into(), ConfigVal::Int(0));
        let error = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            raw,
            HashMap::new(),
        ))
        .unwrap_err();
        assert!(error.contains("positive integer"));
    }

    #[test]
    fn protected_lsp_read_only_roots_are_absolute_and_affect_identity() {
        let mut raw = HashMap::new();
        raw.insert(
            "source_lsp_read_only_roots".into(),
            ConfigVal::List(vec![ConfigVal::Str("/opt/rust-toolchain".into())]),
        );
        raw.insert(
            "source_lsp_sandbox".into(),
            ConfigVal::Str("protected-lsp".into()),
        );
        let settings = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            raw,
            HashMap::new(),
        ));
        if !cfg!(feature = "source-sandbox")
            && !super::config::sandboxing_disabled_by_env()
        {
            assert_eq!(
                settings.unwrap_err(),
                "protected-lsp requires the optional source-sandbox feature (MXC)"
            );
            return;
        }
        let settings = settings.unwrap();
        assert_eq!(
            settings.lsp_read_only_roots,
            [Path::new("/opt/rust-toolchain")]
        );
        let mut changed = settings.clone();
        changed
            .lsp_read_only_roots
            .push(Path::new("/opt/rust-src").to_path_buf());
        assert_ne!(
            settings.lsp_configuration_identity("rust"),
            changed.lsp_configuration_identity("rust")
        );

        let mut invalid = HashMap::new();
        invalid.insert(
            "source_lsp_read_only_roots".into(),
            ConfigVal::List(vec![ConfigVal::Str("../toolchain".into())]),
        );
        let error = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            invalid,
            HashMap::new(),
        ))
        .unwrap_err();
        assert!(error.contains("absolute non-root paths"));
    }

    #[test]
    fn lsp_read_only_roots_reject_non_strings_and_filesystem_root() {
        let settings_with_root = |root| {
            let mut raw = HashMap::new();
            raw.insert(
                "source_lsp_read_only_roots".into(),
                ConfigVal::List(vec![root]),
            );
            super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
                raw,
                HashMap::new(),
            ))
        };

        assert!(settings_with_root(ConfigVal::Int(7))
            .unwrap_err()
            .contains("entries must be path strings"));
        assert!(settings_with_root(ConfigVal::Str("/".into()))
            .unwrap_err()
            .contains("absolute non-root paths"));
    }

    #[cfg(feature = "source-sandbox")]
    #[test]
    fn protected_lsp_settings_parse_explicit_roots_and_environment_allowlist() {
        let mut raw = HashMap::new();
        raw.insert(
            "source_lsp_sandbox".into(),
            ConfigVal::Str("protected-lsp".into()),
        );
        raw.insert(
            "source_lsp_read_only_roots".into(),
            ConfigVal::List(vec![ConfigVal::Str("/opt/rust-toolchain".into())]),
        );
        raw.insert(
            "source_lsp_sandbox_environment".into(),
            ConfigVal::Map(vec![(
                "RUSTUP_HOME".into(),
                ConfigVal::Str("/opt/rust-toolchain/rustup".into()),
            )]),
        );

        let settings = super::config::SourceDocsSettings::from_sphinx_config(&SphinxConfig::new(
            raw,
            HashMap::new(),
        ))
        .unwrap();

        assert_eq!(
            settings.lsp_sandbox,
            super::config::SourceSandboxMode::ProtectedLsp
        );
        assert_eq!(
            settings.lsp_read_only_roots,
            [Path::new("/opt/rust-toolchain")]
        );
        assert_eq!(
            settings
                .lsp_sandbox_environment
                .get("RUSTUP_HOME")
                .map(String::as_str),
            Some("/opt/rust-toolchain/rustup")
        );
    }
}
