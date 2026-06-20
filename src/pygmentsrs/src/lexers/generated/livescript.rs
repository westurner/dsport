#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.javascript:LiveScriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.javascript:LiveScriptLexer:livescript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: livescript, live-script
pub struct LivescriptLexer;

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
        r"commentsandwhitespace",
        vec![
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::bygroups(
                r"(?ms)(#.*?)(\n)",
                vec![Some(COMMENT_SINGLE), Some(WHITESPACE)],
            ),
        ],
    );
    m.insert(
        r"multilineregex",
        vec![
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::bygroups(
                r"(?ms)(#.*?)(\n)",
                vec![Some(COMMENT_SINGLE), Some(WHITESPACE)],
            ),
            Rule::token_to(r"(?ms)//([gimuysd]+\b|\B)", STRING_REGEX, NewState::Pop(1)),
            Rule::token(r"(?ms)/", STRING_REGEX),
            Rule::token(r"(?ms)[^/#]+", STRING_REGEX),
        ],
    );
    m.insert(
        r"slashstartsregex",
        vec![
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::bygroups(
                r"(?ms)(#.*?)(\n)",
                vec![Some(COMMENT_SINGLE), Some(WHITESPACE)],
            ),
            Rule::token_to(
                r"(?ms)//",
                STRING_REGEX,
                NewState::Push(vec![r"#pop", r"multilineregex"]),
            ),
            Rule::token_to(
                r"(?ms)/(?! )(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gimuysd]+\b|\B)",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?ms)/", OPERATOR, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(r"root", vec![
        Rule::token_to(r"(?ms)\A(?=\s|/)", TEXT, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ms)(#.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)(?:\([^()]+\))?[ ]*[~-]{1,2}>|(?:\(?[^()\n]+\)?)?[ ]*<[~-]{1,2}", NAME_FUNCTION),
        Rule::token_to(r"(?ms)\+\+|&&|(?<![.$])\b(?:and|x?or|is|isnt|not)\b|\?|:|=|\|\||\\(?=\n)|(<<|>>>?|==?|!=?|~(?!\~?>)|-(?!\-?>)|<(?!\[)|(?<!\])>|[+*`%&|^/])=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[})\].]", PUNCTUATION),
        Rule::token_to(r"(?ms)(?<![.$])(for|own|in|of|while|until|loop|break|return|continue|switch|when|then|if|unless|else|throw|try|catch|finally|new|delete|typeof|instanceof|super|extends|this|class|by|const|var|to|til)\b", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)(?<![.$])(true|false|yes|no|on|off|null|NaN|Infinity|undefined|void)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Math|Number|Object|RegExp|String|decodeURI|decodeURIComponent|encodeURI|encodeURIComponent|eval|isFinite|isNaN|parseFloat|parseInt|document|window|globalThis|Symbol|Symbol|BigInt)\b", NAME_BUILTIN),
        Rule::bygroups_to(r"(?ms)([$a-zA-Z_][\w.\-:$]*)(\s*)([:=])(\s+)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE)], NewState::Push(vec![r"slashstartsregex"])),
        Rule::bygroups_to(r"(?ms)(@[$a-zA-Z_][\w.\-:$]*)(\s*)([:=])(\s+)", vec![Some(NAME_VARIABLE_INSTANCE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE)], NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)@", NAME_OTHER, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)@?[$a-zA-Z_][\w-]*", NAME_OTHER, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[0-9]+\.[0-9]+([eE][0-9]+)?[fd]?(?:[a-zA-Z_]+)?", NUMBER_FLOAT),
        Rule::token(r"(?ms)[0-9]+(~[0-9a-z]+)?(?:[a-zA-Z_]+)?", NUMBER_INTEGER),
        Rule::token_to(r#"(?ms)""""#, STRING, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r"(?ms)'''", STRING, NewState::Push(vec![r"tsqs"])),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?ms)'", STRING, NewState::Push(vec![r"sqs"])),
        Rule::token(r"(?ms)\\\S+", STRING),
        Rule::token(r"(?ms)<\[.*?\]>", STRING),
    ]);
    m.insert(r"strings", vec![Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING)]);
    m.insert(r"interpoling_string", vec![
        Rule::token_to(r"(?ms)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\A(?=\s|/)", TEXT, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ms)(#.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)(?:\([^()]+\))?[ ]*[~-]{1,2}>|(?:\(?[^()\n]+\)?)?[ ]*<[~-]{1,2}", NAME_FUNCTION),
        Rule::token_to(r"(?ms)\+\+|&&|(?<![.$])\b(?:and|x?or|is|isnt|not)\b|\?|:|=|\|\||\\(?=\n)|(<<|>>>?|==?|!=?|~(?!\~?>)|-(?!\-?>)|<(?!\[)|(?<!\])>|[+*`%&|^/])=?", OPERATOR, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)[{(\[;,]", PUNCTUATION, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[})\].]", PUNCTUATION),
        Rule::token_to(r"(?ms)(?<![.$])(for|own|in|of|while|until|loop|break|return|continue|switch|when|then|if|unless|else|throw|try|catch|finally|new|delete|typeof|instanceof|super|extends|this|class|by|const|var|to|til)\b", KEYWORD, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)(?<![.$])(true|false|yes|no|on|off|null|NaN|Infinity|undefined|void)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Math|Number|Object|RegExp|String|decodeURI|decodeURIComponent|encodeURI|encodeURIComponent|eval|isFinite|isNaN|parseFloat|parseInt|document|window|globalThis|Symbol|Symbol|BigInt)\b", NAME_BUILTIN),
        Rule::bygroups_to(r"(?ms)([$a-zA-Z_][\w.\-:$]*)(\s*)([:=])(\s+)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE)], NewState::Push(vec![r"slashstartsregex"])),
        Rule::bygroups_to(r"(?ms)(@[$a-zA-Z_][\w.\-:$]*)(\s*)([:=])(\s+)", vec![Some(NAME_VARIABLE_INSTANCE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE)], NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)@", NAME_OTHER, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token_to(r"(?ms)@?[$a-zA-Z_][\w-]*", NAME_OTHER, NewState::Push(vec![r"slashstartsregex"])),
        Rule::token(r"(?ms)[0-9]+\.[0-9]+([eE][0-9]+)?[fd]?(?:[a-zA-Z_]+)?", NUMBER_FLOAT),
        Rule::token(r"(?ms)[0-9]+(~[0-9a-z]+)?(?:[a-zA-Z_]+)?", NUMBER_INTEGER),
        Rule::token_to(r#"(?ms)""""#, STRING, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r"(?ms)'''", STRING, NewState::Push(vec![r"tsqs"])),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?ms)'", STRING, NewState::Push(vec![r"sqs"])),
        Rule::token(r"(?ms)\\\S+", STRING),
        Rule::token(r"(?ms)<\[.*?\]>", STRING),
    ]);
    m.insert(
        r"dqs",
        vec![
            Rule::token_to(r#"(?ms)""#, STRING, NewState::Pop(1)),
            Rule::token(r"(?ms)\\.|\'", STRING),
            Rule::token_to(
                r"(?ms)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpoling_string"]),
            ),
            Rule::token(r"(?ms)#", STRING),
            Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(
        r"sqs",
        vec![
            Rule::token_to(r"(?ms)'", STRING, NewState::Pop(1)),
            Rule::token(r#"(?ms)#|\\.|""#, STRING),
            Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(
        r"tdqs",
        vec![
            Rule::token_to(r#"(?ms)""""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?ms)\\.|\'|""#, STRING),
            Rule::token_to(
                r"(?ms)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpoling_string"]),
            ),
            Rule::token(r"(?ms)#", STRING),
            Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(
        r"tsqs",
        vec![
            Rule::token_to(r"(?ms)'''", STRING, NewState::Pop(1)),
            Rule::token(r#"(?ms)#|\\.|\'|""#, STRING),
            Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for LivescriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
