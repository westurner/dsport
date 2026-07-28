//! `sphinxdocrs::domains::scan` — text-level RST scanners feeding the
//! **H3a**/**H3c**/**H5b** domain data and xref recovery.
//!
//! See the accepted-deviation note on `domains` (the module doc comment)
//! for why these operate on raw source text rather than a structural
//! doctree: `docutilsrs`'s parser doesn't yet model Sphinx-specific
//! directives (`glossary`, `rst:directive`, ...) or emit a `pending_xref`
//! node for cross-reference roles. These scanners follow the same
//! indentation-based heuristics as `environment::scan_toctree_entries`.

use super::PendingXref;

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
/// (`ref`, `doc`, `term`, `numref`, `keyword`, and any `rst:*` role);
/// other domain-prefixed or unprefixed roles (`:py:func:`, `:option:`,
/// custom roles, ...) are not yet collected — extend the `KNOWN_REFTYPES`
/// list here as more domains land.
pub fn scan_xref_roles(source: &str) -> Vec<PendingXref> {
    const KNOWN_STD_REFTYPES: &[&str] = &["ref", "doc", "term", "numref", "keyword"];
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
                let recognized = (domain == "std"
                    && KNOWN_STD_REFTYPES.contains(&reftype.as_str()))
                    || domain == "rst";
                if recognized {
                    let (target, explicit_title) = split_phrase(&content);
                    out.push(PendingXref {
                        domain,
                        reftype,
                        target,
                        explicit_title,
                        line: line_idx + 1,
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
fn split_phrase(content: &str) -> (String, Option<String>) {
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
        let src = "This is :emphasis:`not a xref` and :py:func:`also skipped`.\n";
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
}
