#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.boa:BoaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.boa:BoaLexer:boa

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: boa
pub struct BoaLexer;

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
        Rule::token(r"(?m)#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)\b(after|b(?:efore|reak)|case|default|e(?:lse|xists)|f(?:oreach|unction)|i(?:f(?:(?:all)?)|nput)|o(?:f|utput)|return|s(?:top|witch)|visit(?:(?:or)?)|w(?:eight|hile))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)\b(a(?:bs|cos(?:(?:h)?)|dd(?:(?:day|month|week|year)?)|sin(?:(?:h)?)|tan(?:(?:[2h])?))|bool|c(?:eil|lear|o(?:ntains|s(?:(?:h)?)))|d(?:ayof(?:month|week|year(?:(?:)?))|ef)|exp|f(?:lo(?:at|or)|ormat(?:(?:time)?))|get(?:(?:as|snapsho)t)|h(?:as(?:filetype|h|key)|ighbit|ourof)|i(?:nt|s(?:fi(?:nite|xingrevision)|inf|kind|literal|n(?:an|ormal)))|keys|l(?:en|o(?:g(?:(?:10)?)|okup|wercase))|m(?:a(?:tch(?:(?:(?:posn|str)s)?)|x)|in(?:(?:uteof)?))|n(?:ew|ow|rand)|pow|r(?:and|e(?:gex|move(?:(?:)?))|ound)|s(?:econdof|in(?:(?:h)?)|ort|plit(?:(?:all|n)?)|qrt|tr(?:find|ing|r(?:eplace|find))|ubstring)|t(?:an(?:(?:h)?)|ime|r(?:im|unc(?:(?:to(?:day|hour|m(?:inute|onth)|second|year))?)))|uppercase|values|yearof)\(", NAME_FUNCTION),
        Rule::token(r"(?m)\b(array|bo(?:ol|ttom)|collection|enum|f(?:alse|loat)|int|m(?:a(?:p|ximum)|ean|inimum)|s(?:et|t(?:ack|ring)|um)|t(?:ime|op|(?:ru|yp)e))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(ASTRoot|C(?:hange(?:Kind|dFile)|odeRepository)|Declaration|Expression(?:(?:Kind)?)|F(?:(?:il|org)eKind)|M(?:ethod|odifier(?:(?:Kind)?))|Namespace|P(?:erson|roject)|Re(?:positoryKind|vision)|Statement(?:(?:Kind)?)|Type(?:(?:Kind)?)|V(?:ariable|isibility))\b", TokenType::new(&["Name", "Classes"])),
        Rule::token(r"(?m)(\&\&|\+\+|\->|:=|<<|\|\||[!*+\-:<=>])", OPERATOR),
        Rule::token(r"(?m)[\]\[(),;{}\\.]", PUNCTUATION),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)`(\\\\|\\[^\\]|[^`\\])*`", STRING_BACKTICK),
        Rule::token(r#"(?m)(["`])"#, STRING_DELIMITER),
        Rule::token(r"(?m)[a-zA-Z_]+", NAME_VARIABLE),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for BoaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
