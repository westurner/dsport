#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:ZeekLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:ZeekLexer:zeek

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: zeek, bro
pub struct ZeekLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m)@(load-plugin|load-sigs|load|unload)\b.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)@(DEBUG|DIR|FILENAME|deprecated|if|ifdef|ifndef|else|endif)\b", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)(@prefixes)(\s*)((\+?=).*)$", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::token(r"(?m)&(add_func|create_expire|de(?:fault|lete_func|precated)|e(?:ncrypt|rror_handler|xpire_func)|log|mergeable|optional|p(?:ersistent|riority)|r(?:aw_output|e(?:ad_expire|def)|otate_(?:interval|size))|synchronized|type_column|write_expire)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)(a(?:ddr|ny)|bool|count|double|e(?:num|vent)|f(?:ile|unction)|hook|int(?:(?:erval)?)|p(?:attern|ort)|record|s(?:et|tring|ubnet)|t(?:(?:abl|im)e)|vector)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?m)(opaque)(\s+)(of)(\s+)((?:[A-Za-z_]\w*)(?:::(?:[A-Za-z_]\w*))*)\b", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(OPERATOR_WORD), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)(type)(\s+)((?:[A-Za-z_]\w*)(?:::(?:[A-Za-z_]\w*))*)(\s*)(:)(\s*)\b(record|enum)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)(type)(\s+)((?:[A-Za-z_]\w*)(?:::(?:[A-Za-z_]\w*))*)(\s*)(:)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(redef)(\s+)(record|enum)(\s+)((?:[A-Za-z_]\w*)(?:::(?:[A-Za-z_]\w*))*)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::token(r"(?m)(add|break|c(?:(?:as|ontinu)e)|de(?:fault|lete)|e(?:lse|xport)|f(?:allthrough|or)|if|next|re(?:def|turn)|s(?:chedule|witch)|timeout|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?m)(print)\b", KEYWORD),
        Rule::token(r"(?m)(global|local|const|option)\b", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?m)(module)(\s+)(([A-Za-z_]\w*)(?:::([A-Za-z_]\w*))*)\b", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)/(?=.*/)", STRING_REGEX, NewState::Push(vec![r"regex"])),
        Rule::token(r"(?m)(T|F)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\d{1,5}/(udp|tcp|icmp|unknown)\b", NUMBER),
        Rule::token(r"(?m)(\d{1,3}.){3}(\d{1,3})\b", NUMBER),
        Rule::token(r"(?m)\[([0-9a-fA-F]{0,4}:){2,7}([0-9a-fA-F]{0,4})?((\d{1,3}.){3}(\d{1,3}))?\]", NUMBER),
        Rule::token(r"(?m)0[xX][0-9a-fA-F]+\b", NUMBER_HEX),
        Rule::token(r"(?m)((\d*\.?\d+)|(\d+\.?\d*))([eE][-+]?\d+)?\s*(day|hr|min|sec|msec|usec)s?\b", NUMBER_FLOAT),
        Rule::token(r"(?m)((\d*\.?\d+)|(\d+\.?\d*))([eE][-+]?\d+)?\b", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+)\b", NUMBER_INTEGER),
        Rule::token(r"(?m)[A-Za-z0-9][-A-Za-z0-9]*(\.[A-Za-z0-9][-A-Za-z0-9]*)+", STRING),
        Rule::token(r"(?m)[!%*/+<=>~|&^-]", OPERATOR),
        Rule::token(r"(?m)([-+=&|]{2}|[+=!><-]=)", OPERATOR),
        Rule::token(r"(?m)(in|as|is|of)\b", OPERATOR_WORD),
        Rule::token(r"(?m)\??\$", OPERATOR),
        Rule::token(r"(?m)[{}()\[\],;.]", PUNCTUATION),
        Rule::token(r"(?m)[?:]", PUNCTUATION),
        Rule::token(r"(?m)((?:[A-Za-z_]\w*)(?:::(?:[A-Za-z_]\w*))*)(?=\s*\()", NAME_FUNCTION),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(::)", vec![Some(NAME), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        ],
    );
    m.insert(r"comments", vec![Rule::token(r"(?m)#.*$", COMMENT)]);
    m.insert(
        r"directives",
        vec![
            Rule::token(
                r"(?m)@(load-plugin|load-sigs|load|unload)\b.*$",
                COMMENT_PREPROC,
            ),
            Rule::token(
                r"(?m)@(DEBUG|DIR|FILENAME|deprecated|if|ifdef|ifndef|else|endif)\b",
                COMMENT_PREPROC,
            ),
            Rule::bygroups(
                r"(?m)(@prefixes)(\s*)((\+?=).*)$",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(WHITESPACE),
                    Some(COMMENT_PREPROC),
                ],
            ),
        ],
    );
    m.insert(r"attributes", vec![
        Rule::token(r"(?m)&(add_func|create_expire|de(?:fault|lete_func|precated)|e(?:ncrypt|rror_handler|xpire_func)|log|mergeable|optional|p(?:ersistent|riority)|r(?:aw_output|e(?:ad_expire|def)|otate_(?:interval|size))|synchronized|type_column|write_expire)\b", KEYWORD_PSEUDO),
    ]);
    m.insert(r"types", vec![
        Rule::token(r"(?m)(a(?:ddr|ny)|bool|count|double|e(?:num|vent)|f(?:ile|unction)|hook|int(?:(?:erval)?)|p(?:attern|ort)|record|s(?:et|tring|ubnet)|t(?:(?:abl|im)e)|vector)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?m)(opaque)(\s+)(of)(\s+)((?:[A-Za-z_]\w*)(?:::(?:[A-Za-z_]\w*))*)\b", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(OPERATOR_WORD), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)(type)(\s+)((?:[A-Za-z_]\w*)(?:::(?:[A-Za-z_]\w*))*)(\s*)(:)(\s*)\b(record|enum)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)(type)(\s+)((?:[A-Za-z_]\w*)(?:::(?:[A-Za-z_]\w*))*)(\s*)(:)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(redef)(\s+)(record|enum)(\s+)((?:[A-Za-z_]\w*)(?:::(?:[A-Za-z_]\w*))*)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(NAME_CLASS)]),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(add|break|c(?:(?:as|ontinu)e)|de(?:fault|lete)|e(?:lse|xport)|f(?:allthrough|or)|if|next|re(?:def|turn)|s(?:chedule|witch)|timeout|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?m)(print)\b", KEYWORD),
        Rule::token(r"(?m)(global|local|const|option)\b", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?m)(module)(\s+)(([A-Za-z_]\w*)(?:::([A-Za-z_]\w*))*)\b", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
    ]);
    m.insert(
        r"literals",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token_to(
                r"(?m)/(?=.*/)",
                STRING_REGEX,
                NewState::Push(vec![r"regex"]),
            ),
            Rule::token(r"(?m)(T|F)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m)\d{1,5}/(udp|tcp|icmp|unknown)\b", NUMBER),
            Rule::token(r"(?m)(\d{1,3}.){3}(\d{1,3})\b", NUMBER),
            Rule::token(
                r"(?m)\[([0-9a-fA-F]{0,4}:){2,7}([0-9a-fA-F]{0,4})?((\d{1,3}.){3}(\d{1,3}))?\]",
                NUMBER,
            ),
            Rule::token(r"(?m)0[xX][0-9a-fA-F]+\b", NUMBER_HEX),
            Rule::token(
                r"(?m)((\d*\.?\d+)|(\d+\.?\d*))([eE][-+]?\d+)?\s*(day|hr|min|sec|msec|usec)s?\b",
                NUMBER_FLOAT,
            ),
            Rule::token(
                r"(?m)((\d*\.?\d+)|(\d+\.?\d*))([eE][-+]?\d+)?\b",
                NUMBER_FLOAT,
            ),
            Rule::token(r"(?m)(\d+)\b", NUMBER_INTEGER),
            Rule::token(
                r"(?m)[A-Za-z0-9][-A-Za-z0-9]*(\.[A-Za-z0-9][-A-Za-z0-9]*)+",
                STRING,
            ),
        ],
    );
    m.insert(
        r"operators",
        vec![
            Rule::token(r"(?m)[!%*/+<=>~|&^-]", OPERATOR),
            Rule::token(r"(?m)([-+=&|]{2}|[+=!><-]=)", OPERATOR),
            Rule::token(r"(?m)(in|as|is|of)\b", OPERATOR_WORD),
            Rule::token(r"(?m)\??\$", OPERATOR),
        ],
    );
    m.insert(
        r"punctuation",
        vec![
            Rule::token(r"(?m)[{}()\[\],;.]", PUNCTUATION),
            Rule::token(r"(?m)[?:]", PUNCTUATION),
        ],
    );
    m.insert(
        r"identifiers",
        vec![
            Rule::bygroups(
                r"(?m)([a-zA-Z_]\w*)(::)",
                vec![Some(NAME), Some(PUNCTUATION)],
            ),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r"(?m)%-?[0-9]*(\.[0-9]+)?[DTd-gsx]", STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r"(?m).", STRING),
        ],
    );
    m.insert(
        r"regex",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r"(?m)/", STRING_REGEX, NewState::Pop(1)),
            Rule::token(r"(?m).", STRING_REGEX),
        ],
    );
    Table(m)
}

impl Lexer for ZeekLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
