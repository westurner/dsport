#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.oberon:ComponentPascalLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.oberon:ComponentPascalLexer:componentpascal

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: componentpascal, cp
pub struct ComponentpascalLexer;

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
        Rule::token(r"(?ms)\n+", TEXT),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)\(\*([^$].*?)\*\)", COMMENT_MULTILINE),
        Rule::token(r"(?ms)[()\[\]{},.:;|]", PUNCTUATION),
        Rule::token(r"(?ms)[0-9A-F]+X\b", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9A-F]+[HL]\b", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+\.[0-9]+E[+-][0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?ms)[0-9]+\.[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?ms)'[^\n']*'", STRING),
        Rule::token(r#"(?ms)"[^\n"]*""#, STRING),
        Rule::token(r"(?ms)[+-]", OPERATOR),
        Rule::token(r"(?ms)[*/]", OPERATOR),
        Rule::token(r"(?ms)[=#<>]", OPERATOR),
        Rule::token(r"(?ms)\^", OPERATOR),
        Rule::token(r"(?ms)&", OPERATOR),
        Rule::token(r"(?ms)~", OPERATOR),
        Rule::token(r"(?ms):=", OPERATOR),
        Rule::token(r"(?ms)\.\.", OPERATOR),
        Rule::token(r"(?ms)\$", OPERATOR),
        Rule::token(r"(?ms)(ANY(?:PTR|REC)|B(?:OOLEAN|YTE)|CHAR|INTEGER|LONGINT|REAL|S(?:ET|HORT(?:CHAR|INT|REAL)))\b", KEYWORD_TYPE),
        Rule::token(r"(?ms)(A(?:BS(?:(?:TRACT)?)|RRAY|S(?:H|SERT))|B(?:EGIN|ITS|Y)|C(?:A(?:P|SE)|HR|LOSE|ONST)|D(?:EC|IV|O)|E(?:LS(?:E|IF)|MPTY|N(?:D|TIER)|X(?:CL|IT|TENSIBLE))|FOR|HALT|I(?:MPORT|NC(?:(?:L)?)|[FNS])|L(?:EN|IMITED|O(?:NG|OP))|M(?:AX|IN|OD(?:(?:ULE)?))|NEW|O(?:DD|RD|UT|[FR])|P(?:OINTER|ROCEDURE)|RE(?:CORD|PEAT|TURN)|S(?:HORT(?:(?:CHAR|INT)?)|IZE)|T(?:HEN|O|YPE)|UNTIL|VAR|W(?:HILE|ITH))\b", KEYWORD_RESERVED),
        Rule::token(r"(?ms)(TRUE|FALSE|NIL|INF)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)([a-zA-Z_$][\w$]*)", NAME),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?ms)\n+", TEXT),
            Rule::token(r"(?ms)\s+", TEXT),
        ],
    );
    m.insert(
        r"comments",
        vec![Rule::token(r"(?ms)\(\*([^$].*?)\*\)", COMMENT_MULTILINE)],
    );
    m.insert(
        r"punctuation",
        vec![Rule::token(r"(?ms)[()\[\]{},.:;|]", PUNCTUATION)],
    );
    m.insert(
        r"numliterals",
        vec![
            Rule::token(r"(?ms)[0-9A-F]+X\b", NUMBER_HEX),
            Rule::token(r"(?ms)[0-9A-F]+[HL]\b", NUMBER_HEX),
            Rule::token(r"(?ms)[0-9]+\.[0-9]+E[+-][0-9]+", NUMBER_FLOAT),
            Rule::token(r"(?ms)[0-9]+\.[0-9]+", NUMBER_FLOAT),
            Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"strings",
        vec![
            Rule::token(r"(?ms)'[^\n']*'", STRING),
            Rule::token(r#"(?ms)"[^\n"]*""#, STRING),
        ],
    );
    m.insert(
        r"operators",
        vec![
            Rule::token(r"(?ms)[+-]", OPERATOR),
            Rule::token(r"(?ms)[*/]", OPERATOR),
            Rule::token(r"(?ms)[=#<>]", OPERATOR),
            Rule::token(r"(?ms)\^", OPERATOR),
            Rule::token(r"(?ms)&", OPERATOR),
            Rule::token(r"(?ms)~", OPERATOR),
            Rule::token(r"(?ms):=", OPERATOR),
            Rule::token(r"(?ms)\.\.", OPERATOR),
            Rule::token(r"(?ms)\$", OPERATOR),
        ],
    );
    m.insert(r"builtins", vec![
        Rule::token(r"(?ms)(ANY(?:PTR|REC)|B(?:OOLEAN|YTE)|CHAR|INTEGER|LONGINT|REAL|S(?:ET|HORT(?:CHAR|INT|REAL)))\b", KEYWORD_TYPE),
        Rule::token(r"(?ms)(A(?:BS(?:(?:TRACT)?)|RRAY|S(?:H|SERT))|B(?:EGIN|ITS|Y)|C(?:A(?:P|SE)|HR|LOSE|ONST)|D(?:EC|IV|O)|E(?:LS(?:E|IF)|MPTY|N(?:D|TIER)|X(?:CL|IT|TENSIBLE))|FOR|HALT|I(?:MPORT|NC(?:(?:L)?)|[FNS])|L(?:EN|IMITED|O(?:NG|OP))|M(?:AX|IN|OD(?:(?:ULE)?))|NEW|O(?:DD|RD|UT|[FR])|P(?:OINTER|ROCEDURE)|RE(?:CORD|PEAT|TURN)|S(?:HORT(?:(?:CHAR|INT)?)|IZE)|T(?:HEN|O|YPE)|UNTIL|VAR|W(?:HILE|ITH))\b", KEYWORD_RESERVED),
        Rule::token(r"(?ms)(TRUE|FALSE|NIL|INF)\b", KEYWORD_CONSTANT),
    ]);
    m.insert(
        r"identifiers",
        vec![Rule::token(r"(?ms)([a-zA-Z_$][\w$]*)", NAME)],
    );
    Table(m)
}

impl Lexer for ComponentpascalLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
