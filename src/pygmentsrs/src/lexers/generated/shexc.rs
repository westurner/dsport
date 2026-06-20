#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.rdf:ShExCLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.rdf:ShExCLexer:shexc

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: shexc, shex
pub struct ShexcLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?i)(base|prefix|start|external|literal|iri|bnode|nonliteral|length|minlength|maxlength|mininclusive|minexclusive|maxinclusive|maxexclusive|totaldigits|fractiondigits|closed|extra)\b", KEYWORD),
        Rule::token(r"(?m)(a)\b", KEYWORD),
        Rule::token(r#"(?m)(<(?:[^\x00-\x20<>"{}|^`\\]|\\(?:u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8}))*>)"#, NAME_LABEL),
        Rule::token(r"(?m)(_:[0-9a-zA-ZÀ-ÖØ-öø-˿Ͱ-ͽͿ-῿‌-‍⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�_](?:[a-zA-ZÀ-ÖØ-öø-˿Ͱ-ͽͿ-῿‌-‍⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�_\-0-9·̀-ͯ‿-⁀.]*[a-zA-ZÀ-ÖØ-öø-˿Ͱ-ͽͿ-῿‌-‍⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�_\-0-9·̀-ͯ‿-⁀])?)", NAME_LABEL),
        Rule::bygroups(r"(?m)([a-zA-ZÀ-ÖØ-öø-˿Ͱ-ͽͿ-῿‌-‍⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�](?:[a-zA-ZÀ-ÖØ-öø-˿Ͱ-ͽͿ-῿‌-‍⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�_\-0-9·̀-ͯ‿-⁀.]*[a-zA-ZÀ-ÖØ-öø-˿Ͱ-ͽͿ-῿‌-‍⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�_\-0-9·̀-ͯ‿-⁀])?)?(\:)((?:[a-zA-ZÀ-ÖØ-öø-˿Ͱ-ͽͿ-῿‌-‍⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�_:0-9]|(?:%[0-9A-Fa-f][0-9A-Fa-f])|(?:\\[_~.\-!$&'()*+,;=/?#@%]))(?:(?:[a-zA-ZÀ-ÖØ-öø-˿Ͱ-ͽͿ-῿‌-‍⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�_\-0-9·̀-ͯ‿-⁀.:]|(?:%[0-9A-Fa-f][0-9A-Fa-f])|(?:\\[_~.\-!$&'()*+,;=/?#@%]))*(?:[a-zA-ZÀ-ÖØ-öø-˿Ͱ-ͽͿ-῿‌-‍⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�_\-0-9·̀-ͯ‿-⁀:]|(?:%[0-9A-Fa-f][0-9A-Fa-f])|(?:\\[_~.\-!$&'()*+,;=/?#@%])))?)?", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_TAG)]),
        Rule::token(r"(?m)(true|false)", KEYWORD_CONSTANT),
        Rule::token(r"(?m)[+\-]?(\d+\.\d*[eE][+-]?\d+|\.?\d+[eE][+-]?\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)[+\-]?(\d+\.\d*|\.\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)[+\-]?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[@|$&=*+?^\-~]", OPERATOR),
        Rule::token(r"(?m)(?i)(and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)[(){}.;,:^\[\]]", PUNCTUATION),
        Rule::token(r"(?m)#[^\n]*", COMMENT),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Push(vec![r"triple-double-quoted-string"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"single-double-quoted-string"])),
        Rule::token_to(r"(?m)'''", STRING, NewState::Push(vec![r"triple-single-quoted-string"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"single-single-quoted-string"])),
    ]);
    m.insert(
        r"triple-double-quoted-string",
        vec![
            Rule::token_to(r#"(?m)""""#, STRING, NewState::Push(vec![r"end-of-string"])),
            Rule::token(r"(?m)[^\\]+", STRING),
            Rule::token_to(r"(?m)\\", STRING, NewState::Push(vec![r"string-escape"])),
        ],
    );
    m.insert(
        r"single-double-quoted-string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"end-of-string"])),
            Rule::token(r#"(?m)[^"\\\n]+"#, STRING),
            Rule::token_to(r"(?m)\\", STRING, NewState::Push(vec![r"string-escape"])),
        ],
    );
    m.insert(
        r"triple-single-quoted-string",
        vec![
            Rule::token_to(r"(?m)'''", STRING, NewState::Push(vec![r"end-of-string"])),
            Rule::token(r"(?m)[^\\]+", STRING),
            Rule::token_to(
                r"(?m)\\",
                STRING_ESCAPE,
                NewState::Push(vec![r"string-escape"]),
            ),
        ],
    );
    m.insert(
        r"single-single-quoted-string",
        vec![
            Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"end-of-string"])),
            Rule::token(r"(?m)[^'\\\n]+", STRING),
            Rule::token_to(r"(?m)\\", STRING, NewState::Push(vec![r"string-escape"])),
        ],
    );
    m.insert(
        r"string-escape",
        vec![
            Rule::token_to(
                r"(?m)(?:u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})",
                STRING_ESCAPE,
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m).", STRING_ESCAPE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"end-of-string",
        vec![
            Rule::bygroups_to(
                r"(?m)(@)([a-zA-Z]+(?:-[a-zA-Z0-9]+)*)",
                vec![Some(OPERATOR), Some(NAME_FUNCTION)],
                NewState::Pop(2),
            ),
            Rule::token_to(r"(?m)\^\^", OPERATOR, NewState::Pop(2)),
            Rule::default(NewState::Pop(2)),
        ],
    );
    Table(m)
}

impl Lexer for ShexcLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
