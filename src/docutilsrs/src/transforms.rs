//! Tree-rewriting transforms run after parsing.
//!
//! Mirrors the docutils transforms layer: the parser produces a raw tree,
//! then a pipeline of transforms resolves cross-references and promotes
//! document structure. Each transform is exposed as a `pub fn` so callers
//! can run a custom pipeline or skip stages.
//!
//! The default pipeline run by [`run_default_pipeline`] matches the order
//! used by `parse_rst_with_source`.

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
