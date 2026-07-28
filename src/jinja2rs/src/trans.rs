//! `jinja2rs::trans` — `{% trans %}` / `{% pluralize %}` / `{% endtrans %}`
//! support (Jinja2's `jinja2.ext.i18n` extension).
//!
//! ## Design
//!
//! `minijinja` (the engine `jinja2rs` wraps) has no extension mechanism for
//! adding new block tags to its parser/compiler — unlike Python Jinja2's
//! `Extension` API, there is no hook to register a new `Stmt` AST node.
//! Forking the vendored `minijinja` parser to add one would be a much larger,
//! higher-risk change than this feature warrants.
//!
//! Instead, `{% trans %}` support is implemented as a **source-to-source
//! desugaring pass** ([`preprocess_trans`]) that runs before a template is
//! compiled: every `{% trans ... %}...{% endtrans %}` block (with an
//! optional `{% pluralize ... %}` split) is rewritten into a call to a
//! single registered global function, `__trans(vars, msgid, msgid_plural,
//! count)`, backed by [`TransGlobal`]. This keeps the change entirely inside
//! `jinja2rs` and requires no modification to the vendored `minijinja` crate.
//!
//! `TransGlobal` optionally wraps an [`I18nProvider`](crate::i18n::I18nProvider)
//! — the same provider [`crate::Environment::install_gettext`] already uses.
//! When no provider is installed, `{% trans %}` blocks still render (in the
//! source language), using naive English pluralization (`n == 1` → singular).
//! `sphinxdocrs` may install a real provider (backed by its own `.mo`/`.po`
//! catalog lookup) via [`crate::Environment::set_trans_provider`], or leave
//! it unset for a plain passthrough — "the sphinxdocrs lookup, or not".
//!
//! ## What is supported
//!
//! - `{% trans %}...{% endtrans %}` — plain translatable block.
//! - `{% trans name %}...{% endtrans %}` — shorthand binding (`name=name`).
//! - `{% trans name=expr, other=expr2 %}...{% endtrans %}` — explicit
//!   bindings; `expr` may be any valid Jinja expression (e.g. `copyright|e`).
//! - `{% trans trimmed ... %}` / `{% trans notrimmed ... %}` — collapse
//!   internal whitespace/newlines to single spaces (default: not trimmed).
//! - `{% pluralize %}` / `{% pluralize count_expr %}` — plural body; with no
//!   argument, the count expression defaults to the bare variable `count`
//!   (matching upstream Jinja2's default).
//! - Bare `{{ name }}` interpolations inside the body that reference a name
//!   already bound (via the tag or a plain context variable) are recovered
//!   as `%(name)s` gettext-style placeholders.
//!
//! **Accepted deviation:** only simple `{{ name }}` output tags are
//! recognized as placeholders inside the body proper; anything else (a
//! filtered or computed expression written directly in the body rather than
//! bound via `name=expr` in the tag) is auto-bound under a synthetic
//! `_trans_argN` name — more permissive than upstream Jinja2, which requires
//! an explicit binding for non-trivial expressions.

use std::borrow::Cow;
use std::fmt;
use std::sync::{Arc, Mutex};

use minijinja::value::{Object, ObjectRepr, Value};
use minijinja::{Error, ErrorKind, State};

use crate::i18n::I18nProvider;

// ── GettextAlias (`_`) ─────────────────────────────────────────────────────────

/// The `_` global — Jinja2 `ext.i18n`'s shorthand for `gettext`, used
/// throughout real Sphinx templates for bare literal strings (e.g.
/// `{{ _('Search') }}`), as distinct from a `{% trans %}` block (which
/// additionally supports variable interpolation and pluralization).
///
/// Shares the same provider cell as [`TransGlobal`] (both are backed by
/// [`crate::Environment::set_trans_provider`]) — one control point for every
/// translation surface a template can use.
pub struct GettextAlias {
    provider: Arc<Mutex<Option<I18nProvider>>>,
}

impl GettextAlias {
    pub fn new(provider: Arc<Mutex<Option<I18nProvider>>>) -> Self {
        Self { provider }
    }
}

impl fmt::Debug for GettextAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<GettextAlias>")
    }
}

impl fmt::Display for GettextAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<GettextAlias>")
    }
}

