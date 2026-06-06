//! AUTO-GENERATED from `pygments.pygments.lexers.graph:CypherLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.graph:CypherLexer:cypher

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cypher
pub struct CypherLexer;

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
        Rule::bygroups(r"(?im)(create)(\s+)(index|unique)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(drop)(\s+)(contraint|index)(\s+)(on)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(ends)(\s+)(with)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(is)(\s+)(node)(\s+)(key)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(is)(\s+)(null|unique)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(load)(\s+)(csv)(\s+)(from)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(on)(\s+)(match|create)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(optional)(\s+)(match)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(order)(\s+)(by)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(starts)(\s+)(with)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(union)(\s+)(all)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(using)(\s+)(periodic)(\s+)(commit)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(using)(\s+)(index)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(using)(\s+)(range|text|point)(\s+)(index)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?im)(a(?:ll|ny|s(?:(?:c(?:(?:ending)?)|sert)?))|c(?:a(?:ll|se)|ollect|reate)|d(?:e(?:lete|sc(?:(?:ending)?))|istinct)|end|f(?:ieldterminator|oreach)|in|limit|m(?:atch|erge)|n(?:o(?:ne|t)|ull)|re(?:move|turn)|s(?:et|ingle|kip|tart)|then|un(?:ion|wind)|w(?:he(?:n|re)|ith)|yield)\b", KEYWORD),
        Rule::token(r"(?im)(create|order|match|limit|set|skip|start|return|with|where|delete|foreach|not|by|true|false)\b", KEYWORD),
        Rule::bygroups_g(r"(?im)(-\[)(.*?)(\]->)", vec![Some(GroupAction::Token(OPERATOR)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(OPERATOR))]),
        Rule::bygroups_g(r"(?im)(<-\[)(.*?)(\]-)", vec![Some(GroupAction::Token(OPERATOR)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(OPERATOR))]),
        Rule::bygroups_g(r"(?im)(-\[)(.*?)(\]-)", vec![Some(GroupAction::Token(OPERATOR)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(OPERATOR))]),
        Rule::token(r"(?im)-->|<--|\[|\]", OPERATOR),
        Rule::token(r"(?im)<|>|<>|=|<=|=>|\(|\)|\||:|,|;", PUNCTUATION),
        Rule::token(r"(?im)[.*{}]", PUNCTUATION),
        Rule::token(r#"(?im)([\'"])(?:\\[tbnrf\'"\\]|[^\\])*?\1"#, STRING),
        Rule::token(r"(?im)`(?:``|[^`])+`", NAME_VARIABLE),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)[a-z]\w*", NAME),
        Rule::token(r"(?im)\d+", NUMBER),
        Rule::token(r"(?im)//.*$", COMMENT_SINGLE),
    ]);
    m.insert(r"clauses", vec![
        Rule::bygroups(r"(?im)(create)(\s+)(index|unique)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(drop)(\s+)(contraint|index)(\s+)(on)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(ends)(\s+)(with)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(is)(\s+)(node)(\s+)(key)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(is)(\s+)(null|unique)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(load)(\s+)(csv)(\s+)(from)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(on)(\s+)(match|create)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(optional)(\s+)(match)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(order)(\s+)(by)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(starts)(\s+)(with)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(union)(\s+)(all)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(using)(\s+)(periodic)(\s+)(commit)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(using)(\s+)(index)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(using)(\s+)(range|text|point)(\s+)(index)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?im)(a(?:ll|ny|s(?:(?:c(?:(?:ending)?)|sert)?))|c(?:a(?:ll|se)|ollect|reate)|d(?:e(?:lete|sc(?:(?:ending)?))|istinct)|end|f(?:ieldterminator|oreach)|in|limit|m(?:atch|erge)|n(?:o(?:ne|t)|ull)|re(?:move|turn)|s(?:et|ingle|kip|tart)|then|un(?:ion|wind)|w(?:he(?:n|re)|ith)|yield)\b", KEYWORD),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?im)(create|order|match|limit|set|skip|start|return|with|where|delete|foreach|not|by|true|false)\b", KEYWORD),
    ]);
    m.insert(r"relations", vec![
        Rule::bygroups_g(r"(?im)(-\[)(.*?)(\]->)", vec![Some(GroupAction::Token(OPERATOR)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(OPERATOR))]),
        Rule::bygroups_g(r"(?im)(<-\[)(.*?)(\]-)", vec![Some(GroupAction::Token(OPERATOR)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(OPERATOR))]),
        Rule::bygroups_g(r"(?im)(-\[)(.*?)(\]-)", vec![Some(GroupAction::Token(OPERATOR)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(OPERATOR))]),
        Rule::token(r"(?im)-->|<--|\[|\]", OPERATOR),
        Rule::token(r"(?im)<|>|<>|=|<=|=>|\(|\)|\||:|,|;", PUNCTUATION),
        Rule::token(r"(?im)[.*{}]", PUNCTUATION),
    ]);
    m.insert(r"strings", vec![
        Rule::token(r#"(?im)([\'"])(?:\\[tbnrf\'"\\]|[^\\])*?\1"#, STRING),
        Rule::token(r"(?im)`(?:``|[^`])+`", NAME_VARIABLE),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
    ]);
    m.insert(r"barewords", vec![
        Rule::token(r"(?im)[a-z]\w*", NAME),
        Rule::token(r"(?im)\d+", NUMBER),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?im)//.*$", COMMENT_SINGLE),
    ]);
    Table(m)
}

impl Lexer for CypherLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
