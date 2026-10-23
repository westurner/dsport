//! `sphinxdocrs::autodoc_runtime` — **H9a**: runtime-import bridge for
//! `sphinx.ext.autodoc`/`sphinx.ext.autosummary.generate`.
//!
//! [`crate::autodoc`]'s `ruff_python_ast`-based extraction is purely static:
//! it can never see decorated/dynamically-created members, C-extension
//! modules, or an `__all__` computed at import time. This module closes that
//! gap by importing the target module *for real* through PyO3 and
//! introspecting it via Python's own `inspect` module — mirroring what
//! upstream `autodoc` actually does.
//!
//! Every public entry point returns [`PyResult`]; callers are expected to
//! fall back to the static `crate::autodoc` path when import/introspection
//! fails (module not found, a genuinely missing dependency not covered by
//! `autodoc_mock_imports`, etc.) rather than treat that as a hard error —
//! see [`crate::autodoc::document_module_auto`] and
//! [`crate::autogen::generate::StubContext::from_entry_runtime`].
//!
//! ## Security
//!
//! Unlike the rest of `crate::autodoc`, this module **does** execute
//! arbitrary Python code: importing the target module runs its top-level
//! statements, exactly as upstream `autodoc` does (and must, to support
//! runtime-only members). This is an explicit, opt-in bridge, not the
//! default code path this crate otherwise favors for parsing untrusted RST.

use std::collections::HashSet;

use pyo3::prelude::*;
use pyo3::types::PyDict;

fn python_runtime_lock(py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
    let sys = py.import("sys")?;
    let threading = py.import("threading")?;
    let init_lock = threading.getattr("_active_limbo_lock")?;
    init_lock.call_method0("acquire")?;

    let result = match sys.getattr("_sphinxdocrs_autodoc_runtime_lock") {
        Ok(lock) => Ok(lock),
        Err(_) => {
            let lock = threading.getattr("RLock")?.call0()?;
            sys.setattr("_sphinxdocrs_autodoc_runtime_lock", &lock)?;
            Ok(lock)
        }
    };
    let release_result = init_lock.call_method0("release");
    match (result, release_result) {
        (Ok(lock), Ok(_)) => Ok(lock),
        (Err(error), Ok(_)) => Err(error),
        (Ok(_), Err(error)) | (Err(_), Err(error)) => Err(error.into()),
    }
}

fn with_python_runtime_lock<T>(
    py: Python<'_>,
    operation: impl FnOnce() -> PyResult<T>,
) -> PyResult<T> {
    let lock = python_runtime_lock(py)?;
    lock.call_method0("acquire")?;
    let result = operation();
    let release_result = lock.call_method0("release");
    match (result, release_result) {
        (Ok(value), Ok(_)) => Ok(value),
        (Err(error), Ok(_)) => Err(error),
        (Ok(_), Err(error)) | (Err(_), Err(error)) => Err(error.into()),
    }
}

/// Coarse kind of a runtime-introspected member, for template/rendering
/// dispatch downstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberKind {
    Module,
    Class,
    Exception,
    Function,
    Method,
    Property,
    Attribute,
}

/// A single introspected module/class member.
#[derive(Debug, Clone)]
pub struct MemberInfo {
    pub name: String,
    pub kind: MemberKind,
    /// `str(inspect.signature(obj))`, when available (functions/methods only).
    pub signature: Option<String>,
    /// `inspect.getdoc(obj)` — already dedented, unlike `obj.__doc__`.
    pub docstring: Option<String>,
}

impl MemberInfo {
    fn is_public(&self) -> bool {
        !self.name.starts_with('_')
    }
}

/// Result of introspecting one module through the runtime bridge.
#[derive(Debug, Clone, Default)]
pub struct ModuleIntrospection {
    pub name: String,
    pub docstring: Option<String>,
    /// The module's `__all__`, if it defines one.
    pub all: Option<Vec<String>>,
    pub members: Vec<MemberInfo>,
}

impl ModuleIntrospection {
    fn names_of(&self, kind: MemberKind, public_only: bool) -> Vec<String> {
        let mut names: Vec<String> = self
            .members
            .iter()
            .filter(|m| m.kind == kind && (!public_only || self.is_documented(m)))
            .map(|m| m.name.clone())
            .collect();
        names.sort();
        names
    }

