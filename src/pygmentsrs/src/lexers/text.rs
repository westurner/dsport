//! Trivial passthrough lexer — `pygments.lexers.special.TextLexer`.
//!
//! Emits the entire input as a single `Token.Text` token. Used as
//! the default for `.. code-block:: text` in docutils.

use crate::lexer::Lexer;
use crate::token::{self, TokenType};

pub struct TextLexer;

impl Lexer for TextLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        if code.is_empty() {
            return Vec::new();
        }
        vec![(token::TEXT, code.to_string())]
    }
}
