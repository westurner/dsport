//! AUTO-GENERATED from `pygments.pygments.lexers.pony:PonyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.pony:PonyLexer:pony

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pony
pub struct PonyLexer;

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
        Rule::token(r"(?m)\n", TEXT),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)//.*\n", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"nested_comment"])),
        Rule::token(r#"(?m)"""(?:.|\n)*?""""#, STRING_DOC),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\'.*\'", STRING_CHAR),
        Rule::token(r"(?m)=>|[\]{}:().~;,|&!^?\[]", PUNCTUATION),
        Rule::token(r"(?m)(a(?:ddressof|nd|s)|consume|digestof|is(?:(?:nt)?)|not|or)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|[-+/*%=<>]", OPERATOR),
        Rule::token(r"(?m)(\#(?:any|read|s(?:end|hare))|b(?:ox|reak)|co(?:mpile_(?:error|intrinsic)|ntinue)|do|e(?:lse(?:(?:if)?)|mbed|nd|rror)|for|i(?:fdef|so|[fn])|l(?:ambda|et)|match|object|re(?:cover|f|peat|turn)|t(?:ag|h(?:en|is)|r(?:[ny]))|u(?:ntil|se)|va(?:[lr])|w(?:h(?:(?:er|il)e)|ith))\b", KEYWORD),
        Rule::bygroups_to(r"(?m)(actor|class|struct|primitive|interface|trait|type)((?:\s)+)", vec![Some(KEYWORD), Some(TEXT)], NewState::Push(vec![r"typename"])),
        Rule::bygroups_to(r"(?m)(new|fun|be)((?:\s)+)", vec![Some(KEYWORD), Some(TEXT)], NewState::Push(vec![r"methodname"])),
        Rule::token(r"(?m)(A(?:(?:n|rra)y)|Bool|F(?:32|64)|I(?:1(?:28|6)|32|64|8|Long|Size|terator)|None|Pointer|String|U(?:1(?:28|6)|32|64|8|Long|Size))\b", TokenType::new(&["Name", "Builtin", "Type"])),
        Rule::token(r"(?m)_?[A-Z]\w*", TokenType::new(&["Name", "Type"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)(true|false)\b", NAME_BUILTIN),
        Rule::token(r"(?m)_\d*", NAME),
        Rule::token(r"(?m)_?[a-z][\w\']*", NAME),
    ]);
    m.insert(
        r"typename",
        vec![Rule::bygroups_to(
            r"(?m)(iso|trn|ref|val|box|tag)?((?:\s)*)(_?[A-Z]\w*)",
            vec![Some(KEYWORD), Some(TEXT), Some(NAME_CLASS)],
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"methodname",
        vec![Rule::bygroups_to(
            r"(?m)(iso|trn|ref|val|box|tag)?((?:\s)*)(_?[a-z]\w*)",
            vec![Some(KEYWORD), Some(TEXT), Some(NAME_FUNCTION)],
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"nested_comment",
        vec![
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)\\""#, STRING),
            Rule::token(r#"(?m)[^\\"]+"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for PonyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
