//! AUTO-GENERATED from `pygments.pygments.lexers.templates:HandlebarsLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:HandlebarsLexer:handlebars

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: handlebars
pub struct HandlebarsLexer;

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
            Rule::token(r"(?m)[^{]+", OTHER),
            Rule::token(r"(?m)\{\{!.*\}\}", COMMENT),
            Rule::bygroups_to(
                r"(?m)(\{\{\{)(\s*)",
                vec![Some(COMMENT_SPECIAL), Some(TEXT)],
                NewState::Push(vec![r"tag"]),
            ),
            Rule::bygroups_to(
                r"(?m)(\{\{)([#~/]+)([^\s}]*)",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(TokenType::new(&["Literal", "Number", "Attribute"])),
                    Some(TokenType::new(&["Literal", "Number", "Attribute"])),
                ],
                NewState::Push(vec![r"tag"]),
            ),
            Rule::bygroups_to(
                r"(?m)(\{\{)(\s*)",
                vec![Some(COMMENT_PREPROC), Some(TEXT)],
                NewState::Push(vec![r"tag"]),
            ),
        ],
    );
    m.insert(
        r"tag",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token_to(r"(?m)\}\}\}", COMMENT_SPECIAL, NewState::Pop(1)),
            Rule::bygroups_to(
                r"(?m)(~?)(\}\})",
                vec![Some(NUMBER), Some(COMMENT_PREPROC)],
                NewState::Pop(1),
            ),
            Rule::bygroups(
                r"(?m)([^\s}]+)(=)",
                vec![Some(NAME_ATTRIBUTE), Some(OPERATOR)],
            ),
            Rule::bygroups(
                r"(?m)(>)(\s*)(@partial-block)",
                vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD)],
            ),
            Rule::bygroups(
                r"(?m)(#?>)(\s*)([\w-]+)",
                vec![Some(KEYWORD), Some(TEXT), Some(NAME_VARIABLE)],
            ),
            Rule::bygroups_to(
                r"(?m)(>)(\s*)(\()",
                vec![Some(KEYWORD), Some(TEXT), Some(PUNCTUATION)],
                NewState::Push(vec![r"dynamic-partial"]),
            ),
            Rule::token(r"(?m)[()/@a-zA-Z][\w-]*", NAME_VARIABLE),
            Rule::token(r"(?m)\.[\w-]+", NAME_VARIABLE),
            Rule::token(r"(?m)(this\/|\.\/|(\.\.\/)+)[\w-]+", NAME_VARIABLE),
            Rule::token(r#"(?m):?"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?m):?'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(
                r"(?m)[0-9](\.[0-9]*)?(eE[+-][0-9])?[flFLdD]?|0[xX][0-9a-fA-F]+[Ll]?",
                NUMBER,
            ),
        ],
    );
    m.insert(
        r"generic",
        vec![
            Rule::token(r"(?m)[()/@a-zA-Z][\w-]*", NAME_VARIABLE),
            Rule::token(r"(?m)\.[\w-]+", NAME_VARIABLE),
            Rule::token(r"(?m)(this\/|\.\/|(\.\.\/)+)[\w-]+", NAME_VARIABLE),
            Rule::token(r#"(?m):?"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?m):?'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(
                r"(?m)[0-9](\.[0-9]*)?(eE[+-][0-9])?[flFLdD]?|0[xX][0-9a-fA-F]+[Ll]?",
                NUMBER,
            ),
        ],
    );
    m.insert(
        r"variable",
        vec![
            Rule::token(r"(?m)[()/@a-zA-Z][\w-]*", NAME_VARIABLE),
            Rule::token(r"(?m)\.[\w-]+", NAME_VARIABLE),
            Rule::token(r"(?m)(this\/|\.\/|(\.\.\/)+)[\w-]+", NAME_VARIABLE),
        ],
    );
    m.insert(
        r"dynamic-partial",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::bygroups(
                r"(?m)(lookup)(\s+)(\.|this)(\s+)",
                vec![Some(KEYWORD), Some(TEXT), Some(NAME_VARIABLE), Some(TEXT)],
            ),
            Rule::bygroups_g(
                r"(?m)(lookup)(\s+)(\S+)",
                vec![
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::Token(TEXT)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "variable"]),
                    }),
                ],
            ),
            Rule::token(r"(?m)[\w-]+", NAME_FUNCTION),
            Rule::token(r"(?m)[()/@a-zA-Z][\w-]*", NAME_VARIABLE),
            Rule::token(r"(?m)\.[\w-]+", NAME_VARIABLE),
            Rule::token(r"(?m)(this\/|\.\/|(\.\.\/)+)[\w-]+", NAME_VARIABLE),
            Rule::token(r#"(?m):?"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?m):?'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(
                r"(?m)[0-9](\.[0-9]*)?(eE[+-][0-9])?[flFLdD]?|0[xX][0-9a-fA-F]+[Ll]?",
                NUMBER,
            ),
        ],
    );
    Table(m)
}

impl Lexer for HandlebarsLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
