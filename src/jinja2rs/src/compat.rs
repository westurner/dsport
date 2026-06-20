//! `jinja2rs::compat` — Jinja2 vs minijinja compatibility modes.
//!
//! This module provides configuration for four compatibility modes:
//!
//! - **Jinja2 mode** (default): Drop-in compatible with Python Jinja2. Enables
//!   Python method syntax like `obj.items()`, `obj.values()`, `dict.get()`,
//!   `str.upper()`, etc. via the `minijinja-contrib` pycompat module.
//!
//! - **minijinja mode**: Uses minijinja's native filter-based approach.
//!   Methods like `.items()` are not available; use `|items` filter instead.
//!   This mode has lower overhead and is more explicit.
//!
//! - **Ansible mode**: Specialized mode for Ansible playbooks with:
//!   - Curated Ansible standard filters (`to_nice_json`, `combine`, `regex_*`, etc.)
//!   - Inventory support (hosts, groups, hostvars)
//!   - YAML validation for playbooks and inventories
//!   - Composable method syntax (can combine with Jinja2 or filter-based approach)
//!
//! - **Django mode**: Drop-in compatible with Django's template language.
//!   - Django-specific filters: `slugify`, `truncatewords`, `floatformat`, `pluralize`, etc.
//!   - App-directory template loading convention (`templates/` subdirectory)
//!   - HTML auto-escaping enabled by default (matches Django behavior)
//!   - Context processor support

use std::path::PathBuf;

/// Ansible inventory source.
#[derive(Debug, Clone, PartialEq)]
pub enum AnsibleInventorySource {
    /// Load inventory from a file path
    File(PathBuf),
    /// Load inventory from standard input
    Stdin,
    /// Load inventory from inline YAML/JSON string
    Inline(String),
}

/// Ansible-specific configuration (composable with method_syntax).
#[derive(Debug, Clone, PartialEq)]
pub struct AnsibleMode {
    /// Whether to enable Python method syntax (Jinja2-style).
    /// If false, use filter-based syntax (minijinja-style).
    pub method_syntax: bool,

    /// Enable YAML/JSON validation for playbooks and inventories.
    /// When enabled, validates structure before rendering.
    pub enable_validation: bool,

    /// Inventory source for Ansible variables.
    /// When set, loads inventory and provides:
    /// - `inventory_hostname` — current host name
    /// - `groups` — group membership
    /// - `hostvars` — host variables
    /// - `group_names` — list of groups
    pub inventory_source: Option<AnsibleInventorySource>,
}

impl Default for AnsibleMode {
    fn default() -> Self {
        AnsibleMode {
            method_syntax: true,     // Default to Jinja2-style methods
            enable_validation: true, // Validate by default
            inventory_source: None,
        }
    }
}

impl AnsibleMode {
    /// Create an Ansible mode with all features enabled.
    pub fn full() -> Self {
        AnsibleMode::default()
    }

    /// Create an Ansible mode with method syntax enabled.
    pub fn with_methods() -> Self {
        AnsibleMode {
            method_syntax: true,
            enable_validation: true,
            inventory_source: None,
        }
    }

    /// Create an Ansible mode with filter-based syntax (no methods).
    pub fn filter_only() -> Self {
        AnsibleMode {
            method_syntax: false,
            enable_validation: true,
            inventory_source: None,
        }
    }

    /// Enable inventory support from a file.
    pub fn with_inventory_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.inventory_source = Some(AnsibleInventorySource::File(path.into()));
        self
    }

    /// Enable inventory support from stdin.
    pub fn with_inventory_stdin(mut self) -> Self {
        self.inventory_source = Some(AnsibleInventorySource::Stdin);
        self
    }

    /// Enable inventory support from inline YAML/JSON.
    pub fn with_inventory_inline(mut self, data: String) -> Self {
        self.inventory_source = Some(AnsibleInventorySource::Inline(data));
        self
    }

    /// Enable or disable validation.
    pub fn with_validation(mut self, enable: bool) -> Self {
        self.enable_validation = enable;
        self
    }
}

/// Kubernetes manifest inventory source.
#[derive(Debug, Clone, PartialEq)]
pub enum KubernetesInventorySource {
    /// Load manifests from a file path
    File(PathBuf),
    /// Load manifests from standard input
    Stdin,
    /// Load manifests from inline YAML string
    Inline(String),
}

