//! `sphinxdocrs::domains::scan` — text-level RST scanners feeding the
//! **H3a**/**H3c**/**H5b** domain data and xref recovery.
//!
//! See the accepted-deviation note on `domains` (the module doc comment)
//! for why these operate on raw source text rather than a structural
//! doctree: `docutilsrs`'s parser doesn't yet model Sphinx-specific
//! directives (`glossary`, `rst:directive`, ...) or emit a `pending_xref`
//! node for cross-reference roles. These scanners follow the same
//! indentation-based heuristics as `environment::scan_toctree_entries`.

use super::{IndexEntry, IndexEntryKind, PendingXref};

/// Scan for `.. _label:` explicit hyperlink targets, pairing each with
/// the section title immediately following it (if any).
///
/// Returns `(label_name, Some(section_title))` when the target sits
/// directly above an underlined section title, else `(label_name, None)`.
///
/// Mirrors the subset of `docutils.transforms.references.PropagateTargets`
/// and `Sphinx.note_labels` relevant to plain `:ref:` resolution.
/// Overline+title+underline sections and multi-target chains such as
/// `` .. _a:\n.. _b:\n\nTitle\n===== `` are accepted deviations: only the
/// single target directly preceding a title is recognized.
pub fn scan_labels(source: &str) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix(".. _") else {
            continue;
        };
        let Some(name) = rest.strip_suffix(':') else {
            continue;
        };
        let name = name
            .strip_prefix('`')
            .and_then(|s| s.strip_suffix('`'))
            .unwrap_or(name);
        if name.is_empty() {
            continue;
        }

        let mut j = i + 1;
        while j < lines.len() && lines[j].trim().is_empty() {
            j += 1;
        }
        let sectionname = if j < lines.len() && j + 1 < lines.len() {
            let title_line = lines[j].trim();
            let under_line = lines[j + 1].trim();
            if !title_line.is_empty() && is_section_underline(under_line, title_line) {
                Some(title_line.to_string())
            } else {
                None
            }
        } else {
            None
        };
        out.push((name.to_string(), sectionname));
    }
    out
}

/// `true` if `under` is a valid docutils section-underline for `title`:
/// a single repeated punctuation character, at least as wide as the title.
fn is_section_underline(under: &str, title: &str) -> bool {
    const SECTION_CHARS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    let Some(c) = under.chars().next() else {
        return false;
    };
    if !SECTION_CHARS.contains(c) {
        return false;
    }
    under.chars().all(|ch| ch == c) && under.chars().count() >= title.chars().count()
}

/// Scan `.. glossary::` directive bodies, extracting each term line at
/// the block's content-indentation level. Multiple consecutive term
/// lines that share one definition (a comma-free multi-term entry) are
/// each returned individually, matching upstream (every term gets its
/// own `objects`/`terms` entry).
pub fn scan_glossary_terms(source: &str) -> Vec<String> {
    let mut terms = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        let Some(rest) = trimmed.strip_prefix("..") else {
            i += 1;
            continue;
        };
        let rest = rest.trim_start();
        if !rest.starts_with("glossary::") {
            i += 1;
            continue;
        }
        let base_indent = lines[i].len() - trimmed.len();
        i += 1;
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
        if i >= lines.len() {
            continue;
        }
        let content_indent = lines[i].len() - lines[i].trim_start().len();
        if content_indent <= base_indent {
            continue;
        }
        while i < lines.len() {
            let line = lines[i];
            if line.trim().is_empty() {
                i += 1;
                continue;
            }
            let indent = line.len() - line.trim_start().len();
            if indent < content_indent {
                break;
            }
            if indent == content_indent {
                let body = line.trim();
                if !body.starts_with(':') {
                    terms.push(body.to_string());
                }
            }
            i += 1;
        }
    }
    terms
}

/// Scan `.. rst:directive:: name` / `.. rst:role:: name` object
/// descriptions, returning `(objtype, name)` pairs (`objtype` is
/// `"directive"` or `"role"`).
pub fn scan_rst_domain_objects(source: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix(".. rst:directive::") {
            let name = rest.trim();
            if !name.is_empty() {
                out.push(("directive".to_string(), name.to_string()));
            }
        } else if let Some(rest) = trimmed.strip_prefix(".. rst:role::") {
            let name = rest.trim();
            if !name.is_empty() {
                out.push(("role".to_string(), name.to_string()));
            }
        }
    }
    out
}

