//! AUTO-GENERATED from `pygments.pygments.lexers.grammar_notation:JsgfLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.grammar_notation:JsgfLexer:jsgf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: jsgf
pub struct JsgfLexer;

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
            Rule::token_to(
                r"(?m)/\*\*(?!/)",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"documentation comment"]),
            ),
            Rule::token(r"(?m)/\*[\w\W]*?\*/", COMMENT_MULTILINE),
            Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
            Rule::token(r"(?m)\A#JSGF[^;]*", COMMENT_PREPROC),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m);", PUNCTUATION),
            Rule::token(r"(?m)[=|()\[\]*+]", OPERATOR),
            Rule::token(r"(?m)/[^/]+/", NUMBER_FLOAT),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token_to(r"(?m)\{", STRING_OTHER, NewState::Push(vec![r"tag"])),
            Rule::token(r"(?m)(import|public)\b", KEYWORD_RESERVED),
            Rule::token_to(
                r"(?m)grammar\b",
                KEYWORD_RESERVED,
                NewState::Push(vec![r"grammar name"]),
            ),
            Rule::bygroups(
                r"(?m)(<)(NULL|VOID)(>)",
                vec![Some(PUNCTUATION), Some(NAME_BUILTIN), Some(PUNCTUATION)],
            ),
            Rule::token_to(r"(?m)<", PUNCTUATION, NewState::Push(vec![r"rulename"])),
            Rule::token(r#"(?m)\w+|[^\s;=|()\[\]*+/"{<\w]+"#, TEXT),
        ],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token_to(
                r"(?m)/\*\*(?!/)",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"documentation comment"]),
            ),
            Rule::token(r"(?m)/\*[\w\W]*?\*/", COMMENT_MULTILINE),
            Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"non-comments",
        vec![
            Rule::token(r"(?m)\A#JSGF[^;]*", COMMENT_PREPROC),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m);", PUNCTUATION),
            Rule::token(r"(?m)[=|()\[\]*+]", OPERATOR),
            Rule::token(r"(?m)/[^/]+/", NUMBER_FLOAT),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token_to(r"(?m)\{", STRING_OTHER, NewState::Push(vec![r"tag"])),
            Rule::token(r"(?m)(import|public)\b", KEYWORD_RESERVED),
            Rule::token_to(
                r"(?m)grammar\b",
                KEYWORD_RESERVED,
                NewState::Push(vec![r"grammar name"]),
            ),
            Rule::bygroups(
                r"(?m)(<)(NULL|VOID)(>)",
                vec![Some(PUNCTUATION), Some(NAME_BUILTIN), Some(PUNCTUATION)],
            ),
            Rule::token_to(r"(?m)<", PUNCTUATION, NewState::Push(vec![r"rulename"])),
            Rule::token(r#"(?m)\w+|[^\s;=|()\[\]*+/"{<\w]+"#, TEXT),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r#"(?m)[^\\"]+"#, STRING_DOUBLE),
        ],
    );
    m.insert(
        r"tag",
        vec![
            Rule::token_to(r"(?m)\}", STRING_OTHER, NewState::Pop(1)),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r"(?m)[^\\}]+", STRING_OTHER),
        ],
    );
    m.insert(
        r"grammar name",
        vec![
            Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\.", PUNCTUATION),
            Rule::token(r"(?m)[^;\s.]+", NAME_NAMESPACE),
        ],
    );
    m.insert(
        r"rulename",
        vec![
            Rule::token_to(r"(?m)>", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)\*", PUNCTUATION),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?m)([^.>]+)(\s*)(\.)",
                vec![Some(NAME_NAMESPACE), Some(TEXT), Some(PUNCTUATION)],
            ),
            Rule::token(r"(?m)[^.>]+", NAME_CONSTANT),
        ],
    );
    m.insert(
        r"documentation comment",
        vec![
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::bygroups_g(
                r"(?m)^(\s*)(\*?)(\s*)(@(?:example|see))(\s+)([\w\W]*?(?=(?:^\s*\*?\s*@|\*/)))",
                vec![
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(COMMENT_MULTILINE)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(COMMENT_SPECIAL)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "example"]),
                    }),
                ],
            ),
            Rule::bygroups(
                r"(?m)(^\s*\*?\s*)(@\S*)",
                vec![Some(COMMENT_MULTILINE), Some(COMMENT_SPECIAL)],
            ),
            Rule::token(r"(?m)[^*\n@]+|\w|\W", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"example",
        vec![
            Rule::bygroups(
                r"(?m)(\n\s*)(\*)",
                vec![Some(WHITESPACE), Some(COMMENT_MULTILINE)],
            ),
            Rule::token(r"(?m)\A#JSGF[^;]*", COMMENT_PREPROC),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m);", PUNCTUATION),
            Rule::token(r"(?m)[=|()\[\]*+]", OPERATOR),
            Rule::token(r"(?m)/[^/]+/", NUMBER_FLOAT),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token_to(r"(?m)\{", STRING_OTHER, NewState::Push(vec![r"tag"])),
            Rule::token(r"(?m)(import|public)\b", KEYWORD_RESERVED),
            Rule::token_to(
                r"(?m)grammar\b",
                KEYWORD_RESERVED,
                NewState::Push(vec![r"grammar name"]),
            ),
            Rule::bygroups(
                r"(?m)(<)(NULL|VOID)(>)",
                vec![Some(PUNCTUATION), Some(NAME_BUILTIN), Some(PUNCTUATION)],
            ),
            Rule::token_to(r"(?m)<", PUNCTUATION, NewState::Push(vec![r"rulename"])),
            Rule::token(r#"(?m)\w+|[^\s;=|()\[\]*+/"{<\w]+"#, TEXT),
            Rule::token(r"(?m).", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for JsgfLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
