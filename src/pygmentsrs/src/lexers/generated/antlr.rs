//! AUTO-GENERATED from `pygments.pygments.lexers.parsers:AntlrLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.parsers:AntlrLexer:antlr

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: antlr
pub struct AntlrLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"whitespace", vec![Rule::token(r"(?m)\s+", WHITESPACE)]);
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)//.*$", COMMENT),
            Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
        ],
    );
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)//.*$", COMMENT),
            Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
            Rule::bygroups(
                r"(?m)(lexer|parser|tree)?(\s*)(grammar\b)(\s*)([A-Za-z]\w*)(;)",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(NAME_CLASS),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::token_to(r"(?m)options\b", KEYWORD, NewState::Push(vec![r"options"])),
            Rule::token_to(r"(?m)tokens\b", KEYWORD, NewState::Push(vec![r"tokens"])),
            Rule::bygroups_to(
                r"(?m)(scope)(\s*)([A-Za-z]\w*)(\s*)(\{)",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(NAME_VARIABLE),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"action"]),
            ),
            Rule::token_to(
                r"(?m)(catch|finally)\b",
                KEYWORD,
                NewState::Push(vec![r"exception"]),
            ),
            Rule::bygroups_to(
                r"(?m)(@[A-Za-z]\w*)(\s*)(::)?(\s*)([A-Za-z]\w*)(\s*)(\{)",
                vec![
                    Some(NAME_LABEL),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(NAME_LABEL),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"action"]),
            ),
            Rule::bygroups_to(
                r"(?m)((?:protected|private|public|fragment)\b)?(\s*)([A-Za-z]\w*)(!)?",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(NAME_LABEL),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"rule-alts", r"rule-prelims"]),
            ),
        ],
    );
    m.insert(
        r"exception",
        vec![
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
            Rule::token(r"(?m)\s", WHITESPACE),
            Rule::token(r"(?m)//.*$", COMMENT),
            Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"nested-arg-action"]),
            ),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"action"])),
        ],
    );
    m.insert(
        r"rule-prelims",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)//.*$", COMMENT),
            Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
            Rule::token(r"(?m)returns\b", KEYWORD),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"nested-arg-action"]),
            ),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"action"])),
            Rule::bygroups(
                r"(?m)(throws)(\s+)([A-Za-z]\w*)",
                vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_LABEL)],
            ),
            Rule::bygroups(
                r"(?m)(,)(\s*)([A-Za-z]\w*)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_LABEL)],
            ),
            Rule::token_to(r"(?m)options\b", KEYWORD, NewState::Push(vec![r"options"])),
            Rule::bygroups_to(
                r"(?m)(scope)(\s+)(\{)",
                vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Push(vec![r"action"]),
            ),
            Rule::bygroups(
                r"(?m)(scope)(\s+)([A-Za-z]\w*)(\s*)(;)",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(NAME_LABEL),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_to(
                r"(?m)(@[A-Za-z]\w*)(\s*)(\{)",
                vec![Some(NAME_LABEL), Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Push(vec![r"action"]),
            ),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"rule-alts",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)//.*$", COMMENT),
            Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
            Rule::token_to(r"(?m)options\b", KEYWORD, NewState::Push(vec![r"options"])),
            Rule::token(r"(?m):", PUNCTUATION),
            Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?m)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?m)<<([^>]|>[^>])>>", STRING),
            Rule::token(r"(?m)\$?[A-Z_]\w*", NAME_CONSTANT),
            Rule::token(r"(?m)\$?[a-z_]\w*", NAME_VARIABLE),
            Rule::token(
                r"(?m)(\+|\||->|=>|=|\(|\)|\.\.|\.|\?|\*|\^|!|\#|~)",
                OPERATOR,
            ),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"nested-arg-action"]),
            ),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"action"])),
            Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"tokens",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)//.*$", COMMENT),
            Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
            Rule::token(r"(?m)\{", PUNCTUATION),
            Rule::bygroups(
                r"(?m)([A-Z]\w*)(\s*)(=)?(\s*)(\'(?:\\\\|\\\'|[^\']*)\')?(\s*)(;)",
                vec![
                    Some(NAME_LABEL),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(STRING),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(r"options", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//.*$", COMMENT),
        Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
        Rule::token(r"(?m)\{", PUNCTUATION),
        Rule::bygroups(r"(?m)([A-Za-z]\w*)(\s*)(=)(\s*)([A-Za-z]\w*|\'(?:\\\\|\\\'|[^\']*)\'|[0-9]+|\*)(\s*)(;)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(TEXT), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"action", vec![
        Rule::token(r#"(?m)([^${}\'"/\\]+|"(\\\\|\\[^\\]|[^"\\])*"|'(\\\\|\\[^\\]|[^'\\])*'|//.*$\n?|/\*(.|\n)*?\*/|/(?!\*)(\\\\|\\[^\\]|[^/\\])*/|\\(?!%)|/)+"#, OTHER),
        Rule::bygroups(r"(?m)(\\)(%)", vec![Some(PUNCTUATION), Some(OTHER)]),
        Rule::bygroups(r"(?m)(\$[a-zA-Z]+)(\.?)(text|value)?", vec![Some(NAME_VARIABLE), Some(PUNCTUATION), Some(NAME_PROPERTY)]),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"nested-arg-action", vec![
        Rule::token(r#"(?m)([^$\[\]\'"/]+|"(\\\\|\\[^\\]|[^"\\])*"|'(\\\\|\\[^\\]|[^'\\])*'|//.*$\n?|/\*(.|\n)*?\*/|/(?!\*)(\\\\|\\[^\\]|[^/\\])*/|/)+"#, OTHER),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::bygroups(r"(?m)(\$[a-zA-Z]+)(\.?)(text|value)?", vec![Some(NAME_VARIABLE), Some(PUNCTUATION), Some(NAME_PROPERTY)]),
        Rule::token(r"(?m)(\\\\|\\\]|\\\[|[^\[\]])+", OTHER),
    ]);
    Table(m)
}

impl Lexer for AntlrLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
