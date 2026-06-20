//! RST scanning for `.. autosummary::` directives.
//!
//! Mirrors `sphinx.ext.autosummary.generate.find_autosummary_in_lines`
//! and `find_autosummary_in_files`.
//!
//! This is the **pure, native** part of the autogen port: parse RST text
//! and extract entries without importing Python objects.

use std::path::Path;

use regex::Regex;

/// Mirrors `AutosummaryEntry` (name, toctree, template, recursive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutosummaryEntry {
    /// Fully-qualified object name (leading `~` stripped).
    pub name: String,
    /// `:toctree:` path (absolute), or `None`.
    pub toctree: Option<String>,
    /// `:template:` value, or empty string.
    pub template: String,
    /// Whether `:recursive:` was set.
    pub recursive: bool,
}

/// Read each file in `filenames` and collect `AutosummaryEntry` items.
///
/// Mirrors `find_autosummary_in_files`.
pub fn find_autosummary_in_files(filenames: &[impl AsRef<Path>]) -> Vec<AutosummaryEntry> {
    let mut all = Vec::new();
    for fname in filenames {
        let path = fname.as_ref();
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let lines: Vec<&str> = text.lines().collect();
        let lines_owned: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        let mut entries = find_autosummary_in_lines(&lines_owned, None, Some(path));
        all.append(&mut entries);
    }
    all
}

/// Parse RST lines and extract `autosummary::` directive entries.
///
/// Mirrors `find_autosummary_in_lines`.
pub fn find_autosummary_in_lines(
    lines: &[String],
    module: Option<&str>,
    filename: Option<&Path>,
) -> Vec<AutosummaryEntry> {
    let autosummary_re = Regex::new(r"^(\s*)\.\.\s+autosummary::\s*$").unwrap();
    let module_re = Regex::new(r"^\s*\.\.\s+(current)?module::\s*([a-zA-Z0-9_.]+)\s*$").unwrap();
    let item_re = Regex::new(r"^\s+(~?[_a-zA-Z][a-zA-Z0-9_.]*)\s*.*?").unwrap();
    let recursive_re = Regex::new(r"^\s+:recursive:\s*$").unwrap();
    let toctree_re = Regex::new(r"^\s+:toctree:\s*(.*?)\s*$").unwrap();
    let template_re = Regex::new(r"^\s+:template:\s*(.*?)\s*$").unwrap();

    let mut result: Vec<AutosummaryEntry> = Vec::new();
    let mut in_autosummary = false;
    let mut base_indent = String::new();
    let mut recursive = false;
    let mut toctree: Option<String> = None;
    let mut template = String::new();
    let mut current_module = module.map(str::to_owned);

    for line in lines {
        if in_autosummary {
            if recursive_re.is_match(line) {
                recursive = true;
                continue;
            }

            if let Some(caps) = toctree_re.captures(line) {
                let raw = caps
                    .get(1)
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_owned();
                toctree = Some(if let Some(fpath) = filename {
                    fpath
                        .parent()
                        .unwrap_or(Path::new("."))
                        .join(&raw)
                        .to_string_lossy()
                        .into_owned()
                } else {
                    raw
                });
                continue;
            }

            if let Some(caps) = template_re.captures(line) {
                template = caps
                    .get(1)
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_owned();
                continue;
            }

            if line.trim_start().starts_with(':') {
                continue; // skip other options
            }

            if let Some(caps) = item_re.captures(line) {
                let raw_name = caps.get(1).map(|m| m.as_str()).unwrap_or("").trim();
                let name = raw_name.trim_start_matches('~');
                let full_name = if let Some(ref m) = current_module {
                    if !name.starts_with(&format!("{m}.")) {
                        format!("{m}.{name}")
                    } else {
                        name.to_owned()
                    }
                } else {
                    name.to_owned()
                };
                result.push(AutosummaryEntry {
                    name: full_name,
                    toctree: toctree.clone(),
                    template: template.clone(),
                    recursive,
                });
                continue;
            }

            // Empty line or deeper indent — stay in block.
            if line.trim().is_empty() || line.starts_with(&format!("{base_indent} ")) {
                continue;
            }

            // Anything else ends the block.
            in_autosummary = false;
        }

        // Detect start of autosummary directive.
        if let Some(caps) = autosummary_re.captures(line) {
            in_autosummary = true;
            base_indent = caps.get(1).map(|m| m.as_str()).unwrap_or("").to_owned();
            recursive = false;
            toctree = None;
            template = String::new();
            continue;
        }

        // Track current module for name resolution.
        if let Some(caps) = module_re.captures(line) {
            current_module = caps.get(2).map(|m| m.as_str().to_owned());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(s: &str) -> Vec<String> {
        s.lines().map(String::from).collect()
    }

    #[test]
    fn basic_autosummary_block() {
        let rst = "
.. autosummary::
   :toctree: generated

   mymodule.MyClass
   mymodule.my_function
";
        let entries = find_autosummary_in_lines(&lines(rst), None, None);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "mymodule.MyClass");
        assert_eq!(entries[1].name, "mymodule.my_function");
        // toctree is set (path without base dir since filename=None)
        assert!(
            entries[0]
                .toctree
                .as_deref()
                .unwrap_or("")
                .contains("generated")
        );
    }

    #[test]
    fn recursive_flag() {
        let rst = "
.. autosummary::
   :toctree: api
   :recursive:

   mymod
";
        let entries = find_autosummary_in_lines(&lines(rst), None, None);
        assert_eq!(entries.len(), 1);
        assert!(entries[0].recursive);
    }

    #[test]
    fn template_option() {
        let rst = "
.. autosummary::
   :template: custom.rst

   mymod.Foo
";
        let entries = find_autosummary_in_lines(&lines(rst), None, None);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].template, "custom.rst");
    }

    #[test]
    fn module_directive_scopes_names() {
        let rst = "
.. currentmodule:: mypackage

.. autosummary::
   Foo
   Bar
";
        let entries = find_autosummary_in_lines(&lines(rst), None, None);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "mypackage.Foo");
        assert_eq!(entries[1].name, "mypackage.Bar");
    }

    #[test]
    fn tilde_prefix_stripped() {
        let rst = "
.. autosummary::

   ~mymod.LongClass
";
        let entries = find_autosummary_in_lines(&lines(rst), None, None);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "mymod.LongClass");
    }

    #[test]
    fn no_entries_returns_empty() {
        let rst = "Some plain text.\n\n.. note::\n   Not an autosummary.\n";
        let entries = find_autosummary_in_lines(&lines(rst), None, None);
        assert!(entries.is_empty());
    }

    #[test]
    fn multiple_blocks() {
        let rst = "
.. autosummary::

   Foo

Some text.

.. autosummary::
   :toctree: api

   Bar
";
        let entries = find_autosummary_in_lines(&lines(rst), None, None);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "Foo");
        assert_eq!(entries[1].name, "Bar");
    }
}
