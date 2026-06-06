//! AUTO-GENERATED from `pygments.pygments.lexers.lisp:CommonLispLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lisp:CommonLispLexer:common_lisp

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: common-lisp, cl, lisp
pub struct CommonLispLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"root", vec![
        Rule::default(NewState::Push(vec![r"body"])),
    ]);
    m.insert(r"multiline-comment", vec![
        Rule::token_to(r"(?im)#\|", COMMENT_MULTILINE, NewState::PushSame),
        Rule::token_to(r"(?im)\|#", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?im)[^|#]+", COMMENT_MULTILINE),
        Rule::token(r"(?im)[|#]", COMMENT_MULTILINE),
    ]);
    m.insert(r"commented-form", vec![
        Rule::token_to(r"(?im)\(", COMMENT_PREPROC, NewState::PushSame),
        Rule::token_to(r"(?im)\)", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::token(r"(?im)[^()]+", COMMENT_PREPROC),
    ]);
    m.insert(r"body", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im);.*$", COMMENT_SINGLE),
        Rule::token_to(r"(?im)#\|", COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comment"])),
        Rule::token(r"(?im)#\d*Y.*$", COMMENT_SPECIAL),
        Rule::token(r#"(?im)"(\\.|\\\n|[^"\\])*""#, STRING),
        Rule::token(r"(?im):(\|[^|]+\||(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~])(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~]|[#.:])*)", STRING_SYMBOL),
        Rule::token(r"(?im)::(\|[^|]+\||(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~])(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~]|[#.:])*)", STRING_SYMBOL),
        Rule::token(r"(?im):#(\|[^|]+\||(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~])(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~]|[#.:])*)", STRING_SYMBOL),
        Rule::token(r"(?im)'(\|[^|]+\||(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~])(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~]|[#.:])*)", STRING_SYMBOL),
        Rule::token(r"(?im)'", OPERATOR),
        Rule::token(r"(?im)`", OPERATOR),
        Rule::token(r#"(?im)[-+]?\d+\.?(?=[ "()\'\n,;`])"#, NUMBER_INTEGER),
        Rule::token(r#"(?im)[-+]?\d+/\d+(?=[ "()\'\n,;`])"#, NUMBER),
        Rule::token(r#"(?im)[-+]?(\d*\.\d+([defls][-+]?\d+)?|\d+(\.\d*)?[defls][-+]?\d+)(?=[ "()\'\n,;`])"#, NUMBER_FLOAT),
        Rule::token(r#"(?im)#\\.(?=[ "()\'\n,;`])"#, STRING_CHAR),
        Rule::token(r"(?im)#\\(\|[^|]+\||(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~])(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~]|[#.:])*)", STRING_CHAR),
        Rule::token_to(r"(?im)#\(", OPERATOR, NewState::Push(vec![r"body"])),
        Rule::token(r"(?im)#\d*\*[01]*", TokenType::new(&["Literal", "Other"])),
        Rule::token(r"(?im)#:(\|[^|]+\||(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~])(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~]|[#.:])*)", STRING_SYMBOL),
        Rule::token(r"(?im)#[.,]", OPERATOR),
        Rule::token(r"(?im)#\'", NAME_FUNCTION),
        Rule::token(r"(?im)#b[+-]?[01]+(/[01]+)?", NUMBER_BIN),
        Rule::token(r"(?im)#o[+-]?[0-7]+(/[0-7]+)?", NUMBER_OCT),
        Rule::token(r"(?im)#x[+-]?[0-9a-f]+(/[0-9a-f]+)?", NUMBER_HEX),
        Rule::token(r"(?im)#\d+r[+-]?[0-9a-z]+(/[0-9a-z]+)?", NUMBER),
        Rule::bygroups_to(r"(?im)(#c)(\()", vec![Some(NUMBER), Some(PUNCTUATION)], NewState::Push(vec![r"body"])),
        Rule::bygroups_to(r"(?im)(#\d+a)(\()", vec![Some(TokenType::new(&["Literal", "Other"])), Some(PUNCTUATION)], NewState::Push(vec![r"body"])),
        Rule::bygroups_to(r"(?im)(#s)(\()", vec![Some(TokenType::new(&["Literal", "Other"])), Some(PUNCTUATION)], NewState::Push(vec![r"body"])),
        Rule::token(r#"(?im)#p?"(\\.|[^"\\])*""#, TokenType::new(&["Literal", "Other"])),
        Rule::token(r"(?im)#\d+=", OPERATOR),
        Rule::token(r"(?im)#\d+#", OPERATOR),
        Rule::token_to(r#"(?im)#+nil(?=[ "()\'\n,;`])\s*\("#, COMMENT_PREPROC, NewState::Push(vec![r"commented-form"])),
        Rule::token(r"(?im)#[+-]", OPERATOR),
        Rule::token(r"(?im)(,@|,|\.)", OPERATOR),
        Rule::token(r#"(?im)(t|nil)(?=[ "()\'\n,;`])"#, NAME_CONSTANT),
        Rule::token(r"(?im)\*(\|[^|]+\||(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~])(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~]|[#.:])*)\*", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?im)(\|[^|]+\||(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~])(?:\\.|[\w!$%&*+-/<=>?@\[\]^{}~]|[#.:])*)", NAME_VARIABLE),
        Rule::token_to(r"(?im)\(", PUNCTUATION, NewState::Push(vec![r"body"])),
        Rule::token_to(r"(?im)\)", PUNCTUATION, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for CommonLispLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
