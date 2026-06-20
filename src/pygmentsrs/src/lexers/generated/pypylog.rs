#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.console:PyPyLogLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.console:PyPyLogLexer:pypylog

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pypylog, pypy
pub struct PypylogLexer;

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
                r"(?m)\[\w+\] \{jit-log-.*?$",
                KEYWORD,
                NewState::Push(vec![r"jit-log"]),
            ),
            Rule::token_to(
                r"(?m)\[\w+\] \{jit-backend-counts$",
                KEYWORD,
                NewState::Push(vec![r"jit-backend-counts"]),
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*?$", COMMENT),
        ],
    );
    m.insert(
        r"extra-stuff",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*?$", COMMENT),
        ],
    );
    m.insert(r"jit-log", vec![
        Rule::token_to(r"(?m)\[\w+\] jit-log-.*?}$", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)^\+\d+: ", COMMENT),
        Rule::token(r"(?m)--end of the loop--", COMMENT),
        Rule::token(r"(?m)[ifp]\d+", NAME),
        Rule::token(r"(?m)ptr\d+", NAME),
        Rule::bygroups(r"(?m)(\()(\w+(?:\.\w+)?)(\))", vec![Some(PUNCTUATION), Some(NAME_BUILTIN), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[\[\]=,()]", PUNCTUATION),
        Rule::token(r"(?m)(\d+\.\d+|inf|-inf)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)'.*'", STRING),
        Rule::token(r"(?m)(None|descr|ConstClass|ConstPtr|TargetToken)", NAME),
        Rule::token(r"(?m)<.*?>+", NAME_BUILTIN),
        Rule::token(r"(?m)(label|debug_merge_point|jump|finish)", NAME_CLASS),
        Rule::token(r"(?m)(int_add_ovf|int_add|int_sub_ovf|int_sub|int_mul_ovf|int_mul|int_floordiv|int_mod|int_lshift|int_rshift|int_and|int_or|int_xor|int_eq|int_ne|int_ge|int_gt|int_le|int_lt|int_is_zero|int_is_true|uint_floordiv|uint_ge|uint_lt|float_add|float_sub|float_mul|float_truediv|float_neg|float_eq|float_ne|float_ge|float_gt|float_le|float_lt|float_abs|ptr_eq|ptr_ne|instance_ptr_eq|instance_ptr_ne|cast_int_to_float|cast_float_to_int|force_token|quasiimmut_field|same_as|virtual_ref_finish|virtual_ref|mark_opaque_ptr|call_may_force|call_assembler|call_loopinvariant|call_release_gil|call_pure|call|new_with_vtable|new_array|newstr|newunicode|new|arraylen_gc|getarrayitem_gc_pure|getarrayitem_gc|setarrayitem_gc|getarrayitem_raw|setarrayitem_raw|getfield_gc_pure|getfield_gc|getinteriorfield_gc|setinteriorfield_gc|getfield_raw|setfield_gc|setfield_raw|strgetitem|strsetitem|strlen|copystrcontent|unicodegetitem|unicodesetitem|unicodelen|guard_true|guard_false|guard_value|guard_isnull|guard_nonnull_class|guard_nonnull|guard_class|guard_no_overflow|guard_not_forced|guard_no_exception|guard_not_invalidated)", NAME_BUILTIN),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*?$", COMMENT),
    ]);
    m.insert(
        r"jit-backend-counts",
        vec![
            Rule::token_to(
                r"(?m)\[\w+\] jit-backend-counts}$",
                KEYWORD,
                NewState::Pop(1),
            ),
            Rule::token(r"(?m):", PUNCTUATION),
            Rule::token(r"(?m)\d+", NUMBER),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*?$", COMMENT),
        ],
    );
    Table(m)
}

impl Lexer for PypylogLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
