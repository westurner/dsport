//! AUTO-GENERATED from `pygments.pygments.lexers.ul4:UL4Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ul4:UL4Lexer:ul4

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ul4
pub struct Ul4Lexer;

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
        Rule::bygroups(r"(?ms)(<\?)(\s*)(ul4)(\s*)(\?>)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::bygroups_to(r"(?ms)(<\?)(\s*)(ul4)(\s*)([a-zA-Z_][a-zA-Z_0-9]*)?", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)], NewState::Push(vec![r"ul4"])),
        Rule::token_to(r"(?ms)<\?\s*note\s*\?>", COMMENT, NewState::Push(vec![r"note"])),
        Rule::token(r"(?ms)<\?\s*note\s.*?\?>", COMMENT),
        Rule::token_to(r"(?ms)<\?\s*doc\s*\?>", STRING_DOC, NewState::Push(vec![r"doc"])),
        Rule::token(r"(?ms)<\?\s*doc\s.*?\?>", STRING_DOC),
        Rule::token_to(r"(?ms)<\?\s*ignore\s*\?>", COMMENT, NewState::Push(vec![r"ignore"])),
        Rule::bygroups_to(r"(?ms)(<\?)(\s*)(def)(\s*)([a-zA-Z_][a-zA-Z_0-9]*)?", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)], NewState::Push(vec![r"ul4"])),
        Rule::bygroups_to(r"(?ms)(<\?)(\s*)(printx|print|for|if|elif|else|while|code|renderblocks?|render)\b", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(KEYWORD)], NewState::Push(vec![r"ul4"])),
        Rule::bygroups_to(r"(?ms)(<\?)(\s*)(end)\b", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(KEYWORD)], NewState::Push(vec![r"end"])),
        Rule::bygroups_to(r"(?ms)(<\?)(\s*)(whitespace)\b", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(KEYWORD)], NewState::Push(vec![r"whitespace"])),
        Rule::token(r"(?ms)[^<]+", OTHER),
        Rule::token(r"(?ms)<", OTHER),
    ]);
    m.insert(r"ignore", vec![
        Rule::token_to(r"(?ms)<\?\s*ignore\s*\?>", COMMENT, NewState::PushSame),
        Rule::token_to(r"(?ms)<\?\s*end\s+ignore\s*\?>", COMMENT, NewState::Pop(1)),
        Rule::token(r"(?ms)[^<]+", COMMENT),
        Rule::token(r"(?ms).", COMMENT),
    ]);
    m.insert(r"note", vec![
        Rule::token_to(r"(?ms)<\?\s*note\s*\?>", COMMENT, NewState::PushSame),
        Rule::token_to(r"(?ms)<\?\s*end\s+note\s*\?>", COMMENT, NewState::Pop(1)),
        Rule::token(r"(?ms)[^<]+", COMMENT),
        Rule::token(r"(?ms).", COMMENT),
    ]);
    m.insert(r"doc", vec![
        Rule::token_to(r"(?ms)<\?\s*doc\s*\?>", STRING_DOC, NewState::PushSame),
        Rule::token_to(r"(?ms)<\?\s*end\s+doc\s*\?>", STRING_DOC, NewState::Pop(1)),
        Rule::token(r"(?ms)[^<]+", STRING_DOC),
        Rule::token(r"(?ms).", STRING_DOC),
    ]);
    m.insert(r"ul4", vec![
        Rule::token_to(r"(?ms)\?>", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::token_to(r"(?ms)'''", STRING, NewState::Push(vec![r"string13"])),
        Rule::token_to(r#"(?ms)""""#, STRING, NewState::Push(vec![r"string23"])),
        Rule::token_to(r"(?ms)'", STRING, NewState::Push(vec![r"string1"])),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"string2"])),
        Rule::token(r"(?ms)\d+\.\d*([eE][+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?ms)\.\d+([eE][+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?ms)\d+[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?ms)0[bB][01]+", NUMBER_BIN),
        Rule::token(r"(?ms)0[oO][0-7]+", NUMBER_OCT),
        Rule::token(r"(?ms)0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)@\(\d\d\d\d-\d\d-\d\d(T(\d\d:\d\d(:\d\d(\.\d{6})?)?)?)?\)", LITERAL_DATE),
        Rule::token(r"(?ms)#[0-9a-fA-F]{8}", TokenType::new(&["Literal", "Color"])),
        Rule::token(r"(?ms)#[0-9a-fA-F]{6}", TokenType::new(&["Literal", "Color"])),
        Rule::token(r"(?ms)#[0-9a-fA-F]{3,4}", TokenType::new(&["Literal", "Color"])),
        Rule::token(r"(?ms)\d+", NUMBER_INTEGER),
        Rule::token(r"(?ms)//|==|!=|>=|<=|<<|>>|\+=|-=|\*=|/=|//=|<<=|>>=|&=|\|=|^=|=|[\[\]{},:*/().~%&|<>^+-]", OPERATOR),
        Rule::token(r"(?ms)(and|else|for|i(?:[fns])|not|or)\b", KEYWORD),
        Rule::token(r"(?ms)((?:Fals|Non|Tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)[a-zA-Z_][a-zA-Z0-9_]*", NAME),
        Rule::token(r"(?ms)\s+", WHITESPACE),
    ]);
    m.insert(r"end", vec![
        Rule::token_to(r"(?ms)\?>", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::token(r"(?ms)(def|for|if|renderblock(?:(?:s)?)|while)\b", KEYWORD),
        Rule::token(r"(?ms)\s+", TEXT),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token_to(r"(?ms)\?>", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::token(r"(?ms)(keep|s(?:mart|trip))\b", COMMENT_PREPROC),
        Rule::token(r"(?ms)\s+", WHITESPACE),
    ]);
    m.insert(r"stringescapes", vec![
        Rule::token(r#"(?ms)\\[\\'"abtnfr]"#, STRING_ESCAPE),
        Rule::token(r"(?ms)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\U[0-9a-fA-F]{8}", STRING_ESCAPE),
    ]);
    m.insert(r"string13", vec![
        Rule::token_to(r"(?ms)'''", STRING, NewState::Pop(1)),
        Rule::token(r#"(?ms)\\[\\'"abtnfr]"#, STRING_ESCAPE),
        Rule::token(r"(?ms)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\U[0-9a-fA-F]{8}", STRING_ESCAPE),
        Rule::token(r"(?ms)[^\\']+", STRING),
        Rule::token(r"(?ms).", STRING),
    ]);
    m.insert(r"string23", vec![
        Rule::token_to(r#"(?ms)""""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?ms)\\[\\'"abtnfr]"#, STRING_ESCAPE),
        Rule::token(r"(?ms)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\U[0-9a-fA-F]{8}", STRING_ESCAPE),
        Rule::token(r#"(?ms)[^\\"]+"#, STRING),
        Rule::token(r"(?ms).", STRING),
    ]);
    m.insert(r"string1", vec![
        Rule::token_to(r"(?ms)'", STRING, NewState::Pop(1)),
        Rule::token(r#"(?ms)\\[\\'"abtnfr]"#, STRING_ESCAPE),
        Rule::token(r"(?ms)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\U[0-9a-fA-F]{8}", STRING_ESCAPE),
        Rule::token(r"(?ms)[^\\']+", STRING),
        Rule::token(r"(?ms).", STRING),
    ]);
    m.insert(r"string2", vec![
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?ms)\\[\\'"abtnfr]"#, STRING_ESCAPE),
        Rule::token(r"(?ms)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
        Rule::token(r"(?ms)\\U[0-9a-fA-F]{8}", STRING_ESCAPE),
        Rule::token(r#"(?ms)[^\\"]+"#, STRING),
        Rule::token(r"(?ms).", STRING),
    ]);
    Table(m)
}

impl Lexer for Ul4Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
