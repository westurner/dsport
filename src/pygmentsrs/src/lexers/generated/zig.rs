//! AUTO-GENERATED from `pygments.pygments.lexers.zig:ZigLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.zig:ZigLexer:zig

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: zig
pub struct ZigLexer;

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
        Rule::token(r"(?m)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)(a(?:s(?:m|ync)|wait)|break|c(?:a(?:ncel|tch)|ontinue)|defer|errdefer|re(?:sume|turn)|suspend|try|unreachable)\b", KEYWORD),
        Rule::token(r"(?m)(al(?:ign|lowzero)|co(?:mptime|nst)|ex(?:port|tern)|inline|linksection|n(?:akedcc|oalias)|p(?:acked|ub)|stdcallcc|threadlocal|v(?:ar|olatile))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(e(?:num|rror)|struct|union)\b", KEYWORD),
        Rule::token(r"(?m)(for|while)\b", KEYWORD),
        Rule::token(r"(?m)(anyerror|bool|c(?:_(?:int|long(?:(?:double|long)?)|short|u(?:int|long(?:(?:long)?)|short)|voidi8)|omptime_(?:(?:floa|in)t))|f(?:1(?:28|6)|32|64)|i(?:0|1(?:28|6)|32|64|size)|noreturn|promise|type|u(?:1(?:28|6)|32|64|size|[08])|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(false|null|true|undefined)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(and|else|if|or(?:(?:else)?)|switch)\b", KEYWORD),
        Rule::token(r"(?m)(fn|test|usingnamespace)\b", KEYWORD),
        Rule::token(r"(?m)0x[0-9a-fA-F]+\.[0-9a-fA-F]+([pP][\-+]?[0-9a-fA-F]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+\.?[pP][\-+]?[0-9a-fA-F]+", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+\.[0-9]+([eE][-+]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+\.?[eE][-+]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)\'\\\'\'", STRING_ESCAPE),
        Rule::token(r#"(?m)\'\\(x[a-fA-F0-9]{2}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{6}|[nr\\t\'"])\'"#, STRING_ESCAPE),
        Rule::token(r"(?m)\'[^\\\']\'", STRING),
        Rule::token(r"(?m)\\\\[^\n]*", STRING_HEREDOC),
        Rule::token(r"(?m)c\\\\[^\n]*", STRING_HEREDOC),
        Rule::token_to(r#"(?m)c?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[+%=><|^!?/\-*&~:]", OPERATOR),
        Rule::token(r"(?m)[{}()\[\],.;]", PUNCTUATION),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)\\(x[a-fA-F0-9]{2}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{6}|[nr\\t\'"])"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for ZigLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
