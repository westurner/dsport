//! AUTO-GENERATED from `pygments.pygments.lexers.markup:MarkdownLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.markup:MarkdownLexer:markdown

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{
    DispatchCodeBlockSpec, GroupAction, GroupEmit, Rule, StateTable, tokenize,
};
use crate::token::*;

/// Aliases: markdown, md
pub struct MarkdownLexer;

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
                r"(?m)(^#[^#].+)(\n)",
                vec![Some(GENERIC_HEADING), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(^#{2,6}[^#].+)(\n)",
                vec![Some(GENERIC_SUBHEADING), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)^(.+)(\n)(=+)(\n)",
                vec![
                    Some(GENERIC_HEADING),
                    Some(TEXT),
                    Some(GENERIC_HEADING),
                    Some(TEXT),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(.+)(\n)(-+)(\n)",
                vec![
                    Some(GENERIC_SUBHEADING),
                    Some(TEXT),
                    Some(GENERIC_SUBHEADING),
                    Some(TEXT),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)^(\s*)([*-] )(\[[ xX]\])( .+\n)",
                vec![
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inline"]),
                    }),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)^(\s*)([*-])(\s)(.+\n)",
                vec![
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inline"]),
                    }),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)^(\s*)([0-9]+\.)( .+\n)",
                vec![
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inline"]),
                    }),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\s*>\s)(.+\n)",
                vec![Some(KEYWORD), Some(GENERIC_EMPH)],
            ),
            Rule::token(r"(?m)^(\s*```\n[\w\W]*?^\s*```$\n)", STRING_BACKTICK),
            Rule::dispatch_code_block(
                r"(?m)(?x)
              ^(?P<initial>\s*```)
              (?P<lang>[\w\-]+)
              (?P<afterlang>
                 (?P<whitespace>[^\S\n]+)
                 (?P<extra>.*))?
              (?P<newline>\n)
              (?P<code>(.|\n)*?)
              (?P<terminator>^\s*```$\n)
              ",
                DispatchCodeBlockSpec {
                    prefix: vec![
                        GroupEmit {
                            group: 1,
                            token: TokenType::new(&["Literal", "String", "Backtick"]),
                            skip_if_none: false,
                        },
                        GroupEmit {
                            group: 2,
                            token: TokenType::new(&["Literal", "String", "Backtick"]),
                            skip_if_none: false,
                        },
                        GroupEmit {
                            group: 4,
                            token: WHITESPACE,
                            skip_if_none: true,
                        },
                        GroupEmit {
                            group: 5,
                            token: TEXT,
                            skip_if_none: true,
                        },
                        GroupEmit {
                            group: 6,
                            token: WHITESPACE,
                            skip_if_none: false,
                        },
                    ],
                    lang_group: 2,
                    code_groups: vec![7],
                    suffix: vec![GroupEmit {
                        group: 9,
                        token: TokenType::new(&["Literal", "String", "Backtick"]),
                        skip_if_none: false,
                    }],
                    fallback_token: STRING,
                    strip_indent_from_group: None,
                },
            ),
            Rule::token(r"(?m)\\.", TEXT),
            Rule::bygroups(
                r"(?m)([^`]?)(`[^`\n]+`)",
                vec![Some(TEXT), Some(STRING_BACKTICK)],
            ),
            Rule::bygroups(
                r"(?m)([^\*]?)(\*\*[^* \n][^*\n]*\*\*)",
                vec![Some(TEXT), Some(GENERIC_STRONG)],
            ),
            Rule::bygroups(
                r"(?m)([^_]?)(__[^_ \n][^_\n]*__)",
                vec![Some(TEXT), Some(GENERIC_STRONG)],
            ),
            Rule::bygroups(
                r"(?m)([^\*]?)(\*[^* \n][^*\n]*\*)",
                vec![Some(TEXT), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups(
                r"(?m)([^_]?)(_[^_ \n][^_\n]*_)",
                vec![Some(TEXT), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups(
                r"(?m)([^~]?)(~~[^~ \n][^~\n]*~~)",
                vec![Some(TEXT), Some(GENERIC_DELETED)],
            ),
            Rule::token(r"(?m)[@#][\w/:]+", NAME_ENTITY),
            Rule::bygroups(
                r"(?m)(!?\[)([^\]]+)(\])(\()([^)]+)(\))",
                vec![
                    Some(TEXT),
                    Some(NAME_TAG),
                    Some(TEXT),
                    Some(TEXT),
                    Some(NAME_ATTRIBUTE),
                    Some(TEXT),
                ],
            ),
            Rule::bygroups(
                r"(?m)(\[)([^\]]+)(\])(\[)([^\]]*)(\])",
                vec![
                    Some(TEXT),
                    Some(NAME_TAG),
                    Some(TEXT),
                    Some(TEXT),
                    Some(NAME_LABEL),
                    Some(TEXT),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\s*\[)([^\]]*)(\]:\s*)(.+)",
                vec![
                    Some(TEXT),
                    Some(NAME_LABEL),
                    Some(TEXT),
                    Some(NAME_ATTRIBUTE),
                ],
            ),
            Rule::token(r"(?m)[^\\\s]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"inline",
        vec![
            Rule::token(r"(?m)\\.", TEXT),
            Rule::bygroups(
                r"(?m)([^`]?)(`[^`\n]+`)",
                vec![Some(TEXT), Some(STRING_BACKTICK)],
            ),
            Rule::bygroups(
                r"(?m)([^\*]?)(\*\*[^* \n][^*\n]*\*\*)",
                vec![Some(TEXT), Some(GENERIC_STRONG)],
            ),
            Rule::bygroups(
                r"(?m)([^_]?)(__[^_ \n][^_\n]*__)",
                vec![Some(TEXT), Some(GENERIC_STRONG)],
            ),
            Rule::bygroups(
                r"(?m)([^\*]?)(\*[^* \n][^*\n]*\*)",
                vec![Some(TEXT), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups(
                r"(?m)([^_]?)(_[^_ \n][^_\n]*_)",
                vec![Some(TEXT), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups(
                r"(?m)([^~]?)(~~[^~ \n][^~\n]*~~)",
                vec![Some(TEXT), Some(GENERIC_DELETED)],
            ),
            Rule::token(r"(?m)[@#][\w/:]+", NAME_ENTITY),
            Rule::bygroups(
                r"(?m)(!?\[)([^\]]+)(\])(\()([^)]+)(\))",
                vec![
                    Some(TEXT),
                    Some(NAME_TAG),
                    Some(TEXT),
                    Some(TEXT),
                    Some(NAME_ATTRIBUTE),
                    Some(TEXT),
                ],
            ),
            Rule::bygroups(
                r"(?m)(\[)([^\]]+)(\])(\[)([^\]]*)(\])",
                vec![
                    Some(TEXT),
                    Some(NAME_TAG),
                    Some(TEXT),
                    Some(TEXT),
                    Some(NAME_LABEL),
                    Some(TEXT),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\s*\[)([^\]]*)(\]:\s*)(.+)",
                vec![
                    Some(TEXT),
                    Some(NAME_LABEL),
                    Some(TEXT),
                    Some(NAME_ATTRIBUTE),
                ],
            ),
            Rule::token(r"(?m)[^\\\s]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for MarkdownLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
