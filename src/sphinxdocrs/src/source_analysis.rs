//! Language-neutral source declarations used by Rust and Lean documentation.
//!
//! The adapters in this module's feature-gated submodules own parser-specific
//! types. Everything consumed by domains, autodoc, apidoc, persistence, and
//! search crosses this boundary as one of the serde-stable records below.

use std::collections::BTreeMap;
use std::fs;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Bump when the serialized declaration contract changes incompatibly.
pub const SOURCE_ANALYSIS_SCHEMA_VERSION: u32 = 1;

/// Source language represented by an analysis snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceLanguage {
    Python,
    Rust,
    Lean,
}

impl fmt::Display for SourceLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Python => "python",
            Self::Rust => "rust",
            Self::Lean => "lean",
        })
    }
}

/// Visibility after the source adapter has interpreted language syntax.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Public,
    Restricted(String),
    Private,
    Unknown,
}

/// Declaration kinds shared by the source-aware domain and renderers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeclarationKind {
    Abbrev,
    AssociatedConstant,
    AssociatedType,
    Axiom,
    Class,
    Constant,
    Definition,
    Enum,
    Example,
    Field,
    Function,
    Impl,
    Inductive,
    Instance,
    Lemma,
    Macro,
    Method,
    Module,
    Namespace,
    Notation,
    Opaque,
    Static,
    Struct,
    Structure,
    Theorem,
    Trait,
    TypeAlias,
    Union,
    Variant,
    Other(String),
}

impl DeclarationKind {
    /// Stable label used in IDs, diagnostics, and search object types.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Abbrev => "abbrev",
            Self::AssociatedConstant => "associated_constant",
            Self::AssociatedType => "associated_type",
            Self::Axiom => "axiom",
            Self::Class => "class",
            Self::Constant => "constant",
            Self::Definition => "definition",
            Self::Enum => "enum",
            Self::Example => "example",
            Self::Field => "field",
            Self::Function => "function",
            Self::Impl => "impl",
            Self::Inductive => "inductive",
            Self::Instance => "instance",
            Self::Lemma => "lemma",
            Self::Macro => "macro",
            Self::Method => "method",
            Self::Module => "module",
            Self::Namespace => "namespace",
            Self::Notation => "notation",
            Self::Opaque => "opaque",
            Self::Static => "static",
            Self::Struct => "struct",
            Self::Structure => "structure",
            Self::Theorem => "theorem",
            Self::Trait => "trait",
            Self::TypeAlias => "type_alias",
            Self::Union => "union",
            Self::Variant => "variant",
            Self::Other(value) => value.as_str(),
        }
    }
}

/// One position in a normalized source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourcePosition {
    /// Zero-based byte offset in the normalized source.
    pub byte: usize,
    /// One-based source line.
    pub line: usize,
    /// Zero-based UTF-8 byte column within the source line.
    pub column: usize,
}

/// A source path and inclusive/exclusive declaration span.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub path: String,
    pub start: SourcePosition,
    pub end: Option<SourcePosition>,
}

impl SourceSpan {
    pub fn new(path: impl AsRef<Path>, start: SourcePosition, end: Option<SourcePosition>) -> Self {
        Self {
            path: normalize_path(path.as_ref()),
            start,
            end,
        }
    }
}

/// One declaration lowered from a language-specific source backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceDeclaration {
    /// Stable within a snapshot and deterministic across processes.
    pub id: String,
    pub language: SourceLanguage,
    /// Adapter identity, including a schema/parser version when applicable.
    pub backend: String,
    pub qualified_name: String,
    pub short_name: String,
    pub parent: Option<String>,
    pub kind: DeclarationKind,
    pub signature: Option<String>,
    pub type_text: Option<String>,
    pub documentation: String,
    pub visibility: Visibility,
    pub source: SourceSpan,
    pub aliases: Vec<String>,
    pub children: Vec<String>,
    pub deprecated: bool,
    pub noindex: bool,
    pub attributes: BTreeMap<String, String>,
}

