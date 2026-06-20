//! AUTO-GENERATED from `pygments.pygments.lexers.lisp:EmacsLispLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lisp:EmacsLispLexer:emacs_lisp

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: emacs-lisp, elisp, emacs
pub struct EmacsLispLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"root", vec![Rule::default(NewState::Push(vec![r"body"]))]);
    m.insert(r"body", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\?([^\\]|\\.)", STRING_CHAR),
        Rule::token(r"(?m):((?:\\.|[\w!$%&*+-/<=>?@^{}~|])(?:\\.|[\w!$%&*+-/<=>?@^{}~|]|[#.:])*)", NAME_BUILTIN),
        Rule::token(r"(?m)::((?:\\.|[\w!$%&*+-/<=>?@^{}~|])(?:\\.|[\w!$%&*+-/<=>?@^{}~|]|[#.:])*)", STRING_SYMBOL),
        Rule::token(r"(?m)'((?:\\.|[\w!$%&*+-/<=>?@^{}~|])(?:\\.|[\w!$%&*+-/<=>?@^{}~|]|[#.:])*)", STRING_SYMBOL),
        Rule::token(r"(?m)'", OPERATOR),
        Rule::token(r"(?m)`", OPERATOR),
        Rule::token(r#"(?m)[-+]?\d+\.?(?=[ "()\]\'\n,;`])"#, NUMBER_INTEGER),
        Rule::token(r#"(?m)[-+]?\d+/\d+(?=[ "()\]\'\n,;`])"#, NUMBER),
        Rule::token(r#"(?m)[-+]?(\d*\.\d+([defls][-+]?\d+)?|\d+(\.\d*)?[defls][-+]?\d+)(?=[ "()\]\'\n,;`])"#, NUMBER_FLOAT),
        Rule::token(r"(?m)\[|\]", PUNCTUATION),
        Rule::token(r"(?m)#:((?:\\.|[\w!$%&*+-/<=>?@^{}~|])(?:\\.|[\w!$%&*+-/<=>?@^{}~|]|[#.:])*)", STRING_SYMBOL),
        Rule::token(r"(?m)#\^\^?", OPERATOR),
        Rule::token(r"(?m)#\'", NAME_FUNCTION),
        Rule::token(r"(?m)#[bB][+-]?[01]+(/[01]+)?", NUMBER_BIN),
        Rule::token(r"(?m)#[oO][+-]?[0-7]+(/[0-7]+)?", NUMBER_OCT),
        Rule::token(r"(?m)#[xX][+-]?[0-9a-fA-F]+(/[0-9a-fA-F]+)?", NUMBER_HEX),
        Rule::token(r"(?m)#\d+r[+-]?[0-9a-zA-Z]+(/[0-9a-zA-Z]+)?", NUMBER),
        Rule::token(r"(?m)#\d+=", OPERATOR),
        Rule::token(r"(?m)#\d+#", OPERATOR),
        Rule::token(r"(?m)(,@|,|\.|:)", OPERATOR),
        Rule::token(r#"(?m)(t|nil)(?=[ "()\]\'\n,;`])"#, NAME_CONSTANT),
        Rule::token(r"(?m)\*((?:\\.|[\w!$%&*+-/<=>?@^{}~|])(?:\\.|[\w!$%&*+-/<=>?@^{}~|]|[#.:])*)\*", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)((?:\\.|[\w!$%&*+-/<=>?@^{}~|])(?:\\.|[\w!$%&*+-/<=>?@^{}~|]|[#.:])*)", NAME_VARIABLE),
        Rule::token_to(r"(?m)#\(", OPERATOR, NewState::Push(vec![r"body"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"body"])),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)[^"\\`]+"#, STRING),
            Rule::token(
                r"(?m)`((?:\\.|[\w!$%&*+-/<=>?@^{}~|])(?:\\.|[\w!$%&*+-/<=>?@^{}~|]|[#.:])*)\'",
                STRING_SYMBOL,
            ),
            Rule::token(r"(?m)`", STRING),
            Rule::token(r"(?m)\\.", STRING),
            Rule::token(r"(?m)\\\n", STRING),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for EmacsLispLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
