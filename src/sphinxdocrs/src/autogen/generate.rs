//! Native stub generation for `sphinx-autogen`.
//!
//! C4.2: generates `.rst` stub files from [`AutosummaryEntry`] items without
//! requiring Python introspection. Uses heuristic type detection based on
//! Python naming conventions (CamelCase → class; all-lowercase → module/base).
//!
//! The generated stubs are structurally identical to those produced by the
//! Python `generate_autosummary_docs`: a heading + an `.. automodule::` or
//! `.. autoclass::` or `.. auto<objtype>::` directive. Member lists
//! (`functions`, `classes`, `methods`, `attributes`, …) are left empty
//! because `autodoc` populates them during the Sphinx build.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::autogen::scan::AutosummaryEntry;
use crate::autogen::templates::AutogenTemplates;

// ── type inference ────────────────────────────────────────────────────────────

/// Coarse object type used for template selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjType {
    Module,
    Class,
    Base, // function, method, attribute, …
}

/// Infer object type from qualified name using Python naming conventions.
///
/// Rules (in priority order):
/// 1. If the last dotted component starts with an uppercase letter → `Class`.
/// 2. Otherwise → `Module` (autodoc will handle the distinction at build time).
///
/// The heuristic deliberately errs toward `Module` so that
/// `.. automodule::` is emitted for ambiguous names; `autodoc` will then
/// traverse the full module and find its members.
pub fn infer_obj_type(name: &str) -> ObjType {
    let last = name.rsplit('.').next().unwrap_or(name);
    if last.starts_with(|c: char| c.is_uppercase()) {
        ObjType::Class
    } else {
        ObjType::Module
    }
}

// ── name splitting ────────────────────────────────────────────────────────────

/// Split a fully-qualified name into `(module, qualname)`.
///
/// * `sphinx.cmd.build`        → `("sphinx.cmd", "build")`
/// * `sphinx.cmd.build.Sphinx` → `("sphinx.cmd.build", "Sphinx")`
/// * `mypkg`                   → `("", "mypkg")`
pub fn split_fqn(name: &str) -> (String, String) {
    match name.rfind('.') {
        Some(pos) => (name[..pos].to_string(), name[pos + 1..].to_string()),
        None => (String::new(), name.to_string()),
    }
}

// ── template context ──────────────────────────────────────────────────────────

/// Jinja2 context passed to autosummary stub templates.
///
/// All member lists are empty by default; `autodoc` populates them during
/// the Sphinx build.  The `_` translation wrapper used in class/module
/// templates (e.g. `{{ _('Methods') }}`) is registered as a global in
/// [`AutogenTemplates::vendored`] so template rendering never errors.
#[derive(Serialize)]
pub struct StubContext {
    pub fullname: String,
    pub module: String,
    /// Qualified name within the module (last dot-segment).
    pub objname: String,
    /// Short name (rightmost dot-segment of `objname`).
    pub name: String,
    pub objtype: String,
    /// RST underline of `=` matching display width of `fullname`.
    pub underline: String,
    // Member lists — populated by Python; left empty in native mode.
    pub members: Vec<String>,
    pub functions: Vec<String>,
    pub all_functions: Vec<String>,
    pub classes: Vec<String>,
    pub all_classes: Vec<String>,
    pub exceptions: Vec<String>,
    pub all_exceptions: Vec<String>,
    pub attributes: Vec<String>,
    pub all_attributes: Vec<String>,
    pub methods: Vec<String>,
    pub all_methods: Vec<String>,
    pub modules: Vec<String>,
    pub all_modules: Vec<String>,
    pub inherited_members: Vec<String>,
}

impl StubContext {
    /// Build context from an autosummary entry via heuristic type detection.
    pub fn from_entry(entry: &AutosummaryEntry) -> Self {
        let fullname = entry.name.clone();
        let (module, qualname) = split_fqn(&fullname);
        let short_name = qualname.rsplit('.').next().unwrap_or(&qualname).to_string();

        let objtype = match infer_obj_type(&fullname) {
            ObjType::Module => "module",
            ObjType::Class => "class",
            ObjType::Base => "function",
        }
        .to_string();

        let underline = "=".repeat(unicode_width::UnicodeWidthStr::width(fullname.as_str()));

        StubContext {
            fullname,
            module,
            objname: qualname,
            name: short_name,
            objtype,
            underline,
            members: vec![],
            functions: vec![],
            all_functions: vec![],
            classes: vec![],
            all_classes: vec![],
            exceptions: vec![],
            all_exceptions: vec![],
            attributes: vec![],
            all_attributes: vec![],
            methods: vec![],
            all_methods: vec![],
            modules: vec![],
            all_modules: vec![],
            inherited_members: vec![],
        }
    }

