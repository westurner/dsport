#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.haskell:KokaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.haskell:KokaLexer:koka

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: koka
pub struct KokaLexer;

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
        Rule::bygroups(r"(?m)(\n\s*)(#.*)$", vec![Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)::?(?![$%&*+@!/\\^~=.:\-?|<>]+)", NAME_ATTRIBUTE, NewState::Push(vec![r"type"])),
        Rule::bygroups_to(r"(?m)(alias)(\s+)([a-z]\w*)?", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)], NewState::Push(vec![r"alias-type"])),
        Rule::bygroups_to(r"(?m)(struct)(\s+)([a-z]\w*)?", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)], NewState::Push(vec![r"struct-type"])),
        Rule::bygroups_to(r"(?m)(type|cotype|rectype|alias|struct|enum)(\s+)([a-z]\w*)?", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)], NewState::Push(vec![r"type"])),
        Rule::bygroups(r"(?m)(module)(\s+)(interface(?=\s))?(\s+)?((?:[a-z]\w*/)*[a-z]\w*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups(r"(?m)(import)(\s+)((?:[a-z]\w*/)*[a-z]\w*)(?:(\s*)(=)(\s*)(qualified)?(\s*)((?:[a-z]\w*/)*[a-z]\w*))?", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups(r"(?m)^(public|private)?(\s+)?(function|fun|val)(\s+)([a-z]\w*|\((?:[$%&*+@!/\\^~=.:\-?|<>]+|/)\))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)^(?:(public|private)(?=\s+external))?((?<!^)\s+)?(external)(\s+)(inline(?=\s))?(\s+)?([a-z]\w*|\((?:[$%&*+@!/\\^~=.:\-?|<>]+|/)\))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)(forall|exists|some|with)(?![\w/])", KEYWORD_TYPE),
        Rule::token(r"(?m)(infix|infixr|infixl|type|cotype|rectype|alias|struct|con|fun|function|val|var|external|if|then|else|elif|return|match|private|public|private|module|import|as|include|inline|rec|try|yield|enum|interface|instance)(?![\w/])", KEYWORD),
        Rule::token(r"(?m)(for|while|repeat|foreach|foreach-indexed|error|catch|finally|cs|js|file|ref|assigned)(?![\w/])", KEYWORD_PSEUDO),
        Rule::token(r"(?m)::?|:=|\->|[=.](?![$%&*+@!/\\^~=.:\-?|<>]+)", KEYWORD),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([A-Z]\w*)", vec![Some(NAME_NAMESPACE), Some(GENERIC_EMPH)]),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([a-z]\w*)", vec![Some(NAME_NAMESPACE), Some(NAME)]),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)(\((?:[$%&*+@!/\\^~=.:\-?|<>]+|/)\))", vec![Some(NAME_NAMESPACE), Some(NAME)]),
        Rule::token(r"(?m)_\w*", NAME_VARIABLE),
        Rule::token_to(r#"(?m)@""#, STRING_DOUBLE, NewState::Push(vec![r"litstring"])),
        Rule::token(r"(?m)[$%&*+@!/\\^~=.:\-?|<>]+|/(?![*/])", OPERATOR),
        Rule::token(r"(?m)`", OPERATOR),
        Rule::token(r"(?m)[{}()\[\];,]", PUNCTUATION),
        Rule::token(r"(?m)[0-9]+\.[0-9]+([eE][\-+]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"char"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::bygroups(
                r"(?m)(\n\s*)(#.*)$",
                vec![Some(WHITESPACE), Some(COMMENT_PREPROC)],
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        ],
    );
    m.insert(r"alias-type", vec![
        Rule::token(r"(?m)=", KEYWORD),
        Rule::token_to(r"(?m)[(\[<]", NAME_ATTRIBUTE, NewState::Push(vec![r"type-nested"])),
        Rule::bygroups(r"(?m)(\n\s*)(#.*)$", vec![Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)(forall|exists|some|with)(?![\w/])", KEYWORD),
        Rule::token_to(r"(?m)(?=((infix|infixr|infixl|type|cotype|rectype|alias|struct|con|fun|function|val|var|external|if|then|else|elif|return|match|private|public|private|module|import|as|include|inline|rec|try|yield|enum|interface|instance)(?![\w/])))", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)[EPHVX](?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)[a-z][0-9]*(?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)_\w*", TokenType::new(&["Name", "Attribute", "Variable"])),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([A-Z]\w*)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([a-z]\w+)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?m)::|->|[.:|]", NAME_ATTRIBUTE),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"type", vec![
        Rule::token_to(r"(?m)[(\[<]", NAME_ATTRIBUTE, NewState::Push(vec![r"type-nested"])),
        Rule::bygroups(r"(?m)(\n\s*)(#.*)$", vec![Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)(forall|exists|some|with)(?![\w/])", KEYWORD),
        Rule::token_to(r"(?m)(?=((infix|infixr|infixl|type|cotype|rectype|alias|struct|con|fun|function|val|var|external|if|then|else|elif|return|match|private|public|private|module|import|as|include|inline|rec|try|yield|enum|interface|instance)(?![\w/])))", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)[EPHVX](?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)[a-z][0-9]*(?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)_\w*", TokenType::new(&["Name", "Attribute", "Variable"])),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([A-Z]\w*)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([a-z]\w+)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?m)::|->|[.:|]", NAME_ATTRIBUTE),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"type-content", vec![
        Rule::bygroups(r"(?m)(\n\s*)(#.*)$", vec![Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)(forall|exists|some|with)(?![\w/])", KEYWORD),
        Rule::token_to(r"(?m)(?=((infix|infixr|infixl|type|cotype|rectype|alias|struct|con|fun|function|val|var|external|if|then|else|elif|return|match|private|public|private|module|import|as|include|inline|rec|try|yield|enum|interface|instance)(?![\w/])))", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)[EPHVX](?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)[a-z][0-9]*(?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)_\w*", TokenType::new(&["Name", "Attribute", "Variable"])),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([A-Z]\w*)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([a-z]\w+)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?m)::|->|[.:|]", NAME_ATTRIBUTE),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"struct-type", vec![
        Rule::token_to(r"(?m)(?=\((?!,*\)))", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)[(\[<]", NAME_ATTRIBUTE, NewState::Push(vec![r"type-nested"])),
        Rule::bygroups(r"(?m)(\n\s*)(#.*)$", vec![Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)(forall|exists|some|with)(?![\w/])", KEYWORD),
        Rule::token_to(r"(?m)(?=((infix|infixr|infixl|type|cotype|rectype|alias|struct|con|fun|function|val|var|external|if|then|else|elif|return|match|private|public|private|module|import|as|include|inline|rec|try|yield|enum|interface|instance)(?![\w/])))", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)[EPHVX](?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)[a-z][0-9]*(?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)_\w*", TokenType::new(&["Name", "Attribute", "Variable"])),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([A-Z]\w*)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([a-z]\w+)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?m)::|->|[.:|]", NAME_ATTRIBUTE),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"type-nested", vec![
        Rule::token_to(r"(?m)[)\]>]", NAME_ATTRIBUTE, NewState::Pop(1)),
        Rule::token_to(r"(?m)[(\[<]", NAME_ATTRIBUTE, NewState::Push(vec![r"type-nested"])),
        Rule::token(r"(?m),", NAME_ATTRIBUTE),
        Rule::bygroups(r"(?m)([a-z]\w*)(\s*)(:)(?!:)", vec![Some(NAME), Some(WHITESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?m)(\n\s*)(#.*)$", vec![Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)(forall|exists|some|with)(?![\w/])", KEYWORD),
        Rule::token_to(r"(?m)(?=((infix|infixr|infixl|type|cotype|rectype|alias|struct|con|fun|function|val|var|external|if|then|else|elif|return|match|private|public|private|module|import|as|include|inline|rec|try|yield|enum|interface|instance)(?![\w/])))", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)[EPHVX](?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)[a-z][0-9]*(?![\w/])", NAME_ATTRIBUTE),
        Rule::token(r"(?m)_\w*", TokenType::new(&["Name", "Attribute", "Variable"])),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([A-Z]\w*)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?m)((?:[a-z]\w*/)*)([a-z]\w+)", vec![Some(NAME_NAMESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?m)::|->|[.:|]", NAME_ATTRIBUTE),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^/*]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"litstring",
        vec![
            Rule::token(r#"(?m)[^"]+"#, STRING_DOUBLE),
            Rule::token(r#"(?m)"""#, STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)[^\\"\n]+"#, STRING_DOUBLE),
            Rule::token(r#"(?m)\\[nrt\\"\']"#, STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r"(?m)\\U[0-9a-fA-F]{6}", STRING_ESCAPE),
            Rule::token_to(r#"(?m)["\n]"#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"escape-sequence",
        vec![
            Rule::token(r#"(?m)\\[nrt\\"\']"#, STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r"(?m)\\U[0-9a-fA-F]{6}", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"char",
        vec![
            Rule::token(r"(?m)[^\\\'\n]+", STRING_CHAR),
            Rule::token(r#"(?m)\\[nrt\\"\']"#, STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r"(?m)\\U[0-9a-fA-F]{6}", STRING_ESCAPE),
            Rule::token_to(r"(?m)[\'\n]", STRING_CHAR, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for KokaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
