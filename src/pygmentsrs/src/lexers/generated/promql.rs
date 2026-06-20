#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.promql:PromQLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.promql:PromQLLexer:promql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: promql
pub struct PromqlLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)(b(?:ool|y)|group_(?:(?:lef|righ)t)|ignoring|o(?:ffset|n)|without)\b", KEYWORD),
        Rule::token(r"(?m)(avg|bottomk|count(?:(?:_values)?)|group|m(?:ax|in)|quantile|s(?:td(?:dev|var)|um)|topk)\b", KEYWORD),
        Rule::token(r"(?m)(a(?:bs(?:(?:ent(?:(?:_over_time)?))?)|vg_over_time)|c(?:eil|hanges|lamp_m(?:ax|in)|ount_over_time)|d(?:ay(?:_of_(?:month|week)|s_in_month)|e(?:lta|riv))|exp|floor|h(?:istogram_quantile|o(?:lt_winters|ur))|i(?:delta|(?:ncreas|rat)e)|l(?:abel_(?:join|replace)|n|og(?:10|2))|m(?:ax_over_time|in(?:(?:_over_tim|ut)e)|onth)|predict_linear|quantile_over_time|r(?:ate|esets|ound)|s(?:calar|ort(?:(?:_desc)?)|qrt|(?:td(?:dev|var)|um)_over_time)|time(?:(?:stamp)?)|(?:vecto|yea)r)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)[1-9][0-9]*[smhdwy]", STRING),
        Rule::token(r"(?m)-?[0-9]+\.[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)(\+|\-|\*|\/|\%|\^)", OPERATOR),
        Rule::token(r"(?m)==|!=|>=|<=|<|>", OPERATOR),
        Rule::token(r"(?m)and|or|unless", OPERATOR_WORD),
        Rule::token(r"(?m)[_a-zA-Z][a-zA-Z0-9_]+", NAME_VARIABLE),
        Rule::bygroups(r#"(?m)(["\'])(.*?)(["\'])"#, vec![Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::token_to(r"(?m)\(", OPERATOR, NewState::Push(vec![r"function"])),
        Rule::token(r"(?m)\)", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"labels"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"range"])),
    ]);
    m.insert(
        r"labels",
        vec![
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::bygroups(
                r#"(?m)([_a-zA-Z][a-zA-Z0-9_]*?)(\s*?)(=~|!=|=|!~)(\s*?)("|\')(.*?)("|\')"#,
                vec![
                    Some(NAME_LABEL),
                    Some(WHITESPACE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(STRING),
                    Some(PUNCTUATION),
                ],
            ),
        ],
    );
    m.insert(
        r"range",
        vec![
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[1-9][0-9]*[smhdwy]", STRING),
        ],
    );
    m.insert(
        r"function",
        vec![
            Rule::token_to(r"(?m)\)", OPERATOR, NewState::Pop(1)),
            Rule::token_to(r"(?m)\(", OPERATOR, NewState::PushSame),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for PromqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
