//! AUTO-GENERATED from `pygments.pygments.lexers.unicon:UcodeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.unicon:UcodeLexer:ucode

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ucode
pub struct UcodeLexer;

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
        Rule::token(r"(?m)(#.*\n)", COMMENT),
        Rule::token(r"(?m)\b(con|declend|end|global|i(?:mpl|nvocable)|l(?:ab|ink|ocal)|record|u(?:id|nions)|version)\b", NAME_FUNCTION),
        Rule::token(r"(?m)\b(colm|filen|line|synt)\b", COMMENT),
        Rule::token(r"(?m)\b(asgn|b(?:ang|scan)|c(?:at|case|hfail|o(?:act|fail|mpl|ret)|reate|set)|d(?:i(?:ff|v)|up)|e(?:fail|init|nd|qv|r(?:et|ror)|s(?:can|usp))|field|goto|in(?:it|t(?:(?:er)?)|voke)|keywd|l(?:concat|ex(?:eq|g(?:[et])|l(?:[et])|ne)|imit|list|susp)|m(?:ark(?:(?:0)?)|inus|od|ult)|n(?:e(?:g|qv)|o(?:nnull|op)|u(?:ll|m(?:ber|eq|g(?:[et])|l(?:[et])|ne)))|p(?:fail|lus|null|o(?:p|wer)|r(?:et|oc)|susp|ush(?:(?:(?:n)?)1))|r(?:a(?:ndom|sgn)|cv(?:(?:bk)?)|e(?:al|fresh)|swap)|s(?:dup|ect|ize|nd(?:(?:bk)?)|tr|ubsc|wap)|t(?:a(?:bmat|lly)|oby|race)|unmark|va(?:lue|r))\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(any|case|e(?:nd(?:case|every|if(?:(?:else)?)|repeat|suspend|until|while)|very)|if(?:(?:else)?)|repeat|suspend|until|while)\b", NAME_CONSTANT),
        Rule::token(r"(?m)\d+(\s*|\.$|$)", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-]?\d*\.\d+(E[-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d+\.\d*(E[-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)(<>|=>|[()|:;,.'`]|[{}]|[%^]|[&?])", PUNCTUATION),
        Rule::token(r"(?m)\s+\b", TEXT),
        Rule::token(r"(?m)[\w-]+", TEXT),
    ]);
    Table(m)
}

impl Lexer for UcodeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
