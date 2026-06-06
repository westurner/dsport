//! AUTO-GENERATED from `pygments.pygments.lexers.asm:NasmLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asm:NasmLexer:nasm

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: nasm
pub struct NasmLexer;

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
        Rule::token_to(r"(?im)^\s*%", COMMENT_PREPROC, NewState::Push(vec![r"preproc"])),
        Rule::token(r"(?im)\n", WHITESPACE),
        Rule::token(r"(?im)[ \t]+", WHITESPACE),
        Rule::token(r"(?im);.*", COMMENT_SINGLE),
        Rule::token(r"(?im)#.*", COMMENT_SINGLE),
        Rule::token(r"(?im)[a-z$._?][\w$.?#@~]*:", NAME_LABEL),
        Rule::bygroups_to(r"(?im)([a-z$._?][\w$.?#@~]*)(\s+)(equ)", vec![Some(NAME_CONSTANT), Some(WHITESPACE), Some(KEYWORD_DECLARATION)], NewState::Push(vec![r"instruction-args"])),
        Rule::token_to(r"(?im)(?:BITS|USE16|USE32|SECTION|SEGMENT|ABSOLUTE|EXTERN|GLOBAL|ORG|ALIGN|STRUC|ENDSTRUC|COMMON|CPU|GROUP|UPPERCASE|IMPORT|EXPORT|LIBRARY|MODULE)(?=\s)", KEYWORD, NewState::Push(vec![r"instruction-args"])),
        Rule::token_to(r"(?im)(?:res|d)[bwdqt]|times", KEYWORD_DECLARATION, NewState::Push(vec![r"instruction-args"])),
        Rule::token_to(r"(?im)[a-z$._?][\w$.?#@~]*", NAME_FUNCTION, NewState::Push(vec![r"instruction-args"])),
        Rule::token(r"(?im)[\r\n]+", WHITESPACE),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?im)\n", WHITESPACE),
        Rule::token(r"(?im)[ \t]+", WHITESPACE),
        Rule::token(r"(?im);.*", COMMENT_SINGLE),
        Rule::token(r"(?im)#.*", COMMENT_SINGLE),
    ]);
    m.insert(r"instruction-args", vec![
        Rule::token(r#"(?im)"(\\"|[^"\n])*"|'(\\'|[^'\n])*'|`(\\`|[^`\n])*`"#, STRING),
        Rule::token(r"(?im)(?:0x[0-9a-f]+|$0[0-9a-f]*|[0-9]+[0-9a-f]*h)", NUMBER_HEX),
        Rule::token(r"(?im)[0-7]+q", NUMBER_OCT),
        Rule::token(r"(?im)[01]+b", NUMBER_BIN),
        Rule::token(r"(?im)[0-9]+\.e?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?im)[,{}():\[\]]+", PUNCTUATION),
        Rule::token(r"(?im)[&|^<>+*/%~-]+", OPERATOR),
        Rule::token(r"(?im)[$]+", KEYWORD_CONSTANT),
        Rule::token(r"(?im)seg|wrt|strict|rel|abs", OPERATOR_WORD),
        Rule::token(r"(?im)byte|[dq]?word", KEYWORD_TYPE),
        Rule::token(r"(?im)(r[0-9][0-5]?[bwd]?|[a-d][lh]|[er]?[a-d]x|[er]?[sb]p|[er]?[sd]i|[c-gs]s|st[0-7]|mm[0-7]|cr[0-4]|dr[0-367]|tr[3-7]|k[0-7]|[xyz]mm(?:[12][0-9]?|3[01]?|[04-9]))\b", NAME_BUILTIN),
        Rule::token(r"(?im)[a-z$._?][\w$.?#@~]*", NAME_VARIABLE),
        Rule::token_to(r"(?im)[\r\n]+", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?im)\n", WHITESPACE),
        Rule::token(r"(?im)[ \t]+", WHITESPACE),
        Rule::token(r"(?im);.*", COMMENT_SINGLE),
        Rule::token(r"(?im)#.*", COMMENT_SINGLE),
    ]);
    m.insert(r"punctuation", vec![
        Rule::token(r"(?im)[,{}():\[\]]+", PUNCTUATION),
        Rule::token(r"(?im)[&|^<>+*/%~-]+", OPERATOR),
        Rule::token(r"(?im)[$]+", KEYWORD_CONSTANT),
        Rule::token(r"(?im)seg|wrt|strict|rel|abs", OPERATOR_WORD),
        Rule::token(r"(?im)byte|[dq]?word", KEYWORD_TYPE),
    ]);
    m.insert(r"preproc", vec![
        Rule::token(r"(?im)[^;\n]+", COMMENT_PREPROC),
        Rule::token_to(r"(?im);.*?\n", COMMENT_SINGLE, NewState::Pop(1)),
        Rule::token_to(r"(?im)\n", COMMENT_PREPROC, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for NasmLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
