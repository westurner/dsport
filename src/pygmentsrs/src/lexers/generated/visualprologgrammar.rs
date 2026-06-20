//! AUTO-GENERATED from `pygments.pygments.lexers.vip:VisualPrologGrammarLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.vip:VisualPrologGrammarLexer:visualprologgrammar

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: visualprologgrammar
pub struct VisualprologgrammarLexer;

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
        Rule::token(r"(?m)(grammar|n(?:amespace|onterminals)|open|precedence|(?:rule|(?:startsymbo|termina)l)s)\b", KEYWORD),
        Rule::token(r"(?m)#((?:bin|string)include)\b", TokenType::new(&["Keyword", "Directive"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\bend\s+(foreach|if|try)\b", TokenType::new(&["Keyword", "Minor"])),
        Rule::token(r"(?m)end", KEYWORD),
        Rule::token(r"(?m)(and|catch|d(?:iv|o)|e(?:lse(?:(?:if)?)|rroneous|xternally)|f(?:ailure|inally|oreach)|if|mod|o(?:r(?:(?:else)?)|therwise)|quot|rem|t(?:hen|ry))\b", TokenType::new(&["Keyword", "Minor"])),
        Rule::token(r"(?m)0[xo][\da-fA-F_]+", NUMBER),
        Rule::token(r"(?m)((\d[\d_]*)?\.)?\d[\d_]*([eE][\-+]?\d+)?", NUMBER),
        Rule::token(r"(?m)_\w*", TokenType::new(&["Name", "Variable", "Anonymous"])),
        Rule::token(r"(?m)[A-Z]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)@\w+", NAME_VARIABLE),
        Rule::token(r"(?m)[a-z]\w*", NAME),
        Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token_to(r"(?m)\%", COMMENT, NewState::Push(vec![r"commentline"])),
        Rule::token_to(r#"(?m)""#, STRING_SYMBOL, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)\'", STRING_SYMBOL, NewState::Push(vec![r"stringsingle"])),
        Rule::token_to(r#"(?m)@""#, STRING_SYMBOL, NewState::Push(vec![r"atstring"])),
        Rule::token(r"(?m)[\-+*^/!?<>=~:]+", OPERATOR),
        Rule::token(r"(?m)[$,.\[\]|(){}\\]+", PUNCTUATION),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(
        r"commentdoc",
        vec![
            Rule::token(r"(?m)@(detail|end|short|withdomain)\b", COMMENT_PREPROC),
            Rule::token(r"(?m)@", COMMENT),
        ],
    );
    m.insert(
        r"commentline",
        vec![
            Rule::token(r"(?m)@(detail|end|short|withdomain)\b", COMMENT_PREPROC),
            Rule::token(r"(?m)@", COMMENT),
            Rule::token(r"(?m)[^@\n]+", COMMENT),
            Rule::token_to(r"(?m)$", COMMENT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)@(detail|end|short|withdomain)\b", COMMENT_PREPROC),
            Rule::token(r"(?m)@", COMMENT),
            Rule::token(r"(?m)[^@*/]+", COMMENT),
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT),
        ],
    );
    m.insert(
        r"stringescape",
        vec![
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r#"(?m)\\[\'"ntr\\]"#, STRING_ESCAPE),
        ],
    );
    m.insert(
        r"stringsingle",
        vec![
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r#"(?m)\\[\'"ntr\\]"#, STRING_ESCAPE),
            Rule::token_to(r"(?m)\'", STRING_SYMBOL, NewState::Pop(1)),
            Rule::token(r"(?m)[^\'\\\n]+", STRING),
            Rule::token_to(
                r"(?m)\n",
                TokenType::new(&["Literal", "String", "Escape", "Error"]),
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r#"(?m)\\[\'"ntr\\]"#, STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING_SYMBOL, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"\\\n]+"#, STRING),
            Rule::token_to(
                r"(?m)\n",
                TokenType::new(&["Literal", "String", "Escape", "Error"]),
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(
        r"atstring",
        vec![
            Rule::token(r#"(?m)"""#, STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING_SYMBOL, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"]+"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for VisualprologgrammarLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
