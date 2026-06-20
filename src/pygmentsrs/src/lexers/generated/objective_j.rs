#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.javascript:ObjectiveJLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.javascript:ObjectiveJLexer:objective_j

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: objective-j, objectivej, obj-j, objj
pub struct ObjectiveJLexer;

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
        Rule::bygroups(r#"(?ms)(@import)(\s+)("(?:\\\\|\\"|[^"])*")"#, vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::bygroups(r"(?ms)(@import)(\s+)(<(?:\\\\|\\>|[^>])*>)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::bygroups(r#"(?ms)(#(?:include|import))(\s+)("(?:\\\\|\\"|[^"])*")"#, vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::bygroups(r"(?ms)(#(?:include|import))(\s+)(<(?:\\\\|\\>|[^>])*>)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::token_to(r"(?ms)#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?ms)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::bygroups(r"(?ms)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)//(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?ms)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)<!--", COMMENT),
        Rule::bygroups_g(r"(?ms)^((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*[+-](?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)([(a-zA-Z_].*?[^(])((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*\{)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::UsingThis { state: Some(vec!["root", "function_signature"]) }), Some(GroupAction::UsingThis { state: None })]),
        Rule::bygroups_to(r"(?ms)(@interface|@implementation)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::bygroups_to(r"(?ms)(@class|@protocol)(\s*)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"forward_classname"])),
        Rule::bygroups(r"(?ms)(\s*)(@end)(\s*)", vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE)]),
        Rule::token_to(r#"(?ms)(L|@)?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?ms)(L|@)?'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?ms)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+[Ll]?", NUMBER_HEX),
        Rule::token(r"(?ms)0[0-7]+[Ll]?", NUMBER_OCT),
        Rule::token(r"(?ms)\d+[Ll]?", NUMBER_INTEGER),
        Rule::token_to(r"(?ms)^(?=\s|/|<!--)", TEXT, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)\+\+|--|~|&&|\?|:|\|\||\\(?=\n)|(<<|>>>?|==?|!=?|[-<>+*%&|^/])=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[})\].]", PUNCTUATION),
        Rule::token_to(r"(?ms)(for|in|while|do|break|return|continue|switch|case|default|if|else|throw|try|catch|finally|new|delete|typeof|instanceof|void|prototype|__proto__)\b", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)(var|with|function)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)(@selector|@private|@protected|@public|@encode|@synchronized|@try|@throw|@catch|@finally|@end|@property|@synthesize|@dynamic|@for|@accessors|new)\b", KEYWORD),
        Rule::token(r"(?ms)(int|long|float|short|double|char|unsigned|signed|void|id|BOOL|bool|boolean|IBOutlet|IBAction|SEL|@outlet|@action)\b", KEYWORD_TYPE),
        Rule::token(r"(?ms)(self|super)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)(TRUE|YES|FALSE|NO|Nil|nil|NULL)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(true|false|null|NaN|Infinity|undefined)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(ABS|ASIN|ACOS|ATAN|ATAN2|SIN|COS|TAN|EXP|POW|CEIL|FLOOR|ROUND|MIN|MAX|RAND|SQRT|E|LN2|LN10|LOG2E|LOG10E|PI|PI2|PI_2|SQRT1_2|SQRT2)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Math|Number|Object|RegExp|String|decodeURI|decodeURIComponent|encodeURI|encodeURIComponent|Error|eval|isFinite|isNaN|parseFloat|parseInt|document|this|window|globalThis|Symbol)\b", NAME_BUILTIN),
        Rule::bygroups_g(r"(?ms)([$a-zA-Z_]\w*)((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)(?=\()", vec![Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::UsingThis { state: None })]),
        Rule::token(r"(?ms)[$a-zA-Z_]\w*", NAME),
        Rule::token(r"(?ms)[{()}]", PUNCTUATION),
        Rule::token(r"(?ms);", PUNCTUATION),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::bygroups(
                r#"(?ms)(@import)(\s+)("(?:\\\\|\\"|[^"])*")"#,
                vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)],
            ),
            Rule::bygroups(
                r"(?ms)(@import)(\s+)(<(?:\\\\|\\>|[^>])*>)",
                vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)],
            ),
            Rule::bygroups(
                r#"(?ms)(#(?:include|import))(\s+)("(?:\\\\|\\"|[^"])*")"#,
                vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)],
            ),
            Rule::bygroups(
                r"(?ms)(#(?:include|import))(\s+)(<(?:\\\\|\\>|[^>])*>)",
                vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)],
            ),
            Rule::token_to(
                r"(?ms)#if\s+0",
                COMMENT_PREPROC,
                NewState::Push(vec![r"if0"]),
            ),
            Rule::token_to(r"(?ms)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?ms)(\\)(\n)",
                vec![Some(STRING_ESCAPE), Some(WHITESPACE)],
            ),
            Rule::token(r"(?ms)//(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
            Rule::token(r"(?ms)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?ms)<!--", COMMENT),
        ],
    );
    m.insert(r"statements", vec![
        Rule::token_to(r#"(?ms)(L|@)?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?ms)(L|@)?'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?ms)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+[Ll]?", NUMBER_HEX),
        Rule::token(r"(?ms)0[0-7]+[Ll]?", NUMBER_OCT),
        Rule::token(r"(?ms)\d+[Ll]?", NUMBER_INTEGER),
        Rule::token_to(r"(?ms)^(?=\s|/|<!--)", TEXT, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)\+\+|--|~|&&|\?|:|\|\||\\(?=\n)|(<<|>>>?|==?|!=?|[-<>+*%&|^/])=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[})\].]", PUNCTUATION),
        Rule::token_to(r"(?ms)(for|in|while|do|break|return|continue|switch|case|default|if|else|throw|try|catch|finally|new|delete|typeof|instanceof|void|prototype|__proto__)\b", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)(var|with|function)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)(@selector|@private|@protected|@public|@encode|@synchronized|@try|@throw|@catch|@finally|@end|@property|@synthesize|@dynamic|@for|@accessors|new)\b", KEYWORD),
        Rule::token(r"(?ms)(int|long|float|short|double|char|unsigned|signed|void|id|BOOL|bool|boolean|IBOutlet|IBAction|SEL|@outlet|@action)\b", KEYWORD_TYPE),
        Rule::token(r"(?ms)(self|super)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)(TRUE|YES|FALSE|NO|Nil|nil|NULL)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(true|false|null|NaN|Infinity|undefined)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(ABS|ASIN|ACOS|ATAN|ATAN2|SIN|COS|TAN|EXP|POW|CEIL|FLOOR|ROUND|MIN|MAX|RAND|SQRT|E|LN2|LN10|LOG2E|LOG10E|PI|PI2|PI_2|SQRT1_2|SQRT2)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Math|Number|Object|RegExp|String|decodeURI|decodeURIComponent|encodeURI|encodeURIComponent|Error|eval|isFinite|isNaN|parseFloat|parseInt|document|this|window|globalThis|Symbol)\b", NAME_BUILTIN),
        Rule::bygroups_g(r"(?ms)([$a-zA-Z_]\w*)((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)(?=\()", vec![Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::UsingThis { state: None })]),
        Rule::token(r"(?ms)[$a-zA-Z_]\w*", NAME),
    ]);
    m.insert(
        r"slashstartsregex",
        vec![
            Rule::bygroups(
                r#"(?ms)(@import)(\s+)("(?:\\\\|\\"|[^"])*")"#,
                vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)],
            ),
            Rule::bygroups(
                r"(?ms)(@import)(\s+)(<(?:\\\\|\\>|[^>])*>)",
                vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)],
            ),
            Rule::bygroups(
                r#"(?ms)(#(?:include|import))(\s+)("(?:\\\\|\\"|[^"])*")"#,
                vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)],
            ),
            Rule::bygroups(
                r"(?ms)(#(?:include|import))(\s+)(<(?:\\\\|\\>|[^>])*>)",
                vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)],
            ),
            Rule::token_to(
                r"(?ms)#if\s+0",
                COMMENT_PREPROC,
                NewState::Push(vec![r"if0"]),
            ),
            Rule::token_to(r"(?ms)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?ms)(\\)(\n)",
                vec![Some(STRING_ESCAPE), Some(WHITESPACE)],
            ),
            Rule::token(r"(?ms)//(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
            Rule::token(r"(?ms)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?ms)<!--", COMMENT),
            Rule::token_to(
                r"(?ms)/(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gim]+\b|\B)",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?ms)(?=/)",
                TEXT,
                NewState::Push(vec![r"#pop", r"badregex"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"badregex",
        vec![Rule::token_to(r"(?ms)\n", WHITESPACE, NewState::Pop(1))],
    );
    m.insert(r"classname", vec![
        Rule::bygroups_g_to(r"(?ms)([a-zA-Z_]\w*)((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*:(?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)([a-zA-Z_]\w*)?", vec![Some(GroupAction::Token(NAME_CLASS)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_CLASS))], NewState::Pop(1)),
        Rule::bygroups_g_to(r"(?ms)([a-zA-Z_]\w*)((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*\()([a-zA-Z_]\w*)(\))", vec![Some(GroupAction::Token(NAME_CLASS)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_LABEL)), Some(GroupAction::Token(TEXT))], NewState::Pop(1)),
        Rule::token_to(r"(?ms)([a-zA-Z_]\w*)", NAME_CLASS, NewState::Pop(1)),
    ]);
    m.insert(
        r"forward_classname",
        vec![
            Rule::bygroups_to(
                r"(?ms)([a-zA-Z_]\w*)(\s*)(,)(\s*)",
                vec![
                    Some(NAME_CLASS),
                    Some(WHITESPACE),
                    Some(TEXT),
                    Some(WHITESPACE),
                ],
                NewState::PushSame,
            ),
            Rule::bygroups_to(
                r"(?ms)([a-zA-Z_]\w*)(\s*)(;?)",
                vec![Some(NAME_CLASS), Some(WHITESPACE), Some(TEXT)],
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(r"function_signature", vec![
        Rule::bygroups(r#"(?ms)(@import)(\s+)("(?:\\\\|\\"|[^"])*")"#, vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::bygroups(r"(?ms)(@import)(\s+)(<(?:\\\\|\\>|[^>])*>)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::bygroups(r#"(?ms)(#(?:include|import))(\s+)("(?:\\\\|\\"|[^"])*")"#, vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::bygroups(r"(?ms)(#(?:include|import))(\s+)(<(?:\\\\|\\>|[^>])*>)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::token_to(r"(?ms)#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?ms)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::bygroups(r"(?ms)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)//(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?ms)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)<!--", COMMENT),
        Rule::bygroups_g_to(r"(?ms)(\((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)([a-zA-Z_]\w+)((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*\)(?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)([$a-zA-Z_]\w+(?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*:)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(KEYWORD_TYPE)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION))], NewState::Push(vec![r"function_parameters"])),
        Rule::bygroups_g_to(r"(?ms)(\((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)([a-zA-Z_]\w+)((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*\)(?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)([$a-zA-Z_]\w+)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(KEYWORD_TYPE)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION))], NewState::Pop(1)),
        Rule::bygroups_to(r"(?ms)([$a-zA-Z_]\w+(?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*:)", vec![Some(NAME_FUNCTION)], NewState::Push(vec![r"function_parameters"])),
        Rule::bygroups_to(r"(?ms)([$a-zA-Z_]\w+)", vec![Some(NAME_FUNCTION)], NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"function_parameters", vec![
        Rule::bygroups(r#"(?ms)(@import)(\s+)("(?:\\\\|\\"|[^"])*")"#, vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::bygroups(r"(?ms)(@import)(\s+)(<(?:\\\\|\\>|[^>])*>)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::bygroups(r#"(?ms)(#(?:include|import))(\s+)("(?:\\\\|\\"|[^"])*")"#, vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::bygroups(r"(?ms)(#(?:include|import))(\s+)(<(?:\\\\|\\>|[^>])*>)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING_DOUBLE)]),
        Rule::token_to(r"(?ms)#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?ms)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::bygroups(r"(?ms)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)//(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?ms)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)<!--", COMMENT),
        Rule::bygroups_g(r"(?ms)(\((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)([^)]+)((?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*\)(?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*)([$a-zA-Z_]\w+)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(KEYWORD_TYPE)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(TEXT))]),
        Rule::token(r"(?ms)([$a-zA-Z_]\w+(?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*:)", NAME_FUNCTION),
        Rule::token(r"(?ms)(:)", NAME_FUNCTION),
        Rule::using_this(r"(?ms)(,(?:\s|//[^\n]*\n|/[*](?:[^*]|[*][^/])*[*]/)*\.\.\.)", None),
        Rule::token(r"(?ms)([$a-zA-Z_]\w+)", TEXT),
    ]);
    m.insert(
        r"expression",
        vec![
            Rule::bygroups(
                r"(?ms)([$a-zA-Z_]\w*)(\()",
                vec![Some(NAME_FUNCTION), Some(PUNCTUATION)],
            ),
            Rule::token_to(r"(?ms)(\))", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?ms)""#, STRING, NewState::Pop(1)),
            Rule::token(
                r#"(?ms)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|[0-7]{1,3})"#,
                STRING_ESCAPE,
            ),
            Rule::token(r#"(?ms)[^\\"\n]+"#, STRING),
            Rule::bygroups(
                r"(?ms)(\\)(\n)",
                vec![Some(STRING_ESCAPE), Some(WHITESPACE)],
            ),
            Rule::token(r"(?ms)\\", STRING),
        ],
    );
    m.insert(
        r"macro",
        vec![
            Rule::token(r"(?ms)[^/\n]+", COMMENT_PREPROC),
            Rule::token(r"(?ms)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::bygroups_to(
                r"(?ms)(//.*?)(\n)",
                vec![Some(COMMENT_SINGLE), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?ms)/", COMMENT_PREPROC),
            Rule::token(r"(?ms)(?<=\\)\n", WHITESPACE),
            Rule::token_to(r"(?ms)\n", WHITESPACE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"if0",
        vec![
            Rule::token_to(
                r"(?ms)^\s*#if.*?(?<!\\)\n",
                COMMENT_PREPROC,
                NewState::PushSame,
            ),
            Rule::token_to(
                r"(?ms)^\s*#endif.*?(?<!\\)\n",
                COMMENT_PREPROC,
                NewState::Pop(1),
            ),
            Rule::bygroups(r"(?ms)(.*?)(\n)", vec![Some(COMMENT), Some(WHITESPACE)]),
        ],
    );
    Table(m)
}

impl Lexer for ObjectiveJLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
