//! `jinja2rs::compat` — Jinja2 vs minijinja compatibility modes.
//!
//! This module provides configuration for three compatibility modes:
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
            method_syntax: true,      // Default to Jinja2-style methods
            enable_validation: true,   // Validate by default
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

    /// Check if this mode supports Python method syntax.
    ///
    /// Returns true for:
    /// - Jinja2 mode (always has method syntax)
    /// - Ansible mode with `method_syntax: true`
    /// - Kubernetes mode with `method_syntax: true`
    pub fn has_method_syntax(&self) -> bool {
        match self {
            CompatMode::Jinja2 => true,
            CompatMode::Minijinja => false,
            CompatMode::Ansible(cfg) => cfg.method_syntax,
            CompatMode::Kubernetes(cfg) => cfg.method_syntax,
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
        let mode = AnsibleMode::default()
            .with_inventory_file("/etc/ansible/hosts");
        assert!(mode.inventory_source.is_some());
    }

    #[test]
    fn test_ansible_mode_as_ansible() {
        let mode = CompatMode::Ansible(AnsibleMode::default());
        assert!(mode.as_ansible().is_some());

        let mode = CompatMode::Jinja2;
        assert!(mode.as_ansible().is_none());
    }
}
