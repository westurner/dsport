//! Shared domain implementation for Rust and Lean source declarations.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::environment::BuildEnvironment;
use crate::source_docs::{
    DeclarationKind, SourceDeclaration, SourceLanguage, SourceSpan, separator_for,
};

use super::{Domain, ObjectEntry, XrefTarget, normalize_id};

/// Search metadata retained for a source declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceObjectEntry {
    pub domain: String,
    pub language: SourceLanguage,
    pub kind: DeclarationKind,
    pub name: String,
    pub canonical_name: String,
    pub aliases: Vec<String>,
    pub docname: String,
    pub anchor: String,
    pub documentation: String,
    pub signature: Option<String>,
    pub source: SourceSpan,
    pub deprecated: bool,
    pub noindex: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SourceRecord {
    docname: String,
    declaration: SourceDeclaration,
}

/// A language-aware declaration table shared by the Rust and Lean domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceDomain {
    pub language: SourceLanguage,
    records: Vec<SourceRecord>,
    /// Canonical or alias lookup spelling -> record indices.
    aliases: BTreeMap<String, Vec<usize>>,
}

impl SourceDomain {
    pub fn new(language: SourceLanguage) -> Self {
        Self {
            language,
            records: Vec::new(),
            aliases: BTreeMap::new(),
        }
    }

    pub fn note_snapshot(&mut self, docname: &str, declarations: &[SourceDeclaration]) {
        self.clear_doc(docname);
        for declaration in declarations {
            if declaration.language != self.language {
                continue;
            }
            self.records.push(SourceRecord {
                docname: docname.to_string(),
                declaration: declaration.clone(),
            });
        }
        self.rebuild_aliases();
    }

    pub fn clear_doc(&mut self, docname: &str) {
        self.records.retain(|record| record.docname != docname);
        self.rebuild_aliases();
    }

    pub fn declarations(&self) -> impl Iterator<Item = &SourceDeclaration> {
        self.records.iter().map(|record| &record.declaration)
    }

    pub fn source_objects(&self, domain: &str) -> Vec<SourceObjectEntry> {
        let mut objects = Vec::new();
        for record in &self.records {
            let declaration = &record.declaration;
            if declaration.noindex {
                continue;
            }
            let anchor = anchor_for(domain, declaration);
            objects.push(SourceObjectEntry {
                domain: domain.to_string(),
                language: declaration.language,
                kind: declaration.kind.clone(),
                name: declaration.qualified_name.clone(),
                canonical_name: declaration.qualified_name.clone(),
                aliases: declaration.aliases.clone(),
                docname: record.docname.clone(),
                anchor,
                documentation: declaration.documentation.clone(),
                signature: declaration.signature.clone(),
                source: declaration.source.clone(),
                deprecated: declaration.deprecated,
                noindex: declaration.noindex,
            });
        }
        objects.sort_by(|left, right| {
            left.name
                .cmp(&right.name)
                .then(left.kind.cmp(&right.kind))
                .then(left.docname.cmp(&right.docname))
        });
        objects
    }

