//! AUTO-GENERATED from `pygments.pygments.lexers.c_like:ValaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.c_like:ValaLexer:vala

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: vala, vapi
pub struct ValaLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"whitespace", vec![
        Rule::token_to(r"(?m)^\s*#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
    ]);
    m.insert(r"statements", vec![
        Rule::token_to(r#"(?m)[L@]?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)L?'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r#"(?m)(?s)""".*?""""#, STRING),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[Ll]?", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+[Ll]?", NUMBER_OCT),
        Rule::token(r"(?m)\d+[Ll]?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::bygroups(r"(?m)(\[)(Compact|Immutable|(?:Boolean|Simple)Type)(\])", vec![Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\[)(CCode|(?:Integer|Floating)Type)", vec![Some(PUNCTUATION), Some(NAME_DECORATOR)]),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::token(r"(?m)(as|b(?:ase|reak)|c(?:a(?:se|tch)|on(?:struct|tinue))|d(?:e(?:fault|lete)|o)|e(?:lse|num)|f(?:inally|or(?:(?:each)?))|get|i(?:[fns])|lock|new|out|params|return|s(?:et|izeof|witch)|t(?:h(?:is|row)|ry|ypeof)|while|yield)\b", KEYWORD),
        Rule::token(r"(?m)(abstract|const|d(?:elegate|ynamic)|e(?:nsures|xtern)|in(?:line|ternal)|o(?:verride|wned)|p(?:r(?:ivate|otected)|ublic)|re(?:f|quires)|s(?:ignal|tatic)|throws|unowned|v(?:ar|irtual|olatile)|weak|yields)\b", KEYWORD_DECLARATION),
        Rule::bygroups_to(r"(?m)(namespace|using)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(class|errordomain|interface|struct)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"class"])),
        Rule::bygroups(r"(?m)(\.)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?m)(bool|char|double|float|int(?:(?:16|32|64|8)?)|long|s(?:hort|ize_t|size_t|tring)|time_t|u(?:char|int(?:(?:16|32|64|8)?)|long|nichar|short)|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(true|false|null)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)^\s*#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::default(NewState::Push(vec![r"statement"])),
    ]);
    m.insert(r"statement", vec![
        Rule::token_to(r"(?m)^\s*#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token_to(r#"(?m)[L@]?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)L?'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r#"(?m)(?s)""".*?""""#, STRING),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[Ll]?", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+[Ll]?", NUMBER_OCT),
        Rule::token(r"(?m)\d+[Ll]?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::bygroups(r"(?m)(\[)(Compact|Immutable|(?:Boolean|Simple)Type)(\])", vec![Some(PUNCTUATION), Some(NAME_DECORATOR), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\[)(CCode|(?:Integer|Floating)Type)", vec![Some(PUNCTUATION), Some(NAME_DECORATOR)]),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::token(r"(?m)(as|b(?:ase|reak)|c(?:a(?:se|tch)|on(?:struct|tinue))|d(?:e(?:fault|lete)|o)|e(?:lse|num)|f(?:inally|or(?:(?:each)?))|get|i(?:[fns])|lock|new|out|params|return|s(?:et|izeof|witch)|t(?:h(?:is|row)|ry|ypeof)|while|yield)\b", KEYWORD),
        Rule::token(r"(?m)(abstract|const|d(?:elegate|ynamic)|e(?:nsures|xtern)|in(?:line|ternal)|o(?:verride|wned)|p(?:r(?:ivate|otected)|ublic)|re(?:f|quires)|s(?:ignal|tatic)|throws|unowned|v(?:ar|irtual|olatile)|weak|yields)\b", KEYWORD_DECLARATION),
        Rule::bygroups_to(r"(?m)(namespace|using)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(class|errordomain|interface|struct)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"class"])),
        Rule::bygroups(r"(?m)(\.)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?m)(bool|char|double|float|int(?:(?:16|32|64|8)?)|long|s(?:hort|ize_t|size_t|tring)|time_t|u(?:char|int(?:(?:16|32|64|8)?)|long|nichar|short)|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(true|false|null)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)[{}]", PUNCTUATION),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::token(r"(?m)\\\n", STRING),
        Rule::token(r"(?m)\\", STRING),
    ]);
    m.insert(r"if0", vec![
        Rule::token_to(r"(?m)^\s*#if.*?(?<!\\)\n", COMMENT_PREPROC, NewState::PushSame),
        Rule::token_to(r"(?m)^\s*#el(?:se|if).*\n", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::token_to(r"(?m)^\s*#endif.*?(?<!\\)\n", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::token(r"(?m).*?\n", COMMENT),
    ]);
    m.insert(r"class", vec![
        Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_CLASS, NewState::Pop(1)),
    ]);
    m.insert(r"namespace", vec![
        Rule::token_to(r"(?m)[a-zA-Z_][\w.]*", NAME_NAMESPACE, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for ValaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
