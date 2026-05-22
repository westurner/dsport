//! Lexer alias → lexer instance resolution.
//!
//! Mirrors `pygments.lexers.get_lexer_by_name`. New lexers register
//! their aliases by extending the `match` below.

use crate::lexer::Lexer;
use crate::lexers::python::PythonLexer;
use crate::lexers::text::TextLexer;

pub fn get_lexer_by_name(alias: &str) -> Option<Box<dyn Lexer>> {
    match alias {
        "text" | "plain" | "" => Some(Box::new(TextLexer)),
        "python" | "py" | "python3" | "py3" => Some(Box::new(PythonLexer)),
        _ => None,
    }
}
