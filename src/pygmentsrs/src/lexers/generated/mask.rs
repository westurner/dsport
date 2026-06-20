//! AUTO-GENERATED from `pygments.pygments.lexers.javascript:MaskLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.javascript:MaskLexer:mask

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mask
pub struct MaskLexer;

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
            Rule::token(r"(?ims)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?ims)(//.*?)(\n)",
                vec![Some(COMMENT_SINGLE), Some(WHITESPACE)],
            ),
            Rule::token(r"(?ims)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token(r"(?ims)[{};>]", PUNCTUATION),
            Rule::token_to(
                r"(?ims)'''",
                STRING,
                NewState::Push(vec![r"string-trpl-single"]),
            ),
            Rule::token_to(
                r#"(?ims)""""#,
                STRING,
                NewState::Push(vec![r"string-trpl-double"]),
            ),
            Rule::token_to(r"(?ims)'", STRING, NewState::Push(vec![r"string-single"])),
            Rule::token_to(r#"(?ims)""#, STRING, NewState::Push(vec![r"string-double"])),
            Rule::token_to(r"(?ims)([\w-]+)", NAME_TAG, NewState::Push(vec![r"node"])),
            Rule::token_to(
                r"(?ims)([^.#;{>\s]+)",
                NAME_CLASS,
                NewState::Push(vec![r"node"]),
            ),
            Rule::token_to(
                r"(?ims)(#[\w-]+)",
                NAME_FUNCTION,
                NewState::Push(vec![r"node"]),
            ),
            Rule::token_to(
                r"(?ims)(\.[\w-]+)",
                NAME_VARIABLE_CLASS,
                NewState::Push(vec![r"node"]),
            ),
        ],
    );
    m.insert(
        r"string-base",
        vec![
            Rule::token(r"(?ims)\\.", STRING_ESCAPE),
            Rule::token_to(
                r"(?ims)~\[",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token(r"(?ims).", STRING_SINGLE),
        ],
    );
    m.insert(
        r"string-single",
        vec![
            Rule::token_to(r"(?ims)'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?ims)\\.", STRING_ESCAPE),
            Rule::token_to(
                r"(?ims)~\[",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token(r"(?ims).", STRING_SINGLE),
        ],
    );
    m.insert(
        r"string-double",
        vec![
            Rule::token_to(r#"(?ims)""#, STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?ims)\\.", STRING_ESCAPE),
            Rule::token_to(
                r"(?ims)~\[",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token(r"(?ims).", STRING_SINGLE),
        ],
    );
    m.insert(
        r"string-trpl-single",
        vec![
            Rule::token_to(r"(?ims)'''", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?ims)\\.", STRING_ESCAPE),
            Rule::token_to(
                r"(?ims)~\[",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token(r"(?ims).", STRING_SINGLE),
        ],
    );
    m.insert(
        r"string-trpl-double",
        vec![
            Rule::token_to(r#"(?ims)""""#, STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?ims)\\.", STRING_ESCAPE),
            Rule::token_to(
                r"(?ims)~\[",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token(r"(?ims).", STRING_SINGLE),
        ],
    );
    m.insert(
        r"interpolation",
        vec![
            Rule::token_to(r"(?ims)\]", STRING_INTERPOL, NewState::Pop(1)),
            Rule::bygroups_to(
                r"(?ims)(\s*)(:)",
                vec![Some(WHITESPACE), Some(STRING_INTERPOL)],
                NewState::Push(vec![r"expression"]),
            ),
            Rule::bygroups(
                r"(?ims)(\s*)(\w+)(:)",
                vec![Some(WHITESPACE), Some(NAME_OTHER), Some(PUNCTUATION)],
            ),
            Rule::token(r"(?ims)[^\]]+", STRING_INTERPOL),
        ],
    );
    m.insert(
        r"expression",
        vec![Rule::using_lexer_to(
            r"(?ims)[^\]]+",
            "javascript",
            None,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"node",
        vec![
            Rule::token(r"(?ims)\s+", WHITESPACE),
            Rule::token_to(
                r"(?ims)\.",
                NAME_VARIABLE_CLASS,
                NewState::Push(vec![r"node-class"]),
            ),
            Rule::token_to(r"(?ims)\#", NAME_FUNCTION, NewState::Push(vec![r"node-id"])),
            Rule::bygroups_to(
                r"(?ims)(style)([ \t]*)(=)",
                vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)],
                NewState::Push(vec![r"node-attr-style-value"]),
            ),
            Rule::bygroups_to(
                r"(?ims)([\w:-]+)([ \t]*)(=)",
                vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)],
                NewState::Push(vec![r"node-attr-value"]),
            ),
            Rule::token(r"(?ims)[\w:-]+", NAME_ATTRIBUTE),
            Rule::token_to(r"(?ims)[>{;]", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"node-class",
        vec![
            Rule::token(r"(?ims)[\w-]+", NAME_VARIABLE_CLASS),
            Rule::token_to(
                r"(?ims)~\[",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"node-id",
        vec![
            Rule::token(r"(?ims)[\w-]+", NAME_FUNCTION),
            Rule::token_to(
                r"(?ims)~\[",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"node-attr-value",
        vec![
            Rule::token(r"(?ims)\s+", WHITESPACE),
            Rule::token_to(r"(?ims)\w+", NAME_VARIABLE, NewState::Pop(1)),
            Rule::token_to(
                r"(?ims)'",
                STRING,
                NewState::Push(vec![r"string-single-pop2"]),
            ),
            Rule::token_to(
                r#"(?ims)""#,
                STRING,
                NewState::Push(vec![r"string-double-pop2"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"node-attr-style-value",
        vec![
            Rule::token(r"(?ims)\s+", WHITESPACE),
            Rule::token_to(
                r"(?ims)'",
                STRING_SINGLE,
                NewState::Push(vec![r"css-single-end"]),
            ),
            Rule::token_to(
                r#"(?ims)""#,
                STRING_SINGLE,
                NewState::Push(vec![r"css-double-end"]),
            ),
            Rule::token(r"(?ims)\s+", WHITESPACE),
            Rule::token_to(r"(?ims)\w+", NAME_VARIABLE, NewState::Pop(1)),
            Rule::token_to(
                r"(?ims)'",
                STRING,
                NewState::Push(vec![r"string-single-pop2"]),
            ),
            Rule::token_to(
                r#"(?ims)""#,
                STRING,
                NewState::Push(vec![r"string-double-pop2"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"css-base",
        vec![
            Rule::token(r"(?ims)\s+", WHITESPACE),
            Rule::token(r"(?ims);", PUNCTUATION),
            Rule::token(r"(?ims)[\w\-]+\s*:", NAME_BUILTIN),
        ],
    );
    m.insert(
        r"css-single-end",
        vec![
            Rule::token(r"(?ims)\s+", WHITESPACE),
            Rule::token(r"(?ims);", PUNCTUATION),
            Rule::token(r"(?ims)[\w\-]+\s*:", NAME_BUILTIN),
            Rule::token_to(r"(?ims)'", STRING_SINGLE, NewState::Pop(2)),
            Rule::token(r"(?ims)[^;']+", NAME_ENTITY),
        ],
    );
    m.insert(
        r"css-double-end",
        vec![
            Rule::token(r"(?ims)\s+", WHITESPACE),
            Rule::token(r"(?ims);", PUNCTUATION),
            Rule::token(r"(?ims)[\w\-]+\s*:", NAME_BUILTIN),
            Rule::token_to(r#"(?ims)""#, STRING_SINGLE, NewState::Pop(2)),
            Rule::token(r#"(?ims)[^;"]+"#, NAME_ENTITY),
        ],
    );
    m.insert(
        r"string-single-pop2",
        vec![
            Rule::token_to(r"(?ims)'", STRING_SINGLE, NewState::Pop(2)),
            Rule::token(r"(?ims)\\.", STRING_ESCAPE),
            Rule::token_to(
                r"(?ims)~\[",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token(r"(?ims).", STRING_SINGLE),
        ],
    );
    m.insert(
        r"string-double-pop2",
        vec![
            Rule::token_to(r#"(?ims)""#, STRING_SINGLE, NewState::Pop(2)),
            Rule::token(r"(?ims)\\.", STRING_ESCAPE),
            Rule::token_to(
                r"(?ims)~\[",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token(r"(?ims).", STRING_SINGLE),
        ],
    );
    Table(m)
}

impl Lexer for MaskLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