impl SourceDeclaration {
    /// Build a declaration while deriving names and its stable ID.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        language: SourceLanguage,
        backend: impl Into<String>,
        qualified_name: impl Into<String>,
        separator: &str,
        kind: DeclarationKind,
        signature: Option<String>,
        type_text: Option<String>,
        documentation: impl AsRef<str>,
        visibility: Visibility,
        source: SourceSpan,
    ) -> Self {
        let backend = backend.into();
        let qualified_name = normalize_name(&qualified_name.into(), separator);
        let short_name = short_name(&qualified_name, separator).to_string();
        let parent = parent_name(&qualified_name, separator).map(str::to_string);
        let id = stable_declaration_id(language, &qualified_name, &kind);
        Self {
            id,
            language,
            backend,
            qualified_name,
            short_name,
            parent,
            kind,
            signature: signature.map(|value| normalize_text(&value)),
            type_text: type_text.map(|value| normalize_text(&value)),
            documentation: normalize_text(documentation.as_ref()),
            visibility,
            source,
            aliases: Vec::new(),
            children: Vec::new(),
            deprecated: false,
            noindex: false,
            attributes: BTreeMap::new(),
        }
    }

    /// Add an alternate lookup spelling while keeping the serialized order
    /// deterministic.
    pub fn add_alias(&mut self, alias: impl Into<String>) {
        let alias = normalize_name(&alias.into(), separator_for(self.language));
        if !alias.is_empty() && alias != self.qualified_name && !self.aliases.contains(&alias) {
            self.aliases.push(alias);
            self.aliases.sort();
        }
    }

    /// Add a child declaration ID or qualified name deterministically.
    pub fn add_child(&mut self, child: impl Into<String>) {
        let child = child.into();
        if !child.is_empty() && !self.children.contains(&child) {
            self.children.push(child);
            self.children.sort();
        }
    }
}

/// Severity for backend diagnostics retained with a snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// A non-fatal or fatal diagnostic emitted by a source backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisDiagnostic {
    pub severity: DiagnosticSeverity,
    pub backend: String,
    pub message: String,
    pub source: Option<SourceSpan>,
    pub declaration: Option<String>,
}

/// Backend selection policy supplied by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendPolicy {
    Auto,
    Required,
    AllowFallback,
}

impl Default for BackendPolicy {
    fn default() -> Self {
        Self::Auto
    }
}

/// Backend family that produced a normalized source snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceBackendKind {
    #[default]
    Static,
    Lsp,
    Hybrid,
}

/// In-memory provenance for normalized snapshot fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SourceProvenance {
    #[default]
    Static,
    Lsp,
    MergedStaticLsp,
}

/// Input shared by all source analyzers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceAnalysisRequest {
    pub source_root: PathBuf,
    pub selected: Vec<PathBuf>,
    pub package: Option<String>,
    pub module: Option<String>,
    pub features: Vec<String>,
    pub target: Option<String>,
    pub cfg: Vec<String>,
    pub include_private: bool,
    pub backend_policy: BackendPolicy,
    /// Upper bound for backend-owned work; interpretation is adapter-specific.
    pub resource_limit: Option<u64>,
}

impl SourceAnalysisRequest {
    pub fn new(source_root: impl Into<PathBuf>) -> Self {
        Self {
            source_root: source_root.into(),
            selected: Vec::new(),
            package: None,
            module: None,
            features: Vec::new(),
            target: None,
            cfg: Vec::new(),
            include_private: false,
            backend_policy: BackendPolicy::Auto,
            resource_limit: None,
        }
    }

    /// Stable cache identity for the source/configuration inputs that control
    /// analyzer output. Collection-valued options are sorted because their
    /// order does not affect source semantics.
    pub fn cache_identity(&self) -> String {
        let mut selected = self
            .selected
            .iter()
            .map(|path| normalize_path(path))
            .collect::<Vec<_>>();
        let mut features = self.features.clone();
        let mut cfg = self.cfg.clone();
        selected.sort();
        features.sort();
        cfg.sort();
        let value = format!(
            "source-request-v1\0root={}\0selected={selected:?}\0package={:?}\0module={:?}\0features={features:?}\0target={:?}\0cfg={cfg:?}\0private={}\0policy={:?}\0limit={:?}",
            normalize_path(&self.source_root),
            self.package,
            self.module,
            self.target,
            self.include_private,
            self.backend_policy,
            self.resource_limit,
        );
        let mut hash = 0xcbf29ce484222325u64;
        for byte in value.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("source-request-v1-{hash:016x}")
    }
}

