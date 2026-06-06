//! AUTO-GENERATED from `pygments.pygments.lexers.typst:TypstLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.typst:TypstLexer:typst

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: typst
pub struct TypstLexer;

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
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token(r"(?m)^\s*=+.*$", GENERIC_HEADING),
        Rule::token(r"(?m)[*][^*]*[*]", GENERIC_STRONG),
        Rule::token(r"(?m)_[^_]*_", GENERIC_EMPH),
        Rule::token_to(r"(?m)\$", PUNCTUATION, NewState::Push(vec![r"math"])),
        Rule::token(r"(?m)`[^`]*`", STRING_BACKTICK),
        Rule::bygroups(r"(?m)^(\s*)(-)(\s+)", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)^(\s*)(\+)(\s+)", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)^(\s*)([0-9]+\.)", vec![Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)^(\s*)(/)(\s+)([^:]+)(:)", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_VARIABLE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)<[a-zA-Z_][a-zA-Z0-9_-]*>", NAME_LABEL),
        Rule::token(r"(?m)@[a-zA-Z_][a-zA-Z0-9_-]*", NAME_LABEL),
        Rule::token(r"(?m)\\#", TEXT),
        Rule::token_to(r"(?m)(\#(?:let|s(?:et|how)))\b", KEYWORD_DECLARATION, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)(\#i(?:mport|nclude))\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)(\#(?:export|for|if|while))\b", KEYWORD_RESERVED, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::token_to(r"(?m)#\(", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::bygroups_to(r"(?m)(#[a-zA-Z_][a-zA-Z0-9_-]*)(\[)", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"markup"])),
        Rule::bygroups_to(r"(?m)(#[a-zA-Z_][a-zA-Z0-9_-]*)(\()", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"code"])),
        Rule::token(r"(?m)(\#(?:auto|(?:fals|non|tru)e))\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)#[a-zA-Z_][a-zA-Z0-9_]*", NAME_VARIABLE),
        Rule::token(r"(?m)#0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)#0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)#0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)#[0-9]+[\.e][0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)#[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)```(?:.|\n)*?```", STRING_BACKTICK),
        Rule::token(r"(?m)https?://[0-9a-zA-Z~/%#&=\',;.+?]*", GENERIC_EMPH),
        Rule::token(r"(?m)(\-\-(?:(?:\-)?)|\.\.\.|[\\~])\B", PUNCTUATION),
        Rule::token(r"(?m)\\\[", PUNCTUATION),
        Rule::token(r"(?m)\\\]", PUNCTUATION),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)[ \t]+\n?|\n", WHITESPACE),
        Rule::token(r"(?m)((?![*_$`<@\\#\] ]|https?://).)+", TEXT),
    ]);
    m.insert(r"markup", vec![
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token(r"(?m)^\s*=+.*$", GENERIC_HEADING),
        Rule::token(r"(?m)[*][^*]*[*]", GENERIC_STRONG),
        Rule::token(r"(?m)_[^_]*_", GENERIC_EMPH),
        Rule::token_to(r"(?m)\$", PUNCTUATION, NewState::Push(vec![r"math"])),
        Rule::token(r"(?m)`[^`]*`", STRING_BACKTICK),
        Rule::bygroups(r"(?m)^(\s*)(-)(\s+)", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)^(\s*)(\+)(\s+)", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)^(\s*)([0-9]+\.)", vec![Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)^(\s*)(/)(\s+)([^:]+)(:)", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_VARIABLE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)<[a-zA-Z_][a-zA-Z0-9_-]*>", NAME_LABEL),
        Rule::token(r"(?m)@[a-zA-Z_][a-zA-Z0-9_-]*", NAME_LABEL),
        Rule::token(r"(?m)\\#", TEXT),
        Rule::token_to(r"(?m)(\#(?:let|s(?:et|how)))\b", KEYWORD_DECLARATION, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)(\#i(?:mport|nclude))\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)(\#(?:export|for|if|while))\b", KEYWORD_RESERVED, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::token_to(r"(?m)#\(", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::bygroups_to(r"(?m)(#[a-zA-Z_][a-zA-Z0-9_-]*)(\[)", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"markup"])),
        Rule::bygroups_to(r"(?m)(#[a-zA-Z_][a-zA-Z0-9_-]*)(\()", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"code"])),
        Rule::token(r"(?m)(\#(?:auto|(?:fals|non|tru)e))\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)#[a-zA-Z_][a-zA-Z0-9_]*", NAME_VARIABLE),
        Rule::token(r"(?m)#0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)#0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)#0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)#[0-9]+[\.e][0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)#[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)```(?:.|\n)*?```", STRING_BACKTICK),
        Rule::token(r"(?m)https?://[0-9a-zA-Z~/%#&=\',;.+?]*", GENERIC_EMPH),
        Rule::token(r"(?m)(\-\-(?:(?:\-)?)|\.\.\.|[\\~])\B", PUNCTUATION),
        Rule::token(r"(?m)\\\[", PUNCTUATION),
        Rule::token(r"(?m)\\\]", PUNCTUATION),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)[ \t]+\n?|\n", WHITESPACE),
        Rule::token(r"(?m)((?![*_$`<@\\#\] ]|https?://).)+", TEXT),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
    ]);
    m.insert(r"into_code", vec![
        Rule::token_to(r"(?m)(\#(?:let|s(?:et|how)))\b", KEYWORD_DECLARATION, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)(\#i(?:mport|nclude))\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)(\#(?:export|for|if|while))\b", KEYWORD_RESERVED, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::token_to(r"(?m)#\(", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::bygroups_to(r"(?m)(#[a-zA-Z_][a-zA-Z0-9_-]*)(\[)", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"markup"])),
        Rule::bygroups_to(r"(?m)(#[a-zA-Z_][a-zA-Z0-9_-]*)(\()", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"code"])),
        Rule::token(r"(?m)(\#(?:auto|(?:fals|non|tru)e))\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)#[a-zA-Z_][a-zA-Z0-9_]*", NAME_VARIABLE),
        Rule::token(r"(?m)#0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)#0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)#0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)#[0-9]+[\.e][0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)#[0-9]+", NUMBER_INTEGER),
    ]);
    m.insert(r"math", vec![
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(\\(?:[&\^_]))", TEXT),
        Rule::token(r"(?m)([&;\^_])", PUNCTUATION),
        Rule::token(r"(?m)(!=|\-(?:(?:(?:[\->])?)>)|\.\.\.|:(?:(?:(?::)?)=)|<(?:\-(?:\->|[\-<>])|<(?:[\-<])|=(?:=>|[=>])|\~\~|[\-<=~])|=(?:=>|[:>])|>(?:\->|>>|[=>])|\[\||\|(?:\->|=>|[\]|])|\~(?:(?:(?:\~)?)>)|['*+\-/:<=>|~])", OPERATOR),
        Rule::token(r"(?m)\\", PUNCTUATION),
        Rule::token(r"(?m)\\\$", PUNCTUATION),
        Rule::token_to(r"(?m)\$", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)(\#(?:let|s(?:et|how)))\b", KEYWORD_DECLARATION, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)(\#i(?:mport|nclude))\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)(\#(?:export|for|if|while))\b", KEYWORD_RESERVED, NewState::Push(vec![r"inline_code"])),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::token_to(r"(?m)#\(", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::bygroups_to(r"(?m)(#[a-zA-Z_][a-zA-Z0-9_-]*)(\[)", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"markup"])),
        Rule::bygroups_to(r"(?m)(#[a-zA-Z_][a-zA-Z0-9_-]*)(\()", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"code"])),
        Rule::token(r"(?m)(\#(?:auto|(?:fals|non|tru)e))\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)#[a-zA-Z_][a-zA-Z0-9_]*", NAME_VARIABLE),
        Rule::token(r"(?m)#0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)#0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)#0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)#[0-9]+[\.e][0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)#[0-9]+", NUMBER_INTEGER),
        Rule::bygroups(r"(?m)([a-zA-Z][a-zA-Z0-9-]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)([a-zA-Z][a-zA-Z0-9-]*)(:)", vec![Some(NAME_VARIABLE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)([a-zA-Z][a-zA-Z0-9-]*)", NAME_VARIABLE),
        Rule::token(r"(?m)[0-9]+(\.[0-9]+)?", NUMBER),
        Rule::token(r"(?m)\.{1,3}|\(|\)|,|\{|\}", PUNCTUATION),
        Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?m)[ \t\n]+", WHITESPACE),
    ]);
    m.insert(r"code", vec![
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"markup"])),
        Rule::token_to(r"(?m)\(|\{", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::token_to(r"(?m)\)|\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?m),|\.{1,2}", PUNCTUATION),
        Rule::token(r"(?m)=", OPERATOR),
        Rule::token(r"(?m)(and|not|or)\b", OPERATOR_WORD),
        Rule::token(r"(?m)=>|<=|==|!=|>|<|-=|\+=|\*=|/=|\+|-|\\|\*", OPERATOR),
        Rule::bygroups(r"(?m)([a-zA-Z_][a-zA-Z0-9_-]*)(:)", vec![Some(NAME_VARIABLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?m)([a-zA-Z_][a-zA-Z0-9_-]*)(\()", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"code"])),
        Rule::token(r"(?m)(as|break|continue|e(?:lse|xport)|for|i(?:[fn])|return|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(i(?:mport|nclude))\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(auto|(?:fals|non|tru)e)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?m)([0-9.]+)(mm|pt|cm|in|em|fr|%)", vec![Some(NUMBER), Some(KEYWORD_RESERVED)]),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)[0-9]+[\.e][0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)(let|s(?:et|how))\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)([a-zA-Z_][a-zA-Z0-9_-]*)", NAME_VARIABLE),
        Rule::token(r"(?m)[ \t\n]+", WHITESPACE),
        Rule::token(r"(?m):", PUNCTUATION),
    ]);
    m.insert(r"inline_code", vec![
        Rule::token_to(r"(?m);\b", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"markup"])),
        Rule::token_to(r"(?m)\(|\{", PUNCTUATION, NewState::Push(vec![r"code"])),
        Rule::token_to(r"(?m)\)|\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?m),|\.{1,2}", PUNCTUATION),
        Rule::token(r"(?m)=", OPERATOR),
        Rule::token(r"(?m)(and|not|or)\b", OPERATOR_WORD),
        Rule::token(r"(?m)=>|<=|==|!=|>|<|-=|\+=|\*=|/=|\+|-|\\|\*", OPERATOR),
        Rule::bygroups(r"(?m)([a-zA-Z_][a-zA-Z0-9_-]*)(:)", vec![Some(NAME_VARIABLE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?m)([a-zA-Z_][a-zA-Z0-9_-]*)(\()", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"code"])),
        Rule::token(r"(?m)(as|break|continue|e(?:lse|xport)|for|i(?:[fn])|return|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(i(?:mport|nclude))\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(auto|(?:fals|non|tru)e)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?m)([0-9.]+)(mm|pt|cm|in|em|fr|%)", vec![Some(NUMBER), Some(KEYWORD_RESERVED)]),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)[0-9]+[\.e][0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)(let|s(?:et|how))\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)([a-zA-Z_][a-zA-Z0-9_-]*)", NAME_VARIABLE),
        Rule::token(r"(?m)[ \t\n]+", WHITESPACE),
        Rule::token(r"(?m):", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for TypstLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