/// Kubernetes-specific configuration (composable with method_syntax).
#[derive(Debug, Clone, PartialEq)]
pub struct KubernetesMode {
    /// Whether to enable Python method syntax (Jinja2-style).
    /// If false, use filter-based syntax (minijinja-style).
    pub method_syntax: bool,

    /// Enable YAML validation for Kubernetes manifests.
    /// When enabled, validates structure before rendering.
    pub enable_validation: bool,

    /// Manifest source for Kubernetes resource variables.
    /// When set, loads manifests and provides:
    /// - `kubernetes_resources` — all resources by kind
    /// - `kubernetes_pods` — pod details (name, namespace, labels)
    /// - `kubernetes_deployments` — deployment details
    /// - `kubernetes_services` — service details
    /// - `kubernetes_namespace` — current namespace
    pub manifest_source: Option<KubernetesInventorySource>,

    /// Default namespace for resource operations (default: "default")
    pub namespace: String,

    /// Filter resources by kind (e.g., "Deployment", "Pod", "Service")
    /// None = include all kinds
    pub resource_kind_filter: Option<String>,
}

impl Default for KubernetesMode {
    fn default() -> Self {
        KubernetesMode {
            method_syntax: true,
            enable_validation: true,
            manifest_source: None,
            namespace: "default".to_string(),
            resource_kind_filter: None,
        }
    }
}

impl KubernetesMode {
    /// Create Kubernetes mode with all features enabled.
    pub fn full() -> Self {
        KubernetesMode::default()
    }

    /// Create Kubernetes mode with method syntax enabled.
    pub fn with_methods() -> Self {
        KubernetesMode {
            method_syntax: true,
            enable_validation: true,
            manifest_source: None,
            namespace: "default".to_string(),
            resource_kind_filter: None,
        }
    }

    /// Create Kubernetes mode with filter-based syntax (no methods).
    pub fn filter_only() -> Self {
        KubernetesMode {
            method_syntax: false,
            enable_validation: true,
            manifest_source: None,
            namespace: "default".to_string(),
            resource_kind_filter: None,
        }
    }

    /// Set the default namespace.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = namespace.into();
        self
    }

    /// Filter resources by kind (e.g., "Deployment", "Pod").
    pub fn with_resource_kind(mut self, kind: impl Into<String>) -> Self {
        self.resource_kind_filter = Some(kind.into());
        self
    }

    /// Enable manifest support from a file.
    pub fn with_manifest_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.manifest_source = Some(KubernetesInventorySource::File(path.into()));
        self
    }

    /// Enable manifest support from stdin.
    pub fn with_manifest_stdin(mut self) -> Self {
        self.manifest_source = Some(KubernetesInventorySource::Stdin);
        self
    }

    /// Enable manifest support from inline YAML.
    pub fn with_manifest_inline(mut self, data: String) -> Self {
        self.manifest_source = Some(KubernetesInventorySource::Inline(data));
        self
    }

    /// Enable or disable validation.
    pub fn with_validation(mut self, enable: bool) -> Self {
        self.enable_validation = enable;
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Django mode
// ─────────────────────────────────────────────────────────────────────────────

/// Auto-escaping strategy for Django mode.
///
/// Matches the Django `django.template.backends.jinja2` auto-escape defaults.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DjangoAutoEscape {
    /// Escape `.html`, `.htm`, `.xml` templates (Django default).
    Html,
    /// Always escape all templates.
    Always,
    /// Never auto-escape (opt-in safety).
    #[default]
    Never,
}

