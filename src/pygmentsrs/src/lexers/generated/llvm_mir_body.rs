#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.asm:LlvmMirBodyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asm:LlvmMirBodyLexer:llvm_mir_body

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: llvm-mir-body
pub struct LlvmMirBodyLexer;

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
        Rule::token(r"(?m)((?:livein|successor)s):", KEYWORD),
        Rule::token(r"(?m)bb\.[0-9]+(\.[a-zA-Z0-9_.-]+)?( \(address-taken\))?:", NAME_LABEL),
        Rule::token(r"(?m)bb\.[0-9]+ \(%[a-zA-Z0-9_.-]+\)( \(address-taken\))?:", NAME_LABEL),
        Rule::token(r"(?m)%bb\.[0-9]+(\.\w+)?", NAME_LABEL),
        Rule::token(r"(?m)%stack\.[0-9]+(\.\w+\.addr)?", NAME),
        Rule::token(r"(?m)%subreg\.\w+", NAME),
        Rule::token_to(r"(?m)%[a-zA-Z0-9_]+ *", NAME_VARIABLE, NewState::Push(vec![r"vreg"])),
        Rule::token(r"(?m)\@[a-zA-Z0-9_.]+", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)intrinsic\(\@[a-zA-Z0-9_.]+\)", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)intpred\((eq|ne|s(?:g(?:[et])|l(?:[et]))|u(?:g(?:[et])|l(?:[et])))\)", NAME_BUILTIN),
        Rule::token(r"(?m)floatpred\((o(?:eq|g(?:[et])|l(?:[et])|ne)|u(?:g(?:[et])|l(?:[et])))\)", NAME_BUILTIN),
        Rule::token(r"(?m)\$\w+", STRING_SINGLE),
        Rule::token(r"(?m)=", OPERATOR),
        Rule::token(r"(?m)(G_ANYEXT|G_[SZ]EXT|G_SEXT_INREG|G_TRUNC|G_IMPLICIT_DEF|G_PHI|G_FRAME_INDEX|G_GLOBAL_VALUE|G_INTTOPTR|G_PTRTOINT|G_BITCAST|G_CONSTANT|G_FCONSTANT|G_VASTART|G_VAARG|G_CTLZ|G_CTLZ_ZERO_UNDEF|G_CTTZ|G_CTTZ_ZERO_UNDEF|G_CTPOP|G_BSWAP|G_BITREVERSE|G_ADDRSPACE_CAST|G_BLOCK_ADDR|G_JUMP_TABLE|G_DYN_STACKALLOC|G_ADD|G_SUB|G_MUL|G_[SU]DIV|G_[SU]REM|G_AND|G_OR|G_XOR|G_SHL|G_[LA]SHR|G_[IF]CMP|G_SELECT|G_GEP|G_PTR_MASK|G_SMIN|G_SMAX|G_UMIN|G_UMAX|G_[US]ADDO|G_[US]ADDE|G_[US]SUBO|G_[US]SUBE|G_[US]MULO|G_[US]MULH|G_FNEG|G_FPEXT|G_FPTRUNC|G_FPTO[US]I|G_[US]ITOFP|G_FABS|G_FCOPYSIGN|G_FCANONICALIZE|G_FMINNUM|G_FMAXNUM|G_FMINNUM_IEEE|G_FMAXNUM_IEEE|G_FMINIMUM|G_FMAXIMUM|G_FADD|G_FSUB|G_FMUL|G_FMA|G_FMAD|G_FDIV|G_FREM|G_FPOW|G_FEXP|G_FEXP2|G_FLOG|G_FLOG2|G_FLOG10|G_FCEIL|G_FCOS|G_FSIN|G_FSQRT|G_FFLOOR|G_FRINT|G_FNEARBYINT|G_INTRINSIC_TRUNC|G_INTRINSIC_ROUND|G_LOAD|G_[ZS]EXTLOAD|G_INDEXED_LOAD|G_INDEXED_[ZS]EXTLOAD|G_STORE|G_INDEXED_STORE|G_ATOMIC_CMPXCHG_WITH_SUCCESS|G_ATOMIC_CMPXCHG|G_ATOMICRMW_(XCHG|ADD|SUB|AND|NAND|OR|XOR|MAX|MIN|UMAX|UMIN|FADD|FSUB)|G_FENCE|G_EXTRACT|G_UNMERGE_VALUES|G_INSERT|G_MERGE_VALUES|G_BUILD_VECTOR|G_BUILD_VECTOR_TRUNC|G_CONCAT_VECTORS|G_INTRINSIC|G_INTRINSIC_W_SIDE_EFFECTS|G_BR|G_BRCOND|G_BRINDIRECT|G_BRJT|G_INSERT_VECTOR_ELT|G_EXTRACT_VECTOR_ELT|G_SHUFFLE_VECTOR)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(COPY|PHI|INSERT_SUBREG|EXTRACT_SUBREG|REG_SEQUENCE)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(implicit|killed)", KEYWORD),
        Rule::bygroups_to(r"(?m)(i[0-9]+)( +)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE)], NewState::Push(vec![r"constantint"])),
        Rule::token_to(r"(?m)(half|float|double) +", KEYWORD_TYPE, NewState::Push(vec![r"constantfloat"])),
        Rule::token(r"(?m)-?[0-9]+", NUMBER_INTEGER),
        Rule::bygroups_to(r"(?m)(::)( *)", vec![Some(OPERATOR), Some(WHITESPACE)], NewState::Push(vec![r"mmo"])),
        Rule::token(r"(?m);.*", COMMENT),
        Rule::token(r"(?m)[a-zA-Z0-9_]+", NAME),
        Rule::token(r"(?m)[(), \n]+", TEXT),
    ]);
    m.insert(
        r"global",
        vec![Rule::token(r"(?m)\@[a-zA-Z0-9_.]+", NAME_VARIABLE_GLOBAL)],
    );
    m.insert(
        r"integer",
        vec![Rule::token(r"(?m)-?[0-9]+", NUMBER_INTEGER)],
    );
    m.insert(
        r"constantint",
        vec![
            Rule::token(r"(?m)-?[0-9]+", NUMBER_INTEGER),
            Rule::token_to(r"(?m)(?=.)", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"constantfloat",
        vec![
            Rule::token(r"(?m)-?[0-9]+\.[0-9]+(e[+-][0-9]+)?", NUMBER_FLOAT),
            Rule::token_to(r"(?m)(?=.)", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"float",
        vec![Rule::token(
            r"(?m)-?[0-9]+\.[0-9]+(e[+-][0-9]+)?",
            NUMBER_FLOAT,
        )],
    );
    m.insert(
        r"vreg",
        vec![
            Rule::bygroups_to(
                r"(?m)( *)(:(?!:))",
                vec![Some(WHITESPACE), Some(KEYWORD)],
                NewState::Push(vec![r"#pop", r"vreg_bank_or_class"]),
            ),
            Rule::bygroups_to(
                r"(?m)( *)(\()",
                vec![Some(WHITESPACE), Some(TEXT)],
                NewState::Push(vec![r"vreg_type"]),
            ),
            Rule::token_to(r"(?m)(?=.)", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"vreg_bank_or_class",
        vec![
            Rule::bygroups(
                r"(?m)( *)(_)",
                vec![Some(WHITESPACE), Some(NAME_VARIABLE_MAGIC)],
            ),
            Rule::bygroups(
                r"(?m)( *)([a-zA-Z0-9_]+)",
                vec![Some(WHITESPACE), Some(NAME_VARIABLE)],
            ),
            Rule::bygroups_to(
                r"(?m)( *)(\()",
                vec![Some(WHITESPACE), Some(TEXT)],
                NewState::Push(vec![r"vreg_type"]),
            ),
            Rule::token_to(r"(?m)(?=.)", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"vreg_type",
        vec![
            Rule::bygroups(
                r"(?m)( *)([sp][0-9]+)",
                vec![Some(WHITESPACE), Some(KEYWORD_TYPE)],
            ),
            Rule::bygroups(
                r"(?m)( *)(<[0-9]+ *x *[sp][0-9]+>)",
                vec![Some(WHITESPACE), Some(KEYWORD_TYPE)],
            ),
            Rule::token_to(r"(?m)\)", TEXT, NewState::Pop(1)),
            Rule::token_to(r"(?m)(?=.)", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(r"mmo", vec![
        Rule::token(r"(?m)\(", TEXT),
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token(r"(?m)(a(?:cq(?:_rel|uire)|lign)|from|into|load|monotonic|on|release|s(?:eq_cst|tore))", KEYWORD),
        Rule::token(r"(?m)%ir\.[a-zA-Z0-9_.-]+", NAME),
        Rule::token(r"(?m)%ir-block\.[a-zA-Z0-9_.-]+", NAME),
        Rule::token(r"(?m)[-+]", OPERATOR),
        Rule::token(r"(?m)-?[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)\@[a-zA-Z0-9_.]+", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)\), \(", TEXT),
        Rule::token_to(r"(?m)\)", TEXT, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for LlvmMirBodyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
