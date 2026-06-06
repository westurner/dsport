//! AUTO-GENERATED from `pygments.pygments.lexers.minecraft:SNBTLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.minecraft:SNBTLexer:snbt

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: snbt
pub struct SnbtLexer;

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
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"compound"])),
        Rule::token(r"(?m)[^\{]+", TEXT),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"operators", vec![
        Rule::token(r"(?m)[,:;]", PUNCTUATION),
    ]);
    m.insert(r"literals", vec![
        Rule::token(r"(?m)(true|false)", KEYWORD_CONSTANT),
        Rule::token(r"(?m)-?\d+[eE]-?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d*\.\d+[fFdD]?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+[bBsSlLfFdD]?", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"literals.string_double"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"literals.string_single"])),
    ]);
    m.insert(r"literals.string_double", vec![
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
    ]);
    m.insert(r"literals.string_single", vec![
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token(r"(?m)[^\\'\n]+", STRING_SINGLE),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
    ]);
    m.insert(r"compound", vec![
        Rule::token(r"(?m)[A-Z_a-z]+", NAME_ATTRIBUTE),
        Rule::token(r"(?m)[,:;]", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)(true|false)", KEYWORD_CONSTANT),
        Rule::token(r"(?m)-?\d+[eE]-?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d*\.\d+[fFdD]?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+[bBsSlLfFdD]?", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"literals.string_double"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"literals.string_single"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"list"])),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"list", vec![
        Rule::token(r"(?m)[A-Z_a-z]+", NAME_ATTRIBUTE),
        Rule::token(r"(?m)(true|false)", KEYWORD_CONSTANT),
        Rule::token(r"(?m)-?\d+[eE]-?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d*\.\d+[fFdD]?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+[bBsSlLfFdD]?", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"literals.string_double"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"literals.string_single"])),
        Rule::token(r"(?m)[,:;]", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"compound"])),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for SnbtLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
