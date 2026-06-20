#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.basic:CbmBasicV2Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.basic:CbmBasicV2Lexer:cbmbas

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cbmbas
pub struct CbmbasLexer;

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
        Rule::token(r"(?im)rem.*\n", COMMENT_SINGLE),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)new|run|end|for|to|next|step|go(to|sub)?|on|return|stop|cont|if|then|input#?|read|wait|load|save|verify|poke|sys|print#?|list|clr|cmd|open|close|get#?", KEYWORD_RESERVED),
        Rule::token(r"(?im)data|restore|dim|let|def|fn", KEYWORD_DECLARATION),
        Rule::token(r"(?im)tab|spc|sgn|int|abs|usr|fre|pos|sqr|rnd|log|exp|cos|sin|tan|atn|peek|len|val|asc|(str|chr|left|right|mid)\$", NAME_BUILTIN),
        Rule::token(r"(?im)[-+*/^<>=]", OPERATOR),
        Rule::token(r"(?im)not|and|or", OPERATOR_WORD),
        Rule::token(r#"(?im)"[^"\n]*."#, STRING),
        Rule::token(r"(?im)\d+|[-+]?\d*\.\d*(e[-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)[(),:;]", PUNCTUATION),
        Rule::token(r"(?im)\w+[$%]?", NAME),
    ]);
    Table(m)
}

impl Lexer for CbmbasLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
