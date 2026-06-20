#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.tablegen:TableGenLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.tablegen:TableGenLexer:tablegen

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tablegen, td
pub struct TablegenLexer;

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
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)#(define|ifdef|ifndef|else|endif)", COMMENT_PREPROC),
        Rule::token(r"(?m)0b[10]+", NUMBER_BIN),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)(assert|c(?:lass|ode)|d(?:ef(?:(?:m|set|var)?)|ump)|else|f(?:ield|oreach)|i(?:nclude|[fn])|let|multiclass|then)\b", KEYWORD),
        Rule::token(r"(?m)((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(bit(?:(?:s)?)|dag|int|list|string)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\!(a(?:(?:[dn])d)|c(?:ast|on(?:(?:d)?))|d(?:ag|iv)|e(?:mpty|q|xists)|f(?:i(?:lter|nd)|o(?:ldl|reach))|g(?:etdag(?:arg|name|op)|[et])|head|i(?:f|nterleave|sa)|l(?:ist(?:concat|remove|splat)|ogtwo|[et])|mul|n(?:e|ot)|or|r(?:ange|epr)|s(?:etdag(?:arg|name|op)|hl|ize|r(?:[al])|trconcat|ub(?:(?:st(?:(?:r)?))?))|t(?:ail|o(?:(?:low|upp)er))|xor)\b", OPERATOR),
        Rule::token(r"(?m)![a-zA-Z]+", ERROR),
        Rule::token(r"(?m)[0-9]*[a-zA-Z_][a-zA-Z_0-9]*", NAME),
        Rule::token(r"(?m)\$[a-zA-Z_][a-zA-Z_0-9]*", NAME_VARIABLE),
        Rule::token(r"(?m)[-\+]?[0-9]+", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?m)\[\{", PUNCTUATION, NewState::Push(vec![r"codeblock"])),
        Rule::token(r"(?m)[-+\[\]{}()<>\.,;:=?#]+", PUNCTUATION),
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
    m.insert(
        r"strings",
        vec![
            Rule::token(r#"(?m)\\[\\\'"tn]"#, STRING_ESCAPE),
            Rule::token(r#"(?m)[^\\"]+"#, STRING),
        ],
    );
    m.insert(
        r"dqs",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)\\[\\\'"tn]"#, STRING_ESCAPE),
            Rule::token(r#"(?m)[^\\"]+"#, STRING),
        ],
    );
    m.insert(
        r"codeblock",
        vec![
            Rule::token_to(r"(?m)\}\]", TEXT, NewState::Pop(1)),
            Rule::using_lexer_to(r"(?m)([^}]+|\}[^\]])*", "cpp", None, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for TablegenLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
