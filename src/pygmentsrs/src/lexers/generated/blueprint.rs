//! AUTO-GENERATED from `pygments.pygments.lexers.blueprint:BlueprintLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.blueprint:BlueprintLexer:blueprint

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: blueprint
pub struct BlueprintLexer;

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
        Rule::bygroups(r"(?im)(using)(\s+)([a-z_][a-z0-9_\-]*)(\s+)(\d[\d\.]*)(;)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?im)(menu|section|submenu)(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-block"])),
        Rule::token_to(r"(?im)template", KEYWORD_DECLARATION, NewState::Push(vec![r"template"])),
        Rule::bygroups_to(r"(?im)(responses|items|mime-types|patterns|suffixes|marks|widgets|strings|styles)(\s*)(\[)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"bracket-block"])),
        Rule::bygroups_to(r"(?im)(accessibility|setters|layout|item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(condition|mark|item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::token_to(r"(?im)\[", PUNCTUATION, NewState::Push(vec![r"child-type"])),
        Rule::bygroups_to(r"(?im)([a-z_][a-z0-9_\-]*(?:::[a-z0-9_]+)?)(\s*)(:|=>)", vec![Some(NAME_PROPERTY), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"statement"])),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)(after|bi(?:directional|nd(?:(?:\-property)?))|d(?:e(?:fault|structive)|isabled)|inverted|no\-sync\-create|s(?:uggested|wapped|ync\-create)|template)", KEYWORD),
        Rule::bygroups_to(r"(?im)(C?_)(\s*)(\()", vec![Some(TokenType::new(&["Name", "Function", "Builtin"])), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(as)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::bygroups_to(r"(?im)(\$?[a-z_][a-z0-9_\-]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(?:(\$\s*[a-z_][a-z0-9_\-]+)|(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*))(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(NAME_CLASS), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
        Rule::token(r"(?im),|\.", PUNCTUATION),
    ]);
    m.insert(r"block-content", vec![
        Rule::bygroups(r"(?im)(using)(\s+)([a-z_][a-z0-9_\-]*)(\s+)(\d[\d\.]*)(;)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?im)(menu|section|submenu)(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-block"])),
        Rule::token_to(r"(?im)template", KEYWORD_DECLARATION, NewState::Push(vec![r"template"])),
        Rule::bygroups_to(r"(?im)(responses|items|mime-types|patterns|suffixes|marks|widgets|strings|styles)(\s*)(\[)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"bracket-block"])),
        Rule::bygroups_to(r"(?im)(accessibility|setters|layout|item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(condition|mark|item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::token_to(r"(?im)\[", PUNCTUATION, NewState::Push(vec![r"child-type"])),
        Rule::bygroups_to(r"(?im)([a-z_][a-z0-9_\-]*(?:::[a-z0-9_]+)?)(\s*)(:|=>)", vec![Some(NAME_PROPERTY), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"statement"])),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)(after|bi(?:directional|nd(?:(?:\-property)?))|d(?:e(?:fault|structive)|isabled)|inverted|no\-sync\-create|s(?:uggested|wapped|ync\-create)|template)", KEYWORD),
        Rule::bygroups_to(r"(?im)(C?_)(\s*)(\()", vec![Some(TokenType::new(&["Name", "Function", "Builtin"])), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(as)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::bygroups_to(r"(?im)(\$?[a-z_][a-z0-9_\-]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(?:(\$\s*[a-z_][a-z0-9_\-]+)|(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*))(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(NAME_CLASS), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
        Rule::token(r"(?im),|\.", PUNCTUATION),
    ]);
    m.insert(r"content", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)(after|bi(?:directional|nd(?:(?:\-property)?))|d(?:e(?:fault|structive)|isabled)|inverted|no\-sync\-create|s(?:uggested|wapped|ync\-create)|template)", KEYWORD),
        Rule::bygroups_to(r"(?im)(C?_)(\s*)(\()", vec![Some(TokenType::new(&["Name", "Function", "Builtin"])), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(as)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::bygroups_to(r"(?im)(\$?[a-z_][a-z0-9_\-]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(?:(\$\s*[a-z_][a-z0-9_\-]+)|(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*))(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(NAME_CLASS), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
        Rule::token(r"(?im),|\.", PUNCTUATION),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
    ]);
    m.insert(r"value", vec![
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
    ]);
    m.insert(r"type", vec![
        Rule::token(r"(?im)\$\s*[a-z_][a-z0-9_\-]*", NAME_CLASS),
        Rule::bygroups(r"(?im)(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*)", vec![Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)]),
    ]);
    m.insert(r"comment-multiline", vec![
        Rule::token_to(r"(?im)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?im)[^*]+", COMMENT_MULTILINE),
        Rule::token(r"(?im)\*", COMMENT_MULTILINE),
    ]);
    m.insert(r"typeof", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)\$\s*[a-z_][a-z0-9_\-]*", NAME_CLASS),
        Rule::bygroups(r"(?im)(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*)", vec![Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::token_to(r"(?im)>", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"paren-block", vec![
        Rule::bygroups(r"(?im)(using)(\s+)([a-z_][a-z0-9_\-]*)(\s+)(\d[\d\.]*)(;)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?im)(menu|section|submenu)(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-block"])),
        Rule::token_to(r"(?im)template", KEYWORD_DECLARATION, NewState::Push(vec![r"template"])),
        Rule::bygroups_to(r"(?im)(responses|items|mime-types|patterns|suffixes|marks|widgets|strings|styles)(\s*)(\[)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"bracket-block"])),
        Rule::bygroups_to(r"(?im)(accessibility|setters|layout|item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(condition|mark|item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::token_to(r"(?im)\[", PUNCTUATION, NewState::Push(vec![r"child-type"])),
        Rule::bygroups_to(r"(?im)([a-z_][a-z0-9_\-]*(?:::[a-z0-9_]+)?)(\s*)(:|=>)", vec![Some(NAME_PROPERTY), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"statement"])),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)(after|bi(?:directional|nd(?:(?:\-property)?))|d(?:e(?:fault|structive)|isabled)|inverted|no\-sync\-create|s(?:uggested|wapped|ync\-create)|template)", KEYWORD),
        Rule::bygroups_to(r"(?im)(C?_)(\s*)(\()", vec![Some(TokenType::new(&["Name", "Function", "Builtin"])), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(as)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::bygroups_to(r"(?im)(\$?[a-z_][a-z0-9_\-]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(?:(\$\s*[a-z_][a-z0-9_\-]+)|(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*))(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(NAME_CLASS), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
        Rule::token(r"(?im),|\.", PUNCTUATION),
        Rule::token_to(r"(?im)\)", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"paren-content", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)(after|bi(?:directional|nd(?:(?:\-property)?))|d(?:e(?:fault|structive)|isabled)|inverted|no\-sync\-create|s(?:uggested|wapped|ync\-create)|template)", KEYWORD),
        Rule::bygroups_to(r"(?im)(C?_)(\s*)(\()", vec![Some(TokenType::new(&["Name", "Function", "Builtin"])), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(as)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::bygroups_to(r"(?im)(\$?[a-z_][a-z0-9_\-]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(?:(\$\s*[a-z_][a-z0-9_\-]+)|(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*))(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(NAME_CLASS), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
        Rule::token(r"(?im),|\.", PUNCTUATION),
        Rule::token_to(r"(?im)\)", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"bracket-block", vec![
        Rule::bygroups(r"(?im)(using)(\s+)([a-z_][a-z0-9_\-]*)(\s+)(\d[\d\.]*)(;)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?im)(menu|section|submenu)(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-block"])),
        Rule::token_to(r"(?im)template", KEYWORD_DECLARATION, NewState::Push(vec![r"template"])),
        Rule::bygroups_to(r"(?im)(responses|items|mime-types|patterns|suffixes|marks|widgets|strings|styles)(\s*)(\[)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"bracket-block"])),
        Rule::bygroups_to(r"(?im)(accessibility|setters|layout|item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(condition|mark|item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::token_to(r"(?im)\[", PUNCTUATION, NewState::Push(vec![r"child-type"])),
        Rule::bygroups_to(r"(?im)([a-z_][a-z0-9_\-]*(?:::[a-z0-9_]+)?)(\s*)(:|=>)", vec![Some(NAME_PROPERTY), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"statement"])),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)(after|bi(?:directional|nd(?:(?:\-property)?))|d(?:e(?:fault|structive)|isabled)|inverted|no\-sync\-create|s(?:uggested|wapped|ync\-create)|template)", KEYWORD),
        Rule::bygroups_to(r"(?im)(C?_)(\s*)(\()", vec![Some(TokenType::new(&["Name", "Function", "Builtin"])), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(as)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::bygroups_to(r"(?im)(\$?[a-z_][a-z0-9_\-]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(?:(\$\s*[a-z_][a-z0-9_\-]+)|(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*))(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(NAME_CLASS), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
        Rule::token(r"(?im),|\.", PUNCTUATION),
        Rule::token_to(r"(?im)\]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"brace-block", vec![
        Rule::bygroups(r"(?im)(using)(\s+)([a-z_][a-z0-9_\-]*)(\s+)(\d[\d\.]*)(;)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?im)(menu|section|submenu)(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-block"])),
        Rule::token_to(r"(?im)template", KEYWORD_DECLARATION, NewState::Push(vec![r"template"])),
        Rule::bygroups_to(r"(?im)(responses|items|mime-types|patterns|suffixes|marks|widgets|strings|styles)(\s*)(\[)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"bracket-block"])),
        Rule::bygroups_to(r"(?im)(accessibility|setters|layout|item)(\s*)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(condition|mark|item)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::token_to(r"(?im)\[", PUNCTUATION, NewState::Push(vec![r"child-type"])),
        Rule::bygroups_to(r"(?im)([a-z_][a-z0-9_\-]*(?:::[a-z0-9_]+)?)(\s*)(:|=>)", vec![Some(NAME_PROPERTY), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"statement"])),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)(after|bi(?:directional|nd(?:(?:\-property)?))|d(?:e(?:fault|structive)|isabled)|inverted|no\-sync\-create|s(?:uggested|wapped|ync\-create)|template)", KEYWORD),
        Rule::bygroups_to(r"(?im)(C?_)(\s*)(\()", vec![Some(TokenType::new(&["Name", "Function", "Builtin"])), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(as)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::bygroups_to(r"(?im)(\$?[a-z_][a-z0-9_\-]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(?:(\$\s*[a-z_][a-z0-9_\-]+)|(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*))(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(NAME_CLASS), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
        Rule::token(r"(?im),|\.", PUNCTUATION),
        Rule::token_to(r"(?im)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"statement", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)(after|bi(?:directional|nd(?:(?:\-property)?))|d(?:e(?:fault|structive)|isabled)|inverted|no\-sync\-create|s(?:uggested|wapped|ync\-create)|template)", KEYWORD),
        Rule::bygroups_to(r"(?im)(C?_)(\s*)(\()", vec![Some(TokenType::new(&["Name", "Function", "Builtin"])), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(as)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::bygroups_to(r"(?im)(\$?[a-z_][a-z0-9_\-]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"paren-content"])),
        Rule::bygroups_to(r"(?im)(?:(\$\s*[a-z_][a-z0-9_\-]+)|(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*))(?:(\s+)([a-z_][a-z0-9_\-]*))?(\s*)(\{)", vec![Some(NAME_CLASS), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"brace-block"])),
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
        Rule::token(r"(?im),|\.", PUNCTUATION),
        Rule::token_to(r"(?im);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"child-type", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::bygroups(r"(?im)(action)(\s+)(response)(\s*)(=)(\s*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::token(r"(?im)(default|internal\-child|response)", KEYWORD),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_DECORATOR),
        Rule::bygroups_to(r"(?im)(typeof)(\s*)(<)", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"typeof"])),
        Rule::token(r"(?im)(false|null|true)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)[a-z_][a-z0-9_\-]*", NAME_VARIABLE),
        Rule::token(r"(?im)\|", OPERATOR),
        Rule::token(r#"(?im)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?im)\'.*?\'", STRING_SINGLE),
        Rule::token(r"(?im)0x[\d_]*", NUMBER_HEX),
        Rule::token(r"(?im)[0-9_]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\d[\d\.a-z_]*", NUMBER),
        Rule::token(r"(?im)=", PUNCTUATION),
        Rule::token_to(r"(?im)\]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"template", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multiline"])),
        Rule::token(r"(?im)\$\s*[a-z_][a-z0-9_\-]*", NAME_CLASS),
        Rule::bygroups(r"(?im)(?:([a-z_][a-z0-9_\-]*)(\s*)(\.)(\s*))?([a-z_][a-z0-9_\-]*)", vec![Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::token(r"(?im):", PUNCTUATION),
        Rule::token_to(r"(?im)\{", PUNCTUATION, NewState::Push(vec![r"#pop", r"brace-block"])),
    ]);
    Table(m)
}

impl Lexer for BlueprintLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
