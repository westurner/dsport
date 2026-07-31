//! `sphinxdocrs::autodoc` — Rust port of the extraction core of
//! `sphinx.ext.autodoc`, backed by the [`ruff_python_parser`] AST, plus
//! (**H9a**) an optional runtime-import bridge through
//! [`crate::autodoc_runtime`] for when static parsing structurally can't see
//! a member (decorated/dynamically-created members, C-extension modules,
//! an `__all__` computed at import time).
//!
//! Parses a Python source file and produces reStructuredText using the
//! `py` domain directives (`.. py:module::`, `.. py:function::`,
//! `.. py:class::`, `.. py:method::`, `.. py:property::`,
//! `.. py:staticmethod::`, `.. py:classmethod::`, `.. py:exception::`) with
//! the associated docstrings — matching the output shape that
//! `automodule` / `autoclass` / `autofunction` expand to.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! |-----------------|-------------|-------|
//! | `ModuleDocumenter` | [`document_module`] | module docstring + members |
//! | `FunctionDocumenter` | [`render_function`] | signature + docstring |
//! | `ClassDocumenter` | [`render_class`] | bases + docstring + methods |
//! | `MethodDocumenter` | [`render_function`] (indented) | methods of a class |
//! | signature formatting | [`format_signature`]/[`format_signature_with_hints`] | args, defaults, `*args`, `**kwargs`, type hints (**H9c**) |
//! | `Documenter.filter_members` | [`select_members`] | `:members:`/`:undoc-members:`/`:private-members:`/`:special-members:`/`:exclude-members:`/`:member-order:` (**H9b**) |
//! | runtime import | [`crate::autodoc_runtime`] | PyO3 + `inspect`, with `ruff_python_ast` fallback (**H9a**) |
//!
//! **Accepted deviations**: `:inherited-members:` is parsed into
//! [`AutodocOptions`] but not yet expanded into a base class's own members
//! (base-class member lookup needs either a whole-project symbol table for
//! the static path, or `inspect.getmro` for the runtime path — both larger
//! than this pass); overload sets (`@typing.overload`) render only the last
//! definition, matching plain Python's own runtime shadowing rather than
//! upstream's specific multi-signature rendering; `autodoc_typehints =
//! "description"` is treated the same as `"signature"` (inline), not moved
//! into the body.
//!
//! ## Security
//!
//! The static (`ruff_python_ast`-only) path **never imports or executes**
//! the target module — it only parses source text. [`crate::autodoc_runtime`]
//! is the opt-in exception: it imports the target module for real (as
//! upstream `autodoc` always does), so it carries the same code-execution
//! surface as upstream. Callers that want the static-only guarantee should
//! use [`document_module`]/[`document_module_source`]/
//! [`document_module_source_with_options`] directly and never call into
//! [`document_module_auto`]/[`crate::autodoc_runtime`].

use std::collections::{HashMap, HashSet};
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

// ── H9b: member selection options ───────────────────────────────────────────

/// `autodoc_typehints` handling for a rendered signature (**H9c**).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeHints {
    /// Render parameter/return annotations inline in the signature.
    /// Matches upstream's default (`autodoc_typehints = "signature"`) — and,
    /// per the module doc, also stands in for `"description"` (an accepted
    /// deviation: not moved into the body).
    Signature,
    /// Strip all annotations (`autodoc_typehints = "none"`).
    None,
}

/// One `:members:`/`:private-members:`/`:special-members:`-style option
/// value: not given at all, given bare (select every matching candidate),
/// or given with an explicit comma-separated name list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberSelector {
    Unset,
    All,
    Named(Vec<String>),
}

/// `:member-order:` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberOrder {
    Alphabetical,
    Bysource,
    Groupwise,
}

/// Parsed `.. automodule::`/`.. autoclass::`/`.. autofunction::` directive
/// options (**H9b**), mirroring `sphinx.ext.autodoc.Documenter.filter_members`'s
/// inputs.
#[derive(Debug, Clone)]
pub struct AutodocOptions {
    pub members: MemberSelector,
    pub undoc_members: bool,
    pub private_members: MemberSelector,
    pub special_members: MemberSelector,
    /// Parsed but not yet expanded — see the module-level accepted
    /// deviation note.
    pub inherited_members: bool,
    pub exclude_members: HashSet<String>,
    pub member_order: MemberOrder,
    pub typehints: TypeHints,
}

impl Default for AutodocOptions {
    fn default() -> Self {
        AutodocOptions {
            members: MemberSelector::Unset,
            undoc_members: false,
            private_members: MemberSelector::Unset,
            special_members: MemberSelector::Unset,
            inherited_members: false,
            exclude_members: HashSet::new(),
            member_order: MemberOrder::Alphabetical,
            typehints: TypeHints::Signature,
        }
    }
}

