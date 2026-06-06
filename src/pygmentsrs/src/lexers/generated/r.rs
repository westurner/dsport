//! AUTO-GENERATED from `pygments.pygments.lexers.r:SLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.r:SLexer:r

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: splus, s, r
pub struct RLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"comments", vec![
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
    ]);
    m.insert(r"valid_name", vec![
        Rule::token(r"(?m)`[^`\\]*(?:\\.[^`\\]*)*`|(?:[a-zA-Z]|\.[A-Za-z_.])[\w.]*|\.", NAME),
    ]);
    m.insert(r"function_name", vec![
        Rule::token(r"(?m)(`[^`\\]*(?:\\.[^`\\]*)*`|(?:[a-zA-Z]|\.[A-Za-z_.])[\w.]*|\.)\s*(?=\()", NAME_FUNCTION),
    ]);
    m.insert(r"punctuation", vec![
        Rule::token(r"(?m)\[{1,2}|\]{1,2}|\(|\)|;|,", PUNCTUATION),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(if|else|for|while|repeat|in|next|break|return|switch|function)(?![\w.])", KEYWORD_RESERVED),
    ]);
    m.insert(r"operators", vec![
        Rule::token(r"(?m)<<?-|->>?|-|==|<=|>=|\|>|<|>|&&?|!=|\|\|?|\?", OPERATOR),
        Rule::token(r"(?m)\*|\+|\^|/|!|%[^%]*%|=|~|\$|@|:{1,3}", OPERATOR),
    ]);
    m.insert(r"builtin_symbols", vec![
        Rule::token(r"(?m)(NULL|NA(_(integer|real|complex|character)_)?|letters|LETTERS|Inf|TRUE|FALSE|NaN|pi|\.\.(\.|[0-9]+))(?![\w.])", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(T|F)\b", NAME_BUILTIN_PSEUDO),
    ]);
    m.insert(r"numbers", vec![
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+([pP][0-9]+)?[Li]?", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?([0-9]+(\.[0-9]+)?|\.[0-9]+|\.)([eE][+-]?[0-9]+)?[Li]?", NUMBER),
    ]);
    m.insert(r"statements", vec![
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)\'", STRING, NewState::Push(vec![r"string_squote"])),
        Rule::token_to(r#"(?m)\""#, STRING, NewState::Push(vec![r"string_dquote"])),
        Rule::token(r"(?m)(NULL|NA(_(integer|real|complex|character)_)?|letters|LETTERS|Inf|TRUE|FALSE|NaN|pi|\.\.(\.|[0-9]+))(?![\w.])", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(T|F)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(if|else|for|while|repeat|in|next|break|return|switch|function)(?![\w.])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(`[^`\\]*(?:\\.[^`\\]*)*`|(?:[a-zA-Z]|\.[A-Za-z_.])[\w.]*|\.)\s*(?=\()", NAME_FUNCTION),
        Rule::token(r"(?m)`[^`\\]*(?:\\.[^`\\]*)*`|(?:[a-zA-Z]|\.[A-Za-z_.])[\w.]*|\.", NAME),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+([pP][0-9]+)?[Li]?", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?([0-9]+(\.[0-9]+)?|\.[0-9]+|\.)([eE][+-]?[0-9]+)?[Li]?", NUMBER),
        Rule::token(r"(?m)\[{1,2}|\]{1,2}|\(|\)|;|,", PUNCTUATION),
        Rule::token(r"(?m)<<?-|->>?|-|==|<=|>=|\|>|<|>|&&?|!=|\|\|?|\?", OPERATOR),
        Rule::token(r"(?m)\*|\+|\^|/|!|%[^%]*%|=|~|\$|@|:{1,3}", OPERATOR),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)\'", STRING, NewState::Push(vec![r"string_squote"])),
        Rule::token_to(r#"(?m)\""#, STRING, NewState::Push(vec![r"string_dquote"])),
        Rule::token(r"(?m)(NULL|NA(_(integer|real|complex|character)_)?|letters|LETTERS|Inf|TRUE|FALSE|NaN|pi|\.\.(\.|[0-9]+))(?![\w.])", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(T|F)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(if|else|for|while|repeat|in|next|break|return|switch|function)(?![\w.])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(`[^`\\]*(?:\\.[^`\\]*)*`|(?:[a-zA-Z]|\.[A-Za-z_.])[\w.]*|\.)\s*(?=\()", NAME_FUNCTION),
        Rule::token(r"(?m)`[^`\\]*(?:\\.[^`\\]*)*`|(?:[a-zA-Z]|\.[A-Za-z_.])[\w.]*|\.", NAME),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+([pP][0-9]+)?[Li]?", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?([0-9]+(\.[0-9]+)?|\.[0-9]+|\.)([eE][+-]?[0-9]+)?[Li]?", NUMBER),
        Rule::token(r"(?m)\[{1,2}|\]{1,2}|\(|\)|;|,", PUNCTUATION),
        Rule::token(r"(?m)<<?-|->>?|-|==|<=|>=|\|>|<|>|&&?|!=|\|\|?|\?", OPERATOR),
        Rule::token(r"(?m)\*|\+|\^|/|!|%[^%]*%|=|~|\$|@|:{1,3}", OPERATOR),
        Rule::token(r"(?m)\{|\}", PUNCTUATION),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"string_squote", vec![
        Rule::token_to(r"(?m)([^\'\\]|\\.)*\'", STRING, NewState::Pop(1)),
    ]);
    m.insert(r"string_dquote", vec![
        Rule::token_to(r#"(?m)([^"\\]|\\.)*""#, STRING, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for RLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
