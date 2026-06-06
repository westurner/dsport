//! AUTO-GENERATED from `pygments.pygments.lexers.scripting:JclLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scripting:JclLexer:jcl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: jcl
pub struct JclLexer;

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
        Rule::token(r"(?im)//\*.*\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)//", KEYWORD_PSEUDO, NewState::Push(vec![r"statement"])),
        Rule::token_to(r"(?im)/\*", KEYWORD_PSEUDO, NewState::Push(vec![r"jes2_statement"])),
        Rule::token(r"(?im).*\n", OTHER),
    ]);
    m.insert(r"statement", vec![
        Rule::token_to(r"(?im)\s*\n", WHITESPACE, NewState::Pop(1)),
        Rule::bygroups_to(r"(?im)([a-z]\w*)(\s+)(exec|job)(\s*)", vec![Some(NAME_LABEL), Some(WHITESPACE), Some(KEYWORD_RESERVED), Some(WHITESPACE)], NewState::Push(vec![r"option"])),
        Rule::token_to(r"(?im)[a-z]\w*", NAME_VARIABLE, NewState::Push(vec![r"statement_command"])),
        Rule::token_to(r"(?im)\s+", WHITESPACE, NewState::Push(vec![r"statement_command"])),
    ]);
    m.insert(r"statement_command", vec![
        Rule::token_to(r"(?im)\s+(command|cntl|dd|endctl|endif|else|include|jcllib|output|pend|proc|set|then|xmit)\s+", KEYWORD_RESERVED, NewState::Push(vec![r"option"])),
        Rule::token(r"(?im)\*", NAME_BUILTIN),
        Rule::token(r"(?im)[\[\](){}<>;,]", PUNCTUATION),
        Rule::token(r"(?im)[-+*/=&%]", OPERATOR),
        Rule::token(r"(?im)[a-z_]\w*", NAME),
        Rule::token(r"(?im)\d+\.\d*", NUMBER_FLOAT),
        Rule::token(r"(?im)\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?im)\d+", NUMBER_INTEGER),
        Rule::token_to(r"(?im)'", STRING, NewState::Push(vec![r"option_string"])),
        Rule::token_to(r"(?im)[ \t]+", WHITESPACE, NewState::Push(vec![r"option_comment"])),
        Rule::token(r"(?im)\.", PUNCTUATION),
    ]);
    m.insert(r"option", vec![
        Rule::token(r"(?im)\*", NAME_BUILTIN),
        Rule::token(r"(?im)[\[\](){}<>;,]", PUNCTUATION),
        Rule::token(r"(?im)[-+*/=&%]", OPERATOR),
        Rule::token(r"(?im)[a-z_]\w*", NAME),
        Rule::token(r"(?im)\d+\.\d*", NUMBER_FLOAT),
        Rule::token(r"(?im)\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?im)\d+", NUMBER_INTEGER),
        Rule::token_to(r"(?im)'", STRING, NewState::Push(vec![r"option_string"])),
        Rule::token_to(r"(?im)[ \t]+", WHITESPACE, NewState::Push(vec![r"option_comment"])),
        Rule::token(r"(?im)\.", PUNCTUATION),
    ]);
    m.insert(r"jes2_statement", vec![
        Rule::token_to(r"(?im)\s*\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r"(?im)\$", KEYWORD, NewState::Push(vec![r"option"])),
        Rule::token_to(r"(?im)\b(jobparam|message|netacct|notify|output|priority|route|setup|signoff|xeq|xmit)\b", KEYWORD, NewState::Push(vec![r"option"])),
    ]);
    m.insert(r"option_string", vec![
        Rule::bygroups(r"(?im)(\n)(//)", vec![Some(TEXT), Some(KEYWORD_PSEUDO)]),
        Rule::token(r"(?im)''", STRING),
        Rule::token(r"(?im)[^']", STRING),
        Rule::token_to(r"(?im)'", STRING, NewState::Pop(1)),
    ]);
    m.insert(r"option_comment", vec![
        Rule::token(r"(?im).+", COMMENT_SINGLE),
    ]);
    Table(m)
}

impl Lexer for JclLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
