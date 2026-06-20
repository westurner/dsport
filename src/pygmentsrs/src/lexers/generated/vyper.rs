//! AUTO-GENERATED from `pygments.pygments.lexers.vyper:VyperLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.vyper:VyperLexer:vyper

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: vyper
pub struct VyperLexer;

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
        Rule::bygroups(r"(?m)(\\)(\n|\r\n|\r)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token_to(r#"(?m)\"\"\""#, COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comment"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"single-string"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"double-string"])),
        Rule::bygroups(r"(?m)(def)(\s+)([a-zA-Z_][a-zA-Z0-9_]*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(event|struct|interface|log)(\s+)([a-zA-Z_][a-zA-Z0-9_]*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(from)(\s+)(vyper\.\w+)(\s+)(import)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::token(r"(?m)\b0x[0-9a-fA-F]+\b", NUMBER_HEX),
        Rule::token(r"(?m)\b(\d{1,3}(?:_\d{3})*|\d+)\b", NUMBER_INTEGER),
        Rule::token(r"(?m)\b\d+\.\d*\b", NUMBER_FLOAT),
        Rule::token(r"(?m)\b(assert|def|e(?:l(?:if|se)|vent|xtcall)|f(?:or|rom)|i(?:mp(?:lements|ort)|n(?:dexed|terface)|[fn])|log|pass|r(?:aise|eturn)|st(?:aticcall|ruct)|while)\b", KEYWORD),
        Rule::token(r"(?m)\b(constant|immutable|nonpayable|p(?:rivate|u(?:blic|re))|view)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(_abi_(?:(?:de|en)code)|a(?:bs|s_wei_value)|b(?:itwise_(?:and|not|(?:(?:x)?)or)|lockhash)|c(?:eil|on(?:(?:ca|ver)t)|reate_(?:copy_of|from_blueprint|minimal_proxy_to))|e(?:c(?:add|mul|recover)|mpty|xtract32)|floor|isqrt|keccak256|len|m(?:ax(?:(?:_value)?)|ethod_id|in(?:(?:_value)?))|p(?:ow_mod256|rint)|range|s(?:h(?:a256|ift)|lice|qrt)|u(?:int2(?:56_(?:(?:add|mul)mod)|str)|nsafe_(?:add|div|mul|sub)))\b", NAME_BUILTIN),
        Rule::token(r"(?m)\b(block\.(?:number|timestamp)|msg\.(?:gas|sender|value))\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)\b(String|address|b(?:ool|ytes(?:(?:1(?:[0123456789])|2(?:[0123456789])|3(?:[012])|[123456789])?))|decimal|enum|int(?:(?:1(?:28|6)|256|32|64|8)?)|str(?:ing|uct)|uint(?:(?:1(?:28|6)|256|32|64|8)?))\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?m)\b(indexed)\b(\s*)(\()(\s*)(\w+)(\s*)(\))", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)(\+|\-|\*|\/|<=?|>=?|==|!=|=|\||&|%)", OPERATOR),
        Rule::token(r"(?m)[.,:;()\[\]{}]", PUNCTUATION),
        Rule::token(r"(?m)@[\w.]+", NAME_DECORATOR),
        Rule::token(r"(?m)__\w+__", TokenType::new(&["Name", "Magic"])),
        Rule::token(r"(?m)EMPTY_BYTES32", NAME_CONSTANT),
        Rule::token(r"(?m)\bERC20\b", NAME_CLASS),
        Rule::token(r"(?m)\bself\b", NAME_ATTRIBUTE),
        Rule::token(r"(?m)Bytes\[\d+\]", KEYWORD_TYPE),
        Rule::token(r"(?m)\b[a-zA-Z_]\w*\b:", NAME_VARIABLE),
        Rule::token(r"(?m)\b[a-zA-Z_]\w*\b", NAME),
    ]);
    m.insert(
        r"multiline-comment",
        vec![
            Rule::token_to(r#"(?m)\"\"\""#, COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"]+"#, COMMENT_MULTILINE),
            Rule::token(r#"(?m)\""#, COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"single-string",
        vec![
            Rule::token(r"(?m)[^\\']+", STRING_SINGLE),
            Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"double-string",
        vec![
            Rule::token(r#"(?m)[^\\"]+"#, STRING_DOUBLE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
        ],
    );
    Table(m)
}

impl Lexer for VyperLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
