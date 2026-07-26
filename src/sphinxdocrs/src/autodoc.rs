//! `sphinxdocrs::autodoc` — Rust port of the extraction core of
//! `sphinx.ext.autodoc`, backed by the [`ruff_python_parser`] AST.
//!
//! Parses a Python source file and produces reStructuredText using the
//! `py` domain directives (`.. py:module::`, `.. py:function::`,
//! `.. py:class::`, `.. py:method::`) with the associated docstrings —
//! matching the output shape that `automodule` / `autoclass` / `autofunction`
//! expand to.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! |-----------------|-------------|-------|
//! | `ModuleDocumenter` | [`document_module`] | module docstring + members |
//! | `FunctionDocumenter` | [`render_function`] | signature + docstring |
//! | `ClassDocumenter` | [`render_class`] | bases + docstring + methods |
//! | `MethodDocumenter` | [`render_function`] (indented) | methods of a class |
//! | signature formatting | [`format_signature`] | args, defaults, `*args`, `**kwargs` |
//!
//! **Deferred**: runtime import (`autodoc` imports modules; this is static —
//! no code execution), `:members:` option filtering, inherited members,
//! type-hint resolution, overloads, decorators inspection, `__all__` ordering.
//!
//! ## Security
//!
//! Unlike Python's `autodoc`, this port **never imports or executes** the
//! target module — it only parses source text.  That eliminates the arbitrary
//! code-execution surface that `autodoc` has by design.

use std::path::Path;

use ruff_python_ast::{Expr, Parameters, Stmt, StmtClassDef, StmtFunctionDef};
use ruff_python_parser::parse_module;

/// Error type for autodoc extraction.
#[derive(Debug)]
pub enum AutodocError {
    /// The source file could not be read.
    Io(std::io::Error),
    /// The Python source failed to parse.
    Parse(String),
}

impl std::fmt::Display for AutodocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutodocError::Io(e) => write!(f, "I/O error: {e}"),
            AutodocError::Parse(s) => write!(f, "parse error: {s}"),
        }
    }
}

impl std::error::Error for AutodocError {}

impl From<std::io::Error> for AutodocError {
    fn from(e: std::io::Error) -> Self {
        AutodocError::Io(e)
    }
}

/// Extract the leading docstring from a statement body, if present.
///
/// Mirrors CPython's rule: a docstring is a bare string-literal expression
/// statement that is the *first* statement in a module/class/function body.
fn extract_docstring(body: &[Stmt]) -> Option<String> {
    let first = body.first()?;
    let Stmt::Expr(expr_stmt) = first else {
        return None;
    };
    let lit = expr_stmt.value.as_string_literal_expr()?;
    Some(dedent_docstring(lit.value.to_str()))
}

/// Normalise a docstring the way Sphinx does: strip a uniform leading indent
/// from every line after the first, and trim surrounding blank lines.
///
/// This is a faithful (if compact) port of `inspect.cleandoc` semantics.
fn dedent_docstring(raw: &str) -> String {
    let lines: Vec<&str> = raw.lines().collect();
    if lines.is_empty() {
        return String::new();
    }
    // Determine the minimum indentation of lines 2..N (ignoring blank lines).
    let mut min_indent: Option<usize> = None;
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        min_indent = Some(match min_indent {
            Some(m) => m.min(indent),
            None => indent,
        });
    }
    let indent = min_indent.unwrap_or(0);

    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    // First line: strip leading whitespace only.
    out.push(lines[0].trim_start().to_owned());
    for line in lines.iter().skip(1) {
        if line.len() >= indent {
            out.push(line[indent..].trim_end().to_owned());
        } else {
            out.push(line.trim_end().to_owned());
        }
    }
    // Trim leading/trailing blank lines.
    while out.first().is_some_and(|l| l.is_empty()) {
        out.remove(0);
    }
    while out.last().is_some_and(|l| l.is_empty()) {
        out.pop();
    }
    out.join("\n")
}

/// Render a single expression back to a compact Python-ish source snippet,
/// used for default values and base-class names.  Only the common cases are
/// handled; anything else falls back to `...`.
fn render_expr(expr: &Expr) -> String {
    match expr {
        Expr::Name(n) => n.id.to_string(),
        Expr::NumberLiteral(n) => format!("{:?}", n.value)
            .trim_start_matches("Int(")
            .to_string(),
        Expr::StringLiteral(s) => {
            // Render with single quotes to match common Python source style
            // (and Sphinx's signature formatting), escaping embedded quotes.
            let v = s.value.to_str();
            if v.contains('\'') && !v.contains('"') {
                format!("\"{v}\"")
            } else {
                format!("'{}'", v.replace('\'', "\\'"))
            }
        }
        Expr::BooleanLiteral(b) => if b.value { "True" } else { "False" }.to_string(),
        Expr::NoneLiteral(_) => "None".to_string(),
        Expr::Attribute(a) => format!("{}.{}", render_expr(&a.value), a.attr),
        _ => "...".to_string(),
    }
}

