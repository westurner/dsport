//! JSON-LD lexer — port of `pygments.lexers.data.JsonLdLexer`.
//!
//! Extends the native [`JsonLexer`] with JSON-LD keyword highlighting:
//! `"@context"`, `"@type"`, `"@id"`, etc. are emitted as
//! `Token.Name.Decorator` instead of `Token.Name.Tag`.
//!
//! This is a direct port of the Python class:
//!
//! ```python
//! class JsonLdLexer(JsonLexer):
//!     def get_tokens_unprocessed(self, text):
//!         for start, token, value in super().get_tokens_unprocessed(text):
//!             if token is Name.Tag and value in self.json_ld_keywords:
//!                 yield start, Name.Decorator, value
//!             else:
//!                 yield start, token, value
//! ```

use crate::lexer::Lexer;
use crate::lexers::json::JsonLexer;
use crate::token::{NAME_DECORATOR, NAME_TAG, TokenType};

/// All JSON-LD keyword strings as they appear in token output
/// (including the surrounding double quotes, matching the `Name.Tag`
/// value emitted by `JsonLexer`).
const JSON_LD_KEYWORDS: &[&str] = &[
    "\"@base\"",
    "\"@container\"",
    "\"@context\"",
    "\"@direction\"",
    "\"@graph\"",
    "\"@id\"",
    "\"@import\"",
    "\"@included\"",
    "\"@index\"",
    "\"@json\"",
    "\"@language\"",
    "\"@list\"",
    "\"@nest\"",
    "\"@none\"",
    "\"@prefix\"",
    "\"@propagate\"",
    "\"@protected\"",
    "\"@reverse\"",
    "\"@set\"",
    "\"@type\"",
    "\"@value\"",
    "\"@version\"",
    "\"@vocab\"",
];

fn is_json_ld_keyword(s: &str) -> bool {
    // Linear scan is fine — only 23 keywords.
    JSON_LD_KEYWORDS.contains(&s)
}

pub struct JsonLdLexer;

impl Lexer for JsonLdLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        JsonLexer
            .get_tokens(code)
            .into_iter()
            .map(|(t, v)| {
                if t == NAME_TAG && is_json_ld_keyword(&v) {
                    (NAME_DECORATOR, v)
                } else {
                    (t, v)
                }
            })
            .collect()
    }
}