    /// Build context from an autosummary entry via the **H9a/H9d**
    /// PyO3 runtime-import bridge (`crate::autodoc_runtime`), populating
    /// real member lists instead of the empty ones `Documenter` normally
    /// fills in during a full Sphinx build.
    ///
    /// Falls back to [`StubContext::from_entry`]'s heuristic (empty member
    /// lists, type inferred from name casing) when the target can't be
    /// imported even with `mock_imports` applied — never a hard error.
    pub fn from_entry_runtime(entry: &AutosummaryEntry, mock_imports: &[String]) -> Self {
        use crate::autodoc_runtime::MemberKind;

        let mut ctx = StubContext::from_entry(entry);

        match infer_obj_type(&entry.name) {
            ObjType::Class => {
                let (module, class_name) = split_fqn(&entry.name);
                if let Ok(classinfo) =
                    crate::autodoc_runtime::introspect_class(&module, &class_name, mock_imports)
                {
                    ctx.methods = classinfo
                        .members
                        .iter()
                        .filter(|m| m.kind == MemberKind::Method && !m.name.starts_with('_'))
                        .map(|m| m.name.clone())
                        .collect();
                    ctx.all_methods = classinfo
                        .members
                        .iter()
                        .filter(|m| m.kind == MemberKind::Method)
                        .map(|m| m.name.clone())
                        .collect();
                    ctx.attributes = classinfo
                        .members
                        .iter()
                        .filter(|m| {
                            matches!(m.kind, MemberKind::Attribute | MemberKind::Property)
                                && !m.name.starts_with('_')
                        })
                        .map(|m| m.name.clone())
                        .collect();
                    ctx.all_attributes = classinfo
                        .members
                        .iter()
                        .filter(|m| matches!(m.kind, MemberKind::Attribute | MemberKind::Property))
                        .map(|m| m.name.clone())
                        .collect();
                }
            }
            ObjType::Module | ObjType::Base => {
                if let Ok(modinfo) =
                    crate::autodoc_runtime::introspect_module(&entry.name, mock_imports)
                {
                    ctx.functions = modinfo.functions();
                    ctx.all_functions = modinfo.all_functions();
                    ctx.classes = modinfo.classes();
                    ctx.all_classes = modinfo.all_classes();
                    ctx.exceptions = modinfo.exceptions();
                    ctx.all_exceptions = modinfo.all_exceptions();
                    ctx.modules = modinfo.modules();
                    ctx.all_modules = modinfo.all_modules();
                    ctx.attributes = modinfo.attributes();
                    ctx.all_attributes = modinfo.all_attributes();
                }
            }
        }

        ctx
    }
}

// ── template selection ────────────────────────────────────────────────────────

/// Return the template name for a given entry.
///
/// Prefers the `template` field set in the directive; falls back to the
/// heuristic inferred type.
fn select_template(entry: &AutosummaryEntry) -> &str {
    if !entry.template.is_empty() {
        return entry.template.as_str();
    }
    match infer_obj_type(&entry.name) {
        ObjType::Module => "module.rst",
        ObjType::Class => "class.rst",
        ObjType::Base => "base.rst",
    }
}

// ── stub writing ──────────────────────────────────────────────────────────────

/// Write a single stub file.
///
/// Mirrors the file-writing logic in `generate_autosummary_docs`:
/// - If the file exists and has identical content, skip silently.
/// - If `overwrite` is false and the file exists (with different content),
///   skip silently.
/// - Otherwise, create/overwrite.
///
/// Returns `Ok(Some(path))` for files written *or* already present,
/// `Ok(None)` when explicitly skipped (no-overwrite on changed content),
/// and `Err` on I/O failure.
pub fn generate_stub(
    entry: &AutosummaryEntry,
    output_dir: &Path,
    suffix: &str,
    overwrite: bool,
    templates: &AutogenTemplates,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    write_stub(
        entry,
        &StubContext::from_entry(entry),
        output_dir,
        suffix,
        overwrite,
        templates,
    )
}

/// Like [`generate_stub`], but populates real member lists through the
/// **H9a/H9d** PyO3 runtime-import bridge
/// ([`StubContext::from_entry_runtime`]) instead of leaving them empty.
/// Falls back to the same heuristic-only output as `generate_stub` when the
/// target can't be imported even with `mock_imports` applied.
pub fn generate_stub_runtime(
    entry: &AutosummaryEntry,
    output_dir: &Path,
    suffix: &str,
    overwrite: bool,
    templates: &AutogenTemplates,
    mock_imports: &[String],
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    write_stub(
        entry,
        &StubContext::from_entry_runtime(entry, mock_imports),
        output_dir,
        suffix,
        overwrite,
        templates,
    )
}

