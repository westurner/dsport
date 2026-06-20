//! AUTO-GENERATED from `pygments.pygments.lexers.smalltalk:SmalltalkLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.smalltalk:SmalltalkLexer:smalltalk

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: smalltalk, squeak, st
pub struct SmalltalkLexer;

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
        Rule::bygroups(r"(?m)(<)(\w+:)(.*?)(>)", vec![Some(TEXT), Some(KEYWORD), Some(TEXT), Some(TEXT)]),
        Rule::token(r#"(?m)^"(""|[^"])*"!"#, KEYWORD),
        Rule::token(r"(?m)^'(''|[^'])*'!", KEYWORD),
        Rule::bygroups(r"(?m)^(!)(\w+)( commentStamp: )(.*?)( prior: .*?!\n)(.*?)(!)", vec![Some(KEYWORD), Some(NAME_CLASS), Some(KEYWORD), Some(STRING), Some(KEYWORD), Some(TEXT), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^(!)(\w+(?: class)?)( methodsFor: )('(?:''|[^'])*')(.*?!)", vec![Some(KEYWORD), Some(NAME_CLASS), Some(KEYWORD), Some(STRING), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^(\w+)( subclass: )(#\w+)(\s+instanceVariableNames: )(.*?)(\s+classVariableNames: )(.*?)(\s+poolDictionaries: )(.*?)(\s+category: )(.*?)(!)", vec![Some(NAME_CLASS), Some(KEYWORD), Some(STRING_SYMBOL), Some(KEYWORD), Some(STRING), Some(KEYWORD), Some(STRING), Some(KEYWORD), Some(STRING), Some(KEYWORD), Some(STRING), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^(\w+(?: class)?)(\s+instanceVariableNames: )(.*?)(!)", vec![Some(NAME_CLASS), Some(KEYWORD), Some(STRING), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)(!\n)(\].*)(! !)$", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD)]),
        Rule::token(r"(?m)! !$", KEYWORD),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r#"(?m)"(""|[^"])*""#, COMMENT),
        Rule::bygroups(r"(?m)([a-zA-Z]+\w*:)(\s*)(\w+)", vec![Some(NAME_FUNCTION), Some(TEXT), Some(NAME_VARIABLE)]),
        Rule::bygroups(r"(?m)^(\b[a-zA-Z]+\w*\b)(\s*)$", vec![Some(NAME_FUNCTION), Some(TEXT)]),
        Rule::bygroups(r"(?m)^([-+*/\\~<>=|&!?,@%]+)(\s*)(\w+)(\s*)$", vec![Some(NAME_FUNCTION), Some(TEXT), Some(NAME_VARIABLE), Some(TEXT)]),
        Rule::bygroups(r"(?m)(\|)([\w\s]*)(\|)", vec![Some(OPERATOR), Some(NAME_VARIABLE), Some(OPERATOR)]),
        Rule::token_to(r"(?m)\[", TEXT, NewState::Push(vec![r"blockvariables"])),
        Rule::token_to(r"(?m)\]", TEXT, NewState::Push(vec![r"afterobject"])),
        Rule::token_to(r"(?m)\b(self|super|true|false|nil|thisContext)\b", NAME_BUILTIN_PSEUDO, NewState::Push(vec![r"afterobject"])),
        Rule::token_to(r"(?m)\b[A-Z]\w*(?!:)\b", NAME_CLASS, NewState::Push(vec![r"afterobject"])),
        Rule::token_to(r"(?m)\b[a-z]\w*(?!:)\b", NAME_VARIABLE, NewState::Push(vec![r"afterobject"])),
        Rule::token_to(r#"(?m)#("(""|[^"])*"|[-+*/\\~<>=|&!?,@%]+|[\w:]+)"#, STRING_SYMBOL, NewState::Push(vec![r"afterobject"])),
        Rule::token_to(r"(?m)'(''|[^'])*'", STRING, NewState::Push(vec![r"afterobject"])),
        Rule::token_to(r"(?m)\$.", STRING_CHAR, NewState::Push(vec![r"afterobject"])),
        Rule::token_to(r"(?m)#\(", STRING_SYMBOL, NewState::Push(vec![r"parenth"])),
        Rule::token_to(r"(?m)\)", TEXT, NewState::Push(vec![r"afterobject"])),
        Rule::token_to(r"(?m)(\d+r)?-?\d+(\.\d+)?(e-?\d+)?", NUMBER, NewState::Push(vec![r"afterobject"])),
        Rule::token(r"(?m)\^|\:=|\_", OPERATOR),
        Rule::token(r"(?m)[\]({}.;!]", TEXT),
    ]);
    m.insert(r"squeak fileout", vec![
        Rule::token(r#"(?m)^"(""|[^"])*"!"#, KEYWORD),
        Rule::token(r"(?m)^'(''|[^'])*'!", KEYWORD),
        Rule::bygroups(r"(?m)^(!)(\w+)( commentStamp: )(.*?)( prior: .*?!\n)(.*?)(!)", vec![Some(KEYWORD), Some(NAME_CLASS), Some(KEYWORD), Some(STRING), Some(KEYWORD), Some(TEXT), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^(!)(\w+(?: class)?)( methodsFor: )('(?:''|[^'])*')(.*?!)", vec![Some(KEYWORD), Some(NAME_CLASS), Some(KEYWORD), Some(STRING), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^(\w+)( subclass: )(#\w+)(\s+instanceVariableNames: )(.*?)(\s+classVariableNames: )(.*?)(\s+poolDictionaries: )(.*?)(\s+category: )(.*?)(!)", vec![Some(NAME_CLASS), Some(KEYWORD), Some(STRING_SYMBOL), Some(KEYWORD), Some(STRING), Some(KEYWORD), Some(STRING), Some(KEYWORD), Some(STRING), Some(KEYWORD), Some(STRING), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^(\w+(?: class)?)(\s+instanceVariableNames: )(.*?)(!)", vec![Some(NAME_CLASS), Some(KEYWORD), Some(STRING), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)(!\n)(\].*)(! !)$", vec![Some(KEYWORD), Some(TEXT), Some(KEYWORD)]),
        Rule::token(r"(?m)! !$", KEYWORD),
    ]);
    m.insert(
        r"whitespaces",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"(""|[^"])*""#, COMMENT),
        ],
    );
    m.insert(
        r"method definition",
        vec![
            Rule::bygroups(
                r"(?m)([a-zA-Z]+\w*:)(\s*)(\w+)",
                vec![Some(NAME_FUNCTION), Some(TEXT), Some(NAME_VARIABLE)],
            ),
            Rule::bygroups(
                r"(?m)^(\b[a-zA-Z]+\w*\b)(\s*)$",
                vec![Some(NAME_FUNCTION), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)^([-+*/\\~<>=|&!?,@%]+)(\s*)(\w+)(\s*)$",
                vec![
                    Some(NAME_FUNCTION),
                    Some(TEXT),
                    Some(NAME_VARIABLE),
                    Some(TEXT),
                ],
            ),
        ],
    );
    m.insert(
        r"objects",
        vec![
            Rule::token_to(r"(?m)\[", TEXT, NewState::Push(vec![r"blockvariables"])),
            Rule::token_to(r"(?m)\]", TEXT, NewState::Push(vec![r"afterobject"])),
            Rule::token_to(
                r"(?m)\b(self|super|true|false|nil|thisContext)\b",
                NAME_BUILTIN_PSEUDO,
                NewState::Push(vec![r"afterobject"]),
            ),
            Rule::token_to(
                r"(?m)\b[A-Z]\w*(?!:)\b",
                NAME_CLASS,
                NewState::Push(vec![r"afterobject"]),
            ),
            Rule::token_to(
                r"(?m)\b[a-z]\w*(?!:)\b",
                NAME_VARIABLE,
                NewState::Push(vec![r"afterobject"]),
            ),
            Rule::token_to(
                r#"(?m)#("(""|[^"])*"|[-+*/\\~<>=|&!?,@%]+|[\w:]+)"#,
                STRING_SYMBOL,
                NewState::Push(vec![r"afterobject"]),
            ),
            Rule::token_to(
                r"(?m)'(''|[^'])*'",
                STRING,
                NewState::Push(vec![r"afterobject"]),
            ),
            Rule::token_to(
                r"(?m)\$.",
                STRING_CHAR,
                NewState::Push(vec![r"afterobject"]),
            ),
            Rule::token_to(r"(?m)#\(", STRING_SYMBOL, NewState::Push(vec![r"parenth"])),
            Rule::token_to(r"(?m)\)", TEXT, NewState::Push(vec![r"afterobject"])),
            Rule::token_to(
                r"(?m)(\d+r)?-?\d+(\.\d+)?(e-?\d+)?",
                NUMBER,
                NewState::Push(vec![r"afterobject"]),
            ),
        ],
    );
    m.insert(
        r"literals",
        vec![
            Rule::token_to(
                r"(?m)'(''|[^'])*'",
                STRING,
                NewState::Push(vec![r"afterobject"]),
            ),
            Rule::token_to(
                r"(?m)\$.",
                STRING_CHAR,
                NewState::Push(vec![r"afterobject"]),
            ),
            Rule::token_to(r"(?m)#\(", STRING_SYMBOL, NewState::Push(vec![r"parenth"])),
            Rule::token_to(r"(?m)\)", TEXT, NewState::Push(vec![r"afterobject"])),
            Rule::token_to(
                r"(?m)(\d+r)?-?\d+(\.\d+)?(e-?\d+)?",
                NUMBER,
                NewState::Push(vec![r"afterobject"]),
            ),
        ],
    );
    m.insert(
        r"blockvariables",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"(""|[^"])*""#, COMMENT),
            Rule::bygroups(
                r"(?m)(:)(\s*)(\w+)",
                vec![Some(OPERATOR), Some(TEXT), Some(NAME_VARIABLE)],
            ),
            Rule::token_to(r"(?m)\|", OPERATOR, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"_parenth_helper",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"(""|[^"])*""#, COMMENT),
            Rule::token(r"(?m)(\d+r)?-?\d+(\.\d+)?(e-?\d+)?", NUMBER),
            Rule::token(r"(?m)[-+*/\\~<>=|&#!?,@%\w:]+", STRING_SYMBOL),
            Rule::token(r"(?m)'(''|[^'])*'", STRING),
            Rule::token(r"(?m)\$.", STRING_CHAR),
            Rule::token_to(
                r"(?m)#*\(",
                STRING_SYMBOL,
                NewState::Push(vec![r"inner_parenth"]),
            ),
        ],
    );
    m.insert(
        r"parenth",
        vec![
            Rule::token_to(
                r"(?m)\)",
                STRING_SYMBOL,
                NewState::Push(vec![r"root", r"afterobject"]),
            ),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"(""|[^"])*""#, COMMENT),
            Rule::token(r"(?m)(\d+r)?-?\d+(\.\d+)?(e-?\d+)?", NUMBER),
            Rule::token(r"(?m)[-+*/\\~<>=|&#!?,@%\w:]+", STRING_SYMBOL),
            Rule::token(r"(?m)'(''|[^'])*'", STRING),
            Rule::token(r"(?m)\$.", STRING_CHAR),
            Rule::token_to(
                r"(?m)#*\(",
                STRING_SYMBOL,
                NewState::Push(vec![r"inner_parenth"]),
            ),
        ],
    );
    m.insert(
        r"inner_parenth",
        vec![
            Rule::token_to(r"(?m)\)", STRING_SYMBOL, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"(""|[^"])*""#, COMMENT),
            Rule::token(r"(?m)(\d+r)?-?\d+(\.\d+)?(e-?\d+)?", NUMBER),
            Rule::token(r"(?m)[-+*/\\~<>=|&#!?,@%\w:]+", STRING_SYMBOL),
            Rule::token(r"(?m)'(''|[^'])*'", STRING),
            Rule::token(r"(?m)\$.", STRING_CHAR),
            Rule::token_to(
                r"(?m)#*\(",
                STRING_SYMBOL,
                NewState::Push(vec![r"inner_parenth"]),
            ),
        ],
    );
    m.insert(
        r"afterobject",
        vec![
            Rule::token_to(r"(?m)! !$", KEYWORD, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"(""|[^"])*""#, COMMENT),
            Rule::token_to(
                r"(?m)\b(ifTrue:|ifFalse:|whileTrue:|whileFalse:|timesRepeat:)",
                NAME_BUILTIN,
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)\b(new\b(?!:))", NAME_BUILTIN),
            Rule::token_to(r"(?m)\:=|\_", OPERATOR, NewState::Pop(1)),
            Rule::token_to(r"(?m)\b[a-zA-Z]+\w*:", NAME_FUNCTION, NewState::Pop(1)),
            Rule::token(r"(?m)\b[a-zA-Z]+\w*", NAME_FUNCTION),
            Rule::token_to(
                r"(?m)\w+:?|[-+*/\\~<>=|&!?,@%]+",
                NAME_FUNCTION,
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m)\.", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m);", PUNCTUATION),
            Rule::token(r"(?m)[\])}]", TEXT),
            Rule::token_to(r"(?m)[\[({]", TEXT, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for SmalltalkLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
