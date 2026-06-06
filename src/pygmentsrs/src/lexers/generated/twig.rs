//! AUTO-GENERATED from `pygments.pygments.lexers.templates:TwigLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:TwigLexer:twig

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: twig
pub struct TwigLexer;

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
        Rule::token(r"(?ms)[^{]+", OTHER),
        Rule::token_to(r"(?ms)\{\{", COMMENT_PREPROC, NewState::Push(vec![r"var"])),
        Rule::token(r"(?ms)\{\#.*?\#\}", COMMENT),
        Rule::bygroups(r"(?ms)(\{%)(-?\s*)(raw)(\s*-?)(%\})(.*?)(\{%)(-?\s*)(endraw)(\s*-?)(%\})", vec![Some(COMMENT_PREPROC), Some(TEXT), Some(KEYWORD), Some(TEXT), Some(COMMENT_PREPROC), Some(OTHER), Some(COMMENT_PREPROC), Some(TEXT), Some(KEYWORD), Some(TEXT), Some(COMMENT_PREPROC)]),
        Rule::bygroups(r"(?ms)(\{%)(-?\s*)(verbatim)(\s*-?)(%\})(.*?)(\{%)(-?\s*)(endverbatim)(\s*-?)(%\})", vec![Some(COMMENT_PREPROC), Some(TEXT), Some(KEYWORD), Some(TEXT), Some(COMMENT_PREPROC), Some(OTHER), Some(COMMENT_PREPROC), Some(TEXT), Some(KEYWORD), Some(TEXT), Some(COMMENT_PREPROC)]),
        Rule::bygroups_to(r"(?ms)(\{%)(-?\s*)(filter)(\s+)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*)", vec![Some(COMMENT_PREPROC), Some(TEXT), Some(KEYWORD), Some(TEXT), Some(NAME_FUNCTION)], NewState::Push(vec![r"tag"])),
        Rule::bygroups_to(r"(?ms)(\{%)(-?\s*)([a-zA-Z_]\w*)", vec![Some(COMMENT_PREPROC), Some(TEXT), Some(KEYWORD)], NewState::Push(vec![r"tag"])),
        Rule::token(r"(?ms)\{", OTHER),
    ]);
    m.insert(r"varnames", vec![
        Rule::bygroups(r"(?ms)(\|)(\s*)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*)", vec![Some(OPERATOR), Some(TEXT), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?ms)(is)(\s+)(not)?(\s*)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*)", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD), Some(TEXT), Some(NAME_FUNCTION)]),
        Rule::token(r"(?ms)(?i)(true|false|none|null)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?ms)(in|not|and|b-and|or|b-or|b-xor|isif|elseif|else|importconstant|defined|divisibleby|empty|even|iterable|odd|sameasmatches|starts\s+with|ends\s+with)\b", KEYWORD),
        Rule::token(r"(?ms)(loop|block|parent)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ms)\.(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ms)\.[0-9]+", NUMBER),
        Rule::token(r#"(?ms):?"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms):?'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?ms)([{}()\[\]+\-*/,:~%]|\.\.|\?|:|\*\*|\/\/|!=|[><=]=?)", OPERATOR),
        Rule::token(r"(?ms)[0-9](\.[0-9]*)?(eE[+-][0-9])?[flFLdD]?|0[xX][0-9a-fA-F]+[Ll]?", NUMBER),
    ]);
    m.insert(r"var", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::bygroups_to(r"(?ms)(-?)(\}\})", vec![Some(TEXT), Some(COMMENT_PREPROC)], NewState::Pop(1)),
        Rule::bygroups(r"(?ms)(\|)(\s*)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*)", vec![Some(OPERATOR), Some(TEXT), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?ms)(is)(\s+)(not)?(\s*)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*)", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD), Some(TEXT), Some(NAME_FUNCTION)]),
        Rule::token(r"(?ms)(?i)(true|false|none|null)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?ms)(in|not|and|b-and|or|b-or|b-xor|isif|elseif|else|importconstant|defined|divisibleby|empty|even|iterable|odd|sameasmatches|starts\s+with|ends\s+with)\b", KEYWORD),
        Rule::token(r"(?ms)(loop|block|parent)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ms)\.(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ms)\.[0-9]+", NUMBER),
        Rule::token(r#"(?ms):?"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms):?'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?ms)([{}()\[\]+\-*/,:~%]|\.\.|\?|:|\*\*|\/\/|!=|[><=]=?)", OPERATOR),
        Rule::token(r"(?ms)[0-9](\.[0-9]*)?(eE[+-][0-9])?[flFLdD]?|0[xX][0-9a-fA-F]+[Ll]?", NUMBER),
    ]);
    m.insert(r"tag", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::bygroups_to(r"(?ms)(-?)(%\})", vec![Some(TEXT), Some(COMMENT_PREPROC)], NewState::Pop(1)),
        Rule::bygroups(r"(?ms)(\|)(\s*)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*)", vec![Some(OPERATOR), Some(TEXT), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?ms)(is)(\s+)(not)?(\s*)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*)", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD), Some(TEXT), Some(NAME_FUNCTION)]),
        Rule::token(r"(?ms)(?i)(true|false|none|null)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?ms)(in|not|and|b-and|or|b-or|b-xor|isif|elseif|else|importconstant|defined|divisibleby|empty|even|iterable|odd|sameasmatches|starts\s+with|ends\s+with)\b", KEYWORD),
        Rule::token(r"(?ms)(loop|block|parent)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ms)\.(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w-]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ms)\.[0-9]+", NUMBER),
        Rule::token(r#"(?ms):?"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms):?'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?ms)([{}()\[\]+\-*/,:~%]|\.\.|\?|:|\*\*|\/\/|!=|[><=]=?)", OPERATOR),
        Rule::token(r"(?ms)[0-9](\.[0-9]*)?(eE[+-][0-9])?[flFLdD]?|0[xX][0-9a-fA-F]+[Ll]?", NUMBER),
        Rule::token(r"(?ms).", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for TwigLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
