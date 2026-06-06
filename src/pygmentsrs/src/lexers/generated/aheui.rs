//! AUTO-GENERATED from `pygments.pygments.lexers.esoteric:AheuiLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.esoteric:AheuiLexer:aheui

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: aheui
pub struct AheuiLexer;

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
        Rule::token(r"(?m)[나-낳냐-냫너-넣녀-녛노-놓뇨-눟뉴-닇다-닿댜-댷더-덯뎌-뎧도-돟됴-둫듀-딓따-땋땨-떃떠-떻뗘-뗳또-똫뚀-뚷뜌-띟라-랗랴-럏러-렇려-렿로-롷료-뤃류-릫마-맣먀-먛머-멓며-몋모-뫃묘-뭏뮤-믷바-밯뱌-뱧버-벟벼-볗보-봏뵤-붛뷰-빃빠-빻뺘-뺳뻐-뻫뼈-뼣뽀-뽛뾰-뿧쀼-삏사-샇샤-샿서-섷셔-셯소-솧쇼-숳슈-싛싸-쌓쌰-썋써-쎃쎠-쎻쏘-쏳쑈-쑿쓔-씧자-잫쟈-쟣저-젛져-졓조-좋죠-줗쥬-즿차-챃챠-챻처-첳쳐-쳫초-촣쵸-춯츄-칗카-캏캬-컇커-컿켜-켷코-콯쿄-쿻큐-킣타-탛탸-턓터-텋텨-톃토-톻툐-퉇튜-틯파-팧퍄-퍟퍼-펗펴-폏포-퐇표-풓퓨-픻하-핳햐-햫허-헣혀-혛호-홓효-훟휴-힇]", OPERATOR),
        Rule::token(r"(?m).", COMMENT),
    ]);
    Table(m)
}

impl Lexer for AheuiLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
