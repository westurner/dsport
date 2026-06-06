//! AUTO-GENERATED from `pygments.pygments.lexers.scripting:ChaiscriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scripting:ChaiscriptLexer:chaiscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: chaiscript, chai
pub struct ChaiscriptLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"commentsandwhitespace", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)^\#.*?\n", COMMENT_SINGLE),
    ]);
    m.insert(r"slashstartsregex", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)^\#.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)/(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gim]+\b|\B)", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?=/)", TEXT, NewState::Push(vec![r"#pop", r"badregex"])),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"badregex", vec![
        Rule::token_to(r"(?ms)\n", TEXT, NewState::Pop(1)),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)^\#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)\n", TEXT),
        Rule::token(r"(?ms)[^\S\n]+", TEXT),
        Rule::token_to(r"(?ms)\+\+|--|~|&&|\?|:|\|\||\\(?=\n)|\.\.(<<|>>>?|==?|!=?|[-<>+*%&|^/])=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[})\].]", PUNCTUATION),
        Rule::token(r"(?ms)[=+\-*/]", OPERATOR),
        Rule::token_to(r"(?ms)(for|in|while|do|break|return|continue|if|else|throw|try|catch)\b", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)(var)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)(attr|def|fun)\b", KEYWORD_RESERVED),
        Rule::token(r"(?ms)(true|false)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(eval|throw)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)`\S+`", NAME_BUILTIN),
        Rule::token(r"(?ms)[$a-zA-Z_]\w*", NAME_OTHER),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token_to(r#"(?ms)""#, STRING_DOUBLE, NewState::Push(vec![r"dqstring"])),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
    ]);
    m.insert(r"dqstring", vec![
        Rule::token(r#"(?ms)\$\{[^"}]+?\}"#, STRING_INTERPOL),
        Rule::token(r"(?ms)\$", STRING_DOUBLE),
        Rule::token(r"(?ms)\\\\", STRING_DOUBLE),
        Rule::token(r#"(?ms)\\""#, STRING_DOUBLE),
        Rule::token(r#"(?ms)[^\\"$]+"#, STRING_DOUBLE),
        Rule::token_to(r#"(?ms)""#, STRING_DOUBLE, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for ChaiscriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