    /// Return the actual registered reference-role spellings that accept a
    /// declaration kind. Used by status tooling to audit xref coverage.
    pub fn reference_roles_for_kind(&self, kind: &DeclarationKind) -> Vec<&'static str> {
        let candidates: &[&str] = match self.language {
            SourceLanguage::Rust => &[
                "associatedconst",
                "associatedtype",
                "const",
                "enum",
                "field",
                "func",
                "impl",
                "macro",
                "meth",
                "mod",
                "static",
                "struct",
                "trait",
                "type",
                "union",
                "variant",
            ],
            SourceLanguage::Lean => &[
                "abbrev",
                "axiom",
                "class",
                "constructor",
                "def",
                "field",
                "inductive",
                "instance",
                "lemma",
                "mod",
                "notation",
                "ns",
                "opaque",
                "structure",
                "theorem",
                "thm",
            ],
            SourceLanguage::Python => &[],
        };
        candidates
            .iter()
            .copied()
            .filter(|role| self.accepts(role, kind))
            .collect()
    }

    fn rebuild_aliases(&mut self) {
        self.aliases.clear();
        for (index, record) in self.records.iter().enumerate() {
            self.aliases
                .entry(record.declaration.qualified_name.clone())
                .or_default()
                .push(index);
            for alias in &record.declaration.aliases {
                self.aliases.entry(alias.clone()).or_default().push(index);
            }
        }
    }

    fn accepts(&self, reftype: &str, kind: &DeclarationKind) -> bool {
        let accepted: &[DeclarationKind] = match self.language {
            SourceLanguage::Rust => match reftype {
                "associatedconst" => &[DeclarationKind::AssociatedConstant],
                "associatedtype" => &[DeclarationKind::AssociatedType],
                "const" => &[DeclarationKind::Constant, DeclarationKind::AssociatedConstant],
                "enum" => &[DeclarationKind::Enum],
                "field" => &[DeclarationKind::Field],
                "func" => &[DeclarationKind::Function],
                "impl" => &[DeclarationKind::Impl],
                "macro" => &[DeclarationKind::Macro],
                "meth" => &[DeclarationKind::Method],
                "mod" => &[DeclarationKind::Module],
                "obj" => &[],
                "static" => &[DeclarationKind::Static],
                "struct" => &[DeclarationKind::Struct],
                "trait" => &[DeclarationKind::Trait],
                "type" => &[DeclarationKind::TypeAlias, DeclarationKind::AssociatedType],
                "union" => &[DeclarationKind::Union],
                "variant" => &[DeclarationKind::Variant],
                _ => return false,
            },
            SourceLanguage::Lean => match reftype {
                "abbrev" => &[DeclarationKind::Abbrev],
                "axiom" => &[DeclarationKind::Axiom],
                "class" => &[DeclarationKind::Class],
                "constructor" => &[DeclarationKind::Variant],
                "def" => &[DeclarationKind::Definition],
                "field" => &[DeclarationKind::Field],
                "inductive" => &[DeclarationKind::Inductive],
                "instance" => &[DeclarationKind::Instance],
                "lemma" => &[DeclarationKind::Lemma],
                "mod" => &[DeclarationKind::Module],
                "notation" => &[DeclarationKind::Notation],
                "ns" => &[DeclarationKind::Namespace],
                "obj" => &[],
                "opaque" => &[DeclarationKind::Opaque],
                "structure" => &[DeclarationKind::Structure],
                "theorem" | "thm" => &[DeclarationKind::Theorem],
                _ => return false,
            },
            SourceLanguage::Python => return false,
        };
        if reftype == "obj" {
            return true;
        }
        accepted.iter().any(|candidate| candidate == kind)
    }

    fn resolve_name(&self, env: &BuildEnvironment, target: &str) -> Vec<String> {
        let separator = separator_for(self.language);
        let target = target.trim();
        if target.is_empty() {
            return Vec::new();
        }
        let context_key = match self.language {
            SourceLanguage::Lean => "lean:namespace",
            SourceLanguage::Python => "",
            SourceLanguage::Rust => "rust:module",
        };
        let context = env.ref_context.get(context_key).map(String::as_str);
        let mut names = Vec::new();
        let normalized = target.strip_prefix(separator).unwrap_or(target);
        let self_prefix = format!("self{separator}");
        let super_prefix = format!("super{separator}");
        if let Some(rest) = normalized.strip_prefix(&self_prefix) {
            if let Some(context) = context {
                names.push(format!("{context}{separator}{rest}"));
            }
        } else if let Some(rest) = normalized.strip_prefix(&super_prefix) {
            if let Some(context) = context
                && let Some(parent) = context.rsplit_once(separator).map(|(parent, _)| parent)
            {
                names.push(format!("{parent}{separator}{rest}"));
            }
        } else {
            if let Some(context) = context && !normalized.contains(separator) {
                names.push(format!("{context}{separator}{normalized}"));
            }
            names.push(normalized.to_string());
        }
        names.sort();
        names.dedup();
        names
    }

    fn lookup(
        &self,
        env: &BuildEnvironment,
        reftype: &str,
        target: &str,
    ) -> Option<&SourceRecord> {
        let mut matches = Vec::new();
        for name in self.resolve_name(env, target) {
            if let Some(indices) = self.aliases.get(&name) {
                for index in indices {
                    let record = &self.records[*index];
                    if self.accepts(reftype, &record.declaration.kind) {
                        matches.push(*index);
                    }
                }
            }
        }
        if matches.is_empty() && !target.contains(separator_for(self.language)) {
            for (index, record) in self.records.iter().enumerate() {
                if self.accepts(reftype, &record.declaration.kind)
                    && record.declaration.short_name == target
                {
                    matches.push(index);
                }
            }
        }
        matches.sort_unstable();
        matches.dedup();
        (matches.len() == 1).then(|| &self.records[matches[0]])
    }
}

