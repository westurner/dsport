//! AUTO-GENERATED from `pygments.pygments.lexers.trafficscript:RtsLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.trafficscript:RtsLexer:trafficscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: trafficscript, rts
pub struct TrafficscriptLexer;

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
        Rule::token(r"(?m)'(\\\\|\\[^\\]|[^'\\])*'", STRING),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"escapable-string"])),
        Rule::token(r"(?m)(0x[0-9a-fA-F]+|\d+)", NUMBER),
        Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\$[a-zA-Z](\w|_)*", NAME_VARIABLE),
        Rule::token(r"(?m)(if|else|for(each)?|in|while|do|break|sub|return|import)", KEYWORD),
        Rule::token(r"(?m)[a-zA-Z][\w.]*", NAME_FUNCTION),
        Rule::token(r"(?m)[-+*/%=,;(){}<>^.!~|&\[\]\?\:]", OPERATOR),
        Rule::token(r"(?m)(>=|<=|==|!=|&&|\|\||\+=|.=|-=|\*=|/=|%=|<<=|>>=|&=|\|=|\^=|>>|<<|\+\+|--|=>)", OPERATOR),
        Rule::token(r"(?m)[ \t\r]+", TEXT),
        Rule::token(r"(?m)#[^\n]*", COMMENT),
    ]);
    m.insert(r"escapable-string", vec![
        Rule::token(r"(?m)\\[tsn]", STRING_ESCAPE),
        Rule::token(r#"(?m)[^"]"#, STRING),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for TrafficscriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
