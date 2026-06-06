//! AUTO-GENERATED from `pygments.pygments.lexers.scripting:RexxLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scripting:RexxLexer:rexx

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: rexx, arexx
pub struct RexxLexer;

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
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?im)'", STRING, NewState::Push(vec![r"string_single"])),
        Rule::token(r"(?im)[0-9]+(\.[0-9]+)?(e[+-]?[0-9])?", NUMBER),
        Rule::bygroups(r"(?im)([a-z_]\w*)(\s*)(:)(\s*)(procedure)\b", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_DECLARATION)]),
        Rule::bygroups(r"(?im)([a-z_]\w*)(\s*)(:)", vec![Some(NAME_LABEL), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?im)(a(?:b(?:brev|s)|ddress|rg)|b(?:2x|it(?:and|(?:(?:x)?)or))|c(?:2(?:[dx])|enter|har(?:in|out|s)|o(?:mpare|ndition|pies))|d(?:2(?:[cx])|at(?:(?:(?:atyp)?)e)|el(?:str|word)|igits)|errortext|f(?:orm(?:(?:at)?)|uzz)|insert|l(?:astpos|e(?:ft|ngth)|ine(?:in|out|s))|m(?:ax|in)|overlay|pos|queued|r(?:andom|everse|ight)|s(?:ign|ourceline|pace|tr(?:eam|ip)|ub(?:str|word)|ymbol)|t(?:ime|r(?:a(?:(?:c|nslat)e)|unc))|v(?:alue|erify)|word(?:(?:index|length|(?:(?:po)?)s)?)|x(?:2(?:[bcd])|range))(\s*)(\()", vec![Some(NAME_BUILTIN), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?im)(address|arg|by|call|do|drop|else|end|exit|for|forever|if|interpret|iterate|leave|nop|numeric|off|on|options|parse|pull|push|queue|return|say|select|signal|to|then|trace|until|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?im)(-|//|/|\(|\)|\*\*|\*|\\<<|\\<|\\==|\\=|\\>>|\\>|\\|\|\||\||&&|&|%|\+|<<=|<<|<=|<>|<|==|=|><|>=|>>=|>>|>|¬<<|¬<|¬==|¬=|¬>>|¬>|¬|\.|,)", OPERATOR),
        Rule::token(r"(?im)[a-z_]\w*", TEXT),
    ]);
    m.insert(r"function", vec![
        Rule::bygroups(r"(?im)(a(?:b(?:brev|s)|ddress|rg)|b(?:2x|it(?:and|(?:(?:x)?)or))|c(?:2(?:[dx])|enter|har(?:in|out|s)|o(?:mpare|ndition|pies))|d(?:2(?:[cx])|at(?:(?:(?:atyp)?)e)|el(?:str|word)|igits)|errortext|f(?:orm(?:(?:at)?)|uzz)|insert|l(?:astpos|e(?:ft|ngth)|ine(?:in|out|s))|m(?:ax|in)|overlay|pos|queued|r(?:andom|everse|ight)|s(?:ign|ourceline|pace|tr(?:eam|ip)|ub(?:str|word)|ymbol)|t(?:ime|r(?:a(?:(?:c|nslat)e)|unc))|v(?:alue|erify)|word(?:(?:index|length|(?:(?:po)?)s)?)|x(?:2(?:[bcd])|range))(\s*)(\()", vec![Some(NAME_BUILTIN), Some(WHITESPACE), Some(OPERATOR)]),
    ]);
    m.insert(r"keyword", vec![
        Rule::token(r"(?im)(address|arg|by|call|do|drop|else|end|exit|for|forever|if|interpret|iterate|leave|nop|numeric|off|on|options|parse|pull|push|queue|return|say|select|signal|to|then|trace|until|while)\b", KEYWORD_RESERVED),
    ]);
    m.insert(r"operator", vec![
        Rule::token(r"(?im)(-|//|/|\(|\)|\*\*|\*|\\<<|\\<|\\==|\\=|\\>>|\\>|\\|\|\||\||&&|&|%|\+|<<=|<<|<=|<>|<|==|=|><|>=|>>=|>>|>|¬<<|¬<|¬==|¬=|¬>>|¬>|¬|\.|,)", OPERATOR),
    ]);
    m.insert(r"string_double", vec![
        Rule::token(r#"(?im)[^"\n]+"#, STRING),
        Rule::token(r#"(?im)"""#, STRING),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Pop(1)),
        Rule::token_to(r"(?im)\n", TEXT, NewState::Pop(1)),
    ]);
    m.insert(r"string_single", vec![
        Rule::token(r"(?im)[^\'\n]+", STRING),
        Rule::token(r"(?im)\'\'", STRING),
        Rule::token_to(r"(?im)\'", STRING, NewState::Pop(1)),
        Rule::token_to(r"(?im)\n", TEXT, NewState::Pop(1)),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?im)[^*]+", COMMENT_MULTILINE),
        Rule::token_to(r"(?im)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?im)\*", COMMENT_MULTILINE),
    ]);
    Table(m)
}

impl Lexer for RexxLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