/// Django template language compatibility mode.
///
/// Configures `jinja2rs` to render templates written for the Django template
/// engine with high fidelity:
///
/// - Django-specific filters (`slugify`, `truncatewords`, `floatformat`, …)
/// - App-directory loader: searches `<app_dir>/templates/` by convention.
/// - HTML auto-escaping on by default (matching Django's safe-by-default policy).
/// - Context processors: callables that inject variables into every render.
///
/// # Example
///
/// ```rust,ignore
/// use jinja2rs::compat::{CompatMode, DjangoMode};
/// use jinja2rs::Environment;
/// use std::path::PathBuf;
///
/// let mut env = Environment::new();
/// env.set_compat_mode(CompatMode::Django(
///     DjangoMode::default()
///         .with_app_directory("/myapp")
///         .with_timezone("UTC"),
/// ));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DjangoMode {
    /// Directories searched for templates using the app-directory convention.
    ///
    /// Each directory is searched as `<dir>/templates/<name>`.  Directories are
    /// tried in the order they are listed; the first match wins.
    pub app_directories: Vec<PathBuf>,

    /// Auto-escape strategy (default: `Html` — escape `.html`/`.htm`/`.xml`).
    pub auto_escape: DjangoAutoEscape,

    /// Default timezone name used by date/time filters (e.g. `"UTC"`, `"Europe/Paris"`).
    pub timezone: String,

    /// BCP-47 locale tag used by i18n-aware filters (e.g. `"en-US"`, `"fr-FR"`).
    pub locale: String,

    /// Enable URL resolution for `{% url %}` tag.
    ///
    /// When `false`, the `{% url %}` tag will render as an empty string rather
    /// than attempting reverse URL lookup.
    pub enable_url_resolution: bool,
}

impl Default for DjangoMode {
    fn default() -> Self {
        DjangoMode {
            app_directories: Vec::new(),
            auto_escape: DjangoAutoEscape::Html,
            timezone: "UTC".to_string(),
            locale: "en-US".to_string(),
            enable_url_resolution: false,
        }
    }
}

impl DjangoMode {
    /// Create a Django mode with all features enabled and empty search paths.
    pub fn new() -> Self {
        DjangoMode::default()
    }

    /// Create a minimal Django mode (no app directories, no URL resolution).
    pub fn minimal() -> Self {
        DjangoMode {
            app_directories: Vec::new(),
            auto_escape: DjangoAutoEscape::Html,
            timezone: "UTC".to_string(),
            locale: "en-US".to_string(),
            enable_url_resolution: false,
        }
    }

    /// Append an app directory to the search path.
    ///
    /// The directory will be searched as `<dir>/templates/<template_name>`.
    pub fn with_app_directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.app_directories.push(path.into());
        self
    }

    /// Set the auto-escape strategy.
    pub fn with_auto_escape(mut self, strategy: DjangoAutoEscape) -> Self {
        self.auto_escape = strategy;
        self
    }

    /// Set the default timezone name.
    pub fn with_timezone(mut self, tz: impl Into<String>) -> Self {
        self.timezone = tz.into();
        self
    }

    /// Set the locale tag.
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = locale.into();
        self
    }

    /// Enable or disable URL resolution for `{% url %}`.
    pub fn with_url_resolution(mut self, enable: bool) -> Self {
        self.enable_url_resolution = enable;
        self
    }

    /// Return `true` if this mode uses HTML auto-escaping.
    pub fn html_auto_escape(&self) -> bool {
        !matches!(self.auto_escape, DjangoAutoEscape::Never)
    }
}

#[derive(Debug, PartialEq)]
pub enum CompatMode {
    /// Jinja2 compatibility mode (default).
    ///
    /// Enables Python method syntax via the `minijinja-contrib` pycompat module:
    /// - `dict.items()`, `dict.values()`, `dict.keys()`, `dict.get()`
    /// - `str.upper()`, `str.lower()`, `str.split()`, `str.format()`, etc.
    /// - `list.count()`
    ///
    /// This makes templates written for Python Jinja2 work without modification.
    Jinja2,

    /// minijinja compatibility mode (strict).
    ///
    /// Uses minijinja's native approach with no Python method support.
    /// Methods are not available; use filters instead:
    /// - Use `|items`, `|values`, `|keys` filters instead of `.items()`, etc.
    /// - Use `|upper`, `|lower` instead of `.upper()`, etc.
    ///
    /// This mode is more efficient and encourages explicit filter-based syntax.
    Minijinja,

