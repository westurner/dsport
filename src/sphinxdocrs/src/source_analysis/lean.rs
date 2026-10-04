//! Lean source analysis through the pinned Arborium Lean grammar.
//!
//! Arborium/tree-sitter owns syntax structure and byte ranges here. The
//! lowering code intentionally keeps parser node types out of the public
//! declaration model so grammar upgrades can be versioned and snapshot-tested.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use arborium::tree_sitter::{Language, Node, Parser, Tree};

use super::{
    AnalysisDiagnostic, AnalysisError, AnalysisSnapshot, DeclarationKind, DiagnosticSeverity,
    SourceAnalysisRequest, SourceAnalyzer, SourceDeclaration, SourceLanguage, SourcePosition,
    SourceSpan, Visibility, normalize_text,
};

pub const BACKEND: &str = "arborium-lean";
pub const BACKEND_VERSION: &str = "2.18.2";

/// Syntax analyzer for one or more Lean source files.
#[derive(Debug, Clone, Default)]
pub struct ArboriumLeanAnalyzer;

impl SourceAnalyzer for ArboriumLeanAnalyzer {
    fn analyze(
        &self,
        request: &SourceAnalysisRequest,
    ) -> Result<AnalysisSnapshot, AnalysisError> {
        let files = source_files(request)?;
        if files.is_empty() {
            return Err(AnalysisError::BackendUnavailable {
                backend: BACKEND.to_string(),
                message: format!("no .lean source files found under {}", request.source_root.display()),
            });
        }

        let mut declarations = Vec::new();
        let mut diagnostics = Vec::new();
        let mut hash_input = Vec::new();
        for path in files {
            let bytes = fs::read(&path).map_err(|error| AnalysisError::Io {
                path: path.display().to_string(),
                message: error.to_string(),
            })?;
            hash_input.extend_from_slice(&bytes);
            let source = String::from_utf8_lossy(&bytes).replace("\r\n", "\n").replace('\r', "\n");
            let mut parser = Parser::new();
            let language = Language::new(arborium_lean::language());
            parser.set_language(&language).map_err(|error| AnalysisError::BackendUnavailable {
                backend: BACKEND.to_string(),
                message: error.to_string(),
            })?;
            let tree = parser.parse(source.as_bytes(), None).ok_or_else(|| AnalysisError::Parse {
                backend: BACKEND.to_string(),
                message: format!("{}: parser returned no tree", path.display()),
            })?;
            if tree.root_node().has_error() {
                diagnostics.push(AnalysisDiagnostic {
                    severity: DiagnosticSeverity::Warning,
                    backend: BACKEND.to_string(),
                    message: "Lean source contains recoverable syntax errors".to_string(),
                    source: Some(file_span(&path, &source)),
                    declaration: None,
                });
            }
            let module_name = module_name(&request.source_root, &path);
            let mut walker = Walker {
                source: &source,
                path: &path,
                declarations: &mut declarations,
                seen: BTreeSet::new(),
                include_private: request.include_private,
            };
            walker.visit(tree, module_name.as_deref());
        }

        let mut snapshot = AnalysisSnapshot::new(
            SourceLanguage::Lean,
            BACKEND,
            BACKEND_VERSION,
            &request.source_root,
            format_hash(&hash_input),
            declarations,
            diagnostics,
        );
        snapshot.set_request_identity(request);
        Ok(snapshot)
    }

    fn cache_identity(&self, request: &SourceAnalysisRequest) -> String {
        format!(
            "lean-source-v1:{BACKEND_VERSION}:{}",
            request.cache_identity()
        )
    }
}

struct Walker<'a> {
    source: &'a str,
    path: &'a Path,
    declarations: &'a mut Vec<SourceDeclaration>,
    seen: BTreeSet<(usize, DeclarationKind, String)>,
    include_private: bool,
}

impl<'a> Walker<'a> {
    fn visit(&mut self, tree: Tree, module_name: Option<&str>) {
        self.visit_node(tree.root_node(), module_name, None);
    }