/// Format a function's parameter list into a Sphinx signature string,
/// e.g. `a, b=1, *args, key=None, **kwargs`.
pub fn format_signature(params: &Parameters) -> String {
    let mut parts: Vec<String> = Vec::new();

    // Positional-only parameters, followed by a `/` marker when present.
    for p in &params.posonlyargs {
        parts.push(render_param(&p.parameter.name, p.default.as_deref()));
    }
    if !params.posonlyargs.is_empty() {
        parts.push("/".to_string());
    }

    // Regular positional-or-keyword parameters.
    for p in &params.args {
        parts.push(render_param(&p.parameter.name, p.default.as_deref()));
    }

    // *args (or a bare `*` when there are kw-only args but no vararg).
    if let Some(vararg) = &params.vararg {
        parts.push(format!("*{}", vararg.name.as_str()));
    } else if !params.kwonlyargs.is_empty() {
        parts.push("*".to_string());
    }

    // Keyword-only parameters.
    for p in &params.kwonlyargs {
        parts.push(render_param(&p.parameter.name, p.default.as_deref()));
    }

    // **kwargs.
    if let Some(kwarg) = &params.kwarg {
        parts.push(format!("**{}", kwarg.name.as_str()));
    }

    parts.join(", ")
}

/// Render a single parameter with an optional default.
fn render_param(name: &ruff_python_ast::Identifier, default: Option<&Expr>) -> String {
    match default {
        Some(d) => format!("{}={}", name.as_str(), render_expr(d)),
        None => name.as_str().to_string(),
    }
}

