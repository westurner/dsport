//! AUTO-GENERATED from `pygments.pygments.lexers.minecraft:MCSchemaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.minecraft:MCSchemaLexer:mcschema

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mcschema
pub struct McschemaLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*.*?\*/", COMMENT_MULTILINE),
    ]);
    m.insert(r"slashstartsregex", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gimuysd]+\b|\B)", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?m)(?=/)", TEXT, NewState::Push(vec![r"#pop", r"badregex"])),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"badregex", vec![
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"singlestring", vec![
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)[^\\']+", STRING_SINGLE),
    ]);
    m.insert(r"doublestring", vec![
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\"]+"#, STRING_DOUBLE),
    ]);
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)^(?=\s|/|<!--)", TEXT, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(?<=: )opt", OPERATOR_WORD),
        Rule::token(r#"(?m)(?<=\s)[\w-]*(?=(\s+"|\n))"#, KEYWORD_DECLARATION),
        Rule::token(r"(?m)0[bB][01]+", NUMBER_BIN),
        Rule::token(r"(?m)0[oO]?[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)(\.\d+|\d+\.\d*|\d+)([eE][-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\.\.\.|=>", PUNCTUATION),
        Rule::token_to(r"(?m)\+\+|--|~|\?\?=?|\?|:|\\(?=\n)|(<<|>>>?|==?|!=?|(?:\*\*|\|\||&&|[-<>+*%&|^/]))=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?m)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?m)[})\].]", PUNCTUATION),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)[\w-]*?(?=:\{?\n)", STRING_SYMBOL),
        Rule::bygroups(r"(?m)([\w-]*?)(:)(\d+)(?:(\.)(\d+)(?:(\.)(\d+)(?:(\-)((?:[^\W_]|-)*(?:\.(?:[^\W_]|-)*)*))?(?:(\+)((?:[^\W_]|-)+(?:\.(?:[^\W_]|-)+)*))?)?)?(?=:\{?\n)", vec![Some(STRING_SYMBOL), Some(OPERATOR), Some(NUMBER_INTEGER), Some(OPERATOR), Some(NUMBER_INTEGER), Some(OPERATOR), Some(NUMBER_INTEGER), Some(OPERATOR), Some(STRING), Some(OPERATOR), Some(STRING)]),
        Rule::token(r"(?m).*\n", TEXT),
    ]);
    Table(m)
}

impl Lexer for McschemaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