/// Complete result of a source analysis pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSnapshot {
    pub schema_version: u32,
    pub language: SourceLanguage,
    /// Provider family, distinct from the concrete adapter in `backend`.
    #[serde(default)]
    pub backend_kind: SourceBackendKind,
    pub backend: String,
    pub backend_version: String,
    pub toolchain: Option<String>,
    pub source_root: String,
    pub source_hash: String,
    /// Identity of the request options that produced this snapshot.
    #[serde(default)]
    pub request_identity: String,
    /// Identity of provider configuration and implementation used for analysis.
    #[serde(default)]
    pub provider_identity: String,
    /// Per-field origin metadata is intentionally process-local. It is
    /// recomputed whenever providers produce or merge a snapshot.
    #[serde(skip)]
    pub provenance: BTreeMap<String, SourceProvenance>,
    pub declarations: Vec<SourceDeclaration>,
    pub diagnostics: Vec<AnalysisDiagnostic>,
}

impl AnalysisSnapshot {
    pub fn new(
        language: SourceLanguage,
        backend: impl Into<String>,
        backend_version: impl Into<String>,
        source_root: impl AsRef<Path>,
        source_hash: impl Into<String>,
        declarations: Vec<SourceDeclaration>,
        diagnostics: Vec<AnalysisDiagnostic>,
    ) -> Self {
        let mut snapshot = Self {
            schema_version: SOURCE_ANALYSIS_SCHEMA_VERSION,
            language,
            backend_kind: SourceBackendKind::Static,
            backend: backend.into(),
            backend_version: backend_version.into(),
            toolchain: None,
            source_root: normalize_path(source_root.as_ref()),
            source_hash: source_hash.into(),
            request_identity: String::new(),
            provider_identity: String::new(),
            provenance: BTreeMap::new(),
            declarations,
            diagnostics,
        };
        snapshot.normalize_and_sort();
        snapshot
    }

    /// Associate this snapshot with the request that produced it.
    pub fn set_request_identity(&mut self, request: &SourceAnalysisRequest) {
        self.request_identity = request.cache_identity();
    }

    /// Set provenance for an individual declaration field.
    pub fn set_provenance(
        &mut self,
        declaration_id: &str,
        field: &str,
        provenance: SourceProvenance,
    ) {
        self.provenance
            .insert(format!("{declaration_id}.{field}"), provenance);
    }

    /// Normalize adapter output and impose deterministic ordering at the
    /// shared boundary, so downstream consumers can hash/cache it directly.
    pub fn normalize_and_sort(&mut self) {
        for declaration in &mut self.declarations {
            declaration.qualified_name = normalize_name(
                &declaration.qualified_name,
                separator_for(declaration.language),
            );
            declaration.short_name = short_name(
                &declaration.qualified_name,
                separator_for(declaration.language),
            )
            .to_string();
            declaration.parent = parent_name(
                &declaration.qualified_name,
                separator_for(declaration.language),
            )
            .map(str::to_string);
            declaration.documentation = normalize_text(&declaration.documentation);
            declaration.signature = declaration.signature.take().map(|value| normalize_text(&value));
            declaration.type_text = declaration.type_text.take().map(|value| normalize_text(&value));
            declaration.aliases = std::mem::take(&mut declaration.aliases)
                .into_iter()
                .map(|alias| normalize_name(&alias, separator_for(declaration.language)))
                .filter(|alias| !alias.is_empty() && alias != &declaration.qualified_name)
                .collect();
            declaration.aliases.sort();
            declaration.aliases.dedup();
            declaration.children.sort();
            declaration.children.dedup();
            declaration.source.path = normalize_path(Path::new(&declaration.source.path));
        }
        self.declarations.sort_by(|left, right| {
            left.qualified_name
                .cmp(&right.qualified_name)
                .then(left.kind.cmp(&right.kind))
                .then(left.id.cmp(&right.id))
        });
        self.diagnostics.sort_by(|left, right| {
            left.backend
                .cmp(&right.backend)
                .then(left.message.cmp(&right.message))
        });
    }
}

/// Backend-neutral provider contract consumed by source-documentation layers.
///
/// `SourceAnalyzer` remains a compatibility re-export during migration.
pub trait SourceSnapshotProvider {
    fn analyze(
        &self,
        request: &SourceAnalysisRequest,
    ) -> Result<AnalysisSnapshot, AnalysisError>;

    fn backend_kind(&self) -> SourceBackendKind {
        SourceBackendKind::Static
    }

    /// Identity of provider-specific configuration that affects snapshots.
    fn cache_identity(&self, request: &SourceAnalysisRequest) -> String {
        request.cache_identity()
    }