/// Indent every non-empty line of `text` by `spaces` spaces.
fn indent(text: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    text.lines()
        .map(|l| {
            if l.is_empty() {
                String::new()
            } else {
                format!("{pad}{l}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Render a function/method as a `.. py:function::` (or `py:method::`) block.
///
/// `directive` is `"function"` for module-level functions and `"method"`
/// for class members; `base_indent` shifts the whole block (methods are
/// nested under their class).
pub fn render_function(func: &StmtFunctionDef, directive: &str, base_indent: usize) -> String {
    let sig = format_signature(&func.parameters);
    let mut out = String::new();
    out.push_str(&format!(
        ".. py:{directive}:: {}({})\n",
        func.name.as_str(),
        sig
    ));
    if let Some(doc) = extract_docstring(&func.body) {
        out.push('\n');
        // Docstring body is indented 3 spaces under the directive.
        out.push_str(&indent(&doc, 3));
        out.push('\n');
    }
    if base_indent > 0 {
        indent(&out, base_indent)
    } else {
        out
    }
}

/// Render a class as a `.. py:class::` block including its methods.
pub fn render_class(class: &StmtClassDef) -> String {
    let mut out = String::new();

    // Base classes, if any, rendered into the class signature.
    let bases = class
        .arguments
        .as_ref()
        .map(|args| {
            args.args
                .iter()
                .map(render_expr)
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();

    if bases.is_empty() {
        out.push_str(&format!(".. py:class:: {}\n", class.name.as_str()));
    } else {
        out.push_str(&format!(
            ".. py:class:: {}({})\n",
            class.name.as_str(),
            bases
        ));
    }

    if let Some(doc) = extract_docstring(&class.body) {
        out.push('\n');
        out.push_str(&indent(&doc, 3));
        out.push('\n');
    }

    // Public methods (skip dunder/private, matching autodoc defaults).
    for stmt in &class.body {
        if let Stmt::FunctionDef(method) = stmt {
            let name = method.name.as_str();
            if name.starts_with('_') && (name != "__init__") {
                continue;
            }
            out.push('\n');
            // Methods are nested 3 spaces under the class directive.
            out.push_str(&render_function(method, "method", 3));
        }
    }

    out
}

/// Parse `source` (whole module) and render RST documentation for it.
///
/// `module_name` becomes the `.. py:module::` target.  Only public
/// top-level functions and classes are documented (names not starting
/// with `_`), matching `automodule` defaults without `:private-members:`.
pub fn document_module_source(module_name: &str, source: &str) -> Result<String, AutodocError> {
    let parsed = parse_module(source).map_err(|e| AutodocError::Parse(e.to_string()))?;
    let suite = parsed.suite();

    let mut out = String::new();
    out.push_str(&format!(".. py:module:: {module_name}\n"));

    // Module docstring.
    if let Some(doc) = extract_docstring(suite) {
        out.push('\n');
        out.push_str(&doc);
        out.push('\n');
    }

    for stmt in suite {
        match stmt {
            Stmt::FunctionDef(func) => {
                let name = func.name.as_str();
                if name.starts_with('_') {
                    continue;
                }
                out.push('\n');
                out.push_str(&render_function(func, "function", 0));
            }
            Stmt::ClassDef(class) => {
                if class.name.as_str().starts_with('_') {
                    continue;
                }
                out.push('\n');
                out.push_str(&render_class(class));
            }
            _ => {}
        }
    }

    Ok(out)
}

/// Read a Python file from disk and render its RST documentation.
///
/// The module name is derived from the file stem.
pub fn document_module(path: &Path) -> Result<String, AutodocError> {
    let source = std::fs::read_to_string(path)?;
    let module_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module")
        .to_owned();
    document_module_source(&module_name, &source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_docstring_is_rendered() {
        let src = "\"\"\"Top-level module docs.\"\"\"\n\ndef noop():\n    pass\n";
        let rst = document_module_source("mymod", src).unwrap();
        assert!(rst.contains(".. py:module:: mymod"));
        assert!(rst.contains("Top-level module docs."));
    }

    #[test]
    fn function_signature_and_docstring() {
        let src = "\
def greet(name, greeting='hello', *args, loud=False, **kwargs):
    \"\"\"Greet someone.\"\"\"
    return None
";
        let rst = document_module_source("m", src).unwrap();
        assert!(
            rst.contains(
                ".. py:function:: greet(name, greeting='hello', *args, loud=False, **kwargs)"
            ),
            "got:\n{rst}"
        );
        assert!(rst.contains("Greet someone."));
    }

    #[test]
    fn class_with_methods_and_bases() {
        let src = "\
class Widget(Base):
    \"\"\"A widget.\"\"\"

    def __init__(self, size):
        \"\"\"Make a widget.\"\"\"
        self.size = size

    def area(self):
        \"\"\"Return area.\"\"\"
        return self.size ** 2

    def _private(self):
        pass
";
        let rst = document_module_source("m", src).unwrap();
        assert!(rst.contains(".. py:class:: Widget(Base)"), "got:\n{rst}");
        assert!(rst.contains("A widget."));
        assert!(rst.contains(".. py:method:: __init__(self, size)"));
        assert!(rst.contains(".. py:method:: area(self)"));
        // Private method must be skipped.
        assert!(!rst.contains("_private"), "private method leaked:\n{rst}");
    }

    #[test]
    fn private_toplevel_names_skipped() {
        let src = "\
def _hidden():
    pass

def visible():
    pass

class _Secret:
    pass

class Public:
    pass
";
        let rst = document_module_source("m", src).unwrap();
        assert!(rst.contains(".. py:function:: visible()"));
        assert!(rst.contains(".. py:class:: Public"));
        assert!(!rst.contains("_hidden"));
        assert!(!rst.contains("_Secret"));
    }

    #[test]
    fn dedent_docstring_multiline() {
        let raw = "First line.\n\n    Indented body.\n    More body.\n    ";
        let out = dedent_docstring(raw);
        assert!(out.starts_with("First line."));
        assert!(out.contains("Indented body."));
        // The common indent must be stripped.
        assert!(!out.contains("    Indented"));
    }

    #[test]
    fn positional_only_and_keyword_only() {
        let src = "\
def f(a, b, /, c, *, d, e=5):
    pass
";
        let rst = document_module_source("m", src).unwrap();
        assert!(
            rst.contains(".. py:function:: f(a, b, /, c, *, d, e=5)"),
            "got:\n{rst}"
        );
    }

    #[test]
    fn parse_error_is_reported() {
        let src = "def broken(:\n";
        let err = document_module_source("m", src).unwrap_err();
        assert!(matches!(err, AutodocError::Parse(_)));
    }

    #[test]
    fn no_module_docstring_still_lists_members() {
        let src = "def a():\n    pass\n";
        let rst = document_module_source("m", src).unwrap();
        assert!(rst.contains(".. py:module:: m"));
        assert!(rst.contains(".. py:function:: a()"));
    }
}
