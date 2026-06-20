//! AUTO-GENERATED from `pygments.pygments.lexers.cddl:CddlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.cddl:CddlLexer:cddl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cddl
pub struct CddlLexer;

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
        r"commentsandwhitespace",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m);.+$", COMMENT_SINGLE),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m);.+$", COMMENT_SINGLE),
        Rule::token(r"(?m)#(\d\.(?:0b[01]+|0x[0-9a-fA-F]+|[1-9]\d*|0(?!\d)))?", KEYWORD_TYPE),
        Rule::bygroups(r"(?m)((?:0b[01]+|0x[0-9a-fA-F]+|[1-9]\d*|0(?!\d)))?(\*)((?:0b[01]+|0x[0-9a-fA-F]+|[1-9]\d*|0(?!\d)))?", vec![Some(NUMBER), Some(OPERATOR), Some(NUMBER)]),
        Rule::token(r"(?m)\?|\+", OPERATOR),
        Rule::token(r"(?m)\^", OPERATOR),
        Rule::token(r"(?m)(\.\.\.|\.\.)", OPERATOR),
        Rule::token(r"(?m)(\.(?:and|bits|cbor(?:(?:seq)?)|default|eq|g(?:[et])|l(?:[et])|ne|regexp|size|within))\b", OPERATOR_WORD),
        Rule::token(r"(?m)&(?=\s*([$@A-Z_a-z](?:[\-\.]+(?=[$@0-9A-Z_a-z])|[$@0-9A-Z_a-z])*|\())", OPERATOR),
        Rule::token(r"(?m)~(?=\s*[$@A-Z_a-z](?:[\-\.]+(?=[$@0-9A-Z_a-z])|[$@0-9A-Z_a-z])*)", OPERATOR),
        Rule::token(r"(?m)//|/(?!/)", OPERATOR),
        Rule::token(r"(?m)=>|/==|/=|=", OPERATOR),
        Rule::token(r"(?m)[\[\]{}\(\),<>:]", PUNCTUATION),
        Rule::bygroups_to(r"(?m)(b64)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"bstrb64url"])),
        Rule::bygroups_to(r"(?m)(h)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"bstrh"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"bstr"])),
        Rule::bygroups(r"(?m)([$@A-Z_a-z](?:[\-\.]+(?=[$@0-9A-Z_a-z])|[$@0-9A-Z_a-z])*)(\s*)(:)", vec![Some(STRING), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)(?![\-_$@])\b(any|b(?:64(?:legacy|url)|ig(?:(?:floa|(?:(?:[nu])?)in)t)|ool|str|ytes)|cbor\-any|decfrac|e(?:b(?:16|64(?:legacy|url))|ncoded\-cbor)|f(?:alse|loat(?:(?:16(?:(?:\-32)?)|32(?:(?:\-64)?)|64)?))|int(?:(?:eger)?)|mime\-message|n(?:i(?:l|nt)|u(?:ll|mber))|regexp|t(?:date|ext|ime|rue|str)|u(?:int|n(?:(?:defi|sig)ned)|ri))\b(?![\-_$@])", NAME_BUILTIN),
        Rule::token(r"(?m)[$@A-Z_a-z](?:[\-\.]+(?=[$@0-9A-Z_a-z])|[$@0-9A-Z_a-z])*", NAME_CLASS),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+(\.[0-9a-fA-F]+)?p[+-]?\d+", NUMBER_HEX),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)-?(?:0b[01]+|0x[0-9a-fA-F]+|[1-9]\d*|0(?!\d))(?=(\.\d|e[+-]?\d))(?:\.\d+)?(?:e[+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?(?:0b[01]+|0x[0-9a-fA-F]+|[1-9]\d*|0(?!\d))", NUMBER_INTEGER),
        Rule::token(r#"(?m)"(\\\\|\\"|[^"])*""#, STRING_DOUBLE),
    ]);
    m.insert(
        r"bstrb64url",
        vec![
            Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m);.+$", COMMENT_SINGLE),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r"(?m)[0-9a-zA-Z\-_=]+", STRING_SINGLE),
            Rule::token(r"(?m).", ERROR),
        ],
    );
    m.insert(
        r"bstrh",
        vec![
            Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m);.+$", COMMENT_SINGLE),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r"(?m)[0-9a-fA-F]+", STRING_SINGLE),
            Rule::token(r"(?m).", ERROR),
        ],
    );
    m.insert(
        r"bstr",
        vec![
            Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r"(?m)[^'\\]+", STRING_SINGLE),
        ],
    );
    Table(m)
}

impl Lexer for CddlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