    /// Whether `all` restricts the public surface, and if so, whether `m` is
    /// in it. With no `__all__`, falls back to the plain `_`-prefix rule.
    fn is_documented(&self, m: &MemberInfo) -> bool {
        match &self.all {
            Some(names) => names.iter().any(|n| n == &m.name),
            None => m.is_public(),
        }
    }

    pub fn functions(&self) -> Vec<String> {
        self.names_of(MemberKind::Function, true)
    }
    pub fn classes(&self) -> Vec<String> {
        self.names_of(MemberKind::Class, true)
    }
    pub fn exceptions(&self) -> Vec<String> {
        self.names_of(MemberKind::Exception, true)
    }
    pub fn modules(&self) -> Vec<String> {
        self.names_of(MemberKind::Module, true)
    }
    pub fn attributes(&self) -> Vec<String> {
        self.names_of(MemberKind::Attribute, true)
    }
    pub fn all_functions(&self) -> Vec<String> {
        self.names_of(MemberKind::Function, false)
    }
    pub fn all_classes(&self) -> Vec<String> {
        self.names_of(MemberKind::Class, false)
    }
    pub fn all_exceptions(&self) -> Vec<String> {
        self.names_of(MemberKind::Exception, false)
    }
    pub fn all_modules(&self) -> Vec<String> {
        self.names_of(MemberKind::Module, false)
    }
    pub fn all_attributes(&self) -> Vec<String> {
        self.names_of(MemberKind::Attribute, false)
    }
}

/// Result of introspecting one class through the runtime bridge.
#[derive(Debug, Clone, Default)]
pub struct ClassIntrospection {
    pub name: String,
    pub docstring: Option<String>,
    /// `str(inspect.signature(class))`, excluding the implicit `self` from
    /// `__init__` and matching the signature shown by Sphinx autodoc.
    pub signature: Option<String>,
    pub members: Vec<MemberInfo>,
}

impl ClassIntrospection {
    pub fn methods(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .members
            .iter()
            .filter(|m| m.kind == MemberKind::Method && m.is_public())
            .map(|m| m.name.clone())
            .collect();
        names.sort();
        names
    }

    pub fn attributes(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .members
            .iter()
            .filter(|m| {
                matches!(m.kind, MemberKind::Attribute | MemberKind::Property) && m.is_public()
            })
            .map(|m| m.name.clone())
            .collect();
        names.sort();
        names
    }
}

/// Install `unittest.mock.MagicMock` stand-ins into `sys.modules` for every
/// name in `mock_imports` that cannot genuinely be imported.
///
/// Mirrors `sphinx.ext.autodoc.mock.mock()`'s purpose (`autodoc_mock_imports`):
/// lets a module that does `import numpy` be introspected even when `numpy`
/// isn't installed, by making the import succeed with an object that accepts
/// any attribute access/call. Every dotted parent prefix is mocked too (e.g.
/// mocking `a.b.c` also mocks `a` and `a.b` if not already importable), since
/// `import a.b.c` requires each ancestor package to already be in
/// `sys.modules`.
///
/// A name that *is* genuinely importable is left alone (its real module is
/// used, matching upstream's `ismock()`-aware skip).
pub fn install_mock_imports(py: Python<'_>, mock_imports: &[String]) -> PyResult<()> {
    with_python_runtime_lock(py, || install_mock_imports_unlocked(py, mock_imports))
}

fn install_mock_imports_unlocked(py: Python<'_>, mock_imports: &[String]) -> PyResult<()> {
    if mock_imports.is_empty() {
        return Ok(());
    }
    let sys = py.import("sys")?;
    let modules = sys.getattr("modules")?;
    let modules: &pyo3::Bound<'_, PyDict> = modules.cast()?;
    let mock_mod = py.import("unittest.mock")?;
    let magicmock = mock_mod.getattr("MagicMock")?;

    for name in mock_imports {
        if py.import(name.as_str()).is_ok() {
            continue;
        }
        let mut prefix = String::new();
        for part in name.split('.') {
            if !prefix.is_empty() {
                prefix.push('.');
            }
            prefix.push_str(part);
            if modules.get_item(&prefix)?.is_none() {
                let m = magicmock.call0()?;
                modules.set_item(&prefix, m)?;
            }
        }
    }
    Ok(())
}

