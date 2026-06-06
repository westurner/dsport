//! AUTO-GENERATED from `pygments.pygments.lexers.teal:TealLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.teal:TealLexer:teal

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: teal
pub struct TealLexer;

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
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)//[^\n]+", COMMENT_SINGLE),
        Rule::token(r"(?m)^#pragma .*\r?\n", TokenType::new(&["Comment", "Directive"])),
        Rule::bygroups(r"(?m)([^ \t\n]+(?=\/\/)|[^ \t\n]+:)([ 	].*)", vec![Some(NAME_LABEL), Some(COMMENT_SINGLE)]),
        Rule::token_to(r"(?m)[^ \t\n]+(?=\/\/)|[^ \t\n]+", NAME_FUNCTION, NewState::Push(vec![r"function-args"])),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)//[^\n]+", COMMENT_SINGLE),
    ]);
    m.insert(r"function-args", vec![
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)//[^\n]+", COMMENT_SINGLE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?m)(b(?:ase)?(?:32|64) ?)(\(?[a-zA-Z0-9+/=]+\)?)", vec![Some(STRING_AFFIX), Some(STRING_OTHER)]),
        Rule::token(r"(?m)[A-Z2-7]{58}", NUMBER),
        Rule::token(r"(?m)0x[\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)(A(?:ccounts|mount|pp(?:lication(?:Args|ID)|rovalProgram)|sset(?:Amount|Balance|Cl(?:awback|oseTo)|De(?:cimals|faultFrozen)|Fr(?:eeze|ozen)|M(?:anager|etadataHash)|Name|Re(?:ceiver|serve)|Sender|Total|U(?:RL|nitName)))|C(?:l(?:earState(?:(?:Program)?)|ose(?:Out|RemainderTo))|onfigAsset(?:(?:Clawback|De(?:cimals|faultFrozen)|Freeze|M(?:anager|etadataHash)|Name|Reserve|Total|U(?:RL|nitName))?)|urrentApplicationID)|DeleteApplication|F(?:ee|irstValid(?:(?:Time)?)|reezeAsset(?:(?:Account|Frozen)?))|Group(?:Index|Size)|L(?:a(?:stValid|testTimestamp)|ease|ogicSigVersion)|M(?:(?:axTxnLif|in(?:Balanc|TxnFe))e)|N(?:o(?:Op|te)|umA(?:(?:ccount|ppArg)s))|O(?:(?:nCompletio|ptI)n)|R(?:e(?:ceiver|keyTo)|ound)|Se(?:lectionPK|nder)|T(?:xID|ype(?:(?:Enum)?))|UpdateApplication|Vote(?:First|KeyDilution|Last|PK)|XferAsset|ZeroAddress)\b", KEYWORD),
        Rule::token(r"(?m)[^ \t\n]+(?=\/\/)|[^ \t\n]+", TokenType::new(&["Name", "Attributes"])),
        Rule::token_to(r"(?m)\r?\n", TEXT, NewState::Pop(1)),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)\\(?:["nrt\\]|x\d\d)"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\\"\n]+"#, STRING),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for TealLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
