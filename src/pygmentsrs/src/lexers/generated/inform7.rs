#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.int_fiction:Inform7Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.int_fiction:Inform7Lexer:inform7

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: inform7, i7
pub struct Inform7Lexer;

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
        r"+i6-root",
        vec![
            Rule::token_to(
                r"(?ms)\A(!%[^\n  ]*[\n  ])+",
                COMMENT_PREPROC,
                NewState::Push(vec![r"directive"]),
            ),
            Rule::default(NewState::Push(vec![r"directive"])),
        ],
    );
    m.insert(
        r"+i6t-root",
        vec![Rule::token_to(
            r"(?ms)[^\n  ]*",
            COMMENT_PREPROC,
            NewState::Push(vec![r"directive", r"+p"]),
        )],
    );
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?ms)(\|?\s)+", TEXT),
            Rule::token_to(
                r"(?ms)\[",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"+comment"]),
            ),
            Rule::token_to(
                r#"(?ms)["“”]"#,
                GENERIC_HEADING,
                NewState::Push(vec![r"+main", r"+titling", r"+titling-string"]),
            ),
            Rule::default(NewState::Push(vec![r"+main", r"+heading?"])),
        ],
    );
    m.insert(
        r"+titling-string",
        vec![
            Rule::token(r#"(?ms)[^"“”]+"#, GENERIC_HEADING),
            Rule::token_to(r#"(?ms)["“”]"#, GENERIC_HEADING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"+titling",
        vec![
            Rule::token_to(
                r"(?ms)\[",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"+comment"]),
            ),
            Rule::token(r#"(?ms)[^"“”.;:|\n  ]+"#, GENERIC_HEADING),
            Rule::token_to(
                r#"(?ms)["“”]"#,
                GENERIC_HEADING,
                NewState::Push(vec![r"+titling-string"]),
            ),
            Rule::token_to(
                r#"(?ms)[\n  ]{2}|(?<=[\s"“”])\|[\s"“”]"#,
                TEXT,
                NewState::Push(vec![r"#pop", r"+heading?"]),
            ),
            Rule::token_to(r#"(?ms)[.;:]|(?<=[\s"“”])\|"#, TEXT, NewState::Pop(1)),
            Rule::token(r"(?ms)[|\n  ]", GENERIC_HEADING),
        ],
    );
    m.insert(
        r"+main",
        vec![
            Rule::token(r#"(?ms)(?i)[^"“”:a\[(|\n  ]+"#, TEXT),
            Rule::token_to(
                r#"(?ms)["“”]"#,
                STRING_DOUBLE,
                NewState::Push(vec![r"+text"]),
            ),
            Rule::token_to(r"(?ms):", TEXT, NewState::Push(vec![r"+phrase-definition"])),
            Rule::token_to(
                r"(?ms)(?i)\bas\b",
                TEXT,
                NewState::Push(vec![r"+use-option"]),
            ),
            Rule::token_to(
                r"(?ms)\[",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"+comment"]),
            ),
            Rule::bygroups_g(
                r"(?ms)(\([\-‐-—])(.*?)([\-‐-—]\))",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["+i6-root", "directive"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token_to(
                r#"(?ms)(\A|(?<=[\n  ])|(?<=[\s;:."“”]))\|\s|[\n  ]{2,}"#,
                TEXT,
                NewState::Push(vec![r"+heading?"]),
            ),
            Rule::token(r"(?ms)(?i)[a(|\n  ]", TEXT),
        ],
    );
    m.insert(
        r"+phrase-definition",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token_to(
                r"(?ms)\[",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"+comment"]),
            ),
            Rule::bygroups_g_to(
                r"(?ms)(\([\-‐-—])(.*?)([\-‐-—]\))",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["+i6-root", "directive", "default", "statements"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
                NewState::Pop(1),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"+use-option",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token_to(
                r"(?ms)\[",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"+comment"]),
            ),
            Rule::bygroups_g_to(
                r"(?ms)(\([\-‐-—])(.*?)([\-‐-—]\))",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["+i6-root", "directive"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
                NewState::Pop(1),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"+comment",
        vec![
            Rule::token(r"(?ms)[^\[\]]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?ms)\[", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?ms)\]", COMMENT_MULTILINE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"+text",
        vec![
            Rule::token(r#"(?ms)[^\["“”]+"#, STRING_DOUBLE),
            Rule::token(r"(?ms)\[.*?\]", STRING_INTERPOL),
            Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"+heading?",
        vec![
            Rule::token(r"(?ms)(\|?\s)+", TEXT),
            Rule::token_to(
                r"(?ms)\[",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"+comment"]),
            ),
            Rule::token_to(
                r"(?ms)[\-‐-—]{4}\s+",
                TEXT,
                NewState::Push(vec![r"+documentation-heading"]),
            ),
            Rule::token(r"(?ms)[\-‐-—]{1,3}", TEXT),
            Rule::token_to(
                r"(?ms)(?i)(volume|book|part|chapter|section)\b[^\n  ]*",
                GENERIC_HEADING,
                NewState::Pop(1),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"+documentation-heading",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token_to(
                r"(?ms)\[",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"+comment"]),
            ),
            Rule::token_to(
                r"(?ms)(?i)documentation\s+",
                TEXT,
                NewState::Push(vec![r"+documentation-heading2"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"+documentation-heading2",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token_to(
                r"(?ms)\[",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"+comment"]),
            ),
            Rule::token_to(
                r"(?ms)[\-‐-—]{4}\s",
                TEXT,
                NewState::Push(vec![r"+documentation"]),
            ),
            Rule::default(NewState::Pop(2)),
        ],
    );
    m.insert(
        r"+documentation",
        vec![
            Rule::token(
                r"(?ms)(?i)(\A|(?<=[\n  ]))\s*(chapter|example)\s*:[^\n  ]*",
                GENERIC_HEADING,
            ),
            Rule::token(
                r"(?ms)(?i)(\A|(?<=[\n  ]))\s*section\s*:[^\n  ]*",
                GENERIC_SUBHEADING,
            ),
            Rule::using_this(
                r"(?ms)((\A|(?<=[\n  ]))\t.*?[\n  ])+",
                Some(vec!["root", "+main"]),
            ),
            Rule::token(r"(?ms)[^\n  \[]+|[\n  \[]", TEXT),
            Rule::token_to(
                r"(?ms)\[",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"+comment"]),
            ),
        ],
    );
    m.insert(
        r"+i6t-not-inline",
        vec![
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
        ],
    );
    m.insert(
        r"+i6t-use-option",
        vec![
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{)(N)(\})",
                vec![Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)],
            ),
        ],
    );
    m.insert(
        r"+i6t-inline",
        vec![Rule::bygroups_g(
            r"(?ms)(\{)(\S[^}]*)?(\})",
            vec![
                Some(GroupAction::Token(PUNCTUATION)),
                Some(GroupAction::UsingThis {
                    state: Some(vec!["root", "+main"]),
                }),
                Some(GroupAction::Token(PUNCTUATION)),
            ],
        )],
    );
    m.insert(
        r"+i6t",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
        ],
    );
    m.insert(
        r"+p",
        vec![
            Rule::token(r"(?ms)[^@]+", COMMENT_PREPROC),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
                NewState::Pop(1),
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]|Purpose:)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
            ),
            Rule::token(r"(?ms)@", COMMENT_PREPROC),
        ],
    );
    m.insert(
        r"+lines",
        vec![
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)(\A|(?<=[\n  ]))@\w*[ \n  ]", KEYWORD),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::bygroups_to(
                r"(?ms)(\{)([\-‐-—]endlines)(\})",
                vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?ms)[^@!{]+?([\n  ]|\Z)|.", TEXT),
        ],
    );
    m.insert(
        r"_whitespace",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"default",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)\[",
                PUNCTUATION,
                NewState::Push(vec![r"many-values"]),
            ),
            Rule::token_to(r"(?ms):|(?=;)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?ms)<", PUNCTUATION),
            Rule::default(NewState::Push(vec![r"expression", r"_expression"])),
        ],
    );
    m.insert(
        r"_expression",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms)(?=sp\b)", TEXT, NewState::Pop(1)),
            Rule::token_to(
                r#"(?ms)(?=["“”'‘’$0-9#a-zA-Z_])"#,
                TEXT,
                NewState::Push(vec![r"#pop", r"value"]),
            ),
            Rule::token(r"(?ms)\+\+|[\-‐-—]{1,2}(?!>)|~~?", OPERATOR),
            Rule::token_to(r"(?ms)(?=[()\[\-‐-—,?@{:;])", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"expression",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)\(",
                PUNCTUATION,
                NewState::Push(vec![r"expression", r"_expression"]),
            ),
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r"(?ms)\[",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"statements", r"locals"]),
            ),
            Rule::token(r"(?ms)>(?=(\s+|(![^\n  ]*))*[>;])", PUNCTUATION),
            Rule::token(r"(?ms)\+\+|[\-‐-—]{2}(?!>)", OPERATOR),
            Rule::token_to(r"(?ms),", PUNCTUATION, NewState::Push(vec![r"_expression"])),
            Rule::token_to(
                r"(?ms)&&?|\|\|?|[=~><]?=|[\-‐-—]{1,2}>?|\.\.?[&#]?|::|[<>+*/%]",
                OPERATOR,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token_to(
                r"(?ms)(has|hasnt|in|notin|ofclass|or|provides)\b",
                OPERATOR_WORD,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token(r"(?ms)sp\b", NAME),
            Rule::token_to(r"(?ms)\?~?", NAME_LABEL, NewState::Push(vec![r"label?"])),
            Rule::token(r"(?ms)[@{]", ERROR),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"_assembly-expression",
        vec![
            Rule::token_to(
                r"(?ms)\(",
                PUNCTUATION,
                NewState::Push(vec![r"#push", r"_expression"]),
            ),
            Rule::token(r"(?ms)[\[\]]", PUNCTUATION),
            Rule::token_to(
                r"(?ms)[\-‐-—]>",
                PUNCTUATION,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token(r"(?ms)sp\b", KEYWORD_PSEUDO),
            Rule::token_to(r"(?ms);", PUNCTUATION, NewState::Pop(3)),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)\(",
                PUNCTUATION,
                NewState::Push(vec![r"expression", r"_expression"]),
            ),
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r"(?ms)\[",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"statements", r"locals"]),
            ),
            Rule::token(r"(?ms)>(?=(\s+|(![^\n  ]*))*[>;])", PUNCTUATION),
            Rule::token(r"(?ms)\+\+|[\-‐-—]{2}(?!>)", OPERATOR),
            Rule::token_to(r"(?ms),", PUNCTUATION, NewState::Push(vec![r"_expression"])),
            Rule::token_to(
                r"(?ms)&&?|\|\|?|[=~><]?=|[\-‐-—]{1,2}>?|\.\.?[&#]?|::|[<>+*/%]",
                OPERATOR,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token_to(
                r"(?ms)(has|hasnt|in|notin|ofclass|or|provides)\b",
                OPERATOR_WORD,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token(r"(?ms)sp\b", NAME),
            Rule::token_to(r"(?ms)\?~?", NAME_LABEL, NewState::Push(vec![r"label?"])),
            Rule::token(r"(?ms)[@{]", ERROR),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"_for-expression",
        vec![
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(2)),
            Rule::token_to(r"(?ms):", PUNCTUATION, NewState::Pop(1)),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)\(",
                PUNCTUATION,
                NewState::Push(vec![r"expression", r"_expression"]),
            ),
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r"(?ms)\[",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"statements", r"locals"]),
            ),
            Rule::token(r"(?ms)>(?=(\s+|(![^\n  ]*))*[>;])", PUNCTUATION),
            Rule::token(r"(?ms)\+\+|[\-‐-—]{2}(?!>)", OPERATOR),
            Rule::token_to(r"(?ms),", PUNCTUATION, NewState::Push(vec![r"_expression"])),
            Rule::token_to(
                r"(?ms)&&?|\|\|?|[=~><]?=|[\-‐-—]{1,2}>?|\.\.?[&#]?|::|[<>+*/%]",
                OPERATOR,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token_to(
                r"(?ms)(has|hasnt|in|notin|ofclass|or|provides)\b",
                OPERATOR_WORD,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token(r"(?ms)sp\b", NAME),
            Rule::token_to(r"(?ms)\?~?", NAME_LABEL, NewState::Push(vec![r"label?"])),
            Rule::token(r"(?ms)[@{]", ERROR),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"_keyword-expression",
        vec![
            Rule::token_to(
                r"(?ms)(from|near|to)\b",
                KEYWORD,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)\(",
                PUNCTUATION,
                NewState::Push(vec![r"expression", r"_expression"]),
            ),
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r"(?ms)\[",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"statements", r"locals"]),
            ),
            Rule::token(r"(?ms)>(?=(\s+|(![^\n  ]*))*[>;])", PUNCTUATION),
            Rule::token(r"(?ms)\+\+|[\-‐-—]{2}(?!>)", OPERATOR),
            Rule::token_to(r"(?ms),", PUNCTUATION, NewState::Push(vec![r"_expression"])),
            Rule::token_to(
                r"(?ms)&&?|\|\|?|[=~><]?=|[\-‐-—]{1,2}>?|\.\.?[&#]?|::|[<>+*/%]",
                OPERATOR,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token_to(
                r"(?ms)(has|hasnt|in|notin|ofclass|or|provides)\b",
                OPERATOR_WORD,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token(r"(?ms)sp\b", NAME),
            Rule::token_to(r"(?ms)\?~?", NAME_LABEL, NewState::Push(vec![r"label?"])),
            Rule::token(r"(?ms)[@{]", ERROR),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"_list-expression",
        vec![
            Rule::token_to(r"(?ms),", PUNCTUATION, NewState::Pop(1)),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)\(",
                PUNCTUATION,
                NewState::Push(vec![r"expression", r"_expression"]),
            ),
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r"(?ms)\[",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"statements", r"locals"]),
            ),
            Rule::token(r"(?ms)>(?=(\s+|(![^\n  ]*))*[>;])", PUNCTUATION),
            Rule::token(r"(?ms)\+\+|[\-‐-—]{2}(?!>)", OPERATOR),
            Rule::token_to(r"(?ms),", PUNCTUATION, NewState::Push(vec![r"_expression"])),
            Rule::token_to(
                r"(?ms)&&?|\|\|?|[=~><]?=|[\-‐-—]{1,2}>?|\.\.?[&#]?|::|[<>+*/%]",
                OPERATOR,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token_to(
                r"(?ms)(has|hasnt|in|notin|ofclass|or|provides)\b",
                OPERATOR_WORD,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token(r"(?ms)sp\b", NAME),
            Rule::token_to(r"(?ms)\?~?", NAME_LABEL, NewState::Push(vec![r"label?"])),
            Rule::token(r"(?ms)[@{]", ERROR),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"_object-expression",
        vec![
            Rule::token_to(r"(?ms)has\b", KEYWORD_DECLARATION, NewState::Pop(1)),
            Rule::token_to(r"(?ms),", PUNCTUATION, NewState::Pop(1)),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)\(",
                PUNCTUATION,
                NewState::Push(vec![r"expression", r"_expression"]),
            ),
            Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r"(?ms)\[",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"statements", r"locals"]),
            ),
            Rule::token(r"(?ms)>(?=(\s+|(![^\n  ]*))*[>;])", PUNCTUATION),
            Rule::token(r"(?ms)\+\+|[\-‐-—]{2}(?!>)", OPERATOR),
            Rule::token_to(r"(?ms),", PUNCTUATION, NewState::Push(vec![r"_expression"])),
            Rule::token_to(
                r"(?ms)&&?|\|\|?|[=~><]?=|[\-‐-—]{1,2}>?|\.\.?[&#]?|::|[<>+*/%]",
                OPERATOR,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token_to(
                r"(?ms)(has|hasnt|in|notin|ofclass|or|provides)\b",
                OPERATOR_WORD,
                NewState::Push(vec![r"_expression"]),
            ),
            Rule::token(r"(?ms)sp\b", NAME),
            Rule::token_to(r"(?ms)\?~?", NAME_LABEL, NewState::Push(vec![r"label?"])),
            Rule::token(r"(?ms)[@{]", ERROR),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(r"value", vec![
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)['‘’][^@]['‘’]", STRING_CHAR, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@\{[0-9a-fA-F]*\})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@.{2})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)['‘’]", STRING_SINGLE, NewState::Push(vec![r"#pop", r"dictionary-word"])),
        Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Push(vec![r"#pop", r"string"])),
        Rule::token_to(r"(?ms)\$[<>]?[+\-‐-—][0-9]*\.?[0-9]*([eE][+\-‐-—]?[0-9]+)?", NUMBER_FLOAT, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$[0-9a-fA-F]+", NUMBER_HEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$\$[01]+", NUMBER_BIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[0-9]+", NUMBER_INTEGER, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(##|#a\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(#g\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_VARIABLE_GLOBAL)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#[nw]\$", OPERATOR, NewState::Push(vec![r"#pop", r"obsolete-dictionary-word"])),
        Rule::bygroups_to(r"(?ms)(#r\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#", NAME_BUILTIN, NewState::Push(vec![r"#pop", r"system-constant"])),
        Rule::token_to(r"(?ms)(child(?:(?:ren)?)|elde(?:r|st)|glk|indirect|metaclass|parent|random|sibling|younge(?:r|st))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Class|Object|Routine|String)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Box__Routine|C(?:A__Pr|DefArt|InDefArt|P__Tab|l__Ms|opy__Primitive)|D(?:A__Pr|B__Pr|efArt|ynam__String)|EnglishNumber|Glk__Wrap|I(?:A__Pr|B__Pr|nDefArt)|M(?:ain__|eta__class)|O(?:B__(?:(?:M|Rem)ove)|C__Cl|P__Pr)|Print(?:ShortName|__(?:Addr|PName))|R(?:A__(?:Pr|Sc)|L__Pr|T__(?:Ch(?:Gt|LD(?:[BW])|P(?:rint(?:[ACOS])|[RS])|ST(?:[BW])|[GRT])|Err|TrPS)|V__Pr|_Process)|Symb__Tab|Unsigned__Compare|WV__Pr|Z__Region)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(D(?:EBUG|ICT_(?:CHAR_SIZE|ENTRY_BYTES|(?:IS_UNICOD|WORD_SIZ)E)|OUBLE_(?:HI_(?:INFINITY|N(?:AN|INFINITY))|LO_(?:INFINITY|N(?:AN|INFINITY))))|FLOAT_(?:INFINITY|N(?:AN|INFINITY))|G(?:OBJ(?:FIELD_(?:CH(?:AIN|ILD)|NAME|P(?:ARENT|ROPTAB)|SIBLING)|_(?:EXT_START|TOTAL_LENGTH))|rammar__Version)|IN(?:DIV_PROP_START|FIX)|MODULE_MODE|NUM_ATTR_BYTES|STRICT_MODE|TARGET_(?:GLULX|ZCODE)|USE_MODULES|WORDSIZE|c(?:all|opy|reate)|destroy|false|infix__watching|n(?:ame|othing)|print(?:(?:_to_array)?)|re(?:create|maining)|s(?:e(?:lf|nder)|w__var|ys_(?:_glob(?:[012])|statusline_flag))|t(?:emp_(?:_global(?:[234])|global)|rue))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME, NewState::Pop(1)),
    ]);
    m.insert(r"value?", vec![
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)['‘’][^@]['‘’]", STRING_CHAR, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@\{[0-9a-fA-F]*\})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@.{2})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)['‘’]", STRING_SINGLE, NewState::Push(vec![r"#pop", r"dictionary-word"])),
        Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Push(vec![r"#pop", r"string"])),
        Rule::token_to(r"(?ms)\$[<>]?[+\-‐-—][0-9]*\.?[0-9]*([eE][+\-‐-—]?[0-9]+)?", NUMBER_FLOAT, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$[0-9a-fA-F]+", NUMBER_HEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$\$[01]+", NUMBER_BIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[0-9]+", NUMBER_INTEGER, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(##|#a\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(#g\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_VARIABLE_GLOBAL)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#[nw]\$", OPERATOR, NewState::Push(vec![r"#pop", r"obsolete-dictionary-word"])),
        Rule::bygroups_to(r"(?ms)(#r\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#", NAME_BUILTIN, NewState::Push(vec![r"#pop", r"system-constant"])),
        Rule::token_to(r"(?ms)(child(?:(?:ren)?)|elde(?:r|st)|glk|indirect|metaclass|parent|random|sibling|younge(?:r|st))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Class|Object|Routine|String)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Box__Routine|C(?:A__Pr|DefArt|InDefArt|P__Tab|l__Ms|opy__Primitive)|D(?:A__Pr|B__Pr|efArt|ynam__String)|EnglishNumber|Glk__Wrap|I(?:A__Pr|B__Pr|nDefArt)|M(?:ain__|eta__class)|O(?:B__(?:(?:M|Rem)ove)|C__Cl|P__Pr)|Print(?:ShortName|__(?:Addr|PName))|R(?:A__(?:Pr|Sc)|L__Pr|T__(?:Ch(?:Gt|LD(?:[BW])|P(?:rint(?:[ACOS])|[RS])|ST(?:[BW])|[GRT])|Err|TrPS)|V__Pr|_Process)|Symb__Tab|Unsigned__Compare|WV__Pr|Z__Region)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(D(?:EBUG|ICT_(?:CHAR_SIZE|ENTRY_BYTES|(?:IS_UNICOD|WORD_SIZ)E)|OUBLE_(?:HI_(?:INFINITY|N(?:AN|INFINITY))|LO_(?:INFINITY|N(?:AN|INFINITY))))|FLOAT_(?:INFINITY|N(?:AN|INFINITY))|G(?:OBJ(?:FIELD_(?:CH(?:AIN|ILD)|NAME|P(?:ARENT|ROPTAB)|SIBLING)|_(?:EXT_START|TOTAL_LENGTH))|rammar__Version)|IN(?:DIV_PROP_START|FIX)|MODULE_MODE|NUM_ATTR_BYTES|STRICT_MODE|TARGET_(?:GLULX|ZCODE)|USE_MODULES|WORDSIZE|c(?:all|opy|reate)|destroy|false|infix__watching|n(?:ame|othing)|print(?:(?:_to_array)?)|re(?:create|maining)|s(?:e(?:lf|nder)|w__var|ys_(?:_glob(?:[012])|statusline_flag))|t(?:emp_(?:_global(?:[234])|global)|rue))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(
        r"dictionary-word",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)[~^]+|//[^'‘’]*", STRING_ESCAPE),
            Rule::token(r"(?ms)[^~^/\\@({'‘’]+", STRING_SINGLE),
            Rule::token(r"(?ms)[/({]", STRING_SINGLE),
            Rule::token(r"(?ms)@\{[0-9a-fA-F]*\}", STRING_ESCAPE),
            Rule::token(r"(?ms)@.{2}", STRING_ESCAPE),
            Rule::token_to(r"(?ms)['‘’]", STRING_SINGLE, NewState::Pop(1)),
        ],
    );
    m.insert(r"string", vec![
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)[~^]+", STRING_ESCAPE),
        Rule::token(r#"(?ms)[^~^\\@({"“”]+"#, STRING_DOUBLE),
        Rule::token(r"(?ms)[({]", STRING_DOUBLE),
        Rule::token(r"(?ms)\\", STRING_ESCAPE),
        Rule::token(r"(?ms)@(\\\s*[\n  ]\s*)*@((\\\s*[\n  ]\s*)*[0-9])*", STRING_ESCAPE),
        Rule::token(r"(?ms)@(\\\s*[\n  ]\s*)*[({]((\\\s*[\n  ]\s*)*[0-9a-zA-Z_])*(\\\s*[\n  ]\s*)*[)}]", STRING_ESCAPE),
        Rule::token(r"(?ms)@(\\\s*[\n  ]\s*)*.(\\\s*[\n  ]\s*)*.", STRING_ESCAPE),
        Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Pop(1)),
    ]);
    m.insert(
        r"plain-string",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r#"(?ms)[^~^\\({\[\]"“”]+"#, STRING_DOUBLE),
            Rule::token(r"(?ms)[~^({\[\]]", STRING_DOUBLE),
            Rule::token(r"(?ms)\\", STRING_ESCAPE),
            Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(r"_constant", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME_CONSTANT, NewState::Pop(1)),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)['‘’][^@]['‘’]", STRING_CHAR, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@\{[0-9a-fA-F]*\})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@.{2})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)['‘’]", STRING_SINGLE, NewState::Push(vec![r"#pop", r"dictionary-word"])),
        Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Push(vec![r"#pop", r"string"])),
        Rule::token_to(r"(?ms)\$[<>]?[+\-‐-—][0-9]*\.?[0-9]*([eE][+\-‐-—]?[0-9]+)?", NUMBER_FLOAT, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$[0-9a-fA-F]+", NUMBER_HEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$\$[01]+", NUMBER_BIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[0-9]+", NUMBER_INTEGER, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(##|#a\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(#g\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_VARIABLE_GLOBAL)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#[nw]\$", OPERATOR, NewState::Push(vec![r"#pop", r"obsolete-dictionary-word"])),
        Rule::bygroups_to(r"(?ms)(#r\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#", NAME_BUILTIN, NewState::Push(vec![r"#pop", r"system-constant"])),
        Rule::token_to(r"(?ms)(child(?:(?:ren)?)|elde(?:r|st)|glk|indirect|metaclass|parent|random|sibling|younge(?:r|st))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Class|Object|Routine|String)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Box__Routine|C(?:A__Pr|DefArt|InDefArt|P__Tab|l__Ms|opy__Primitive)|D(?:A__Pr|B__Pr|efArt|ynam__String)|EnglishNumber|Glk__Wrap|I(?:A__Pr|B__Pr|nDefArt)|M(?:ain__|eta__class)|O(?:B__(?:(?:M|Rem)ove)|C__Cl|P__Pr)|Print(?:ShortName|__(?:Addr|PName))|R(?:A__(?:Pr|Sc)|L__Pr|T__(?:Ch(?:Gt|LD(?:[BW])|P(?:rint(?:[ACOS])|[RS])|ST(?:[BW])|[GRT])|Err|TrPS)|V__Pr|_Process)|Symb__Tab|Unsigned__Compare|WV__Pr|Z__Region)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(D(?:EBUG|ICT_(?:CHAR_SIZE|ENTRY_BYTES|(?:IS_UNICOD|WORD_SIZ)E)|OUBLE_(?:HI_(?:INFINITY|N(?:AN|INFINITY))|LO_(?:INFINITY|N(?:AN|INFINITY))))|FLOAT_(?:INFINITY|N(?:AN|INFINITY))|G(?:OBJ(?:FIELD_(?:CH(?:AIN|ILD)|NAME|P(?:ARENT|ROPTAB)|SIBLING)|_(?:EXT_START|TOTAL_LENGTH))|rammar__Version)|IN(?:DIV_PROP_START|FIX)|MODULE_MODE|NUM_ATTR_BYTES|STRICT_MODE|TARGET_(?:GLULX|ZCODE)|USE_MODULES|WORDSIZE|c(?:all|opy|reate)|destroy|false|infix__watching|n(?:ame|othing)|print(?:(?:_to_array)?)|re(?:create|maining)|s(?:e(?:lf|nder)|w__var|ys_(?:_glob(?:[012])|statusline_flag))|t(?:emp_(?:_global(?:[234])|global)|rue))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME, NewState::Pop(1)),
    ]);
    m.insert(
        r"constant*",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token(r"(?ms),", PUNCTUATION),
            Rule::token_to(r"(?ms)=", PUNCTUATION, NewState::Push(vec![r"value?"])),
            Rule::token_to(
                r"(?ms)[a-zA-Z_]\w*",
                NAME_CONSTANT,
                NewState::Push(vec![r"value?"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(r"_global", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME_VARIABLE_GLOBAL, NewState::Pop(1)),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)['‘’][^@]['‘’]", STRING_CHAR, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@\{[0-9a-fA-F]*\})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@.{2})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)['‘’]", STRING_SINGLE, NewState::Push(vec![r"#pop", r"dictionary-word"])),
        Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Push(vec![r"#pop", r"string"])),
        Rule::token_to(r"(?ms)\$[<>]?[+\-‐-—][0-9]*\.?[0-9]*([eE][+\-‐-—]?[0-9]+)?", NUMBER_FLOAT, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$[0-9a-fA-F]+", NUMBER_HEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$\$[01]+", NUMBER_BIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[0-9]+", NUMBER_INTEGER, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(##|#a\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(#g\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_VARIABLE_GLOBAL)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#[nw]\$", OPERATOR, NewState::Push(vec![r"#pop", r"obsolete-dictionary-word"])),
        Rule::bygroups_to(r"(?ms)(#r\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#", NAME_BUILTIN, NewState::Push(vec![r"#pop", r"system-constant"])),
        Rule::token_to(r"(?ms)(child(?:(?:ren)?)|elde(?:r|st)|glk|indirect|metaclass|parent|random|sibling|younge(?:r|st))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Class|Object|Routine|String)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Box__Routine|C(?:A__Pr|DefArt|InDefArt|P__Tab|l__Ms|opy__Primitive)|D(?:A__Pr|B__Pr|efArt|ynam__String)|EnglishNumber|Glk__Wrap|I(?:A__Pr|B__Pr|nDefArt)|M(?:ain__|eta__class)|O(?:B__(?:(?:M|Rem)ove)|C__Cl|P__Pr)|Print(?:ShortName|__(?:Addr|PName))|R(?:A__(?:Pr|Sc)|L__Pr|T__(?:Ch(?:Gt|LD(?:[BW])|P(?:rint(?:[ACOS])|[RS])|ST(?:[BW])|[GRT])|Err|TrPS)|V__Pr|_Process)|Symb__Tab|Unsigned__Compare|WV__Pr|Z__Region)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(D(?:EBUG|ICT_(?:CHAR_SIZE|ENTRY_BYTES|(?:IS_UNICOD|WORD_SIZ)E)|OUBLE_(?:HI_(?:INFINITY|N(?:AN|INFINITY))|LO_(?:INFINITY|N(?:AN|INFINITY))))|FLOAT_(?:INFINITY|N(?:AN|INFINITY))|G(?:OBJ(?:FIELD_(?:CH(?:AIN|ILD)|NAME|P(?:ARENT|ROPTAB)|SIBLING)|_(?:EXT_START|TOTAL_LENGTH))|rammar__Version)|IN(?:DIV_PROP_START|FIX)|MODULE_MODE|NUM_ATTR_BYTES|STRICT_MODE|TARGET_(?:GLULX|ZCODE)|USE_MODULES|WORDSIZE|c(?:all|opy|reate)|destroy|false|infix__watching|n(?:ame|othing)|print(?:(?:_to_array)?)|re(?:create|maining)|s(?:e(?:lf|nder)|w__var|ys_(?:_glob(?:[012])|statusline_flag))|t(?:emp_(?:_global(?:[234])|global)|rue))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME, NewState::Pop(1)),
    ]);
    m.insert(
        r"label?",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME_LABEL, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"variable?",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME_VARIABLE, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"obsolete-dictionary-word",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token_to(r"(?ms)\S\w*", STRING_OTHER, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"system-constant",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME_BUILTIN, NewState::Pop(1)),
        ],
    );
    m.insert(r"directive", vec![
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token(r"(?ms)#", PUNCTUATION),
        Rule::token_to(r"(?ms);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\[", PUNCTUATION, NewState::Push(vec![r"default", r"statements", r"locals", r"routine-name?"])),
        Rule::token_to(r"(?ms)(?i)(abbreviate|dictionary|endif|if(?:def|false|n(?:def|ot)|true|v(?:[35]))|release|s(?:erial|witches|ystem_file)|version)\b", KEYWORD, NewState::Push(vec![r"default"])),
        Rule::token_to(r"(?ms)(?i)(array|global)\b", KEYWORD, NewState::Push(vec![r"default", r"directive-keyword?", r"_global"])),
        Rule::token_to(r"(?ms)(?i)attribute\b", KEYWORD, NewState::Push(vec![r"default", r"alias?", r"_constant"])),
        Rule::token_to(r"(?ms)(?i)class\b", KEYWORD, NewState::Push(vec![r"object-body", r"duplicates", r"class-name"])),
        Rule::token_to(r"(?ms)(?i)(constant|default)\b", KEYWORD, NewState::Push(vec![r"default", r"constant*"])),
        Rule::bygroups(r"(?ms)(?i)(end\b)(.*)", vec![Some(KEYWORD), Some(TEXT)]),
        Rule::token_to(r"(?ms)(?i)(extend|verb)\b", KEYWORD, NewState::Push(vec![r"grammar"])),
        Rule::token_to(r"(?ms)(?i)fake_action\b", KEYWORD, NewState::Push(vec![r"default", r"_constant"])),
        Rule::token_to(r"(?ms)(?i)import\b", KEYWORD, NewState::Push(vec![r"manifest"])),
        Rule::token_to(r"(?ms)(?i)(include|link|origsource)\b", KEYWORD, NewState::Push(vec![r"default", r"before-plain-string?"])),
        Rule::token_to(r"(?ms)(?i)(lowstring|undef)\b", KEYWORD, NewState::Push(vec![r"default", r"_constant"])),
        Rule::token_to(r"(?ms)(?i)message\b", KEYWORD, NewState::Push(vec![r"default", r"diagnostic"])),
        Rule::token_to(r"(?ms)(?i)(nearby|object)\b", KEYWORD, NewState::Push(vec![r"object-body", r"_object-head"])),
        Rule::token_to(r"(?ms)(?i)property\b", KEYWORD, NewState::Push(vec![r"default", r"alias?", r"_constant", r"property-keyword*"])),
        Rule::token_to(r"(?ms)(?i)replace\b", KEYWORD, NewState::Push(vec![r"default", r"routine-name?", r"routine-name?"])),
        Rule::token_to(r"(?ms)(?i)statusline\b", KEYWORD, NewState::Push(vec![r"default", r"directive-keyword?"])),
        Rule::token_to(r"(?ms)(?i)stub\b", KEYWORD, NewState::Push(vec![r"default", r"routine-name?"])),
        Rule::token_to(r"(?ms)(?i)trace\b", KEYWORD, NewState::Push(vec![r"default", r"trace-keyword?", r"trace-keyword?"])),
        Rule::token_to(r"(?ms)(?i)zcharacter\b", KEYWORD, NewState::Push(vec![r"default", r"directive-keyword?", r"directive-keyword?"])),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME_CLASS, NewState::Push(vec![r"object-body", r"_object-head"])),
    ]);
    m.insert(
        r"routine-name?",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME_FUNCTION, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"locals",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms);", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?ms)\*", PUNCTUATION),
            Rule::token_to(
                r#"(?ms)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"plain-string"]),
            ),
            Rule::token(r"(?ms)[a-zA-Z_]\w*", NAME_VARIABLE),
        ],
    );
    m.insert(
        r"many-values",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token(r"(?ms);", PUNCTUATION),
            Rule::token_to(r"(?ms)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?ms):", ERROR),
            Rule::default(NewState::Push(vec![r"expression", r"_expression"])),
        ],
    );
    m.insert(
        r"alias?",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)alias\b",
                KEYWORD,
                NewState::Push(vec![r"#pop", r"_constant"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"class-name",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)(?=[,;]|(class|has|private|with)\b)",
                TEXT,
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME_CLASS, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"duplicates",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)\(",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"expression", r"_expression"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(r"_object-head", vec![
        Rule::token(r"(?ms)[\-‐-—]>", PUNCTUATION),
        Rule::token_to(r"(?ms)(class|has|private|with)\b", KEYWORD_DECLARATION, NewState::Pop(1)),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME_VARIABLE_GLOBAL, NewState::Pop(1)),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)['‘’][^@]['‘’]", STRING_CHAR, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@\{[0-9a-fA-F]*\})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@.{2})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)['‘’]", STRING_SINGLE, NewState::Push(vec![r"#pop", r"dictionary-word"])),
        Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Push(vec![r"#pop", r"string"])),
        Rule::token_to(r"(?ms)\$[<>]?[+\-‐-—][0-9]*\.?[0-9]*([eE][+\-‐-—]?[0-9]+)?", NUMBER_FLOAT, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$[0-9a-fA-F]+", NUMBER_HEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$\$[01]+", NUMBER_BIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[0-9]+", NUMBER_INTEGER, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(##|#a\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(#g\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_VARIABLE_GLOBAL)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#[nw]\$", OPERATOR, NewState::Push(vec![r"#pop", r"obsolete-dictionary-word"])),
        Rule::bygroups_to(r"(?ms)(#r\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#", NAME_BUILTIN, NewState::Push(vec![r"#pop", r"system-constant"])),
        Rule::token_to(r"(?ms)(child(?:(?:ren)?)|elde(?:r|st)|glk|indirect|metaclass|parent|random|sibling|younge(?:r|st))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Class|Object|Routine|String)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Box__Routine|C(?:A__Pr|DefArt|InDefArt|P__Tab|l__Ms|opy__Primitive)|D(?:A__Pr|B__Pr|efArt|ynam__String)|EnglishNumber|Glk__Wrap|I(?:A__Pr|B__Pr|nDefArt)|M(?:ain__|eta__class)|O(?:B__(?:(?:M|Rem)ove)|C__Cl|P__Pr)|Print(?:ShortName|__(?:Addr|PName))|R(?:A__(?:Pr|Sc)|L__Pr|T__(?:Ch(?:Gt|LD(?:[BW])|P(?:rint(?:[ACOS])|[RS])|ST(?:[BW])|[GRT])|Err|TrPS)|V__Pr|_Process)|Symb__Tab|Unsigned__Compare|WV__Pr|Z__Region)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(D(?:EBUG|ICT_(?:CHAR_SIZE|ENTRY_BYTES|(?:IS_UNICOD|WORD_SIZ)E)|OUBLE_(?:HI_(?:INFINITY|N(?:AN|INFINITY))|LO_(?:INFINITY|N(?:AN|INFINITY))))|FLOAT_(?:INFINITY|N(?:AN|INFINITY))|G(?:OBJ(?:FIELD_(?:CH(?:AIN|ILD)|NAME|P(?:ARENT|ROPTAB)|SIBLING)|_(?:EXT_START|TOTAL_LENGTH))|rammar__Version)|IN(?:DIV_PROP_START|FIX)|MODULE_MODE|NUM_ATTR_BYTES|STRICT_MODE|TARGET_(?:GLULX|ZCODE)|USE_MODULES|WORDSIZE|c(?:all|opy|reate)|destroy|false|infix__watching|n(?:ame|othing)|print(?:(?:_to_array)?)|re(?:create|maining)|s(?:e(?:lf|nder)|w__var|ys_(?:_glob(?:[012])|statusline_flag))|t(?:emp_(?:_global(?:[234])|global)|rue))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME, NewState::Pop(1)),
    ]);
    m.insert(
        r"object-body",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms);", PUNCTUATION, NewState::Pop(2)),
            Rule::token(r"(?ms),", PUNCTUATION),
            Rule::token_to(
                r"(?ms)class\b",
                KEYWORD_DECLARATION,
                NewState::Push(vec![r"class-segment"]),
            ),
            Rule::token(r"(?ms)(has|private|with)\b", KEYWORD_DECLARATION),
            Rule::token(r"(?ms):", ERROR),
            Rule::default(NewState::Push(vec![r"_object-expression", r"_expression"])),
        ],
    );
    m.insert(
        r"class-segment",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)(?=[,;]|(class|has|private|with)\b)",
                TEXT,
                NewState::Pop(1),
            ),
            Rule::token(r"(?ms)[a-zA-Z_]\w*", NAME_CLASS),
            Rule::default(NewState::Push(vec![r"value"])),
        ],
    );
    m.insert(
        r"grammar",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)=",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"default"]),
            ),
            Rule::token_to(
                r"(?ms)\*",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"grammar-line"]),
            ),
            Rule::default(NewState::Push(vec![r"_directive-keyword"])),
        ],
    );
    m.insert(
        r"grammar-line",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms);", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?ms)[/*]", PUNCTUATION),
            Rule::token_to(
                r"(?ms)[\-‐-—]>",
                PUNCTUATION,
                NewState::Push(vec![r"value"]),
            ),
            Rule::token_to(
                r"(?ms)(noun|scope)\b",
                KEYWORD,
                NewState::Push(vec![r"=routine"]),
            ),
            Rule::default(NewState::Push(vec![r"_directive-keyword"])),
        ],
    );
    m.insert(
        r"=routine",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)=",
                PUNCTUATION,
                NewState::Push(vec![r"routine-name?"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"manifest",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms);", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?ms),", PUNCTUATION),
            Rule::token_to(
                r"(?ms)(?i)global\b",
                KEYWORD,
                NewState::Push(vec![r"_global"]),
            ),
            Rule::default(NewState::Push(vec![r"_global"])),
        ],
    );
    m.insert(
        r"diagnostic",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r#"(?ms)["“”]"#,
                STRING_DOUBLE,
                NewState::Push(vec![r"#pop", r"message-string"]),
            ),
            Rule::default(NewState::Push(vec![
                r"#pop",
                r"before-plain-string?",
                r"directive-keyword?",
            ])),
        ],
    );
    m.insert(
        r"before-plain-string?",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r#"(?ms)["“”]"#,
                STRING_DOUBLE,
                NewState::Push(vec![r"#pop", r"plain-string"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"message-string",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)[~^]+", STRING_ESCAPE),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r#"(?ms)[^~^\\({\[\]"“”]+"#, STRING_DOUBLE),
            Rule::token(r"(?ms)[~^({\[\]]", STRING_DOUBLE),
            Rule::token(r"(?ms)\\", STRING_ESCAPE),
            Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(r"_directive-keyword!", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)(a(?:dditive|lias)|buffer|c(?:lass|reature)|data|error|f(?:atalerror|irst)|h(?:as|eld)|in(?:dividual|it(?:ial|str))|l(?:ast|ong)|m(?:eta|ulti(?:(?:except|held|inside)?))|n(?:oun|umber)|only|private|re(?:(?:plac|vers)e)|s(?:co(?:(?:[pr])e)|pecial|tring)|t(?:able|erminating|ime|opic)|w(?:arning|ith))\b", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?ms)static\b", KEYWORD),
        Rule::token_to(r"(?ms)[\-‐-—]{1,2}>|[+=]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"_directive-keyword", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)(a(?:dditive|lias)|buffer|c(?:lass|reature)|data|error|f(?:atalerror|irst)|h(?:as|eld)|in(?:dividual|it(?:ial|str))|l(?:ast|ong)|m(?:eta|ulti(?:(?:except|held|inside)?))|n(?:oun|umber)|only|private|re(?:(?:plac|vers)e)|s(?:co(?:(?:[pr])e)|pecial|tring)|t(?:able|erminating|ime|opic)|w(?:arning|ith))\b", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?ms)static\b", KEYWORD),
        Rule::token_to(r"(?ms)[\-‐-—]{1,2}>|[+=]", PUNCTUATION, NewState::Pop(1)),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)['‘’][^@]['‘’]", STRING_CHAR, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@\{[0-9a-fA-F]*\})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(['‘’])(@.{2})(['‘’])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)['‘’]", STRING_SINGLE, NewState::Push(vec![r"#pop", r"dictionary-word"])),
        Rule::token_to(r#"(?ms)["“”]"#, STRING_DOUBLE, NewState::Push(vec![r"#pop", r"string"])),
        Rule::token_to(r"(?ms)\$[<>]?[+\-‐-—][0-9]*\.?[0-9]*([eE][+\-‐-—]?[0-9]+)?", NUMBER_FLOAT, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$[0-9a-fA-F]+", NUMBER_HEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$\$[01]+", NUMBER_BIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[0-9]+", NUMBER_INTEGER, NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(##|#a\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)(#g\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_VARIABLE_GLOBAL)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#[nw]\$", OPERATOR, NewState::Push(vec![r"#pop", r"obsolete-dictionary-word"])),
        Rule::bygroups_to(r"(?ms)(#r\$)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)], NewState::Pop(1)),
        Rule::token_to(r"(?ms)#", NAME_BUILTIN, NewState::Push(vec![r"#pop", r"system-constant"])),
        Rule::token_to(r"(?ms)(child(?:(?:ren)?)|elde(?:r|st)|glk|indirect|metaclass|parent|random|sibling|younge(?:r|st))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Class|Object|Routine|String)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(Box__Routine|C(?:A__Pr|DefArt|InDefArt|P__Tab|l__Ms|opy__Primitive)|D(?:A__Pr|B__Pr|efArt|ynam__String)|EnglishNumber|Glk__Wrap|I(?:A__Pr|B__Pr|nDefArt)|M(?:ain__|eta__class)|O(?:B__(?:(?:M|Rem)ove)|C__Cl|P__Pr)|Print(?:ShortName|__(?:Addr|PName))|R(?:A__(?:Pr|Sc)|L__Pr|T__(?:Ch(?:Gt|LD(?:[BW])|P(?:rint(?:[ACOS])|[RS])|ST(?:[BW])|[GRT])|Err|TrPS)|V__Pr|_Process)|Symb__Tab|Unsigned__Compare|WV__Pr|Z__Region)\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?i)(D(?:EBUG|ICT_(?:CHAR_SIZE|ENTRY_BYTES|(?:IS_UNICOD|WORD_SIZ)E)|OUBLE_(?:HI_(?:INFINITY|N(?:AN|INFINITY))|LO_(?:INFINITY|N(?:AN|INFINITY))))|FLOAT_(?:INFINITY|N(?:AN|INFINITY))|G(?:OBJ(?:FIELD_(?:CH(?:AIN|ILD)|NAME|P(?:ARENT|ROPTAB)|SIBLING)|_(?:EXT_START|TOTAL_LENGTH))|rammar__Version)|IN(?:DIV_PROP_START|FIX)|MODULE_MODE|NUM_ATTR_BYTES|STRICT_MODE|TARGET_(?:GLULX|ZCODE)|USE_MODULES|WORDSIZE|c(?:all|opy|reate)|destroy|false|infix__watching|n(?:ame|othing)|print(?:(?:_to_array)?)|re(?:create|maining)|s(?:e(?:lf|nder)|w__var|ys_(?:_glob(?:[012])|statusline_flag))|t(?:emp_(?:_global(?:[234])|global)|rue))\b", NAME_BUILTIN, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*", NAME, NewState::Pop(1)),
    ]);
    m.insert(r"directive-keyword?", vec![
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)(a(?:dditive|lias)|buffer|c(?:lass|reature)|data|error|f(?:atalerror|irst)|h(?:as|eld)|in(?:dividual|it(?:ial|str))|l(?:ast|ong)|m(?:eta|ulti(?:(?:except|held|inside)?))|n(?:oun|umber)|only|private|re(?:(?:plac|vers)e)|s(?:co(?:(?:[pr])e)|pecial|tring)|t(?:able|erminating|ime|opic)|w(?:arning|ith))\b", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?ms)static\b", KEYWORD),
        Rule::token_to(r"(?ms)[\-‐-—]{1,2}>|[+=]", PUNCTUATION, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(
        r"property-keyword*",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token(
                r"(?ms)(additive|individual|long)\b(?=(\s*|(![^\n  ]*[\n  ]))*[_a-zA-Z])",
                KEYWORD,
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(r"trace-keyword?", vec![
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)(assembly|dictionary|expressions|lin(?:es|ker)|o(?:bjects|ff|n)|(?:symbol|token|verb)s)\b", KEYWORD, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"statements", vec![
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?ms)[;{}]", PUNCTUATION),
        Rule::token_to(r"(?ms)(b(?:ox|reak)|continue|default|give|inversion|new_line|quit|r(?:e(?:ad|move|turn)|(?:fals|tru)e)|s(?:paces|tring)|until)\b", KEYWORD, NewState::Push(vec![r"default"])),
        Rule::token(r"(?ms)(do|else)\b", KEYWORD),
        Rule::token_to(r"(?ms)(font|style)\b", KEYWORD, NewState::Push(vec![r"default", r"miscellaneous-keyword?"])),
        Rule::token_to(r"(?ms)for\b", KEYWORD, NewState::Push(vec![r"for", r"(?"])),
        Rule::token_to(r"(?ms)(if|switch|while)", KEYWORD, NewState::Push(vec![r"expression", r"_expression", r"(?"])),
        Rule::token_to(r"(?ms)(jump|save|restore)\b", KEYWORD, NewState::Push(vec![r"default", r"label?"])),
        Rule::token_to(r"(?ms)objectloop\b", KEYWORD, NewState::Push(vec![r"_keyword-expression", r"variable?", r"(?"])),
        Rule::token_to(r#"(?ms)print(_ret)?\b|(?=["“”])"#, KEYWORD, NewState::Push(vec![r"print-list"])),
        Rule::token_to(r"(?ms)\.", NAME_LABEL, NewState::Push(vec![r"label?"])),
        Rule::token_to(r"(?ms)@", KEYWORD, NewState::Push(vec![r"opcode"])),
        Rule::token_to(r"(?ms)#(?![agrnw]\$|#)", PUNCTUATION, NewState::Push(vec![r"directive"])),
        Rule::token_to(r"(?ms)<", PUNCTUATION, NewState::Push(vec![r"default"])),
        Rule::token_to(r"(?ms)move\b", KEYWORD, NewState::Push(vec![r"default", r"_keyword-expression", r"_expression"])),
        Rule::default(NewState::Push(vec![r"default", r"_keyword-expression", r"_expression"])),
    ]);
    m.insert(r"miscellaneous-keyword?", vec![
        Rule::bygroups(r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)", vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"+lines"])),
        Rule::bygroups(r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?ms)(\(\+)(.*?)(\+\)|\Z)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "+main"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)", COMMENT_PREPROC),
        Rule::token(r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)", GENERIC_HEADING, NewState::Push(vec![r"+p"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
        Rule::token_to(r"(?ms)(bold|fixed|from|near|off|on|reverse|roman|to|underline)\b", KEYWORD, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(a|A|an|address|char|name|number|object|property|string|the|The)\b(?=(\s+|(![^\n  ]*))*\))", KEYWORD_PSEUDO, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*(?=(\s+|(![^\n  ]*))*\))", NAME_FUNCTION, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(
        r"(?",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms)\(", PUNCTUATION, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"for",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms);",
                PUNCTUATION,
                NewState::Push(vec![r"_for-expression", r"_expression"]),
            ),
            Rule::default(NewState::Push(vec![r"_for-expression", r"_expression"])),
        ],
    );
    m.insert(
        r"print-list",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(r"(?ms);", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?ms):", ERROR),
            Rule::default(NewState::Push(vec![
                r"_list-expression",
                r"_expression",
                r"_list-expression",
                r"form",
            ])),
        ],
    );
    m.insert(
        r"form",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?ms)\(",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"miscellaneous-keyword?"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"opcode",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)![^\n  ]*", COMMENT_SINGLE),
            Rule::token_to(
                r#"(?ms)["“”]"#,
                STRING_DOUBLE,
                NewState::Push(vec![r"operands", r"plain-string"]),
            ),
            Rule::token_to(
                r"(?ms)[\-‐-—]{1,2}>",
                PUNCTUATION,
                NewState::Push(vec![r"operands"]),
            ),
            Rule::token_to(
                r"(?ms)[a-zA-Z_]\w*",
                KEYWORD,
                NewState::Push(vec![r"operands"]),
            ),
        ],
    );
    m.insert(
        r"operands",
        vec![
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])(![^}]*)(\}?)",
                vec![Some(PUNCTUATION), Some(COMMENT_SINGLE), Some(PUNCTUATION)],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{[\-‐-—])(lines)(:)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"+lines"]),
            ),
            Rule::bygroups(
                r"(?ms)(\{[\-‐-—])([^:}]*)(:?)([^}]*)(\}?)",
                vec![
                    Some(PUNCTUATION),
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?ms)(\(\+)(.*?)(\+\)|\Z)",
                vec![
                    Some(GroupAction::Token(PUNCTUATION)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "+main"]),
                    }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@c( .*?)?([\n  ]|\Z)",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?ms)(\A|(?<=[\n  ]))@([\-‐-—]+|Purpose:)[^\n  ]*",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?ms)(\A|(?<=[\n  ]))@p( .*?)?([\n  ]|\Z)",
                GENERIC_HEADING,
                NewState::Push(vec![r"+p"]),
            ),
            Rule::token(r"(?ms):", ERROR),
            Rule::default(NewState::Push(vec![
                r"_assembly-expression",
                r"_expression",
            ])),
        ],
    );
    Table(m)
}

impl Lexer for Inform7Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