impl AutodocOptions {
    /// Options matching this crate's pre-H9b hardcoded behavior: every
    /// public top-level function/class and public method, always including
    /// `__init__`, in source declaration order, regardless of whether a
    /// docstring is present. [`document_module_source`]/[`document_module`]
    /// use this so their output doesn't change for existing callers.
    pub fn legacy_default() -> Self {
        AutodocOptions {
            members: MemberSelector::All,
            undoc_members: true,
            private_members: MemberSelector::Unset,
            special_members: MemberSelector::Named(vec!["__init__".to_string()]),
            inherited_members: false,
            exclude_members: HashSet::new(),
            member_order: MemberOrder::Bysource,
            typehints: TypeHints::Signature,
        }
    }

    /// Parse options from `(name, raw_value)` pairs collected from an
    /// `.. automodule::`/`.. autoclass::`/`.. autofunction::` option block.
    /// A flag-only option (`:members:` with no argument) has an empty
    /// `raw_value`; `:members: a, b` has a comma-separated `raw_value`.
    pub fn from_option_pairs(pairs: &[(String, String)]) -> Self {
        let mut opts = AutodocOptions::default();
        let parse_selector = |value: &str| -> MemberSelector {
            if value.trim().is_empty() {
                MemberSelector::All
            } else {
                MemberSelector::Named(
                    value
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                )
            }
        };
        for (name, value) in pairs {
            match name.as_str() {
                "members" => opts.members = parse_selector(value),
                "undoc-members" => opts.undoc_members = true,
                "private-members" => opts.private_members = parse_selector(value),
                "special-members" => opts.special_members = parse_selector(value),
                "inherited-members" => opts.inherited_members = true,
                "exclude-members" => {
                    opts.exclude_members = value
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                "member-order" => {
                    opts.member_order = match value.trim() {
                        "bysource" => MemberOrder::Bysource,
                        "groupwise" => MemberOrder::Groupwise,
                        _ => MemberOrder::Alphabetical,
                    };
                }
                _ => {}
            }
        }
        opts
    }
}

/// Coarse kind tag used only for `:member-order: groupwise` ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemberKindTag {
    Attribute,
    Property,
    Method,
    Function,
    Class,
    Exception,
    Module,
}

/// One candidate member awaiting `filter_members`-style selection.
#[derive(Debug, Clone)]
pub struct MemberCandidate {
    pub name: String,
    pub has_doc: bool,
    /// Declaration order (or, for the runtime bridge, `inspect.getmembers`
    /// order — already alphabetical there, an accepted **bysource**
    /// deviation for runtime-introspected members, see `crate::autodoc_runtime`).
    pub source_order: usize,
    pub kind: MemberKindTag,
}

fn is_dunder(name: &str) -> bool {
    name.starts_with("__") && name.ends_with("__") && name.len() > 4
}

fn is_private_not_dunder(name: &str) -> bool {
    name.starts_with('_') && !is_dunder(name)
}

/// Apply `:members:`/`:undoc-members:`/`:private-members:`/
/// `:special-members:`/`:exclude-members:`/`:member-order:` filtering,
/// mirroring `sphinx.ext.autodoc.Documenter.filter_members` (**H9b**).
///
/// `module_all`, when `Some`, restricts *and orders* the implicit
/// (bare `:members:`) selection to names present in it — mirrors
/// `automodule`'s `__all__` honoring. Pass `None` for a class's members
/// (`__all__` is module-scoped only).
pub fn select_members(
    candidates: &[MemberCandidate],
    options: &AutodocOptions,
    module_all: Option<&[String]>,
) -> Vec<String> {
    let mut included: Vec<MemberCandidate> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut protected_from_undoc: HashSet<String> = HashSet::new();

    match &options.members {
        MemberSelector::Unset => {}
        MemberSelector::All => {
            if let Some(all_names) = module_all {
                for n in all_names {
                    if let Some(c) = candidates.iter().find(|c| &c.name == n) {
                        if seen.insert(c.name.clone()) {
                            included.push(c.clone());
                        }
                    }
                }
            } else {
                for c in candidates
                    .iter()
                    .filter(|c| !is_dunder(&c.name) && !is_private_not_dunder(&c.name))
                {
                    if seen.insert(c.name.clone()) {
                        included.push(c.clone());
                    }
                }
            }
        }
        MemberSelector::Named(names) => {
            for n in names {
                if let Some(c) = candidates.iter().find(|c| &c.name == n) {
                    if seen.insert(c.name.clone()) {
                        included.push(c.clone());
                    }
                    protected_from_undoc.insert(c.name.clone());
                }
            }
        }
    }

    // `:private-members:` — adds names matching `_foo` (not `__foo__`).
    match &options.private_members {
        MemberSelector::Unset => {}
        MemberSelector::All => {
            for c in candidates.iter().filter(|c| is_private_not_dunder(&c.name)) {
                if seen.insert(c.name.clone()) {
                    included.push(c.clone());
                }
            }
        }
        MemberSelector::Named(names) => {
            for n in names {
                if let Some(c) = candidates.iter().find(|c| &c.name == n) {
                    if seen.insert(c.name.clone()) {
                        included.push(c.clone());
                    }
                    protected_from_undoc.insert(c.name.clone());
                }
            }
        }
    }

    // `:special-members:` — adds dunder names (`__init__`, `__str__`, ...).
    match &options.special_members {
        MemberSelector::Unset => {}
        MemberSelector::All => {
            for c in candidates.iter().filter(|c| is_dunder(&c.name)) {
                if seen.insert(c.name.clone()) {
                    included.push(c.clone());
                }
            }
        }
        MemberSelector::Named(names) => {
            for n in names {
                if let Some(c) = candidates.iter().find(|c| &c.name == n) {
                    if seen.insert(c.name.clone()) {
                        included.push(c.clone());
                    }
                    protected_from_undoc.insert(c.name.clone());
                }
            }
        }
    }

    if !options.undoc_members {
        included.retain(|c| c.has_doc || protected_from_undoc.contains(&c.name));
    }

    included.retain(|c| !options.exclude_members.contains(&c.name));

    match options.member_order {
        MemberOrder::Alphabetical => included.sort_by(|a, b| a.name.cmp(&b.name)),
        MemberOrder::Bysource => included.sort_by_key(|c| c.source_order),
        MemberOrder::Groupwise => {
            included.sort_by(|a, b| (a.kind, &a.name).cmp(&(b.kind, &b.name)))
        }
    }

    included.into_iter().map(|c| c.name).collect()
}

