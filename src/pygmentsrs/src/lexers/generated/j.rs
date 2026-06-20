#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.j:JLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.j:JLexer:j

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: j
pub struct JLexer;

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
        Rule::token(r"(?m)#!.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)NB\..*", COMMENT_SINGLE),
        Rule::bygroups_to(r"(?m)(\n+\s*)(Note)", vec![Some(WHITESPACE), Some(COMMENT_MULTILINE)], NewState::Push(vec![r"comment"])),
        Rule::bygroups(r"(?m)(\s*)(Note.*)", vec![Some(WHITESPACE), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlequote"])),
        Rule::token_to(r"(?m)0\s+:\s*0", NAME_ENTITY, NewState::Push(vec![r"nounDefinition"])),
        Rule::bygroups_to(r"(?m)(noun)(\s+)(define)(\s*)$", vec![Some(NAME_ENTITY), Some(WHITESPACE), Some(NAME_ENTITY), Some(WHITESPACE)], NewState::Push(vec![r"nounDefinition"])),
        Rule::token_to(r"(?m)([1-4]|13)\s+:\s*0\b", NAME_FUNCTION, NewState::Push(vec![r"explicitDefinition"])),
        Rule::bygroups_to(r"(?m)(adverb|conjunction|dyad|monad|verb)(\s+)(define)\b", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_FUNCTION)], NewState::Push(vec![r"explicitDefinition"])),
        Rule::token(r"(?m)((?:for|goto|label)_)\b[a-zA-Z]\w*\.", NAME_LABEL),
        Rule::token(r"(?m)(assert|break|c(?:a(?:se|tch(?:(?:[dt])?))|ontinue)|do|e(?:lse(?:(?:if)?)|nd)|f(?:case|or)|if|return|select|t(?:hrow|ry)|whil(?:e|st))\.", NAME_LABEL),
        Rule::token(r"(?m)\b[a-zA-Z]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)(ARGV|CR(?:(?:LF)?)|D(?:EL|ebug)|E(?:AV|MPTY)|FF|JVERSION|LF(?:(?:2)?)|Note|TAB|a(?:lpha(?:(?:[12])7)|pply)|b(?:ind|ox(?:(?:(?:x)?)open)|x)|c(?:lear|ut(?:LF|open))|d(?:atatype|ef|fh|rop)|e(?:ach|cho|mpty|rase|v(?:ery|tloop)|x(?:it|pand))|f(?:etch|i(?:le2url|xdotdot)|liprgb)|get(?:args|env)|hfd|i(?:nv(?:(?:erse)?)|ospath|s(?:atty|utf8)|tems)|l(?:eaf|ist)|n(?:ame(?:class|list|s)|[cl])|on|pick|rows|s(?:cript(?:(?:d)?)|ign|m(?:info|output)|ort|plit|td(?:err|in|out))|t(?:a(?:(?:bl|k)e)|ime(?:(?:(?:space)?)x)|moutput|o(?:CRLF|HOST|J|(?:low|upp)er)|ype)|u(?:cp(?:(?:count)?)|sleep|tf8|ucp))", NAME_FUNCTION),
        Rule::token(r"(?m)=[.:]", OPERATOR),
        Rule::token(r#"(?m)[-=+*#$%@!~`^&";:.,<>{}\[\]\\|/?]"#, OPERATOR),
        Rule::token(r"(?m)[abCdDeEfHiIjLMoprtT]\.", KEYWORD_RESERVED),
        Rule::token(r"(?m)[aDiLpqsStux]\:", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_[0-9])\:", KEYWORD_CONSTANT),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"parentheses"])),
        Rule::token(r"(?m)\b_{1,2}\b", NUMBER),
        Rule::token(r"(?m)_?\d+(\.\d+)?(\s*[ejr]\s*)_?\d+(\.?=\d+)?", NUMBER),
        Rule::token(r"(?m)_?\d+\.(?=\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)_?\d+x", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)_?\d+", NUMBER_INTEGER),
    ]);
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)\b_{1,2}\b", NUMBER),
            Rule::token(r"(?m)_?\d+(\.\d+)?(\s*[ejr]\s*)_?\d+(\.?=\d+)?", NUMBER),
            Rule::token(r"(?m)_?\d+\.(?=\d+)", NUMBER_FLOAT),
            Rule::token(
                r"(?m)_?\d+x",
                TokenType::new(&["Literal", "Number", "Integer", "Long"]),
            ),
            Rule::token(r"(?m)_?\d+", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^)]", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)^\)", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[)]", COMMENT_MULTILINE),
        ],
    );
    m.insert(r"explicitDefinition", vec![
        Rule::token(r"(?m)\b[nmuvxy]\b", NAME_DECORATOR),
        Rule::token(r"(?m)#!.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)NB\..*", COMMENT_SINGLE),
        Rule::bygroups_to(r"(?m)(\n+\s*)(Note)", vec![Some(WHITESPACE), Some(COMMENT_MULTILINE)], NewState::Push(vec![r"comment"])),
        Rule::bygroups(r"(?m)(\s*)(Note.*)", vec![Some(WHITESPACE), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlequote"])),
        Rule::token_to(r"(?m)0\s+:\s*0", NAME_ENTITY, NewState::Push(vec![r"nounDefinition"])),
        Rule::bygroups_to(r"(?m)(noun)(\s+)(define)(\s*)$", vec![Some(NAME_ENTITY), Some(WHITESPACE), Some(NAME_ENTITY), Some(WHITESPACE)], NewState::Push(vec![r"nounDefinition"])),
        Rule::token_to(r"(?m)([1-4]|13)\s+:\s*0\b", NAME_FUNCTION, NewState::Push(vec![r"explicitDefinition"])),
        Rule::bygroups_to(r"(?m)(adverb|conjunction|dyad|monad|verb)(\s+)(define)\b", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_FUNCTION)], NewState::Push(vec![r"explicitDefinition"])),
        Rule::token(r"(?m)((?:for|goto|label)_)\b[a-zA-Z]\w*\.", NAME_LABEL),
        Rule::token(r"(?m)(assert|break|c(?:a(?:se|tch(?:(?:[dt])?))|ontinue)|do|e(?:lse(?:(?:if)?)|nd)|f(?:case|or)|if|return|select|t(?:hrow|ry)|whil(?:e|st))\.", NAME_LABEL),
        Rule::token(r"(?m)\b[a-zA-Z]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)(ARGV|CR(?:(?:LF)?)|D(?:EL|ebug)|E(?:AV|MPTY)|FF|JVERSION|LF(?:(?:2)?)|Note|TAB|a(?:lpha(?:(?:[12])7)|pply)|b(?:ind|ox(?:(?:(?:x)?)open)|x)|c(?:lear|ut(?:LF|open))|d(?:atatype|ef|fh|rop)|e(?:ach|cho|mpty|rase|v(?:ery|tloop)|x(?:it|pand))|f(?:etch|i(?:le2url|xdotdot)|liprgb)|get(?:args|env)|hfd|i(?:nv(?:(?:erse)?)|ospath|s(?:atty|utf8)|tems)|l(?:eaf|ist)|n(?:ame(?:class|list|s)|[cl])|on|pick|rows|s(?:cript(?:(?:d)?)|ign|m(?:info|output)|ort|plit|td(?:err|in|out))|t(?:a(?:(?:bl|k)e)|ime(?:(?:(?:space)?)x)|moutput|o(?:CRLF|HOST|J|(?:low|upp)er)|ype)|u(?:cp(?:(?:count)?)|sleep|tf8|ucp))", NAME_FUNCTION),
        Rule::token(r"(?m)=[.:]", OPERATOR),
        Rule::token(r#"(?m)[-=+*#$%@!~`^&";:.,<>{}\[\]\\|/?]"#, OPERATOR),
        Rule::token(r"(?m)[abCdDeEfHiIjLMoprtT]\.", KEYWORD_RESERVED),
        Rule::token(r"(?m)[aDiLpqsStux]\:", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_[0-9])\:", KEYWORD_CONSTANT),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"parentheses"])),
        Rule::token(r"(?m)\b_{1,2}\b", NUMBER),
        Rule::token(r"(?m)_?\d+(\.\d+)?(\s*[ejr]\s*)_?\d+(\.?=\d+)?", NUMBER),
        Rule::token(r"(?m)_?\d+\.(?=\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)_?\d+x", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)_?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[^)]", NAME),
        Rule::token_to(r"(?m)^\)", NAME_LABEL, NewState::Pop(1)),
        Rule::token(r"(?m)[)]", NAME),
    ]);
    m.insert(
        r"nounDefinition",
        vec![
            Rule::token(r"(?m)[^)]+", STRING),
            Rule::token_to(r"(?m)^\)", NAME_LABEL, NewState::Pop(1)),
            Rule::token(r"(?m)[)]", STRING),
        ],
    );
    m.insert(r"parentheses", vec![
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)\b[nmuvxy]\b", NAME_DECORATOR),
        Rule::token(r"(?m)#!.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)NB\..*", COMMENT_SINGLE),
        Rule::bygroups_to(r"(?m)(\n+\s*)(Note)", vec![Some(WHITESPACE), Some(COMMENT_MULTILINE)], NewState::Push(vec![r"comment"])),
        Rule::bygroups(r"(?m)(\s*)(Note.*)", vec![Some(WHITESPACE), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlequote"])),
        Rule::token_to(r"(?m)0\s+:\s*0", NAME_ENTITY, NewState::Push(vec![r"nounDefinition"])),
        Rule::bygroups_to(r"(?m)(noun)(\s+)(define)(\s*)$", vec![Some(NAME_ENTITY), Some(WHITESPACE), Some(NAME_ENTITY), Some(WHITESPACE)], NewState::Push(vec![r"nounDefinition"])),
        Rule::token_to(r"(?m)([1-4]|13)\s+:\s*0\b", NAME_FUNCTION, NewState::Push(vec![r"explicitDefinition"])),
        Rule::bygroups_to(r"(?m)(adverb|conjunction|dyad|monad|verb)(\s+)(define)\b", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_FUNCTION)], NewState::Push(vec![r"explicitDefinition"])),
        Rule::token(r"(?m)((?:for|goto|label)_)\b[a-zA-Z]\w*\.", NAME_LABEL),
        Rule::token(r"(?m)(assert|break|c(?:a(?:se|tch(?:(?:[dt])?))|ontinue)|do|e(?:lse(?:(?:if)?)|nd)|f(?:case|or)|if|return|select|t(?:hrow|ry)|whil(?:e|st))\.", NAME_LABEL),
        Rule::token(r"(?m)\b[a-zA-Z]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)(ARGV|CR(?:(?:LF)?)|D(?:EL|ebug)|E(?:AV|MPTY)|FF|JVERSION|LF(?:(?:2)?)|Note|TAB|a(?:lpha(?:(?:[12])7)|pply)|b(?:ind|ox(?:(?:(?:x)?)open)|x)|c(?:lear|ut(?:LF|open))|d(?:atatype|ef|fh|rop)|e(?:ach|cho|mpty|rase|v(?:ery|tloop)|x(?:it|pand))|f(?:etch|i(?:le2url|xdotdot)|liprgb)|get(?:args|env)|hfd|i(?:nv(?:(?:erse)?)|ospath|s(?:atty|utf8)|tems)|l(?:eaf|ist)|n(?:ame(?:class|list|s)|[cl])|on|pick|rows|s(?:cript(?:(?:d)?)|ign|m(?:info|output)|ort|plit|td(?:err|in|out))|t(?:a(?:(?:bl|k)e)|ime(?:(?:(?:space)?)x)|moutput|o(?:CRLF|HOST|J|(?:low|upp)er)|ype)|u(?:cp(?:(?:count)?)|sleep|tf8|ucp))", NAME_FUNCTION),
        Rule::token(r"(?m)=[.:]", OPERATOR),
        Rule::token(r#"(?m)[-=+*#$%@!~`^&";:.,<>{}\[\]\\|/?]"#, OPERATOR),
        Rule::token(r"(?m)[abCdDeEfHiIjLMoprtT]\.", KEYWORD_RESERVED),
        Rule::token(r"(?m)[aDiLpqsStux]\:", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_[0-9])\:", KEYWORD_CONSTANT),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"parentheses"])),
        Rule::token(r"(?m)\b_{1,2}\b", NUMBER),
        Rule::token(r"(?m)_?\d+(\.\d+)?(\s*[ejr]\s*)_?\d+(\.?=\d+)?", NUMBER),
        Rule::token(r"(?m)_?\d+\.(?=\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)_?\d+x", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)_?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[^)]", NAME),
        Rule::token_to(r"(?m)^\)", NAME_LABEL, NewState::Pop(1)),
        Rule::token(r"(?m)[)]", NAME),
        Rule::token(r"(?m)#!.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)NB\..*", COMMENT_SINGLE),
        Rule::bygroups_to(r"(?m)(\n+\s*)(Note)", vec![Some(WHITESPACE), Some(COMMENT_MULTILINE)], NewState::Push(vec![r"comment"])),
        Rule::bygroups(r"(?m)(\s*)(Note.*)", vec![Some(WHITESPACE), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlequote"])),
        Rule::token_to(r"(?m)0\s+:\s*0", NAME_ENTITY, NewState::Push(vec![r"nounDefinition"])),
        Rule::bygroups_to(r"(?m)(noun)(\s+)(define)(\s*)$", vec![Some(NAME_ENTITY), Some(WHITESPACE), Some(NAME_ENTITY), Some(WHITESPACE)], NewState::Push(vec![r"nounDefinition"])),
        Rule::token_to(r"(?m)([1-4]|13)\s+:\s*0\b", NAME_FUNCTION, NewState::Push(vec![r"explicitDefinition"])),
        Rule::bygroups_to(r"(?m)(adverb|conjunction|dyad|monad|verb)(\s+)(define)\b", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_FUNCTION)], NewState::Push(vec![r"explicitDefinition"])),
        Rule::token(r"(?m)((?:for|goto|label)_)\b[a-zA-Z]\w*\.", NAME_LABEL),
        Rule::token(r"(?m)(assert|break|c(?:a(?:se|tch(?:(?:[dt])?))|ontinue)|do|e(?:lse(?:(?:if)?)|nd)|f(?:case|or)|if|return|select|t(?:hrow|ry)|whil(?:e|st))\.", NAME_LABEL),
        Rule::token(r"(?m)\b[a-zA-Z]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)(ARGV|CR(?:(?:LF)?)|D(?:EL|ebug)|E(?:AV|MPTY)|FF|JVERSION|LF(?:(?:2)?)|Note|TAB|a(?:lpha(?:(?:[12])7)|pply)|b(?:ind|ox(?:(?:(?:x)?)open)|x)|c(?:lear|ut(?:LF|open))|d(?:atatype|ef|fh|rop)|e(?:ach|cho|mpty|rase|v(?:ery|tloop)|x(?:it|pand))|f(?:etch|i(?:le2url|xdotdot)|liprgb)|get(?:args|env)|hfd|i(?:nv(?:(?:erse)?)|ospath|s(?:atty|utf8)|tems)|l(?:eaf|ist)|n(?:ame(?:class|list|s)|[cl])|on|pick|rows|s(?:cript(?:(?:d)?)|ign|m(?:info|output)|ort|plit|td(?:err|in|out))|t(?:a(?:(?:bl|k)e)|ime(?:(?:(?:space)?)x)|moutput|o(?:CRLF|HOST|J|(?:low|upp)er)|ype)|u(?:cp(?:(?:count)?)|sleep|tf8|ucp))", NAME_FUNCTION),
        Rule::token(r"(?m)=[.:]", OPERATOR),
        Rule::token(r#"(?m)[-=+*#$%@!~`^&";:.,<>{}\[\]\\|/?]"#, OPERATOR),
        Rule::token(r"(?m)[abCdDeEfHiIjLMoprtT]\.", KEYWORD_RESERVED),
        Rule::token(r"(?m)[aDiLpqsStux]\:", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_[0-9])\:", KEYWORD_CONSTANT),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"parentheses"])),
        Rule::token(r"(?m)\b_{1,2}\b", NUMBER),
        Rule::token(r"(?m)_?\d+(\.\d+)?(\s*[ejr]\s*)_?\d+(\.?=\d+)?", NUMBER),
        Rule::token(r"(?m)_?\d+\.(?=\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)_?\d+x", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)_?\d+", NUMBER_INTEGER),
    ]);
    m.insert(
        r"singlequote",
        vec![
            Rule::token(r"(?m)[^']+", STRING),
            Rule::token(r"(?m)''", STRING),
            Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for JLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
