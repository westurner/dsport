#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.modula2:Modula2Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.modula2:Modula2Lexer:modula2

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: modula2, m2
pub struct Modula2Lexer;

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
        r"whitespace",
        vec![
            Rule::token(r"(?ms)\n+", TEXT),
            Rule::token(r"(?ms)\s+", TEXT),
        ],
    );
    m.insert(
        r"dialecttags",
        vec![
            Rule::token(r"(?ms)\(\*!m2pim\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2iso\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2r10\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!objm2\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2iso\+aglet\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2pim\+gm2\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2iso\+p1\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2iso\+xds\*\)", COMMENT_SPECIAL),
        ],
    );
    m.insert(
        r"identifiers",
        vec![Rule::token(r"(?ms)([a-zA-Z_$][\w$]*)", NAME)],
    );
    m.insert(
        r"prefixed_number_literals",
        vec![
            Rule::token(r"(?ms)0b[01]+(\'[01]+)*", NUMBER_BIN),
            Rule::token(r"(?ms)0[ux][0-9A-F]+(\'[0-9A-F]+)*", NUMBER_HEX),
        ],
    );
    m.insert(
        r"plain_number_literals",
        vec![
            Rule::token(
                r"(?ms)[0-9]+(\'[0-9]+)*\.[0-9]+(\'[0-9]+)*[eE][+-]?[0-9]+(\'[0-9]+)*",
                NUMBER_FLOAT,
            ),
            Rule::token(r"(?ms)[0-9]+(\'[0-9]+)*\.[0-9]+(\'[0-9]+)*", NUMBER_FLOAT),
            Rule::token(r"(?ms)[0-9]+(\'[0-9]+)*", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"suffixed_number_literals",
        vec![
            Rule::token(r"(?ms)[0-7]+B", NUMBER_OCT),
            Rule::token(r"(?ms)[0-7]+C", NUMBER_OCT),
            Rule::token(r"(?ms)[0-9A-F]+H", NUMBER_HEX),
        ],
    );
    m.insert(
        r"string_literals",
        vec![
            Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        ],
    );
    m.insert(
        r"digraph_operators",
        vec![
            Rule::token(r"(?ms)\*\.", OPERATOR),
            Rule::token(r"(?ms)\+>", OPERATOR),
            Rule::token(r"(?ms)<>", OPERATOR),
            Rule::token(r"(?ms)<=", OPERATOR),
            Rule::token(r"(?ms)>=", OPERATOR),
            Rule::token(r"(?ms)==", OPERATOR),
            Rule::token(r"(?ms)::", OPERATOR),
            Rule::token(r"(?ms):=", OPERATOR),
            Rule::token(r"(?ms)\+\+", OPERATOR),
            Rule::token(r"(?ms)--", OPERATOR),
        ],
    );
    m.insert(
        r"unigraph_operators",
        vec![
            Rule::token(r"(?ms)[+-]", OPERATOR),
            Rule::token(r"(?ms)[*/]", OPERATOR),
            Rule::token(r"(?ms)\\", OPERATOR),
            Rule::token(r"(?ms)[=#<>]", OPERATOR),
            Rule::token(r"(?ms)\^", OPERATOR),
            Rule::token(r"(?ms)@", OPERATOR),
            Rule::token(r"(?ms)&", OPERATOR),
            Rule::token(r"(?ms)~", OPERATOR),
            Rule::token(r"(?ms)`", OPERATOR),
        ],
    );
    m.insert(
        r"digraph_punctuation",
        vec![
            Rule::token(r"(?ms)\.\.", PUNCTUATION),
            Rule::token(r"(?ms)<<", PUNCTUATION),
            Rule::token(r"(?ms)>>", PUNCTUATION),
            Rule::token(r"(?ms)->", PUNCTUATION),
            Rule::token(r"(?ms)\|#", PUNCTUATION),
            Rule::token(r"(?ms)##", PUNCTUATION),
            Rule::token(r"(?ms)\|\*", PUNCTUATION),
        ],
    );
    m.insert(
        r"unigraph_punctuation",
        vec![
            Rule::token(r"(?ms)[()\[\]{},.:;|]", PUNCTUATION),
            Rule::token(r"(?ms)!", PUNCTUATION),
            Rule::token(r"(?ms)\?", PUNCTUATION),
        ],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?ms)^//.*?\n", COMMENT_SINGLE),
            Rule::token(r"(?ms)\(\*([^$].*?)\*\)", COMMENT_MULTILINE),
            Rule::token(r"(?ms)/\*(.*?)\*/", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"pragmas",
        vec![
            Rule::token(r"(?ms)<\*.*?\*>", COMMENT_PREPROC),
            Rule::token(r"(?ms)\(\*\$.*?\*\)", COMMENT_PREPROC),
        ],
    );
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?ms)\n+", TEXT),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)\(\*!m2pim\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2iso\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2r10\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!objm2\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2iso\+aglet\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2pim\+gm2\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2iso\+p1\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)\(\*!m2iso\+xds\*\)", COMMENT_SPECIAL),
            Rule::token(r"(?ms)<\*.*?\*>", COMMENT_PREPROC),
            Rule::token(r"(?ms)\(\*\$.*?\*\)", COMMENT_PREPROC),
            Rule::token(r"(?ms)^//.*?\n", COMMENT_SINGLE),
            Rule::token(r"(?ms)\(\*([^$].*?)\*\)", COMMENT_MULTILINE),
            Rule::token(r"(?ms)/\*(.*?)\*/", COMMENT_MULTILINE),
            Rule::token(r"(?ms)([a-zA-Z_$][\w$]*)", NAME),
            Rule::token(r"(?ms)[0-7]+B", NUMBER_OCT),
            Rule::token(r"(?ms)[0-7]+C", NUMBER_OCT),
            Rule::token(r"(?ms)[0-9A-F]+H", NUMBER_HEX),
            Rule::token(r"(?ms)0b[01]+(\'[01]+)*", NUMBER_BIN),
            Rule::token(r"(?ms)0[ux][0-9A-F]+(\'[0-9A-F]+)*", NUMBER_HEX),
            Rule::token(
                r"(?ms)[0-9]+(\'[0-9]+)*\.[0-9]+(\'[0-9]+)*[eE][+-]?[0-9]+(\'[0-9]+)*",
                NUMBER_FLOAT,
            ),
            Rule::token(r"(?ms)[0-9]+(\'[0-9]+)*\.[0-9]+(\'[0-9]+)*", NUMBER_FLOAT),
            Rule::token(r"(?ms)[0-9]+(\'[0-9]+)*", NUMBER_INTEGER),
            Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?ms)\.\.", PUNCTUATION),
            Rule::token(r"(?ms)<<", PUNCTUATION),
            Rule::token(r"(?ms)>>", PUNCTUATION),
            Rule::token(r"(?ms)->", PUNCTUATION),
            Rule::token(r"(?ms)\|#", PUNCTUATION),
            Rule::token(r"(?ms)##", PUNCTUATION),
            Rule::token(r"(?ms)\|\*", PUNCTUATION),
            Rule::token(r"(?ms)\*\.", OPERATOR),
            Rule::token(r"(?ms)\+>", OPERATOR),
            Rule::token(r"(?ms)<>", OPERATOR),
            Rule::token(r"(?ms)<=", OPERATOR),
            Rule::token(r"(?ms)>=", OPERATOR),
            Rule::token(r"(?ms)==", OPERATOR),
            Rule::token(r"(?ms)::", OPERATOR),
            Rule::token(r"(?ms):=", OPERATOR),
            Rule::token(r"(?ms)\+\+", OPERATOR),
            Rule::token(r"(?ms)--", OPERATOR),
            Rule::token(r"(?ms)[()\[\]{},.:;|]", PUNCTUATION),
            Rule::token(r"(?ms)!", PUNCTUATION),
            Rule::token(r"(?ms)\?", PUNCTUATION),
            Rule::token(r"(?ms)[+-]", OPERATOR),
            Rule::token(r"(?ms)[*/]", OPERATOR),
            Rule::token(r"(?ms)\\", OPERATOR),
            Rule::token(r"(?ms)[=#<>]", OPERATOR),
            Rule::token(r"(?ms)\^", OPERATOR),
            Rule::token(r"(?ms)@", OPERATOR),
            Rule::token(r"(?ms)&", OPERATOR),
            Rule::token(r"(?ms)~", OPERATOR),
            Rule::token(r"(?ms)`", OPERATOR),
        ],
    );
    Table(m)
}

impl Lexer for Modula2Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
