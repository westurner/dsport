//! `jinja2rs::environment` — core template environment.
//!
//! Wraps [`minijinja::Environment`] and registers the standard Jinja2
//! built-in filters plus the Sphinx-specific extras defined in
//! [`crate::filters`] and [`crate::globals`].

use std::path::PathBuf;
use std::sync::Arc;

use minijinja::Environment as MiniEnv;
use minijinja::Value;
use serde::Serialize;

use crate::compat::CompatMode;
use crate::errors::Jinja2Error;
use crate::filters;
use crate::globals;
use crate::i18n;
use crate::loaders::FileSystemLoader;

/// Template handle returned by [`Environment::get_template`].
pub struct Template<'env> {
    inner: minijinja::Template<'env, 'env>,
}

impl<'env> Template<'env> {
    /// Render the template with the given context value.
    pub fn render<S: Serialize>(&self, ctx: S) -> Result<String, Jinja2Error> {
        Ok(self.inner.render(ctx)?)
    }
}

/// A Jinja2-compatible template environment backed by minijinja.
///
/// Mirrors `jinja2.Environment`:
/// - Manages a collection of templates (string or file-loaded).
/// - Maintains filter, test, and global registries.
/// - Configures auto-escaping by file extension (`.html` → HTML-safe by default).
pub struct Environment {
    pub(crate) inner: MiniEnv<'static>,
    pub(crate) search_paths: Vec<PathBuf>,
}

impl Environment {
    /// Create a new environment with Jinja2 defaults:
    /// - All built-in minijinja filters and tests.
    /// - Sphinx utility filters (`tobool`, `toint`, `todim`, `slice_index`).
    /// - Phase 3 filters: `indent`, `wordwrap`, `xmlattr`, `urlencode`, `filesizeformat`.
    /// - Phase 4 globals: `debug`, `cycler`, `joiner`, `lipsum`.
    /// - Sphinx globals (`accesskey`, `idgen`, `warning`).
    /// - Auto-escape enabled for `.html`, `.xml`, `.htm`.
    pub fn new() -> Self {
        let mut env = MiniEnv::new();

        // Sphinx-specific filters
        env.add_filter("tobool", filters::tobool);
        env.add_filter("toint", filters::toint);
        env.add_filter("todim", filters::todim);
        env.add_filter("filesizeformat", filters::filesizeformat);
        env.add_filter("slice_index", filters::slice_index);

        // Phase 3 filters
        env.add_filter("indent", filters::indent);
        env.add_filter("wordwrap", filters::wordwrap);
        env.add_filter("xmlattr", filters::xmlattr);
        env.add_filter("urlencode", filters::urlencode);

        // Sphinx-specific and Phase 4 globals
        env.add_global("idgen", minijinja::Value::from_object(globals::IdGen::new()));
        env.add_global("accesskey", minijinja::Value::from_object(globals::AccessKey::new()));
        env.add_global("debug", minijinja::Value::from_object(globals::Debug::new()));
        
        // Phase 4 global factories
        env.add_global("cycler", minijinja::Value::from_object(globals::CyclerFactory::new()));
        env.add_global("joiner", minijinja::Value::from_object(globals::JoinerFactory::new()));
        env.add_global("lipsum", minijinja::Value::from_object(globals::LipsumFactory::new()));

        Self {
            inner: env,
            search_paths: Vec::new(),
        }
    }

    /// Create an environment with a filesystem loader rooted at `path`.
    pub fn with_loader(path: impl Into<PathBuf>) -> Self {
        let mut env = Self::new();
        env.search_paths.push(path.into());
        let paths: Arc<Vec<PathBuf>> = Arc::new(env.search_paths.clone());
        env.inner.set_loader(move |name| {
            Ok(FileSystemLoader::load_source(&paths, name))
        });
        env
    }

    /// Add a named template from a string (mirrors `env.add_template()`).
    pub fn add_template(&mut self, name: &'static str, source: &'static str) -> Result<(), Jinja2Error> {
        self.inner.add_template(name, source)?;
        Ok(())
    }

