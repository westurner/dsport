#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.pddl:PddlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.pddl:PddlLexer:pddl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pddl
pub struct PddlLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m);.*$", TokenType::new(&["Comment", "Singleline"])),
        Rule::token(r"(?m)(:(?:a(?:ction(?:(?:\-costs)?)|dl|gent)|con(?:dition(?:(?:al\-effects)?)|(?:st(?:(?:a|rai)n)|tinuous\-effec)ts)|d(?:erived(?:(?:\-predicates)?)|isjunctive\-preconditions|omain|urati(?:on|ve\-action(?:(?:s)?)))|e(?:ffect|quality|xistential\-preconditions)|f(?:actored\-privacy|(?:luent|unction)s)|goal|init|length|m(?:etric|ulti\-agent)|n(?:egative\-preconditions|on\-deterministic|umeric\-fluents)|object(?:(?:(?:\-fluent)?)s)|p(?:ara(?:llel|meters)|re(?:condition|(?:dicat|ferenc)es))|requirements|s(?:erial|trips)|t(?:ime\-intial\-literals|yp(?:es|ing))|un(?:factored\-privacy|iversal\-preconditions)))\b", KEYWORD),
        Rule::token(r"(?m)(a(?:l(?:l|ways(?:(?:\-within)?))|nd|ssign|t(?:(?:\-most\-once)?))|d(?:e(?:(?:creas|fin)e)|omain)|e(?:ither|nd|xists)|forall|hold\-(?:after|during)|i(?:mply|ncrease|s\-violated)|m(?:(?:ax|in)imize)|not|o(?:bject|(?:(?:ve)?)r)|pr(?:eference|oblem)|s(?:cale\-(?:down|up)|ometime(?:(?:\-(?:after|before))?)|tart)|total\-time|w(?:(?:he|ithi)n))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[()]", PUNCTUATION),
        Rule::token(r"(?m)[=/*+><-]", OPERATOR),
        Rule::token(r"(?m)[a-zA-Z][a-zA-Z0-9_-]*", NAME),
        Rule::token(r"(?m)\?[a-zA-Z][a-zA-Z0-9_-]*", NAME_VARIABLE),
        Rule::token(r"(?m)[0-9]+\.[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(:(?:a(?:ction(?:(?:\-costs)?)|dl|gent)|con(?:dition(?:(?:al\-effects)?)|(?:st(?:(?:a|rai)n)|tinuous\-effec)ts)|d(?:erived(?:(?:\-predicates)?)|isjunctive\-preconditions|omain|urati(?:on|ve\-action(?:(?:s)?)))|e(?:ffect|quality|xistential\-preconditions)|f(?:actored\-privacy|(?:luent|unction)s)|goal|init|length|m(?:etric|ulti\-agent)|n(?:egative\-preconditions|on\-deterministic|umeric\-fluents)|object(?:(?:(?:\-fluent)?)s)|p(?:ara(?:llel|meters)|re(?:condition|(?:dicat|ferenc)es))|requirements|s(?:erial|trips)|t(?:ime\-intial\-literals|yp(?:es|ing))|un(?:factored\-privacy|iversal\-preconditions)))\b", KEYWORD),
    ]);
    m.insert(r"builtins", vec![
        Rule::token(r"(?m)(a(?:l(?:l|ways(?:(?:\-within)?))|nd|ssign|t(?:(?:\-most\-once)?))|d(?:e(?:(?:creas|fin)e)|omain)|e(?:ither|nd|xists)|forall|hold\-(?:after|during)|i(?:mply|ncrease|s\-violated)|m(?:(?:ax|in)imize)|not|o(?:bject|(?:(?:ve)?)r)|pr(?:eference|oblem)|s(?:cale\-(?:down|up)|ometime(?:(?:\-(?:after|before))?)|tart)|total\-time|w(?:(?:he|ithi)n))\b", NAME_BUILTIN),
    ]);
    Table(m)
}

impl Lexer for PddlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