    fn visit_node(&mut self, node: Node<'_>, namespace: Option<&str>, section: Option<&str>) {
        let mut next_namespace = namespace.map(str::to_string);
        let mut next_section = section.map(str::to_string);
        if let Some(keyword) = header_keyword(self.source_slice(node)) {
            let name = declaration_name(self.source_slice(node), keyword);
            let kind = kind_for_keyword(keyword);
            let name = name.or_else(|| (keyword == "module").then(|| "Main".to_string()));
            if let Some(name) = name {
                let full_name = if keyword == "namespace"
                    && section.is_none()
                    && namespace.is_some_and(|current| current.rsplit('.').next() == Some(name.as_str()))
                {
                    namespace.unwrap_or_default().to_string()
                } else {
                    qualified_name(namespace, section, &name)
                };
                let is_private = name.starts_with('_') || keyword == "private";
                if self.include_private || !is_private {
                    let key = (node.start_byte(), kind.clone(), full_name.clone());
                    if self.seen.insert(key) {
                        let span = node_span(self.path, node);
                        let mut declaration = SourceDeclaration::new(
                            SourceLanguage::Lean,
                            BACKEND,
                            &full_name,
                            ".",
                            kind.clone(),
                            Some(normalize_text(self.source_slice(node))),
                            Some(normalize_text(self.source_slice(node))),
                            documentation_before(self.source, node.start_byte()),
                            if is_private { Visibility::Private } else { Visibility::Public },
                            span,
                        );
                        if let Some(attributes) = attributes_before(self.source, node.start_byte()) {
                            declaration.attributes.insert("attributes".to_string(), attributes);
                        }
                        self.declarations.push(declaration);
                    }
                }
                if keyword == "namespace" {
                    next_namespace = Some(full_name);
                } else if keyword == "section" {
                    next_section = Some(full_name);
                }
            }
        }

        for index in 0..node.child_count() {
            if let Some(child) = node.child(index as u32) {
                self.visit_node(child, next_namespace.as_deref(), next_section.as_deref());
            }
        }
    }

    fn source_slice(&self, node: Node<'_>) -> &str {
        self.source
            .get(node.byte_range())
            .unwrap_or_default()
    }
}

fn source_files(request: &SourceAnalysisRequest) -> Result<Vec<PathBuf>, AnalysisError> {
    let mut files = if !request.selected.is_empty() {
        request.selected.clone()
    } else if request.source_root.is_file() {
        vec![request.source_root.clone()]
    } else {
        let mut files = Vec::new();
        collect_lean_files(&request.source_root, &mut files).map_err(|error| AnalysisError::Io {
            path: request.source_root.display().to_string(),
            message: error.to_string(),
        })?;
        files
    };
    files.retain(|path| path.extension().is_some_and(|extension| extension == "lean"));
    files.sort();
    files.dedup();
    Ok(files)
}

fn collect_lean_files(root: &Path, output: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_lean_files(&path, output)?;
        } else if path.extension().is_some_and(|extension| extension == "lean") {
            output.push(path);
        }
    }
    Ok(())
}

fn module_name(root: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(root).ok()?;
    let mut parts = relative
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .map(str::to_string)
        .collect::<Vec<_>>();
    let last = parts.last_mut()?;
    *last = last.strip_suffix(".lean").unwrap_or(last).to_string();
    Some(parts.join("."))
}

fn header_keyword(source: &str) -> Option<&'static str> {
    let mut line = source.lines().find(|line| {
        let line = line.trim();
        !line.is_empty() && !line.starts_with("/-") && !line.starts_with("--") && !line.starts_with("@[")
    })?.trim_start();
    for modifier in [
        "private ", "protected ", "scoped ", "unsafe ", "noncomputable ", "partial ", "mutual ",
    ] {
        if let Some(rest) = line.strip_prefix(modifier) {
            line = rest.trim_start();
        }
    }
    [
        "namespace", "section", "def", "theorem", "lemma", "example", "inductive", "structure",
        "class", "instance", "abbrev", "axiom", "opaque", "notation", "module",
    ]
    .into_iter()
    .find(|keyword| line == *keyword || line.starts_with(&format!("{keyword} ")) || line.starts_with(&format!("{keyword}\t")))
}

fn declaration_name(source: &str, keyword: &str) -> Option<String> {
    let line = source.lines().find(|line| line.trim_start().starts_with(keyword))?.trim();
    let rest = line.strip_prefix(keyword)?.trim_start();
    if rest.is_empty() || rest.starts_with(':') || rest.starts_with("=>") {
        return None;
    }
    let name = rest
        .split(|character: char| character.is_whitespace() || matches!(character, ':' | '(' | '{' | '[' | '=' | ','))
        .find(|part| !part.is_empty())?;
    Some(name.trim_matches('`').to_string())
}

fn kind_for_keyword(keyword: &str) -> DeclarationKind {
    match keyword {
        "abbrev" => DeclarationKind::Abbrev,
        "axiom" => DeclarationKind::Axiom,
        "class" => DeclarationKind::Class,
        "def" => DeclarationKind::Definition,
        "example" => DeclarationKind::Example,
        "inductive" => DeclarationKind::Inductive,
        "instance" => DeclarationKind::Instance,
        "lemma" => DeclarationKind::Lemma,
        "module" => DeclarationKind::Module,
        "namespace" => DeclarationKind::Namespace,
        "notation" => DeclarationKind::Notation,
        "opaque" => DeclarationKind::Opaque,
        "section" => DeclarationKind::Namespace,
        "structure" => DeclarationKind::Structure,
        "theorem" => DeclarationKind::Theorem,
        _ => DeclarationKind::Other(keyword.to_string()),
    }
}

