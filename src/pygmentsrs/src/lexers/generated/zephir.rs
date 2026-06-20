//! AUTO-GENERATED from `pygments.pygments.lexers.php:ZephirLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.php:ZephirLexer:zephir

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: zephir
pub struct ZephirLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"commentsandwhitespace",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"slashstartsregex",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token_to(
                r"(?ms)/(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gim]+\b|\B)",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?ms)/", OPERATOR, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"badregex",
        vec![Rule::token_to(r"(?ms)\n", TEXT, NewState::Pop(1))],
    );
    m.insert(r"root", vec![
        Rule::token_to(r"(?ms)^(?=\s|/)", TEXT, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token_to(r"(?ms)\+\+|--|~|&&|\?|:|\|\||\\(?=\n)|(<<|>>>?|==?|!=?|->|[-<>+*%&|^/])=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[})\].]", PUNCTUATION),
        Rule::token_to(r"(?ms)(for|in|while|do|break|return|continue|switch|case|default|if|else|loop|require|inline|throw|try|catch|finally|new|delete|typeof|instanceof|void|namespace|use|extends|this|fetch|isset|unset|echo|fetch|likely|unlikely|empty)\b", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)(var|let|with|function)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)(abstract|boolean|bool|char|class|const|double|enum|export|extends|final|native|goto|implements|import|int|string|interface|long|ulong|char|uchar|float|unsigned|private|protected|public|short|static|self|throws|reverse|transient|volatile|readonly)\b", KEYWORD_RESERVED),
        Rule::token(r"(?ms)(true|false|null|undefined)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|_REQUEST|_COOKIE|_SESSION|_GET|_POST|_SERVER|this|stdClass|range|count|iterator|window)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)[$a-zA-Z_][\w\\]*", NAME_OTHER),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
    ]);
    Table(m)
}

impl Lexer for ZephirLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