impl Object for GettextAlias {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    /// `_(msgid)` — translate via the installed provider, or return `msgid`
    /// unchanged when none is installed.
    fn call(self: &Arc<Self>, _state: &State<'_, '_>, args: &[Value]) -> Result<Value, Error> {
        let msgid = args
            .first()
            .map(|v| v.to_string())
            .ok_or_else(|| Error::new(ErrorKind::MissingArgument, "_() requires a message"))?;
        let translated = match &*self.provider.lock().unwrap() {
            Some(provider) => provider.gettext(&msgid),
            None => msgid,
        };
        Ok(Value::from(translated))
    }
}

// ── TransGlobal ───────────────────────────────────────────────────────────────

/// The `__trans` global object that desugared `{% trans %}` blocks call into.
///
/// Shared (`Arc`) so [`crate::Environment::set_trans_provider`] can install a
/// provider after construction and have it take effect for every template.
pub struct TransGlobal {
    provider: Arc<Mutex<Option<I18nProvider>>>,
}

impl TransGlobal {
    /// Wrap a shared provider cell. Cloning the same `Arc<Mutex<_>>` into an
    /// external handle (e.g. on [`crate::Environment`]) lets callers install
    /// a provider after this object has already been registered as a
    /// template global (`Value::from_object` wraps its own internal `Arc`,
    /// separate from any `Arc<TransGlobal>` the caller might hold, so the
    /// *shared cell* — not the `TransGlobal` struct itself — is what must be
    /// shared).
    pub fn new(provider: Arc<Mutex<Option<I18nProvider>>>) -> Self {
        Self { provider }
    }
}

impl fmt::Debug for TransGlobal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<TransGlobal>")
    }
}

impl fmt::Display for TransGlobal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<TransGlobal>")
    }
}

impl Object for TransGlobal {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    /// `__trans(vars, msgid, msgid_plural, count)`.
    ///
    /// `msgid_plural` and `count` are `none` when the source block had no
    /// `{% pluralize %}` split.
    fn call(self: &Arc<Self>, _state: &State<'_, '_>, args: &[Value]) -> Result<Value, Error> {
        let vars = args.first().cloned().unwrap_or(Value::UNDEFINED);
        let msgid = args
            .get(1)
            .map(|v| v.to_string())
            .ok_or_else(|| Error::new(ErrorKind::MissingArgument, "__trans requires a msgid"))?;
        let msgid_plural = args
            .get(2)
            .filter(|v| !matches!(v.kind(), minijinja::value::ValueKind::None))
            .map(|v| v.to_string());
        let count = args
            .get(3)
            .filter(|v| !matches!(v.kind(), minijinja::value::ValueKind::None));

        let translated = {
            let guard = self.provider.lock().unwrap();
            match (&*guard, &msgid_plural, count) {
                (Some(provider), Some(plural), Some(count_val)) => {
                    let n = value_to_count(count_val);
                    provider.ngettext(&msgid, plural, n)
                }
                (None, Some(plural), Some(count_val)) => {
                    if value_to_count(count_val) == 1 {
                        msgid.clone()
                    } else {
                        plural.clone()
                    }
                }
                (Some(provider), _, _) => provider.gettext(&msgid),
                (None, _, _) => msgid.clone(),
            }
        };

        let result = substitute_placeholders(&translated, &vars);
        Ok(markupsafers::minijinja_compat::markup_to_value(
            markupsafers::Markup::from_safe(result),
        ))
    }
}

/// Best-effort conversion of a count [`Value`] to a non-negative `usize`.
fn value_to_count(v: &Value) -> usize {
    i64::try_from(v.clone())
        .ok()
        .and_then(|n| usize::try_from(n).ok())
        .unwrap_or_else(|| {
            v.to_string()
                .parse::<f64>()
                .map(|f| f.max(0.0) as usize)
                .unwrap_or(1)
        })
}

/// Replace every `%(name)s` placeholder in `template` with the corresponding
/// entry from `vars` (a Jinja map value), and `%%` with a literal `%`.
///
/// Missing keys are left as an empty string (best-effort; upstream gettext
/// would raise `KeyError`, but a silently-empty substitution keeps a
/// misconfigured template building rather than failing the whole page).
fn substitute_placeholders(template: &str, vars: &Value) -> String {
    let mut out = String::with_capacity(template.len());
    let bytes = template.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if template[i..].starts_with("%%") {
                out.push('%');
                i += 2;
                continue;
            }
            if template[i..].starts_with("%(") {
                if let Some(close_paren) = template[i + 2..].find(")s") {
                    let name = &template[i + 2..i + 2 + close_paren];
                    let value = vars
                        .get_item(&Value::from(name))
                        .ok()
                        .filter(|v| !matches!(v.kind(), minijinja::value::ValueKind::Undefined))
                        .map(|v| v.to_string())
                        .unwrap_or_default();
                    out.push_str(&value);
                    i += 2 + close_paren + 2;
                    continue;
                }
            }
        }
        // Advance by one UTF-8 character.
        let ch_len = template[i..]
            .chars()
            .next()
            .map(|c| c.len_utf8())
            .unwrap_or(1);
        out.push_str(&template[i..i + ch_len]);
        i += ch_len;
    }
    out
}

