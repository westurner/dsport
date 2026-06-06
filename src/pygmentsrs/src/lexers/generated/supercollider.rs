//! AUTO-GENERATED from `pygments.pygments.lexers.supercollider:SuperColliderLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.supercollider:SuperColliderLexer:supercollider

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: supercollider, sc
pub struct SupercolliderLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"commentsandwhitespace", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)<!--", COMMENT),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
    ]);
    m.insert(r"slashstartsregex", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)<!--", COMMENT),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token_to(r"(?ms)/(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gim]+\b|\B)", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?=/)", TEXT, NewState::Push(vec![r"#pop", r"badregex"])),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"badregex", vec![
        Rule::token_to(r"(?ms)\n", TEXT, NewState::Pop(1)),
    ]);
    m.insert(r"root", vec![
        Rule::token_to(r"(?ms)^(?=\s|/|<!--)", TEXT, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)<!--", COMMENT),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token_to(r"(?ms)\+\+|--|~|&&|\?|:|\|\||\\(?=\n)|(<<|>>>?|==?|!=?|[-<>+*%&|^/])=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[})\].]", PUNCTUATION),
        Rule::token_to(r"(?ms)(break|c(?:a(?:se|tch)|ontinue)|d(?:e(?:fault|lete)|o)|else|f(?:inally|or)|i(?:nstanceof|[fn])|new|return|switch|t(?:hrow|ry|ypeof)|void|while)\b", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)(arg|function|let|var|with)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)(\(abstract|b(?:oolean|yte)|c(?:har|lass|onst)|d(?:ebugger|ouble)|e(?:num|x(?:port|tends))|f(?:inal|loat)|goto|i(?:mp(?:lements|ort)|nt(?:(?:erface)?))|long|native|p(?:ackage|r(?:ivate|otected)|ublic)|s(?:hort|tatic|uper|ynchronized)|t(?:hrows|ransient)|volatile)\b", KEYWORD_RESERVED),
        Rule::token(r"(?ms)(false|inf|nil|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Number|Object|Packages|RegExp|String|is(?:Finite|NaN)|parse(?:(?:Floa|In)t)|super|this(?:(?:Function(?:(?:Def)?)|Method|Process|Thread)?))\b", NAME_BUILTIN),
        Rule::token(r"(?ms)[$a-zA-Z_]\w*", NAME_OTHER),
        Rule::token(r"(?ms)\\?[$a-zA-Z_]\w*", STRING_SYMBOL),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
    ]);
    Table(m)
}

impl Lexer for SupercolliderLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
