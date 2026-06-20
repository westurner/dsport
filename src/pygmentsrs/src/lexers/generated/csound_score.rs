//! AUTO-GENERATED from `pygments.pygments.lexers.csound:CsoundScoreLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.csound:CsoundScoreLexer:csound_score

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: csound-score, csound-sco
pub struct CsoundScoreLexer;

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
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::bygroups_to(
                r"(?m)(\$[A-Z_a-z]\w*\.?)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"macro parameter value list"]),
            ),
            Rule::token(r"(?m)\$[A-Z_a-z]\w*(?:\.|\b)", COMMENT_PREPROC),
            Rule::token(
                r"(?m)#(?:e(?:nd(?:if)?|lse)\b|##)|@@?[ \t]*\d+",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?m)#includestr",
                COMMENT_PREPROC,
                NewState::Push(vec![r"includestr directive"]),
            ),
            Rule::token_to(
                r"(?m)#include",
                COMMENT_PREPROC,
                NewState::Push(vec![r"include directive"]),
            ),
            Rule::token_to(
                r"(?m)#[ \t]*define",
                COMMENT_PREPROC,
                NewState::Push(vec![r"define directive"]),
            ),
            Rule::token_to(
                r"(?m)#(?:ifn?def|undef)\b",
                COMMENT_PREPROC,
                NewState::Push(vec![r"macro directive"]),
            ),
            Rule::token(r"(?m)[aBbCdefiqstvxy]", KEYWORD),
            Rule::token(r"(?m)z", KEYWORD_CONSTANT),
            Rule::bygroups(
                r"(?m)([nNpP][pP])(\d+)",
                vec![Some(KEYWORD), Some(NUMBER_INTEGER)],
            ),
            Rule::token_to(
                r"(?m)[mn]",
                KEYWORD,
                NewState::Push(vec![r"mark statement"]),
            ),
            Rule::token(
                r"(?m)\d+[Ee][+-]?\d+|(\d+\.\d*|\d*\.\d+)([Ee][+-]?\d+)?",
                NUMBER_FLOAT,
            ),
            Rule::bygroups(
                r"(?m)(0[Xx])([0-9A-Fa-f]+)",
                vec![Some(KEYWORD_TYPE), Some(NUMBER_HEX)],
            ),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)[!+\-*/^%&|<>#~.]", OPERATOR),
            Rule::token(r"(?m)[()\[\]]", PUNCTUATION),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"quoted string"])),
            Rule::token_to(
                r"(?m)\{",
                COMMENT_PREPROC,
                NewState::Push(vec![r"loop after left brace"]),
            ),
        ],
    );
    m.insert(
        r"whitespace and macro uses",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::bygroups_to(
                r"(?m)(\$[A-Z_a-z]\w*\.?)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"macro parameter value list"]),
            ),
            Rule::token(r"(?m)\$[A-Z_a-z]\w*(?:\.|\b)", COMMENT_PREPROC),
        ],
    );
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        ],
    );
    m.insert(
        r"macro uses",
        vec![
            Rule::bygroups_to(
                r"(?m)(\$[A-Z_a-z]\w*\.?)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"macro parameter value list"]),
            ),
            Rule::token(r"(?m)\$[A-Z_a-z]\w*(?:\.|\b)", COMMENT_PREPROC),
        ],
    );
    m.insert(
        r"preprocessor directives",
        vec![
            Rule::token(
                r"(?m)#(?:e(?:nd(?:if)?|lse)\b|##)|@@?[ \t]*\d+",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?m)#includestr",
                COMMENT_PREPROC,
                NewState::Push(vec![r"includestr directive"]),
            ),
            Rule::token_to(
                r"(?m)#include",
                COMMENT_PREPROC,
                NewState::Push(vec![r"include directive"]),
            ),
            Rule::token_to(
                r"(?m)#[ \t]*define",
                COMMENT_PREPROC,
                NewState::Push(vec![r"define directive"]),
            ),
            Rule::token_to(
                r"(?m)#(?:ifn?def|undef)\b",
                COMMENT_PREPROC,
                NewState::Push(vec![r"macro directive"]),
            ),
        ],
    );
    m.insert(
        r"numbers",
        vec![
            Rule::token(
                r"(?m)\d+[Ee][+-]?\d+|(\d+\.\d*|\d*\.\d+)([Ee][+-]?\d+)?",
                NUMBER_FLOAT,
            ),
            Rule::bygroups(
                r"(?m)(0[Xx])([0-9A-Fa-f]+)",
                vec![Some(KEYWORD_TYPE), Some(NUMBER_HEX)],
            ),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"mark statement",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::bygroups_to(
                r"(?m)(\$[A-Z_a-z]\w*\.?)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"macro parameter value list"]),
            ),
            Rule::token(r"(?m)\$[A-Z_a-z]\w*(?:\.|\b)", COMMENT_PREPROC),
            Rule::token(r"(?m)[A-Z_a-z]\w*", NAME_LABEL),
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"loop after left brace",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::bygroups_to(
                r"(?m)(\$[A-Z_a-z]\w*\.?)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"macro parameter value list"]),
            ),
            Rule::token(r"(?m)\$[A-Z_a-z]\w*(?:\.|\b)", COMMENT_PREPROC),
            Rule::token_to(
                r"(?m)\d+",
                NUMBER_INTEGER,
                NewState::Push(vec![r"#pop", r"loop after repeat count"]),
            ),
        ],
    );
    m.insert(
        r"loop after repeat count",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::bygroups_to(
                r"(?m)(\$[A-Z_a-z]\w*\.?)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"macro parameter value list"]),
            ),
            Rule::token(r"(?m)\$[A-Z_a-z]\w*(?:\.|\b)", COMMENT_PREPROC),
            Rule::token_to(
                r"(?m)[A-Z_a-z]\w*",
                COMMENT_PREPROC,
                NewState::Push(vec![r"#pop", r"loop"]),
            ),
        ],
    );
    m.insert(
        r"loop",
        vec![
            Rule::token_to(r"(?m)\}", COMMENT_PREPROC, NewState::Pop(1)),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::bygroups_to(
                r"(?m)(\$[A-Z_a-z]\w*\.?)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"macro parameter value list"]),
            ),
            Rule::token(r"(?m)\$[A-Z_a-z]\w*(?:\.|\b)", COMMENT_PREPROC),
            Rule::token(
                r"(?m)#(?:e(?:nd(?:if)?|lse)\b|##)|@@?[ \t]*\d+",
                COMMENT_PREPROC,
            ),
            Rule::token_to(
                r"(?m)#includestr",
                COMMENT_PREPROC,
                NewState::Push(vec![r"includestr directive"]),
            ),
            Rule::token_to(
                r"(?m)#include",
                COMMENT_PREPROC,
                NewState::Push(vec![r"include directive"]),
            ),
            Rule::token_to(
                r"(?m)#[ \t]*define",
                COMMENT_PREPROC,
                NewState::Push(vec![r"define directive"]),
            ),
            Rule::token_to(
                r"(?m)#(?:ifn?def|undef)\b",
                COMMENT_PREPROC,
                NewState::Push(vec![r"macro directive"]),
            ),
            Rule::token(r"(?m)[aBbCdefiqstvxy]", KEYWORD),
            Rule::token(r"(?m)z", KEYWORD_CONSTANT),
            Rule::bygroups(
                r"(?m)([nNpP][pP])(\d+)",
                vec![Some(KEYWORD), Some(NUMBER_INTEGER)],
            ),
            Rule::token_to(
                r"(?m)[mn]",
                KEYWORD,
                NewState::Push(vec![r"mark statement"]),
            ),
            Rule::token(
                r"(?m)\d+[Ee][+-]?\d+|(\d+\.\d*|\d*\.\d+)([Ee][+-]?\d+)?",
                NUMBER_FLOAT,
            ),
            Rule::bygroups(
                r"(?m)(0[Xx])([0-9A-Fa-f]+)",
                vec![Some(KEYWORD_TYPE), Some(NUMBER_HEX)],
            ),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)[!+\-*/^%&|<>#~.]", OPERATOR),
            Rule::token(r"(?m)[()\[\]]", PUNCTUATION),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"quoted string"])),
            Rule::token_to(
                r"(?m)\{",
                COMMENT_PREPROC,
                NewState::Push(vec![r"loop after left brace"]),
            ),
        ],
    );
    m.insert(
        r"braced string",
        vec![
            Rule::token_to(r"(?m)\}\}", STRING, NewState::Pop(1)),
            Rule::token(r"(?m)[^}]|\}(?!\})", STRING),
        ],
    );
    m.insert(
        r"include directive",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::token_to(r"(?m)([^ \t]).*?\1", STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"includestr directive",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::token_to(
                r#"(?m)""#,
                STRING,
                NewState::Push(vec![r"#pop", r"quoted string"]),
            ),
        ],
    );
    m.insert(
        r"define directive",
        vec![
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::bygroups_to(
                r"(?m)([A-Z_a-z]\w*)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"#pop", r"macro parameter name list"]),
            ),
            Rule::token_to(
                r"(?m)[A-Z_a-z]\w*",
                COMMENT_PREPROC,
                NewState::Push(vec![r"#pop", r"before macro body"]),
            ),
        ],
    );
    m.insert(
        r"macro parameter name list",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::token(r"(?m)[A-Z_a-z]\w*", COMMENT_PREPROC),
            Rule::token(r"(?m)['#]", PUNCTUATION),
            Rule::token_to(
                r"(?m)\)",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"before macro body"]),
            ),
        ],
    );
    m.insert(
        r"before macro body",
        vec![
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::token_to(
                r"(?m)#",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"macro body"]),
            ),
        ],
    );
    m.insert(
        r"macro body",
        vec![
            Rule::token(r"(?m)(?:\\(?!#)|[^#\\]|\n)+", COMMENT_PREPROC),
            Rule::token(r"(?m)\\#", COMMENT_PREPROC),
            Rule::token_to(r"(?m)(?<!\\)#", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"macro directive",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)/[*](?:.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::token_to(r"(?m)[A-Z_a-z]\w*", COMMENT_PREPROC, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"macro parameter value list",
        vec![
            Rule::token(r#"(?m)(?:[^\'#"{()]|\{(?!\{))+"#, COMMENT_PREPROC),
            Rule::token(r"(?m)['#]", PUNCTUATION),
            Rule::token_to(
                r#"(?m)""#,
                STRING,
                NewState::Push(vec![r"macro parameter value quoted string"]),
            ),
            Rule::token_to(
                r"(?m)\{\{",
                STRING,
                NewState::Push(vec![r"macro parameter value braced string"]),
            ),
            Rule::token_to(
                r"(?m)\(",
                COMMENT_PREPROC,
                NewState::Push(vec![r"macro parameter value parenthetical"]),
            ),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"macro parameter value quoted string",
        vec![
            Rule::token(r"(?m)\\[#'()]", COMMENT_PREPROC),
            Rule::token(r"(?m)[#'()]", ERROR),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"$]+"#, STRING),
            Rule::bygroups_to(
                r"(?m)(\$[A-Z_a-z]\w*\.?)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"macro parameter value list"]),
            ),
            Rule::token(r"(?m)\$[A-Z_a-z]\w*(?:\.|\b)", COMMENT_PREPROC),
            Rule::token(r"(?m)[$]", STRING),
        ],
    );
    m.insert(
        r"quoted string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"$]+"#, STRING),
            Rule::bygroups_to(
                r"(?m)(\$[A-Z_a-z]\w*\.?)(\()",
                vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)],
                NewState::Push(vec![r"macro parameter value list"]),
            ),
            Rule::token(r"(?m)\$[A-Z_a-z]\w*(?:\.|\b)", COMMENT_PREPROC),
            Rule::token(r"(?m)[$]", STRING),
        ],
    );
    m.insert(
        r"macro parameter value braced string",
        vec![
            Rule::token(r"(?m)\\[#'()]", COMMENT_PREPROC),
            Rule::token(r"(?m)[#'()]", ERROR),
            Rule::token_to(r"(?m)\}\}", STRING, NewState::Pop(1)),
            Rule::token(r"(?m)[^}]|\}(?!\})", STRING),
        ],
    );
    m.insert(
        r"macro parameter value parenthetical",
        vec![
            Rule::token(r"(?m)(?:[^\\()]|\\\))+", COMMENT_PREPROC),
            Rule::token_to(r"(?m)\(", COMMENT_PREPROC, NewState::PushSame),
            Rule::token_to(r"(?m)\)", COMMENT_PREPROC, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for CsoundScoreLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
