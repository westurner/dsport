#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.hdl:VhdlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.hdl:VhdlLexer:vhdl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: vhdl
pub struct VhdlLexer;

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
        Rule::bygroups(r"(?im)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
        Rule::token(r"(?im)--.*?$", COMMENT_SINGLE),
        Rule::token(r"(?im)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?im)'(U|X|0|1|Z|W|L|H|-)'", STRING_CHAR),
        Rule::token(r"(?im)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?im)'[a-z_]\w*", NAME_ATTRIBUTE),
        Rule::token(r"(?im)[()\[\],.;\']", PUNCTUATION),
        Rule::token(r#"(?im)"[^\n\\"]*""#, STRING),
        Rule::bygroups(r"(?im)(library)(\s+)([a-z_]\w*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups(r"(?im)(use)(\s+)(entity)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(use)(\s+)([a-z_][\w.]*\.)(all)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(use)(\s+)([a-z_][\w.]*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups(r"(?im)(std|ieee)(\.[a-z_]\w*)", vec![Some(NAME_NAMESPACE), Some(NAME_NAMESPACE)]),
        Rule::token(r"(?im)(ieee|std|work)\b", NAME_NAMESPACE),
        Rule::bygroups(r"(?im)(entity|component)(\s+)([a-z_]\w*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?im)(architecture|configuration)(\s+)([a-z_]\w*)(\s+)(of)(\s+)([a-z_]\w*)(\s+)(is)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)([a-z_]\w*)(:)(\s+)(process|for)", vec![Some(NAME_CLASS), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups_g_to(r"(?im)(end)(\s+)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(WHITESPACE))], NewState::Push(vec![r"endblock"])),
        Rule::token(r"(?im)(b(?:it(?:(?:_vector)?)|oolean)|character|delay_length|file_open_(?:kind|status)|integer|natural|positive|s(?:everity_level|igned|t(?:d_(?:logic(?:(?:_vector)?)|ulogic(?:(?:_vector)?))|ring))|time|unsigned)\b", KEYWORD_TYPE),
        Rule::token(r"(?im)(a(?:bs|ccess|fter|l(?:ias|l)|nd|r(?:chitecture|ray)|ssert|ttribute)|b(?:egin|lock|ody|u(?:ffer|s))|c(?:ase|o(?:mponent|n(?:figuration|stant)))|d(?:isconnect|ownto)|e(?:ls(?:e|if)|n(?:d|tity)|xit)|f(?:ile|or|unction)|g(?:ener(?:ate|ic)|roup|uarded)|i(?:mpure|n(?:ertial|out)|[fns])|l(?:abel|i(?:brary|nkage|teral)|oop)|m(?:ap|od)|n(?:and|e(?:w|xt)|o(?:[rt])|ull)|o(?:pen|thers|ut|[fnr])|p(?:ackage|o(?:rt|stponed)|roce(?:dure|ss)|ure)|r(?:ange|e(?:cord|gister|ject|m|turn)|o(?:[lr]))|s(?:e(?:lect|verity)|hared|ignal|l(?:[al])|r(?:[al])|ubtype)|t(?:hen|o|ransport|ype)|u(?:n(?:its|til)|se)|variable|w(?:ait|h(?:en|ile)|ith)|x(?:(?:(?:n)?)or))\b", KEYWORD),
        Rule::token(r"(?im)\d{1,2}#[0-9a-f_]+#?", NUMBER_INTEGER),
        Rule::token(r"(?im)\d+", NUMBER_INTEGER),
        Rule::token(r"(?im)(\d+\.\d*|\.\d+|\d+)E[+-]?\d+", NUMBER_FLOAT),
        Rule::token(r#"(?im)X"[0-9a-f_]+""#, NUMBER_HEX),
        Rule::token(r#"(?im)O"[0-7_]+""#, NUMBER_OCT),
        Rule::token(r#"(?im)B"[01_]+""#, NUMBER_BIN),
        Rule::token(r"(?im)[a-z_]\w*", NAME),
    ]);
    m.insert(r"types", vec![
        Rule::token(r"(?im)(b(?:it(?:(?:_vector)?)|oolean)|character|delay_length|file_open_(?:kind|status)|integer|natural|positive|s(?:everity_level|igned|t(?:d_(?:logic(?:(?:_vector)?)|ulogic(?:(?:_vector)?))|ring))|time|unsigned)\b", KEYWORD_TYPE),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?im)(a(?:bs|ccess|fter|l(?:ias|l)|nd|r(?:chitecture|ray)|ssert|ttribute)|b(?:egin|lock|ody|u(?:ffer|s))|c(?:ase|o(?:mponent|n(?:figuration|stant)))|d(?:isconnect|ownto)|e(?:ls(?:e|if)|n(?:d|tity)|xit)|f(?:ile|or|unction)|g(?:ener(?:ate|ic)|roup|uarded)|i(?:mpure|n(?:ertial|out)|[fns])|l(?:abel|i(?:brary|nkage|teral)|oop)|m(?:ap|od)|n(?:and|e(?:w|xt)|o(?:[rt])|ull)|o(?:pen|thers|ut|[fnr])|p(?:ackage|o(?:rt|stponed)|roce(?:dure|ss)|ure)|r(?:ange|e(?:cord|gister|ject|m|turn)|o(?:[lr]))|s(?:e(?:lect|verity)|hared|ignal|l(?:[al])|r(?:[al])|ubtype)|t(?:hen|o|ransport|ype)|u(?:n(?:its|til)|se)|variable|w(?:ait|h(?:en|ile)|ith)|x(?:(?:(?:n)?)or))\b", KEYWORD),
    ]);
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?im)\d{1,2}#[0-9a-f_]+#?", NUMBER_INTEGER),
            Rule::token(r"(?im)\d+", NUMBER_INTEGER),
            Rule::token(r"(?im)(\d+\.\d*|\.\d+|\d+)E[+-]?\d+", NUMBER_FLOAT),
            Rule::token(r#"(?im)X"[0-9a-f_]+""#, NUMBER_HEX),
            Rule::token(r#"(?im)O"[0-7_]+""#, NUMBER_OCT),
            Rule::token(r#"(?im)B"[01_]+""#, NUMBER_BIN),
        ],
    );
    m.insert(r"endblock", vec![
        Rule::token(r"(?im)(a(?:bs|ccess|fter|l(?:ias|l)|nd|r(?:chitecture|ray)|ssert|ttribute)|b(?:egin|lock|ody|u(?:ffer|s))|c(?:ase|o(?:mponent|n(?:figuration|stant)))|d(?:isconnect|ownto)|e(?:ls(?:e|if)|n(?:d|tity)|xit)|f(?:ile|or|unction)|g(?:ener(?:ate|ic)|roup|uarded)|i(?:mpure|n(?:ertial|out)|[fns])|l(?:abel|i(?:brary|nkage|teral)|oop)|m(?:ap|od)|n(?:and|e(?:w|xt)|o(?:[rt])|ull)|o(?:pen|thers|ut|[fnr])|p(?:ackage|o(?:rt|stponed)|roce(?:dure|ss)|ure)|r(?:ange|e(?:cord|gister|ject|m|turn)|o(?:[lr]))|s(?:e(?:lect|verity)|hared|ignal|l(?:[al])|r(?:[al])|ubtype)|t(?:hen|o|ransport|ype)|u(?:n(?:its|til)|se)|variable|w(?:ait|h(?:en|ile)|ith)|x(?:(?:(?:n)?)or))\b", KEYWORD),
        Rule::token(r"(?im)[a-z_]\w*", NAME_CLASS),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token_to(r"(?im);", PUNCTUATION, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for VhdlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