/// Scan for inline cross-reference roles (`` :ref:`target` ``,
/// `` :doc:`Title <target>` ``, `` :rst:dir:`toctree` ``, ...) that this
/// port knows how to resolve.
///
/// Recognizes the standard `:role:`content`` and domain-prefixed
/// `:domain:role:`content`` forms. Within `content`, the
/// `` Title <target> `` phrase-reference form is split into an explicit
/// title + target; otherwise `content` is the target verbatim.
///
/// **Accepted deviation:** only roles this port resolves are recognized
/// (`ref`, `doc`, `term`, `numref`, `keyword`; any `rst:*` role; the `py`
/// domain's `func`/`class`/`meth`/`classmethod`/`staticmethod`/`attr`/
/// `data`/`exc`/`mod`/`obj`; the `js` domain's `func`/`class`/`meth`/
/// `attr`/`data`/`mod`); other domain-prefixed or unprefixed roles
/// (`:option:`, custom roles, ...) are not yet collected — extend the
/// `KNOWN_*_REFTYPES` lists here as more domains land.
pub fn scan_xref_roles(source: &str) -> Vec<PendingXref> {
    const KNOWN_STD_REFTYPES: &[&str] = &["ref", "doc", "term", "numref", "keyword"];
    const KNOWN_PY_REFTYPES: &[&str] = &[
        "func",
        "class",
        "meth",
        "classmethod",
        "staticmethod",
        "attr",
        "data",
        "exc",
        "mod",
        "obj",
    ];
    const KNOWN_JS_REFTYPES: &[&str] = &["func", "class", "meth", "attr", "data", "mod"];
    let mut out = Vec::new();
    for (line_idx, line) in source.lines().enumerate() {
        let bytes = line.as_bytes();
        let mut cursor = 0usize;
        while cursor < bytes.len() {
            let Some(colon) = line[cursor..].find(':') else {
                break;
            };
            let start = cursor + colon;
            if let Some((role, content, end)) = try_match_role_at(line, start) {
                let (domain, reftype) = match role.split_once(':') {
                    Some((d, r)) => (d.to_string(), r.to_string()),
                    None => ("std".to_string(), role.clone()),
                };
                let recognized = match domain.as_str() {
                    "std" => KNOWN_STD_REFTYPES.contains(&reftype.as_str()),
                    "rst" => true,
                    "py" => KNOWN_PY_REFTYPES.contains(&reftype.as_str()),
                    "js" => KNOWN_JS_REFTYPES.contains(&reftype.as_str()),
                    _ => false,
                };
                if recognized {
                    let (raw_target, explicit_title) = split_phrase(&content);
                    // The `~` truncation prefix is generic to every
                    // `XRefRole` upstream (`sphinx.roles.XRefRole.__call__`),
                    // not domain-specific: strip it here so every domain's
                    // `resolve_xref` sees a clean target.
                    let (target, shorten) = match raw_target.strip_prefix('~') {
                        Some(rest) => (rest.to_string(), true),
                        None => (raw_target, false),
                    };
                    out.push(PendingXref {
                        domain,
                        reftype,
                        target,
                        explicit_title,
                        line: line_idx + 1,
                        shorten,
                    });
                }
                cursor = end;
            } else {
                cursor = start + 1;
            }
        }
    }
    out
}

/// Try to match `` :role:`content` `` starting at byte offset `start`
/// (which must point at the leading `:`). Returns `(role, content, end)`.
/// `role` may be a plain name (`ref`) or a domain-prefixed name
/// (`rst:dir`, one embedded colon).
fn try_match_role_at(line: &str, start: usize) -> Option<(String, String, usize)> {
    use std::sync::OnceLock;

    static ROLE_PREFIX_RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = ROLE_PREFIX_RE.get_or_init(|| {
        regex::Regex::new(r"^:([A-Za-z][A-Za-z0-9_+-]*(?::[A-Za-z][A-Za-z0-9_+-]*)?):`").unwrap()
    });

    let slice = line.get(start..)?;
    let caps = re.captures(slice)?;
    let whole_len = caps.get(0)?.as_str().len();
    let role = caps.get(1)?.as_str().to_string();

    let content_start = start + whole_len;
    let rest = &line[content_start..];
    let end_rel = rest.find('`')?;
    let content = rest[..end_rel].to_string();
    Some((role, content, content_start + end_rel + 1))
}

