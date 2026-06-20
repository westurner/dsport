//! AUTO-GENERATED from `pygments.pygments.lexers.savi:SaviLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.savi:SaviLexer:savi

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: savi
pub struct SaviLexer;

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
        r"root",
        vec![
            Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
            Rule::token(r"(?m)::.*?$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\')(\w+)(?=[^\'])", vec![Some(OPERATOR), Some(NAME)]),
            Rule::token_to(
                r#"(?m)\w?""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string.double"]),
            ),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"string.char"])),
            Rule::token(r"(?m)(_?[A-Z]\w*)", NAME_CLASS),
            Rule::bygroups(
                r"(?m)(\.)(\s*)(_?[A-Z]\w*)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)],
            ),
            Rule::bygroups_to(
                r"(?m)^([ \t]*)(:\w+)",
                vec![Some(WHITESPACE), Some(NAME_TAG)],
                NewState::Push(vec![r"decl"]),
            ),
            Rule::token(r"(?m)((\w+|\+|\-|\*)\!)", GENERIC_DELETED),
            Rule::token(r"(?m)\b\d([\d_]*(\.[\d_]+)?)\b", NUMBER),
            Rule::token(r"(?m)\b0x([0-9a-fA-F_]+)\b", NUMBER_HEX),
            Rule::token(r"(?m)\b0b([01_]+)\b", NUMBER_BIN),
            Rule::token(r"(?m)\w+(?=\()", NAME_FUNCTION),
            Rule::bygroups(
                r"(?m)(\.)(\s*)(\w+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_FUNCTION)],
            ),
            Rule::bygroups(
                r"(?m)(@)(\w+)",
                vec![Some(PUNCTUATION), Some(NAME_FUNCTION)],
            ),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::bygroups_to(
                r"(?m)(\])(\!)",
                vec![Some(PUNCTUATION), Some(GENERIC_DELETED)],
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[,;:\.@]", PUNCTUATION),
            Rule::token(r"(?m)(\|\>)", OPERATOR),
            Rule::token(r"(?m)(\&\&|\|\||\?\?|\&\?|\|\?|\.\?)", OPERATOR),
            Rule::token(r"(?m)(\<\=\>|\=\~|\=\=|\<\=|\>\=|\<|\>)", OPERATOR),
            Rule::token(r"(?m)(\+|\-|\/|\*|\%)", OPERATOR),
            Rule::token(r"(?m)(\=)", OPERATOR),
            Rule::token(r"(?m)(\!|\<\<|\<|\&|\|)", OPERATOR),
            Rule::token(r"(?m)\b\w+\b", NAME),
            Rule::token(r"(?m)[ \t\r]+\n*|\n+", WHITESPACE),
        ],
    );
    m.insert(
        r"decl",
        vec![
            Rule::token(r"(?m)\b[a-z_]\w*\b(?!\!)", KEYWORD_DECLARATION),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
            Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
            Rule::token(r"(?m)::.*?$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\')(\w+)(?=[^\'])", vec![Some(OPERATOR), Some(NAME)]),
            Rule::token_to(
                r#"(?m)\w?""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string.double"]),
            ),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"string.char"])),
            Rule::token(r"(?m)(_?[A-Z]\w*)", NAME_CLASS),
            Rule::bygroups(
                r"(?m)(\.)(\s*)(_?[A-Z]\w*)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)],
            ),
            Rule::bygroups_to(
                r"(?m)^([ \t]*)(:\w+)",
                vec![Some(WHITESPACE), Some(NAME_TAG)],
                NewState::Push(vec![r"decl"]),
            ),
            Rule::token(r"(?m)((\w+|\+|\-|\*)\!)", GENERIC_DELETED),
            Rule::token(r"(?m)\b\d([\d_]*(\.[\d_]+)?)\b", NUMBER),
            Rule::token(r"(?m)\b0x([0-9a-fA-F_]+)\b", NUMBER_HEX),
            Rule::token(r"(?m)\b0b([01_]+)\b", NUMBER_BIN),
            Rule::token(r"(?m)\w+(?=\()", NAME_FUNCTION),
            Rule::bygroups(
                r"(?m)(\.)(\s*)(\w+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_FUNCTION)],
            ),
            Rule::bygroups(
                r"(?m)(@)(\w+)",
                vec![Some(PUNCTUATION), Some(NAME_FUNCTION)],
            ),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::bygroups_to(
                r"(?m)(\])(\!)",
                vec![Some(PUNCTUATION), Some(GENERIC_DELETED)],
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[,;:\.@]", PUNCTUATION),
            Rule::token(r"(?m)(\|\>)", OPERATOR),
            Rule::token(r"(?m)(\&\&|\|\||\?\?|\&\?|\|\?|\.\?)", OPERATOR),
            Rule::token(r"(?m)(\<\=\>|\=\~|\=\=|\<\=|\>\=|\<|\>)", OPERATOR),
            Rule::token(r"(?m)(\+|\-|\/|\*|\%)", OPERATOR),
            Rule::token(r"(?m)(\=)", OPERATOR),
            Rule::token(r"(?m)(\!|\<\<|\<|\&|\|)", OPERATOR),
            Rule::token(r"(?m)\b\w+\b", NAME),
            Rule::token(r"(?m)[ \t\r]+\n*|\n+", WHITESPACE),
        ],
    );
    m.insert(
        r"string.double",
        vec![
            Rule::token_to(
                r"(?m)\\\(",
                STRING_INTERPOL,
                NewState::Push(vec![r"string.interpolation"]),
            ),
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
            Rule::token(r"(?m)\\[bfnrt\\\']", STRING_ESCAPE),
            Rule::token(r#"(?m)\\""#, STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?m)[^\\"]+"#, STRING_DOUBLE),
            Rule::token(r"(?m).", ERROR),
        ],
    );
    m.insert(
        r"string.char",
        vec![
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
            Rule::token(r"(?m)\\[bfnrt\\\']", STRING_ESCAPE),
            Rule::token(r"(?m)\\'", STRING_ESCAPE),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Pop(1)),
            Rule::token(r"(?m)[^\\']+", STRING_CHAR),
            Rule::token(r"(?m).", ERROR),
        ],
    );
    m.insert(
        r"string.interpolation",
        vec![
            Rule::token_to(r"(?m)\)", STRING_INTERPOL, NewState::Pop(1)),
            Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
            Rule::token(r"(?m)::.*?$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)(\')(\w+)(?=[^\'])", vec![Some(OPERATOR), Some(NAME)]),
            Rule::token_to(
                r#"(?m)\w?""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string.double"]),
            ),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"string.char"])),
            Rule::token(r"(?m)(_?[A-Z]\w*)", NAME_CLASS),
            Rule::bygroups(
                r"(?m)(\.)(\s*)(_?[A-Z]\w*)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)],
            ),
            Rule::bygroups_to(
                r"(?m)^([ \t]*)(:\w+)",
                vec![Some(WHITESPACE), Some(NAME_TAG)],
                NewState::Push(vec![r"decl"]),
            ),
            Rule::token(r"(?m)((\w+|\+|\-|\*)\!)", GENERIC_DELETED),
            Rule::token(r"(?m)\b\d([\d_]*(\.[\d_]+)?)\b", NUMBER),
            Rule::token(r"(?m)\b0x([0-9a-fA-F_]+)\b", NUMBER_HEX),
            Rule::token(r"(?m)\b0b([01_]+)\b", NUMBER_BIN),
            Rule::token(r"(?m)\w+(?=\()", NAME_FUNCTION),
            Rule::bygroups(
                r"(?m)(\.)(\s*)(\w+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_FUNCTION)],
            ),
            Rule::bygroups(
                r"(?m)(@)(\w+)",
                vec![Some(PUNCTUATION), Some(NAME_FUNCTION)],
            ),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::bygroups_to(
                r"(?m)(\])(\!)",
                vec![Some(PUNCTUATION), Some(GENERIC_DELETED)],
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[,;:\.@]", PUNCTUATION),
            Rule::token(r"(?m)(\|\>)", OPERATOR),
            Rule::token(r"(?m)(\&\&|\|\||\?\?|\&\?|\|\?|\.\?)", OPERATOR),
            Rule::token(r"(?m)(\<\=\>|\=\~|\=\=|\<\=|\>\=|\<|\>)", OPERATOR),
            Rule::token(r"(?m)(\+|\-|\/|\*|\%)", OPERATOR),
            Rule::token(r"(?m)(\=)", OPERATOR),
            Rule::token(r"(?m)(\!|\<\<|\<|\&|\|)", OPERATOR),
            Rule::token(r"(?m)\b\w+\b", NAME),
            Rule::token(r"(?m)[ \t\r]+\n*|\n+", WHITESPACE),
        ],
    );
    Table(m)
}

impl Lexer for SaviLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
