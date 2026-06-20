#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.c_like:ArduinoLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.c_like:ArduinoLexer:arduino

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: arduino
pub struct ArduinoLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"statements", vec![
        Rule::bygroups(r#"(?m)((?:[LuU]|u8)?R)(")([^\\()\s]{,16})(\()((?:.|\n)*?)(\)\3)(")"#, vec![Some(STRING_AFFIX), Some(STRING), Some(STRING_DELIMITER), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(STRING)]),
        Rule::bygroups_to(r"(?m)(class|concept|typename)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token(r"(?m)(__restrict|and(?:(?:_eq)?)|bit(?:and|or)|c(?:atch|lass|o(?:_(?:await|return|yield)|mpl|n(?:cept|st(?:_cast|eval|init)|tract_assert)))|d(?:e(?:(?:cltyp|let)e)|ynamic_cast)|exp(?:(?:lici|or)t)|f(?:inal|riend)|import|m(?:(?:odu|utab)le)|n(?:ew|o(?:except|t(?:(?:_eq)?)))|o(?:perator|r(?:(?:_eq)?)|verride)|p(?:ost|r(?:e|ivate|otected)|ublic)|re(?:interpret_cast|quires)|static_cast|t(?:emplate|h(?:is|row(?:(?:s)?))|ry|ype(?:id|name))|using|virtual|xor(?:(?:_eq)?))\b", KEYWORD),
        Rule::token_to(r"(?m)namespace\b", KEYWORD, NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(enum)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"enumname"])),
        Rule::bygroups_to(r"(?m)(struct|union)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token_to(r"(?m)case\b", KEYWORD, NewState::Push(vec![r"case-value"])),
        Rule::token(r"(?m)(_Pragma|a(?:lign(?:as|of)|sm|uto)|break|con(?:st(?:(?:expr)?)|tinue)|d(?:efault|o)|e(?:lse|num|xtern)|for(?:(?:tran)?)|goto|if|re(?:gister|stricted|turn)|s(?:izeof|t(?:atic(?:(?:_assert)?)|ruct)|witch)|t(?:hread_local|ype(?:def|of(?:(?:_unqual)?)))|union|(?:volat|wh)ile)\b", KEYWORD),
        Rule::token(r"(?m)(_(?:(?:(?:_)?)inline)|inline|naked|restrict|thread)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(__m(128i|128d|128|64))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)__(as(?:m|sume)|based|cdecl|declspec|except|f(?:astcall|inally|orceinline)|identifier|leave|n(?:oop|ull)|raise|stdcall|try|unaligned|w64)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)char(16_t|32_t|8_t)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)__(int(?:16|32|64|8)|wchar_t)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_(?:BitInt|_int128)|bool|char|double|float|int|long|s(?:hort|igned)|(?:unsigne|voi)d)\b", KEYWORD_TYPE),
        Rule::bygroups_to(r#"(?m)([LuU]|u8)?(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?m)([LuU]|u8)?(')(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])(')", vec![Some(STRING_AFFIX), Some(STRING_CHAR), Some(STRING_CHAR), Some(STRING_CHAR)]),
        Rule::token(r"(?m)0[xX]([0-9a-fA-F](\'?[0-9a-fA-F])*\.[0-9a-fA-F](\'?[0-9a-fA-F])*|\.[0-9a-fA-F](\'?[0-9a-fA-F])*|[0-9a-fA-F](\'?[0-9a-fA-F])*)[pP][+-]?[0-9a-fA-F](\'?[0-9a-fA-F])*[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?(\d(\'?\d)*\.\d(\'?\d)*|\.\d(\'?\d)*|\d(\'?\d)*)[eE][+-]?\d(\'?\d)*[fFlL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?((\d(\'?\d)*\.(\d(\'?\d)*)?|\.\d(\'?\d)*)[fFlL]?)|(\d(\'?\d)*[fFlL])", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?0[xX][0-9a-fA-F](\'?[0-9a-fA-F])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_HEX),
        Rule::token(r"(?m)(-)?0[bB][01](\'?[01])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_BIN),
        Rule::token(r"(?m)(-)?0(\'?[0-7])+(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_OCT),
        Rule::token(r"(?m)(-)?\d(\'?\d)*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::token(r"(?m)(true|false|NULL|nullptr)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+", NAME),
    ]);
    m.insert(r"keywords", vec![
        Rule::bygroups_to(r"(?m)(class|concept|typename)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token(r"(?m)(__restrict|and(?:(?:_eq)?)|bit(?:and|or)|c(?:atch|lass|o(?:_(?:await|return|yield)|mpl|n(?:cept|st(?:_cast|eval|init)|tract_assert)))|d(?:e(?:(?:cltyp|let)e)|ynamic_cast)|exp(?:(?:lici|or)t)|f(?:inal|riend)|import|m(?:(?:odu|utab)le)|n(?:ew|o(?:except|t(?:(?:_eq)?)))|o(?:perator|r(?:(?:_eq)?)|verride)|p(?:ost|r(?:e|ivate|otected)|ublic)|re(?:interpret_cast|quires)|static_cast|t(?:emplate|h(?:is|row(?:(?:s)?))|ry|ype(?:id|name))|using|virtual|xor(?:(?:_eq)?))\b", KEYWORD),
        Rule::token_to(r"(?m)namespace\b", KEYWORD, NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(enum)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"enumname"])),
        Rule::bygroups_to(r"(?m)(struct|union)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token_to(r"(?m)case\b", KEYWORD, NewState::Push(vec![r"case-value"])),
        Rule::token(r"(?m)(_Pragma|a(?:lign(?:as|of)|sm|uto)|break|con(?:st(?:(?:expr)?)|tinue)|d(?:efault|o)|e(?:lse|num|xtern)|for(?:(?:tran)?)|goto|if|re(?:gister|stricted|turn)|s(?:izeof|t(?:atic(?:(?:_assert)?)|ruct)|witch)|t(?:hread_local|ype(?:def|of(?:(?:_unqual)?)))|union|(?:volat|wh)ile)\b", KEYWORD),
        Rule::token(r"(?m)(_(?:(?:(?:_)?)inline)|inline|naked|restrict|thread)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(__m(128i|128d|128|64))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)__(as(?:m|sume)|based|cdecl|declspec|except|f(?:astcall|inally|orceinline)|identifier|leave|n(?:oop|ull)|raise|stdcall|try|unaligned|w64)\b", KEYWORD_RESERVED),
    ]);
    m.insert(r"types", vec![
        Rule::token(r"(?m)char(16_t|32_t|8_t)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)__(int(?:16|32|64|8)|wchar_t)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_(?:BitInt|_int128)|bool|char|double|float|int|long|s(?:hort|igned)|(?:unsigne|voi)d)\b", KEYWORD_TYPE),
    ]);
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)^#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)^#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#if\s+0)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"if0"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"macro"])),
        Rule::bygroups(r"(?m)(^[ \t]*)(?!(?:public|private|protected|default)\b)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+)(\s*)(:)(?!:)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(?:.|(?<=\\)\n)*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        Rule::bygroups_to(r"(?m)(class|concept|typename)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token(r"(?m)(__restrict|and(?:(?:_eq)?)|bit(?:and|or)|c(?:atch|lass|o(?:_(?:await|return|yield)|mpl|n(?:cept|st(?:_cast|eval|init)|tract_assert)))|d(?:e(?:(?:cltyp|let)e)|ynamic_cast)|exp(?:(?:lici|or)t)|f(?:inal|riend)|import|m(?:(?:odu|utab)le)|n(?:ew|o(?:except|t(?:(?:_eq)?)))|o(?:perator|r(?:(?:_eq)?)|verride)|p(?:ost|r(?:e|ivate|otected)|ublic)|re(?:interpret_cast|quires)|static_cast|t(?:emplate|h(?:is|row(?:(?:s)?))|ry|ype(?:id|name))|using|virtual|xor(?:(?:_eq)?))\b", KEYWORD),
        Rule::token_to(r"(?m)namespace\b", KEYWORD, NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(enum)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"enumname"])),
        Rule::bygroups_to(r"(?m)(struct|union)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token_to(r"(?m)case\b", KEYWORD, NewState::Push(vec![r"case-value"])),
        Rule::token(r"(?m)(_Pragma|a(?:lign(?:as|of)|sm|uto)|break|con(?:st(?:(?:expr)?)|tinue)|d(?:efault|o)|e(?:lse|num|xtern)|for(?:(?:tran)?)|goto|if|re(?:gister|stricted|turn)|s(?:izeof|t(?:atic(?:(?:_assert)?)|ruct)|witch)|t(?:hread_local|ype(?:def|of(?:(?:_unqual)?)))|union|(?:volat|wh)ile)\b", KEYWORD),
        Rule::token(r"(?m)(_(?:(?:(?:_)?)inline)|inline|naked|restrict|thread)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(__m(128i|128d|128|64))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)__(as(?:m|sume)|based|cdecl|declspec|except|f(?:astcall|inally|orceinline)|identifier|leave|n(?:oop|ull)|raise|stdcall|try|unaligned|w64)\b", KEYWORD_RESERVED),
        Rule::bygroups_g_to(r#"(?m)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|::)+(?:[&*\s])+)(\s*(?:(?:(?://(?:.|(?<=\\)\n)*\n)|(?:/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/))\s*)*)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|::)+)(\s*(?:(?:(?://(?:.|(?<=\\)\n)*\n)|(?:/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/))\s*)*)(\([^;"\')]*?\))(\s*(?:(?:(?://(?:.|(?<=\\)\n)*\n)|(?:/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/))\s*)*)([^;{/"\']*)(\{)"#, vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::UsingThis { state: Some(vec!["root", "whitespace"]) }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "whitespace"]) }), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::UsingThis { state: Some(vec!["root", "whitespace"]) }), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"function"])),
        Rule::bygroups_g(r#"(?m)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|::)+(?:[&*\s])+)(\s*(?:(?:(?://(?:.|(?<=\\)\n)*\n)|(?:/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/))\s*)*)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|::)+)(\s*(?:(?:(?://(?:.|(?<=\\)\n)*\n)|(?:/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/))\s*)*)(\([^;"\')]*?\))(\s*(?:(?:(?://(?:.|(?<=\\)\n)*\n)|(?:/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/))\s*)*)([^;/"\']*)(;)"#, vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::UsingThis { state: Some(vec!["root", "whitespace"]) }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "whitespace"]) }), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::UsingThis { state: Some(vec!["root", "whitespace"]) }), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?m)char(16_t|32_t|8_t)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)__(int(?:16|32|64|8)|wchar_t)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_(?:BitInt|_int128)|bool|char|double|float|int|long|s(?:hort|igned)|(?:unsigne|voi)d)\b", KEYWORD_TYPE),
        Rule::default(NewState::Push(vec![r"statement"])),
        Rule::token(r"(?m)__(e(?:vent|xtends)|finally|i(?:mplements|nterface)|multiple_inheritance|null|s(?:ingle_inheritance|uper)|uuidof|virtual_inheritance)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)__(offload|blockingoffload|outer)\b", KEYWORD_PSEUDO),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token_to(r"(?m)^#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)^#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#if\s+0)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"if0"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"macro"])),
        Rule::bygroups(r"(?m)(^[ \t]*)(?!(?:public|private|protected|default)\b)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+)(\s*)(:)(?!:)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(?:.|(?<=\\)\n)*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
    ]);
    m.insert(r"enumname", vec![
        Rule::token_to(r"(?m)^#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)^#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#if\s+0)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"if0"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"macro"])),
        Rule::bygroups(r"(?m)(^[ \t]*)(?!(?:public|private|protected|default)\b)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+)(\s*)(:)(?!:)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(?:.|(?<=\\)\n)*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        Rule::token(r"(?m)(class|struct)\b", KEYWORD),
        Rule::token_to(r"(?m)(?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+", NAME_CLASS, NewState::Pop(1)),
        Rule::token_to(r"(?m)\s*(?=>)", TEXT, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"namespace", vec![
        Rule::token_to(r"(?m)[;{]", PUNCTUATION, NewState::Push(vec![r"#pop", r"root"])),
        Rule::token(r"(?m)inline\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+", NAME_NAMESPACE),
        Rule::token_to(r"(?m)^#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)^#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#if\s+0)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"if0"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"macro"])),
        Rule::bygroups(r"(?m)(^[ \t]*)(?!(?:public|private|protected|default)\b)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+)(\s*)(:)(?!:)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(?:.|(?<=\\)\n)*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        Rule::bygroups(r#"(?m)((?:[LuU]|u8)?R)(")([^\\()\s]{,16})(\()((?:.|\n)*?)(\)\3)(")"#, vec![Some(STRING_AFFIX), Some(STRING), Some(STRING_DELIMITER), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(STRING)]),
        Rule::bygroups_to(r"(?m)(class|concept|typename)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token(r"(?m)(__restrict|and(?:(?:_eq)?)|bit(?:and|or)|c(?:atch|lass|o(?:_(?:await|return|yield)|mpl|n(?:cept|st(?:_cast|eval|init)|tract_assert)))|d(?:e(?:(?:cltyp|let)e)|ynamic_cast)|exp(?:(?:lici|or)t)|f(?:inal|riend)|import|m(?:(?:odu|utab)le)|n(?:ew|o(?:except|t(?:(?:_eq)?)))|o(?:perator|r(?:(?:_eq)?)|verride)|p(?:ost|r(?:e|ivate|otected)|ublic)|re(?:interpret_cast|quires)|static_cast|t(?:emplate|h(?:is|row(?:(?:s)?))|ry|ype(?:id|name))|using|virtual|xor(?:(?:_eq)?))\b", KEYWORD),
        Rule::token_to(r"(?m)namespace\b", KEYWORD, NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(enum)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"enumname"])),
        Rule::bygroups_to(r"(?m)(struct|union)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token_to(r"(?m)case\b", KEYWORD, NewState::Push(vec![r"case-value"])),
        Rule::token(r"(?m)(_Pragma|a(?:lign(?:as|of)|sm|uto)|break|con(?:st(?:(?:expr)?)|tinue)|d(?:efault|o)|e(?:lse|num|xtern)|for(?:(?:tran)?)|goto|if|re(?:gister|stricted|turn)|s(?:izeof|t(?:atic(?:(?:_assert)?)|ruct)|witch)|t(?:hread_local|ype(?:def|of(?:(?:_unqual)?)))|union|(?:volat|wh)ile)\b", KEYWORD),
        Rule::token(r"(?m)(_(?:(?:(?:_)?)inline)|inline|naked|restrict|thread)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(__m(128i|128d|128|64))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)__(as(?:m|sume)|based|cdecl|declspec|except|f(?:astcall|inally|orceinline)|identifier|leave|n(?:oop|ull)|raise|stdcall|try|unaligned|w64)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)char(16_t|32_t|8_t)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)__(int(?:16|32|64|8)|wchar_t)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_(?:BitInt|_int128)|bool|char|double|float|int|long|s(?:hort|igned)|(?:unsigne|voi)d)\b", KEYWORD_TYPE),
        Rule::bygroups_to(r#"(?m)([LuU]|u8)?(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?m)([LuU]|u8)?(')(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])(')", vec![Some(STRING_AFFIX), Some(STRING_CHAR), Some(STRING_CHAR), Some(STRING_CHAR)]),
        Rule::token(r"(?m)0[xX]([0-9a-fA-F](\'?[0-9a-fA-F])*\.[0-9a-fA-F](\'?[0-9a-fA-F])*|\.[0-9a-fA-F](\'?[0-9a-fA-F])*|[0-9a-fA-F](\'?[0-9a-fA-F])*)[pP][+-]?[0-9a-fA-F](\'?[0-9a-fA-F])*[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?(\d(\'?\d)*\.\d(\'?\d)*|\.\d(\'?\d)*|\d(\'?\d)*)[eE][+-]?\d(\'?\d)*[fFlL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?((\d(\'?\d)*\.(\d(\'?\d)*)?|\.\d(\'?\d)*)[fFlL]?)|(\d(\'?\d)*[fFlL])", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?0[xX][0-9a-fA-F](\'?[0-9a-fA-F])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_HEX),
        Rule::token(r"(?m)(-)?0[bB][01](\'?[01])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_BIN),
        Rule::token(r"(?m)(-)?0(\'?[0-7])+(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_OCT),
        Rule::token(r"(?m)(-)?\d(\'?\d)*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::token(r"(?m)(true|false|NULL|nullptr)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+", NAME),
        Rule::token(r"(?m)\}", PUNCTUATION),
        Rule::token_to(r"(?m)[{;]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"statement", vec![
        Rule::token_to(r"(?m)^#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)^#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#if\s+0)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"if0"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"macro"])),
        Rule::bygroups(r"(?m)(^[ \t]*)(?!(?:public|private|protected|default)\b)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+)(\s*)(:)(?!:)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(?:.|(?<=\\)\n)*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        Rule::bygroups(r#"(?m)((?:[LuU]|u8)?R)(")([^\\()\s]{,16})(\()((?:.|\n)*?)(\)\3)(")"#, vec![Some(STRING_AFFIX), Some(STRING), Some(STRING_DELIMITER), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(STRING)]),
        Rule::bygroups_to(r"(?m)(class|concept|typename)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token(r"(?m)(__restrict|and(?:(?:_eq)?)|bit(?:and|or)|c(?:atch|lass|o(?:_(?:await|return|yield)|mpl|n(?:cept|st(?:_cast|eval|init)|tract_assert)))|d(?:e(?:(?:cltyp|let)e)|ynamic_cast)|exp(?:(?:lici|or)t)|f(?:inal|riend)|import|m(?:(?:odu|utab)le)|n(?:ew|o(?:except|t(?:(?:_eq)?)))|o(?:perator|r(?:(?:_eq)?)|verride)|p(?:ost|r(?:e|ivate|otected)|ublic)|re(?:interpret_cast|quires)|static_cast|t(?:emplate|h(?:is|row(?:(?:s)?))|ry|ype(?:id|name))|using|virtual|xor(?:(?:_eq)?))\b", KEYWORD),
        Rule::token_to(r"(?m)namespace\b", KEYWORD, NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(enum)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"enumname"])),
        Rule::bygroups_to(r"(?m)(struct|union)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token_to(r"(?m)case\b", KEYWORD, NewState::Push(vec![r"case-value"])),
        Rule::token(r"(?m)(_Pragma|a(?:lign(?:as|of)|sm|uto)|break|con(?:st(?:(?:expr)?)|tinue)|d(?:efault|o)|e(?:lse|num|xtern)|for(?:(?:tran)?)|goto|if|re(?:gister|stricted|turn)|s(?:izeof|t(?:atic(?:(?:_assert)?)|ruct)|witch)|t(?:hread_local|ype(?:def|of(?:(?:_unqual)?)))|union|(?:volat|wh)ile)\b", KEYWORD),
        Rule::token(r"(?m)(_(?:(?:(?:_)?)inline)|inline|naked|restrict|thread)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(__m(128i|128d|128|64))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)__(as(?:m|sume)|based|cdecl|declspec|except|f(?:astcall|inally|orceinline)|identifier|leave|n(?:oop|ull)|raise|stdcall|try|unaligned|w64)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)char(16_t|32_t|8_t)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)__(int(?:16|32|64|8)|wchar_t)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_(?:BitInt|_int128)|bool|char|double|float|int|long|s(?:hort|igned)|(?:unsigne|voi)d)\b", KEYWORD_TYPE),
        Rule::bygroups_to(r#"(?m)([LuU]|u8)?(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?m)([LuU]|u8)?(')(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])(')", vec![Some(STRING_AFFIX), Some(STRING_CHAR), Some(STRING_CHAR), Some(STRING_CHAR)]),
        Rule::token(r"(?m)0[xX]([0-9a-fA-F](\'?[0-9a-fA-F])*\.[0-9a-fA-F](\'?[0-9a-fA-F])*|\.[0-9a-fA-F](\'?[0-9a-fA-F])*|[0-9a-fA-F](\'?[0-9a-fA-F])*)[pP][+-]?[0-9a-fA-F](\'?[0-9a-fA-F])*[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?(\d(\'?\d)*\.\d(\'?\d)*|\.\d(\'?\d)*|\d(\'?\d)*)[eE][+-]?\d(\'?\d)*[fFlL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?((\d(\'?\d)*\.(\d(\'?\d)*)?|\.\d(\'?\d)*)[fFlL]?)|(\d(\'?\d)*[fFlL])", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?0[xX][0-9a-fA-F](\'?[0-9a-fA-F])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_HEX),
        Rule::token(r"(?m)(-)?0[bB][01](\'?[01])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_BIN),
        Rule::token(r"(?m)(-)?0(\'?[0-7])+(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_OCT),
        Rule::token(r"(?m)(-)?\d(\'?\d)*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::token(r"(?m)(true|false|NULL|nullptr)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+", NAME),
        Rule::token(r"(?m)\}", PUNCTUATION),
        Rule::token_to(r"(?m)[{;]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"function", vec![
        Rule::token_to(r"(?m)^#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)^#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#if\s+0)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"if0"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"macro"])),
        Rule::bygroups(r"(?m)(^[ \t]*)(?!(?:public|private|protected|default)\b)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+)(\s*)(:)(?!:)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(?:.|(?<=\\)\n)*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        Rule::bygroups(r#"(?m)((?:[LuU]|u8)?R)(")([^\\()\s]{,16})(\()((?:.|\n)*?)(\)\3)(")"#, vec![Some(STRING_AFFIX), Some(STRING), Some(STRING_DELIMITER), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(STRING)]),
        Rule::bygroups_to(r"(?m)(class|concept|typename)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token(r"(?m)(__restrict|and(?:(?:_eq)?)|bit(?:and|or)|c(?:atch|lass|o(?:_(?:await|return|yield)|mpl|n(?:cept|st(?:_cast|eval|init)|tract_assert)))|d(?:e(?:(?:cltyp|let)e)|ynamic_cast)|exp(?:(?:lici|or)t)|f(?:inal|riend)|import|m(?:(?:odu|utab)le)|n(?:ew|o(?:except|t(?:(?:_eq)?)))|o(?:perator|r(?:(?:_eq)?)|verride)|p(?:ost|r(?:e|ivate|otected)|ublic)|re(?:interpret_cast|quires)|static_cast|t(?:emplate|h(?:is|row(?:(?:s)?))|ry|ype(?:id|name))|using|virtual|xor(?:(?:_eq)?))\b", KEYWORD),
        Rule::token_to(r"(?m)namespace\b", KEYWORD, NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(enum)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"enumname"])),
        Rule::bygroups_to(r"(?m)(struct|union)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token_to(r"(?m)case\b", KEYWORD, NewState::Push(vec![r"case-value"])),
        Rule::token(r"(?m)(_Pragma|a(?:lign(?:as|of)|sm|uto)|break|con(?:st(?:(?:expr)?)|tinue)|d(?:efault|o)|e(?:lse|num|xtern)|for(?:(?:tran)?)|goto|if|re(?:gister|stricted|turn)|s(?:izeof|t(?:atic(?:(?:_assert)?)|ruct)|witch)|t(?:hread_local|ype(?:def|of(?:(?:_unqual)?)))|union|(?:volat|wh)ile)\b", KEYWORD),
        Rule::token(r"(?m)(_(?:(?:(?:_)?)inline)|inline|naked|restrict|thread)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(__m(128i|128d|128|64))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)__(as(?:m|sume)|based|cdecl|declspec|except|f(?:astcall|inally|orceinline)|identifier|leave|n(?:oop|ull)|raise|stdcall|try|unaligned|w64)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)char(16_t|32_t|8_t)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)__(int(?:16|32|64|8)|wchar_t)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_(?:BitInt|_int128)|bool|char|double|float|int|long|s(?:hort|igned)|(?:unsigne|voi)d)\b", KEYWORD_TYPE),
        Rule::bygroups_to(r#"(?m)([LuU]|u8)?(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?m)([LuU]|u8)?(')(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])(')", vec![Some(STRING_AFFIX), Some(STRING_CHAR), Some(STRING_CHAR), Some(STRING_CHAR)]),
        Rule::token(r"(?m)0[xX]([0-9a-fA-F](\'?[0-9a-fA-F])*\.[0-9a-fA-F](\'?[0-9a-fA-F])*|\.[0-9a-fA-F](\'?[0-9a-fA-F])*|[0-9a-fA-F](\'?[0-9a-fA-F])*)[pP][+-]?[0-9a-fA-F](\'?[0-9a-fA-F])*[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?(\d(\'?\d)*\.\d(\'?\d)*|\.\d(\'?\d)*|\d(\'?\d)*)[eE][+-]?\d(\'?\d)*[fFlL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?((\d(\'?\d)*\.(\d(\'?\d)*)?|\.\d(\'?\d)*)[fFlL]?)|(\d(\'?\d)*[fFlL])", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?0[xX][0-9a-fA-F](\'?[0-9a-fA-F])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_HEX),
        Rule::token(r"(?m)(-)?0[bB][01](\'?[01])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_BIN),
        Rule::token(r"(?m)(-)?0(\'?[0-7])+(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_OCT),
        Rule::token(r"(?m)(-)?\d(\'?\d)*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::token(r"(?m)(true|false|NULL|nullptr)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+", NAME),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::token(r"(?m)\\\n", STRING),
        Rule::token(r"(?m)\\", STRING),
    ]);
    m.insert(
        r"macro",
        vec![
            Rule::bygroups_g(
                r#"(?m)(\s*(?:/[*].*?[*]/\s*)?)(include)(\s*(?:/[*].*?[*]/\s*)?)("[^"]+")([^\n]*)"#,
                vec![
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(COMMENT_PREPROCFILE)),
                    Some(GroupAction::Token(COMMENT_SINGLE)),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)(\s*(?:/[*].*?[*]/\s*)?)(include)(\s*(?:/[*].*?[*]/\s*)?)(<[^>]+>)([^\n]*)",
                vec![
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(COMMENT_PREPROCFILE)),
                    Some(GroupAction::Token(COMMENT_SINGLE)),
                ],
            ),
            Rule::token(r"(?m)[^/\n]+", COMMENT_PREPROC),
            Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)//.*?\n", COMMENT_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?m)/", COMMENT_PREPROC),
            Rule::token(r"(?m)(?<=\\)\n", COMMENT_PREPROC),
            Rule::token_to(r"(?m)\n", COMMENT_PREPROC, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"if0",
        vec![
            Rule::token_to(
                r"(?m)^\s*#if.*?(?<!\\)\n",
                COMMENT_PREPROC,
                NewState::PushSame,
            ),
            Rule::token_to(
                r"(?m)^\s*#el(?:se|if).*\n",
                COMMENT_PREPROC,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)^\s*#endif.*?(?<!\\)\n",
                COMMENT_PREPROC,
                NewState::Pop(1),
            ),
            Rule::token(r"(?m).*?\n", COMMENT),
        ],
    );
    m.insert(
        r"classname",
        vec![
            Rule::token_to(
                r"(?m)(?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+",
                NAME_CLASS,
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m)\s*(?=>)", TEXT, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(r"case-value", vec![
        Rule::token_to(r"(?m)(?<!:)(:)(?!:)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+", NAME_CONSTANT),
        Rule::token_to(r"(?m)^#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)^#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#if\s+0)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"if0"])),
        Rule::bygroups_g_to(r"(?m)^(\s*(?:/[*].*?[*]/\s*)?)(#)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(COMMENT_PREPROC))], NewState::Push(vec![r"macro"])),
        Rule::bygroups(r"(?m)(^[ \t]*)(?!(?:public|private|protected|default)\b)((?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+)(\s*)(:)(?!:)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(?:.|(?<=\\)\n)*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/(?:\\\n)?[*](?:[^*]|[*](?!(?:\\\n)?/))*[*](?:\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        Rule::bygroups(r#"(?m)((?:[LuU]|u8)?R)(")([^\\()\s]{,16})(\()((?:.|\n)*?)(\)\3)(")"#, vec![Some(STRING_AFFIX), Some(STRING), Some(STRING_DELIMITER), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(STRING)]),
        Rule::bygroups_to(r"(?m)(class|concept|typename)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token(r"(?m)(__restrict|and(?:(?:_eq)?)|bit(?:and|or)|c(?:atch|lass|o(?:_(?:await|return|yield)|mpl|n(?:cept|st(?:_cast|eval|init)|tract_assert)))|d(?:e(?:(?:cltyp|let)e)|ynamic_cast)|exp(?:(?:lici|or)t)|f(?:inal|riend)|import|m(?:(?:odu|utab)le)|n(?:ew|o(?:except|t(?:(?:_eq)?)))|o(?:perator|r(?:(?:_eq)?)|verride)|p(?:ost|r(?:e|ivate|otected)|ublic)|re(?:interpret_cast|quires)|static_cast|t(?:emplate|h(?:is|row(?:(?:s)?))|ry|ype(?:id|name))|using|virtual|xor(?:(?:_eq)?))\b", KEYWORD),
        Rule::token_to(r"(?m)namespace\b", KEYWORD, NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(enum)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"enumname"])),
        Rule::bygroups_to(r"(?m)(struct|union)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token_to(r"(?m)case\b", KEYWORD, NewState::Push(vec![r"case-value"])),
        Rule::token(r"(?m)(_Pragma|a(?:lign(?:as|of)|sm|uto)|break|con(?:st(?:(?:expr)?)|tinue)|d(?:efault|o)|e(?:lse|num|xtern)|for(?:(?:tran)?)|goto|if|re(?:gister|stricted|turn)|s(?:izeof|t(?:atic(?:(?:_assert)?)|ruct)|witch)|t(?:hread_local|ype(?:def|of(?:(?:_unqual)?)))|union|(?:volat|wh)ile)\b", KEYWORD),
        Rule::token(r"(?m)(_(?:(?:(?:_)?)inline)|inline|naked|restrict|thread)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(__m(128i|128d|128|64))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)__(as(?:m|sume)|based|cdecl|declspec|except|f(?:astcall|inally|orceinline)|identifier|leave|n(?:oop|ull)|raise|stdcall|try|unaligned|w64)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)char(16_t|32_t|8_t)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)__(int(?:16|32|64|8)|wchar_t)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(_(?:BitInt|_int128)|bool|char|double|float|int|long|s(?:hort|igned)|(?:unsigne|voi)d)\b", KEYWORD_TYPE),
        Rule::bygroups_to(r#"(?m)([LuU]|u8)?(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?m)([LuU]|u8)?(')(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])(')", vec![Some(STRING_AFFIX), Some(STRING_CHAR), Some(STRING_CHAR), Some(STRING_CHAR)]),
        Rule::token(r"(?m)0[xX]([0-9a-fA-F](\'?[0-9a-fA-F])*\.[0-9a-fA-F](\'?[0-9a-fA-F])*|\.[0-9a-fA-F](\'?[0-9a-fA-F])*|[0-9a-fA-F](\'?[0-9a-fA-F])*)[pP][+-]?[0-9a-fA-F](\'?[0-9a-fA-F])*[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?(\d(\'?\d)*\.\d(\'?\d)*|\.\d(\'?\d)*|\d(\'?\d)*)[eE][+-]?\d(\'?\d)*[fFlL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?((\d(\'?\d)*\.(\d(\'?\d)*)?|\.\d(\'?\d)*)[fFlL]?)|(\d(\'?\d)*[fFlL])", NUMBER_FLOAT),
        Rule::token(r"(?m)(-)?0[xX][0-9a-fA-F](\'?[0-9a-fA-F])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_HEX),
        Rule::token(r"(?m)(-)?0[bB][01](\'?[01])*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_BIN),
        Rule::token(r"(?m)(-)?0(\'?[0-7])+(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_OCT),
        Rule::token(r"(?m)(-)?\d(\'?\d)*(([uU]?[zZ])|([zZ][uU])|([uU][lL]{0,2})|([lL]{1,2}[uU]?))?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::token(r"(?m)(true|false|NULL|nullptr)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?!\d)(?:[\w$]|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8})+", NAME),
    ]);
    Table(m)
}

impl Lexer for ArduinoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