    /// Retrieve a template by name.
    pub fn get_template(&self, name: &str) -> Result<Template<'_>, Jinja2Error> {
        let t = self.inner.get_template(name).map_err(|e| {
            if e.kind() == minijinja::ErrorKind::TemplateNotFound {
                Jinja2Error::TemplateNotFound(name.to_owned())
            } else {
                Jinja2Error::Render(e)
            }
        })?;
        Ok(Template { inner: t })
    }

    /// Render a one-off template string without registering it.
    pub fn render_str<S: Serialize>(&self, source: &str, ctx: S) -> Result<String, Jinja2Error> {
        Ok(self.inner.render_str(source, ctx)?)
    }

    /// Add a custom filter function (mirrors `env.filters[name] = fn`).
    pub fn add_filter<F>(&mut self, name: &'static str, f: F)
    where
        F: minijinja::filters::Filter<minijinja::Value, (minijinja::Value,)> + Send + Sync + 'static,
    {
        self.inner.add_filter(name, f);
    }

    /// Add a global variable (mirrors `env.globals[name] = value`).
    pub fn add_global(&mut self, name: &'static str, value: impl Into<minijinja::Value>) {
        self.inner.add_global(name, value);
    }

    /// Install i18n (gettext/ngettext) support.
    ///
    /// Registers `gettext` and `ngettext` globals for template translation.
    /// The provider can be configured with translation dictionaries.
    ///
    /// Example:
    /// ```ignore
    /// let provider = i18n::I18nProvider::new();
    /// env.install_gettext(provider);
    /// ```
    pub fn install_gettext(&mut self, provider: i18n::I18nProvider) {
        let gettext_global = i18n::GettextGlobal::new(provider.clone());
        let ngettext_global = i18n::NgettextGlobal::new(provider);

        self.inner
            .add_global("gettext", minijinja::Value::from_object(gettext_global));
        self.inner
            .add_global("ngettext", minijinja::Value::from_object(ngettext_global));
    }

    /// Enable Jinja2 compatibility mode (default).
    ///
    /// This mode adds Python method syntax support via the `minijinja-contrib`
    /// pycompat module, making templates written for Python Jinja2 work without
    /// modification:
    ///
    /// - `dict.items()`, `dict.values()`, `dict.keys()`, `dict.get()`
    /// - `str.upper()`, `str.lower()`, `str.split()`, `str.format()`, etc.
    /// - `list.count()`
    ///
    /// Example:
    /// ```ignore
    /// let mut env = Environment::new();
    /// env.set_compat_mode(CompatMode::Jinja2);  // Enable Python methods
    /// env.render_str("{{ user.items() }}", ctx).unwrap();
    /// ```
    pub fn set_compat_mode(&mut self, mode: CompatMode) {
        match mode {
            CompatMode::Jinja2 => self.enable_jinja2_compat(),
            CompatMode::Minijinja => self.enable_minijinja_compat(),
            CompatMode::Ansible(cfg) => {
                // Register Ansible filters
                self.register_ansible_filters();
                
                // Set method syntax based on Ansible mode configuration
                if cfg.method_syntax {
                    self.enable_jinja2_compat();
                } else {
                    self.enable_minijinja_compat();
                }
                
                // TODO: Add inventory support when cfg.inventory_source is Some
                // TODO: Add YAML validation when cfg.enable_validation is true
            }
            CompatMode::Kubernetes(cfg) => {
                // Register Kubernetes filters
                self.register_kubernetes_filters();
                
                // Set method syntax based on Kubernetes mode configuration
                if cfg.method_syntax {
                    self.enable_jinja2_compat();
                } else {
                    self.enable_minijinja_compat();
                }
                
                // TODO: Add manifest support when cfg.manifest_source is Some
                // TODO: Add YAML validation when cfg.enable_validation is true
            }
        }
    }

    /// Register Ansible-specific filters.
    fn register_ansible_filters(&mut self) {
        // to_nice_json returns Result - wrap in a filter-compatible closure
        self.inner.add_filter("to_nice_json", |val: Value| {
            crate::ansible_filters::to_nice_json(val)
                .unwrap_or_else(|_| Value::from(""))
        });
        
        // from_json returns Result - wrap in a filter-compatible closure
        self.inner.add_filter("from_json", |val: Value| {
            crate::ansible_filters::from_json(val)
                .unwrap_or_else(|_| Value::from(""))
        });
        
        self.add_filter("quote", crate::ansible_filters::quote);
        
        // path_join takes 2 arguments - wrap in closure accepting 2 args
        self.inner.add_filter("path_join", |val: Value, other: Value| {
            crate::ansible_filters::path_join(val, other)
        });
        
        // combine takes 2 arguments - wrap in closure accepting 2 args
        self.inner.add_filter("combine", |val: Value, other: Value| {
            crate::ansible_filters::combine(val, other)
                .unwrap_or_else(|_| Value::from(""))
        });
        
        // regex_search takes 2 arguments - wrap in closure accepting 2 args
        self.inner.add_filter("regex_search", |val: Value, pattern: Value| {
            crate::ansible_filters::regex_search(val, pattern)
        });
        
        // regex_replace takes 3 arguments - wrap in closure accepting 3 args
        self.inner.add_filter("regex_replace", |val: Value, pattern: Value, replacement: Value| {
            crate::ansible_filters::regex_replace(val, pattern, replacement)
        });
        
        // regex_findall takes 2 arguments - wrap in closure accepting 2 args
        self.inner.add_filter("regex_findall", |val: Value, pattern: Value| {
            crate::ansible_filters::regex_findall(val, pattern)
        });
        
        self.add_filter("to_nice_yaml", crate::ansible_filters::to_nice_yaml);
        self.add_filter("from_yaml", crate::ansible_filters::from_yaml);
    }

    /// Register Kubernetes-specific filters.
    fn register_kubernetes_filters(&mut self) {
        // Workload introspection filters
        self.add_filter("replicas", crate::kubernetes_filters::replicas);
        self.add_filter("container_image", crate::kubernetes_filters::container_image);
        
        // Metadata accessors
        self.inner.add_filter("label", |val: Value, key: Value| {
            crate::kubernetes_filters::label(val, key)
        });
        
        self.inner.add_filter("annotation", |val: Value, key: Value| {
            crate::kubernetes_filters::annotation(val, key)
        });
        
        // Resource kind/name/namespace accessors
        self.add_filter("k8s_kind", crate::kubernetes_filters::k8s_kind);
        self.add_filter("k8s_name", crate::kubernetes_filters::k8s_name);
        self.add_filter("k8s_namespace", crate::kubernetes_filters::k8s_namespace);
        self.add_filter("k8s_labels", crate::kubernetes_filters::k8s_labels);
        self.add_filter("k8s_annotations", crate::kubernetes_filters::k8s_annotations);
        
        // Resource filtering and checking
        self.inner.add_filter("k8s_in_namespace", |val: Value, namespace: Value| {
            crate::kubernetes_filters::k8s_in_namespace(val, namespace)
        });
        
        self.inner.add_filter("k8s_has_label", |val: Value, key: Value, expected_val: Value| {
            crate::kubernetes_filters::k8s_has_label(val, key, expected_val)
        });
    }

    /// Enable Jinja2 compatibility mode explicitly.
    ///
    /// This is the default and enables Python method syntax.
    /// Use this if you need to switch back from minijinja mode.
    pub fn enable_jinja2_compat(&mut self) {
        self.inner
            .set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);
    }

    /// Enable minijinja compatibility mode (strict).
    ///
    /// This mode disables Python method syntax and uses filter-based approach only.
    /// Methods like `.items()` are not available; use `|items` filter instead.
    ///
    /// This mode is more efficient and encourages explicit filter-based syntax.
    ///
    /// Example:
    /// ```ignore
    /// let mut env = Environment::new();
    /// env.enable_minijinja_compat();  // Disable Python methods
    /// // env.render_str("{{ user.items() }}", ctx) -> Error!
    /// env.render_str("{{ user | items }}", ctx).unwrap();  // OK
    /// ```
    pub fn enable_minijinja_compat(&mut self) {
        // Disable unknown method callback (no-op by default in minijinja)
        // This is a no-op since minijinja doesn't support unknown methods by default
    }
}