// ── preprocess_trans ──────────────────────────────────────────────────────────

/// Rewrite every `{% trans %}...{% endtrans %}` block in `source` into a
/// `{{ __trans(...) }}` call. Returns `Cow::Borrowed` unchanged when the
/// source contains no `trans` block tag (the common case), so callers that
/// only need to preprocess `'static` template constants can skip leaking
/// memory when nothing was rewritten.
pub fn preprocess_trans(source: &str) -> Cow<'_, str> {
    // Fast path: no possible `{% trans %}` tag at all.
    if !source.contains("trans") {
        return Cow::Borrowed(source);
    }

    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    let mut rewrote_any = false;

    loop {
        let Some((before, tag_inner, tag_close_idx)) = next_trans_tag(rest) else {
            out.push_str(rest);
            break;
        };
        out.push_str(before);
        rewrote_any = true;

        let after_tag = &rest[tag_close_idx..];
        match parse_trans_block(tag_inner, after_tag) {
            Some((replacement, remainder)) => {
                out.push_str(&replacement);
                rest = remainder;
            }
            None => {
                // Malformed block (no matching `{% endtrans %}`): emit the
                // opening tag verbatim and keep scanning — minijinja will
                // raise a clear "unknown tag" style error when it tries to
                // parse the untouched `{% trans %}` text.
                out.push_str("{%");
                out.push_str(tag_inner);
                out.push_str("%}");
                rest = after_tag;
            }
        }
    }

    if rewrote_any {
        Cow::Owned(out)
    } else {
        Cow::Borrowed(source)
    }
}

/// Find the next `{% trans ... %}` tag in `source`.
///
/// Returns `(text_before_tag, tag_inner_text, index_just_after_tag)`.
fn next_trans_tag(source: &str) -> Option<(&str, &str, usize)> {
    let mut search_from = 0;
    loop {
        let open_rel = source[search_from..].find("{%")?;
        let open = search_from + open_rel;
        let after_open = open + 2;
        let close_rel = source[after_open..].find("%}")?;
        let close = after_open + close_rel;
        let tag_end = close + 2;
        let inner = &source[after_open..close];
        let word = tag_keyword(inner);
        if word == "trans" {
            return Some((&source[..open], inner, tag_end));
        }
        search_from = tag_end;
    }
}

/// Extract the leading keyword of a `{% ... %}` tag body, ignoring
/// whitespace-control markers (`-`/`+`) and surrounding whitespace.
fn tag_keyword(tag_inner: &str) -> &str {
    let trimmed = tag_inner.trim().trim_start_matches(['-', '+']).trim_start();
    let end = trimmed
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(trimmed.len());
    &trimmed[..end]
}

/// The tag arguments following the keyword, with whitespace-control markers
/// and the closing `-`/`+` stripped.
fn tag_args(tag_inner: &str, keyword: &str) -> String {
    let trimmed = tag_inner.trim().trim_start_matches(['-', '+']).trim_start();
    let after_kw = trimmed.strip_prefix(keyword).unwrap_or(trimmed);
    after_kw
        .trim()
        .trim_end_matches(['-', '+'])
        .trim()
        .to_string()
}

