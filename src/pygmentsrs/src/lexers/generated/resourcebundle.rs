//! AUTO-GENERATED from `pygments.pygments.lexers.resource:ResourceLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.resource:ResourceLexer:resourcebundle

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: resourcebundle, resource
pub struct ResourcebundleLexer;

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
            Rule::token(r"(?im)//.*?$", COMMENT),
            Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token(r"(?im)-?\d+", NUMBER_INTEGER),
            Rule::token(r"(?im)[,{}]", OPERATOR),
            Rule::bygroups(
                r"(?im)([^\s{:]+)(\s*)(:table|:array|:string|:bin|:import|:intvector|:int|:alias?)",
                vec![Some(NAME), Some(TEXT), Some(KEYWORD)],
            ),
            Rule::token(r"(?im)\s+", TEXT),
            Rule::token(
                r"(?im)(:(?:a(?:lias|rray)|bin|i(?:mport|nt(?:(?:vector)?))|string|table))",
                KEYWORD,
            ),
        ],
    );
    m.insert(r"string", vec![
        Rule::token(r#"(?im)(\\x[0-9a-f]{2}|\\u[0-9a-f]{4}|\\U00[0-9a-f]{6}|\\[0-7]{1,3}|\\c.|\\[abtnvfre\'"?\\]|\\\{|[^"{\\])+"#, STRING),
        Rule::token_to(r"(?im)\{", STRING_ESCAPE, NewState::Push(vec![r"msgname"])),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Pop(1)),
    ]);
    m.insert(
        r"msgname",
        vec![Rule::bygroups_to(
            r"(?im)([^{},]+)(\s*)",
            vec![Some(NAME), Some(STRING_ESCAPE)],
            NewState::Push(vec![r"#pop", r"message"]),
        )],
    );
    m.insert(
        r"message",
        vec![
            Rule::token_to(r"(?im)\{", STRING_ESCAPE, NewState::Push(vec![r"msgname"])),
            Rule::token_to(r"(?im)\}", STRING_ESCAPE, NewState::Pop(1)),
            Rule::bygroups_to(
                r"(?im)(,)(\s*)([a-z]+)(\s*\})",
                vec![
                    Some(OPERATOR),
                    Some(STRING_ESCAPE),
                    Some(KEYWORD),
                    Some(STRING_ESCAPE),
                ],
                NewState::Pop(1),
            ),
            Rule::bygroups_to(
                r"(?im)(,)(\s*)([a-z]+)(\s*)(,)(\s*)(offset)(\s*)(:)(\s*)(-?\d+)(\s*)",
                vec![
                    Some(OPERATOR),
                    Some(STRING_ESCAPE),
                    Some(KEYWORD),
                    Some(STRING_ESCAPE),
                    Some(OPERATOR),
                    Some(STRING_ESCAPE),
                    Some(OPERATOR_WORD),
                    Some(STRING_ESCAPE),
                    Some(OPERATOR),
                    Some(STRING_ESCAPE),
                    Some(NUMBER_INTEGER),
                    Some(STRING_ESCAPE),
                ],
                NewState::Push(vec![r"choice"]),
            ),
            Rule::bygroups_to(
                r"(?im)(,)(\s*)([a-z]+)(\s*)(,)(\s*)",
                vec![
                    Some(OPERATOR),
                    Some(STRING_ESCAPE),
                    Some(KEYWORD),
                    Some(STRING_ESCAPE),
                    Some(OPERATOR),
                    Some(STRING_ESCAPE),
                ],
                NewState::Push(vec![r"choice"]),
            ),
            Rule::token(r"(?im)\s+", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"choice",
        vec![
            Rule::bygroups_to(
                r"(?im)(=|<|>|<=|>=|!=)(-?\d+)(\s*\{)",
                vec![Some(OPERATOR), Some(NUMBER_INTEGER), Some(STRING_ESCAPE)],
                NewState::Push(vec![r"message"]),
            ),
            Rule::bygroups_to(
                r"(?im)([a-z]+)(\s*\{)",
                vec![Some(KEYWORD_TYPE), Some(STRING_ESCAPE)],
                NewState::Push(vec![r"str"]),
            ),
            Rule::token_to(
                r"(?im)\}",
                STRING_ESCAPE,
                NewState::Push(vec![r"#pop", r"#pop"]),
            ),
            Rule::token(r"(?im)\s+", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"str",
        vec![
            Rule::token_to(r"(?im)\}", STRING_ESCAPE, NewState::Pop(1)),
            Rule::token_to(r"(?im)\{", STRING_ESCAPE, NewState::Push(vec![r"msgname"])),
            Rule::token(r"(?im)[^{}]+", STRING),
        ],
    );
    Table(m)
}

impl Lexer for ResourcebundleLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
