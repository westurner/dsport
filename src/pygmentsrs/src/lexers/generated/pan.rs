//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:PanLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:PanLexer:pan

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pan
pub struct PanLexer;

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
        Rule::token(r"(?m)\b(bind|declaration|e(?:(?:ls|xtensibl)e)|f(?:inal|or(?:(?:each)?)|unction)|i(?:f|nclude)|object|prefix|structure|t(?:(?:emplat|yp)e)|unique|va(?:lid|riable)|w(?:hile|ith))\b", KEYWORD),
        Rule::token(r"(?m)\b(append|base64_(?:(?:de|en)code)|c(?:(?:lon|reat)e)|d(?:e(?:bug|lete|precated)|igest)|e(?:rror|scape|xists)|f(?:i(?:le_contents|rst)|ormat)|i(?:f_exists|ndex|s_(?:boolean|d(?:efined|ouble)|l(?:ist|ong)|n(?:list|u(?:ll|mber))|property|resource|string))|key|l(?:ength|ist)|m(?:atch(?:(?:es)?)|erge)|n(?:(?:ex|lis)t)|p(?:ath_exists|repend)|re(?:place|turn)|s(?:pli(?:ce|t)|ubstr)|t(?:o_(?:boolean|double|lo(?:ng|wercase)|string|uppercase)|raceback)|(?:unescap|valu)e)\b", NAME_BUILTIN),
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]+", OPERATOR),
        Rule::token(r"(?m)<<\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"curly"])),
        Rule::token(r#"(?m)(?s)"(\\\\|\\[0-7]+|\\.|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)(?s)'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\;#]+"#, TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
    ]);
    m.insert(r"basic", vec![
        Rule::token(r"(?m)\b(bind|declaration|e(?:(?:ls|xtensibl)e)|f(?:inal|or(?:(?:each)?)|unction)|i(?:f|nclude)|object|prefix|structure|t(?:(?:emplat|yp)e)|unique|va(?:lid|riable)|w(?:hile|ith))\b", KEYWORD),
        Rule::token(r"(?m)\b(append|base64_(?:(?:de|en)code)|c(?:(?:lon|reat)e)|d(?:e(?:bug|lete|precated)|igest)|e(?:rror|scape|xists)|f(?:i(?:le_contents|rst)|ormat)|i(?:f_exists|ndex|s_(?:boolean|d(?:efined|ouble)|l(?:ist|ong)|n(?:list|u(?:ll|mber))|property|resource|string))|key|l(?:ength|ist)|m(?:atch(?:(?:es)?)|erge)|n(?:(?:ex|lis)t)|p(?:ath_exists|repend)|re(?:place|turn)|s(?:pli(?:ce|t)|ubstr)|t(?:o_(?:boolean|double|lo(?:ng|wercase)|string|uppercase)|raceback)|(?:unescap|valu)e)\b", NAME_BUILTIN),
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]+", OPERATOR),
        Rule::token(r"(?m)<<\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m);", PUNCTUATION),
    ]);
    m.insert(r"data", vec![
        Rule::token(r#"(?m)(?s)"(\\\\|\\[0-7]+|\\.|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)(?s)'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\;#]+"#, TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
    ]);
    m.insert(r"curly", vec![
        Rule::token_to(r"(?m)\}", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m):-", KEYWORD),
        Rule::token(r"(?m)\w+", NAME_VARIABLE),
        Rule::token(r#"(?m)[^}:"\'`$]+"#, PUNCTUATION),
        Rule::token(r"(?m):", PUNCTUATION),
        Rule::token(r"(?m)\b(bind|declaration|e(?:(?:ls|xtensibl)e)|f(?:inal|or(?:(?:each)?)|unction)|i(?:f|nclude)|object|prefix|structure|t(?:(?:emplat|yp)e)|unique|va(?:lid|riable)|w(?:hile|ith))\b", KEYWORD),
        Rule::token(r"(?m)\b(append|base64_(?:(?:de|en)code)|c(?:(?:lon|reat)e)|d(?:e(?:bug|lete|precated)|igest)|e(?:rror|scape|xists)|f(?:i(?:le_contents|rst)|ormat)|i(?:f_exists|ndex|s_(?:boolean|d(?:efined|ouble)|l(?:ist|ong)|n(?:list|u(?:ll|mber))|property|resource|string))|key|l(?:ength|ist)|m(?:atch(?:(?:es)?)|erge)|n(?:(?:ex|lis)t)|p(?:ath_exists|repend)|re(?:place|turn)|s(?:pli(?:ce|t)|ubstr)|t(?:o_(?:boolean|double|lo(?:ng|wercase)|string|uppercase)|raceback)|(?:unescap|valu)e)\b", NAME_BUILTIN),
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]+", OPERATOR),
        Rule::token(r"(?m)<<\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"curly"])),
        Rule::token(r#"(?m)(?s)"(\\\\|\\[0-7]+|\\.|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)(?s)'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\;#]+"#, TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
    ]);
    m.insert(r"paren", vec![
        Rule::token_to(r"(?m)\)", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)\b(bind|declaration|e(?:(?:ls|xtensibl)e)|f(?:inal|or(?:(?:each)?)|unction)|i(?:f|nclude)|object|prefix|structure|t(?:(?:emplat|yp)e)|unique|va(?:lid|riable)|w(?:hile|ith))\b", KEYWORD),
        Rule::token(r"(?m)\b(append|base64_(?:(?:de|en)code)|c(?:(?:lon|reat)e)|d(?:e(?:bug|lete|precated)|igest)|e(?:rror|scape|xists)|f(?:i(?:le_contents|rst)|ormat)|i(?:f_exists|ndex|s_(?:boolean|d(?:efined|ouble)|l(?:ist|ong)|n(?:list|u(?:ll|mber))|property|resource|string))|key|l(?:ength|ist)|m(?:atch(?:(?:es)?)|erge)|n(?:(?:ex|lis)t)|p(?:ath_exists|repend)|re(?:place|turn)|s(?:pli(?:ce|t)|ubstr)|t(?:o_(?:boolean|double|lo(?:ng|wercase)|string|uppercase)|raceback)|(?:unescap|valu)e)\b", NAME_BUILTIN),
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]+", OPERATOR),
        Rule::token(r"(?m)<<\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\{", KEYWORD, NewState::Push(vec![r"curly"])),
        Rule::token(r#"(?m)(?s)"(\\\\|\\[0-7]+|\\.|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)(?s)'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\;#]+"#, TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
    ]);
    Table(m)
}

impl Lexer for PanLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