/// Given the tag-arg text of an opening `{% trans ... %}` and the source
/// immediately following that tag, parse the full block (through the
/// matching `{% endtrans %}`, with an optional `{% pluralize %}` split) and
/// produce `({{ __trans(...) }}` replacement text, remaining source `)`.
fn parse_trans_block<'a>(trans_tag_inner: &str, after_tag: &'a str) -> Option<(String, &'a str)> {
    let args = tag_args(trans_tag_inner, "trans");
    let (trimmed_mode, args) = strip_trim_modifier(&args);
    let mut bindings: Vec<(String, String)> = Vec::new();
    for chunk in split_top_level(args, ',') {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }
        if let Some(eq) = find_top_level_eq(chunk) {
            let name = chunk[..eq].trim().to_string();
            let expr = chunk[eq + 1..].trim().to_string();
            bindings.push((name, expr));
        } else {
            bindings.push((chunk.to_string(), chunk.to_string()));
        }
    }

    // Scan forward for `{% pluralize %}` or `{% endtrans %}`.
    let (singular_body, plural_part, remainder) = split_body(after_tag)?;

    let mut var_order: Vec<String> = bindings.iter().map(|(n, _)| n.clone()).collect();
    let mut var_exprs: Vec<(String, String)> = bindings;
    let mut synth_counter = 0usize;

    let msgid = build_msgid(
        singular_body,
        trimmed_mode,
        &mut var_order,
        &mut var_exprs,
        &mut synth_counter,
    );

    let (msgid_plural, count_expr) = if let Some((plural_tag_args, plural_body)) = plural_part {
        let plural_msgid = build_msgid(
            plural_body,
            trimmed_mode,
            &mut var_order,
            &mut var_exprs,
            &mut synth_counter,
        );
        let count_expr = if plural_tag_args.trim().is_empty() {
            "count".to_string()
        } else {
            plural_tag_args.trim().to_string()
        };
        (Some(plural_msgid), Some(count_expr))
    } else {
        (None, None)
    };

    let mut vars_literal = String::from("{");
    for (i, name) in var_order.iter().enumerate() {
        if i > 0 {
            vars_literal.push_str(", ");
        }
        let expr = var_exprs
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, e)| e.as_str())
            .unwrap_or(name.as_str());
        vars_literal.push('"');
        vars_literal.push_str(&escape_jinja_string(name));
        vars_literal.push_str("\": (");
        vars_literal.push_str(expr);
        vars_literal.push(')');
    }
    vars_literal.push('}');

    let msgid_literal = format!("\"{}\"", escape_jinja_string(&msgid));
    let msgid_plural_literal = match &msgid_plural {
        Some(p) => format!("\"{}\"", escape_jinja_string(p)),
        None => "none".to_string(),
    };
    let count_literal = count_expr.unwrap_or_else(|| "none".to_string());

    let replacement = format!(
        "{{{{ __trans({vars_literal}, {msgid_literal}, {msgid_plural_literal}, {count_literal}) }}}}"
    );

    Some((replacement, remainder))
}

/// Strip a leading `trimmed`/`notrimmed` modifier from trans-tag args.
///
/// Returns `(is_trimmed, remaining_args)`.
fn strip_trim_modifier(args: &str) -> (bool, &str) {
    let trimmed_args = args.trim_start();
    if let Some(rest) = trimmed_args.strip_prefix("trimmed") {
        if rest.is_empty() || rest.starts_with(char::is_whitespace) || rest.starts_with(',') {
            return (true, rest.trim_start_matches(','));
        }
    }
    if let Some(rest) = trimmed_args.strip_prefix("notrimmed") {
        if rest.is_empty() || rest.starts_with(char::is_whitespace) || rest.starts_with(',') {
            return (false, rest.trim_start_matches(','));
        }
    }
    (false, args)
}

/// `(singular_body, Some((pluralize_tag_args, plural_body)), remainder_after_endtrans)`.
type SplitBody<'a> = (&'a str, Option<(&'a str, &'a str)>, &'a str);

/// Split `source` (the text right after an opening `{% trans %}` tag) into
/// its singular body, an optional pluralize split, and the remainder.
fn split_body(source: &str) -> Option<SplitBody<'_>> {
    let mut search_from = 0;
    loop {
        let open_rel = source[search_from..].find("{%")?;
        let open = search_from + open_rel;
        let after_open = open + 2;
        let close_rel = source[after_open..].find("%}")?;
        let close = after_open + close_rel;
        let tag_end = close + 2;
        let inner = &source[after_open..close];
        let word = tag_keyword(inner);
        match word {
            "pluralize" => {
                let plural_args = tag_args(inner, "pluralize");
                let singular_body = &source[..open];
                // Now find the matching `{% endtrans %}` after this point.
                let rest = &source[tag_end..];
                let (plural_body, remainder) = find_endtrans(rest)?;
                return Some((
                    singular_body,
                    Some((leak_str(plural_args), plural_body)),
                    remainder,
                ));
            }
            "endtrans" => {
                let singular_body = &source[..open];
                return Some((singular_body, None, &source[tag_end..]));
            }
            _ => {
                search_from = tag_end;
            }
        }
    }
}

