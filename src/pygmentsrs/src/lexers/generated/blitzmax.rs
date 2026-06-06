//! AUTO-GENERATED from `pygments.pygments.lexers.basic:BlitzMaxLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.basic:BlitzMaxLexer:blitzmax

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: blitzmax, bmax
pub struct BlitzmaxLexer;

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
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::bygroups(r"(?im)(\.\.)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?im)'.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?im)([ \t]*)\bRem\n(\n|.)*?\s*\bEnd([ \t]*)Rem", COMMENT_MULTILINE),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)[0-9]+\.[0-9]*(?!\.)", NUMBER_FLOAT),
        Rule::token(r"(?im)\.[0-9]*(?!\.)", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\$[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?im)\%[10]+", NUMBER_BIN),
        Rule::token(r"(?im)(?:(?:(:)?([ \t]*)(:?\b(Shl|Shr|Sar|Mod)\b|([+\-*/&|~]))|Or|And|Not|[=<>^]))", OPERATOR),
        Rule::token(r"(?im)[(),.:\[\]]", PUNCTUATION),
        Rule::token(r"(?im)(?:#[\w \t]*)", NAME_LABEL),
        Rule::token(r"(?im)(?:\?[\w \t]*)", COMMENT_PREPROC),
        Rule::bygroups(r"(?im)\b(New)\b([ \t]?)([(]?)([a-z_]\w*)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(PUNCTUATION), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?im)\b(Import|Framework|Module)([ \t]+)([a-z_]\w*\.[a-z_]\w*)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(KEYWORD_NAMESPACE)]),
        Rule::bygroups(r"(?im)([a-z_]\w*)(?:(?:([ \t]*)(@{1,2}|[!#$%])|([ \t]*:[ \t]*\b(?:Shl|Shr|Sar|Mod)\b)|([ \t]*)(:)([ \t]*)(?:\b(Int|Byte|Short|Float|Double|Long)\b|([a-z_]\w*)))(?:([ \t]*)(Ptr))?)?((?:[ \t]|\.\.\n)*)([(])", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(OPERATOR), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(NAME_CLASS), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?im)([a-z_]\w*)(?:(?:([ \t]*)(@{1,2}|[!#$%])|([ \t]*:[ \t]*\b(?:Shl|Shr|Sar|Mod)\b)|([ \t]*)(:)([ \t]*)(?:\b(Int|Byte|Short|Float|Double|Long)\b|([a-z_]\w*)))(?:([ \t]*)(Ptr))?)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(OPERATOR), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(NAME_CLASS), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?im)\b(Type|Extends)([ \t]+)([a-z_]\w*)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::token(r"(?im)\b(Ptr)\b", KEYWORD_TYPE),
        Rule::token(r"(?im)\b(Pi|True|False|Null|Self|Super)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?im)\b(Local|Global|Const|Field)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?im)\b(T(?:(?:ArrayBounds|Null(?:Function|Method|Object)|Runtime)Exception))\b", NAME_EXCEPTION),
        Rule::token(r"(?im)\b(A(?:bs(?:(?:tract)?)|s(?:c|sert))|C(?:a(?:se|tch)|hr|ontinue)|De(?:f(?:Data|ault)|lete)|E(?:achIn|lse(?:(?:If)?)|nd(?:(?:Extern|Function|If|Method|Select|T(?:ry|ype)|While)?)|x(?:it|te(?:nds|rn)))|F(?:inal|or(?:(?:ever)?)|ramework|unction)|Goto|I(?:f|mport|nc(?:bin(?:(?:Len|Ptr)?)|lude))|Len|M(?:ax|ethod|in|odule(?:(?:Info)?))|Ne(?:w|xt)|P(?:rivate|ublic)|Re(?:adData|lease|peat|storeData|turn)|S(?:elect|gn|izeOf|t(?:ep|rict)|uperStrict)|T(?:h(?:en|row)|o|ry|ype)|Until|Var(?:(?:Ptr)?)|W(?:end|hile))\b", KEYWORD_RESERVED),
        Rule::token(r"(?im)([a-z_]\w*)", NAME_VARIABLE),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?im)"""#, STRING_DOUBLE),
        Rule::token_to(r#"(?im)"C?"#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?im)[^"]+"#, STRING_DOUBLE),
    ]);
    Table(m)
}

impl Lexer for BlitzmaxLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
