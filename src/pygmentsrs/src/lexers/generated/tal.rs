//! AUTO-GENERATED from `pygments.pygments.lexers.tal:TalLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.tal:TalLexer:tal

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tal, uxntal
pub struct TalLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"comment",
        vec![
            Rule::token_to(
                r"(?m)(?<!\S)\((?!\S)",
                COMMENT_MULTILINE,
                NewState::PushSame,
            ),
            Rule::token_to(r"(?m)(?<!\S)\)(?!\S)", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[^()]+", COMMENT_MULTILINE),
            Rule::token(r"(?m)[()]+", COMMENT_MULTILINE),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)(?<!\S)\((?!\S)", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)(?<!\S)(A(?:(?:[DN])D)|BRK|D(?:E(?:[IO])|IV|UP)|E(?:OR|QU)|GTH|INC|J(?:CN|MP|SR)|L(?:D(?:[ARZ])|IT|TH)|MUL|N(?:EQ|IP)|O(?:RA|VR)|POP|ROT|S(?:FT|T(?:[AHRZ])|UB|WP))2?k?r?(?!\S)", KEYWORD_RESERVED),
        Rule::token(r"(?m)[\]\[{}](?!\S)", PUNCTUATION),
        Rule::token(r"(?m)#([0-9a-f]{2}){1,2}(?!\S)", NUMBER_HEX),
        Rule::token(r#"(?m)"\S+"#, STRING),
        Rule::token(r"(?m)([0-9a-f]{2}){1,2}(?!\S)", LITERAL),
        Rule::token(r"(?m)[|$][0-9a-f]{1,4}(?!\S)", KEYWORD_DECLARATION),
        Rule::token(r"(?m)%\S+", NAME_DECORATOR),
        Rule::token(r"(?m)@\S+", NAME_FUNCTION),
        Rule::token(r"(?m)&\S+", NAME_LABEL),
        Rule::token(r"(?m)/\S+", NAME_TAG),
        Rule::token(r"(?m)\.\S+", NAME_VARIABLE_MAGIC),
        Rule::token(r"(?m),\S+", NAME_VARIABLE_INSTANCE),
        Rule::token(r"(?m);\S+", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)-\S+", LITERAL),
        Rule::token(r"(?m)_\S+", LITERAL),
        Rule::token(r"(?m)=\S+", LITERAL),
        Rule::token(r"(?m)!\S+", NAME_FUNCTION),
        Rule::token(r"(?m)\?\S+", NAME_FUNCTION),
        Rule::token(r"(?m)~\S+", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)\S+", NAME_FUNCTION),
    ]);
    Table(m)
}

impl Lexer for TalLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