    /// Ansible compatibility mode (composable).
    ///
    /// Specialized mode for Ansible playbooks with:
    /// - Curated Ansible standard filters
    /// - Inventory support (hosts, groups, variables)
    /// - YAML validation
    /// - Composable method syntax (can use Jinja2 or filter-based)
    ///
    /// # Examples
    ///
    /// Ansible with method syntax:
    /// ```rust,ignore
    /// CompatMode::Ansible(AnsibleMode::with_methods())
    /// ```
    ///
    /// Ansible with filter-based syntax:
    /// ```rust,ignore
    /// CompatMode::Ansible(AnsibleMode::filter_only())
    /// ```
    ///
    /// Ansible with inventory:
    /// ```rust,ignore
    /// CompatMode::Ansible(
    ///     AnsibleMode::default()
    ///         .with_inventory_file("/etc/ansible/hosts")
    /// )
    /// ```
    Ansible(AnsibleMode),

    /// Kubernetes compatibility mode (composable).
    ///
    /// Specialized mode for Kubernetes manifests with:
    /// - Kubernetes resource filters and accessors
    /// - Manifest loading (from `podman generate kube` output or raw K8s YAML)
    /// - Support for workload introspection (Pods, Deployments, Services, etc.)
    /// - Composable method syntax (can use Jinja2 or filter-based)
    ///
    /// # Examples
    ///
    /// Kubernetes with method syntax:
    /// ```rust,ignore
    /// CompatMode::Kubernetes(KubernetesMode::with_methods())
    /// ```
    ///
    /// Kubernetes with manifest file:
    /// ```rust,ignore
    /// CompatMode::Kubernetes(
    ///     KubernetesMode::default()
    ///         .with_manifest_file("deployment.yaml")
    ///         .with_namespace("production")
    /// )
    /// ```
    Kubernetes(KubernetesMode),

    /// Django template language compatibility mode.
    ///
    /// Enables Django-specific filters, app-directory template loading, and
    /// HTML auto-escaping by default.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// CompatMode::Django(
    ///     DjangoMode::default()
    ///         .with_app_directory("/myproject/myapp")
    ///         .with_timezone("Europe/London")
    /// )
    /// ```
    Django(DjangoMode),
}

impl Default for CompatMode {
    /// Default compatibility mode is Jinja2 (for maximum compatibility).
    fn default() -> Self {
        CompatMode::Jinja2
    }
}

impl CompatMode {
    /// Returns true if this is Jinja2 compatibility mode.
    pub fn is_jinja2(&self) -> bool {
        matches!(self, CompatMode::Jinja2)
    }

    /// Returns true if this is minijinja compatibility mode.
    pub fn is_minijinja(&self) -> bool {
        matches!(self, CompatMode::Minijinja)
    }

    /// Returns true if this is Ansible compatibility mode.
    pub fn is_ansible(&self) -> bool {
        matches!(self, CompatMode::Ansible(_))
    }

    /// Get the Ansible mode if this is Ansible mode, otherwise None.
    pub fn as_ansible(&self) -> Option<&AnsibleMode> {
        match self {
            CompatMode::Ansible(cfg) => Some(cfg),
            _ => None,
        }
    }

    /// Returns true if this is Kubernetes compatibility mode.
    pub fn is_kubernetes(&self) -> bool {
        matches!(self, CompatMode::Kubernetes(_))
    }

    /// Get the Kubernetes mode if this is Kubernetes mode, otherwise None.
    pub fn as_kubernetes(&self) -> Option<&KubernetesMode> {
        match self {
            CompatMode::Kubernetes(cfg) => Some(cfg),
            _ => None,
        }
    }

    /// Returns true if this is Django compatibility mode.
    pub fn is_django(&self) -> bool {
        matches!(self, CompatMode::Django(_))
    }

    /// Get the Django mode configuration if this is Django mode.
    pub fn as_django(&self) -> Option<&DjangoMode> {
        match self {
            CompatMode::Django(cfg) => Some(cfg),
            _ => None,
        }
    }

