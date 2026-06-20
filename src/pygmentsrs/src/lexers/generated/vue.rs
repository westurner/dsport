#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.html:VueLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.html:VueLexer:vue

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: vue
pub struct VueLexer;

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
            Rule::bygroups_g(
                r"(?ims)(\{\{)(.*?)(\}\})",
                vec![
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingLexer {
                        alias: "javascript",
                        state: None,
                    }),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                ],
            ),
            Rule::token(r"(?ims)[^<&{]+", TEXT),
            Rule::token(r"(?ims)[^<&]+", TEXT),
            Rule::token(r"(?ims)&\S*?;", NAME_ENTITY),
            Rule::token(r"(?ims)\<\!\[CDATA\[.*?\]\]\>", COMMENT_PREPROC),
            Rule::token(r"(?ims)<!--.*?-->", COMMENT_MULTILINE),
            Rule::token(r"(?ims)<\?.*?\?>", COMMENT_PREPROC),
            Rule::token(r"(?ims)<![^>]*>", COMMENT_PREPROC),
            Rule::bygroups_to(
                r"(?ims)(<)(\s*)(script)(\s*)",
                vec![Some(PUNCTUATION), Some(TEXT), Some(NAME_TAG), Some(TEXT)],
                NewState::Push(vec![r"script-content", r"tag"]),
            ),
            Rule::bygroups_to(
                r"(?ims)(<)(\s*)(style)(\s*)",
                vec![Some(PUNCTUATION), Some(TEXT), Some(NAME_TAG), Some(TEXT)],
                NewState::Push(vec![r"style-content", r"tag"]),
            ),
            Rule::bygroups_to(
                r"(?ims)(<)(\s*)([\w:.-]+)",
                vec![Some(PUNCTUATION), Some(TEXT), Some(NAME_TAG)],
                NewState::Push(vec![r"tag"]),
            ),
            Rule::bygroups(
                r"(?ims)(<)(\s*)(/)(\s*)([\w:.-]+)(\s*)(>)",
                vec![
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(NAME_TAG),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
            ),
        ],
    );
    m.insert(
        r"tag",
        vec![
            Rule::token(r"(?ims)\s+", TEXT),
            Rule::bygroups_g_to(
                r"(?ims)((?:[@:]|v-)(?:[.\w:-]|\[[^\]]*?\])+\s*)(=)(\s*)",
                vec![
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["name"]),
                    }),
                    Some(GroupAction::Token(OPERATOR)),
                    Some(GroupAction::Token(TEXT)),
                ],
                NewState::Push(vec![r"attr-directive"]),
            ),
            Rule::bygroups_to(
                r"(?ims)([\w:-]+\s*)(=)(\s*)",
                vec![Some(NAME_ATTRIBUTE), Some(OPERATOR), Some(TEXT)],
                NewState::Push(vec![r"attr"]),
            ),
            Rule::token(r"(?ims)[\w:-]+", NAME_ATTRIBUTE),
            Rule::bygroups_to(
                r"(?ims)(/?)(\s*)(>)",
                vec![Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION)],
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(
        r"name",
        vec![
            Rule::token(r"(?ims)[\w-]+", NAME_ATTRIBUTE),
            Rule::token(r"(?ims)[:@.]", PUNCTUATION),
            Rule::bygroups_g(
                r"(?ims)(\[)([^\]]*?)(\])",
                vec![
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingLexer {
                        alias: "javascript",
                        state: None,
                    }),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                ],
            ),
        ],
    );
    m.insert(
        r"attr-directive",
        vec![
            Rule::bygroups_g_to(
                r#"(?ims)(["\'])(.*?)(\1)"#,
                vec![
                    Some(GroupAction::Token(STRING)),
                    Some(GroupAction::UsingLexer {
                        alias: "javascript",
                        state: None,
                    }),
                    Some(GroupAction::Token(STRING)),
                ],
                NewState::Pop(1),
            ),
            Rule::using_lexer_to(r"(?ims)[^\s>]+", "javascript", None, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"script-content",
        vec![
            Rule::bygroups_to(
                r"(?ims)(<)(\s*)(/)(\s*)(script)(\s*)(>)",
                vec![
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(NAME_TAG),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Pop(1),
            ),
            Rule::using_lexer(r"(?ims).+?(?=<\s*/\s*script\s*>)", "javascript", None),
            Rule::using_lexer_to(r"(?ims).+?\n", "javascript", None, NewState::Pop(1)),
            Rule::using_lexer_to(r"(?ims).+", "javascript", None, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"style-content",
        vec![
            Rule::bygroups_to(
                r"(?ims)(<)(\s*)(/)(\s*)(style)(\s*)(>)",
                vec![
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(NAME_TAG),
                    Some(TEXT),
                    Some(PUNCTUATION),
                ],
                NewState::Pop(1),
            ),
            Rule::using_lexer(r"(?ims).+?(?=<\s*/\s*style\s*>)", "css", None),
            Rule::using_lexer_to(r"(?ims).+?\n", "css", None, NewState::Pop(1)),
            Rule::using_lexer_to(r"(?ims).+", "css", None, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"attr",
        vec![
            Rule::token_to(r#"(?ims)".*?""#, STRING, NewState::Pop(1)),
            Rule::token_to(r"(?ims)'.*?'", STRING, NewState::Pop(1)),
            Rule::token_to(r"(?ims)[^\s>]+", STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for VueLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
