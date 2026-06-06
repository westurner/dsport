//! AUTO-GENERATED from `pygments.pygments.lexers.smv:NuSMVLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.smv:NuSMVLexer:nusmv

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: nusmv
pub struct NusmvLexer;

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
        Rule::token(r"(?m)(?s)\/\-\-.*?\-\-/", COMMENT),
        Rule::token(r"(?m)--.*\n", COMMENT),
        Rule::token(r"(?m)(ASSIGN|C(?:O(?:MP(?:ASSION|UTE|WFF)|NST(?:ANTS|RAINT))|TL(?:SPEC|WFF))|DEFINE|F(?:AIRNESS|ROZENVAR)|I(?:N(?:(?:IT|VAR(?:(?:SPEC)?))?)|SA|VAR)|JUSTICE|LTL(?:SPEC|WFF)|M(?:AX|DEFINE|I(?:N|RROR)|ODULE)|NAME|P(?:RED(?:(?:ICATES)?)|SL(?:SPEC|WFF))|S(?:IMPWFF|PEC)|TRANS|VAR)(?![\w$#-])", KEYWORD_DECLARATION),
        Rule::token(r"(?m)process(?![\w$#-])", KEYWORD),
        Rule::token(r"(?m)(array|boolean|integer|of|real|word)(?![\w$#-])", KEYWORD_TYPE),
        Rule::token(r"(?m)(case|esac)(?![\w$#-])", KEYWORD),
        Rule::token(r"(?m)(abs|bool|count|extend|init|m(?:ax|in)|resize|s(?:elf|i(?:gned|zeof)|wconst)|u(?:nsigned|wconst)|word1)(?![\w$#-])", NAME_BUILTIN),
        Rule::token(r"(?m)(A(?:B(?:[FG])|[FGX])|BU|E(?:B(?:[FG])|[FGX])|in|mod|next|union|x(?:(?:(?:n)?)or)|[AEFGHOSTUVXYZ])(?![\w$#-])", OPERATOR_WORD),
        Rule::token(r"(?m)((?:FALS|TRU)E)(?![\w$#-])", KEYWORD_CONSTANT),
        Rule::token(r"(?m)[a-zA-Z_][\w$#-]*", NAME_VARIABLE),
        Rule::token(r"(?m):=", OPERATOR),
        Rule::token(r"(?m)[-&|+*/<>!=]", OPERATOR),
        Rule::token(r"(?m)\-?\d+\b", NUMBER_INTEGER),
        Rule::token(r"(?m)0[su][bB]\d*_[01_]+", NUMBER_BIN),
        Rule::token(r"(?m)0[su][oO]\d*_[0-7_]+", NUMBER_OCT),
        Rule::token(r"(?m)0[su][dD]\d*_[\d_]+", TokenType::new(&["Literal", "Number", "Decimal"])),
        Rule::token(r"(?m)0[su][hH]\d*_[\da-fA-F_]+", NUMBER_HEX),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)[()\[\]{};?:.,]", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for NusmvLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
