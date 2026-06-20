#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.markup:TiddlyWiki5Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.markup:TiddlyWiki5Lexer:tid

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{
    DispatchCodeBlockSpec, GroupAction, GroupEmit, Rule, StateTable, tokenize,
};
use crate::token::*;

/// Aliases: tid
pub struct TidLexer;

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
            Rule::bygroups(
                r"(?m)^(title)(:\s)(.+\n)",
                vec![Some(KEYWORD), Some(TEXT), Some(GENERIC_HEADING)],
            ),
            Rule::bygroups(
                r"(?m)^(!)([^!].+\n)",
                vec![Some(GENERIC_HEADING), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)^(!{2,6})(.+\n)",
                vec![Some(GENERIC_SUBHEADING), Some(TEXT)],
            ),
            Rule::bygroups_g(
                r"(?m)^(\s*)([*#>]+)(\s*)(.+\n)",
                vec![
                    Some(GroupAction::Token(TEXT)),
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::Token(TEXT)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inline"]),
                    }),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(<<<.*\n)([\w\W]*?)(^<<<.*$)",
                vec![Some(STRING), Some(TEXT), Some(STRING)],
            ),
            Rule::bygroups(r"(?m)^(\|.*?\|h)$", vec![Some(GENERIC_STRONG)]),
            Rule::bygroups(r"(?m)^(\|.*?\|[cf])$", vec![Some(GENERIC_EMPH)]),
            Rule::bygroups(r"(?m)^(\|.*?\|k)$", vec![Some(NAME_TAG)]),
            Rule::bygroups(r"(?m)^(;.*)$", vec![Some(GENERIC_STRONG)]),
            Rule::bygroups(
                r"(?m)^(```\n)([\w\W]*?)(^```$)",
                vec![Some(STRING), Some(TEXT), Some(STRING)],
            ),
            Rule::dispatch_code_block(
                r"(?m)^(```)(\w+)(\n)([\w\W]*?)(^```$)",
                DispatchCodeBlockSpec {
                    prefix: vec![
                        GroupEmit {
                            group: 1,
                            token: STRING,
                            skip_if_none: false,
                        },
                        GroupEmit {
                            group: 2,
                            token: STRING,
                            skip_if_none: false,
                        },
                        GroupEmit {
                            group: 3,
                            token: TEXT,
                            skip_if_none: false,
                        },
                    ],
                    lang_group: 2,
                    code_groups: vec![4],
                    suffix: vec![GroupEmit {
                        group: 5,
                        token: STRING,
                        skip_if_none: false,
                    }],
                    fallback_token: STRING,
                    strip_indent_from_group: None,
                },
            ),
            Rule::using_lexer(r"(?m)^(<style>)(\n)([\w\W]*?)(^</style>$)", "css", None),
            Rule::token(
                r"(?m)^(\\(?:define|end)|c(?:aption|reated)|modified|t(?:ags|(?:itl|yp)e))\b",
                KEYWORD,
            ),
            Rule::token(r"(?m)\\.", TEXT),
            Rule::token(r"(?m)\d{17}", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)(\s)(//[^/]+//)((?=\W|\n))",
                vec![Some(TEXT), Some(GENERIC_EMPH), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(\s)(\^\^[^\^]+\^\^)",
                vec![Some(TEXT), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups(r"(?m)(\s)(,,[^,]+,,)", vec![Some(TEXT), Some(GENERIC_EMPH)]),
            Rule::bygroups(
                r"(?m)(\s)(__[^_]+__)",
                vec![Some(TEXT), Some(GENERIC_STRONG)],
            ),
            Rule::bygroups(
                r"(?m)(\s)(''[^']+'')((?=\W|\n))",
                vec![Some(TEXT), Some(GENERIC_STRONG), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(\s)(~~[^~]+~~)((?=\W|\n))",
                vec![Some(TEXT), Some(GENERIC_DELETED), Some(TEXT)],
            ),
            Rule::token(r"(?m)<<[^>]+>>", NAME_TAG),
            Rule::token(r"(?m)\$\$[^$]+\$\$", NAME_TAG),
            Rule::token(r"(?m)\$\([^)]+\)\$", NAME_TAG),
            Rule::token(r"(?m)^@@.*$", NAME_TAG),
            Rule::token(r"(?m)</?[^>]+>", NAME_TAG),
            Rule::token(r"(?m)`[^`]+`", STRING_BACKTICK),
            Rule::token(r"(?m)&\S*?;", STRING_REGEX),
            Rule::bygroups(
                r"(?m)(\[{2})([^\]\|]+)(\]{2})",
                vec![Some(TEXT), Some(NAME_TAG), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(\[{2})([^\]\|]+)(\|)([^\]\|]+)(\]{2})",
                vec![
                    Some(TEXT),
                    Some(NAME_TAG),
                    Some(TEXT),
                    Some(NAME_ATTRIBUTE),
                    Some(TEXT),
                ],
            ),
            Rule::bygroups(
                r"(?m)(\{{2})([^}]+)(\}{2})",
                vec![Some(TEXT), Some(NAME_TAG), Some(TEXT)],
            ),
            Rule::bygroups(r#"(?m)(\b.?.?tps?://[^\s"]+)"#, vec![Some(NAME_ATTRIBUTE)]),
            Rule::token(r"(?m)[\w]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"keywords",
        vec![Rule::token(
            r"(?m)^(\\(?:define|end)|c(?:aption|reated)|modified|t(?:ags|(?:itl|yp)e))\b",
            KEYWORD,
        )],
    );
    m.insert(
        r"inline",
        vec![
            Rule::token(r"(?m)\\.", TEXT),
            Rule::token(r"(?m)\d{17}", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)(\s)(//[^/]+//)((?=\W|\n))",
                vec![Some(TEXT), Some(GENERIC_EMPH), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(\s)(\^\^[^\^]+\^\^)",
                vec![Some(TEXT), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups(r"(?m)(\s)(,,[^,]+,,)", vec![Some(TEXT), Some(GENERIC_EMPH)]),
            Rule::bygroups(
                r"(?m)(\s)(__[^_]+__)",
                vec![Some(TEXT), Some(GENERIC_STRONG)],
            ),
            Rule::bygroups(
                r"(?m)(\s)(''[^']+'')((?=\W|\n))",
                vec![Some(TEXT), Some(GENERIC_STRONG), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(\s)(~~[^~]+~~)((?=\W|\n))",
                vec![Some(TEXT), Some(GENERIC_DELETED), Some(TEXT)],
            ),
            Rule::token(r"(?m)<<[^>]+>>", NAME_TAG),
            Rule::token(r"(?m)\$\$[^$]+\$\$", NAME_TAG),
            Rule::token(r"(?m)\$\([^)]+\)\$", NAME_TAG),
            Rule::token(r"(?m)^@@.*$", NAME_TAG),
            Rule::token(r"(?m)</?[^>]+>", NAME_TAG),
            Rule::token(r"(?m)`[^`]+`", STRING_BACKTICK),
            Rule::token(r"(?m)&\S*?;", STRING_REGEX),
            Rule::bygroups(
                r"(?m)(\[{2})([^\]\|]+)(\]{2})",
                vec![Some(TEXT), Some(NAME_TAG), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(\[{2})([^\]\|]+)(\|)([^\]\|]+)(\]{2})",
                vec![
                    Some(TEXT),
                    Some(NAME_TAG),
                    Some(TEXT),
                    Some(NAME_ATTRIBUTE),
                    Some(TEXT),
                ],
            ),
            Rule::bygroups(
                r"(?m)(\{{2})([^}]+)(\}{2})",
                vec![Some(TEXT), Some(NAME_TAG), Some(TEXT)],
            ),
            Rule::bygroups(r#"(?m)(\b.?.?tps?://[^\s"]+)"#, vec![Some(NAME_ATTRIBUTE)]),
            Rule::token(r"(?m)[\w]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for TidLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
