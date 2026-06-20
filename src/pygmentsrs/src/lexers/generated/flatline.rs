#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:FlatlineLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:FlatlineLexer:flatline

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: flatline
pub struct FlatlineLexer;

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
        Rule::token(r"(?m)[,]+", TEXT),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)0x-?[a-f\d]+", NUMBER_HEX),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)\\(.|[a-z]+)", STRING_CHAR),
        Rule::token(r"(?m)_", STRING_SYMBOL),
        Rule::token(r"(?m)(let) ", KEYWORD),
        Rule::token(r"(?m)(!=|<=|>=|a(?:bs|cos|ll(?:(?:\-(?:but|with\-(?:defaults|numeric\-default)))?)|nd|sin|tan|vg(?:(?:\-window)?))|bin\-c(?:enter|ount)|c(?:a(?:ll|tegory\-count)|eil|o(?:n(?:d\-window|[ds])|s(?:(?:h)?)|unt))|di(?:ff\-window|v)|e(?:nsure\-(?:(?:(?:weighted\-)?)value)|poch(?:(?:\-(?:day|fields|hour|m(?:i(?:llisecond|nute)|onth)|second|weekday|year))?)|xp)|f(?:i(?:eld(?:(?:\-prop|s)?)|lter|rst)|loor)|head|i(?:nteger|[fn])|l(?:anguage|e(?:ngth|venshtein)|i(?:near\-regression|st)|n|og(?:(?:10)?))|m(?:a(?:tches(?:(?:\?)?)|ximum|[px])|d5|e(?:(?:(?:di)?)an)|i(?:n(?:(?:imum)?)|ssing(?:(?:\-count|\?|_count)?))|od(?:(?:e)?))|n(?:o(?:rmalize|t)|th)|o(?:ccurrences|r)|p(?:ercentile(?:(?:\-label)?)|o(?:pulation(?:(?:\-fraction)?)|w)|referred(?:(?:\?)?))|quantile\-label|r(?:and(?:(?:\-int|om\-value)?)|e(?:\-quote|al|place(?:(?:\-first)?)|st)|o(?:und|w\-number))|s(?:egment\-label|ha(?:1|256)|in(?:(?:h)?)|q(?:rt|uare)|t(?:andard(?:(?:[\-_])deviation)|r)|u(?:bs|m(?:(?:\-(?:squares|window)|_squares|mary(?:(?:\-(?:no|str))?))?)))|t(?:a(?:il|n(?:(?:h)?))|o\-(?:(?:degree|radian)s))|v(?:(?:arianc|ectoriz)e)|w(?:eighted\-random\-value|i(?:n(?:(?:[dn])ow)|thin\-percentiles\?))|z\-score|[*+\-<=>f]) ", NAME_BUILTIN),
        Rule::token(r"(?m)(?<=\()(?!#)[\w!$%*+<=>?/.#-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?!#)[\w!$%*+<=>?/.#-]+", NAME_VARIABLE),
        Rule::token(r"(?m)(\(|\))", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for FlatlineLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
