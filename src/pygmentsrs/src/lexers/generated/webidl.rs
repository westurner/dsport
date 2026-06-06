//! AUTO-GENERATED from `pygments.pygments.lexers.webidl:WebIDLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.webidl:WebIDLLexer:webidl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: webidl
pub struct WebidlLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"common", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"extended_attributes"])),
        Rule::token(r"(?m)partial(?![\w-])", KEYWORD),
        Rule::token_to(r"(?m)typedef(?![\w-])", KEYWORD, NewState::Push(vec![r"typedef", r"type"])),
        Rule::token_to(r"(?m)interface(?![\w-])", KEYWORD, NewState::Push(vec![r"interface_rest"])),
        Rule::token_to(r"(?m)enum(?![\w-])", KEYWORD, NewState::Push(vec![r"enum_rest"])),
        Rule::token_to(r"(?m)callback(?![\w-])", KEYWORD, NewState::Push(vec![r"callback_rest"])),
        Rule::token_to(r"(?m)dictionary(?![\w-])", KEYWORD, NewState::Push(vec![r"dictionary_rest"])),
        Rule::token_to(r"(?m)namespace(?![\w-])", KEYWORD, NewState::Push(vec![r"namespace_rest"])),
        Rule::token_to(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CLASS, NewState::Push(vec![r"implements_rest"])),
    ]);
    m.insert(r"extended_attributes", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_DECORATOR),
        Rule::token_to(r"(?m)=", PUNCTUATION, NewState::Push(vec![r"extended_attribute_rest"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"argument_list"])),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"extended_attribute_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME, NewState::Push(vec![r"extended_attribute_named_rest"])),
        Rule::token(r#"(?m)"[^"]*""#, STRING),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"identifier_list"])),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"extended_attribute_named_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"argument_list"])),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"argument_list", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::default(NewState::Push(vec![r"argument"])),
    ]);
    m.insert(r"argument", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)optional(?![\w-])", KEYWORD),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"extended_attributes"])),
        Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(2)),
        Rule::default(NewState::Push(vec![r"argument_rest", r"type"])),
    ]);
    m.insert(r"argument_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_VARIABLE),
        Rule::token(r"(?m)\.\.\.", PUNCTUATION),
        Rule::token_to(r"(?m)=", PUNCTUATION, NewState::Push(vec![r"default_value"])),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"identifier_list", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CLASS),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"type", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)(?:byte|octet|boolean|(?:unsigned\s+)?(?:short|long(?:\s+long)?)|(?:unrestricted\s+)?(?:float|double)|DOMString|ByteString|USVString|Error|DOMException|Uint8Array|Uint16Array|Uint32Array|Uint8ClampedArray|Float32Array|Float64Array|ArrayBuffer|DataView|Int8Array|Int16Array|Int32Array|any|void|object|RegExp)(?![\w-])", KEYWORD_TYPE, NewState::Push(vec![r"type_null"])),
        Rule::token_to(r"(?m)(FrozenArray|(?:Promis|sequenc)e)(?![\w-])", KEYWORD_TYPE, NewState::Push(vec![r"type_identifier"])),
        Rule::token_to(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CLASS, NewState::Push(vec![r"type_identifier"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"union_type"])),
    ]);
    m.insert(r"union_type", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)or(?![\w-])", KEYWORD),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Push(vec![r"#pop", r"type_null"])),
        Rule::default(NewState::Push(vec![r"type"])),
    ]);
    m.insert(r"type_identifier", vec![
        Rule::token_to(r"(?m)<", PUNCTUATION, NewState::Push(vec![r"type_list"])),
        Rule::default(NewState::Push(vec![r"#pop", r"type_null"])),
    ]);
    m.insert(r"type_null", vec![
        Rule::token(r"(?m)\?", PUNCTUATION),
        Rule::default(NewState::Pop(2)),
    ]);
    m.insert(r"default_value", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)(\-Infinity|Infinity|NaN|false|null|true)(?![\w-])", KEYWORD_CONSTANT, NewState::Pop(1)),
        Rule::token_to(r"(?m)-?(?:(?:[0-9]+\.[0-9]*|[0-9]*\.[0-9]+)(?:[Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)", NUMBER_FLOAT, NewState::Pop(1)),
        Rule::token_to(r"(?m)-?[1-9][0-9]*", NUMBER_INTEGER, NewState::Pop(1)),
        Rule::token_to(r"(?m)-?0[Xx][0-9A-Fa-f]+", NUMBER_HEX, NewState::Pop(1)),
        Rule::token_to(r"(?m)-?0[0-7]*", NUMBER_OCT, NewState::Pop(1)),
        Rule::token_to(r#"(?m)"[^"]*""#, STRING, NewState::Pop(1)),
        Rule::token_to(r"(?m)\[\s*\]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"const_value", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)(\-Infinity|Infinity|NaN|false|null|true)(?![\w-])", KEYWORD_CONSTANT, NewState::Pop(1)),
        Rule::token_to(r"(?m)-?(?:(?:[0-9]+\.[0-9]*|[0-9]*\.[0-9]+)(?:[Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)", NUMBER_FLOAT, NewState::Pop(1)),
        Rule::token_to(r"(?m)-?[1-9][0-9]*", NUMBER_INTEGER, NewState::Pop(1)),
        Rule::token_to(r"(?m)-?0[Xx][0-9A-Fa-f]+", NUMBER_HEX, NewState::Pop(1)),
        Rule::token_to(r"(?m)-?0[0-7]*", NUMBER_OCT, NewState::Pop(1)),
    ]);
    m.insert(r"typedef", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CLASS),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"namespace_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_NAMESPACE),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"namespace_body"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"namespace_body", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"extended_attributes"])),
        Rule::token(r"(?m)readonly(?![\w-])", KEYWORD),
        Rule::token_to(r"(?m)attribute(?![\w-])", KEYWORD, NewState::Push(vec![r"attribute_rest", r"type"])),
        Rule::token_to(r"(?m)const(?![\w-])", KEYWORD, NewState::Push(vec![r"const_rest", r"type"])),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::default(NewState::Push(vec![r"operation_rest", r"type"])),
    ]);
    m.insert(r"interface_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CLASS),
        Rule::token(r"(?m):", PUNCTUATION),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"interface_body"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"interface_body", vec![
        Rule::token_to(r"(?m)((?:iterabl|(?:map|set)lik)e)(?![\w-])", KEYWORD, NewState::Push(vec![r"iterable_maplike_setlike_rest"])),
        Rule::token(r"(?m)(creator|deleter|getter|inherit|jsonifier|legacycaller|s(?:etter|t(?:atic|ringifier)))(?![\w-])", KEYWORD),
        Rule::token_to(r"(?m)serializer(?![\w-])", KEYWORD, NewState::Push(vec![r"serializer_rest"])),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"extended_attributes"])),
        Rule::token(r"(?m)readonly(?![\w-])", KEYWORD),
        Rule::token_to(r"(?m)attribute(?![\w-])", KEYWORD, NewState::Push(vec![r"attribute_rest", r"type"])),
        Rule::token_to(r"(?m)const(?![\w-])", KEYWORD, NewState::Push(vec![r"const_rest", r"type"])),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::default(NewState::Push(vec![r"operation_rest", r"type"])),
    ]);
    m.insert(r"attribute_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_VARIABLE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"const_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CONSTANT),
        Rule::token_to(r"(?m)=", PUNCTUATION, NewState::Push(vec![r"const_value"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"operation_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::default(NewState::Push(vec![r"operation"])),
    ]);
    m.insert(r"operation", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_FUNCTION),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"argument_list"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(2)),
    ]);
    m.insert(r"iterable_maplike_setlike_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)<", PUNCTUATION, NewState::Push(vec![r"type_list"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"type_list", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token_to(r"(?m)>", PUNCTUATION, NewState::Pop(1)),
        Rule::default(NewState::Push(vec![r"type"])),
    ]);
    m.insert(r"serializer_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)=", PUNCTUATION, NewState::Push(vec![r"serialization_pattern"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::default(NewState::Push(vec![r"operation"])),
    ]);
    m.insert(r"serialization_pattern", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_VARIABLE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"serialization_pattern_map"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"serialization_pattern_list"])),
    ]);
    m.insert(r"serialization_pattern_map", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)(attribute|getter|inherit)(?![\w-])", KEYWORD),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_VARIABLE),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(2)),
    ]);
    m.insert(r"serialization_pattern_list", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)(attribute|getter)(?![\w-])", KEYWORD),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_VARIABLE),
        Rule::token_to(r"(?m)]", PUNCTUATION, NewState::Pop(2)),
    ]);
    m.insert(r"enum_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CLASS),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"enum_body"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"enum_body", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r#"(?m)"[^"]*""#, STRING),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"callback_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)interface(?![\w-])", KEYWORD, NewState::Push(vec![r"#pop", r"interface_rest"])),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CLASS),
        Rule::token_to(r"(?m)=", PUNCTUATION, NewState::Push(vec![r"operation", r"type"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"dictionary_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CLASS),
        Rule::token(r"(?m):", PUNCTUATION),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"dictionary_body"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"dictionary_body", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"extended_attributes"])),
        Rule::token(r"(?m)required(?![\w-])", KEYWORD),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::default(NewState::Push(vec![r"dictionary_item", r"type"])),
    ]);
    m.insert(r"dictionary_item", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_VARIABLE),
        Rule::token_to(r"(?m)=", PUNCTUATION, NewState::Push(vec![r"default_value"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"implements_rest", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)^#.*", COMMENT_PREPROC),
        Rule::token(r"(?m)implements(?![\w-])", KEYWORD),
        Rule::token(r"(?m)_?[A-Za-z][a-zA-Z0-9_-]*", NAME_CLASS),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for WebidlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
