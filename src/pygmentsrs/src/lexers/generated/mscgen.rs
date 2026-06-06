//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:MscgenLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:MscgenLexer:mscgen

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mscgen, msc
pub struct MscgenLexer;

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
        Rule::token(r"(?m)msc\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(hscale|HSCALE|width|WIDTH|wordwraparcs|WORDWRAPARCS|arcgradient|ARCGRADIENT)\b", NAME_PROPERTY),
        Rule::token(r"(?m)(abox|ABOX|rbox|RBOX|box|BOX|note|NOTE)\b", OPERATOR_WORD),
        Rule::token(r"(?m)(\.|-|\|){3}", KEYWORD),
        Rule::token(r"(?m)(?:-|=|\.|:){2}|<<=>>|<->|<=>|<<>>|<:>|->|=>>|>>|=>|:>|-x|-X|<-|<<=|<<|<=|<:|x-|X-|=", OPERATOR),
        Rule::token(r"(?m)\*", NAME_BUILTIN),
        Rule::token(r#"(?m)(\w+|"(?:\\"|[^"])*")"#, NAME_VARIABLE),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"attrs"])),
        Rule::token(r"(?m)\{|\}|,|;", PUNCTUATION),
        Rule::token(r"(?m)(?://|#).*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*(?:.|\n)*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)[ \t\r\n]+", WHITESPACE),
    ]);
    m.insert(r"comments", vec![
        Rule::token(r"(?m)(?://|#).*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*(?:.|\n)*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)[ \t\r\n]+", WHITESPACE),
    ]);
    m.insert(r"attrs", vec![
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::bygroups(r#"(?m)(\w+|"(?:\\"|[^"])*")(\s*)(=)(\s*)(\w+|"(?:\\"|[^"])*")"#, vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(STRING)]),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)(?://|#).*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*(?:.|\n)*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)[ \t\r\n]+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for MscgenLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
