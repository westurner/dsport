//! AUTO-GENERATED from `pygments.pygments.lexers.graphics:PostScriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.graphics:PostScriptLexer:postscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: postscript, postscr
pub struct PostscriptLexer;

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
        Rule::token(r"(?m)^%!.+$", COMMENT_PREPROC),
        Rule::token(r"(?m)%%.*$", COMMENT_SPECIAL),
        Rule::token(r"(?m)(^%.*\n){2,}", COMMENT_MULTILINE),
        Rule::token(r"(?m)%.*$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)\(", STRING, NewState::Push(vec![r"stringliteral"])),
        Rule::token(r"(?m)[{}<>\[\]]", PUNCTUATION),
        Rule::token(r"(?m)<[0-9A-Fa-f]+>(?=[()<>\[\]{}/%\s])", NUMBER_HEX),
        Rule::token(r"(?m)[0-9]+\#(\-|\+)?([0-9]+\.?|[0-9]*\.[0-9]+|[0-9]+\.[0-9]*)((e|E)[0-9]+)?(?=[()<>\[\]{}/%\s])", NUMBER_OCT),
        Rule::token(r"(?m)(\-|\+)?([0-9]+\.?|[0-9]*\.[0-9]+|[0-9]+\.[0-9]*)((e|E)[0-9]+)?(?=[()<>\[\]{}/%\s])", NUMBER_FLOAT),
        Rule::token(r"(?m)(\-|\+)?[0-9]+(?=[()<>\[\]{}/%\s])", NUMBER_INTEGER),
        Rule::token(r"(?m)\/[^()<>\[\]{}/%\s]+(?=[()<>\[\]{}/%\s])", NAME_VARIABLE),
        Rule::token(r"(?m)[^()<>\[\]{}/%\s]+(?=[()<>\[\]{}/%\s])", NAME_FUNCTION),
        Rule::token(r"(?m)(false|true)(?=[()<>\[\]{}/%\s])", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(eq|ne|g[et]|l[et]|and|or|not|if(?:else)?|for(?:all)?)(?=[()<>\[\]{}/%\s])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(a(?:bs|dd|load|r(?:c(?:(?:n)?)|ray)|tan)|b(?:egin|ind)|c(?:eiling|harpath|l(?:ip|osepath)|o(?:ncat(?:(?:matrix)?)|py|s)|ur(?:rent(?:linewidth|matrix|point)|veto)|v(?:[is]))|d(?:ef(?:(?:aultmatrix)?)|i(?:ct(?:(?:stackoverflow)?)|v)|transform|up)|e(?:nd|x(?:ch|ec|it|p))|f(?:i(?:ll|ndfont)|loor)|g(?:et(?:(?:interval)?)|restore|save|t)|i(?:d(?:entmatrix|iv|transform)|n(?:(?:de|vertmatri)x)|transform)|l(?:ength|ineto|n|o(?:ad|g|op))|m(?:atrix|o(?:d|veto)|ul)|ne(?:g|wpath)|p(?:ath(?:bbox|forall)|op|rint|stack|ut)|quit|r(?:an(?:d|gecheck)|curveto|e(?:peat|store)|lineto|moveto|o(?:ll|tate|und)|un)|s(?:ave|cale(?:(?:font)?)|et(?:dash|font|gray|line(?:cap|join|width)|matrix|rgbcolor)|h(?:fill|ow(?:(?:page)?))|in|qrt|t(?:ack|r(?:ingwidth|oke(?:(?:path)?)))|ub|yntaxerror)|t(?:r(?:ans(?:form|late)|uncate)|ypecheck)|undefined(?:(?:filename|result)?))(?=[()<>\[\]{}/%\s])", NAME_BUILTIN),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"stringliteral", vec![
        Rule::token(r"(?m)[^()\\]+", STRING),
        Rule::token_to(r"(?m)\\", STRING_ESCAPE, NewState::Push(vec![r"escape"])),
        Rule::token_to(r"(?m)\(", STRING, NewState::PushSame),
        Rule::token_to(r"(?m)\)", STRING, NewState::Pop(1)),
    ]);
    m.insert(r"escape", vec![
        Rule::token_to(r"(?m)[0-8]{3}|n|r|t|b|f|\\|\(|\)", STRING_ESCAPE, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for PostscriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
