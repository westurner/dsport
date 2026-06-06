//! AUTO-GENERATED from `pygments.pygments.lexers.nimrod:NimrodLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.nimrod:NimrodLexer:nimrod

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: nimrod, nim
pub struct NimrodLexer;

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
        Rule::token_to(r"(?im)##\[", STRING_DOC, NewState::Push(vec![r"doccomment"])),
        Rule::token(r"(?im)##.*$", STRING_DOC),
        Rule::token_to(r"(?im)#\[", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?im)#.*$", COMMENT),
        Rule::token_to(r"(?im)\{\.", STRING_OTHER, NewState::Push(vec![r"pragma"])),
        Rule::token(r"(?im)[*=><+\-/@$~&%!?|\\\[\]]", OPERATOR),
        Rule::token(r"(?im)\.\.|\.|,|\[\.|\.\]|\{\.|\.\}|\(\.|\.\)|\{|\}|\(|\)|:|\^|`|;", PUNCTUATION),
        Rule::bygroups_to(r"(?im)(\n\s*)(of)(\s)", vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"casebranch"])),
        Rule::token_to(r#"(?im)(?:[\w]+)""#, STRING, NewState::Push(vec![r"rdqs"])),
        Rule::token_to(r#"(?im)""""#, STRING_DOUBLE, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?im)'", STRING_CHAR, NewState::Push(vec![r"chars"])),
        Rule::token(r"(?im)(a_?n_?d_?|o_?r_?|n_?o_?t_?|x_?o_?r_?|s_?h_?l_?|s_?h_?r_?|d_?i_?v_?|m_?o_?d_?|i_?n_?|n_?o_?t_?i_?n_?|i_?s_?|i_?s_?n_?o_?t_?)\b", OPERATOR_WORD),
        Rule::bygroups_to(r"(?im)(proc|func|method|macro|template)(\s)(?![(\[\]])", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"funcname"])),
        Rule::token(r"(?im)(a_?d_?d_?r_?|a_?n_?d_?|a_?s_?|a_?s_?m_?|b_?i_?n_?d_?|b_?l_?o_?c_?k_?|b_?r_?e_?a_?k_?|c_?a_?s_?e_?|c_?a_?s_?t_?|c_?o_?n_?c_?e_?p_?t_?|c_?o_?n_?s_?t_?|c_?o_?n_?t_?i_?n_?u_?e_?|c_?o_?n_?v_?e_?r_?t_?e_?r_?|d_?e_?f_?e_?r_?|d_?i_?s_?c_?a_?r_?d_?|d_?i_?s_?t_?i_?n_?c_?t_?|d_?i_?v_?|d_?o_?|e_?l_?i_?f_?|e_?l_?s_?e_?|e_?n_?d_?|e_?n_?u_?m_?|e_?x_?c_?e_?p_?t_?|e_?x_?p_?o_?r_?t_?|f_?i_?n_?a_?l_?l_?y_?|f_?o_?r_?|i_?f_?|i_?n_?|y_?i_?e_?l_?d_?|i_?n_?t_?e_?r_?f_?a_?c_?e_?|i_?s_?|i_?s_?n_?o_?t_?|i_?t_?e_?r_?a_?t_?o_?r_?|l_?e_?t_?|m_?i_?x_?i_?n_?|m_?o_?d_?|n_?o_?t_?|n_?o_?t_?i_?n_?|o_?b_?j_?e_?c_?t_?|o_?f_?|o_?r_?|o_?u_?t_?|p_?t_?r_?|r_?a_?i_?s_?e_?|r_?e_?f_?|r_?e_?t_?u_?r_?n_?|s_?h_?l_?|s_?h_?r_?|s_?t_?a_?t_?i_?c_?|t_?r_?y_?|t_?u_?p_?l_?e_?|t_?y_?p_?e_?|u_?s_?i_?n_?g_?|w_?h_?e_?n_?|w_?h_?i_?l_?e_?|x_?o_?r_?)\b", KEYWORD),
        Rule::token(r"(?im)(f_?r_?o_?m_?|i_?m_?p_?o_?r_?t_?|i_?n_?c_?l_?u_?d_?e_?|e_?x_?p_?o_?r_?t_?)\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?im)(v_?a_?r)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?im)(i_?n_?t_?|i_?n_?t_?8_?|i_?n_?t_?1_?6_?|i_?n_?t_?3_?2_?|i_?n_?t_?6_?4_?|f_?l_?o_?a_?t_?|f_?l_?o_?a_?t_?3_?2_?|f_?l_?o_?a_?t_?6_?4_?|b_?o_?o_?l_?|c_?h_?a_?r_?|r_?a_?n_?g_?e_?|a_?r_?r_?a_?y_?|s_?e_?q_?|s_?e_?t_?|s_?t_?r_?i_?n_?g_?)\b", NAME_BUILTIN),
        Rule::token(r"(?im)(n_?i_?l_?|t_?r_?u_?e_?|f_?a_?l_?s_?e_?)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?im)\b((?![_\d])\w)(((?!_)\w)|(_(?!_)\w))*", NAME),
        Rule::token_to(r"(?im)[0-9][0-9_]*(?=([e.]|\'f(32|64)))", NUMBER_FLOAT, NewState::Push(vec![r"float-suffix", r"float-number"])),
        Rule::token_to(r"(?im)0x[a-f0-9][a-f0-9_]*", NUMBER_HEX, NewState::Push(vec![r"int-suffix"])),
        Rule::token_to(r"(?im)0b[01][01_]*", NUMBER_BIN, NewState::Push(vec![r"int-suffix"])),
        Rule::token_to(r"(?im)0o[0-7][0-7_]*", NUMBER_OCT, NewState::Push(vec![r"int-suffix"])),
        Rule::token_to(r"(?im)[0-9][0-9_]*", NUMBER_INTEGER, NewState::Push(vec![r"int-suffix"])),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im).+$", ERROR),
    ]);
    m.insert(r"chars", vec![
        Rule::token(r#"(?im)\\([\\abcefnrtvl"\']|x[a-f0-9]{2}|[0-9]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?im)'", STRING_CHAR, NewState::Pop(1)),
        Rule::token(r"(?im).", STRING_CHAR),
    ]);
    m.insert(r"strings", vec![
        Rule::token(r"(?im)(?<!\$)\$(\d+|#|\w+)+", STRING_INTERPOL),
        Rule::token(r#"(?im)[^\\\'"$\n]+"#, STRING),
        Rule::token(r#"(?im)[\'"\\]"#, STRING),
        Rule::token(r"(?im)\$", STRING),
    ]);
    m.insert(r"doccomment", vec![
        Rule::token(r"(?im)[^\]#]+", STRING_DOC),
        Rule::token_to(r"(?im)##\[", STRING_DOC, NewState::PushSame),
        Rule::token_to(r"(?im)\]##", STRING_DOC, NewState::Pop(1)),
        Rule::token(r"(?im)[\]#]", STRING_DOC),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?im)[^\]#]+", COMMENT_MULTILINE),
        Rule::token_to(r"(?im)#\[", COMMENT_MULTILINE, NewState::PushSame),
        Rule::token_to(r"(?im)\]#", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?im)[\]#]", COMMENT_MULTILINE),
    ]);
    m.insert(r"dqs", vec![
        Rule::token(r#"(?im)\\([\\abcefnrtvl"\']|\n|x[a-f0-9]{2}|[0-9]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?im)(?<!\$)\$(\d+|#|\w+)+", STRING_INTERPOL),
        Rule::token(r#"(?im)[^\\\'"$\n]+"#, STRING),
        Rule::token(r#"(?im)[\'"\\]"#, STRING),
        Rule::token(r"(?im)\$", STRING),
    ]);
    m.insert(r"rdqs", vec![
        Rule::token_to(r#"(?im)"(?!")"#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?im)"""#, STRING_ESCAPE),
        Rule::token(r"(?im)(?<!\$)\$(\d+|#|\w+)+", STRING_INTERPOL),
        Rule::token(r#"(?im)[^\\\'"$\n]+"#, STRING),
        Rule::token(r#"(?im)[\'"\\]"#, STRING),
        Rule::token(r"(?im)\$", STRING),
    ]);
    m.insert(r"tdqs", vec![
        Rule::token_to(r#"(?im)""""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r"(?im)(?<!\$)\$(\d+|#|\w+)+", STRING_INTERPOL),
        Rule::token(r#"(?im)[^\\\'"$\n]+"#, STRING),
        Rule::token(r#"(?im)[\'"\\]"#, STRING),
        Rule::token(r"(?im)\$", STRING),
        Rule::token(r"(?im)\n", STRING_DOUBLE),
    ]);
    m.insert(r"funcname", vec![
        Rule::token_to(r"(?im)((?![\d_])\w)(((?!_)\w)|(_(?!_)\w))*", NAME_FUNCTION, NewState::Pop(1)),
        Rule::token_to(r"(?im)`.+`", NAME_FUNCTION, NewState::Pop(1)),
    ]);
    m.insert(r"nl", vec![
        Rule::token(r"(?im)\n", STRING),
    ]);
    m.insert(r"float-number", vec![
        Rule::token(r"(?im)\.(?!\.)[0-9_]*[f]*", NUMBER_FLOAT),
        Rule::token(r"(?im)e[+-]?[0-9][0-9_]*", NUMBER_FLOAT),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"float-suffix", vec![
        Rule::token(r"(?im)\'f(32|64)", NUMBER_FLOAT),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"int-suffix", vec![
        Rule::token(r"(?im)\'i(32|64)", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?im)\'i(8|16)", NUMBER_INTEGER),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"casebranch", vec![
        Rule::token(r"(?im),", PUNCTUATION),
        Rule::token(r"(?im)[\n ]+", WHITESPACE),
        Rule::token_to(r"(?im):", OPERATOR, NewState::Pop(1)),
        Rule::token(r"(?im)\w+|[^:]", NAME_LABEL),
    ]);
    m.insert(r"pragma", vec![
        Rule::token(r"(?im)[:,]", TEXT),
        Rule::token(r"(?im)[\n ]+", WHITESPACE),
        Rule::token_to(r"(?im)\.\}", STRING_OTHER, NewState::Pop(1)),
        Rule::token(r"(?im)\w+|\W+|[^.}]", STRING_OTHER),
    ]);
    Table(m)
}

impl Lexer for NimrodLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
