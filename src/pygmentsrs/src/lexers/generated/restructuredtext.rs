//! AUTO-GENERATED from `pygments.pygments.lexers.markup:RstLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.markup:RstLexer:restructuredtext

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{
    DispatchCodeBlockSpec, GroupAction, GroupEmit, NewState, Rule, StateTable, tokenize,
};
use crate::token::*;

/// Aliases: restructuredtext, rst, rest
pub struct RestructuredtextLexer;

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
        Rule::bygroups(r#"(?m)^(=+|-+|`+|:+|\.+|\'+|"+|~+|\^+|_+|\*+|\++|#+)([ \t]*\n)(.+)(\n)(\1)(\n)"#, vec![Some(GENERIC_HEADING), Some(TEXT), Some(GENERIC_HEADING), Some(TEXT), Some(GENERIC_HEADING), Some(TEXT)]),
        Rule::bygroups(r#"(?m)^(\S.*)(\n)(={3,}|-{3,}|`{3,}|:{3,}|\.{3,}|\'{3,}|"{3,}|~{3,}|\^{3,}|_{3,}|\*{3,}|\+{3,}|#{3,})(\n)"#, vec![Some(GENERIC_HEADING), Some(TEXT), Some(GENERIC_HEADING), Some(TEXT)]),
        Rule::bygroups_g(r"(?m)^(\s*)([-*+])( .+\n(?:\1  .+\n)*)", vec![Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(NUMBER)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::bygroups_g(r"(?m)^(\s*)([0-9#ivxlcmIVXLCM]+\.)( .+\n(?:\1  .+\n)*)", vec![Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(NUMBER)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::bygroups_g(r"(?m)^(\s*)(\(?[0-9#ivxlcmIVXLCM]+\))( .+\n(?:\1  .+\n)*)", vec![Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(NUMBER)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::bygroups_g(r"(?m)^(\s*)([A-Z]+\.)( .+\n(?:\1  .+\n)+)", vec![Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(NUMBER)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::bygroups_g(r"(?m)^(\s*)(\(?[A-Za-z]+\))( .+\n(?:\1  .+\n)+)", vec![Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(NUMBER)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::bygroups_g(r"(?m)^(\s*)(\|)( .+\n(?:\|  .+\n)*)", vec![Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(OPERATOR)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::dispatch_code_block(r"(?m)^( *\.\.)(\s*)((?:source)?code(?:-block)?)(::)([ \t]*)([^\n]+)(\n[ \t]*\n)([ \t]+)(.*)(\n)((?:(?:\8.*)?\n)+)", DispatchCodeBlockSpec {
            prefix: vec![GroupEmit { group: 1, token: PUNCTUATION, skip_if_none: false }, GroupEmit { group: 2, token: TEXT, skip_if_none: false }, GroupEmit { group: 3, token: TokenType::new(&["Operator", "Word"]), skip_if_none: false }, GroupEmit { group: 4, token: PUNCTUATION, skip_if_none: false }, GroupEmit { group: 5, token: TEXT, skip_if_none: false }, GroupEmit { group: 6, token: KEYWORD, skip_if_none: false }, GroupEmit { group: 7, token: TEXT, skip_if_none: false }],
            lang_group: 6,
            code_groups: vec![8, 9, 10, 11],
            suffix: vec![],
            fallback_token: STRING,
            strip_indent_from_group: Some(8),
        }),
        Rule::bygroups_g(r"(?m)^( *\.\.)(\s*)([\w:-]+?)(::)(?:([ \t]*)(.*))", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(OPERATOR_WORD)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(TEXT)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::bygroups_g(r"(?m)^( *\.\.)(\s*)(_(?:[^:\\]|\\.)+:)(.*?)$", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(NAME_TAG)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::bygroups_g(r"(?m)^( *\.\.)(\s*)(\[.+\])(.*?)$", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(NAME_TAG)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::bygroups_g(r"(?m)^( *\.\.)(\s*)(\|.+\|)(\s*)([\w:-]+?)(::)(?:([ \t]*)(.*))", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(NAME_TAG)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(OPERATOR_WORD)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(TEXT)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::token(r"(?m)^ *\.\..*(\n( +.*\n|\n)+)?", COMMENT),
        Rule::bygroups(r"(?m)^( *)(:(?:\\\\|\\:|[^:\n])+:(?=\s))([ \t]*)", vec![Some(TEXT), Some(NAME_CLASS), Some(TEXT)]),
        Rule::bygroups_g(r"(?m)^(\S.*(?<!::)\n)((?:(?: +.*)\n)+)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) }), Some(GroupAction::UsingThis { state: Some(vec!["root", "inline"]) })]),
        Rule::bygroups(r"(?m)(::)(\n[ \t]*\n)([ \t]+)(.*)(\n)((?:(?:\3.*)?\n)+)", vec![Some(STRING_ESCAPE), Some(TEXT), Some(STRING), Some(STRING), Some(TEXT), Some(STRING)]),
        Rule::token(r"(?m)\\.", TEXT),
        Rule::token_to(r"(?m)``", STRING, NewState::Push(vec![r"literal"])),
        Rule::bygroups(r"(?m)(`.+?)(<.+?>)(`__?)", vec![Some(STRING), Some(STRING_INTERPOL), Some(STRING)]),
        Rule::token(r"(?m)`.+?`__?", STRING),
        Rule::bygroups(r"(?m)(`.+?`)(:[a-zA-Z0-9:-]+?:)?", vec![Some(NAME_VARIABLE), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?m)(:[a-zA-Z0-9:-]+?:)(`.+?`)", vec![Some(NAME_ATTRIBUTE), Some(NAME_VARIABLE)]),
        Rule::token(r"(?m)\*\*.+?\*\*", GENERIC_STRONG),
        Rule::token(r"(?m)\*.+?\*", GENERIC_EMPH),
        Rule::token(r"(?m)\[.*?\]_", STRING),
        Rule::token(r"(?m)<.+?>", NAME_TAG),
        Rule::token(r"(?m)[^\\\n\[*`:]+", TEXT),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(
        r"inline",
        vec![
            Rule::token(r"(?m)\\.", TEXT),
            Rule::token_to(r"(?m)``", STRING, NewState::Push(vec![r"literal"])),
            Rule::bygroups(
                r"(?m)(`.+?)(<.+?>)(`__?)",
                vec![Some(STRING), Some(STRING_INTERPOL), Some(STRING)],
            ),
            Rule::token(r"(?m)`.+?`__?", STRING),
            Rule::bygroups(
                r"(?m)(`.+?`)(:[a-zA-Z0-9:-]+?:)?",
                vec![Some(NAME_VARIABLE), Some(NAME_ATTRIBUTE)],
            ),
            Rule::bygroups(
                r"(?m)(:[a-zA-Z0-9:-]+?:)(`.+?`)",
                vec![Some(NAME_ATTRIBUTE), Some(NAME_VARIABLE)],
            ),
            Rule::token(r"(?m)\*\*.+?\*\*", GENERIC_STRONG),
            Rule::token(r"(?m)\*.+?\*", GENERIC_EMPH),
            Rule::token(r"(?m)\[.*?\]_", STRING),
            Rule::token(r"(?m)<.+?>", NAME_TAG),
            Rule::token(r"(?m)[^\\\n\[*`:]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"literal",
        vec![
            Rule::token(r"(?m)[^`]+", STRING),
            Rule::token_to(
                r#"(?m)``((?=$)|(?=[-/:.,; \n\x00‐‑‒–— '"\)\]\}>’”»!\?]))"#,
                STRING,
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)`", STRING),
        ],
    );
    Table(m)
}

impl Lexer for RestructuredtextLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
