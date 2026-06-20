//! AUTO-GENERATED from `pygments.pygments.lexers.berry:BerryLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.berry:BerryLexer:berry

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: berry, be
pub struct BerryLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#-(.|\n)*?-#", COMMENT_MULTILINE),
        Rule::token(r"(?m)#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)(-?\d+\.?|\.\d)\d*([eE][+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)(as|break|continue|import|s(?:elf|tatic|uper))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(true|false|nil)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(var|def)\b", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?m)(def)(\s+)(\b[^\W\d]\w*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)\b(class)(\s+)(\b[^\W\d]\w*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)\b(import)(\s+)(\b[^\W\d]\w*)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)\.\.|[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[(){}\[\],.;]", PUNCTUATION),
        Rule::token(r"(?m)(break|continue|do|e(?:l(?:if|se)|nd|xcept)|for|if|r(?:aise|eturn)|try|while)\b", KEYWORD),
        Rule::token(r"(?m)(assert|b(?:ool|ytes)|c(?:all|lass(?:name|of)|ompile)|file|i(?:n(?:(?:(?:pu)?)t)|s(?:instance|subclass))|list|m(?:ap|odule)|number|open|print|r(?:ange|eal)|s(?:ize|(?:t|upe)r)|type)\b", NAME_BUILTIN),
        Rule::token_to(r"(?m)\b[^\W\d]\w*(?=\s*\()", NAME_FUNCTION, NewState::Pop(1)),
        Rule::token_to(r"(?m)(?<=\.)\b[^\W\d]\w*\b(?!\()", NAME_ATTRIBUTE, NewState::Pop(1)),
        Rule::token(r"(?m)\b[^\W\d]\w*", NAME),
        Rule::token_to(r#"(?m)"([^\\]|\\.)*?""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\'([^\\]|\\.)*?\'", STRING_SINGLE, NewState::Pop(1)),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#-(.|\n)*?-#", COMMENT_MULTILINE),
            Rule::token(r"(?m)#.*?$", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
            Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)(-?\d+\.?|\.\d)\d*([eE][+-]?\d+)?", NUMBER_FLOAT),
        ],
    );
    m.insert(
        r"keywords",
        vec![
            Rule::token(
                r"(?m)(as|break|continue|import|s(?:elf|tatic|uper))\b",
                KEYWORD_RESERVED,
            ),
            Rule::token(r"(?m)(true|false|nil)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?m)(var|def)\b", KEYWORD_DECLARATION),
        ],
    );
    m.insert(r"expr", vec![
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)\.\.|[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[(){}\[\],.;]", PUNCTUATION),
        Rule::token(r"(?m)(break|continue|do|e(?:l(?:if|se)|nd|xcept)|for|if|r(?:aise|eturn)|try|while)\b", KEYWORD),
        Rule::token(r"(?m)(assert|b(?:ool|ytes)|c(?:all|lass(?:name|of)|ompile)|file|i(?:n(?:(?:(?:pu)?)t)|s(?:instance|subclass))|list|m(?:ap|odule)|number|open|print|r(?:ange|eal)|s(?:ize|(?:t|upe)r)|type)\b", NAME_BUILTIN),
        Rule::token_to(r"(?m)\b[^\W\d]\w*(?=\s*\()", NAME_FUNCTION, NewState::Pop(1)),
        Rule::token_to(r"(?m)(?<=\.)\b[^\W\d]\w*\b(?!\()", NAME_ATTRIBUTE, NewState::Pop(1)),
        Rule::token(r"(?m)\b[^\W\d]\w*", NAME),
        Rule::token_to(r#"(?m)"([^\\]|\\.)*?""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\'([^\\]|\\.)*?\'", STRING_SINGLE, NewState::Pop(1)),
    ]);
    m.insert(
        r"controls",
        vec![Rule::token(
            r"(?m)(break|continue|do|e(?:l(?:if|se)|nd|xcept)|for|if|r(?:aise|eturn)|try|while)\b",
            KEYWORD,
        )],
    );
    m.insert(r"builtins", vec![
        Rule::token(r"(?m)(assert|b(?:ool|ytes)|c(?:all|lass(?:name|of)|ompile)|file|i(?:n(?:(?:(?:pu)?)t)|s(?:instance|subclass))|list|m(?:ap|odule)|number|open|print|r(?:ange|eal)|s(?:ize|(?:t|upe)r)|type)\b", NAME_BUILTIN),
    ]);
    m.insert(
        r"funccall",
        vec![Rule::token_to(
            r"(?m)\b[^\W\d]\w*(?=\s*\()",
            NAME_FUNCTION,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"member",
        vec![Rule::token_to(
            r"(?m)(?<=\.)\b[^\W\d]\w*\b(?!\()",
            NAME_ATTRIBUTE,
            NewState::Pop(1),
        )],
    );
    m.insert(r"name", vec![Rule::token(r"(?m)\b[^\W\d]\w*", NAME)]);
    m.insert(
        r"strings",
        vec![
            Rule::token_to(r#"(?m)"([^\\]|\\.)*?""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token_to(r"(?m)\'([^\\]|\\.)*?\'", STRING_SINGLE, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for BerryLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
