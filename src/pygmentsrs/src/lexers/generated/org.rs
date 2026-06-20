#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.markup:OrgLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.markup:OrgLexer:org

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: org, orgmode, org-mode
pub struct OrgLexer;

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
            Rule::token(r"(?m)^# .*", COMMENT_SINGLE),
            Rule::bygroups(
                r"(?m)^(\* )(COMMENT)( .*)",
                vec![
                    Some(GENERIC_HEADING),
                    Some(COMMENT_PREPROC),
                    Some(GENERIC_HEADING),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\*\*+ )(COMMENT)( .*)",
                vec![
                    Some(GENERIC_SUBHEADING),
                    Some(COMMENT_PREPROC),
                    Some(GENERIC_SUBHEADING),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\* )(DONE)( .*)",
                vec![
                    Some(GENERIC_HEADING),
                    Some(GENERIC_DELETED),
                    Some(GENERIC_HEADING),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\*\*+ )(DONE)( .*)",
                vec![
                    Some(GENERIC_SUBHEADING),
                    Some(GENERIC_DELETED),
                    Some(GENERIC_SUBHEADING),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\* )(TODO)( .*)",
                vec![
                    Some(GENERIC_HEADING),
                    Some(GENERIC_ERROR),
                    Some(GENERIC_HEADING),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\*\*+ )(TODO)( .*)",
                vec![
                    Some(GENERIC_SUBHEADING),
                    Some(GENERIC_ERROR),
                    Some(GENERIC_SUBHEADING),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\* .+?)( :[a-zA-Z0-9_@:]+:)?$",
                vec![Some(GENERIC_HEADING), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups(
                r"(?m)^(\*\*+ .+?)( :[a-zA-Z0-9_@:]+:)?$",
                vec![Some(GENERIC_SUBHEADING), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups(
                r"(?m)^(?:( *)([+-] )|( +)(\* ))(\[[ X-]\])?(.+ ::)?",
                vec![
                    Some(WHITESPACE),
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(KEYWORD),
                    Some(GENERIC_PROMPT),
                    Some(NAME_LABEL),
                ],
            ),
            Rule::bygroups(
                r"(?m)^( *)([0-9]+[.)])( \[@[0-9]+\])?",
                vec![Some(WHITESPACE), Some(KEYWORD), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups_g(
                r"(?m)(?i)^( *#\+begin: *)((?:.|\n)*?)(^ *#\+end: *$)",
                vec![
                    Some(GroupAction::Token(OPERATOR_WORD)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(OPERATOR_WORD)),
                ],
            ),
            Rule::bygroups(
                r"(?m)(?i)^( *#\+begin_comment *\n)((?:.|\n)*?)(^ *#\+end_comment *$)",
                vec![
                    Some(OPERATOR_WORD),
                    Some(COMMENT_MULTILINE),
                    Some(OPERATOR_WORD),
                ],
            ),
            Rule::bygroups(
                r"(?m)(?i)^( *#\+begin_src .*)((?:.|\n)*?)(^ *#\+end_src *$)",
                vec![Some(OPERATOR_WORD), Some(TEXT), Some(OPERATOR_WORD)],
            ),
            Rule::bygroups(
                r"(?m)(?i)^( *#\+begin_\w+)( *\n)((?:.|\n)*?)(^ *#\+end_\w+)( *$)",
                vec![
                    Some(OPERATOR_WORD),
                    Some(WHITESPACE),
                    Some(TEXT),
                    Some(OPERATOR_WORD),
                    Some(WHITESPACE),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(#\+\w+:)(.*)$",
                vec![Some(NAME_NAMESPACE), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(?i)^( *:\w+: *\n)((?:.|\n)*?)(^ *:end: *$)",
                vec![
                    Some(NAME_DECORATOR),
                    Some(COMMENT_SPECIAL),
                    Some(NAME_DECORATOR),
                ],
            ),
            Rule::token(r"(?m)\\\\$", OPERATOR),
            Rule::token_to(
                r"(?m)^\s*CLOSED:\s+",
                GENERIC_DELETED,
                NewState::Push(vec![r"dateline"]),
            ),
            Rule::token_to(
                r"(?m)^\s*(?:DEADLINE:|SCHEDULED:)\s+",
                GENERIC_ERROR,
                NewState::Push(vec![r"dateline"]),
            ),
            Rule::token(r"(?m)(?<!\w)\*(.|\n(?!\n))+?\*+(?!\w)", GENERIC_STRONG),
            Rule::token(r"(?m)(?<!\w)/(.|\n(?!\n))+?/(?!\w)", GENERIC_EMPH),
            Rule::token(r"(?m)(?<!\w)=(.|\n(?!\n))+?=(?!\w)", STRING),
            Rule::token(r"(?m)(?<!\w)~(.|\n(?!\n))+?~(?!\w)", STRING),
            Rule::token(r"(?m)(?<!\w)\+(.|\n(?!\n))+?\+(?!\w)", GENERIC_DELETED),
            Rule::token(
                r"(?m)(?<!\w)_(.|\n(?!\n))+?_+(?!\w)",
                TokenType::new(&["Generic", "EmphStrong"]),
            ),
            Rule::token(r"(?m)<.+?>", LITERAL_DATE),
            Rule::token(r"(?m)\{\{\{.+?\}\}\}", COMMENT_PREPROC),
            Rule::token(r"(?m)(?<!\[)\[fn:.+?\]", NAME_TAG),
            Rule::bygroups(
                r"(?m)(?s)(\[\[)(.*?)(\]\[)(.*?)(\]\])",
                vec![
                    Some(PUNCTUATION),
                    Some(NAME_ATTRIBUTE),
                    Some(PUNCTUATION),
                    Some(NAME_TAG),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups(
                r"(?m)(?s)(\[\[)(.+?)(\]\])",
                vec![Some(PUNCTUATION), Some(NAME_ATTRIBUTE), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)(<<)(.+?)(>>)",
                vec![Some(PUNCTUATION), Some(NAME_ATTRIBUTE), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)^( *)(\|[ -].*?[ -]\|)$",
                vec![Some(WHITESPACE), Some(STRING)],
            ),
            Rule::token(r"(?m)[^#*+\-0-9:\\/=~_<{\[|\n]+", TEXT),
            Rule::token(r"(?m)[#*+\-0-9:\\/=~_<{\[|\n]", TEXT),
        ],
    );
    m.insert(
        r"dateline",
        vec![
            Rule::token(r"(?m)\s*CLOSED:\s+", GENERIC_DELETED),
            Rule::token(r"(?m)\s*(?:DEADLINE:|SCHEDULED:)\s+", GENERIC_ERROR),
            Rule::token(r"(?m)\[.+?\]", LITERAL_DATE),
            Rule::token(r"(?m)<[^>]+?>", LITERAL_DATE),
            Rule::token_to(r"(?m)(\s*)$", TEXT, NewState::Pop(1)),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for OrgLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
