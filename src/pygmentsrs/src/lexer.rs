//! Lexer engine — Phase 0 surface only.
//!
//! The full `RegexLexer` engine (state stack, `bygroups`, `include`,
//! `default`, `words`) lands in Phase 1 alongside the Python lexer.
//! For now, expose just the [`Lexer`] trait so the lexer registry +
//! formatters can compile.

use crate::token::TokenType;

pub trait Lexer: Send + Sync {
    /// Tokenize `code` into `(token-type, value)` pairs. Mirrors
    /// `pygments.lexer.Lexer.get_tokens` (without offset).
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)>;
}