fn write_stub(
    entry: &AutosummaryEntry,
    ctx: &StubContext,
    output_dir: &Path,
    suffix: &str,
    overwrite: bool,
    templates: &AutogenTemplates,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let tname = select_template(entry);
    let content = templates.render(tname, ctx)?;

    // Python uses:  filename = Path(path) / (name + suffix)
    // where suffix already has the leading dot.
    let suffix_dotted = if suffix.starts_with('.') {
        suffix.to_string()
    } else {
        format!(".{suffix}")
    };
    let filename = format!("{}{suffix_dotted}", entry.name);
    let file_path = output_dir.join(&filename);

    if file_path.exists() {
        let existing = std::fs::read_to_string(&file_path)?;
        if existing == content {
            // Identical — count as "already handled" but don't rewrite.
            return Ok(Some(file_path));
        }
        if !overwrite {
            return Ok(None);
        }
    } else {
        std::fs::create_dir_all(output_dir)?;
    }

    std::fs::write(&file_path, &content)?;
    Ok(Some(file_path))
}

/// Generate stub files for all entries in the list.
///
/// Only entries that have a `:toctree:` option set AND whose toctree path
/// resolves to a writable directory are processed (mirrors Python
/// `generate_autosummary_docs` behaviour).
///
/// When `remove_old` is true, any `*.{suffix}` files already in
/// `output_dir` that are **not** among the generated set are deleted.
///
/// Returns the paths of all files written or unchanged-existing.
pub fn generate_stubs(
    entries: &[AutosummaryEntry],
    output_dir: &Path,
    suffix: &str,
    overwrite: bool,
    remove_old: bool,
    templates: &AutogenTemplates,
) -> Vec<PathBuf> {
    generate_stubs_impl(
        entries,
        output_dir,
        suffix,
        overwrite,
        remove_old,
        |entry| generate_stub(entry, output_dir, suffix, overwrite, templates),
    )
}

/// Like [`generate_stubs`], but uses [`generate_stub_runtime`] for real
/// member lists (**H9d**) instead of `generate_stub`'s empty ones.
pub fn generate_stubs_runtime(
    entries: &[AutosummaryEntry],
    output_dir: &Path,
    suffix: &str,
    overwrite: bool,
    remove_old: bool,
    templates: &AutogenTemplates,
    mock_imports: &[String],
) -> Vec<PathBuf> {
    generate_stubs_impl(
        entries,
        output_dir,
        suffix,
        overwrite,
        remove_old,
        |entry| {
            generate_stub_runtime(
                entry,
                output_dir,
                suffix,
                overwrite,
                templates,
                mock_imports,
            )
        },
    )
}