fn qualified_name(namespace: Option<&str>, section: Option<&str>, name: &str) -> String {
    let context = section.or(namespace).unwrap_or_default();
    if name.contains('.') || context.is_empty() {
        name.to_string()
    } else {
        format!("{context}.{name}")
    }
}

fn documentation_before(source: &str, byte: usize) -> String {
    let prefix = source.get(..byte).unwrap_or_default();
    let mut comments = Vec::new();
    for line in prefix.lines().rev() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("///") {
            comments.push(value.trim_start().to_string());
        } else if trimmed.starts_with("/--") || trimmed.starts_with("/-!") {
            let value = trimmed
                .trim_start_matches("/--")
                .trim_start_matches("/-!")
                .trim_end_matches("-/")
                .trim();
            comments.push(value.to_string());
            break;
        } else if trimmed.is_empty() && !comments.is_empty() {
            continue;
        } else {
            break;
        }
    }
    comments.reverse();
    normalize_text(&comments.join("\n"))
}

fn attributes_before(source: &str, byte: usize) -> Option<String> {
    let prefix = source.get(..byte)?.lines().next_back()?.trim();
    prefix.starts_with("@[").then(|| prefix.to_string())
}

fn node_span(path: &Path, node: Node<'_>) -> SourceSpan {
    let start = node.start_position();
    let end = node.end_position();
    SourceSpan::new(
        path,
        SourcePosition {
            byte: node.start_byte(),
            line: start.row + 1,
            column: start.column,
        },
        Some(SourcePosition {
            byte: node.end_byte(),
            line: end.row + 1,
            column: end.column,
        }),
    )
}

fn file_span(path: &Path, source: &str) -> SourceSpan {
    let end = source.len();
    let line = source.lines().count().max(1);
    let column = source.rsplit('\n').next().map_or(0, str::len);
    SourceSpan::new(
        path,
        SourcePosition { byte: 0, line: 1, column: 0 },
        Some(SourcePosition { byte: end, line, column }),
    )
}

fn format_hash(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_analysis::normalize_path;
    use tempfile::tempdir;

    #[test]
    fn discovers_lean_files_and_normalizes_module_names() {
        let directory = tempdir().unwrap();
        let nested = directory.path().join("Demo");
        fs::create_dir(&nested).unwrap();
        fs::write(nested.join("Basic.lean"), "def answer : Nat := 42\n").unwrap();
        let request = SourceAnalysisRequest::new(directory.path());
        let snapshot = ArboriumLeanAnalyzer.analyze(&request).unwrap();
        assert_eq!(snapshot.source_root, normalize_path(directory.path()));
        assert!(snapshot.declarations.iter().any(|declaration| declaration.qualified_name == "Demo.Basic.answer"));
    }

    #[test]
    fn cache_identity_includes_lean_grammar_version_and_request_options() {
        let request = SourceAnalysisRequest::new("fixture");
        let analyzer = ArboriumLeanAnalyzer;
        let identity = analyzer.cache_identity(&request);
        let mut include_private = request.clone();
        include_private.include_private = true;

        assert!(identity.starts_with(&format!("lean-source-v1:{BACKEND_VERSION}:")));
        assert_ne!(identity, analyzer.cache_identity(&include_private));
        assert_ne!(identity, request.cache_identity());
    }

    #[test]
    fn doc_comments_attach_to_the_following_declaration() {
        let source = "/-- theorem docs -/\n";
        assert_eq!(documentation_before(source, source.len()), "theorem docs");
        assert_eq!(kind_for_keyword("theorem"), DeclarationKind::Theorem);
    }

    #[test]
    fn fixture_corpus_covers_declarations_comments_and_recoverable_errors() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/lean");
        let snapshot = ArboriumLeanAnalyzer
            .analyze(&SourceAnalysisRequest::new(&root))
            .unwrap();
        let names = snapshot
            .declarations
            .iter()
            .map(|declaration| declaration.qualified_name.as_str())
            .collect::<BTreeSet<_>>();
        for expected in [
            "Demo",
            "Demo.Arithmetic",
            "Demo.Arithmetic.add",
            "Demo.Arithmetic.add_zero",
            "Demo.Arithmetic.Color",
            "Demo.Arithmetic.Point",
            "Demo.Arithmetic.SemigroupLike",
            "Malformed.Broken",
            "Malformed.Broken.incomplete",
        ] {
            assert!(names.contains(expected), "missing declaration {expected}: {names:?}");
        }
        let add = snapshot
            .declarations
            .iter()
            .find(|declaration| declaration.qualified_name == "Demo.Arithmetic.add")
            .unwrap();
        assert!(add.documentation.contains("Add two natural numbers"));
        assert!(add.source.end.as_ref().is_some_and(|end| end.byte > add.source.start.byte));
        assert!(snapshot.diagnostics.iter().any(|diagnostic| {
            diagnostic.message.contains("recoverable syntax errors")
        }));
    }
}