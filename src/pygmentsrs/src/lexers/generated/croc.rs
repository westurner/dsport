//! AUTO-GENERATED from `pygments.pygments.lexers.d:CrocLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.d:CrocLexer:croc

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: croc
pub struct CrocLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"nestedcomment"])),
        Rule::token(r"(?m)(as(?:(?:sert)?)|break|c(?:a(?:se|tch)|lass|ontinue)|d(?:efault|o)|else|f(?:inally|or(?:(?:each)?)|unction)|global|i(?:mport|[fns])|local|module|namespace|return|s(?:cope|uper|witch)|t(?:h(?:is|row)|ry)|vararg|w(?:hile|ith)|yield)\b", KEYWORD),
        Rule::token(r"(?m)(false|true|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)([0-9][0-9_]*)(?=[.eE])(\.[0-9][0-9_]*)?([eE][+\-]?[0-9_]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[bB][01][01_]*", NUMBER_BIN),
        Rule::token(r"(?m)0[xX][0-9a-fA-F][0-9a-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)([0-9][0-9_]*)(?![.eE])", NUMBER_INTEGER),
        Rule::token(r#"(?m)'(\\['"\\nrt]|\\x[0-9a-fA-F]{2}|\\[0-9]{1,3}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|.)'"#, STRING_CHAR),
        Rule::token(r#"(?m)@"(""|[^"])*""#, STRING),
        Rule::token(r"(?m)@`(``|[^`])*`", STRING),
        Rule::token(r"(?m)@'(''|[^'])*'", STRING),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)(~=|\^=|%=|\*=|==|!=|>>>=|>>>|>>=|>>|>=|<=>|\?=|-\>|<<=|<<|<=|\+\+|\+=|--|-=|\|\||\|=|&&|&=|\.\.|/=)|[-/.&$@|\+<>!()\[\]{}?,;:=*%^~#\\]", PUNCTUATION),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(
        r"nestedcomment",
        vec![
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for CrocLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
