#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.mips:MIPSLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.mips:MIPSLexer:mips

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mips
pub struct MipsLexer;

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
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)-?[0-9]+?", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\w*:", NAME_FUNCTION),
        Rule::token(r"(?m)(b(?:(?:eq|g(?:ez(?:(?:al)?)|tz)|l(?:ez|tz(?:(?:al)?))|ne)l))\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)(abs|b(?:(?:eqz|g(?:(?:[et])u|[et])|l(?:(?:[et])u|[et])|nez)?)|l(?:\.(?:[ds])|[adi])|m(?:ove|ulo(?:(?:u)?))|n(?:eg(?:(?:u)?)|ot)|r(?:em(?:(?:u)?)|o(?:[lr]))|s(?:\.(?:[ds])|d|eq|g(?:(?:[et])u|[et])|le(?:(?:u)?)|ne)|u(?:l(?:hu|[hw])|s(?:[hw])))\b", NAME_VARIABLE),
        Rule::token(r"(?m)(a(?:bs\.(?:[ds])|dd(?:(?:\.(?:[ds])|iu|[iu])?)|nd(?:(?:i)?))|b(?:c1(?:[ft])|eq|g(?:ez(?:(?:al)?)|tz)|l(?:ez|tz(?:(?:al)?))|ne|op|reak)|c(?:\.(?:e\.(?:[ds])|l(?:e\.(?:[ds])|t\.(?:[ds])))|eil\.w\.(?:[ds])|l(?:[oz])|vt\.(?:d\.(?:[sw])|s\.(?:[dw])|w\.(?:[ds])))|div(?:(?:\.(?:[ds])|u)?)|eret|floor\.w\.(?:[ds])|j(?:(?:al(?:(?:r)?)|r)?)|l(?:bu|hu|ui|w(?:cl|[lr])|[bhlw])|m(?:add(?:(?:\.(?:[ds])|u)?)|f(?:hi|lo)|ov(?:\.d|e\.s|f\.(?:[ds])|n(?:\.(?:[ds])|zd)|t\.(?:[ds])|z\.(?:[ds])|[fntz])|sub(?:(?:\.(?:[ds])|u)?)|t(?:hi|lo)|ul(?:(?:\.(?:[ds])|tu|[tu])?))|n(?:eg\.(?:[ds])|or)|or(?:(?:i)?)|round\.w\.(?:[ds])|s(?:l(?:lv|t(?:iu|[iu])|[lt])|qrt\.(?:[ds])|r(?:(?:[al])v|[al])|ub(?:(?:\.(?:[ds])|[iu])?)|w(?:[lr])|yscall|[bchw])|t(?:eq(?:(?:i)?)|ge(?:(?:iu|[iu])?)|lt(?:(?:iu|[iu])?)|ne(?:(?:qi)?)|runc\.w\.(?:[ds]))|xor(?:(?:i)?))\b", KEYWORD),
        Rule::token(r"(?m)[slm][ftwd]c[0-9]([.]d)?", KEYWORD),
        Rule::token(r"(?m)\$(f?[0-2][0-9]|f?3[01]|[ft]?[0-9]|[vk][01]|a[0-3]|s[0-7]|[gsf]p|ra|at|zero)", KEYWORD_TYPE),
        Rule::token(r"(?m)(\.(?:a(?:lign|scii(?:(?:z)?))|byte|d(?:ata|ouble)|extern|float|globl|half|k(?:data|text)|space|text|word))\b", NAME_ENTITY),
        Rule::token(r"(?m):|,|;|\{|\}|=>|@|\$|=", NAME_BUILTIN),
        Rule::token(r"(?m)\w+", TEXT),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^\\"]+"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for MipsLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