/// Leak an owned `String` to obtain a `&'static str` for the rare case where
/// we need to return borrowed-shaped data derived from a temporary
/// computation (the `pluralize` argument text). This only runs once per
/// `{% pluralize %}` tag encountered during preprocessing (a one-time,
/// build-time cost), not per-render.
fn leak_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/// Find the next `{% endtrans %}` tag, returning `(body_before_it, remainder_after_it)`.
fn find_endtrans(source: &str) -> Option<(&str, &str)> {
    let mut search_from = 0;
    loop {
        let open_rel = source[search_from..].find("{%")?;
        let open = search_from + open_rel;
        let after_open = open + 2;
        let close_rel = source[after_open..].find("%}")?;
        let close = after_open + close_rel;
        let tag_end = close + 2;
        let inner = &source[after_open..close];
        if tag_keyword(inner) == "endtrans" {
            return Some((&source[..open], &source[tag_end..]));
        }
        search_from = tag_end;
    }
}

/// Extract the message id from a trans-block body: literal text is kept
/// as-is, `{{ name }}` (or `{{- name -}}` etc.) output tags become
/// `%(name)s` gettext placeholders, registering `name` (and its bound
/// expression) in `var_order`/`var_exprs` if not already present.
///
/// Non-identifier `{{ expr }}` output tags are auto-bound under a synthetic
/// `_trans_argN` name (see the module-level accepted deviation).
fn build_msgid(
    body: &str,
    trimmed: bool,
    var_order: &mut Vec<String>,
    var_exprs: &mut Vec<(String, String)>,
    synth_counter: &mut usize,
) -> String {
    let mut msgid = String::with_capacity(body.len());
    let mut rest = body;
    loop {
        let Some(open_rel) = rest.find("{{") else {
            msgid.push_str(rest);
            break;
        };
        let Some(close_rel) = rest[open_rel + 2..].find("}}") else {
            msgid.push_str(rest);
            break;
        };
        msgid.push_str(&rest[..open_rel]);
        let inner = &rest[open_rel + 2..open_rel + 2 + close_rel];
        let expr = inner
            .trim()
            .trim_start_matches('-')
            .trim_end_matches('-')
            .trim();

        let is_ident = !expr.is_empty()
            && expr
                .chars()
                .next()
                .map(|c| c.is_alphabetic() || c == '_')
                .unwrap_or(false)
            && expr.chars().all(|c| c.is_alphanumeric() || c == '_');

        let name = if is_ident {
            let name = expr.to_string();
            if !var_order.contains(&name) {
                var_order.push(name.clone());
                var_exprs.push((name.clone(), name.clone()));
            }
            name
        } else {
            *synth_counter += 1;
            let name = format!("_trans_arg{synth_counter}");
            var_order.push(name.clone());
            var_exprs.push((name.clone(), expr.to_string()));
            name
        };

        msgid.push_str("%(");
        msgid.push_str(&name);
        msgid.push_str(")s");

        rest = &rest[open_rel + 2 + close_rel + 2..];
    }

    if trimmed {
        msgid.split_whitespace().collect::<Vec<_>>().join(" ")
    } else {
        msgid
    }
}

/// Split `s` on top-level occurrences of `delim`, respecting nesting of
/// `()`/`[]`/`{}` and quoted strings (so `f(a, b)` or `"a, b"` are not
/// split internally).
fn split_top_level(s: &str, delim: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut in_quote: Option<char> = None;
    let mut start = 0usize;
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let c = s[i..].chars().next().unwrap();
        if let Some(q) = in_quote {
            if c == q {
                in_quote = None;
            }
        } else {
            match c {
                '\'' | '"' => in_quote = Some(c),
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                c if c == delim && depth == 0 => {
                    parts.push(&s[start..i]);
                    start = i + c.len_utf8();
                }
                _ => {}
            }
        }
        i += c.len_utf8();
    }
    parts.push(&s[start..]);
    parts
}

/// Find the byte index of the first top-level `=` in `s` (not `==`, and not
/// inside nested brackets/quotes) — used to split a `name=expr` binding.
fn find_top_level_eq(s: &str) -> Option<usize> {
    let mut depth = 0i32;
    let mut in_quote: Option<char> = None;
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let c = s[i..].chars().next().unwrap();
        if let Some(q) = in_quote {
            if c == q {
                in_quote = None;
            }
        } else {
            match c {
                '\'' | '"' => in_quote = Some(c),
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                '=' if depth == 0 => {
                    // Reject `==`, `!=`, `<=`, `>=`.
                    let prev_is_cmp = i > 0 && matches!(bytes[i - 1], b'=' | b'!' | b'<' | b'>');
                    let next_is_eq = s[i + 1..].starts_with('=');
                    if !prev_is_cmp && !next_is_eq {
                        return Some(i);
                    }
                }
                _ => {}
            }
        }
        i += c.len_utf8();
    }
    None
}

