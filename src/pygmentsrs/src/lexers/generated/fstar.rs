//! AUTO-GENERATED from `pygments.pygments.lexers.ml:FStarLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ml:FStarLexer:fstar

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: fstar
pub struct FstarLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"escape-sequence", vec![
        Rule::token(r#"(?m)\\[\\"\'ntbr]"#, STRING_ESCAPE),
        Rule::token(r"(?m)\\[0-9]{3}", STRING_ESCAPE),
        Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)false|true|False|True|\(\)|\[\]", NAME_BUILTIN_PSEUDO),
        Rule::token_to(r"(?m)\b([A-Z][\w\']*)(?=\s*\.)", NAME_NAMESPACE, NewState::Push(vec![r"dotted"])),
        Rule::token(r"(?m)\b([A-Z][\w\']*)", NAME_CLASS),
        Rule::token_to(r"(?m)\(\*(?![)])", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)\/\/.+$", COMMENT),
        Rule::token(r"(?m)\b(abstract|attributes|noeq|unopteq|andbegin|by|default|effect|else|end|ensures|exception|exists|false|forall|fun|function|if|in|include|inline|inline_for_extraction|irreducible|logic|match|module|mutable|new|new_effect|noextract|of|open|opaque|private|range_of|reifiable|reify|reflectable|requires|set_range_of|sub_effect|synth|then|total|true|try|type|unfold|unfoldable|val|when|with|not)\b", KEYWORD),
        Rule::token(r"(?m)\b(assume|admit|assert|calc)\b", NAME_EXCEPTION),
        Rule::token(r"(?m)\b(let|rec)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(\$|\}|\||\{|\|\]|\]|\|>|\[\||\[@|\[|!\{|%\[|=|;;|;|:=|::|:|\{:pattern|\.\[\||\.\(\||\.\(|\.\[|\?\.|\?|\.|==>|<==>|<--|<-|->|~>|,|\(\)|\)|\(|&|u#|#|\|\)|\(\||<@|<:|\\/|/\\|-|~)", OPERATOR),
        Rule::token(r"(?m)([=<>@^|&+\*/$%-]|[!?~])?[!$%&*+\./:<=>?@^|~-]", OPERATOR),
        Rule::token(r"(?m)\b(unit|int|float|bool|string|char|list|array)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[^\W\d][\w']*", NAME),
        Rule::token(r"(?m)-?\d[\d_]*(.[\d_]*)?([eE][+\-]?\d[\d_]*)", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX][\da-fA-F][\da-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)0[oO][0-7][0-7_]*", NUMBER_OCT),
        Rule::token(r"(?m)0[bB][01][01_]*", NUMBER_BIN),
        Rule::token(r"(?m)\d[\d_]*", NUMBER_INTEGER),
        Rule::token(r#"(?m)'(?:(\\[\\\"'ntbr ])|(\\[0-9]{3})|(\\x[0-9a-fA-F]{2}))'"#, STRING_CHAR),
        Rule::token(r"(?m)'.'", STRING_CHAR),
        Rule::token(r"(?m)'", KEYWORD),
        Rule::token(r"(?m)\`([\w\'.]+)\`", OPERATOR_WORD),
        Rule::token(r"(?m)\`", KEYWORD),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[~?][a-z][\w\']*:", NAME_VARIABLE),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)[^(*)]+", COMMENT),
        Rule::token_to(r"(?m)\(\*", COMMENT, NewState::PushSame),
        Rule::token_to(r"(?m)\*\)", COMMENT, NewState::Pop(1)),
        Rule::token(r"(?m)[(*)]", COMMENT),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)[^\\"]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)\\[\\"\'ntbr]"#, STRING_ESCAPE),
        Rule::token(r"(?m)\\[0-9]{3}", STRING_ESCAPE),
        Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        Rule::token(r"(?m)\\\n", STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
    ]);
    m.insert(r"dotted", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\.", PUNCTUATION),
        Rule::token(r"(?m)[A-Z][\w\']*(?=\s*\.)", NAME_NAMESPACE),
        Rule::token_to(r"(?m)[A-Z][\w\']*", NAME_CLASS, NewState::Pop(1)),
        Rule::token_to(r"(?m)[a-z_][\w\']*", NAME, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for FstarLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
