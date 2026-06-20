#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.comal:Comal80Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.comal:Comal80Lexer:comal

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: comal, comal80
pub struct ComalLexer;

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
        Rule::token(r"(?im)//.*\n", COMMENT_SINGLE),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im):[=+-]|\<\>|[-+*/^↑<>=]", OPERATOR),
        Rule::token(r"(?im)(and +then|or +else)\b(?!['\[\]←£\\])", OPERATOR_WORD),
        Rule::token(r"(?im)(and|bit(?:and|(?:(?:x)?)or)|div|in|mod|not|or)\b(?!['\[\]←£\\])", OPERATOR_WORD),
        Rule::token(r"(?im)(a(?:ppend|t)|c(?:ase|hain|lose|opy|reate|ursor)|d(?:ata|elete|ir|o)|e(?:l(?:if|se)|nd(?:(?:case|for|if|loop|trap|while)?)|x(?:ec|it))|f(?:ile|or)|goto|handler|i(?:f|nput)|l(?:et|oop)|mount|null|o(?:f|pen|therwise|utput)|p(?:a(?:ge|ss)|oke|rint)|r(?:andom|e(?:ad|name|p(?:(?:ea|or)t)|store|turn))|s(?:elect|t(?:(?:[eo])p)|ys)|t(?:hen|o|rap)|u(?:n(?:it(?:(?:\$)?)|til)|sing)|w(?:h(?:en|ile)|rite)|zone)\b(?!['\[\]←£\\])", KEYWORD_RESERVED),
        Rule::token(r"(?im)(closed|dim|e(?:nd(?:(?:fun|pro)c)|xternal)|func|import|proc|ref|use)\b(?!['\[\]←£\\])", KEYWORD_DECLARATION),
        Rule::token(r"(?im)(a(?:bs|tn)|c(?:hr\$|os)|e(?:o(?:[df])|rr(?:(?:file|text)?)|sc|xp)|int|key\$|l(?:en|og)|ord|peek|r(?:andomize|nd)|s(?:gn|in|pc\$|qr|t(?:(?:atus|r)\$))|t(?:a(?:[bn])|ime)|val)\b(?!['\[\]←£\\])", NAME_BUILTIN),
        Rule::token(r"(?im)(false|pi|true)\b(?!['\[\]←£\\])", KEYWORD_CONSTANT),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)[a-z]['\[\]←£\\\w]*:(?=[ \n/])", NAME_LABEL),
        Rule::token(r"(?im)[a-z]['\[\]←£\\\w]*[$#]?", NAME),
        Rule::token(r"(?im)%[01]+", NUMBER_BIN),
        Rule::token(r"(?im)\$[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?im)\d*\.\d*(e[-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)\d+", NUMBER_INTEGER),
        Rule::token(r"(?im)[(),:;]", PUNCTUATION),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?im)[^"]+"#, STRING),
            Rule::token(r#"(?im)"[0-9]*""#, STRING_ESCAPE),
            Rule::token_to(r#"(?im)""#, STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for ComalLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