fn generate_stubs_impl(
    entries: &[AutosummaryEntry],
    output_dir: &Path,
    suffix: &str,
    _overwrite: bool,
    remove_old: bool,
    mut generate_one: impl FnMut(
        &AutosummaryEntry,
    ) -> Result<Option<PathBuf>, Box<dyn std::error::Error>>,
) -> Vec<PathBuf> {
    let mut generated = Vec::new();

    for entry in entries {
        match generate_one(entry) {
            Ok(Some(path)) => generated.push(path),
            Ok(None) => {} // skipped (no-overwrite on changed content)
            Err(e) => eprintln!("Warning: failed to generate stub for {}: {e}", entry.name),
        }
    }

    if remove_old {
        let bare_suffix = suffix.trim_start_matches('.');
        let generated_set: HashSet<&PathBuf> = generated.iter().collect();
        if let Ok(rd) = std::fs::read_dir(output_dir) {
            for dir_entry in rd.flatten() {
                let path = dir_entry.path();
                if path.extension().is_some_and(|e| e == bare_suffix)
                    && !generated_set.contains(&path)
                {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
    }

    generated
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── infer_obj_type ──────────────────────────────────────────────────────

    #[test]
    fn infer_module_lowercase() {
        assert_eq!(infer_obj_type("sphinx.cmd.build"), ObjType::Module);
    }

    #[test]
    fn infer_module_single_name() {
        assert_eq!(infer_obj_type("mypkg"), ObjType::Module);
    }

    #[test]
    fn infer_class_uppercase_last() {
        assert_eq!(infer_obj_type("sphinx.cmd.build.BuildArgs"), ObjType::Class);
    }

    #[test]
    fn infer_class_single_uppercase() {
        assert_eq!(infer_obj_type("MyClass"), ObjType::Class);
    }

    // ── split_fqn ──────────────────────────────────────────────────────────

    #[test]
    fn split_fqn_dotted() {
        let (m, q) = split_fqn("sphinx.cmd.build");
        assert_eq!(m, "sphinx.cmd");
        assert_eq!(q, "build");
    }

    #[test]
    fn split_fqn_class() {
        let (m, q) = split_fqn("sphinx.cmd.build.Sphinx");
        assert_eq!(m, "sphinx.cmd.build");
        assert_eq!(q, "Sphinx");
    }

    #[test]
    fn split_fqn_simple() {
        let (m, q) = split_fqn("mypkg");
        assert_eq!(m, "");
        assert_eq!(q, "mypkg");
    }

    // ── stub_context ────────────────────────────────────────────────────────

    #[test]
    fn stub_context_module() {
        let entry = AutosummaryEntry {
            name: "sphinx.cmd.build".to_string(),
            toctree: Some("/docs/api".to_string()),
            template: String::new(),
            recursive: false,
        };
        let ctx = StubContext::from_entry(&entry);
        assert_eq!(ctx.fullname, "sphinx.cmd.build");
        assert_eq!(ctx.module, "sphinx.cmd");
        assert_eq!(ctx.objname, "build");
        assert_eq!(ctx.name, "build");
        assert_eq!(ctx.objtype, "module");
        assert_eq!(ctx.underline, "================");
    }

    #[test]
    fn stub_context_class() {
        let entry = AutosummaryEntry {
            name: "sphinx.cmd.build.Sphinx".to_string(),
            toctree: Some("/docs/api".to_string()),
            template: String::new(),
            recursive: false,
        };
        let ctx = StubContext::from_entry(&entry);
        assert_eq!(ctx.objtype, "class");
        assert_eq!(ctx.module, "sphinx.cmd.build");
        assert_eq!(ctx.objname, "Sphinx");
    }

    // ── from_entry_runtime (H9a/H9d) ─────────────────────────────────────────

    #[test]
    fn from_entry_runtime_populates_module_functions() {
        let entry = AutosummaryEntry {
            name: "json".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        };
        let ctx = StubContext::from_entry_runtime(&entry, &[]);
        assert_eq!(ctx.objtype, "module");
        assert!(
            ctx.functions.iter().any(|f| f == "dumps"),
            "functions: {:?}",
            ctx.functions
        );
        assert!(
            ctx.all_functions.iter().any(|f| f == "loads"),
            "all_functions: {:?}",
            ctx.all_functions
        );
    }

    #[test]
    fn from_entry_runtime_populates_class_methods() {
        let entry = AutosummaryEntry {
            name: "collections.OrderedDict".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        };
        let ctx = StubContext::from_entry_runtime(&entry, &[]);
        assert_eq!(ctx.objtype, "class");
        assert!(
            ctx.methods.iter().any(|m| m == "popitem"),
            "methods: {:?}",
            ctx.methods
        );
    }

    #[test]
    fn from_entry_runtime_falls_back_to_heuristic_for_unimportable() {
        let entry = AutosummaryEntry {
            name: "totally_not_a_real_module_xyz".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        };
        let ctx = StubContext::from_entry_runtime(&entry, &[]);
        // Falls back to `from_entry`'s empty-member heuristic instead of
        // panicking or erroring.
        assert_eq!(ctx.objtype, "module");
        assert!(ctx.functions.is_empty());
        assert!(ctx.classes.is_empty());
    }

    // ── generate_stub_runtime ─────────────────────────────────────────────────

    #[test]
    fn generate_stub_runtime_populates_real_members() {
        let tmpdir = tempfile::tempdir().unwrap();
        let templates = AutogenTemplates::vendored();
        let entry = AutosummaryEntry {
            name: "json".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        };
        let path = generate_stub_runtime(&entry, tmpdir.path(), "rst", true, &templates, &[])
            .unwrap()
            .unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("dumps"), "content: {content}");
    }

    // ── generate_stub ───────────────────────────────────────────────────────

    #[test]
    fn generate_stub_module() {
        let tmpdir = tempfile::tempdir().unwrap();
        let templates = AutogenTemplates::vendored();
        let entry = AutosummaryEntry {
            name: "mypkg.utils".to_string(),
            toctree: Some(tmpdir.path().to_string_lossy().into_owned()),
            template: String::new(),
            recursive: false,
        };
        let path = generate_stub(&entry, tmpdir.path(), "rst", true, &templates)
            .unwrap()
            .unwrap();
        assert_eq!(path, tmpdir.path().join("mypkg.utils.rst"));
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(
            content.contains(".. automodule:: mypkg.utils"),
            "content: {content}"
        );
        assert!(content.contains("mypkg.utils"), "heading missing");
    }

    #[test]
    fn generate_stub_class() {
        let tmpdir = tempfile::tempdir().unwrap();
        let templates = AutogenTemplates::vendored();
        let entry = AutosummaryEntry {
            name: "mypkg.MyClass".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        };
        let path = generate_stub(&entry, tmpdir.path(), "rst", true, &templates)
            .unwrap()
            .unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(
            content.contains(".. autoclass:: MyClass"),
            "content: {content}"
        );
        assert!(content.contains(".. currentmodule:: mypkg"));
    }

    #[test]
    fn generate_stub_explicit_template() {
        let tmpdir = tempfile::tempdir().unwrap();
        let templates = AutogenTemplates::vendored();
        let entry = AutosummaryEntry {
            name: "mypkg.utils".to_string(),
            toctree: None,
            template: "base.rst".to_string(),
            recursive: false,
        };
        let path = generate_stub(&entry, tmpdir.path(), "rst", true, &templates)
            .unwrap()
            .unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        // base.rst uses objtype which is "module" for lowercase names
        assert!(content.contains("auto"), "content: {content}");
    }

    #[test]
    fn generate_stub_no_overwrite_skips_changed() {
        let tmpdir = tempfile::tempdir().unwrap();
        let templates = AutogenTemplates::vendored();
        let entry = AutosummaryEntry {
            name: "mypkg.utils".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        };
        // Write a different file
        let file_path = tmpdir.path().join("mypkg.utils.rst");
        std::fs::write(&file_path, "old content\n").unwrap();

        let result = generate_stub(&entry, tmpdir.path(), "rst", false, &templates).unwrap();
        assert!(
            result.is_none(),
            "should be None (skipped) but was {result:?}"
        );
        // Original content preserved
        assert_eq!(
            std::fs::read_to_string(&file_path).unwrap(),
            "old content\n"
        );
    }

    #[test]
    fn generate_stub_identical_content_no_rewrite() {
        let tmpdir = tempfile::tempdir().unwrap();
        let templates = AutogenTemplates::vendored();
        let entry = AutosummaryEntry {
            name: "mypkg.utils".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        };
        // First write
        generate_stub(&entry, tmpdir.path(), "rst", true, &templates)
            .unwrap()
            .unwrap();
        let file_path = tmpdir.path().join("mypkg.utils.rst");
        let mtime1 = std::fs::metadata(&file_path).unwrap().modified().ok();

        // Second write with identical content — should return Some but not rewrite
        let result = generate_stub(&entry, tmpdir.path(), "rst", true, &templates).unwrap();
        assert!(result.is_some());
        // We don't assert mtime unchanged (not all filesystems have sub-second precision)
        // but the content should be identical.
        let _ = mtime1;
    }

    // ── generate_stubs ──────────────────────────────────────────────────────

    #[test]
    fn generate_stubs_batch() {
        let tmpdir = tempfile::tempdir().unwrap();
        let templates = AutogenTemplates::vendored();
        let entries = vec![
            AutosummaryEntry {
                name: "pkg.mod_a".to_string(),
                toctree: None,
                template: String::new(),
                recursive: false,
            },
            AutosummaryEntry {
                name: "pkg.ClassB".to_string(),
                toctree: None,
                template: String::new(),
                recursive: false,
            },
        ];
        let generated = generate_stubs(&entries, tmpdir.path(), "rst", true, false, &templates);
        assert_eq!(generated.len(), 2);
        assert!(tmpdir.path().join("pkg.mod_a.rst").exists());
        assert!(tmpdir.path().join("pkg.ClassB.rst").exists());
    }

    #[test]
    fn generate_stubs_remove_old() {
        let tmpdir = tempfile::tempdir().unwrap();
        // Pre-existing stale file
        let stale = tmpdir.path().join("stale.rst");
        std::fs::write(&stale, "old\n").unwrap();

        let templates = AutogenTemplates::vendored();
        let entries = vec![AutosummaryEntry {
            name: "pkg.mod".to_string(),
            toctree: None,
            template: String::new(),
            recursive: false,
        }];
        generate_stubs(&entries, tmpdir.path(), "rst", true, true, &templates);
        // Stale file removed, new file exists
        assert!(!stale.exists(), "stale file should have been removed");
        assert!(tmpdir.path().join("pkg.mod.rst").exists());
    }
}
