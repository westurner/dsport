#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.webmisc:QmlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.webmisc:QmlLexer:qml

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: qml, qbs
pub struct QmlLexer;

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
            Rule::token(r"(?ms)<!--", COMMENT),
            Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"slashstartsregex",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token(r"(?ms)<!--", COMMENT),
            Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token_to(
                r"(?ms)/(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gim]+\b|\B)",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?ms)(?=/)",
                TEXT,
                NewState::Push(vec![r"#pop", r"badregex"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"badregex",
        vec![Rule::token_to(r"(?ms)\n", TEXT, NewState::Pop(1))],
    );
    m.insert(r"root", vec![
        Rule::token_to(r"(?ms)^(?=\s|/|<!--)", TEXT, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)<!--", COMMENT),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token_to(r"(?ms)\+\+|--|~|&&|\?|:|\|\||\\(?=\n)|(<<|>>>?|==?|!=?|[-<>+*%&|^/])=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[})\].]", PUNCTUATION),
        Rule::token_to(r"(?ms)\bid\s*:\s*[A-Za-z][\w.]*", KEYWORD_DECLARATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)\b[A-Za-z][\w.]*\s*:", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)(for|in|while|do|break|return|continue|switch|case|default|if|else|throw|try|catch|finally|new|delete|typeof|instanceof|void|this)\b", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)(var|let|with|function)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)(abstract|boolean|byte|char|class|const|debugger|double|enum|export|extends|final|float|goto|implements|import|int|interface|long|native|package|private|protected|public|short|static|super|synchronized|throws|transient|volatile)\b", KEYWORD_RESERVED),
        Rule::token(r"(?ms)(true|false|null|NaN|Infinity|undefined)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Math|netscape|Number|Object|Packages|RegExp|String|sun|decodeURI|decodeURIComponent|encodeURI|encodeURIComponent|Error|eval|isFinite|isNaN|parseFloat|parseInt|document|this|window)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)[$a-zA-Z_]\w*", NAME_OTHER),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
    ]);
    Table(m)
}

impl Lexer for QmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
