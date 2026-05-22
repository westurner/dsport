//! Lexer registry — `get_lexer_by_name(alias)` mirrors
//! `pygments.lexers.get_lexer_by_name`.
//!
//! Phase 0 ships only the `text` passthrough so end-to-end builds
//! work; Phase 1 adds the Python lexer; Phase 2 widens the set.

pub mod python;
pub mod registry;
pub mod text;
