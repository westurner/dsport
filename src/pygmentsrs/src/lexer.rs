//! Lexer engine module.
//!
//! The [`Lexer`] trait is the surface concrete lexers implement.
//! The [`engine`] submodule houses the `RegexLexer` port (state
//! machine, `bygroups`, `default`).

use crate::token::TokenType;

pub mod engine;

pub trait Lexer: Send + Sync {
    /// Tokenize `code` into `(token-type, value)` pairs. Mirrors
    /// `pygments.lexer.Lexer.get_tokens` with the standard pygments
    /// behavior of merging adjacent same-type tokens.
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)>;
}
