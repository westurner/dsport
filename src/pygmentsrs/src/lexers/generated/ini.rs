//! AUTO-GENERATED from `pygments.pygments.lexers.configs:IniLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:IniLexer:ini

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ini, cfg, dosini
pub struct IniLexer;

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
        r"root",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[;#].*", COMMENT_SINGLE),
            Rule::bygroups(
                r"(?m)(\[.*?\])([ \t]*)$",
                vec![Some(KEYWORD), Some(WHITESPACE)],
            ),
            Rule::bygroups_to(
                r#"(?m)(.*?)([ \t]*)([=:])([ \t]*)(["'])"#,
                vec![
                    Some(NAME_ATTRIBUTE),
                    Some(WHITESPACE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                    Some(STRING),
                ],
                NewState::Push(vec![r"quoted_value"]),
            ),
            Rule::bygroups_to(
                r"(?m)(.*?)([ \t]*)([=:])([ \t]*)([^;#\n]*)(\\)(\s+)",
                vec![
                    Some(NAME_ATTRIBUTE),
                    Some(WHITESPACE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                    Some(STRING),
                    Some(TEXT),
                    Some(WHITESPACE),
                ],
                NewState::Push(vec![r"value"]),
            ),
            Rule::bygroups(
                r"(?m)(.*?)([ \t]*)([=:])([ \t]*)([^ ;#\n]*(?: +[^ ;#\n]+)*)",
                vec![
                    Some(NAME_ATTRIBUTE),
                    Some(WHITESPACE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                    Some(STRING),
                ],
            ),
            Rule::token(r"(?m)(.+?)$", NAME_ATTRIBUTE),
        ],
    );
    m.insert(
        r"quoted_value",
        vec![
            Rule::bygroups_to(
                r#"(?m)([^"'\n]*)(["'])(\s*)"#,
                vec![Some(STRING), Some(STRING), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)[;#].*", COMMENT_SINGLE),
            Rule::token_to(r"(?m)$", STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"value",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?m)(\s*)(.*)(\\)([ \t]*)",
                vec![Some(WHITESPACE), Some(STRING), Some(TEXT), Some(WHITESPACE)],
            ),
            Rule::token_to(r"(?m).*$", STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for IniLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
