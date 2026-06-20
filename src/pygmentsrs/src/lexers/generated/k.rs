#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.q:KLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.q:KLexer:k

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: k
pub struct KLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)^#!.*", COMMENT_HASHBANG),
            Rule::token_to(
                r"(?m)^/\s*\n",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments"]),
            ),
            Rule::token(r"(?m)(?<!\S)/.*", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(r#"(?m)\""#, STRING_DOUBLE, NewState::Push(vec![r"strings"])),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)^#!.*", COMMENT_HASHBANG),
        Rule::token_to(r"(?m)^/\s*\n", COMMENT_MULTILINE, NewState::Push(vec![r"comments"])),
        Rule::token(r"(?m)(?<!\S)/.*", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r#"(?m)\""#, STRING_DOUBLE, NewState::Push(vec![r"strings"])),
        Rule::token(r"(?m)(a(?:bs|cos|sin|tan|vg)|b(?:in(?:(?:r)?)|y)|co(?:[rsv])|d(?:e(?:lete|v)|iv|o)|e(?:nlist|x(?:ec|it|p))|from|getenv|hopen|i(?:nsert|[fn])|l(?:ast|ike|og)|m(?:ax|in)|prd|s(?:e(?:lect|tenv)|in|qrt|s|um)|tan|update|var|w(?:avg|hile|ithin|sum)|xexp)\b", OPERATOR_WORD),
        Rule::token(r"(?m)^\\ts?", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)^(\\\w\s+[^/\n]*?)(/.*)", vec![Some(COMMENT_PREPROC), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)^\\\w.*", COMMENT_PREPROC),
        Rule::token(r"(?m)^[a-zA-Z]\)", GENERIC_PROMPT),
        Rule::bygroups_to(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)(\s*)(\{)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"functions"])),
        Rule::bygroups(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"functions"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"parentheses"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"brackets"])),
        Rule::token(r"(?m)'`([a-zA-Z][\w.]*)?", NAME_EXCEPTION),
        Rule::token(r"(?m)`:([a-zA-Z/][\w./]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)`([a-zA-Z][\w.]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)[01]+b", NUMBER_BIN),
        Rule::token(r"(?m)0[nNwW][cefghijmndzuvtp]?", NUMBER),
        Rule::token(r"(?m)(?:[0-9]{4}[.][0-9]{2}[.][0-9]{2}|[0-9]+)D(?:[0-9](?:[0-9](?::[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{4}[.][0-9]{2}(?:m|[.][0-9]{2}(?:T(?:[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]{1,3})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", NUMBER_HEX),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)([0-9]*[.]?[0-9]+|[0-9]+[.]?[0-9]*)[eE][+-]?[0-9]+[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)([0-9]*[.][0-9]+|[0-9]+[.][0-9]*)[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+[ef]", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+c", NUMBER),
        Rule::token(r"(?m)[0-9]+[ihtuv]", NUMBER_INTEGER),
        Rule::token(r"(?m)[0-9]+[jnp]?", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)[a-zA-Z][\w.]*", NAME),
        Rule::token(r"(?m)[-=+*#$%@!~^&:.,<>'\\|/?_]", OPERATOR),
        Rule::token(r"(?m);", PUNCTUATION),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(a(?:bs|cos|sin|tan|vg)|b(?:in(?:(?:r)?)|y)|co(?:[rsv])|d(?:e(?:lete|v)|iv|o)|e(?:nlist|x(?:ec|it|p))|from|getenv|hopen|i(?:nsert|[fn])|l(?:ast|ike|og)|m(?:ax|in)|prd|s(?:e(?:lect|tenv)|in|qrt|s|um)|tan|update|var|w(?:avg|hile|ithin|sum)|xexp)\b", OPERATOR_WORD),
    ]);
    m.insert(r"declarations", vec![
        Rule::token(r"(?m)^\\ts?", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)^(\\\w\s+[^/\n]*?)(/.*)", vec![Some(COMMENT_PREPROC), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)^\\\w.*", COMMENT_PREPROC),
        Rule::token(r"(?m)^[a-zA-Z]\)", GENERIC_PROMPT),
        Rule::bygroups_to(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)(\s*)(\{)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"functions"])),
        Rule::bygroups(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"functions"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"parentheses"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"brackets"])),
        Rule::token(r"(?m)'`([a-zA-Z][\w.]*)?", NAME_EXCEPTION),
        Rule::token(r"(?m)`:([a-zA-Z/][\w./]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)`([a-zA-Z][\w.]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)[01]+b", NUMBER_BIN),
        Rule::token(r"(?m)0[nNwW][cefghijmndzuvtp]?", NUMBER),
        Rule::token(r"(?m)(?:[0-9]{4}[.][0-9]{2}[.][0-9]{2}|[0-9]+)D(?:[0-9](?:[0-9](?::[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{4}[.][0-9]{2}(?:m|[.][0-9]{2}(?:T(?:[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]{1,3})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", NUMBER_HEX),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)([0-9]*[.]?[0-9]+|[0-9]+[.]?[0-9]*)[eE][+-]?[0-9]+[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)([0-9]*[.][0-9]+|[0-9]+[.][0-9]*)[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+[ef]", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+c", NUMBER),
        Rule::token(r"(?m)[0-9]+[ihtuv]", NUMBER_INTEGER),
        Rule::token(r"(?m)[0-9]+[jnp]?", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)[a-zA-Z][\w.]*", NAME),
        Rule::token(r"(?m)[-=+*#$%@!~^&:.,<>'\\|/?_]", OPERATOR),
        Rule::token(r"(?m);", PUNCTUATION),
    ]);
    m.insert(r"numbers", vec![
        Rule::token(r"(?m)[01]+b", NUMBER_BIN),
        Rule::token(r"(?m)0[nNwW][cefghijmndzuvtp]?", NUMBER),
        Rule::token(r"(?m)(?:[0-9]{4}[.][0-9]{2}[.][0-9]{2}|[0-9]+)D(?:[0-9](?:[0-9](?::[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{4}[.][0-9]{2}(?:m|[.][0-9]{2}(?:T(?:[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]{1,3})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", NUMBER_HEX),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)([0-9]*[.]?[0-9]+|[0-9]+[.]?[0-9]*)[eE][+-]?[0-9]+[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)([0-9]*[.][0-9]+|[0-9]+[.][0-9]*)[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+[ef]", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+c", NUMBER),
        Rule::token(r"(?m)[0-9]+[ihtuv]", NUMBER_INTEGER),
        Rule::token(r"(?m)[0-9]+[jnp]?", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
    ]);
    m.insert(r"functions", vec![
        Rule::token(r"(?m)^#!.*", COMMENT_HASHBANG),
        Rule::token_to(r"(?m)^/\s*\n", COMMENT_MULTILINE, NewState::Push(vec![r"comments"])),
        Rule::token(r"(?m)(?<!\S)/.*", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r#"(?m)\""#, STRING_DOUBLE, NewState::Push(vec![r"strings"])),
        Rule::token(r"(?m)(a(?:bs|cos|sin|tan|vg)|b(?:in(?:(?:r)?)|y)|co(?:[rsv])|d(?:e(?:lete|v)|iv|o)|e(?:nlist|x(?:ec|it|p))|from|getenv|hopen|i(?:nsert|[fn])|l(?:ast|ike|og)|m(?:ax|in)|prd|s(?:e(?:lect|tenv)|in|qrt|s|um)|tan|update|var|w(?:avg|hile|ithin|sum)|xexp)\b", OPERATOR_WORD),
        Rule::token(r"(?m)^\\ts?", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)^(\\\w\s+[^/\n]*?)(/.*)", vec![Some(COMMENT_PREPROC), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)^\\\w.*", COMMENT_PREPROC),
        Rule::token(r"(?m)^[a-zA-Z]\)", GENERIC_PROMPT),
        Rule::bygroups_to(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)(\s*)(\{)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"functions"])),
        Rule::bygroups(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"functions"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"parentheses"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"brackets"])),
        Rule::token(r"(?m)'`([a-zA-Z][\w.]*)?", NAME_EXCEPTION),
        Rule::token(r"(?m)`:([a-zA-Z/][\w./]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)`([a-zA-Z][\w.]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)[01]+b", NUMBER_BIN),
        Rule::token(r"(?m)0[nNwW][cefghijmndzuvtp]?", NUMBER),
        Rule::token(r"(?m)(?:[0-9]{4}[.][0-9]{2}[.][0-9]{2}|[0-9]+)D(?:[0-9](?:[0-9](?::[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{4}[.][0-9]{2}(?:m|[.][0-9]{2}(?:T(?:[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]{1,3})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", NUMBER_HEX),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)([0-9]*[.]?[0-9]+|[0-9]+[.]?[0-9]*)[eE][+-]?[0-9]+[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)([0-9]*[.][0-9]+|[0-9]+[.][0-9]*)[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+[ef]", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+c", NUMBER),
        Rule::token(r"(?m)[0-9]+[ihtuv]", NUMBER_INTEGER),
        Rule::token(r"(?m)[0-9]+[jnp]?", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)[a-zA-Z][\w.]*", NAME),
        Rule::token(r"(?m)[-=+*#$%@!~^&:.,<>'\\|/?_]", OPERATOR),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"parentheses", vec![
        Rule::token(r"(?m)^#!.*", COMMENT_HASHBANG),
        Rule::token_to(r"(?m)^/\s*\n", COMMENT_MULTILINE, NewState::Push(vec![r"comments"])),
        Rule::token(r"(?m)(?<!\S)/.*", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r#"(?m)\""#, STRING_DOUBLE, NewState::Push(vec![r"strings"])),
        Rule::token(r"(?m)(a(?:bs|cos|sin|tan|vg)|b(?:in(?:(?:r)?)|y)|co(?:[rsv])|d(?:e(?:lete|v)|iv|o)|e(?:nlist|x(?:ec|it|p))|from|getenv|hopen|i(?:nsert|[fn])|l(?:ast|ike|og)|m(?:ax|in)|prd|s(?:e(?:lect|tenv)|in|qrt|s|um)|tan|update|var|w(?:avg|hile|ithin|sum)|xexp)\b", OPERATOR_WORD),
        Rule::token(r"(?m)^\\ts?", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)^(\\\w\s+[^/\n]*?)(/.*)", vec![Some(COMMENT_PREPROC), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)^\\\w.*", COMMENT_PREPROC),
        Rule::token(r"(?m)^[a-zA-Z]\)", GENERIC_PROMPT),
        Rule::bygroups_to(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)(\s*)(\{)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"functions"])),
        Rule::bygroups(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"functions"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"parentheses"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"brackets"])),
        Rule::token(r"(?m)'`([a-zA-Z][\w.]*)?", NAME_EXCEPTION),
        Rule::token(r"(?m)`:([a-zA-Z/][\w./]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)`([a-zA-Z][\w.]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)[01]+b", NUMBER_BIN),
        Rule::token(r"(?m)0[nNwW][cefghijmndzuvtp]?", NUMBER),
        Rule::token(r"(?m)(?:[0-9]{4}[.][0-9]{2}[.][0-9]{2}|[0-9]+)D(?:[0-9](?:[0-9](?::[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{4}[.][0-9]{2}(?:m|[.][0-9]{2}(?:T(?:[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]{1,3})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", NUMBER_HEX),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)([0-9]*[.]?[0-9]+|[0-9]+[.]?[0-9]*)[eE][+-]?[0-9]+[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)([0-9]*[.][0-9]+|[0-9]+[.][0-9]*)[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+[ef]", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+c", NUMBER),
        Rule::token(r"(?m)[0-9]+[ihtuv]", NUMBER_INTEGER),
        Rule::token(r"(?m)[0-9]+[jnp]?", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)[a-zA-Z][\w.]*", NAME),
        Rule::token(r"(?m)[-=+*#$%@!~^&:.,<>'\\|/?_]", OPERATOR),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"brackets", vec![
        Rule::token(r"(?m)^#!.*", COMMENT_HASHBANG),
        Rule::token_to(r"(?m)^/\s*\n", COMMENT_MULTILINE, NewState::Push(vec![r"comments"])),
        Rule::token(r"(?m)(?<!\S)/.*", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r#"(?m)\""#, STRING_DOUBLE, NewState::Push(vec![r"strings"])),
        Rule::token(r"(?m)(a(?:bs|cos|sin|tan|vg)|b(?:in(?:(?:r)?)|y)|co(?:[rsv])|d(?:e(?:lete|v)|iv|o)|e(?:nlist|x(?:ec|it|p))|from|getenv|hopen|i(?:nsert|[fn])|l(?:ast|ike|og)|m(?:ax|in)|prd|s(?:e(?:lect|tenv)|in|qrt|s|um)|tan|update|var|w(?:avg|hile|ithin|sum)|xexp)\b", OPERATOR_WORD),
        Rule::token(r"(?m)^\\ts?", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)^(\\\w\s+[^/\n]*?)(/.*)", vec![Some(COMMENT_PREPROC), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)^\\\w.*", COMMENT_PREPROC),
        Rule::token(r"(?m)^[a-zA-Z]\)", GENERIC_PROMPT),
        Rule::bygroups_to(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)(\s*)(\{)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"functions"])),
        Rule::bygroups(r"(?m)([.]?[a-zA-Z][\w.]*)(\s*)([-.~=!@#$%^&*_+|,<>?/\\:']?:)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"functions"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"parentheses"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"brackets"])),
        Rule::token(r"(?m)'`([a-zA-Z][\w.]*)?", NAME_EXCEPTION),
        Rule::token(r"(?m)`:([a-zA-Z/][\w./]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)`([a-zA-Z][\w.]*)?", STRING_SYMBOL),
        Rule::token(r"(?m)[01]+b", NUMBER_BIN),
        Rule::token(r"(?m)0[nNwW][cefghijmndzuvtp]?", NUMBER),
        Rule::token(r"(?m)(?:[0-9]{4}[.][0-9]{2}[.][0-9]{2}|[0-9]+)D(?:[0-9](?:[0-9](?::[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{4}[.][0-9]{2}(?:m|[.][0-9]{2}(?:T(?:[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]*)?)?)?)?)", LITERAL_DATE),
        Rule::token(r"(?m)[0-9]{2}:[0-9]{2}(?::[0-9]{2}(?:[.][0-9]{1,3})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", NUMBER_HEX),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)([0-9]*[.]?[0-9]+|[0-9]+[.]?[0-9]*)[eE][+-]?[0-9]+[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)([0-9]*[.][0-9]+|[0-9]+[.][0-9]*)[ef]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+[ef]", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+c", NUMBER),
        Rule::token(r"(?m)[0-9]+[ihtuv]", NUMBER_INTEGER),
        Rule::token(r"(?m)[0-9]+[jnp]?", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)[a-zA-Z][\w.]*", NAME),
        Rule::token(r"(?m)[-=+*#$%@!~^&:.,<>'\\|/?_]", OPERATOR),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)[^\\]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)^\\", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)\\", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"strings",
        vec![
            Rule::token(r#"(?m)[^"\\]+"#, STRING_DOUBLE),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for KLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
