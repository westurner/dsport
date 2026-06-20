#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.chapel:ChapelLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.chapel:ChapelLexer:chapel

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: chapel, chpl
pub struct ChapelLexer;

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
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(.*?)\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(con(?:fig|st)|in(?:(?:out)?)|out|param|ref|type|var)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(false|n(?:il|one)|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(b(?:ool|ytes)|complex|i(?:mag|nt)|locale|nothing|opaque|r(?:ange|eal)|string|uint|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(a(?:lign|s|tomic)|b(?:egin|orrowed|reak|y)|c(?:atch|o(?:begin|forall|ntinue))|d(?:e(?:fer|lete)|mapped|o(?:(?:main)?))|e(?:lse|num|x(?:cept|port|tern))|for(?:(?:all|each|warding)?)|i(?:f|mp(?:lements|ort)|n(?:dex|it|line))|l(?:a(?:bel|mbda)|et|ifetime|ocal)|n(?:ew|oinit)|o(?:n(?:(?:ly)?)|therwise|verride|wned)|p(?:r(?:agma|(?:i(?:mitiv|vat)|ototyp)e)|ublic)|re(?:duce|quire|turn)|s(?:can|e(?:lect|rial)|hared|ingle|parse|ubdomain|ync)|t(?:h(?:en|is|row(?:(?:s)?))|ry)|u(?:nmanaged|se)|w(?:h(?:e(?:n|re)|ile)|ith)|yield|zip)\b", KEYWORD),
        Rule::token_to(r"(?m)@", KEYWORD, NewState::Push(vec![r"attributename"])),
        Rule::bygroups_to(r"(?m)(iter)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"procname"])),
        Rule::bygroups_to(r"(?m)(proc)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"procname"])),
        Rule::bygroups_to(r"(?m)(operator)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"procname"])),
        Rule::bygroups_to(r"(?m)(class|interface|module|record|union)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token(r"(?m)\d+i", NUMBER),
        Rule::token(r"(?m)\d+\.\d*([Ee][-+]\d+)?i", NUMBER),
        Rule::token(r"(?m)\.\d+([Ee][-+]\d+)?i", NUMBER),
        Rule::token(r"(?m)\d+[Ee][-+]\d+i", NUMBER),
        Rule::token(r"(?m)(\d*\.\d+)([eE][+-]?[0-9]+)?i?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+i?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[bB][01]+", NUMBER_BIN),
        Rule::token(r"(?m)0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)0[oO][0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r#"(?m)"(\\\\|\\"|[^"])*""#, STRING),
        Rule::token(r"(?m)'(\\\\|\\'|[^'])*'", STRING),
        Rule::token(r"(?m)(=|\+=|-=|\*=|/=|\*\*=|%=|&=|\|=|\^=|&&=|\|\|=|<<=|>>=|<=>|<~>|\.\.|by|#|\.\.\.|&&|\|\||!|&|\||\^|~|<<|>>|==|!=|<=|>=|<|>|[+\-*/%]|\*\*)", OPERATOR),
        Rule::token(r"(?m)[:;,.?()\[\]{}]", PUNCTUATION),
        Rule::token(r"(?m)[a-zA-Z_][\w$]*", NAME_OTHER),
    ]);
    m.insert(
        r"classname",
        vec![Rule::token_to(
            r"(?m)[a-zA-Z_][\w$]*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"procname",
        vec![
            Rule::token_to(
                r"(?m)([a-zA-Z_][.\w$]*|\~[a-zA-Z_][.\w$]*|[+*/!~%<>=&^|\-:]{1,2})",
                NAME_FUNCTION,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"receivertype"]),
            ),
            Rule::token(r"(?m)\)+\.", PUNCTUATION),
        ],
    );
    m.insert(r"receivertype", vec![
        Rule::token(r"(?m)(atomic|borrowed|owned|s(?:hared|ingle|ync)|unmanaged)\b", KEYWORD),
        Rule::token(r"(?m)(b(?:ool|ytes)|complex|i(?:mag|nt)|locale|nothing|opaque|r(?:ange|eal)|string|uint|void)\b", KEYWORD_TYPE),
        Rule::token_to(r"(?m)[^()]*", NAME_OTHER, NewState::Pop(1)),
    ]);
    m.insert(
        r"attributename",
        vec![Rule::token_to(
            r"(?m)[a-zA-Z_][.\w$]*",
            NAME_DECORATOR,
            NewState::Pop(1),
        )],
    );
    Table(m)
}

impl Lexer for ChapelLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