/// Split `` Title <target> `` into `(target, Some(title))`, or return
/// `(content, None)` verbatim when there's no phrase-reference form.
///
/// `pub(crate)` so [`crate::environment::BuildEnvironment::resolve_xref_nodes`]
/// can reuse the exact same label/target splitting when resolving a role
/// node's raw text content, rather than duplicating the logic.
pub(crate) fn split_phrase(content: &str) -> (String, Option<String>) {
    let trimmed = content.trim();
    if let Some(open) = trimmed.rfind('<') {
        if trimmed.ends_with('>') {
            let title = trimmed[..open].trim();
            let target = &trimmed[open + 1..trimmed.len() - 1];
            if !title.is_empty() {
                return (target.trim().to_string(), Some(title.to_string()));
            }
        }
    }
    (trimmed.to_string(), None)
}

// ── H3d / H5e: `.. index::` entry scanning ──────────────────────────────────

/// Scan `.. index::` directive bodies, splitting each raw entry per
/// `sphinx.util.nodes.process_index_entry`'s type-prefix rules: an
/// untyped entry is comma-separated `single` entries; a typed entry
/// starts with `single:`/`pair:`/`triple:`/`see:`/`seealso:` followed by
/// `;`-separated parts; a leading `!` marks a "main" entry.
///
/// Both the inline argument (`.. index:: foo, bar`) and indented body
/// lines (one raw entry per line) are recognized.
///
/// **Accepted deviation:** entries aren't attached to an in-page anchor
/// (there's no node id to reference), so every entry from a document
/// resolves to that document as a whole — the same page-level
/// granularity as the label/toctree scanners.
pub fn scan_index_entries(source: &str) -> Vec<IndexEntry> {
    let mut out = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        let Some(rest) = trimmed.strip_prefix("..") else {
            i += 1;
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix("index::") else {
            i += 1;
            continue;
        };
        let base_indent = lines[i].len() - trimmed.len();
        let inline_arg = rest.trim();
        let mut raw_entries: Vec<String> = Vec::new();
        if !inline_arg.is_empty() {
            raw_entries.push(inline_arg.to_string());
        }
        i += 1;
        while i < lines.len() {
            let l = lines[i];
            if l.trim().is_empty() {
                i += 1;
                continue;
            }
            let indent = l.len() - l.trim_start().len();
            if indent <= base_indent {
                break;
            }
            let body = l.trim();
            if body.starts_with(':') {
                i += 1;
                continue;
            }
            raw_entries.push(body.to_string());
            i += 1;
        }
        for raw in raw_entries {
            out.extend(parse_index_entry(&raw));
        }
    }
    out
}

/// Split one raw `.. index::` entry string into its [`IndexEntry`] form(s).
fn parse_index_entry(raw: &str) -> Vec<IndexEntry> {
    let mut entry = raw.trim();
    let main = entry.starts_with('!');
    if main {
        entry = entry[1..].trim_start();
    }

    for (prefix, kind) in [
        ("single:", IndexEntryKind::Single),
        ("pair:", IndexEntryKind::Pair),
        ("triple:", IndexEntryKind::Triple),
        ("seealso:", IndexEntryKind::SeeAlso),
        ("see:", IndexEntryKind::See),
    ] {
        let Some(value) = entry.strip_prefix(prefix) else {
            continue;
        };
        let value = value.trim();
        return match kind {
            IndexEntryKind::Single => {
                let parts: Vec<&str> = value.splitn(2, ';').map(str::trim).collect();
                let mut entries = vec![IndexEntry {
                    kind: IndexEntryKind::Single,
                    text: parts[0].to_string(),
                    see_target: None,
                    main,
                }];
                if let Some(sub) = parts.get(1).filter(|s| !s.is_empty()) {
                    entries.push(IndexEntry {
                        kind: IndexEntryKind::Single,
                        text: format!("{}; {}", parts[0], sub),
                        see_target: None,
                        main,
                    });
                }
                entries
            }
            IndexEntryKind::Pair => {
                let parts: Vec<&str> = value.split(';').map(str::trim).collect();
                if parts.len() >= 2 {
                    vec![IndexEntry {
                        kind: IndexEntryKind::Pair,
                        text: format!("{}; {}", parts[0], parts[1]),
                        see_target: None,
                        main,
                    }]
                } else {
                    Vec::new()
                }
            }
            IndexEntryKind::Triple => {
                let parts: Vec<&str> = value.split(';').map(str::trim).collect();
                if parts.len() >= 3 {
                    vec![IndexEntry {
                        kind: IndexEntryKind::Triple,
                        text: format!("{}; {}; {}", parts[0], parts[1], parts[2]),
                        see_target: None,
                        main,
                    }]
                } else {
                    Vec::new()
                }
            }
            IndexEntryKind::See | IndexEntryKind::SeeAlso => {
                let parts: Vec<&str> = value.splitn(2, ';').map(str::trim).collect();
                if parts.len() == 2 {
                    vec![IndexEntry {
                        kind,
                        text: parts[0].to_string(),
                        see_target: Some(parts[1].to_string()),
                        main: false,
                    }]
                } else {
                    Vec::new()
                }
            }
        };
    }

    // Untyped: comma-separated single entries.
    entry
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| IndexEntry {
            kind: IndexEntryKind::Single,
            text: s.to_string(),
            see_target: None,
            main,
        })
        .collect()
}

