//! AUTO-GENERATED from `pygments.pygments.lexers.c_like:ClayLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.c_like:ClayLexer:clay

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: clay
pub struct ClayLexer;

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
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)\b(public|private|import|as|record|variant|instance|define|overload|default|external|alias|rvalue|ref|forward|inline|noinline|forceinline|enum|var|and|or|not|if|else|goto|return|while|switch|case|break|continue|for|in|true|false|try|catch|throw|finally|onerror|staticassert|eval|when|newtype|__FILE__|__LINE__|__COLUMN__|__ARG__)\b", KEYWORD),
        Rule::token(r"(?m)[~!%^&*+=|:<>/-]", OPERATOR),
        Rule::token(r"(?m)[#(){}\[\],;.]", PUNCTUATION),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[LlUu]*", NUMBER_HEX),
        Rule::token(r"(?m)\d+[LlUu]*", NUMBER_INTEGER),
        Rule::token(r"(?m)\b(true|false)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?i)[a-z_?][\w?]*", NAME),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"dqs"])),
    ]);
    m.insert(r"strings", vec![
        Rule::token(r"(?m)(?i)\\(x[0-9a-f]{2}|.)", STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"]+"#, STRING),
    ]);
    m.insert(r"nl", vec![
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"dqs", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?m)(?i)\\(x[0-9a-f]{2}|.)", STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"]+"#, STRING),
    ]);
    m.insert(r"tdqs", vec![
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?m)(?i)\\(x[0-9a-f]{2}|.)", STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"]+"#, STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    Table(m)
}

impl Lexer for ClayLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
