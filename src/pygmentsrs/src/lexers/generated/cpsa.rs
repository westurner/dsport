//! AUTO-GENERATED from `pygments.pygments.lexers.lisp:CPSALexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lisp:CPSALexer:cpsa

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cpsa
pub struct CpsaLexer;

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
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)'[\w!$%&*+,/:<=>?@^~|-]+", STRING_SYMBOL),
        Rule::token(r#"(?m)#\\([()/'\"._!§$%& ?=+-]|[a-zA-Z0-9]+)"#, STRING_CHAR),
        Rule::token(r"(?m)(#t|#f)", NAME_CONSTANT),
        Rule::token(r"(?m)('|#|`|,@|,|\.)", OPERATOR),
        Rule::token(r"(?m)(akey|d(?:ata|ef(?:listener|macro|protocol|role|s(?:keleton|trand)))|herald|include|mesg|n(?:ame|on\-orig)|p(?:en\-non\-orig|recedes)|recv|s(?:end|key)|t(?:ext|race)|uniq\-orig|vars)\b", KEYWORD),
        Rule::token(r"(?m)(?<='\()[\w!$%&*+,/:<=>?@^~|-]+", NAME_VARIABLE),
        Rule::token(r"(?m)(?<=#\()[\w!$%&*+,/:<=>?@^~|-]+", NAME_VARIABLE),
        Rule::token(r"(?m)(?<=\()(cat|e(?:nc|xp)|gen|hash|(?:inv|lt|p(?:riv|ub))k)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<=\()[\w!$%&*+,/:<=>?@^~|-]+", NAME_FUNCTION),
        Rule::token(r"(?m)[\w!$%&*+,/:<=>?@^~|-]+", NAME_VARIABLE),
        Rule::token(r"(?m)(\(|\))", PUNCTUATION),
        Rule::token(r"(?m)(\[|\])", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for CpsaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