fn anchor_for(domain: &str, declaration: &SourceDeclaration) -> String {
    format!(
        "{domain}-{}-{}",
        declaration.kind.as_str(),
        normalize_id(&declaration.qualified_name)
    )
}

impl Domain for SourceDomain {
    fn name(&self) -> &'static str {
        match self.language {
            SourceLanguage::Lean => "lean",
            SourceLanguage::Python => "python",
            SourceLanguage::Rust => "rust",
        }
    }

    fn resolve_xref(
        &self,
        env: &BuildEnvironment,
        _fromdocname: &str,
        reftype: &str,
        target: &str,
    ) -> Option<XrefTarget> {
        let record = self.lookup(env, reftype, target)?;
        let declaration = &record.declaration;
        Some(XrefTarget {
            docname: record.docname.clone(),
            anchor: anchor_for(self.name(), declaration),
            title: declaration.qualified_name.clone(),
        })
    }

    fn get_objects(&self) -> Vec<ObjectEntry> {
        self.source_objects(self.name())
            .into_iter()
            .map(|entry| ObjectEntry {
                obj_type: entry.kind.as_str().to_string(),
                name: entry.name,
                docname: entry.docname,
                anchor: entry.anchor,
            })
            .collect()
    }

    fn clear_doc(&mut self, docname: &str) {
        SourceDomain::clear_doc(self, docname);
    }
}

/// Rust source declarations registered by the build environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustDomain(pub SourceDomain);

impl Default for RustDomain {
    fn default() -> Self {
        Self(SourceDomain::new(SourceLanguage::Rust))
    }
}

impl RustDomain {
    pub fn new() -> Self { Self::default() }
    pub fn note_snapshot(&mut self, docname: &str, declarations: &[SourceDeclaration]) {
        self.0.note_snapshot(docname, declarations);
    }
    pub fn source_objects(&self) -> Vec<SourceObjectEntry> {
        self.0.source_objects("rust")
    }
}

impl Domain for RustDomain {
    fn name(&self) -> &'static str { "rust" }
    fn resolve_xref(&self, env: &BuildEnvironment, fromdocname: &str, reftype: &str, target: &str) -> Option<XrefTarget> {
        self.0.resolve_xref(env, fromdocname, reftype, target)
    }
    fn get_objects(&self) -> Vec<ObjectEntry> { self.0.get_objects() }
    fn clear_doc(&mut self, docname: &str) { self.0.clear_doc(docname); }
}

/// Lean source declarations registered by the build environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanDomain(pub SourceDomain);

impl Default for LeanDomain {
    fn default() -> Self {
        Self(SourceDomain::new(SourceLanguage::Lean))
    }
}

impl LeanDomain {
    pub fn new() -> Self { Self::default() }
    pub fn note_snapshot(&mut self, docname: &str, declarations: &[SourceDeclaration]) {
        self.0.note_snapshot(docname, declarations);
    }
    pub fn source_objects(&self) -> Vec<SourceObjectEntry> {
        self.0.source_objects("lean")
    }
}