    /// Check if this mode supports Python method syntax.
    ///
    /// Returns true for:
    /// - Jinja2 mode (always has method syntax)
    /// - Ansible mode with `method_syntax: true`
    /// - Kubernetes mode with `method_syntax: true`
    /// - Django mode uses dot-notation access but not Python method calls.
    pub fn has_method_syntax(&self) -> bool {
        match self {
            CompatMode::Jinja2 => true,
            CompatMode::Minijinja => false,
            CompatMode::Ansible(cfg) => cfg.method_syntax,
            CompatMode::Kubernetes(cfg) => cfg.method_syntax,
            CompatMode::Django(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_jinja2() {
        assert_eq!(CompatMode::default(), CompatMode::Jinja2);
    }

    #[test]
    fn test_is_jinja2() {
        assert!(CompatMode::Jinja2.is_jinja2());
        assert!(!CompatMode::Minijinja.is_jinja2());
        assert!(!CompatMode::Ansible(AnsibleMode::default()).is_jinja2());
    }

    #[test]
    fn test_is_minijinja() {
        assert!(CompatMode::Minijinja.is_minijinja());
        assert!(!CompatMode::Jinja2.is_minijinja());
        assert!(!CompatMode::Ansible(AnsibleMode::default()).is_minijinja());
    }

    #[test]
    fn test_is_ansible() {
        assert!(CompatMode::Ansible(AnsibleMode::default()).is_ansible());
        assert!(!CompatMode::Jinja2.is_ansible());
        assert!(!CompatMode::Minijinja.is_ansible());
    }

    #[test]
    fn test_has_method_syntax_jinja2() {
        assert!(CompatMode::Jinja2.has_method_syntax());
    }

    #[test]
    fn test_has_method_syntax_minijinja() {
        assert!(!CompatMode::Minijinja.has_method_syntax());
    }

    #[test]
    fn test_has_method_syntax_ansible_with_methods() {
        let ansible = CompatMode::Ansible(AnsibleMode::with_methods());
        assert!(ansible.has_method_syntax());
    }

    #[test]
    fn test_has_method_syntax_ansible_filter_only() {
        let ansible = CompatMode::Ansible(AnsibleMode::filter_only());
        assert!(!ansible.has_method_syntax());
    }

    #[test]
    fn test_ansible_mode_default() {
        let mode = AnsibleMode::default();
        assert!(mode.method_syntax);
        assert!(mode.enable_validation);
        assert!(mode.inventory_source.is_none());
    }

    #[test]
    fn test_ansible_mode_with_inventory() {
        let mode = AnsibleMode::default().with_inventory_file("/etc/ansible/hosts");
        assert!(mode.inventory_source.is_some());
    }

    #[test]
    fn test_ansible_mode_as_ansible() {
        let mode = CompatMode::Ansible(AnsibleMode::default());
        assert!(mode.as_ansible().is_some());

        let mode = CompatMode::Jinja2;
        assert!(mode.as_ansible().is_none());
    }

    #[test]
    fn test_django_mode_default() {
        let mode = DjangoMode::default();
        assert!(mode.app_directories.is_empty());
        assert_eq!(mode.timezone, "UTC");
        assert_eq!(mode.locale, "en-US");
        assert!(!mode.enable_url_resolution);
        assert!(mode.html_auto_escape());
    }

    #[test]
    fn test_django_mode_builder() {
        let mode = DjangoMode::default()
            .with_app_directory("/app/myapp")
            .with_timezone("Europe/London")
            .with_locale("en-GB")
            .with_url_resolution(true);
        assert_eq!(mode.app_directories.len(), 1);
        assert_eq!(mode.timezone, "Europe/London");
        assert_eq!(mode.locale, "en-GB");
        assert!(mode.enable_url_resolution);
    }

    #[test]
    fn test_django_mode_multiple_app_dirs() {
        let mode = DjangoMode::default()
            .with_app_directory("/app/accounts")
            .with_app_directory("/app/posts")
            .with_app_directory("/app/common");
        assert_eq!(mode.app_directories.len(), 3);
    }

    #[test]
    fn test_compat_mode_is_django() {
        let mode = CompatMode::Django(DjangoMode::default());
        assert!(mode.is_django());
        assert!(!mode.is_jinja2());
        assert!(!mode.is_ansible());
    }

    #[test]
    fn test_compat_mode_as_django() {
        let mode = CompatMode::Django(DjangoMode::default());
        assert!(mode.as_django().is_some());

        let mode = CompatMode::Jinja2;
        assert!(mode.as_django().is_none());
    }

    #[test]
    fn test_django_mode_no_method_syntax() {
        let mode = CompatMode::Django(DjangoMode::default());
        assert!(!mode.has_method_syntax());
    }
}