    /// Hash inputs consumed by this provider. LSP providers may override this
    /// when their source set differs from the static Rustdoc/Lean inputs.
    fn source_hash(
        &self,
        request: &SourceAnalysisRequest,
        language: SourceLanguage,
    ) -> Result<String, AnalysisError> {
        source_input_hash(request, language)
    }
}

pub use self::SourceSnapshotProvider as SourceAnalyzer;

/// Errors that must not be converted into an empty successful domain.
#[derive(Debug, thiserror::Error)]
pub enum AnalysisError {
    #[error("source backend unavailable: {backend}: {message}")]
    BackendUnavailable { backend: String, message: String },
    #[error("invalid source analysis request: {0}")]
    InvalidRequest(String),
    #[error("source analysis parse failed in {backend}: {message}")]
    Parse { backend: String, message: String },
    #[error("source analysis I/O failed for {path}: {message}")]
    Io { path: String, message: String },
}

/// Hash the same backend inputs that produced an analysis snapshot.
pub fn source_input_hash(
    request: &SourceAnalysisRequest,
    language: SourceLanguage,
) -> Result<String, AnalysisError> {
    let mut files = match language {
        SourceLanguage::Rust => {
            if request
                .source_root
                .extension()
                .is_some_and(|extension| extension == "json")
            {
                vec![request.source_root.clone()]
            } else {
                request
                    .selected
                    .iter()
                    .find(|path| path.extension().is_some_and(|extension| extension == "json"))
                    .cloned()
                    .into_iter()
                    .collect()
            }
        }
        SourceLanguage::Lean => {
            if !request.selected.is_empty() {
                request.selected.clone()
            } else if request.source_root.is_file() {
                vec![request.source_root.clone()]
            } else {
                let mut files = Vec::new();
                collect_source_files(&request.source_root, "lean", &mut files).map_err(|error| {
                    AnalysisError::Io {
                        path: request.source_root.display().to_string(),
                        message: error.to_string(),
                    }
                })?;
                files
            }
        }
        SourceLanguage::Python => {
            return Err(AnalysisError::InvalidRequest(
                "Python snapshots do not have a source-analysis hash".to_string(),
            ));
        }
    };

    files.retain(|path| {
        path.extension().is_some_and(|extension| match language {
            SourceLanguage::Rust => extension == "json",
            SourceLanguage::Lean => extension == "lean",
            SourceLanguage::Python => false,
        })
    });
    files.sort();
    files.dedup();
    if files.is_empty() {
        return Err(AnalysisError::InvalidRequest(format!(
            "no source inputs found for {language}"
        )));
    }

    let mut bytes = Vec::new();
    for path in files {
        let input = fs::read(&path).map_err(|error| AnalysisError::Io {
            path: path.display().to_string(),
            message: error.to_string(),
        })?;
        bytes.extend_from_slice(&input);
    }
    Ok(hash_bytes(&bytes))
}

fn collect_source_files(
    root: &Path,
    extension: &str,
    output: &mut Vec<PathBuf>,
) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_source_files(&path, extension, output)?;
        } else if path.extension().is_some_and(|value| value == extension) {
            output.push(path);
        }
    }
    Ok(())
}

pub(crate) fn hash_bytes(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

/// Return the language's canonical namespace separator.
pub fn separator_for(language: SourceLanguage) -> &'static str {
    match language {
        SourceLanguage::Rust => "::",
        SourceLanguage::Python | SourceLanguage::Lean => ".",
    }
}

/// Normalize a qualified name without changing its language separator.
pub fn normalize_name(name: &str, separator: &str) -> String {
    name.replace('\r', "")
        .split(separator)
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(separator)
}

/// Return the final namespace component of `qualified_name`.
pub fn short_name<'a>(qualified_name: &'a str, separator: &str) -> &'a str {
    qualified_name
        .rsplit_once(separator)
        .map_or(qualified_name, |(_, short)| short)
}

/// Return the namespace containing `qualified_name`.
pub fn parent_name<'a>(qualified_name: &'a str, separator: &str) -> Option<&'a str> {
    qualified_name
        .rsplit_once(separator)
        .map(|(parent, _)| parent)
}

/// Normalize source comments while preserving paragraph line breaks.
pub fn normalize_text(value: &str) -> String {
    value.replace("\r\n", "\n").replace('\r', "\n").trim().to_string()
}

