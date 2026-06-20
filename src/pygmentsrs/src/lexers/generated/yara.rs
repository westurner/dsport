#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.yara:YaraLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.yara:YaraLexer:yara

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: yara, yar
pub struct YaraLexer;

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
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)\#.*?$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)\b(global|i(?:mport|nclude)|(?:privat|rul)e)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(condition|meta|strings)\b", KEYWORD),
        Rule::token(r"(?m)\b(a(?:scii|t)|base64(?:(?:wide)?)|con(?:dition|tains)|defined|en(?:dswith|trypoint)|f(?:ilesize|or|ullword)|i(?:contains|e(?:ndswith|quals)|n(?:(?:clude|t(?:16(?:(?:be)?)|32(?:(?:be)?)|8(?:(?:be)?)))?)|startswith)|m(?:atches|eta)|no(?:(?:cas|n)e)|of|st(?:artswith|rings)|them|uint(?:16(?:(?:be)?)|32(?:(?:be)?)|8(?:(?:be)?))|wide|xor)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(true|false)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(and|or|not|any|all)\b", OPERATOR_WORD),
        Rule::token(r"(?m)(\$\w+)", NAME_VARIABLE),
        Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?m)\'[^\']*\'", STRING_SINGLE),
        Rule::token(r"(?m)\{.*?\}$", NUMBER_HEX),
        Rule::token(r"(?m)(/.*?/)", STRING_REGEX),
        Rule::token(r"(?m)[a-z_]\w*", NAME),
        Rule::token(r"(?m)[$(){}\[\].?+*|]", PUNCTUATION),
        Rule::token(r"(?m)[:=,;]", PUNCTUATION),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for YaraLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