/// Escape a string for embedding as a double-quoted Jinja/minijinja string
/// literal.
fn escape_jinja_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passthrough_when_no_trans_block() {
        let src = "<p>{{ title }}</p>";
        assert!(matches!(preprocess_trans(src), Cow::Borrowed(_)));
    }

    #[test]
    fn simple_trans_block_rewritten() {
        let src = "{% trans %}Hello{% endtrans %}";
        let out = preprocess_trans(src);
        assert!(out.contains("__trans("), "got: {out}");
        assert!(out.contains("\"Hello\""), "got: {out}");
    }

    #[test]
    fn trans_block_with_variable() {
        let src = "{% trans %}Hello {{ name }}!{% endtrans %}";
        let out = preprocess_trans(src);
        assert!(out.contains("%(name)s"), "got: {out}");
        assert!(out.contains("\"name\": (name)"), "got: {out}");
    }

    #[test]
    fn trans_block_with_explicit_binding() {
        let src = "{% trans copyright=copyright_line|e %}&#169; {{ copyright }}.{% endtrans %}";
        let out = preprocess_trans(src);
        assert!(
            out.contains("\"copyright\": (copyright_line|e)"),
            "got: {out}"
        );
        assert!(out.contains("%(copyright)s"), "got: {out}");
    }

    #[test]
    fn trans_block_with_pluralize() {
        let src =
            "{% trans count=count %}{{ count }} item{% pluralize %}{{ count }} items{% endtrans %}";
        let out = preprocess_trans(src);
        assert!(out.contains("\"count\": (count)"), "got: {out}");
        assert!(out.contains("%(count)s item\""), "got: {out}");
        assert!(out.contains("%(count)s items\""), "got: {out}");
        assert!(
            out.contains(", count)"),
            "expected count expr passed, got: {out}"
        );
    }

    #[test]
    fn trans_block_trimmed_collapses_whitespace() {
        let src = "{% trans trimmed %}\n  Hello\n  world\n{% endtrans %}";
        let out = preprocess_trans(src);
        assert!(out.contains("\"Hello world\""), "got: {out}");
    }

    #[test]
    fn render_end_to_end_without_provider() {
        let mut env = crate::Environment::new();
        env.add_template("t.html", "{% trans %}Hello, {{ name }}!{% endtrans %}")
            .unwrap();
        let tmpl = env.get_template("t.html").unwrap();
        let out = tmpl.render(serde_json::json!({"name": "World"})).unwrap();
        assert_eq!(out, "Hello, World!");
    }

    #[test]
    fn render_end_to_end_with_provider() {
        let mut env = crate::Environment::new();
        let provider = crate::i18n::I18nProvider::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert(
            "Hello, %(name)s!".to_string(),
            "Bonjour, %(name)s !".to_string(),
        );
        provider.load_translations(dict);
        env.set_trans_provider(provider);
        env.add_template("t.html", "{% trans %}Hello, {{ name }}!{% endtrans %}")
            .unwrap();
        let tmpl = env.get_template("t.html").unwrap();
        let out = tmpl.render(serde_json::json!({"name": "World"})).unwrap();
        assert_eq!(out, "Bonjour, World !");
    }

    #[test]
    fn render_pluralize_without_provider_uses_naive_english_rule() {
        let mut env = crate::Environment::new();
        env.add_template(
            "t.html",
            "{% trans count=count %}{{ count }} item{% pluralize %}{{ count }} items{% endtrans %}",
        )
        .unwrap();
        let tmpl = env.get_template("t.html").unwrap();
        assert_eq!(
            tmpl.render(serde_json::json!({"count": 1})).unwrap(),
            "1 item"
        );
        assert_eq!(
            tmpl.render(serde_json::json!({"count": 3})).unwrap(),
            "3 items"
        );
    }

    #[test]
    fn html_body_not_double_escaped() {
        let mut env = crate::Environment::new();
        env.add_template(
            "t.html",
            "{% trans link=link %}See {{ link }} here{% endtrans %}",
        )
        .unwrap();
        let tmpl = env.get_template("t.html").unwrap();
        let out = tmpl
            .render(serde_json::json!({"link": "<a href=\"/x\">x</a>"}))
            .unwrap();
        assert!(out.contains("<a href=\"/x\">x</a>"), "got: {out}");
    }
}
