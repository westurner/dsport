//! AUTO-GENERATED from `pygments.pygments.lexers.textedit:AwkLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.textedit:AwkLexer:awk

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: awk, gawk, mawk, nawk
pub struct AwkLexer;

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
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"slashstartsregex",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/\B",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)(?=/)",
                TEXT,
                NewState::Push(vec![r"#pop", r"badregex"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"badregex",
        vec![Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1))],
    );
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)^(?=\s|/)", TEXT, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)\+\+|--|\|\||&&|in\b|\$|!?~|\?|:|(\*\*|[-<>+*%\^/!=|])=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?m)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?m)[})\].]", PUNCTUATION),
        Rule::token_to(r"(?m)(break|continue|do|while|exit|for|if|else|return)\b", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?m)function\b", KEYWORD_DECLARATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?m)(atan2|cos|exp|int|log|rand|sin|sqrt|srand|gensub|gsub|index|length|match|split|sprintf|sub|substr|tolower|toupper|close|fflush|getline|next|nextfile|print|printf|strftime|systime|delete|system)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(ARGC|ARGIND|ARGV|BEGIN|CONVFMT|ENVIRON|END|ERRNO|FIELDWIDTHS|FILENAME|FNR|FS|IGNORECASE|NF|NR|OFMT|OFS|ORFS|RLENGTH|RS|RSTART|RT|SUBSEP)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[$a-zA-Z_]\w*", NAME_OTHER),
        Rule::token(r"(?m)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
    ]);
    Table(m)
}

impl Lexer for AwkLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
