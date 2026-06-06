//! AUTO-GENERATED from `pygments.pygments.lexers.haskell:CryptolLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.haskell:CryptolLexer:cryptol

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cryptol, cry
pub struct CryptolLexer;

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
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token_to(r"(?m)\bimport\b", KEYWORD_RESERVED, NewState::Push(vec![r"import"])),
        Rule::token_to(r"(?m)\bmodule\b", KEYWORD_RESERVED, NewState::Push(vec![r"module"])),
        Rule::token(r"(?m)\berror\b", NAME_EXCEPTION),
        Rule::token(r"(?m)\b(Arith|Bit|Cmp|False|Inf|True|else|export|extern|fin|if|import|inf|lg2|max|min|module|newtype|pragma|property|then|type|where|width)(?!\')\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)^[_a-z][\w\']*", NAME_FUNCTION),
        Rule::token(r"(?m)'?[_a-z][\w']*", NAME),
        Rule::token(r"(?m)('')?[A-Z][\w\']*", KEYWORD_TYPE),
        Rule::token(r"(?m)\\(?![:!#$%&*+.\\/<=>?@^|~-]+)", NAME_FUNCTION),
        Rule::token(r"(?m)(<-|::|->|=>|=)(?![:!#$%&*+.\\/<=>?@^|~-]+)", OPERATOR_WORD),
        Rule::token(r"(?m):[:!#$%&*+.\\/<=>?@^|~-]*", KEYWORD_TYPE),
        Rule::token(r"(?m)[:!#$%&*+.\\/<=>?@^|~-]+", OPERATOR),
        Rule::token(r"(?m)\d+[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+\.\d+([eE][+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[oO][0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"character"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)\(\)", NAME_BUILTIN),
        Rule::token(r"(?m)[\]\[(),;`{}]", PUNCTUATION),
    ]);
    m.insert(r"import", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)qualified\b", KEYWORD),
        Rule::bygroups_to(r"(?m)([A-Z][\w.]*)(\s+)(as)(\s+)([A-Z][\w.]*)", vec![Some(NAME_NAMESPACE), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?m)([A-Z][\w.]*)(\s+)(hiding)(\s+)(\()", vec![Some(NAME_NAMESPACE), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"funclist"])),
        Rule::bygroups_to(r"(?m)([A-Z][\w.]*)(\s+)(\()", vec![Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"funclist"])),
        Rule::token_to(r"(?m)[\w.]+", NAME_NAMESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"module", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups_to(r"(?m)([A-Z][\w.]*)(\s+)(\()", vec![Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"funclist"])),
        Rule::token_to(r"(?m)[A-Z][\w.]*", NAME_NAMESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"funclist", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)[A-Z]\w*", KEYWORD_TYPE),
        Rule::token(r"(?m)(_[\w\']+|[a-z][\w\']*)", NAME_FUNCTION),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)[:!#$%&*+.\\/<=>?@^|~-]+", OPERATOR),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"funclist", r"funclist"])),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(2)),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)[^/*]+", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
        Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
    ]);
    m.insert(r"character", vec![
        Rule::token_to(r"(?m)[^\\']'", STRING_CHAR, NewState::Pop(1)),
        Rule::token_to(r"(?m)\\", STRING_ESCAPE, NewState::Push(vec![r"escape"])),
        Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Pop(1)),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)[^\\"]+"#, STRING),
        Rule::token_to(r"(?m)\\", STRING_ESCAPE, NewState::Push(vec![r"escape"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
    ]);
    m.insert(r"escape", vec![
        Rule::token_to(r#"(?m)[abfnrtv"\'&\\]"#, STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\^[\]\[A-Z@^_]", STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)NUL|SOH|[SE]TX|EOT|ENQ|ACK|BEL|BS|HT|LF|VT|FF|CR|S[OI]|DLE|DC[1-4]|NAK|SYN|ETB|CAN|EM|SUB|ESC|[FGRU]S|SP|DEL", STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)o[0-7]+", STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)x[\da-fA-F]+", STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\d+", STRING_ESCAPE, NewState::Pop(1)),
        Rule::bygroups_to(r"(?m)(\s+)(\\)", vec![Some(WHITESPACE), Some(STRING_ESCAPE)], NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for CryptolLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