/// Import `module_name` and introspect its top-level members.
///
/// `mock_imports` (from the `autodoc_mock_imports` config value) is applied
/// first via [`install_mock_imports`], so an otherwise-unimportable module
/// can still be introspected.
///
/// Returns `Err` (never panics) when the module cannot be imported even
/// with mocking applied — callers should treat that as "fall back to the
/// static path", not a hard failure.
pub fn introspect_module(
    module_name: &str,
    mock_imports: &[String],
) -> PyResult<ModuleIntrospection> {
    Python::attach(|py| {
        let cwd = std::env::current_dir().unwrap_or_default();
        crate::config::add_vendored_import_paths(py, &cwd)?;
        with_python_runtime_lock(py, || {
            install_mock_imports_unlocked(py, mock_imports)?;
            let module = py.import(module_name)?;
            let inspect = py.import("inspect")?;

            let docstring = inspect
                .call_method1("getdoc", (&module,))?
                .extract::<Option<String>>()?;

            let all: Option<Vec<String>> = module
                .getattr("__all__")
                .ok()
                .and_then(|a| a.extract::<Vec<String>>().ok());

            let submodule_names: HashSet<String> = {
                let sys = py.import("sys")?;
                let modules = sys.getattr("modules")?;
                let modules: &pyo3::Bound<'_, PyDict> = modules.cast()?;
                let prefix = format!("{module_name}.");
                let mut set = HashSet::new();
                for key in modules.keys() {
                    if let Ok(k) = key.extract::<String>() {
                        if let Some(rest) = k.strip_prefix(&prefix) {
                            if !rest.contains('.') {
                                set.insert(rest.to_string());
                            }
                        }
                    }
                }
                set
            };

            let mut members = Vec::new();
            let member_pairs = inspect.call_method1("getmembers", (&module,))?;
            for pair in member_pairs.try_iter()? {
                let pair = pair?;
                let name: String = pair.get_item(0)?.extract()?;
                if name.starts_with("__") && name.ends_with("__") {
                    continue; // dunder module attributes (e.g. __name__, __file__)
                }
                let obj = pair.get_item(1)?;

                let kind = classify(py, &inspect, &obj, &submodule_names, &name)?;
                let Some(kind) = kind else { continue };

                let signature = signature_of(&inspect, &obj);
                let member_doc = inspect
                    .call_method1("getdoc", (&obj,))?
                    .extract::<Option<String>>()
                    .unwrap_or(None);

                members.push(MemberInfo {
                    name,
                    kind,
                    signature,
                    docstring: member_doc,
                });
            }

            Ok(ModuleIntrospection {
                name: module_name.to_string(),
                docstring,
                all,
                members,
            })
        })
    })
}

/// Import `module_name`, resolve `class_name` as an attribute of it, and
/// introspect the class's own members (methods/attributes/properties).
pub fn introspect_class(
    module_name: &str,
    class_name: &str,
    mock_imports: &[String],
) -> PyResult<ClassIntrospection> {
    Python::attach(|py| {
        let cwd = std::env::current_dir().unwrap_or_default();
        crate::config::add_vendored_import_paths(py, &cwd)?;
        with_python_runtime_lock(py, || {
            install_mock_imports_unlocked(py, mock_imports)?;
            let module = py.import(module_name)?;
            let class = module.getattr(class_name)?;
            let inspect = py.import("inspect")?;

            let docstring = inspect
                .call_method1("getdoc", (&class,))?
                .extract::<Option<String>>()?;
            let signature = signature_of(&inspect, &class);

            let mut members = Vec::new();
            let member_pairs = inspect.call_method1("getmembers", (&class,))?;
            for pair in member_pairs.try_iter()? {
                let pair = pair?;
                let name: String = pair.get_item(0)?.extract()?;
                if name.starts_with("__") && name.ends_with("__") && name != "__init__" {
                    continue;
                }
                let obj = pair.get_item(1)?;

                let kind = if inspect
                    .call_method1("isroutine", (&obj,))?
                    .extract::<bool>()?
                {
                    MemberKind::Method
                } else if is_property(py, &obj)? {
                    MemberKind::Property
                } else {
                    MemberKind::Attribute
                };

                let signature = if kind == MemberKind::Method {
                    signature_of(&inspect, &obj)
                } else {
                    None
                };
                let member_doc = inspect
                    .call_method1("getdoc", (&obj,))?
                    .extract::<Option<String>>()
                    .unwrap_or(None);

                members.push(MemberInfo {
                    name,
                    kind,
                    signature,
                    docstring: member_doc,
                });
            }

            Ok(ClassIntrospection {
                name: class_name.to_string(),
                docstring,
                signature,
                members,
            })
        })
    })
}

