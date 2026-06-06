//! AUTO-GENERATED from `pygments.pygments.lexers.kusto:KustoLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.kusto:KustoLexer:kql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: kql, kusto
pub struct KqlLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)(!(?:contains|has|startswith)|a(?:nd|s)|b(?:etween|ool|y)|co(?:n(?:sume|tains(?:(?:cs)?))|unt)|d(?:ate(?:(?:time)?)|istinct)|e(?:valuate|xtend)|f(?:acet|i(?:lter|nd)|ork)|getschema|has|in(?:t|voke)|join|l(?:imit|o(?:ng|okup))|m(?:a(?:ke\-series|tches\ regex)|v\-(?:apply|expand))|notcontains(?:(?:cs)?)|o(?:rder|[nr])|p(?:ar(?:se(?:(?:\-(?:kv|where))?)|tition)|r(?:int|oject(?:(?:\-(?:away|keep|re(?:name|order)))?)))|r(?:ange|e(?:al|duce|gex|nder))|s(?:ample(?:(?:\-distinct)?)|can|e(?:arch|rialize)|ort|t(?:artswith|ring)|ummarize)|t(?:ake|ime|op(?:(?:\-(?:hitters|nested))?)|ypeof)|union|where)\b", KEYWORD),
        Rule::token(r"(?m)//.*", COMMENT),
        Rule::token(r"(?m)(!(?:[=~])|\.\.!|<(?:[=>|])|=(?:[=>~])|>=|[%()*+,\-/:;<=>?\[\]{|}])", PUNCTUATION),
        Rule::token(r"(?m)[^\W\d]\w*", NAME),
        Rule::token(r"(?m)\d+[.]\d*|[.]\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"single_string"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"double_string"])),
        Rule::token_to(r"(?m)@'", STRING, NewState::Push(vec![r"single_verbatim"])),
        Rule::token_to(r#"(?m)@""#, STRING, NewState::Push(vec![r"double_verbatim"])),
        Rule::token_to(r"(?m)```", STRING, NewState::Push(vec![r"multi_string"])),
    ]);
    m.insert(r"single_string", vec![
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token(r"(?m)[^'\\]+", STRING),
    ]);
    m.insert(r"double_string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token(r#"(?m)[^"\\]+"#, STRING),
    ]);
    m.insert(r"single_verbatim", vec![
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)[^']+", STRING),
    ]);
    m.insert(r"double_verbatim", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)[^"]+"#, STRING),
    ]);
    m.insert(r"multi_string", vec![
        Rule::token(r"(?m)[^`]+", STRING),
        Rule::token_to(r"(?m)```", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)`", STRING),
    ]);
    Table(m)
}

impl Lexer for KqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
