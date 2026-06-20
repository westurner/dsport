//! AUTO-GENERATED from `pygments.pygments.lexers.parsers:RagelLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.parsers:RagelLexer:ragel

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ragel
pub struct RagelLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"whitespace", vec![Rule::token(r"(?m)\s+", WHITESPACE)]);
    m.insert(r"comments", vec![Rule::token(r"(?m)\#.*$", COMMENT)]);
    m.insert(
        r"keywords",
        vec![
            Rule::token(r"(?m)(access|action|alphtype)\b", KEYWORD),
            Rule::token(r"(?m)(getkey|write|machine|include)\b", KEYWORD),
            Rule::token(
                r"(?m)(any|ascii|extend|alpha|digit|alnum|lower|upper)\b",
                KEYWORD,
            ),
            Rule::token(
                r"(?m)(xdigit|cntrl|graph|print|punct|space|zlen|empty)\b",
                KEYWORD,
            ),
        ],
    );
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)0x[0-9A-Fa-f]+", NUMBER_HEX),
            Rule::token(r"(?m)[+-]?[0-9]+", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"literals",
        vec![
            Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?m)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?m)\[(\\\\|\\[^\\]|[^\\\]])*\]", STRING),
            Rule::token(r"(?m)/(?!\*)(\\\\|\\[^\\]|[^/\\])*/", STRING_REGEX),
        ],
    );
    m.insert(
        r"identifiers",
        vec![Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE)],
    );
    m.insert(
        r"operators",
        vec![
            Rule::token(r"(?m),", OPERATOR),
            Rule::token(r"(?m)\||&|--?", OPERATOR),
            Rule::token(r"(?m)\.|<:|:>>?", OPERATOR),
            Rule::token(r"(?m):", OPERATOR),
            Rule::token(r"(?m)->", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(/|eof\b)", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(!|err\b)", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(\^|lerr\b)", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(~|to\b)", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(\*|from\b)", OPERATOR),
            Rule::token(r"(?m)>|@|\$|%", OPERATOR),
            Rule::token(r"(?m)\*|\?|\+|\{[0-9]*,[0-9]*\}", OPERATOR),
            Rule::token(r"(?m)!|\^", OPERATOR),
            Rule::token(r"(?m)\(|\)", OPERATOR),
        ],
    );
    m.insert(
        r"root",
        vec![
            Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?m)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?m)\[(\\\\|\\[^\\]|[^\\\]])*\]", STRING),
            Rule::token(r"(?m)/(?!\*)(\\\\|\\[^\\]|[^/\\])*/", STRING_REGEX),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\#.*$", COMMENT),
            Rule::token(r"(?m)(access|action|alphtype)\b", KEYWORD),
            Rule::token(r"(?m)(getkey|write|machine|include)\b", KEYWORD),
            Rule::token(
                r"(?m)(any|ascii|extend|alpha|digit|alnum|lower|upper)\b",
                KEYWORD,
            ),
            Rule::token(
                r"(?m)(xdigit|cntrl|graph|print|punct|space|zlen|empty)\b",
                KEYWORD,
            ),
            Rule::token(r"(?m)0x[0-9A-Fa-f]+", NUMBER_HEX),
            Rule::token(r"(?m)[+-]?[0-9]+", NUMBER_INTEGER),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m),", OPERATOR),
            Rule::token(r"(?m)\||&|--?", OPERATOR),
            Rule::token(r"(?m)\.|<:|:>>?", OPERATOR),
            Rule::token(r"(?m):", OPERATOR),
            Rule::token(r"(?m)->", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(/|eof\b)", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(!|err\b)", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(\^|lerr\b)", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(~|to\b)", OPERATOR),
            Rule::token(r"(?m)(>|\$|%|<|@|<>)(\*|from\b)", OPERATOR),
            Rule::token(r"(?m)>|@|\$|%", OPERATOR),
            Rule::token(r"(?m)\*|\?|\+|\{[0-9]*,[0-9]*\}", OPERATOR),
            Rule::token(r"(?m)!|\^", OPERATOR),
            Rule::token(r"(?m)\(|\)", OPERATOR),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"host"])),
            Rule::token(r"(?m)=", OPERATOR),
            Rule::token(r"(?m);", PUNCTUATION),
        ],
    );
    m.insert(r"host", vec![
        Rule::token(r#"(?m)([^{}\'"/#]+|[^\\]\\[{}]|"(\\\\|\\[^\\]|[^"\\])*"|'(\\\\|\\[^\\]|[^'\\])*'|//.*$\n?|/\*(.|\n)*?\*/|\#.*$\n?|/(?!\*)(\\\\|\\[^\\]|[^/\\])*/|/)+"#, OTHER),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for RagelLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
