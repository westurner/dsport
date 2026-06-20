//! AUTO-GENERATED from `pygments.pygments.lexers.php:PhpLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.php:PhpLexer:php

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: php, php3, php4, php5
pub struct PhpLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"root",
        vec![
            Rule::token_to(
                r"(?ims)<\?(php)?",
                COMMENT_PREPROC,
                NewState::Push(vec![r"php"]),
            ),
            Rule::token(r"(?ims)[^<]+", OTHER),
            Rule::token(r"(?ims)<", OTHER),
        ],
    );
    m.insert(r"php", vec![
        Rule::token_to(r"(?ims)\?>", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::bygroups(r#"(?ims)(<<<)([\'"]?)((?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*)(\2\n.*?\n\s*)(\3)(;?)(\n)"#, vec![Some(STRING), Some(STRING), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(PUNCTUATION), Some(TEXT)]),
        Rule::token(r"(?ims)\s+", TEXT),
        Rule::token_to(r"(?ims)#\[", PUNCTUATION, NewState::Push(vec![r"attribute"])),
        Rule::token(r"(?ims)#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ims)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ims)/\*\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ims)/\*\*.*?\*/", STRING_DOC),
        Rule::token(r"(?ims)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ims)(->|::)(\s*)((?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*)", vec![Some(OPERATOR), Some(TEXT), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ims)[~!%^&*+=|:.<>/@-]+", OPERATOR),
        Rule::token(r"(?ims)\?", OPERATOR),
        Rule::token(r"(?ims)[\[\]{}();,]+", PUNCTUATION),
        Rule::bygroups(r"(?ims)(new)(\s+)(class)\b", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD)]),
        Rule::bygroups_to(r"(?ims)(class)(\s+)", vec![Some(KEYWORD), Some(TEXT)], NewState::Push(vec![r"classname"])),
        Rule::bygroups(r"(?ims)(function)(\s*)(?=\()", vec![Some(KEYWORD), Some(TEXT)]),
        Rule::bygroups_to(r"(?ims)(function)(\s+)(&?)(\s*)", vec![Some(KEYWORD), Some(TEXT), Some(OPERATOR), Some(TEXT)], NewState::Push(vec![r"functionname"])),
        Rule::bygroups(r"(?ims)(const)(\s+)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_CONSTANT)]),
        Rule::token(r"(?ims)(and|E_PARSE|old_function|E_ERROR|or|as|E_WARNING|parent|eval|PHP_OS|break|exit|case|extends|PHP_VERSION|cfunction|FALSE|print|for|require|continue|foreach|require_once|declare|return|default|static|do|switch|die|stdClass|echo|else|TRUE|elseif|var|empty|if|xor|enddeclare|include|virtual|endfor|include_once|while|endforeach|global|endif|list|endswitch|new|endwhile|not|array|E_ALL|NULL|final|php_user_filter|interface|implements|public|private|protected|abstract|clone|try|catch|throw|this|use|namespace|trait|yield( from)?|finally|match|readonly)\b", KEYWORD),
        Rule::token(r"(?ims)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ims)(__(?:(?:CLASS|DIR|F(?:ILE|UNCTION)|LINE|METHOD|NAMESPACE|PROPERTY|TRAIT)__))\b", NAME_CONSTANT),
        Rule::token_to(r"(?ims)\$\{", NAME_VARIABLE, NewState::Push(vec![r"variablevariable"])),
        Rule::token(r"(?ims)\$+(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ims)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_OTHER),
        Rule::token(r"(?ims)(\d+\.\d*|\d*\.\d+)(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?ims)\d+e[+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?ims)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?ims)0x[a-f0-9]+", NUMBER_HEX),
        Rule::token(r"(?ims)\d+", NUMBER_INTEGER),
        Rule::token(r"(?ims)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?ims)'([^'\\]*(?:\\.[^'\\]*)*)'", STRING_SINGLE),
        Rule::token(r"(?ims)`([^`\\]*(?:\\.[^`\\]*)*)`", STRING_BACKTICK),
        Rule::token_to(r#"(?ims)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
    ]);
    m.insert(r"magicconstants", vec![
        Rule::token(r"(?ims)(__(?:(?:CLASS|DIR|F(?:ILE|UNCTION)|LINE|METHOD|NAMESPACE|PROPERTY|TRAIT)__))\b", NAME_CONSTANT),
    ]);
    m.insert(r"variablevariable", vec![
        Rule::token_to(r"(?ims)\}", NAME_VARIABLE, NewState::Pop(1)),
        Rule::token_to(r"(?ims)\?>", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::bygroups(r#"(?ims)(<<<)([\'"]?)((?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*)(\2\n.*?\n\s*)(\3)(;?)(\n)"#, vec![Some(STRING), Some(STRING), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(PUNCTUATION), Some(TEXT)]),
        Rule::token(r"(?ims)\s+", TEXT),
        Rule::token_to(r"(?ims)#\[", PUNCTUATION, NewState::Push(vec![r"attribute"])),
        Rule::token(r"(?ims)#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ims)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ims)/\*\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ims)/\*\*.*?\*/", STRING_DOC),
        Rule::token(r"(?ims)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ims)(->|::)(\s*)((?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*)", vec![Some(OPERATOR), Some(TEXT), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ims)[~!%^&*+=|:.<>/@-]+", OPERATOR),
        Rule::token(r"(?ims)\?", OPERATOR),
        Rule::token(r"(?ims)[\[\]{}();,]+", PUNCTUATION),
        Rule::bygroups(r"(?ims)(new)(\s+)(class)\b", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD)]),
        Rule::bygroups_to(r"(?ims)(class)(\s+)", vec![Some(KEYWORD), Some(TEXT)], NewState::Push(vec![r"classname"])),
        Rule::bygroups(r"(?ims)(function)(\s*)(?=\()", vec![Some(KEYWORD), Some(TEXT)]),
        Rule::bygroups_to(r"(?ims)(function)(\s+)(&?)(\s*)", vec![Some(KEYWORD), Some(TEXT), Some(OPERATOR), Some(TEXT)], NewState::Push(vec![r"functionname"])),
        Rule::bygroups(r"(?ims)(const)(\s+)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_CONSTANT)]),
        Rule::token(r"(?ims)(and|E_PARSE|old_function|E_ERROR|or|as|E_WARNING|parent|eval|PHP_OS|break|exit|case|extends|PHP_VERSION|cfunction|FALSE|print|for|require|continue|foreach|require_once|declare|return|default|static|do|switch|die|stdClass|echo|else|TRUE|elseif|var|empty|if|xor|enddeclare|include|virtual|endfor|include_once|while|endforeach|global|endif|list|endswitch|new|endwhile|not|array|E_ALL|NULL|final|php_user_filter|interface|implements|public|private|protected|abstract|clone|try|catch|throw|this|use|namespace|trait|yield( from)?|finally|match|readonly)\b", KEYWORD),
        Rule::token(r"(?ims)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ims)(__(?:(?:CLASS|DIR|F(?:ILE|UNCTION)|LINE|METHOD|NAMESPACE|PROPERTY|TRAIT)__))\b", NAME_CONSTANT),
        Rule::token_to(r"(?ims)\$\{", NAME_VARIABLE, NewState::Push(vec![r"variablevariable"])),
        Rule::token(r"(?ims)\$+(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ims)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_OTHER),
        Rule::token(r"(?ims)(\d+\.\d*|\d*\.\d+)(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?ims)\d+e[+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?ims)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?ims)0x[a-f0-9]+", NUMBER_HEX),
        Rule::token(r"(?ims)\d+", NUMBER_INTEGER),
        Rule::token(r"(?ims)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?ims)'([^'\\]*(?:\\.[^'\\]*)*)'", STRING_SINGLE),
        Rule::token(r"(?ims)`([^`\\]*(?:\\.[^`\\]*)*)`", STRING_BACKTICK),
        Rule::token_to(r#"(?ims)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
    ]);
    m.insert(r"magicfuncs", vec![
        Rule::token(r"(?ims)(__(?:c(?:all(?:(?:Static)?)|lone|onstruct)|de(?:bugInfo|struct)|get|i(?:nvoke|sset)|s(?:et(?:(?:_state)?)|leep)|toString|unset|wakeup))\b", NAME_FUNCTION_MAGIC),
    ]);
    m.insert(
        r"classname",
        vec![Rule::token_to(
            r"(?ims)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(r"functionname", vec![
        Rule::token(r"(?ims)(__(?:c(?:all(?:(?:Static)?)|lone|onstruct)|de(?:bugInfo|struct)|get|i(?:nvoke|sset)|s(?:et(?:(?:_state)?)|leep)|toString|unset|wakeup))\b", NAME_FUNCTION_MAGIC),
        Rule::token_to(r"(?ims)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_FUNCTION, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?ims)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?ims)[^{$"\\]+"#, STRING_DOUBLE),
        Rule::token(r#"(?ims)\\([nrt"$\\]|[0-7]{1,3}|x[0-9a-f]{1,2})"#, STRING_ESCAPE),
        Rule::token(r"(?ims)\$(?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*(\[\S+?\]|->(?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*)?", STRING_INTERPOL),
        Rule::bygroups_g(r"(?ims)(\{\$\{)(.*?)(\}\})", vec![Some(GroupAction::Token(STRING_INTERPOL)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(STRING_INTERPOL))]),
        Rule::bygroups_g(r"(?ims)(\{)(\$.*?)(\})", vec![Some(GroupAction::Token(STRING_INTERPOL)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(STRING_INTERPOL))]),
        Rule::bygroups(r"(?ims)(\$\{)(\S+)(\})", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token(r"(?ims)[${\\]", STRING_DOUBLE),
    ]);
    m.insert(r"attribute", vec![
        Rule::token_to(r"(?ims)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?ims)\(", PUNCTUATION, NewState::Push(vec![r"attributeparams"])),
        Rule::token(r"(?ims)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_DECORATOR),
        Rule::token_to(r"(?ims)\?>", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::bygroups(r#"(?ims)(<<<)([\'"]?)((?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*)(\2\n.*?\n\s*)(\3)(;?)(\n)"#, vec![Some(STRING), Some(STRING), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(PUNCTUATION), Some(TEXT)]),
        Rule::token(r"(?ims)\s+", TEXT),
        Rule::token_to(r"(?ims)#\[", PUNCTUATION, NewState::Push(vec![r"attribute"])),
        Rule::token(r"(?ims)#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ims)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ims)/\*\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ims)/\*\*.*?\*/", STRING_DOC),
        Rule::token(r"(?ims)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ims)(->|::)(\s*)((?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*)", vec![Some(OPERATOR), Some(TEXT), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ims)[~!%^&*+=|:.<>/@-]+", OPERATOR),
        Rule::token(r"(?ims)\?", OPERATOR),
        Rule::token(r"(?ims)[\[\]{}();,]+", PUNCTUATION),
        Rule::bygroups(r"(?ims)(new)(\s+)(class)\b", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD)]),
        Rule::bygroups_to(r"(?ims)(class)(\s+)", vec![Some(KEYWORD), Some(TEXT)], NewState::Push(vec![r"classname"])),
        Rule::bygroups(r"(?ims)(function)(\s*)(?=\()", vec![Some(KEYWORD), Some(TEXT)]),
        Rule::bygroups_to(r"(?ims)(function)(\s+)(&?)(\s*)", vec![Some(KEYWORD), Some(TEXT), Some(OPERATOR), Some(TEXT)], NewState::Push(vec![r"functionname"])),
        Rule::bygroups(r"(?ims)(const)(\s+)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_CONSTANT)]),
        Rule::token(r"(?ims)(and|E_PARSE|old_function|E_ERROR|or|as|E_WARNING|parent|eval|PHP_OS|break|exit|case|extends|PHP_VERSION|cfunction|FALSE|print|for|require|continue|foreach|require_once|declare|return|default|static|do|switch|die|stdClass|echo|else|TRUE|elseif|var|empty|if|xor|enddeclare|include|virtual|endfor|include_once|while|endforeach|global|endif|list|endswitch|new|endwhile|not|array|E_ALL|NULL|final|php_user_filter|interface|implements|public|private|protected|abstract|clone|try|catch|throw|this|use|namespace|trait|yield( from)?|finally|match|readonly)\b", KEYWORD),
        Rule::token(r"(?ims)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ims)(__(?:(?:CLASS|DIR|F(?:ILE|UNCTION)|LINE|METHOD|NAMESPACE|PROPERTY|TRAIT)__))\b", NAME_CONSTANT),
        Rule::token_to(r"(?ims)\$\{", NAME_VARIABLE, NewState::Push(vec![r"variablevariable"])),
        Rule::token(r"(?ims)\$+(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ims)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_OTHER),
        Rule::token(r"(?ims)(\d+\.\d*|\d*\.\d+)(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?ims)\d+e[+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?ims)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?ims)0x[a-f0-9]+", NUMBER_HEX),
        Rule::token(r"(?ims)\d+", NUMBER_INTEGER),
        Rule::token(r"(?ims)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?ims)'([^'\\]*(?:\\.[^'\\]*)*)'", STRING_SINGLE),
        Rule::token(r"(?ims)`([^`\\]*(?:\\.[^`\\]*)*)`", STRING_BACKTICK),
        Rule::token_to(r#"(?ims)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
    ]);
    m.insert(r"attributeparams", vec![
        Rule::token_to(r"(?ims)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?ims)\?>", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::bygroups(r#"(?ims)(<<<)([\'"]?)((?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*)(\2\n.*?\n\s*)(\3)(;?)(\n)"#, vec![Some(STRING), Some(STRING), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(PUNCTUATION), Some(TEXT)]),
        Rule::token(r"(?ims)\s+", TEXT),
        Rule::token_to(r"(?ims)#\[", PUNCTUATION, NewState::Push(vec![r"attribute"])),
        Rule::token(r"(?ims)#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ims)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ims)/\*\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ims)/\*\*.*?\*/", STRING_DOC),
        Rule::token(r"(?ims)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ims)(->|::)(\s*)((?:[_a-z]|[^\x00-\x7f])(?:\w|[^\x00-\x7f])*)", vec![Some(OPERATOR), Some(TEXT), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ims)[~!%^&*+=|:.<>/@-]+", OPERATOR),
        Rule::token(r"(?ims)\?", OPERATOR),
        Rule::token(r"(?ims)[\[\]{}();,]+", PUNCTUATION),
        Rule::bygroups(r"(?ims)(new)(\s+)(class)\b", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD)]),
        Rule::bygroups_to(r"(?ims)(class)(\s+)", vec![Some(KEYWORD), Some(TEXT)], NewState::Push(vec![r"classname"])),
        Rule::bygroups(r"(?ims)(function)(\s*)(?=\()", vec![Some(KEYWORD), Some(TEXT)]),
        Rule::bygroups_to(r"(?ims)(function)(\s+)(&?)(\s*)", vec![Some(KEYWORD), Some(TEXT), Some(OPERATOR), Some(TEXT)], NewState::Push(vec![r"functionname"])),
        Rule::bygroups(r"(?ims)(const)(\s+)((?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_CONSTANT)]),
        Rule::token(r"(?ims)(and|E_PARSE|old_function|E_ERROR|or|as|E_WARNING|parent|eval|PHP_OS|break|exit|case|extends|PHP_VERSION|cfunction|FALSE|print|for|require|continue|foreach|require_once|declare|return|default|static|do|switch|die|stdClass|echo|else|TRUE|elseif|var|empty|if|xor|enddeclare|include|virtual|endfor|include_once|while|endforeach|global|endif|list|endswitch|new|endwhile|not|array|E_ALL|NULL|final|php_user_filter|interface|implements|public|private|protected|abstract|clone|try|catch|throw|this|use|namespace|trait|yield( from)?|finally|match|readonly)\b", KEYWORD),
        Rule::token(r"(?ims)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ims)(__(?:(?:CLASS|DIR|F(?:ILE|UNCTION)|LINE|METHOD|NAMESPACE|PROPERTY|TRAIT)__))\b", NAME_CONSTANT),
        Rule::token_to(r"(?ims)\$\{", NAME_VARIABLE, NewState::Push(vec![r"variablevariable"])),
        Rule::token(r"(?ims)\$+(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_VARIABLE),
        Rule::token(r"(?ims)(?:[\\_a-z]|[^\x00-\x7f])(?:[\\\w]|[^\x00-\x7f])*", NAME_OTHER),
        Rule::token(r"(?ims)(\d+\.\d*|\d*\.\d+)(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?ims)\d+e[+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?ims)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?ims)0x[a-f0-9]+", NUMBER_HEX),
        Rule::token(r"(?ims)\d+", NUMBER_INTEGER),
        Rule::token(r"(?ims)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?ims)'([^'\\]*(?:\\.[^'\\]*)*)'", STRING_SINGLE),
        Rule::token(r"(?ims)`([^`\\]*(?:\\.[^`\\]*)*)`", STRING_BACKTICK),
        Rule::token_to(r#"(?ims)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
    ]);
    Table(m)
}

impl Lexer for PhpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