// ── H3b / H3e: shared signature-directive scanner ───────────────────────────

/// One directive-derived domain object recovered by [`scan_domain_objects`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedObject {
    /// The directive type (`"function"`, `"class"`, `"method"`, ...).
    pub objtype: String,
    /// Fully-qualified dotted name (module/class context joined with the
    /// directive's own local name).
    pub fullname: String,
    /// Rendered parameter list (empty for signature-less directives like
    /// `attribute`/`data`/`module`).
    pub signature: String,
}

/// Generic indentation-nested `.. {domain}:{type}:: args` scanner shared
/// by the `py` (**H3b**) and `js` (**H3e**) domains, generalizing the
/// per-domain nesting/context tracking (`PyObject.before_content` /
/// `after_content`'s module/class stack upstream) so neither domain has
/// to reimplement it.
///
/// - `module_types` set the ambient module context (`env.ref_context`
///   upstream); only the literal `"module"` type additionally registers
///   an object (`"currentmodule"` is context-only).
/// - `class_context_types` (`"currentclass"`) push a class-name context
///   without registering an object.
/// - `container_object_types` (`"class"`, `"exception"`) register an
///   object *and* open a nested naming scope for deeper-indented
///   children, mirroring a class body's methods.
/// - every other name in `recognized_types` is a plain object type
///   (`"function"`, `"attribute"`, ...): registered, no nested scope.
///
/// `signature_of(objtype, args)` parses one directive's argument line
/// into `(local_name, rendered_signature)` — the `py` domain hands it a
/// `ruff_python_parser`-backed implementation ([`super::py_domain::py_signature_of`]);
/// `js` a manual paren-matching one ([`super::js_domain::js_signature_of`]).
#[allow(clippy::too_many_arguments)]
pub fn scan_domain_objects(
    source: &str,
    domain: &str,
    module_types: &[&str],
    class_context_types: &[&str],
    container_object_types: &[&str],
    plain_object_types: &[&str],
    signature_of: impl Fn(&str, &str) -> (String, String),
) -> Vec<ScannedObject> {
    let mut out = Vec::new();
    let mut current_module: Option<String> = None;
    let mut class_stack: Vec<(usize, String)> = Vec::new();
    let directive_prefix = format!("{domain}:");

    for line in source.lines() {
        let trimmed = line.trim_start();
        let own_indent = line.len() - trimmed.len();
        let Some(rest) = trimmed.strip_prefix("..") else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix(directive_prefix.as_str()) else {
            continue;
        };
        let Some((ty, args)) = rest.split_once("::") else {
            continue;
        };
        let ty = ty.trim();
        let args = args.trim();

        let recognized = module_types.contains(&ty)
            || class_context_types.contains(&ty)
            || container_object_types.contains(&ty)
            || plain_object_types.contains(&ty);
        if !recognized {
            continue;
        }

        // Dedent: drop any class scope whose directive sat at an indent
        // ≥ this line's, since we've moved past its indented body.
        while class_stack
            .last()
            .is_some_and(|(indent, _)| *indent >= own_indent)
        {
            class_stack.pop();
        }

        if module_types.contains(&ty) {
            if !args.is_empty() {
                current_module = Some(args.to_string());
                if ty == "module" {
                    out.push(ScannedObject {
                        objtype: "module".to_string(),
                        fullname: args.to_string(),
                        signature: String::new(),
                    });
                }
            }
            continue;
        }

        if class_context_types.contains(&ty) {
            if !args.is_empty() {
                class_stack.push((own_indent, args.to_string()));
            }
            continue;
        }

        let (local_name, signature) = signature_of(ty, args);
        if local_name.is_empty() {
            continue;
        }

        let mut parts: Vec<String> = Vec::new();
        if let Some(m) = &current_module {
            parts.push(m.clone());
        }
        for (_, cname) in &class_stack {
            parts.push(cname.clone());
        }
        parts.push(local_name.clone());
        let fullname = parts.join(".");

        out.push(ScannedObject {
            objtype: ty.to_string(),
            fullname,
            signature,
        });

        if container_object_types.contains(&ty) {
            class_stack.push((own_indent, local_name));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_labels_with_section() {
        let src = ".. _my-label:\n\nMy Section\n===========\n\nBody text.\n";
        let labels = scan_labels(src);
        assert_eq!(
            labels,
            vec![("my-label".to_string(), Some("My Section".to_string()))]
        );
    }

    #[test]
    fn scan_labels_backtick_name() {
        let src = ".. _`a label with spaces`:\n\nText.\n";
        let labels = scan_labels(src);
        assert_eq!(labels, vec![("a label with spaces".to_string(), None)]);
    }

    #[test]
    fn scan_labels_without_section() {
        let src = ".. _standalone:\n\nJust a paragraph, not a heading.\n";
        let labels = scan_labels(src);
        assert_eq!(labels, vec![("standalone".to_string(), None)]);
    }

    #[test]
    fn scan_glossary_terms_basic() {
        let src = "\
.. glossary::

   term one
   term two
      Shared definition.

   another term
      Its own definition.
";
        let terms = scan_glossary_terms(src);
        assert_eq!(
            terms,
            vec![
                "term one".to_string(),
                "term two".to_string(),
                "another term".to_string()
            ]
        );
    }

    #[test]
    fn scan_glossary_skips_option_lines() {
        let src = "\
.. glossary::
   :sorted:

   widget
      A thing.
";
        let terms = scan_glossary_terms(src);
        assert_eq!(terms, vec!["widget".to_string()]);
    }

    #[test]
    fn scan_rst_domain_objects_basic() {
        let src = ".. rst:directive:: toctree\n\n   Body.\n\n.. rst:role:: ref\n\n   Body.\n";
        let objs = scan_rst_domain_objects(src);
        assert_eq!(
            objs,
            vec![
                ("directive".to_string(), "toctree".to_string()),
                ("role".to_string(), "ref".to_string())
            ]
        );
    }

    #[test]
    fn scan_xref_roles_plain_target() {
        let src = "See :ref:`my-label` for details.\n";
        let xrefs = scan_xref_roles(src);
        assert_eq!(xrefs.len(), 1);
        assert_eq!(xrefs[0].domain, "std");
        assert_eq!(xrefs[0].reftype, "ref");
        assert_eq!(xrefs[0].target, "my-label");
        assert_eq!(xrefs[0].explicit_title, None);
    }

    #[test]
    fn scan_xref_roles_phrase_form() {
        let src = "See :doc:`the setup guide <guide/setup>` first.\n";
        let xrefs = scan_xref_roles(src);
        assert_eq!(xrefs.len(), 1);
        assert_eq!(xrefs[0].reftype, "doc");
        assert_eq!(xrefs[0].target, "guide/setup");
        assert_eq!(xrefs[0].explicit_title.as_deref(), Some("the setup guide"));
    }

    #[test]
    fn scan_xref_roles_domain_prefixed() {
        let src = "Use :rst:dir:`toctree` to build a tree.\n";
        let xrefs = scan_xref_roles(src);
        assert_eq!(xrefs.len(), 1);
        assert_eq!(xrefs[0].domain, "rst");
        assert_eq!(xrefs[0].reftype, "dir");
        assert_eq!(xrefs[0].target, "toctree");
    }

    #[test]
    fn scan_xref_roles_ignores_unknown_roles() {
        let src = "This is :emphasis:`not a xref` and :option:`also skipped`.\n";
        assert!(scan_xref_roles(src).is_empty());
    }

    #[test]
    fn scan_xref_roles_multiple_on_one_line() {
        let src = "See :ref:`a` and :term:`b` together.\n";
        let xrefs = scan_xref_roles(src);
        assert_eq!(xrefs.len(), 2);
        assert_eq!(xrefs[0].target, "a");
        assert_eq!(xrefs[1].target, "b");
    }

    #[test]
    fn scan_xref_roles_py_domain_recognized() {
        let src = "See :py:func:`pkg.mod.func` for details.\n";
        let xrefs = scan_xref_roles(src);
        assert_eq!(xrefs.len(), 1);
        assert_eq!(xrefs[0].domain, "py");
        assert_eq!(xrefs[0].reftype, "func");
        assert_eq!(xrefs[0].target, "pkg.mod.func");
        assert!(!xrefs[0].shorten);
    }

    #[test]
    fn scan_xref_roles_tilde_shortens() {
        let src = "See :py:meth:`~pkg.mod.Class.method` for details.\n";
        let xrefs = scan_xref_roles(src);
        assert_eq!(xrefs.len(), 1);
        assert_eq!(xrefs[0].target, "pkg.mod.Class.method");
        assert!(xrefs[0].shorten);
    }

    #[test]
    fn scan_index_entries_untyped_comma_separated() {
        let src = ".. index:: foo, bar\n";
        let entries = scan_index_entries(src);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].text, "foo");
        assert_eq!(entries[1].text, "bar");
        assert!(matches!(entries[0].kind, IndexEntryKind::Single));
    }

    #[test]
    fn scan_index_entries_single_with_sub() {
        let src = ".. index:: single: foo; bar\n";
        let entries = scan_index_entries(src);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].text, "foo");
        assert_eq!(entries[1].text, "foo; bar");
    }

    #[test]
    fn scan_index_entries_pair_and_triple() {
        let src = ".. index::\n   pair: foo; bar\n   triple: a; b; c\n";
        let entries = scan_index_entries(src);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].text, "foo; bar");
        assert!(matches!(entries[0].kind, IndexEntryKind::Pair));
        assert_eq!(entries[1].text, "a; b; c");
        assert!(matches!(entries[1].kind, IndexEntryKind::Triple));
    }

    #[test]
    fn scan_index_entries_see_and_seealso() {
        let src = ".. index::\n   see: widget; gadget\n   seealso: gizmo; gadget\n";
        let entries = scan_index_entries(src);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].text, "widget");
        assert_eq!(entries[0].see_target.as_deref(), Some("gadget"));
        assert!(matches!(entries[0].kind, IndexEntryKind::See));
        assert!(matches!(entries[1].kind, IndexEntryKind::SeeAlso));
    }

    #[test]
    fn scan_index_entries_main_prefix() {
        let src = ".. index:: ! important\n";
        let entries = scan_index_entries(src);
        assert_eq!(entries.len(), 1);
        assert!(entries[0].main);
        assert_eq!(entries[0].text, "important");
    }

    #[test]
    fn scan_domain_objects_module_and_function() {
        use super::super::py_domain::py_signature_of;

        let src = "\
.. py:module:: pkg.mod\n\n.. py:function:: greet(name, loud=False)\n\n   Docs.\n";
        let objs = scan_domain_objects(
            src,
            "py",
            &["module", "currentmodule"],
            &["currentclass"],
            &["class", "exception"],
            &[
                "function",
                "method",
                "classmethod",
                "staticmethod",
                "attribute",
                "data",
            ],
            py_signature_of,
        );
        assert_eq!(objs.len(), 2);
        assert_eq!(objs[0].objtype, "module");
        assert_eq!(objs[0].fullname, "pkg.mod");
        assert_eq!(objs[1].objtype, "function");
        assert_eq!(objs[1].fullname, "pkg.mod.greet");
        assert_eq!(objs[1].signature, "name, loud=False");
    }

    #[test]
    fn scan_domain_objects_nested_class_method() {
        use super::super::py_domain::py_signature_of;

        let src = "\
.. py:module:: pkg\n\n.. py:class:: Widget\n\n   .. py:method:: render(self)\n\n      Docs.\n\n.. py:function:: top_level()\n";
        let objs = scan_domain_objects(
            src,
            "py",
            &["module", "currentmodule"],
            &["currentclass"],
            &["class", "exception"],
            &[
                "function",
                "method",
                "classmethod",
                "staticmethod",
                "attribute",
                "data",
            ],
            py_signature_of,
        );
        assert_eq!(objs.len(), 4);
        assert_eq!(objs[0].fullname, "pkg");
        assert_eq!(objs[1].fullname, "pkg.Widget");
        assert_eq!(objs[2].fullname, "pkg.Widget.render");
        // Dedenting back to top level after the class body drops the
        // class prefix for the next sibling.
        assert_eq!(objs[3].fullname, "pkg.top_level");
    }
}
