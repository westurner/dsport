//! AUTO-GENERATED from `pygments.pygments.lexers.ooc:OocLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ooc:OocLexer:ooc

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ooc
pub struct OocLexer;

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
        Rule::token(r"(?m)\b(a(?:bstract|s)|break|c(?:ase|lass|on(?:st|tinue))|do|e(?:lse|xte(?:nds|rn))|f(?:al(?:lthrough|se)|inal|or|rom)|i(?:mp(?:(?:lemen|or)t)|n(?:(?:lin|terfac)e)|[fn])|n(?:ew|ull)|operator|proto|return|s(?:tatic|uper|witch)|t(?:his|rue)|use|version|while)\b", KEYWORD),
        Rule::token_to(r"(?m)include\b", KEYWORD, NewState::Push(vec![r"include"])),
        Rule::bygroups(r"(?m)(cover)([ \t]+)(from)([ \t]+)(\w+[*@]?)", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD), Some(TEXT), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(func)((?:[ \t]|\\\n)+)(~[a-z_]\w*)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)\bfunc\b", KEYWORD),
        Rule::token(r"(?m)//.*", COMMENT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(==?|\+=?|-[=>]?|\*=?|/=?|:=|!=?|%=?|\?|>{1,3}=?|<{1,3}=?|\.\.|&&?|\|\|?|\^=?)", OPERATOR),
        Rule::bygroups(r"(?m)(\.)([ \t]*)([a-z]\w*)", vec![Some(OPERATOR), Some(TEXT), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)[A-Z][A-Z0-9_]+", NAME_CONSTANT),
        Rule::token(r"(?m)[A-Z]\w*([@*]|\[[ \t]*\])?", NAME_CLASS),
        Rule::bygroups(r"(?m)([a-z]\w*(?:~[a-z]\w*)?)((?:[ \t]|\\\n)*)(?=\()", vec![Some(NAME_FUNCTION), Some(TEXT)]),
        Rule::token(r"(?m)[a-z]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[:(){}\[\];,]", PUNCTUATION),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)0c[0-9]+", NUMBER_OCT),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)[0-9_]\.[0-9_]*(?!\.)", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9_]+", TokenType::new(&["Literal", "Number", "Decimal"])),
        Rule::token(r#"(?m)"(?:\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\"])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)'(?:\\.|\\[0-9]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)@", PUNCTUATION),
        Rule::token(r"(?m)\.", PUNCTUATION),
        Rule::token(r"(?m)\\[ \t\n]", TEXT),
        Rule::token(r"(?m)[ \t]+", TEXT),
    ]);
    m.insert(
        r"include",
        vec![
            Rule::token(r"(?m)[\w/]+", NAME),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)[ \t]", TEXT),
            Rule::token_to(r"(?m)[;\n]", TEXT, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for OocLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
