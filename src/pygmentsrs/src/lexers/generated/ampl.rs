#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.ampl:AmplLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ampl:AmplLexer:ampl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ampl
pub struct AmplLexer;

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
        Rule::token(r"(?m)\n", TEXT),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(c(?:all|d|heck|lose|o(?:ef(?:(?:f)?)|m(?:(?:mand|plement)s)|ver))|d(?:ata|e(?:fault|lete)|i(?:men(?:(?:sion)?)|splay)|rop)|e(?:lse|n(?:d|viron)|x(?:it|pand))|f(?:or(?:(?:mat)?)|rom|unction)|i(?:n(?:clude|terval)|[fn])|load|model|net_(?:in|out)|o(?:bj(?:(?:ective)?)|ption)|p(?:ipe|roblem|urge)|quit|re(?:declare|load|move|peat|s(?:et|tore))|s(?:h(?:ell|ow)|ol(?:expand|ution|ve))|t(?:hen|o(?:(?:_come)?))|u(?:nload|pdate)|w(?:(?:hil|rit)e)|xref)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(integer|binary|symbolic|ordered|circular|reversed|INOUT|IN|OUT|LOCAL)", KEYWORD_TYPE),
        Rule::token(r#"(?m)\".*?\""#, STRING_DOUBLE),
        Rule::token(r"(?m)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?m)[()\[\]{},;:]+", PUNCTUATION),
        Rule::bygroups(r"(?m)\b(\w+)(\.)(astatus|init0|init|lb0|lb1|lb2|lb|lrc|lslack|rc|relax|slack|sstatus|status|ub0|ub1|ub2|ub|urc|uslack|val)", vec![Some(NAME_VARIABLE), Some(PUNCTUATION), Some(KEYWORD_RESERVED)]),
        Rule::bygroups(r"(?m)(set|param|var|arc|minimize|maximize|subject to|s\.t\.|subj to|node|table|suffix|read table|write table)(\s+)(\w+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_VARIABLE)]),
        Rule::bygroups(r"(?m)(param)(\s*)(:)(\s*)(\w+)(\s*)(:)(\s*)((\w|\s)+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_VARIABLE)]),
        Rule::bygroups_g(r"(?m)(let|fix|unfix)(\s*)((?:\{.*\})?)(\s*)(\w+)", vec![Some(GroupAction::Token(KEYWORD_DECLARATION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE))]),
        Rule::token(r"(?m)\b(Beta|Cauchy|Exponential|Gamma|Irand224|Normal(?:(?:01)?)|Poisson|Uniform(?:(?:01)?)|a(?:bs|cos(?:(?:h)?)|lias|rity|sin(?:(?:h)?)|tan(?:(?:[2h])?))|c(?:ard|eil|har|os|time)|exp|f(?:irst|loor)|gsub|i(?:char|ndexarity)|l(?:ast|ength|og(?:(?:10)?))|m(?:a(?:tch|x)|in)|n(?:ext(?:(?:w)?)|um(?:(?:0)?))|ord(?:(?:0)?)|pr(?:e(?:cision|v(?:(?:w)?))|int(?:(?:f)?))|round|s(?:in(?:(?:h)?)|printf|qrt|ub(?:(?:str)?))|t(?:an(?:(?:h)?)|ime|runc))\b", NAME_BUILTIN),
        Rule::token(r"(?m)(\+|\-|\*|/|\*\*|=|<=|>=|==|\||\^|<|>|\!|\.\.|:=|\&|\!=|<<|>>)", OPERATOR),
        Rule::token(r"(?m)(and|by|cross|di(?:ff(?:(?:erence)?)|v)|exists|forall|in(?:(?:ter(?:(?:sect(?:(?:ion)?))?))?)|less|mod|not|or|prod(?:(?:uct)?)|s(?:etof|um|ymdiff)|(?:unio|withi)n)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(\d+\.(?!\.)\d*|\.(?!.)\d+)([eE][+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+([eE][+-]?\d+)?", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-]?Infinity", NUMBER_INTEGER),
        Rule::token(r"(?m)(\w+|(\.(?!\.)))", TEXT),
    ]);
    Table(m)
}

impl Lexer for AmplLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
