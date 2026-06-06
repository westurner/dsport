//! AUTO-GENERATED from `pygments.pygments.lexers.inferno:LimboLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.inferno:LimboLexer:limbo

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: limbo
pub struct LimboLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"whitespace", vec![
        Rule::bygroups(r"(?m)^(\s*)([a-zA-Z_]\w*:)(\s*\n)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::token(r"(?m)\\", STRING),
    ]);
    m.insert(r"statements", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])", NUMBER_FLOAT),
        Rule::token(r"(?m)16r[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)8r[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)((([1-3]\d)|([2-9]))r)?(\d+)", NUMBER_INTEGER),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]|(->)|(<-)|(=>)|(::)", OPERATOR),
        Rule::token(r"(?m)(alt|break|case|continue|cyclic|do|else|exitfor|hd|if|implement|import|include|len|load|orpick|return|spawn|tagof|tl|to|while)\b", KEYWORD),
        Rule::token(r"(?m)(byte|int|big|real|string|array|chan|list|adt|fn|ref|of|module|self|type)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(con|iota|nil)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(r"statement", vec![
        Rule::bygroups(r"(?m)^(\s*)([a-zA-Z_]\w*:)(\s*\n)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])", NUMBER_FLOAT),
        Rule::token(r"(?m)16r[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)8r[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)((([1-3]\d)|([2-9]))r)?(\d+)", NUMBER_INTEGER),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]|(->)|(<-)|(=>)|(::)", OPERATOR),
        Rule::token(r"(?m)(alt|break|case|continue|cyclic|do|else|exitfor|hd|if|implement|import|include|len|load|orpick|return|spawn|tagof|tl|to|while)\b", KEYWORD),
        Rule::token(r"(?m)(byte|int|big|real|string|array|chan|list|adt|fn|ref|of|module|self|type)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(con|iota|nil)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)[{}]", PUNCTUATION),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"root", vec![
        Rule::bygroups(r"(?m)^(\s*)([a-zA-Z_]\w*:)(\s*\n)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::default(NewState::Push(vec![r"statement"])),
    ]);
    Table(m)
}

impl Lexer for LimboLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