/// Return the source comment documentation for a class attribute, searching
/// the class MRO the way autodoc resolves inherited attribute documentation.
pub fn attribute_doc(
    module_name: &str,
    class_name: &str,
    attribute_name: &str,
    mock_imports: &[String],
) -> PyResult<Option<String>> {
    Python::attach(|py| {
        let cwd = std::env::current_dir().unwrap_or_default();
        crate::config::add_vendored_import_paths(py, &cwd)?;
        with_python_runtime_lock(py, || {
            install_mock_imports_unlocked(py, mock_imports)?;
            let module = py.import(module_name)?;
            let class = module.getattr(class_name)?;
            let analyzer_type = py.import("sphinx.pycode")?.getattr("ModuleAnalyzer")?;

            for base in class.getattr("__mro__")?.try_iter()? {
                let base = base?;
                let base_module: String = base.getattr("__module__")?.extract()?;
                let base_name: String = base.getattr("__name__")?.extract()?;
                let Ok(analyzer) = analyzer_type.call_method1("for_module", (&base_module,))
                else {
                    continue;
                };
                let docs = analyzer.call_method0("find_attr_docs")?;
                let lines = docs.call_method1("get", ((base_name, attribute_name),))?;
                if !lines.is_none() {
                    let text = lines.extract::<Vec<String>>()?.join("\n");
                    let text = text.trim();
                    if !text.is_empty() {
                        return Ok(Some(text.to_string()));
                    }
                }
            }
            Ok(None)
        })
    })
}

/// Return a deterministic literal value for simple class attributes.
/// Complex objects are omitted rather than rendered with process-specific reprs.
pub fn attribute_value(
    module_name: &str,
    class_name: &str,
    attribute_name: &str,
    mock_imports: &[String],
) -> PyResult<Option<String>> {
    Python::attach(|py| {
        let cwd = std::env::current_dir().unwrap_or_default();
        crate::config::add_vendored_import_paths(py, &cwd)?;
        with_python_runtime_lock(py, || {
            install_mock_imports_unlocked(py, mock_imports)?;
            let module = py.import(module_name)?;
            let class = module.getattr(class_name)?;
            let inspect = py.import("inspect")?;
            let value = match inspect.call_method1("getattr_static", (&class, attribute_name)) {
                Ok(value) => value,
                Err(error) if error.is_instance_of::<pyo3::exceptions::PyAttributeError>(py) => {
                    return Ok(None);
                }
                Err(error) => return Err(error),
            };

            if value.is_none() {
                return Ok(Some("None".to_string()));
            }
            if let Ok(value) = value.extract::<bool>() {
                return Ok(Some(if value { "True" } else { "False" }.to_string()));
            }
            if let Ok(value) = value.extract::<i64>() {
                return Ok(Some(value.to_string()));
            }
            if value.is_instance_of::<pyo3::types::PyString>() {
                return Ok(Some(value.repr()?.extract::<String>()?));
            }
            Ok(None)
        })
    })
}

fn is_property(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
    let builtins = py.import("builtins")?;
    let property_ty = builtins.getattr("property")?;
    obj.is_instance(&property_ty)
}

fn signature_of(inspect: &Bound<'_, PyAny>, obj: &Bound<'_, PyAny>) -> Option<String> {
    let sig = inspect.call_method1("signature", (obj,)).ok()?;
    let s: String = sig.call_method0("__str__").ok()?.extract().ok()?;
    Some(s)
}