/// Extract a module's `__all__` list, when it's a plain assignment to a
/// list/tuple of string literals (the overwhelming common case). Anything
/// more dynamic (computed at import time) is out of reach for the static
/// path — see [`crate::autodoc_runtime`] for a path that can see it.
fn extract_module_all(suite: &[Stmt]) -> Option<Vec<String>> {
    for stmt in suite {
        if let Stmt::Assign(assign) = stmt {
            if assign.targets.len() == 1 {
                if let Expr::Name(n) = &assign.targets[0] {
                    if n.id.as_str() == "__all__" {
                        return extract_string_list(&assign.value);
                    }
                }
            }
        }
    }
    None
}

fn extract_string_list(expr: &Expr) -> Option<Vec<String>> {
    let elts: &[Expr] = match expr {
        Expr::List(l) => &l.elts,
        Expr::Tuple(t) => &t.elts,
        _ => return None,
    };
    let mut out = Vec::with_capacity(elts.len());
    for e in elts {
        if let Expr::StringLiteral(s) = e {
            out.push(s.value.to_str().to_string());
        } else {
            return None;
        }
    }
    Some(out)
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
///
/// `pub(crate)` so `domains::py_sig` can reuse it for base-class rendering
/// when scanning `.. py:class::`/`.. py:exception::` signatures (**H3b**).
pub(crate) fn render_expr(expr: &Expr) -> String {
    match expr {
        Expr::Name(n) => n.id.to_string(),
        Expr::NumberLiteral(n) => match &n.value {
            ruff_python_ast::Number::Int(i) => i.to_string(),
            ruff_python_ast::Number::Float(f) => f.to_string(),
            ruff_python_ast::Number::Complex { real, imag } => format!("({real}+{imag}j)"),
        },
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
        // Type-hint shapes (**H9c**): `Optional[int]`, `List[str]`, `int | None`.
        Expr::Subscript(s) => format!("{}[{}]", render_expr(&s.value), render_expr(&s.slice)),
        Expr::BinOp(b) => format!(
            "{} {} {}",
            render_expr(&b.left),
            b.op.as_str(),
            render_expr(&b.right)
        ),
        Expr::Tuple(t) => format!(
            "({})",
            t.elts
                .iter()
                .map(render_expr)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expr::List(l) => format!(
            "[{}]",
            l.elts
                .iter()
                .map(render_expr)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expr::UnaryOp(u) => {
            if matches!(u.op, ruff_python_ast::UnaryOp::Not) {
                format!("not {}", render_expr(&u.operand))
            } else {
                format!("{}{}", u.op.as_str(), render_expr(&u.operand))
            }
        }
        _ => "...".to_string(),
    }
}

/// Format a function's parameter list into a Sphinx signature string,
/// e.g. `a, b=1, *args, key=None, **kwargs`. Never renders type annotations
/// (equivalent to [`format_signature_with_hints`] with [`TypeHints::None`]).
pub fn format_signature(params: &Parameters) -> String {
    format_signature_with_hints(params, TypeHints::None)
}

/// Format a function's parameter list, optionally including type-hint
/// annotations (**H9c**, `autodoc_typehints`), e.g.
/// `a: int, b: str = 'x', *args: int, **kwargs: Any`.
pub fn format_signature_with_hints(params: &Parameters, typehints: TypeHints) -> String {
    let mut parts: Vec<String> = Vec::new();

    // Positional-only parameters, followed by a `/` marker when present.
    for p in &params.posonlyargs {
        parts.push(render_param(
            &p.parameter.name,
            p.parameter.annotation.as_deref(),
            p.default.as_deref(),
            typehints,
        ));
    }
    if !params.posonlyargs.is_empty() {
        parts.push("/".to_string());
    }

    // Regular positional-or-keyword parameters.
    for p in &params.args {
        parts.push(render_param(
            &p.parameter.name,
            p.parameter.annotation.as_deref(),
            p.default.as_deref(),
            typehints,
        ));
    }

    // *args (or a bare `*` when there are kw-only args but no vararg).
    if let Some(vararg) = &params.vararg {
        let ann = match typehints {
            TypeHints::Signature => vararg.annotation.as_deref(),
            TypeHints::None => Option::None,
        };
        match ann {
            Some(a) => parts.push(format!("*{}: {}", vararg.name.as_str(), render_expr(a))),
            Option::None => parts.push(format!("*{}", vararg.name.as_str())),
        }
    } else if !params.kwonlyargs.is_empty() {
        parts.push("*".to_string());
    }

    // Keyword-only parameters.
    for p in &params.kwonlyargs {
        parts.push(render_param(
            &p.parameter.name,
            p.parameter.annotation.as_deref(),
            p.default.as_deref(),
            typehints,
        ));
    }

    // **kwargs.
    if let Some(kwarg) = &params.kwarg {
        let ann = match typehints {
            TypeHints::Signature => kwarg.annotation.as_deref(),
            TypeHints::None => Option::None,
        };
        match ann {
            Some(a) => parts.push(format!("**{}: {}", kwarg.name.as_str(), render_expr(a))),
            Option::None => parts.push(format!("**{}", kwarg.name.as_str())),
        }
    }

    parts.join(", ")
}

/// Render `-> ReturnType` for a rendered signature, or `""` when there is no
/// return annotation or `typehints` strips them.
fn render_return_suffix(returns: Option<&Expr>, typehints: TypeHints) -> String {
    match (typehints, returns) {
        (TypeHints::Signature, Some(ret)) => format!(" -> {}", render_expr(ret)),
        _ => String::new(),
    }
}

/// Render a single parameter with an optional annotation and default.
fn render_param(
    name: &ruff_python_ast::Identifier,
    annotation: Option<&Expr>,
    default: Option<&Expr>,
    typehints: TypeHints,
) -> String {
    let annotation = match typehints {
        TypeHints::Signature => annotation,
        TypeHints::None => Option::None,
    };
    let mut s = name.as_str().to_string();
    if let Some(a) = annotation {
        s.push_str(": ");
        s.push_str(&render_expr(a));
    }
    if let Some(d) = default {
        if annotation.is_some() {
            s.push_str(" = ");
        } else {
            s.push('=');
        }
        s.push_str(&render_expr(d));
    }
    s
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

/// Coarse callable kind (**H9c** decorator awareness), determining which
/// `py` domain directive a class member renders as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionKind {
    /// Module-level function → `.. py:function::`.
    Function,
    /// Plain instance method → `.. py:method::`.
    Method,
    /// `@staticmethod` → `.. py:staticmethod::`.
    StaticMethod,
    /// `@classmethod` → `.. py:classmethod::`.
    ClassMethod,
    /// `@property` → `.. py:property::` (no signature/parens rendered).
    Property,
}

impl FunctionKind {
    fn directive(self) -> &'static str {
        match self {
            FunctionKind::Function => "function",
            FunctionKind::Method => "method",
            FunctionKind::StaticMethod => "staticmethod",
            FunctionKind::ClassMethod => "classmethod",
            FunctionKind::Property => "property",
        }
    }
}

/// Render a single decorator expression back to its (dotted) name, e.g.
/// `@property` → `"property"`, `@app.route(...)` → `"route"`.
fn decorator_name(expr: &Expr) -> String {
    match expr {
        Expr::Name(n) => n.id.to_string(),
        Expr::Attribute(a) => a.attr.to_string(),
        Expr::Call(c) => decorator_name(&c.func),
        _ => String::new(),
    }
}

/// Classify a class method by its decorators (**H9c**). Defaults to
/// [`FunctionKind::Method`] when no recognized decorator is present.
fn function_kind(func: &StmtFunctionDef) -> FunctionKind {
    for dec in &func.decorator_list {
        match decorator_name(&dec.expression).as_str() {
            "staticmethod" => return FunctionKind::StaticMethod,
            "classmethod" => return FunctionKind::ClassMethod,
            "property" => return FunctionKind::Property,
            _ => {}
        }
    }
    FunctionKind::Method
}

/// Whether a class's bases look like an exception type, by name (`Error`/
/// `Exception` suffix, or exactly `BaseException`/`Exception`) — a static
/// heuristic; the runtime bridge (`crate::autodoc_runtime`) instead checks
/// `issubclass(cls, BaseException)` directly.
fn is_exception_class(class: &StmtClassDef) -> bool {
    class.arguments.as_ref().is_some_and(|args| {
        args.args.iter().any(|base| {
            let name = render_expr(base);
            let last = name.rsplit('.').next().unwrap_or(&name);
            last == "Exception" || last == "BaseException" || last.ends_with("Error")
        })
    })
}

/// Render a function/method as a `.. py:function::`/`.. py:method::`/
/// `.. py:staticmethod::`/`.. py:classmethod::`/`.. py:property::` block.
///
/// `base_indent` shifts the whole block (methods are nested under their
/// class). [`FunctionKind::Property`] omits the signature entirely, matching
/// `.. py:property:: name` (no parens).
pub fn render_function(
    func: &StmtFunctionDef,
    kind: FunctionKind,
    base_indent: usize,
    typehints: TypeHints,
) -> String {
    let directive = kind.directive();
    let mut out = String::new();
    if kind == FunctionKind::Property {
        out.push_str(&format!(".. py:{directive}:: {}\n", func.name.as_str()));
    } else {
        let sig = format_signature_with_hints(&func.parameters, typehints);
        let ret = render_return_suffix(func.returns.as_deref(), typehints);
        out.push_str(&format!(
            ".. py:{directive}:: {}({sig}){ret}\n",
            func.name.as_str()
        ));
    }
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

/// Render a class as a `.. py:class::`/`.. py:exception::` block including
/// its (filtered — **H9b**) methods.
pub fn render_class(class: &StmtClassDef, options: &AutodocOptions) -> String {
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

    let directive = if is_exception_class(class) {
        "exception"
    } else {
        "class"
    };

    if bases.is_empty() {
        out.push_str(&format!(".. py:{directive}:: {}\n", class.name.as_str()));
    } else {
        out.push_str(&format!(
            ".. py:{directive}:: {}({bases})\n",
            class.name.as_str(),
        ));
    }

    if let Some(doc) = extract_docstring(&class.body) {
        out.push('\n');
        out.push_str(&indent(&doc, 3));
        out.push('\n');
    }

    let mut candidates: Vec<MemberCandidate> = Vec::new();
    let mut by_name: HashMap<&str, &StmtFunctionDef> = HashMap::new();
    for (idx, stmt) in class.body.iter().enumerate() {
        if let Stmt::FunctionDef(method) = stmt {
            let kind = match function_kind(method) {
                FunctionKind::Property => MemberKindTag::Property,
                _ => MemberKindTag::Method,
            };
            candidates.push(MemberCandidate {
                name: method.name.to_string(),
                has_doc: extract_docstring(&method.body).is_some(),
                source_order: idx,
                kind,
            });
            by_name.insert(method.name.as_str(), method);
        }
    }

    for name in select_members(&candidates, options, None) {
        if let Some(method) = by_name.get(name.as_str()) {
            out.push('\n');
            // Methods are nested 3 spaces under the class directive.
            out.push_str(&render_function(
                method,
                function_kind(method),
                3,
                options.typehints,
            ));
        }
    }

    out
}

/// Parse `source` (whole module) and render RST documentation for it, using
/// [`AutodocOptions::legacy_default`] — i.e. this crate's original,
/// pre-H9b hardcoded selection (public top-level functions/classes,
/// public methods, always including `__init__`).
///
/// `module_name` becomes the `.. py:module::` target.
pub fn document_module_source(module_name: &str, source: &str) -> Result<String, AutodocError> {
    document_module_source_with_options(module_name, source, &AutodocOptions::legacy_default())
}

/// Parse `source` (whole module) and render RST documentation for it,
/// applying [`AutodocOptions`] filtering/ordering (**H9b**) and, when the
/// module defines `__all__`, restricting/ordering the implicit member set
/// to it (**H9c**).
pub fn document_module_source_with_options(
    module_name: &str,
    source: &str,
    options: &AutodocOptions,
) -> Result<String, AutodocError> {
    let parsed = parse_module(source).map_err(|e| AutodocError::Parse(e.to_string()))?;
    let suite = parsed.suite();
    let module_all = extract_module_all(suite);

    let mut candidates: Vec<MemberCandidate> = Vec::new();
    enum TopLevel<'a> {
        Function(&'a StmtFunctionDef),
        Class(&'a StmtClassDef),
    }
    let mut by_name: HashMap<&str, TopLevel<'_>> = HashMap::new();
    for (idx, stmt) in suite.iter().enumerate() {
        match stmt {
            Stmt::FunctionDef(func) => {
                candidates.push(MemberCandidate {
                    name: func.name.to_string(),
                    has_doc: extract_docstring(&func.body).is_some(),
                    source_order: idx,
                    kind: MemberKindTag::Function,
                });
                by_name.insert(func.name.as_str(), TopLevel::Function(func));
            }
            Stmt::ClassDef(class) => {
                candidates.push(MemberCandidate {
                    name: class.name.to_string(),
                    has_doc: extract_docstring(&class.body).is_some(),
                    source_order: idx,
                    kind: if is_exception_class(class) {
                        MemberKindTag::Exception
                    } else {
                        MemberKindTag::Class
                    },
                });
                by_name.insert(class.name.as_str(), TopLevel::Class(class));
            }
            _ => {}
        }
    }

    let mut out = String::new();
    out.push_str(&format!(".. py:module:: {module_name}\n"));

    // Module docstring.
    if let Some(doc) = extract_docstring(suite) {
        out.push('\n');
        out.push_str(&doc);
        out.push('\n');
    }

    let selected = select_members(&candidates, options, module_all.as_deref());
    for name in &selected {
        match by_name.get(name.as_str()) {
            Some(TopLevel::Function(func)) => {
                out.push('\n');
                out.push_str(&render_function(
                    func,
                    FunctionKind::Function,
                    0,
                    options.typehints,
                ));
            }
            Some(TopLevel::Class(class)) => {
                out.push('\n');
                out.push_str(&render_class(class, options));
            }
            None => {}
        }
    }

    Ok(out)
}

/// Read a Python file from disk and render its RST documentation using the
/// static path only. The module name is derived from the file stem.
pub fn document_module(path: &Path) -> Result<String, AutodocError> {
    let source = std::fs::read_to_string(path)?;
    let module_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module")
        .to_owned();
    document_module_source(&module_name, &source)
}

/// Read a Python file from disk and render its RST documentation with
/// explicit options, using the static path only.
pub fn document_module_with_options(
    path: &Path,
    options: &AutodocOptions,
) -> Result<String, AutodocError> {
    let source = std::fs::read_to_string(path)?;
    let module_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module")
        .to_owned();
    document_module_source_with_options(&module_name, &source, options)
}

/// **H9a/H9d**: try the runtime-import bridge first (real member lists,
/// runtime-accurate signatures/`__all__`), falling back to the static
/// `ruff_python_ast` path (`document_module_with_options`) when the module
/// can't be imported even with `mock_imports` (`autodoc_mock_imports`)
/// applied — e.g. a genuinely missing dependency, or a C-extension quirk
/// PyO3 itself can't bridge.
///
/// `path` is only used by the static fallback (to locate/parse the source);
/// the runtime path resolves `module_name` purely through Python's own
/// import machinery.
pub fn document_module_auto(
    path: &Path,
    module_name: &str,
    options: &AutodocOptions,
    mock_imports: &[String],
) -> Result<String, AutodocError> {
    match crate::autodoc_runtime::introspect_module(module_name, mock_imports) {
        Ok(modinfo) => Ok(render_module_from_runtime(&modinfo, options, mock_imports)),
        Err(_) => document_module_with_options(path, options),
    }
}

fn member_kind_tag(kind: crate::autodoc_runtime::MemberKind) -> MemberKindTag {
    use crate::autodoc_runtime::MemberKind as K;
    match kind {
        K::Module => MemberKindTag::Module,
        K::Class => MemberKindTag::Class,
        K::Exception => MemberKindTag::Exception,
        K::Function => MemberKindTag::Function,
        K::Method => MemberKindTag::Method,
        K::Property => MemberKindTag::Property,
        K::Attribute => MemberKindTag::Attribute,
    }
}

/// Render RST from a runtime [`crate::autodoc_runtime::ModuleIntrospection`],
/// applying the same [`AutodocOptions`] filtering as the static path.
/// `str(inspect.signature(obj))` already includes the surrounding parens
/// and any `-> ReturnType` suffix, so it's embedded directly after the name.
fn render_module_from_runtime(
    modinfo: &crate::autodoc_runtime::ModuleIntrospection,
    options: &AutodocOptions,
    mock_imports: &[String],
) -> String {
    use crate::autodoc_runtime::MemberKind;

    let mut out = String::new();
    out.push_str(&format!(".. py:module:: {}\n", modinfo.name));
    if let Some(doc) = &modinfo.docstring {
        out.push('\n');
        out.push_str(doc);
        out.push('\n');
    }

    let candidates: Vec<MemberCandidate> = modinfo
        .members
        .iter()
        .enumerate()
        .map(|(idx, m)| MemberCandidate {
            name: m.name.clone(),
            has_doc: m.docstring.is_some(),
            source_order: idx,
            kind: member_kind_tag(m.kind),
        })
        .collect();
    let selected = select_members(&candidates, options, modinfo.all.as_deref());

    for name in &selected {
        let Some(member) = modinfo.members.iter().find(|m| &m.name == name) else {
            continue;
        };
        match member.kind {
            MemberKind::Function => {
                out.push('\n');
                out.push_str(&format!(
                    ".. py:function:: {}{}\n",
                    member.name,
                    member.signature.as_deref().unwrap_or("()")
                ));
                if let Some(doc) = &member.docstring {
                    out.push('\n');
                    out.push_str(&indent(doc, 3));
                    out.push('\n');
                }
            }
            MemberKind::Class | MemberKind::Exception => {
                let directive = if member.kind == MemberKind::Exception {
                    "exception"
                } else {
                    "class"
                };
                out.push('\n');
                out.push_str(&format!(".. py:{directive}:: {}\n", member.name));
                if let Some(doc) = &member.docstring {
                    out.push('\n');
                    out.push_str(&indent(doc, 3));
                    out.push('\n');
                }
                if let Ok(classinfo) = crate::autodoc_runtime::introspect_class(
                    &modinfo.name,
                    &member.name,
                    mock_imports,
                ) {
                    let method_candidates: Vec<MemberCandidate> = classinfo
                        .members
                        .iter()
                        .enumerate()
                        .map(|(idx, m)| MemberCandidate {
                            name: m.name.clone(),
                            has_doc: m.docstring.is_some(),
                            source_order: idx,
                            kind: member_kind_tag(m.kind),
                        })
                        .collect();
                    for method_name in select_members(&method_candidates, options, None) {
                        let Some(m) = classinfo.members.iter().find(|x| x.name == method_name)
                        else {
                            continue;
                        };
                        let directive = match m.kind {
                            crate::autodoc_runtime::MemberKind::Property => "property",
                            _ => "method",
                        };
                        out.push('\n');
                        out.push_str(&indent(
                            &format!(
                                ".. py:{directive}:: {}{}\n",
                                m.name,
                                m.signature.as_deref().unwrap_or("")
                            ),
                            3,
                        ));
                        if let Some(doc) = &m.docstring {
                            out.push('\n');
                            out.push_str(&indent(doc, 6));
                            out.push('\n');
                        }
                    }
                }
            }
            _ => {}
        }
    }

    out
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

    // ── H9c: type hints ──────────────────────────────────────────────────

    #[test]
    fn type_hints_rendered_by_default() {
        let src = "\
def add(a: int, b: int = 0) -> int:
    \"\"\"Add.\"\"\"
    return a + b
";
        let rst = document_module_source("m", src).unwrap();
        assert!(
            rst.contains(".. py:function:: add(a: int, b: int = 0) -> int"),
            "got:\n{rst}"
        );
    }

    #[test]
    fn type_hints_stripped_when_none_mode() {
        let src = "\
def add(a: int, b: int = 0) -> int:
    pass
";
        let mut opts = AutodocOptions::legacy_default();
        opts.typehints = TypeHints::None;
        let rst = document_module_source_with_options("m", src, &opts).unwrap();
        assert!(rst.contains(".. py:function:: add(a, b=0)"), "got:\n{rst}");
        assert!(!rst.contains("int"), "got:\n{rst}");
    }

    #[test]
    fn subscript_type_hint_renders() {
        let src = "\
from typing import Optional
def f(x: Optional[int]) -> None:
    pass
";
        let rst = document_module_source("m", src).unwrap();
        assert!(
            rst.contains(".. py:function:: f(x: Optional[int]) -> None"),
            "got:\n{rst}"
        );
    }

    #[test]
    fn pep604_union_type_hint_renders() {
        let src = "def f(x: int | None):\n    pass\n";
        let rst = document_module_source("m", src).unwrap();
        assert!(
            rst.contains(".. py:function:: f(x: int | None)"),
            "got:\n{rst}"
        );
    }

    // ── H9c: decorators ──────────────────────────────────────────────────

    #[test]
    fn property_renders_without_signature() {
        let src = "\
class C:
    @property
    def value(self):
        \"\"\"The value.\"\"\"
        return 1
";
        let rst = document_module_source("m", src).unwrap();
        assert!(rst.contains(".. py:property:: value\n"), "got:\n{rst}");
        assert!(!rst.contains("value(self)"), "got:\n{rst}");
    }

    #[test]
    fn staticmethod_and_classmethod_render_distinct_directives() {
        let src = "\
class C:
    @staticmethod
    def make():
        pass

    @classmethod
    def from_thing(cls):
        pass
";
        let rst = document_module_source("m", src).unwrap();
        assert!(rst.contains(".. py:staticmethod:: make()"), "got:\n{rst}");
        assert!(
            rst.contains(".. py:classmethod:: from_thing(cls)"),
            "got:\n{rst}"
        );
    }

    #[test]
    fn exception_subclass_renders_as_exception_directive() {
        let src = "\
class MyError(Exception):
    \"\"\"Custom error.\"\"\"
    pass
";
        let rst = document_module_source("m", src).unwrap();
        assert!(
            rst.contains(".. py:exception:: MyError(Exception)"),
            "got:\n{rst}"
        );
    }

    // ── H9b: member selection options ────────────────────────────────────

    #[test]
    fn no_members_option_shows_only_docstring() {
        let src = "\"\"\"Doc.\"\"\"\n\ndef a():\n    pass\n";
        let opts = AutodocOptions::default(); // members: Unset
        let rst = document_module_source_with_options("m", src, &opts).unwrap();
        assert!(rst.contains(".. py:module:: m"));
        assert!(!rst.contains("py:function"), "got:\n{rst}");
    }

    #[test]
    fn undoc_members_false_hides_undocumented() {
        let src = "\
def documented():
    \"\"\"Has docs.\"\"\"
    pass

def undocumented():
    pass
";
        let opts = AutodocOptions {
            members: MemberSelector::All,
            undoc_members: false,
            ..Default::default()
        };
        let rst = document_module_source_with_options("m", src, &opts).unwrap();
        assert!(rst.contains("py:function:: documented"), "got:\n{rst}");
        assert!(!rst.contains("undocumented"), "got:\n{rst}");
    }

    #[test]
    fn explicit_members_list_bypasses_undoc_filter() {
        let src = "\
def a():
    pass

def b():
    pass
";
        let opts = AutodocOptions {
            members: MemberSelector::Named(vec!["a".to_string()]),
            undoc_members: false,
            ..Default::default()
        };
        let rst = document_module_source_with_options("m", src, &opts).unwrap();
        assert!(rst.contains("py:function:: a()"), "got:\n{rst}");
        assert!(!rst.contains("py:function:: b()"), "got:\n{rst}");
    }

    #[test]
    fn private_members_option_includes_underscored_names() {
        let src = "\
class C:
    def pub(self):
        \"\"\"Doc.\"\"\"
        pass

    def _priv(self):
        \"\"\"Doc.\"\"\"
        pass
";
        let mut opts = AutodocOptions::legacy_default();
        opts.private_members = MemberSelector::All;
        let rst = document_module_source_with_options("m", src, &opts).unwrap();
        assert!(rst.contains("py:method:: pub(self)"), "got:\n{rst}");
        assert!(rst.contains("py:method:: _priv(self)"), "got:\n{rst}");
    }

    #[test]
    fn exclude_members_always_wins() {
        let src = "\
def a():
    \"\"\"Doc.\"\"\"
    pass

def b():
    \"\"\"Doc.\"\"\"
    pass
";
        let mut opts = AutodocOptions::legacy_default();
        opts.exclude_members.insert("b".to_string());
        let rst = document_module_source_with_options("m", src, &opts).unwrap();
        assert!(rst.contains("py:function:: a()"), "got:\n{rst}");
        assert!(!rst.contains("py:function:: b()"), "got:\n{rst}");
    }

    #[test]
    fn member_order_alphabetical() {
        let src = "\
def zeta():
    \"\"\"Doc.\"\"\"
    pass

def alpha():
    \"\"\"Doc.\"\"\"
    pass
";
        let mut opts = AutodocOptions::legacy_default();
        opts.member_order = MemberOrder::Alphabetical;
        let rst = document_module_source_with_options("m", src, &opts).unwrap();
        let alpha_pos = rst.find("py:function:: alpha").unwrap();
        let zeta_pos = rst.find("py:function:: zeta").unwrap();
        assert!(alpha_pos < zeta_pos, "got:\n{rst}");
    }

    #[test]
    fn module_all_restricts_bare_members() {
        let src = "\
__all__ = ['keep']

def keep():
    pass

def drop():
    pass
";
        let opts = AutodocOptions {
            members: MemberSelector::All,
            undoc_members: true,
            ..Default::default()
        };
        let rst = document_module_source_with_options("m", src, &opts).unwrap();
        assert!(rst.contains("py:function:: keep()"), "got:\n{rst}");
        assert!(!rst.contains("py:function:: drop()"), "got:\n{rst}");
    }

    // ── select_members unit tests ────────────────────────────────────────

    #[test]
    fn select_members_groupwise_orders_by_kind_then_name() {
        let candidates = vec![
            MemberCandidate {
                name: "b_method".to_string(),
                has_doc: true,
                source_order: 0,
                kind: MemberKindTag::Method,
            },
            MemberCandidate {
                name: "a_attr".to_string(),
                has_doc: true,
                source_order: 1,
                kind: MemberKindTag::Attribute,
            },
            MemberCandidate {
                name: "a_method".to_string(),
                has_doc: true,
                source_order: 2,
                kind: MemberKindTag::Method,
            },
        ];
        let opts = AutodocOptions {
            members: MemberSelector::All,
            member_order: MemberOrder::Groupwise,
            ..Default::default()
        };
        let selected = select_members(&candidates, &opts, None);
        assert_eq!(selected, vec!["a_attr", "a_method", "b_method"]);
    }

    // ── H9a static-path fallback (runtime unavailable) ───────────────────

    #[test]
    fn document_module_auto_falls_back_to_static_for_unimportable_module() {
        let tmpdir = tempfile::tempdir().unwrap();
        let path = tmpdir.path().join("totally_unimportable_mod_xyz.py");
        std::fs::write(&path, "def a():\n    \"\"\"Doc.\"\"\"\n    pass\n").unwrap();
        let rst = document_module_auto(
            &path,
            "totally_unimportable_mod_xyz_does_not_exist",
            &AutodocOptions::legacy_default(),
            &[],
        )
        .unwrap();
        assert!(rst.contains("py:function:: a()"), "got:\n{rst}");
    }

    #[test]
    fn document_module_auto_uses_runtime_when_importable() {
        // `json` is always importable in the embedded interpreter; the
        // runtime path should be used (real member list, not the static
        // fallback file which we deliberately make wrong/empty).
        let tmpdir = tempfile::tempdir().unwrap();
        let path = tmpdir.path().join("json.py");
        std::fs::write(&path, "# not the real json module\n").unwrap();
        let opts = AutodocOptions {
            members: MemberSelector::All,
            undoc_members: true,
            ..Default::default()
        };
        let rst = document_module_auto(&path, "json", &opts, &[]).unwrap();
        assert!(rst.contains("py:function:: dumps"), "got:\n{rst}");
    }
}
