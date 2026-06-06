//! AUTO-GENERATED from `pygments.pygments.lexers.maxima:MaximaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.maxima:MaximaLexer:maxima

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: maxima, macsyma
pub struct MaximaLexer;

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
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r#"(?m)"(?:[^"\\]|\\.)*""#, STRING),
        Rule::token(r"(?m)\(|\)|\[|\]|\{|\}", PUNCTUATION),
        Rule::token(r"(?m)[,;$]", PUNCTUATION),
        Rule::token(r"(?m)(%(?:gamma|p(?:(?:(?:h)?)i)|[ei])|done|false|in(?:finity|[df])|minf|true|un(?:d|known))", NAME_CONSTANT),
        Rule::token(r"(?m)(do(?:(?:wnto)?)|else(?:(?:if)?)|f(?:or|rom)|if|repeat|step|t(?:h(?:en|ru)|o)|until|while)", KEYWORD),
        Rule::token(r"(?m)([!#'*+\-./:<=>@\^|])", OPERATOR),
        Rule::token(r"(?m)(and|not|or)", OPERATOR_WORD),
        Rule::bygroups(r"(?m)(?x)
              ((?:[a-zA-Z_#][\w#]*|`[^`]*`)
              (?:::[a-zA-Z_#][\w#]*|`[^`]*`)*)(\s*)([(])", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)(?x)
              (?:[a-zA-Z_#%][\w#%]*|`[^`]*`)
              (?:::[a-zA-Z_#%][\w#%]*|`[^`]*`)*", NAME_VARIABLE),
        Rule::token(r"(?m)[-+]?(\d*\.\d+([bdefls][-+]?\d+)?|\d+(\.\d*)?[bdefls][-+]?\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)[-+]?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
        Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
    ]);
    Table(m)
}

impl Lexer for MaximaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
