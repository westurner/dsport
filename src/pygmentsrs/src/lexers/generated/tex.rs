//! AUTO-GENERATED from `pygments.pygments.lexers.markup:TexLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.markup:TexLexer:tex

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tex, latex
pub struct TexLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"general", vec![
        Rule::token(r"(?m)%.*?\n", COMMENT),
        Rule::token(r"(?m)[{}]", NAME_BUILTIN),
        Rule::token(r"(?m)[&_^]", NAME_BUILTIN),
    ]);
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)\\\[", STRING_BACKTICK, NewState::Push(vec![r"displaymath"])),
        Rule::token_to(r"(?m)\\\(", STRING, NewState::Push(vec![r"inlinemath"])),
        Rule::token_to(r"(?m)\$\$", STRING_BACKTICK, NewState::Push(vec![r"displaymath"])),
        Rule::token_to(r"(?m)\$", STRING, NewState::Push(vec![r"inlinemath"])),
        Rule::token_to(r"(?m)\\([a-zA-Z@_:]+|\S?)", KEYWORD, NewState::Push(vec![r"command"])),
        Rule::token(r"(?m)\\$", KEYWORD),
        Rule::token(r"(?m)%.*?\n", COMMENT),
        Rule::token(r"(?m)[{}]", NAME_BUILTIN),
        Rule::token(r"(?m)[&_^]", NAME_BUILTIN),
        Rule::token(r"(?m)[^\\$%&_^{}]+", TEXT),
    ]);
    m.insert(r"math", vec![
        Rule::token(r"(?m)\\([a-zA-Z]+|\S?)", NAME_VARIABLE),
        Rule::token(r"(?m)%.*?\n", COMMENT),
        Rule::token(r"(?m)[{}]", NAME_BUILTIN),
        Rule::token(r"(?m)[&_^]", NAME_BUILTIN),
        Rule::token(r"(?m)[0-9]+", NUMBER),
        Rule::token(r"(?m)[-=!+*/()\[\]]", OPERATOR),
        Rule::token(r"(?m)[^=!+*/()\[\]\\$%&_^{}0-9-]+", NAME_BUILTIN),
    ]);
    m.insert(r"inlinemath", vec![
        Rule::token_to(r"(?m)\\\)", STRING, NewState::Pop(1)),
        Rule::token_to(r"(?m)\$", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\([a-zA-Z]+|\S?)", NAME_VARIABLE),
        Rule::token(r"(?m)%.*?\n", COMMENT),
        Rule::token(r"(?m)[{}]", NAME_BUILTIN),
        Rule::token(r"(?m)[&_^]", NAME_BUILTIN),
        Rule::token(r"(?m)[0-9]+", NUMBER),
        Rule::token(r"(?m)[-=!+*/()\[\]]", OPERATOR),
        Rule::token(r"(?m)[^=!+*/()\[\]\\$%&_^{}0-9-]+", NAME_BUILTIN),
    ]);
    m.insert(r"displaymath", vec![
        Rule::token_to(r"(?m)\\\]", STRING, NewState::Pop(1)),
        Rule::token_to(r"(?m)\$\$", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\$", NAME_BUILTIN),
        Rule::token(r"(?m)\\([a-zA-Z]+|\S?)", NAME_VARIABLE),
        Rule::token(r"(?m)%.*?\n", COMMENT),
        Rule::token(r"(?m)[{}]", NAME_BUILTIN),
        Rule::token(r"(?m)[&_^]", NAME_BUILTIN),
        Rule::token(r"(?m)[0-9]+", NUMBER),
        Rule::token(r"(?m)[-=!+*/()\[\]]", OPERATOR),
        Rule::token(r"(?m)[^=!+*/()\[\]\\$%&_^{}0-9-]+", NAME_BUILTIN),
    ]);
    m.insert(r"command", vec![
        Rule::token(r"(?m)\[.*?\]", NAME_ATTRIBUTE),
        Rule::token(r"(?m)\*", KEYWORD),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for TexLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
