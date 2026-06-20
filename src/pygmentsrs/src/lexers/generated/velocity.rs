#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.templates:VelocityLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:VelocityLexer:velocity

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: velocity
pub struct VelocityLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?ms)[^{#$]+", OTHER),
            Rule::bygroups(
                r"(?ms)(#)(\*.*?\*)(#)",
                vec![Some(COMMENT_PREPROC), Some(COMMENT), Some(COMMENT_PREPROC)],
            ),
            Rule::bygroups(
                r"(?ms)(##)(.*?$)",
                vec![Some(COMMENT_PREPROC), Some(COMMENT)],
            ),
            Rule::bygroups_to(
                r"(?ms)(#\{?)([a-zA-Z_]\w*)(\}?)(\s?\()",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(NAME_FUNCTION),
                    Some(COMMENT_PREPROC),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"directiveparams"]),
            ),
            Rule::bygroups(
                r"(?ms)(#\{?)([a-zA-Z_]\w*)(\}|\b)",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(NAME_FUNCTION),
                    Some(COMMENT_PREPROC),
                ],
            ),
            Rule::token_to(
                r"(?ms)\$!?\{?",
                PUNCTUATION,
                NewState::Push(vec![r"variable"]),
            ),
        ],
    );
    m.insert(
        r"variable",
        vec![
            Rule::token(r"(?ms)[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token_to(r"(?ms)\(", PUNCTUATION, NewState::Push(vec![r"funcparams"])),
            Rule::bygroups_to(
                r"(?ms)(\.)([a-zA-Z_]\w*)",
                vec![Some(PUNCTUATION), Some(NAME_VARIABLE)],
                NewState::PushSame,
            ),
            Rule::token_to(r"(?ms)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"directiveparams",
        vec![
            Rule::token(
                r"(?ms)(&&|\|\||==?|!=?|[-<>+*%&|^/])|\b(eq|ne|gt|lt|ge|le|not|in)\b",
                OPERATOR,
            ),
            Rule::token_to(r"(?ms)\[", OPERATOR, NewState::Push(vec![r"rangeoperator"])),
            Rule::token(r"(?ms)\b[a-zA-Z_]\w*\b", NAME_FUNCTION),
            Rule::token_to(
                r"(?ms)\$!?\{?",
                PUNCTUATION,
                NewState::Push(vec![r"variable"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)[,:]", PUNCTUATION),
            Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?ms)0[xX][0-9a-fA-F]+[Ll]?", NUMBER),
            Rule::token(r"(?ms)\b[0-9]+\b", NUMBER),
            Rule::token(r"(?ms)(true|false|null)\b", KEYWORD_CONSTANT),
            Rule::token_to(r"(?ms)\(", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?ms)\{", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?ms)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?ms)\[", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?ms)\]", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"funcparams",
        vec![
            Rule::token_to(
                r"(?ms)\$!?\{?",
                PUNCTUATION,
                NewState::Push(vec![r"variable"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)[,:]", PUNCTUATION),
            Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?ms)0[xX][0-9a-fA-F]+[Ll]?", NUMBER),
            Rule::token(r"(?ms)\b[0-9]+\b", NUMBER),
            Rule::token(r"(?ms)(true|false|null)\b", KEYWORD_CONSTANT),
            Rule::token_to(r"(?ms)\(", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?ms)\{", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?ms)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?ms)\[", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?ms)\]", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"rangeoperator",
        vec![
            Rule::token(r"(?ms)\.\.", OPERATOR),
            Rule::token_to(
                r"(?ms)\$!?\{?",
                PUNCTUATION,
                NewState::Push(vec![r"variable"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)[,:]", PUNCTUATION),
            Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?ms)0[xX][0-9a-fA-F]+[Ll]?", NUMBER),
            Rule::token(r"(?ms)\b[0-9]+\b", NUMBER),
            Rule::token(r"(?ms)(true|false|null)\b", KEYWORD_CONSTANT),
            Rule::token_to(r"(?ms)\(", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?ms)\{", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?ms)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?ms)\[", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?ms)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?ms)\]", OPERATOR, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for VelocityLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
