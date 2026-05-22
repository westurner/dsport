//! Tree-rewriting transforms run after parsing.
//!
//! Mirrors the docutils transforms layer: the parser produces a raw tree,
//! then a pipeline of transforms resolves cross-references and promotes
//! document structure. Each transform is exposed as a `pub fn` so callers
//! can run a custom pipeline or skip stages.
//!
//! Two surfaces are provided:
//!
//! * **Free functions** — [`resolve_references`], [`promote_document_title`],
//!   [`promote_docinfo`], and the orchestration helper
//!   [`run_default_pipeline`]. These match the order used internally by
//!   `parse_rst_with_source` and are the simplest way to run the stock
//!   pipeline against a hand-built tree.
//! * **[`Transform`] trait + [`Pipeline`] builder** — for composing custom
//!   pipelines, inserting third-party passes, or replacing a stock pass
//!   with a parity-checked alternative. Stock transforms are exposed as
//!   the zero-sized [`ResolveReferences`], [`PromoteDocumentTitle`], and
//!   [`PromoteDocinfo`] types so they can be mixed with user-defined
//!   transforms.

use crate::doctree::Doctree;
use crate::parser::{self, ParseCtx};

/// Resolve internal references (named targets, anonymous targets, and
/// footnote / citation cross-links).
pub fn resolve_references(tree: &mut Doctree, ctx: &ParseCtx) {
    parser::resolve_references(tree, ctx);
}

/// Promote a single top-level section into the document title (and an
/// inner second section into the subtitle), matching docutils' DocTitle.
pub fn promote_document_title(tree: &mut Doctree) {
    parser::promote_document_title(tree);
}

/// Promote a leading field list under the title into `<docinfo>`.
pub fn promote_docinfo(tree: &mut Doctree) {
    parser::promote_docinfo(tree);
}

/// Default post-parse pipeline:
/// 1. [`resolve_references`]
/// 2. [`promote_document_title`]
/// 3. [`promote_docinfo`]
pub fn run_default_pipeline(tree: &mut Doctree, ctx: &ParseCtx) {
    resolve_references(tree, ctx);
    promote_document_title(tree);
    promote_docinfo(tree);
}

/// A composable post-parse transform.
///
/// Implementors mutate the tree in place. Transforms that need access to
/// parser-level state (anonymous targets, footnote counters, etc.) receive
/// the [`ParseCtx`] used to produce the tree.
pub trait Transform {
    /// Short, human-readable name used in diagnostics.
    fn name(&self) -> &'static str;

    /// Run the transform against `tree`.
    fn apply(&self, tree: &mut Doctree, ctx: &ParseCtx);
}

/// A builder-style ordered list of transforms.
#[derive(Default)]
pub struct Pipeline {
    transforms: Vec<Box<dyn Transform>>,
}

impl Pipeline {
    /// Empty pipeline.
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
        }
    }

    /// Pipeline pre-populated with the stock transforms in the same order
    /// as [`run_default_pipeline`].
    pub fn default_pipeline() -> Self {
        Self::new()
            .then(ResolveReferences)
            .then(PromoteDocumentTitle)
            .then(PromoteDocinfo)
    }

    /// Append `t` to the pipeline.
    pub fn then<T: Transform + 'static>(mut self, t: T) -> Self {
        self.transforms.push(Box::new(t));
        self
    }

    /// Names of registered transforms, in execution order.
    pub fn names(&self) -> Vec<&'static str> {
        self.transforms.iter().map(|t| t.name()).collect()
    }

    /// Apply every transform in order.
    pub fn run(&self, tree: &mut Doctree, ctx: &ParseCtx) {
        for t in &self.transforms {
            t.apply(tree, ctx);
        }
    }
}

/// Stock transform: resolve internal references.
pub struct ResolveReferences;

impl Transform for ResolveReferences {
    fn name(&self) -> &'static str {
        "resolve_references"
    }
    fn apply(&self, tree: &mut Doctree, ctx: &ParseCtx) {
        resolve_references(tree, ctx);
    }
}

/// Stock transform: promote leading section → document title/subtitle.
pub struct PromoteDocumentTitle;

impl Transform for PromoteDocumentTitle {
    fn name(&self) -> &'static str {
        "promote_document_title"
    }
    fn apply(&self, tree: &mut Doctree, _ctx: &ParseCtx) {
        promote_document_title(tree);
    }
}

/// Stock transform: promote leading field list → `<docinfo>`.
pub struct PromoteDocinfo;

impl Transform for PromoteDocinfo {
    fn name(&self) -> &'static str {
        "promote_docinfo"
    }
    fn apply(&self, tree: &mut Doctree, _ctx: &ParseCtx) {
        promote_docinfo(tree);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pipeline_names() {
        let p = Pipeline::default_pipeline();
        assert_eq!(
            p.names(),
            vec![
                "resolve_references",
                "promote_document_title",
                "promote_docinfo",
            ]
        );
    }

    #[test]
    fn empty_pipeline_has_no_transforms() {
        let p = Pipeline::new();
        assert_eq!(p.names().len(), 0);
    }
}
