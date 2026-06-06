//! AUTO-GENERATED from `pygments.pygments.lexers.scripting:MiniScriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scripting:MiniScriptLexer:miniscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: miniscript, ms
pub struct MiniscriptLexer;

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
        Rule::token(r"(?m)#!(.*?)$", COMMENT_PREPROC),
        Rule::default(NewState::Push(vec![r"base"])),
    ]);
    m.insert(r"base", vec![
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)(?i)(\d*\.\d+|\d+\.\d*)(e[+-]?\d+)?", NUMBER),
        Rule::token(r"(?m)(?i)\d+e[+-]?\d+", NUMBER),
        Rule::token(r"(?m)\d+", NUMBER),
        Rule::token(r"(?m)\n", TEXT),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string_double"])),
        Rule::token(r"(?m)(==|!=|<=|>=|[=+\-*/%^<>.:])", OPERATOR),
        Rule::token(r"(?m)[;,\[\]{}()]", PUNCTUATION),
        Rule::token(r"(?m)(break|continue|e(?:lse|nd)|f(?:or|unction)|i(?:sa|[fn])|re(?:peat|turn)|then|while)\b", KEYWORD),
        Rule::token(r"(?m)(a(?:bs|cos|(?:si|ta)n)|c(?:eil|har|o(?:de|s))|floor|globals|hasIndex|in(?:dex(?:Of|es)|put)|join|l(?:en|o(?:cals|g|wer))|outer|p(?:i|op|rint|u(?:ll|sh))|r(?:ange|e(?:(?:mov|plac)e)|(?:(?:ou)?)nd)|s(?:huffle|i(?:(?:(?:g)?)n)|ort|plit|qrt|tr|um)|t(?:an|ime)|upper|val(?:(?:ues)?)|wait|yield)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(and|or|not|new)\b", OPERATOR_WORD),
        Rule::token(r"(?m)(self|super|__isa)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
    ]);
    m.insert(r"string_double", vec![
        Rule::token(r#"(?m)[^"\n]+"#, STRING),
        Rule::token(r#"(?m)"""#, STRING),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for MiniscriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
