//! AUTO-GENERATED from `pygments.pygments.lexers.dotnet:FSharpLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dotnet:FSharpLexer:fsharp

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: fsharp, f#
pub struct FsharpLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"escape-sequence", vec![
        Rule::token(r#"(?m)\\[\\"\'ntbrafv]"#, STRING_ESCAPE),
        Rule::token(r"(?m)\\[0-9]{3}", STRING_ESCAPE),
        Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
        Rule::token(r"(?m)\\U[0-9a-fA-F]{8}", STRING_ESCAPE),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\(\)|\[\]", NAME_BUILTIN_PSEUDO),
        Rule::token_to(r"(?m)\b(?<!\.)([A-Z][\w\']*)(?=\s*\.)", NAME_NAMESPACE, NewState::Push(vec![r"dotted"])),
        Rule::token(r"(?m)\b([A-Z][\w\']*)", NAME),
        Rule::bygroups(r"(?m)(///.*?)(\n)", vec![Some(STRING_DOC), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token_to(r"(?m)\(\*(?!\))", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token_to(r#"(?m)@""#, STRING, NewState::Push(vec![r"lstring"])),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Push(vec![r"tqs"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?m)\b(open|module)(\s+)([\w.]+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups(r"(?m)\b(let!?)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_VARIABLE)]),
        Rule::bygroups(r"(?m)\b(type)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)\b(member|override)(\s+)(\w+)(\.)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME), Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)\b(abstract|as|assert|base|begin|class|default|delegate|do!|do|done|downcast|downto|elif|else|end|exception|extern|false|finally|for|function|fun|global|if|inherit|inline|interface|internal|in|lazy|let!|let|match|member|module|mutable|namespace|new|null|of|open|override|private|public|rec|return!|return|select|static|struct|then|to|true|try|type|upcast|use!|use|val|void|when|while|with|yield!|yield|atomic|break|checked|component|const|constraint|constructor|continue|eager|event|external|fixed|functor|include|method|mixin|object|parallel|process|protected|pure|sealed|tailcall|trait|virtual|volatile)\b", KEYWORD),
        Rule::token(r"(?m)``([^`\n\r\t]|`[^`\n\r\t])+``", NAME),
        Rule::token(r"(?m)(!=|#|&&|&|\(|\)|\*|\+|,|-\.|->|-|\.\.|\.|::|:=|:>|:|;;|;|<-|<\]|<|>\]|>|\?\?|\?|\[<|\[\||\[|\]|_|`|\{|\|\]|\||\}|~|<@@|<@|=|@>|@@>)", OPERATOR),
        Rule::token(r"(?m)([=<>@^|&+\*/$%-]|[!?~])?[!$%&*+\./:<=>?@^|~-]", OPERATOR),
        Rule::token(r"(?m)\b(and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)\b(sbyte|byte|char|nativeint|unativeint|float32|single|float|double|int8|uint8|int16|uint16|int32|uint32|int64|uint64|decimal|unit|bool|string|list|exn|obj|enum)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?m)(#)([ \t]*)(if|endif|else|line|nowarn|light|\d+)\b(.*?)(\n)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC), Some(COMMENT_PREPROC), Some(WHITESPACE)]),
        Rule::token(r"(?m)[^\W\d][\w']*", NAME),
        Rule::token(r"(?m)\d[\d_]*[uU]?[yslLnQRZINGmM]?", NUMBER_INTEGER),
        Rule::token(r"(?m)0[xX][\da-fA-F][\da-fA-F_]*[uU]?[yslLn]?[fF]?", NUMBER_HEX),
        Rule::token(r"(?m)0[oO][0-7][0-7_]*[uU]?[yslLn]?", NUMBER_OCT),
        Rule::token(r"(?m)0[bB][01][01_]*[uU]?[yslLn]?", NUMBER_BIN),
        Rule::token(r"(?m)-?\d[\d_]*(.[\d_]*)?([eE][+\-]?\d[\d_]*)[fFmM]?", NUMBER_FLOAT),
        Rule::token(r#"(?m)'(?:(\\[\\\"'ntbr ])|(\\[0-9]{3})|(\\x[0-9a-fA-F]{2}))'B?"#, STRING_CHAR),
        Rule::token(r"(?m)'.'", STRING_CHAR),
        Rule::token(r"(?m)'", KEYWORD),
        Rule::token_to(r#"(?m)@?""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[~?][a-z][\w\']*:", NAME_VARIABLE),
    ]);
    m.insert(r"dotted", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\.", PUNCTUATION),
        Rule::token(r"(?m)[A-Z][\w\']*(?=\s*\.)", NAME_NAMESPACE),
        Rule::token_to(r"(?m)[A-Z][\w\']*", NAME, NewState::Pop(1)),
        Rule::token_to(r"(?m)[a-z_][\w\']*", NAME, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r#"(?m)[^(*)@"]+"#, COMMENT),
        Rule::token_to(r"(?m)\(\*", COMMENT, NewState::PushSame),
        Rule::token_to(r"(?m)\*\)", COMMENT, NewState::Pop(1)),
        Rule::token_to(r#"(?m)@""#, STRING, NewState::Push(vec![r"lstring"])),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Push(vec![r"tqs"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[(*)@]", COMMENT),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)[^\\"]+"#, STRING),
        Rule::token(r#"(?m)\\[\\"\'ntbrafv]"#, STRING_ESCAPE),
        Rule::token(r"(?m)\\[0-9]{3}", STRING_ESCAPE),
        Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
        Rule::token(r"(?m)\\U[0-9a-fA-F]{8}", STRING_ESCAPE),
        Rule::token(r"(?m)\\\n", STRING),
        Rule::token(r"(?m)\n", STRING),
        Rule::token_to(r#"(?m)"B?"#, STRING, NewState::Pop(1)),
    ]);
    m.insert(r"lstring", vec![
        Rule::token(r#"(?m)[^"]+"#, STRING),
        Rule::token(r"(?m)\n", STRING),
        Rule::token(r#"(?m)"""#, STRING),
        Rule::token_to(r#"(?m)"B?"#, STRING, NewState::Pop(1)),
    ]);
    m.insert(r"tqs", vec![
        Rule::token(r#"(?m)[^"]+"#, STRING),
        Rule::token(r"(?m)\n", STRING),
        Rule::token_to(r#"(?m)"""B?"#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)""#, STRING),
    ]);
    Table(m)
}

impl Lexer for FsharpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
