#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.archetype:CadlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.archetype:CadlLexer:cadl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cadl
pub struct CadlLexer;

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
        r"path",
        vec![
            Rule::token(r"(?m)[a-z_]\w*", NAME_CLASS),
            Rule::token(r"(?m)/", PUNCTUATION),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"any_code"])),
            Rule::token_to(r"(?m)\s+", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)([ \t]*)(--.*)$", vec![Some(WHITESPACE), Some(COMMENT)]),
        Rule::token(r"(?m)(cardinality|existence|occurrences|group|include|exclude|allow_archetype|use_archetype|use_node)\W", KEYWORD_TYPE),
        Rule::token(r"(?m)(and|or|not|there_exists|xor|implies|for_all)\W", KEYWORD_TYPE),
        Rule::token(r"(?m)(after|before|closed)\W", KEYWORD_TYPE),
        Rule::token(r"(?m)(not)\W", OPERATOR),
        Rule::token(r"(?m)(matches|is_in)\W", OPERATOR),
        Rule::token(r"(?m)(∈|∉)", OPERATOR),
        Rule::token(r"(?m)(∃|∄|∀|∧|∨|⊻|C)", OPERATOR),
        Rule::bygroups(r"(?m)(\{)(\s*)(/[^}]+/)(\s*)(\})", vec![Some(PUNCTUATION), Some(WHITESPACE), Some(STRING_REGEX), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\{)(\s*)(\^[^}]+\^)(\s*)(\})", vec![Some(PUNCTUATION), Some(WHITESPACE), Some(STRING_REGEX), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token_to(r"(?m)/", PUNCTUATION, NewState::Push(vec![r"path"])),
        Rule::bygroups(r"(?m)(\{)((?:\d+\.\.)?(?:\d+|\*))((?:\s*;\s*(?:ordered|unordered|unique)){,2})(\})", vec![Some(PUNCTUATION), Some(NUMBER), Some(NUMBER), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\[\{", PUNCTUATION),
        Rule::token(r"(?m)\}\]", PUNCTUATION),
        Rule::token(r"(?m)\{", PUNCTUATION),
        Rule::token(r"(?m)\}", PUNCTUATION),
        Rule::bygroups_to(r"(?m)(\[)(\w[\w-]*(?:\([^)\n]+\))?)(::)", vec![Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION)], NewState::Push(vec![r"adl14_code_constraint"])),
        Rule::bygroups(r"(?m)(\d*)(\|)(\[\w[\w-]*::\w[\w-]*\])((?:[,;])?)", vec![Some(NUMBER), Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[Xx?YyMmDdHhSs\d]{2,4}([:-][Xx?YyMmDdHhSs\d]{2}){2}", LITERAL_DATE),
        Rule::token(r"(?m)(P[YyMmWwDd]+(T[HhMmSs]+)?|PT[HhMmSs]+)/?", LITERAL_DATE),
        Rule::token(r"(?m)\d{4}-\d{2}-\d{2}T?", LITERAL_DATE),
        Rule::token(r"(?m)\d{2}:\d{2}:\d{2}(\.\d+)?([+-]\d{4}|Z)?", LITERAL_DATE),
        Rule::token(r"(?m)P((\d*(\.\d+)?[YyMmWwDd]){1,3}(T(\d*(\.\d+)?[HhMmSs]){,3})?|T(\d*(\.\d+)?[HhMmSs]){,3})", LITERAL_DATE),
        Rule::token(r"(?m)[+-]?(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d*\.\d+%?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?\d+%?", NUMBER_INTEGER),
        Rule::token(r"(?m)([Tt]rue|[Ff]alse)", LITERAL),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token_to(r"(?m)[a-z][a-z0-9+.-]*:", LITERAL, NewState::Push(vec![r"uri"])),
        Rule::bygroups(r"(?m)(\[)(\w[\w-]*(?:\([^)\n]+\))?)(::)(\w[\w-]*)(\])", vec![Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION)]),
        Rule::token_to(r"(?m)\|", PUNCTUATION, NewState::Push(vec![r"interval"])),
        Rule::token(r"(?m)\.\.\.", PUNCTUATION),
        Rule::token(r"(?m)[A-Z]\w+(<[A-Z]\w+([A-Za-z_<>]*)>)?", NAME_CLASS),
        Rule::token(r"(?m)[a-z_]\w*", NAME_CLASS),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"any_code"])),
        Rule::token(r"(?m)(~|//|\\\\|\+|-|/|\*|\^|!=|=|<=|>=|<|>]?)", OPERATOR),
        Rule::token(r"(?m)\(", PUNCTUATION),
        Rule::token(r"(?m)\)", PUNCTUATION),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m);", PUNCTUATION),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?m)([ \t]*)(--.*)$",
                vec![Some(WHITESPACE), Some(COMMENT)],
            ),
        ],
    );
    m.insert(r"constraint_values", vec![
        Rule::bygroups_to(r"(?m)(\[)(\w[\w-]*(?:\([^)\n]+\))?)(::)", vec![Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION)], NewState::Push(vec![r"adl14_code_constraint"])),
        Rule::bygroups(r"(?m)(\d*)(\|)(\[\w[\w-]*::\w[\w-]*\])((?:[,;])?)", vec![Some(NUMBER), Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[Xx?YyMmDdHhSs\d]{2,4}([:-][Xx?YyMmDdHhSs\d]{2}){2}", LITERAL_DATE),
        Rule::token(r"(?m)(P[YyMmWwDd]+(T[HhMmSs]+)?|PT[HhMmSs]+)/?", LITERAL_DATE),
        Rule::token(r"(?m)\d{4}-\d{2}-\d{2}T?", LITERAL_DATE),
        Rule::token(r"(?m)\d{2}:\d{2}:\d{2}(\.\d+)?([+-]\d{4}|Z)?", LITERAL_DATE),
        Rule::token(r"(?m)P((\d*(\.\d+)?[YyMmWwDd]){1,3}(T(\d*(\.\d+)?[HhMmSs]){,3})?|T(\d*(\.\d+)?[HhMmSs]){,3})", LITERAL_DATE),
        Rule::token(r"(?m)[+-]?(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d*\.\d+%?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?\d+%?", NUMBER_INTEGER),
        Rule::token(r"(?m)([Tt]rue|[Ff]alse)", LITERAL),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token_to(r"(?m)[a-z][a-z0-9+.-]*:", LITERAL, NewState::Push(vec![r"uri"])),
        Rule::bygroups(r"(?m)(\[)(\w[\w-]*(?:\([^)\n]+\))?)(::)(\w[\w-]*)(\])", vec![Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION)]),
        Rule::token_to(r"(?m)\|", PUNCTUATION, NewState::Push(vec![r"interval"])),
        Rule::token(r"(?m)\.\.\.", PUNCTUATION),
    ]);
    m.insert(
        r"date_constraints",
        vec![
            Rule::token(
                r"(?m)[Xx?YyMmDdHhSs\d]{2,4}([:-][Xx?YyMmDdHhSs\d]{2}){2}",
                LITERAL_DATE,
            ),
            Rule::token(
                r"(?m)(P[YyMmWwDd]+(T[HhMmSs]+)?|PT[HhMmSs]+)/?",
                LITERAL_DATE,
            ),
        ],
    );
    m.insert(r"values", vec![
        Rule::token(r"(?m)\d{4}-\d{2}-\d{2}T?", LITERAL_DATE),
        Rule::token(r"(?m)\d{2}:\d{2}:\d{2}(\.\d+)?([+-]\d{4}|Z)?", LITERAL_DATE),
        Rule::token(r"(?m)P((\d*(\.\d+)?[YyMmWwDd]){1,3}(T(\d*(\.\d+)?[HhMmSs]){,3})?|T(\d*(\.\d+)?[HhMmSs]){,3})", LITERAL_DATE),
        Rule::token(r"(?m)[+-]?(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d*\.\d+%?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?\d+%?", NUMBER_INTEGER),
        Rule::token(r"(?m)([Tt]rue|[Ff]alse)", LITERAL),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token_to(r"(?m)[a-z][a-z0-9+.-]*:", LITERAL, NewState::Push(vec![r"uri"])),
        Rule::bygroups(r"(?m)(\[)(\w[\w-]*(?:\([^)\n]+\))?)(::)(\w[\w-]*)(\])", vec![Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION)]),
        Rule::token_to(r"(?m)\|", PUNCTUATION, NewState::Push(vec![r"interval"])),
        Rule::token(r"(?m)\.\.\.", PUNCTUATION),
    ]);
    m.insert(r"ordered_values", vec![
        Rule::token(r"(?m)\d{4}-\d{2}-\d{2}T?", LITERAL_DATE),
        Rule::token(r"(?m)\d{2}:\d{2}:\d{2}(\.\d+)?([+-]\d{4}|Z)?", LITERAL_DATE),
        Rule::token(r"(?m)P((\d*(\.\d+)?[YyMmWwDd]){1,3}(T(\d*(\.\d+)?[HhMmSs]){,3})?|T(\d*(\.\d+)?[HhMmSs]){,3})", LITERAL_DATE),
        Rule::token(r"(?m)[+-]?(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d*\.\d+%?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?\d+%?", NUMBER_INTEGER),
    ]);
    m.insert(r"archetype_id", vec![
        Rule::bygroups(r"(?m)([ \t]*)(([a-zA-Z]\w+(\.[a-zA-Z]\w+)*::)?[a-zA-Z]\w+(-[a-zA-Z]\w+){2}\.\w+[\w-]*\.v\d+(\.\d+){,2}((-[a-z]+)(\.\d+)?)?)", vec![Some(WHITESPACE), Some(NAME_DECORATOR)]),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"]+"#, STRING),
        Rule::token(r"(?m)\\", STRING),
    ]);
    m.insert(
        r"uri",
        vec![
            Rule::token_to(r"(?m)[,>\s]", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[^>\s,]+", LITERAL),
        ],
    );
    m.insert(r"interval", vec![
        Rule::token_to(r"(?m)\|", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)\d{4}-\d{2}-\d{2}T?", LITERAL_DATE),
        Rule::token(r"(?m)\d{2}:\d{2}:\d{2}(\.\d+)?([+-]\d{4}|Z)?", LITERAL_DATE),
        Rule::token(r"(?m)P((\d*(\.\d+)?[YyMmWwDd]){1,3}(T(\d*(\.\d+)?[HhMmSs]){,3})?|T(\d*(\.\d+)?[HhMmSs]){,3})", LITERAL_DATE),
        Rule::token(r"(?m)[+-]?(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d*\.\d+%?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?\d+%?", NUMBER_INTEGER),
        Rule::token(r"(?m)\.\.", PUNCTUATION),
        Rule::token(r"(?m)[<>=] *", PUNCTUATION),
        Rule::token(r"(?m)\+/-", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"any_code", vec![
        Rule::bygroups(r"(?m)([ \t]*)(([a-zA-Z]\w+(\.[a-zA-Z]\w+)*::)?[a-zA-Z]\w+(-[a-zA-Z]\w+){2}\.\w+[\w-]*\.v\d+(\.\d+){,2}((-[a-z]+)(\.\d+)?)?)", vec![Some(WHITESPACE), Some(NAME_DECORATOR)]),
        Rule::token(r"(?m)[a-z_]\w*[0-9.]+(@[^\]]+)?", NAME_DECORATOR),
        Rule::token(r"(?m)[a-z_]\w*", NAME_CLASS),
        Rule::token(r"(?m)[0-9]+", TEXT),
        Rule::token_to(r"(?m)\|", PUNCTUATION, NewState::Push(vec![r"code_rubric"])),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::bygroups(r"(?m)(\s*)(,)(\s*)", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)]),
    ]);
    m.insert(
        r"code_rubric",
        vec![
            Rule::token_to(r"(?m)\|", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[^|]+", STRING),
        ],
    );
    m.insert(
        r"adl14_code_constraint",
        vec![
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\|", PUNCTUATION, NewState::Push(vec![r"code_rubric"])),
            Rule::bygroups(
                r"(?m)(\w[\w-]*)([;,]?)",
                vec![Some(NAME_DECORATOR), Some(PUNCTUATION)],
            ),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?m)([ \t]*)(--.*)$",
                vec![Some(WHITESPACE), Some(COMMENT)],
            ),
        ],
    );
    Table(m)
}

impl Lexer for CadlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
