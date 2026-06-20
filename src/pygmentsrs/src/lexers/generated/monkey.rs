//! AUTO-GENERATED from `pygments.pygments.lexers.basic:MonkeyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.basic:MonkeyLexer:monkey

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: monkey
pub struct MonkeyLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)'.*", COMMENT),
        Rule::token_to(r"(?m)(?i)^#rem\b", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)(?i)^(?:#If|#ElseIf|#Else|#EndIf|#End|#Print|#Error)\b", COMMENT_PREPROC),
        Rule::token_to(r"(?m)^#", COMMENT_PREPROC, NewState::Push(vec![r"variables"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[0-9]+\.[0-9]*(?!\.)", NUMBER_FLOAT),
        Rule::token(r"(?m)\.[0-9]+(?!\.)", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[0-9a-fA-Z]+", NUMBER_HEX),
        Rule::token(r"(?m)\%[10]+", NUMBER_BIN),
        Rule::token(r"(?m)\b(?:Int|Float|String|Bool|Object|Array|Void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(?i)\b(?:Try|Catch|Throw)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)Throwable", NAME_EXCEPTION),
        Rule::token(r"(?m)(?i)\b(?:Null|True|False)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?i)\b(?:Self|Super)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)\b(?:HOST|LANG|TARGET|CONFIG)\b", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(?i)^(Import)(\s+)(.*)(\n)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE)]),
        Rule::token(r"(?m)(?i)^Strict\b.*\n", KEYWORD_RESERVED),
        Rule::bygroups_to(r"(?m)(?i)(Const|Local|Global|Field)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"variables"])),
        Rule::bygroups_to(r"(?m)(?i)(New|Class|Interface|Extends|Implements)(\s+)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::bygroups_to(r"(?m)(?i)(Function|Method)(\s+)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE)], NewState::Push(vec![r"funcname"])),
        Rule::token(r"(?m)(?i)(?:End|Return|Public|Private|Extern|Property|Final|Abstract)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(?i)(?:If|Then|Else|ElseIf|EndIf|Select|Case|Default|While|Wend|Repeat|Until|Forever|For|To|Until|Step|EachIn|Next|Exit|Continue)(?=\s)", KEYWORD_RESERVED),
        Rule::token(r"(?m)(?i)\b(?:Module|Inline)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)[\[\]]", PUNCTUATION),
        Rule::token(r"(?m)<=|>=|<>|\*=|/=|\+=|-=|&=|~=|\|=|[-&*/^+=<>|~]", OPERATOR),
        Rule::token(r"(?m)(?i)(?:Not|Mod|Shl|Shr|And|Or)", OPERATOR_WORD),
        Rule::token(r"(?m)[(){}!#,.:]", PUNCTUATION),
        Rule::token(r"(?m)[A-Z_][A-Z0-9_]*\b", NAME_CONSTANT),
        Rule::token(r"(?m)[A-Z]\w*\b", NAME_FUNCTION),
        Rule::token(r"(?m)[a-z_]\w*\b", NAME_VARIABLE),
    ]);
    m.insert(
        r"funcname",
        vec![
            Rule::token(r"(?m)(?i)[A-Z]\w*\b", NAME_FUNCTION),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Push(vec![r"classname"])),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"variables"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"classname",
        vec![
            Rule::token(r"(?m)[a-z0-9_]*\.", NAME_NAMESPACE),
            Rule::token(
                r"(?m)(?:Int|Float|String|Bool|Object|Array|Void)\b",
                KEYWORD_TYPE,
            ),
            Rule::token(r"(?m)[A-Z]\w*\b", NAME_CLASS),
            Rule::bygroups(
                r"(?m)(\[)(\s*)(\d*)(\s*)(\])",
                vec![
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(NUMBER_INTEGER),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::token_to(r"(?m)\s+(?!<)", WHITESPACE, NewState::Pop(1)),
            Rule::token_to(r"(?m)<", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?m)>", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"variables",
        vec![
            Rule::token(r"(?m)[A-Z_][A-Z0-9_]*\b", NAME_CONSTANT),
            Rule::token(r"(?m)[a-z_]\w*\b", NAME_VARIABLE),
            Rule::token(r"(?m)[?%#$]", KEYWORD_TYPE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Push(vec![r"classname"])),
            Rule::token_to(r"(?m),", PUNCTUATION, NewState::PushSame),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)[^"~]+"#, STRING_DOUBLE),
            Rule::token(r"(?m)~q|~n|~r|~t|~z|~~", STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token_to(r"(?m)(?i)^#rem.*?", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)(?i)^#end.*?", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)\n", COMMENT_MULTILINE),
            Rule::token(r"(?m).+", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for MonkeyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
