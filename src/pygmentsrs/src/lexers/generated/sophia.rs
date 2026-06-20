//! AUTO-GENERATED from `pygments.pygments.lexers.sophia:SophiaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sophia:SophiaLexer:sophia

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: sophia
pub struct SophiaLexer;

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
        r"escape-sequence",
        vec![
            Rule::token(r#"(?m)\\[\\"\'ntbr]"#, STRING_ESCAPE),
            Rule::token(r"(?m)\\[0-9]{3}", STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)(true|false)\b", KEYWORD_CONSTANT),
        Rule::token_to(r"(?m)\b([A-Z][\w\']*)(?=\s*\.)", NAME_CLASS, NewState::Push(vec![r"dotted"])),
        Rule::token(r"(?m)\b([A-Z][\w\']*)", NAME_FUNCTION),
        Rule::token(r"(?m)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?m)\/\*(?!/)", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)0[xX][\da-fA-F][\da-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)#[\da-fA-F][\da-fA-F_]*", NAME_LABEL),
        Rule::token(r"(?m)\d[\d_]*", NUMBER_INTEGER),
        Rule::token(r"(?m)(as|contract|datatype|e(?:l(?:if|se)|ntrypoint)|f(?:or|unction)|hiding|i(?:f|n(?:clude|dexed|terface))|let|main|namespace|p(?:ayable|rivate|ublic)|record|s(?:tateful|witch)|type|using)\b", KEYWORD),
        Rule::token(r"(?m)(abort|put|(?:requir|stat)e)\b", NAME_BUILTIN),
        Rule::token(r"(?m)\b(b(?:and|not|(?:(?:x)?)or)|mod)\b", OPERATOR_WORD),
        Rule::token(r"(?m)\b(address|b(?:its|ool|ytes)|char|event|hash|int|list|map|o(?:ption|racle(?:(?:_query)?))|s(?:ignature|tring)|unit)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[=!<>+\\*/:&|?~@^-]", OPERATOR_WORD),
        Rule::token(r"(?m)[.;:{}(),\[\]]", PUNCTUATION),
        Rule::token(r"(?m)(ak_|ok_|oq_|ct_)[\w']*", NAME_LABEL),
        Rule::token(r"(?m)[^\W\d][\w']*", NAME),
        Rule::token(r#"(?m)'(?:(\\[\\\"'ntbr ])|(\\[0-9]{3})|(\\x[0-9a-fA-F]{2}))'"#, STRING_CHAR),
        Rule::token(r"(?m)'.'", STRING_CHAR),
        Rule::token(r"(?m)'[a-z][\w]*", NAME_VARIABLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^/*]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)\/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*\/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)\*", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)[^\\"]+"#, STRING_DOUBLE),
            Rule::token(r#"(?m)\\[\\"\'ntbr]"#, STRING_ESCAPE),
            Rule::token(r"(?m)\\[0-9]{3}", STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
            Rule::token(r"(?m)\\\n", STRING_DOUBLE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"dotted",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r"(?m)\.", PUNCTUATION),
            Rule::token(r"(?m)[A-Z][\w\']*(?=\s*\.)", NAME_FUNCTION),
            Rule::token_to(r"(?m)[A-Z][\w\']*", NAME_FUNCTION, NewState::Pop(1)),
            Rule::token_to(r"(?m)[a-z_][\w\']*", NAME, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for SophiaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
