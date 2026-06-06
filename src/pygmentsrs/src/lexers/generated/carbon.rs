//! AUTO-GENERATED from `pygments.pygments.lexers.carbon:CarbonLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.carbon:CarbonLexer:carbon

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: carbon
pub struct CarbonLexer;

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
        Rule::token(r"(?ms)\n", WHITESPACE),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)\\\n", TEXT),
        Rule::token(r"(?ms)//(.*?)\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/(\\\n)?[*].*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)(package|import|api|namespace|library)\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?ms)(abstract|alias|fn|class|interface|let|var|virtual|external|base|addr|extends|choice|constraint|impl)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)(Self|a(?:nd|s)|break|c(?:(?:as|ontinu)e)|de(?:fault|structor)|else|f(?:inal|or(?:(?:all)?)|riend)|i(?:[fns])|like|match|not|o(?:bserve|r|verride)|p(?:artial|r(?:ivate|otected))|return(?:(?:ed)?)|t(?:hen|ype)|wh(?:(?:er|il)e))\b", KEYWORD),
        Rule::token(r"(?ms)(self)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?ms)(true|false)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(auto|bool|string|i8|i16|i32|i64|u8|u16|u32|u64|f8|f16|f32|f64)\b", KEYWORD_TYPE),
        Rule::token(r"(?ms)[0-9]*[.][0-9]+", TokenType::new(&["Literal", "Number", "Double"])),
        Rule::token(r"(?ms)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?ms)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r#"(?ms)"(\\.|[^"\\])*""#, STRING),
        Rule::token(r"(?ms)\'(\\.|[^\'\\])\'", STRING_CHAR),
        Rule::token(r"(?ms)<<=|>>=|<<|>>|<=|>=|\+=|-=|\*=|/=|\%=|\|=|&=|\^=|&&|\|\||&|\||\+\+|--|\%|\^|\~|==|!=|::|[.]{3}|->|=>|[+\-*/&]", OPERATOR),
        Rule::token(r"(?ms)[|<>=!()\[\]{}.,;:\?]", PUNCTUATION),
        Rule::token(r"(?ms)[^\W\d]\w*", NAME_OTHER),
    ]);
    Table(m)
}

impl Lexer for CarbonLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
