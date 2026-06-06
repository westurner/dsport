//! AUTO-GENERATED from `pygments.pygments.lexers.nix:NixLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.nix:NixLexer:nixos

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: nixos, nix
pub struct NixosLexer;

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
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(rec\b|with\b|let\b|in\b|inherit\b|assert\b|if\b|else\b|then\b|\.\.\.\b)", KEYWORD),
        Rule::token(r"(?m)(import\b|abort\b|baseNameOf\b|dirOf\b|isNull\b|builtins\b|map\b|removeAttrs\b|throw\b|toString\b|derivation\b)", NAME_BUILTIN),
        Rule::token(r"(?m)\b(true|false|null)\b", NAME_CONSTANT),
        Rule::token(r"(?m)-?(\d+\.\d*|\.\d+)([eE][-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)[\w.+-]*(\/[\w.+-]+)+", LITERAL),
        Rule::token(r"(?m)~(\/[\w.+-]+)+", LITERAL),
        Rule::token(r"(?m)\<[\w.+-]+(\/[\w.+-]+)*\>", LITERAL),
        Rule::token(r"(?m)(\+\+|\+|\?|\.|!|//|==|/|!=|\&\&|\|\||\->|=|<|>|\*|\-)", OPERATOR),
        Rule::token(r"(?m)\b(or|and)\b", OPERATOR_WORD),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"block"])),
        Rule::token(r"(?m)(\(|\)|\[|\]|;|\{|\}|:|,|@)", PUNCTUATION),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"doublequote"])),
        Rule::token_to(r"(?m)''", TokenType::new(&["Literal", "String", "Multiline"]), NewState::Push(vec![r"multiline"])),
        Rule::token(r"(?m)[a-zA-Z][a-zA-Z0-9\+\-\.]*\:[\w%/?:@&=+$,\\.!~*\'-]+", LITERAL),
        Rule::token(r"(?m)[\w-]+(?=\s*=)", STRING_SYMBOL),
        Rule::token(r"(?m)[a-zA-Z_][\w\'-]*", TEXT),
        Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"antiquote"])),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)[^/*]+", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
        Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
    ]);
    m.insert(r"multiline", vec![
        Rule::token(r"(?m)''(\$|'|\\n|\\r|\\t|\\)", STRING_ESCAPE),
        Rule::token_to(r"(?m)''", TokenType::new(&["Literal", "String", "Multiline"]), NewState::Pop(1)),
        Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"antiquote"])),
        Rule::token(r"(?m)[^'\$]+", TokenType::new(&["Literal", "String", "Multiline"])),
        Rule::token(r"(?m)\$[^\{']", TokenType::new(&["Literal", "String", "Multiline"])),
        Rule::token(r"(?m)'[^']", TokenType::new(&["Literal", "String", "Multiline"])),
        Rule::token(r"(?m)\$(?=')", TokenType::new(&["Literal", "String", "Multiline"])),
    ]);
    m.insert(r"doublequote", vec![
        Rule::token(r#"(?m)\\(\\|"|\$|n)"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"antiquote"])),
        Rule::token(r#"(?m)[^"\\\$]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)\$[^\{"]"#, STRING_DOUBLE),
        Rule::token(r#"(?m)\$(?=")"#, STRING_DOUBLE),
        Rule::token(r"(?m)\\", STRING_DOUBLE),
    ]);
    m.insert(r"antiquote", vec![
        Rule::token_to(r"(?m)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::PushSame),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(rec\b|with\b|let\b|in\b|inherit\b|assert\b|if\b|else\b|then\b|\.\.\.\b)", KEYWORD),
        Rule::token(r"(?m)(import\b|abort\b|baseNameOf\b|dirOf\b|isNull\b|builtins\b|map\b|removeAttrs\b|throw\b|toString\b|derivation\b)", NAME_BUILTIN),
        Rule::token(r"(?m)\b(true|false|null)\b", NAME_CONSTANT),
        Rule::token(r"(?m)-?(\d+\.\d*|\.\d+)([eE][-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)[\w.+-]*(\/[\w.+-]+)+", LITERAL),
        Rule::token(r"(?m)~(\/[\w.+-]+)+", LITERAL),
        Rule::token(r"(?m)\<[\w.+-]+(\/[\w.+-]+)*\>", LITERAL),
        Rule::token(r"(?m)(\+\+|\+|\?|\.|!|//|==|/|!=|\&\&|\|\||\->|=|<|>|\*|\-)", OPERATOR),
        Rule::token(r"(?m)\b(or|and)\b", OPERATOR_WORD),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"block"])),
        Rule::token(r"(?m)(\(|\)|\[|\]|;|\{|\}|:|,|@)", PUNCTUATION),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"doublequote"])),
        Rule::token_to(r"(?m)''", TokenType::new(&["Literal", "String", "Multiline"]), NewState::Push(vec![r"multiline"])),
        Rule::token(r"(?m)[a-zA-Z][a-zA-Z0-9\+\-\.]*\:[\w%/?:@&=+$,\\.!~*\'-]+", LITERAL),
        Rule::token(r"(?m)[\w-]+(?=\s*=)", STRING_SYMBOL),
        Rule::token(r"(?m)[a-zA-Z_][\w\'-]*", TEXT),
        Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"antiquote"])),
    ]);
    m.insert(r"block", vec![
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(rec\b|with\b|let\b|in\b|inherit\b|assert\b|if\b|else\b|then\b|\.\.\.\b)", KEYWORD),
        Rule::token(r"(?m)(import\b|abort\b|baseNameOf\b|dirOf\b|isNull\b|builtins\b|map\b|removeAttrs\b|throw\b|toString\b|derivation\b)", NAME_BUILTIN),
        Rule::token(r"(?m)\b(true|false|null)\b", NAME_CONSTANT),
        Rule::token(r"(?m)-?(\d+\.\d*|\.\d+)([eE][-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)[\w.+-]*(\/[\w.+-]+)+", LITERAL),
        Rule::token(r"(?m)~(\/[\w.+-]+)+", LITERAL),
        Rule::token(r"(?m)\<[\w.+-]+(\/[\w.+-]+)*\>", LITERAL),
        Rule::token(r"(?m)(\+\+|\+|\?|\.|!|//|==|/|!=|\&\&|\|\||\->|=|<|>|\*|\-)", OPERATOR),
        Rule::token(r"(?m)\b(or|and)\b", OPERATOR_WORD),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"block"])),
        Rule::token(r"(?m)(\(|\)|\[|\]|;|\{|\}|:|,|@)", PUNCTUATION),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"doublequote"])),
        Rule::token_to(r"(?m)''", TokenType::new(&["Literal", "String", "Multiline"]), NewState::Push(vec![r"multiline"])),
        Rule::token(r"(?m)[a-zA-Z][a-zA-Z0-9\+\-\.]*\:[\w%/?:@&=+$,\\.!~*\'-]+", LITERAL),
        Rule::token(r"(?m)[\w-]+(?=\s*=)", STRING_SYMBOL),
        Rule::token(r"(?m)[a-zA-Z_][\w\'-]*", TEXT),
        Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"antiquote"])),
    ]);
    Table(m)
}

impl Lexer for NixosLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
