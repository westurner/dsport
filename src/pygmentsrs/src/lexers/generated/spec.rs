//! AUTO-GENERATED from `pygments.pygments.lexers.installers:RPMSpecLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.installers:RPMSpecLexer:spec

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: spec
pub struct SpecLexer;

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
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m)%define.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)%\{\!\?.*%define.*\}", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)(%(?:if(?:n?arch)?|else(?:if)?|endif))(.*)$", vec![Some(COMMENT_PREPROC), Some(TEXT)]),
        Rule::bygroups_g(r"(?m)(?i)^(Name|Version|Release|Epoch|Summary|Group|License|Packager|Vendor|Icon|URL|Distribution|Prefix|Patch[0-9]*|Source[0-9]*|Requires\(?[a-z]*\)?|[a-z]+Req|Obsoletes|Suggests|Provides|Conflicts|Build[a-z]+|[a-z]+Arch|Auto[a-z]+)(:)(.*)$", vec![Some(GroupAction::Token(GENERIC_HEADING)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: None })]),
        Rule::token_to(r"(?m)^%description", NAME_DECORATOR, NewState::Push(vec![r"description"])),
        Rule::token_to(r"(?m)^%changelog", NAME_DECORATOR, NewState::Push(vec![r"changelog"])),
        Rule::bygroups(r"(?m)^(%(?:package|prep|build|install|clean|check|pre[a-z]*|post[a-z]*|trigger[a-z]*|files))(.*)$", vec![Some(NAME_DECORATOR), Some(TEXT)]),
        Rule::token(r"(?m)%(attr|defattr|dir|doc(?:dir)?|setup|config(?:ure)?|make(?:install)|ghost|patch[0-9]+|find_lang|exclude|verify)", KEYWORD),
        Rule::token(r"(?m)%\{?__[a-z_]+\}?", NAME_FUNCTION),
        Rule::token(r"(?m)%\{?_([a-z_]+dir|[a-z_]+path|prefix)\}?", KEYWORD_PSEUDO),
        Rule::token(r"(?m)%\{\?\w+\}", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{?RPM_[A-Z0-9_]+\}?", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)%\{[a-zA-Z]\w+\}", KEYWORD_CONSTANT),
        Rule::token(r"(?m)'.*?'", STRING_SINGLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"basic", vec![
        Rule::token(r"(?m)%define.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)%\{\!\?.*%define.*\}", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)(%(?:if(?:n?arch)?|else(?:if)?|endif))(.*)$", vec![Some(COMMENT_PREPROC), Some(TEXT)]),
        Rule::bygroups_g(r"(?m)(?i)^(Name|Version|Release|Epoch|Summary|Group|License|Packager|Vendor|Icon|URL|Distribution|Prefix|Patch[0-9]*|Source[0-9]*|Requires\(?[a-z]*\)?|[a-z]+Req|Obsoletes|Suggests|Provides|Conflicts|Build[a-z]+|[a-z]+Arch|Auto[a-z]+)(:)(.*)$", vec![Some(GroupAction::Token(GENERIC_HEADING)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: None })]),
        Rule::token_to(r"(?m)^%description", NAME_DECORATOR, NewState::Push(vec![r"description"])),
        Rule::token_to(r"(?m)^%changelog", NAME_DECORATOR, NewState::Push(vec![r"changelog"])),
        Rule::bygroups(r"(?m)^(%(?:package|prep|build|install|clean|check|pre[a-z]*|post[a-z]*|trigger[a-z]*|files))(.*)$", vec![Some(NAME_DECORATOR), Some(TEXT)]),
        Rule::token(r"(?m)%(attr|defattr|dir|doc(?:dir)?|setup|config(?:ure)?|make(?:install)|ghost|patch[0-9]+|find_lang|exclude|verify)", KEYWORD),
        Rule::token(r"(?m)%\{?__[a-z_]+\}?", NAME_FUNCTION),
        Rule::token(r"(?m)%\{?_([a-z_]+dir|[a-z_]+path|prefix)\}?", KEYWORD_PSEUDO),
        Rule::token(r"(?m)%\{\?\w+\}", NAME_VARIABLE),
        Rule::token(r"(?m)\$\{?RPM_[A-Z0-9_]+\}?", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)%\{[a-zA-Z]\w+\}", KEYWORD_CONSTANT),
        Rule::token(r"(?m)'.*?'", STRING_SINGLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(
        r"macro",
        vec![
            Rule::token(r"(?m)%define.*$", COMMENT_PREPROC),
            Rule::token(r"(?m)%\{\!\?.*%define.*\}", COMMENT_PREPROC),
            Rule::bygroups(
                r"(?m)(%(?:if(?:n?arch)?|else(?:if)?|endif))(.*)$",
                vec![Some(COMMENT_PREPROC), Some(TEXT)],
            ),
        ],
    );
    m.insert(
        r"interpol",
        vec![
            Rule::token(r"(?m)%\{?__[a-z_]+\}?", NAME_FUNCTION),
            Rule::token(
                r"(?m)%\{?_([a-z_]+dir|[a-z_]+path|prefix)\}?",
                KEYWORD_PSEUDO,
            ),
            Rule::token(r"(?m)%\{\?\w+\}", NAME_VARIABLE),
            Rule::token(r"(?m)\$\{?RPM_[A-Z0-9_]+\}?", NAME_VARIABLE_GLOBAL),
            Rule::token(r"(?m)%\{[a-zA-Z]\w+\}", KEYWORD_CONSTANT),
        ],
    );
    m.insert(r"description", vec![
        Rule::bygroups_to(r"(?m)^(%(?:package|prep|build|install|clean|check|pre[a-z]*|post[a-z]*|trigger[a-z]*|files))(.*)$", vec![Some(NAME_DECORATOR), Some(TEXT)], NewState::Pop(1)),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"changelog", vec![
        Rule::token(r"(?m)\*.*$", GENERIC_SUBHEADING),
        Rule::bygroups_to(r"(?m)^(%(?:package|prep|build|install|clean|check|pre[a-z]*|post[a-z]*|trigger[a-z]*|files))(.*)$", vec![Some(NAME_DECORATOR), Some(TEXT)], NewState::Pop(1)),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|[0-7]{1,3})"#,
                STRING_ESCAPE,
            ),
            Rule::token(r"(?m)%\{?__[a-z_]+\}?", NAME_FUNCTION),
            Rule::token(
                r"(?m)%\{?_([a-z_]+dir|[a-z_]+path|prefix)\}?",
                KEYWORD_PSEUDO,
            ),
            Rule::token(r"(?m)%\{\?\w+\}", NAME_VARIABLE),
            Rule::token(r"(?m)\$\{?RPM_[A-Z0-9_]+\}?", NAME_VARIABLE_GLOBAL),
            Rule::token(r"(?m)%\{[a-zA-Z]\w+\}", KEYWORD_CONSTANT),
            Rule::token(r"(?m).", STRING_DOUBLE),
        ],
    );
    Table(m)
}

impl Lexer for SpecLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