impl Domain for LeanDomain {
    fn name(&self) -> &'static str { "lean" }
    fn resolve_xref(&self, env: &BuildEnvironment, fromdocname: &str, reftype: &str, target: &str) -> Option<XrefTarget> {
        self.0.resolve_xref(env, fromdocname, reftype, target)
    }
    fn get_objects(&self) -> Vec<ObjectEntry> { self.0.get_objects() }
    fn clear_doc(&mut self, docname: &str) { self.0.clear_doc(docname); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SphinxConfig;
    use crate::environment::{BuildEnvironment, EnvProject};
    use crate::source_analysis::{SourcePosition, Visibility};

    fn env() -> BuildEnvironment {
        BuildEnvironment::new(
            SphinxConfig::new_defaults(),
            EnvProject::new("/tmp/src", &[(".rst", "restructuredtext")]),
            "/tmp/src",
            "/tmp/doctrees",
        )
    }

    fn declaration(language: SourceLanguage, name: &str, kind: DeclarationKind) -> SourceDeclaration {
        SourceDeclaration::new(
            language,
            "test",
            name,
            separator_for(language),
            kind,
            Some(format!("{name}()")),
            None,
            "documentation",
            Visibility::Public,
            SourceSpan::new(
                "src/lib.rs",
                SourcePosition { byte: 0, line: 1, column: 0 },
                None,
            ),
        )
    }

    #[test]
    fn canonical_and_alias_lookup_use_language_namespaces() {
        let mut domain = RustDomain::new();
        let mut item = declaration(SourceLanguage::Rust, "crate::api::Thing", DeclarationKind::Struct);
        item.add_alias("crate::api::ThingAlias");
        domain.note_snapshot("api", &[item]);
        let environment = env();

        let canonical = domain.resolve_xref(&environment, "index", "struct", "crate::api::Thing");
        let alias = domain.resolve_xref(&environment, "index", "struct", "crate::api::ThingAlias");
        assert_eq!(canonical.as_ref().map(|target| target.docname.as_str()), Some("api"));
        assert_eq!(alias.as_ref().map(|target| target.title.as_str()), Some("crate::api::Thing"));
        assert!(canonical.unwrap().anchor.contains("rust-struct-crate-api-thing"));
    }

    #[test]
    fn short_name_is_rejected_when_two_namespaces_match() {
        let mut domain = LeanDomain::new();
        let first = declaration(SourceLanguage::Lean, "Alpha.Widget", DeclarationKind::Structure);
        let second = declaration(SourceLanguage::Lean, "Beta.Widget", DeclarationKind::Structure);
        domain.note_snapshot("alpha", &[first]);
        domain.note_snapshot("beta", &[second]);
        assert!(domain.resolve_xref(&env(), "index", "structure", "Widget").is_none());
        assert!(domain.resolve_xref(&env(), "index", "structure", "Alpha.Widget").is_some());
    }

    #[test]
    fn context_resolution_uses_rust_and_lean_separators() {
        let mut rust = RustDomain::new();
        rust.note_snapshot(
            "rust",
            &[declaration(SourceLanguage::Rust, "crate::api::Thing", DeclarationKind::Struct)],
        );
        let mut rust_env = env();
        rust_env.ref_context.insert("rust:module".into(), "crate::api".into());
        assert!(rust.resolve_xref(&rust_env, "index", "struct", "self::Thing").is_some());

        let mut lean = LeanDomain::new();
        lean.note_snapshot(
            "lean",
            &[declaration(SourceLanguage::Lean, "Demo.Widget", DeclarationKind::Structure)],
        );
        let mut lean_env = env();
        lean_env.ref_context.insert("lean:namespace".into(), "Demo".into());
        assert!(lean.resolve_xref(&lean_env, "index", "structure", "self.Widget").is_some());
    }

    #[test]
    fn lean_theorem_role_accepts_long_and_short_spellings() {
        let mut lean = LeanDomain::new();
        lean.note_snapshot(
            "lean",
            &[declaration(
                SourceLanguage::Lean,
                "Demo.answer",
                DeclarationKind::Theorem,
            )],
        );
        let environment = env();
        assert!(lean
            .resolve_xref(&environment, "index", "thm", "Demo.answer")
            .is_some());
        assert!(lean
            .resolve_xref(&environment, "index", "theorem", "Demo.answer")
            .is_some());
    }

    #[test]
    fn clear_doc_rebuilds_aliases_and_source_objects_skip_noindex() {
        let mut domain = RustDomain::new();
        let mut hidden = declaration(SourceLanguage::Rust, "crate::Hidden", DeclarationKind::Struct);
        hidden.noindex = true;
        let visible = declaration(SourceLanguage::Rust, "crate::Visible", DeclarationKind::Struct);
        domain.note_snapshot("first", &[hidden, visible]);
        assert_eq!(domain.source_objects().len(), 1);
        domain.clear_doc("first");
        assert!(domain.get_objects().is_empty());
    }
}
