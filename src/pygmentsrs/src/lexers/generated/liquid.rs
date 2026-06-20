//! AUTO-GENERATED from `pygments.pygments.lexers.templates:LiquidLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:LiquidLexer:liquid

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: liquid
pub struct LiquidLexer;

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
            Rule::token(r"(?m)[^{]+", TEXT),
            Rule::bygroups_to(
                r"(?m)(\{%)(\s*)",
                vec![Some(PUNCTUATION), Some(WHITESPACE)],
                NewState::Push(vec![r"tag-or-block"]),
            ),
            Rule::bygroups_g_to(
                r"(?m)(\{\{)(\s*)([^\s}]+)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "generic"]),
                    }),
                ],
                NewState::Push(vec![r"output"]),
            ),
            Rule::token(r"(?m)\{", TEXT),
        ],
    );
    m.insert(
        r"tag-or-block",
        vec![
            Rule::token_to(
                r"(?m)(if|unless|elsif|case)(?=\s+)",
                KEYWORD_RESERVED,
                NewState::Push(vec![r"condition"]),
            ),
            Rule::bygroups_to(
                r"(?m)(when)(\s+)",
                vec![Some(KEYWORD_RESERVED), Some(WHITESPACE)],
                NewState::Push(vec![r"_tmp_0"]),
            ),
            Rule::bygroups_to(
                r"(?m)(else)(\s*)(%\})",
                vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Pop(1),
            ),
            Rule::bygroups_g_to(
                r"(?m)(capture)(\s+)([^\s%]+)(\s*)(%\})",
                vec![
                    Some(GroupAction::Token(NAME_TAG)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "variable"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
                NewState::Pop(1),
            ),
            Rule::bygroups_to(
                r"(?m)(comment)(\s*)(%\})",
                vec![Some(NAME_TAG), Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Push(vec![r"comment"]),
            ),
            Rule::bygroups_to(
                r"(?m)(raw)(\s*)(%\})",
                vec![Some(NAME_TAG), Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Push(vec![r"raw"]),
            ),
            Rule::bygroups_to(
                r"(?m)(end(case|unless|if))(\s*)(%\})",
                vec![
                    Some(KEYWORD_RESERVED),
                    None,
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                ],
                NewState::Pop(1),
            ),
            Rule::bygroups_to(
                r"(?m)(end([^\s%]+))(\s*)(%\})",
                vec![Some(NAME_TAG), None, Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Pop(1),
            ),
            Rule::bygroups_g_to(
                r"(?m)(cycle)(\s+)(?:([^\s:]*)(:))?(\s*)",
                vec![
                    Some(GroupAction::Token(NAME_TAG)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "generic"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::Token(WHITESPACE)),
                ],
                NewState::Push(vec![r"variable-tag-markup"]),
            ),
            Rule::bygroups_to(
                r"(?m)([^\s%]+)(\s*)",
                vec![Some(NAME_TAG), Some(WHITESPACE)],
                NewState::Push(vec![r"tag-markup"]),
            ),
        ],
    );
    m.insert(
        r"end-of-block",
        vec![Rule::token_to(
            r"(?m)%\}",
            PUNCTUATION,
            NewState::Push(vec![r"#pop", r"#pop"]),
        )],
    );
    m.insert(r"whitespace", vec![Rule::token(r"(?m)[ \t]+", WHITESPACE)]);
    m.insert(
        r"generic",
        vec![
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)(?<=\w)\.(?=\w)", PUNCTUATION),
        ],
    );
    m.insert(
        r"keyword",
        vec![Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT)],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
        ],
    );
    m.insert(
        r"number",
        vec![
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"variable",
        vec![
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)(?<=\w)\.(?=\w)", PUNCTUATION),
        ],
    );
    m.insert(
        r"_tmp_0",
        vec![
            Rule::token_to(
                r"(?m)%\}",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"#pop"]),
            ),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)(?<=\w)\.(?=\w)", PUNCTUATION),
        ],
    );
    m.insert(
        r"output",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token_to(r"(?m)\}\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\|", PUNCTUATION, NewState::Push(vec![r"filters"])),
        ],
    );
    m.insert(
        r"filters",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token_to(
                r"(?m)\}\}",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"#pop"]),
            ),
            Rule::bygroups_to(
                r"(?m)([^\s|:]+)(:?)(\s*)",
                vec![Some(NAME_FUNCTION), Some(PUNCTUATION), Some(WHITESPACE)],
                NewState::Push(vec![r"filter-markup"]),
            ),
        ],
    );
    m.insert(
        r"filter-markup",
        vec![
            Rule::token_to(r"(?m)\|", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\}\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::bygroups(
                r"(?m)([^\s=:]+)(\s*)(=|:)",
                vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)],
            ),
            Rule::bygroups_g(
                r"(?m)(\{\{)(\s*)([^\s}])(\s*)(\}\})",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "variable"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"end-of-tag",
        vec![Rule::token_to(r"(?m)\}\}", PUNCTUATION, NewState::Pop(1))],
    );
    m.insert(
        r"default-param-markup",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::bygroups(
                r"(?m)([^\s=:]+)(\s*)(=|:)",
                vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)],
            ),
            Rule::bygroups_g(
                r"(?m)(\{\{)(\s*)([^\s}])(\s*)(\}\})",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "variable"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"param-markup",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::bygroups(
                r"(?m)([^\s=:]+)(\s*)(=|:)",
                vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)],
            ),
            Rule::bygroups_g(
                r"(?m)(\{\{)(\s*)([^\s}])(\s*)(\}\})",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "variable"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m),", PUNCTUATION),
        ],
    );
    m.insert(
        r"condition",
        vec![
            Rule::token_to(
                r"(?m)%\}",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"#pop"]),
            ),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::bygroups_g(
                r"(?m)([^\s=!><]+)(\s*)([=!><]=?)(\s*)(\S+)(\s*)(%\})",
                vec![
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "generic"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(OPERATOR)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "generic"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(r"(?m)\b!", OPERATOR),
            Rule::token(r"(?m)\bnot\b", OPERATOR_WORD),
            Rule::bygroups_g(
                r#"(?m)([\w.\'"]+)(\s+)(contains)(\s+)([\w.\'"]+)"#,
                vec![
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "generic"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(OPERATOR_WORD)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "generic"]),
                    }),
                ],
            ),
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)(?<=\w)\.(?=\w)", PUNCTUATION),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
        ],
    );
    m.insert(
        r"generic-value",
        vec![
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)(?<=\w)\.(?=\w)", PUNCTUATION),
            Rule::token_to(r"(?m)\s+", WHITESPACE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"end-at-whitespace",
        vec![Rule::token_to(r"(?m)\s+", WHITESPACE, NewState::Pop(1))],
    );
    m.insert(
        r"operator",
        vec![
            Rule::bygroups_to(
                r"(?m)(\s*)((=|!|>|<)=?)(\s*)",
                vec![Some(WHITESPACE), Some(OPERATOR), None, Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::bygroups_to(
                r"(?m)(\s*)(\bcontains\b)(\s*)",
                vec![Some(WHITESPACE), Some(OPERATOR_WORD), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(
        r"variable-param-markup",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::bygroups(
                r"(?m)([^\s=:]+)(\s*)(=|:)",
                vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)],
            ),
            Rule::bygroups_g(
                r"(?m)(\{\{)(\s*)([^\s}])(\s*)(\}\})",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "variable"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)(?<=\w)\.(?=\w)", PUNCTUATION),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"tag-markup",
        vec![
            Rule::token_to(
                r"(?m)%\}",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"#pop"]),
            ),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::bygroups(
                r"(?m)([^\s=:]+)(\s*)(=|:)",
                vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)],
            ),
            Rule::bygroups_g(
                r"(?m)(\{\{)(\s*)([^\s}])(\s*)(\}\})",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "variable"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"variable-tag-markup",
        vec![
            Rule::token_to(
                r"(?m)%\}",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"#pop"]),
            ),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::bygroups(
                r"(?m)([^\s=:]+)(\s*)(=|:)",
                vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)],
            ),
            Rule::bygroups_g(
                r"(?m)(\{\{)(\s*)([^\s}])(\s*)(\}\})",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "variable"]),
                    }),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)\b(false|true)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)(?<=\w)\.(?=\w)", PUNCTUATION),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::bygroups_to(
                r"(?m)(\{%)(\s*)(endcomment)(\s*)(%\})",
                vec![
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(NAME_TAG),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"#pop", r"#pop"]),
            ),
            Rule::token(r"(?m).", COMMENT),
        ],
    );
    m.insert(
        r"raw",
        vec![
            Rule::token(r"(?m)[^{]+", TEXT),
            Rule::bygroups_to(
                r"(?m)(\{%)(\s*)(endraw)(\s*)(%\})",
                vec![
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(NAME_TAG),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                ],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)\{", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for LiquidLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