fn classify(
    py: Python<'_>,
    inspect: &Bound<'_, PyAny>,
    obj: &Bound<'_, PyAny>,
    submodule_names: &HashSet<String>,
    name: &str,
) -> PyResult<Option<MemberKind>> {
    if submodule_names.contains(name) && inspect.call_method1("ismodule", (obj,))?.extract()? {
        return Ok(Some(MemberKind::Module));
    }
    if inspect.call_method1("isclass", (obj,))?.extract::<bool>()? {
        let builtins = py.import("builtins")?;
        let base_exception = builtins.getattr("BaseException")?;
        if obj.is_subclass_of(&base_exception).unwrap_or(false) {
            return Ok(Some(MemberKind::Exception));
        }
        return Ok(Some(MemberKind::Class));
    }
    if inspect
        .call_method1("isfunction", (obj,))?
        .extract::<bool>()?
        || inspect
            .call_method1("isbuiltin", (obj,))?
            .extract::<bool>()?
    {
        return Ok(Some(MemberKind::Function));
    }
    if inspect
        .call_method1("ismodule", (obj,))?
        .extract::<bool>()?
    {
        // A re-exported module that isn't a direct submodule (e.g.
        // `import os` inside the target module) — not a documentable member.
        return Ok(None);
    }
    // Anything else callable-ish gets skipped; simple values are attributes.
    Ok(Some(MemberKind::Attribute))
}

trait PyAnyIsSubclassOf {
    fn is_subclass_of(&self, other: &Bound<'_, PyAny>) -> PyResult<bool>;
}

impl PyAnyIsSubclassOf for Bound<'_, PyAny> {
    fn is_subclass_of(&self, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        let builtins = other.py().import("builtins")?;
        let issubclass = builtins.getattr("issubclass")?;
        issubclass.call1((self, other))?.extract()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn introspects_stdlib_module() {
        let result = introspect_module("json", &[]);
        let modinfo = match result {
            Ok(m) => m,
            Err(e) => panic!("expected json to introspect cleanly: {e}"),
        };
        assert_eq!(modinfo.name, "json");
        assert!(modinfo.functions().contains(&"dumps".to_string()));
        assert!(modinfo.functions().contains(&"loads".to_string()));
    }

    #[test]
    fn introspects_class_methods() {
        let result = introspect_class("collections", "OrderedDict", &[]);
        let classinfo = match result {
            Ok(c) => c,
            Err(e) => panic!("expected OrderedDict to introspect cleanly: {e}"),
        };
        assert!(classinfo.methods().contains(&"popitem".to_string()));

        let decoder = introspect_class("json", "JSONDecoder", &[])
            .expect("expected JSONDecoder to introspect cleanly");
        assert!(
            decoder
                .signature
                .as_deref()
                .is_some_and(|signature| signature.starts_with("(*, object_hook="))
        );
    }

    #[test]
    fn finds_inherited_attribute_source_documentation() {
        let result = attribute_doc(
            "sphinx.builders.html",
            "StandaloneHTMLBuilder",
            "name",
            &[],
        )
        .expect("attribute documentation lookup should succeed");

        assert!(result.is_some_and(|text| text.contains("select builders on the command line")));
    }

    #[test]
    fn extracts_simple_class_attribute_values() {
        let required = attribute_value(
            "docutils.parsers.rst",
            "Directive",
            "required_arguments",
            &[],
        )
        .unwrap();
        let whitespace = attribute_value(
            "docutils.parsers.rst",
            "Directive",
            "final_argument_whitespace",
            &[],
        )
        .unwrap();
        let option_spec = attribute_value(
            "docutils.parsers.rst",
            "Directive",
            "option_spec",
            &[],
        )
        .unwrap();

        assert_eq!(required.as_deref(), Some("0"));
        assert_eq!(whitespace.as_deref(), Some("False"));
        assert_eq!(option_spec.as_deref(), Some("None"));
    }

    #[test]
    fn missing_module_is_an_error_not_a_panic() {
        let result = introspect_module("this_module_does_not_exist_xyz", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn mock_import_lets_missing_dependency_resolve() {
        Python::attach(|py| {
            install_mock_imports(py, &["totally_fake_dependency_xyz".to_string()])
                .expect("mock install should not fail");
            let imported = py.import("totally_fake_dependency_xyz");
            assert!(imported.is_ok(), "mocked module should now import");
        });
    }

    #[test]
    fn mock_import_mocks_parent_packages_too() {
        Python::attach(|py| {
            install_mock_imports(py, &["fake_pkg_xyz.sub.leaf".to_string()])
                .expect("mock install should not fail");
            assert!(py.import("fake_pkg_xyz").is_ok());
            assert!(py.import("fake_pkg_xyz.sub").is_ok());
            assert!(py.import("fake_pkg_xyz.sub.leaf").is_ok());
        });
    }
}