/// Normalize paths used in serialized snapshots to slash-separated paths.
pub fn normalize_path(path: &Path) -> String {
    let value = path.to_string_lossy().replace('\\', "/");
    value.strip_prefix("./").unwrap_or(&value).to_string()
}

/// Deterministic FNV-1a ID for internal graph edges.
pub fn stable_declaration_id(
    language: SourceLanguage,
    qualified_name: &str,
    kind: &DeclarationKind,
) -> String {
    let input = format!("{language}\0{qualified_name}\0{}", kind.as_str());
    let mut hash = 0xcbf29ce484222325u64;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span() -> SourceSpan {
        SourceSpan::new(
            Path::new("./src\\lib.rs"),
            SourcePosition {
                byte: 4,
                line: 2,
                column: 1,
            },
            None,
        )
    }

    #[test]
    fn names_use_language_separator_and_preserve_short_name() {
        assert_eq!(normalize_name(" crate :: api :: Thing ", "::"), "crate::api::Thing");
        assert_eq!(short_name("crate::api::Thing", "::"), "Thing");
        assert_eq!(parent_name("crate::api::Thing", "::"), Some("crate::api"));
        assert_eq!(short_name("Main", "."), "Main");
    }

    #[test]
    fn declaration_metadata_is_normalized_and_aliases_are_sorted() {
        let mut declaration = SourceDeclaration::new(
            SourceLanguage::Rust,
            "rustdoc-json",
            "crate::Thing",
            "::",
            DeclarationKind::Struct,
            Some("pub struct Thing\r\nwhere T: Copy".to_string()),
            None,
            " first line\r\n\r\n second line ",
            Visibility::Public,
            span(),
        );
        declaration.add_alias("crate::Zed");
        declaration.add_alias("crate::Alias");
        declaration.add_alias("crate::Alias");
        declaration.add_child("child-2");
        declaration.add_child("child-1");

        assert_eq!(declaration.documentation, "first line\n\n second line");
        assert_eq!(declaration.signature.as_deref(), Some("pub struct Thing\nwhere T: Copy"));
        assert_eq!(declaration.source.path, "src/lib.rs");
        assert_eq!(declaration.aliases, ["crate::Alias", "crate::Zed"]);
        assert_eq!(declaration.children, ["child-1", "child-2"]);
        assert_eq!(declaration.parent.as_deref(), Some("crate"));
    }

    #[test]
    fn snapshots_sort_and_round_trip() {
        let first = SourceDeclaration::new(
            SourceLanguage::Lean,
            "arborium-lean",
            "Demo.zeta",
            ".",
            DeclarationKind::Theorem,
            None,
            Some("p -> q".to_string()),
            "proof",
            Visibility::Public,
            span(),
        );
        let second = SourceDeclaration::new(
            SourceLanguage::Lean,
            "arborium-lean",
            "Demo.alpha",
            ".",
            DeclarationKind::Definition,
            None,
            Some("Nat".to_string()),
            "value",
            Visibility::Public,
            span(),
        );
        let snapshot = AnalysisSnapshot::new(
            SourceLanguage::Lean,
            "arborium-lean",
            "2.18.2",
            "./lean",
            "hash",
            vec![first, second],
            Vec::new(),
        );
        assert_eq!(snapshot.declarations[0].qualified_name, "Demo.alpha");
        let bytes = serde_json::to_vec(&snapshot).unwrap();
        let restored: AnalysisSnapshot = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(restored.schema_version, SOURCE_ANALYSIS_SCHEMA_VERSION);
        assert_eq!(restored.backend_kind, SourceBackendKind::Static);
        let mut with_provenance = snapshot.clone();
        let declaration_id = with_provenance.declarations[0].id.clone();
        with_provenance.set_provenance(
            &declaration_id,
            "documentation",
            SourceProvenance::Lsp,
        );
        let round_trip: AnalysisSnapshot =
            serde_json::from_slice(&serde_json::to_vec(&with_provenance).unwrap()).unwrap();
        assert!(round_trip.provenance.is_empty());
        assert_eq!(restored.declarations, snapshot.declarations);

        let mut legacy = serde_json::to_value(&snapshot).unwrap();
        legacy.as_object_mut().unwrap().remove("backend_kind");
        legacy.as_object_mut().unwrap().remove("provider_identity");
        let restored_legacy: AnalysisSnapshot = serde_json::from_value(legacy).unwrap();
        assert_eq!(restored_legacy.backend_kind, SourceBackendKind::Static);
        assert!(restored_legacy.provider_identity.is_empty());
    }

    #[test]
    fn stable_ids_depend_on_kind_and_language() {
        let rust_function = stable_declaration_id(
            SourceLanguage::Rust,
            "crate::f",
            &DeclarationKind::Function,
        );
        assert_eq!(rust_function.len(), 16);
        assert_ne!(
            rust_function,
            stable_declaration_id(SourceLanguage::Lean, "crate::f", &DeclarationKind::Function)
        );
        assert_ne!(
            rust_function,
            stable_declaration_id(SourceLanguage::Rust, "crate::f", &DeclarationKind::Struct)
        );
    }

    #[test]
    fn request_identity_canonicalizes_semantic_options() {
        let mut first = SourceAnalysisRequest::new("./src");
        first.features = vec!["serde".to_string(), "default".to_string()];
        first.cfg = vec!["unix".to_string(), "feature=docs".to_string()];
        first.selected = vec![PathBuf::from("src/z.rs"), PathBuf::from("src/a.rs")];
        first.include_private = true;
        let mut second = first.clone();
        second.features.reverse();
        second.cfg.reverse();
        second.selected.reverse();
        assert_eq!(first.cache_identity(), second.cache_identity());

        second.target = Some("wasm32-unknown-unknown".to_string());
        assert_ne!(first.cache_identity(), second.cache_identity());
    }
}

