//! AUTO-GENERATED from `pygments.pygments.lexers.bitbake:BitBakeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.bitbake:BitBakeLexer:bitbake

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bitbake
pub struct BitbakeLexer;

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
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::bygroups_g(r"(?m)(^(?:fakeroot[ \t]+)?)(python)((?:[ \t]+[A-Za-z_][A-Za-z0-9_\-.+]*)?)([ \t]*\([ \t]*\)[ \t]*)(\{[ \t]*\n)((?:.*\n)*?)(^\}[ \t]*$)", vec![Some(GroupAction::Token(KEYWORD_TYPE)), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingLexer { alias: "python", state: None }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)(^(?:fakeroot[ \t]+)?)([A-Za-z_][A-Za-z0-9_\-.+]*)((?::[A-Za-z0-9_\-.+${}]+)*)([ \t]*\([ \t]*\)[ \t]*)(\{[ \t]*\n)((?:.*\n)*?)(^\}[ \t]*$)", vec![Some(GroupAction::Token(KEYWORD_TYPE)), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(NAME_DECORATOR)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingLexer { alias: "bash", state: None }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::using_lexer(r"(?m)^def[ \t]+[A-Za-z_][A-Za-z0-9_\-.+]*[ \t]*\([^)]*\)[ \t]*:[ \t]*\n(?:[ \t]+.*\n|\n)+", "python", None),
        Rule::token_to(r"(?m)^(inherit_defer|inherit|include_all|include|require)\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"include-line"])),
        Rule::token_to(r"(?m)^(addtask|deltask|addhandler|EXPORT_FUNCTIONS)\b", KEYWORD, NewState::Push(vec![r"statement"])),
        Rule::bygroups_to(r"(?m)^([A-Za-z_][A-Za-z0-9_\-.+]*)(\[)([A-Za-z_][A-Za-z0-9_\-.+]*)(\])([ \t]*)((?:\?\?=|\?=|:=|\+=|=\+|\.=|=\.|=))", vec![Some(NAME_VARIABLE), Some(PUNCTUATION), Some(NAME_ATTRIBUTE), Some(PUNCTUATION), Some(WHITESPACE), Some(OPERATOR)], NewState::Push(vec![r"value"])),
        Rule::bygroups_to(r"(?m)^(export[ \t]+)?([A-Za-z_][A-Za-z0-9_\-.+]*)((?::[A-Za-z0-9_\-.+${}]+)*)([ \t]*)((?:\?\?=|\?=|:=|\+=|=\+|\.=|=\.|=))", vec![Some(KEYWORD_TYPE), Some(NAME_VARIABLE), Some(NAME_DECORATOR), Some(WHITESPACE), Some(OPERATOR)], NewState::Push(vec![r"value"])),
        Rule::token(r#"(?m)[^\s#${}\[\]:=+?.@\\"\']+"#, TEXT),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"include-line", vec![
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\$\{@", STRING_INTERPOL, NewState::Push(vec![r"py-interp"])),
        Rule::bygroups(r"(?m)(\$\{)([A-Za-z0-9_\-:.+/]+)(\})", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token(r"(?m)[^\s$]+", STRING),
    ]);
    m.insert(r"interp", vec![
        Rule::token_to(r"(?m)\$\{@", STRING_INTERPOL, NewState::Push(vec![r"py-interp"])),
        Rule::bygroups(r"(?m)(\$\{)([A-Za-z0-9_\-:.+/]+)(\})", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
    ]);
    m.insert(r"statement", vec![
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)(after|before)\b", KEYWORD),
        Rule::token_to(r"(?m)\$\{@", STRING_INTERPOL, NewState::Push(vec![r"py-interp"])),
        Rule::bygroups(r"(?m)(\$\{)([A-Za-z0-9_\-:.+/]+)(\})", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token(r"(?m)[^\s$\\]+", NAME),
    ]);
    m.insert(r"value", vec![
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", STRING_ESCAPE),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string-double"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"string-single"])),
        Rule::token_to(r"(?m)\$\{@", STRING_INTERPOL, NewState::Push(vec![r"py-interp"])),
        Rule::bygroups(r"(?m)(\$\{)([A-Za-z0-9_\-:.+/]+)(\})", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token(r#"(?m)[^\s"\'$\\]+"#, STRING),
    ]);
    m.insert(r"string-double", vec![
        Rule::token(r"(?m)\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\$\{@", STRING_INTERPOL, NewState::Push(vec![r"py-interp"])),
        Rule::bygroups(r"(?m)(\$\{)([A-Za-z0-9_\-:.+/]+)(\})", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token(r#"(?m)[^"\\$]+"#, STRING_DOUBLE),
    ]);
    m.insert(r"string-single", vec![
        Rule::token(r"(?m)\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\$\{@", STRING_INTERPOL, NewState::Push(vec![r"py-interp"])),
        Rule::bygroups(r"(?m)(\$\{)([A-Za-z0-9_\-:.+/]+)(\})", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token(r"(?m)[^'\\$]+", STRING_SINGLE),
    ]);
    m.insert(r"py-interp", vec![
        Rule::token_to(r"(?m)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::using_lexer(r"(?m)[^}]+", "python", None),
    ]);
    Table(m)
}

impl Lexer for BitbakeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