#[cfg(all(
    test,
    any(
        feature = "rust-source-analysis",
        feature = "lean-source-analysis",
        feature = "lsp-source-analysis"
    )
))]
pub(crate) mod provider_contract {
    use super::*;

    pub(crate) fn assert_provider_contract(
        provider: &dyn SourceSnapshotProvider,
        request: &SourceAnalysisRequest,
    ) -> AnalysisSnapshot {
        let snapshot = provider.analyze(request).unwrap();
        let repeated = provider.analyze(request).unwrap();

        assert_eq!(snapshot.backend_kind, provider.backend_kind());
        assert_eq!(snapshot.request_identity, request.cache_identity());
        assert_eq!(snapshot.request_identity, repeated.request_identity);
        assert_eq!(snapshot.source_hash, repeated.source_hash);
        assert_eq!(snapshot.declarations, repeated.declarations);
        assert_eq!(snapshot.diagnostics, repeated.diagnostics);

        let mut declarations = snapshot.declarations.clone();
        declarations.sort_by(|left, right| {
            left.qualified_name
                .cmp(&right.qualified_name)
                .then(left.kind.cmp(&right.kind))
                .then(left.id.cmp(&right.id))
        });
        assert_eq!(snapshot.declarations, declarations);
        let mut ids = snapshot
            .declarations
            .iter()
            .map(|declaration| declaration.id.as_str())
            .collect::<Vec<_>>();
        ids.sort_unstable();
        assert!(ids.windows(2).all(|pair| pair[0] != pair[1]));
        for declaration in &snapshot.declarations {
            assert_eq!(
                declaration.id,
                stable_declaration_id(
                    declaration.language,
                    &declaration.qualified_name,
                    &declaration.kind,
                )
            );
            assert!(!declaration.source.path.is_empty());
            assert!(declaration.source.start.line > 0);
            assert!(declaration.aliases.windows(2).all(|pair| pair[0] < pair[1]));
            assert!(declaration.children.windows(2).all(|pair| pair[0] < pair[1]));
        }

        let mut diagnostics = snapshot.diagnostics.clone();
        diagnostics.sort_by(|left, right| {
            left.backend
                .cmp(&right.backend)
                .then(left.message.cmp(&right.message))
        });
        assert_eq!(snapshot.diagnostics, diagnostics);

        let encoded = serde_json::to_vec(&snapshot).unwrap();
        let decoded: AnalysisSnapshot = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded.backend_kind, snapshot.backend_kind);
        assert_eq!(decoded.backend, snapshot.backend);
        assert_eq!(decoded.request_identity, snapshot.request_identity);
        assert_eq!(decoded.provider_identity, snapshot.provider_identity);
        assert_eq!(decoded.declarations, snapshot.declarations);
        assert_eq!(decoded.diagnostics, snapshot.diagnostics);
        snapshot
    }
}

#[cfg(feature = "rust-source-analysis")]
pub mod rust;

#[cfg(feature = "lean-source-analysis")]
pub mod lean;