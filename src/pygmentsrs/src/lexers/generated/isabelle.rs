//! AUTO-GENERATED from `pygments.pygments.lexers.theorem:IsabelleLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.theorem:IsabelleLexer:isabelle

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: isabelle
pub struct IsabelleLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)\(\*", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token_to(r"(?m)\\<open>", STRING_SYMBOL, NewState::Push(vec![r"cartouche"])),
        Rule::token_to(r"(?m)\{\*|‹", STRING, NewState::Push(vec![r"cartouche"])),
        Rule::token(r"(?m)(::|[!()+,\-:=?\[\]_|])", OPERATOR),
        Rule::token(r"(?m)(\.\.|[.{}])", OPERATOR_WORD),
        Rule::token(r"(?m)\b(a(?:nd|ssumes|ttach|voids)|binder|c(?:hecking|lass_(?:instance|relation)|o(?:de_module|n(?:gs|st(?:ant|rains))))|d(?:(?:atatyp|efin)es)|f(?:i(?:le|xes)|or|unctions)|hints|i(?:dentifier|mports|n(?:cludes|fix(?:(?:[lr])?))|[fns])|keywords|lazy|mo(?:dule_name|(?:no|rphism)s)|no(?:(?:_discs_sel|te)s)|o(?:btains|pen|utput|verloaded)|p(?:arametric|er(?:(?:mis|va)sive))|rep_compat|s(?:hows|tructure)|type_c(?:lass|onstructor)|un(?:checked|safe)|where)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)\b(ML_(?:command|val)|c(?:(?:lass_dep|ode_(?:dep|thm))s)|display_drafts|f(?:ind_(?:(?:const|(?:theore|unused_ass)m)s)|ull_prf)|help|locale_deps|nitpick|p(?:r(?:(?:f|int_(?:ML_antiquotations|a(?:(?:bbrev|ntiquotation|ttribute)s)|b(?:(?:ind|nf|undle)s)|c(?:ase(?:(?:(?:_translation)?)s)|las(?:et|ses)|o(?:de(?:proc|setup)|ercions|mmands|ntext))|de(?:(?:fn_rul|pendenci)es)|facts|in(?:(?:duct(?:(?:_rul|iv)e)|terp)s)|locale(?:(?:s)?)|methods|o(?:(?:ption|rder)s)|quot(?:_maps|consts|ients(?:(?:Q3)?)|mapsQ3)|rules|s(?:impset|tate(?:(?:ment)?)|yntax)|t(?:erm_bindings|heor(?:ems|y)|rans_rules))|op)?)|wd)|quickcheck|refute|s(?:ledgehammer|mt_status|olve_direct|park_status)|t(?:erm|h(?:m(?:(?:_deps)?)|y_deps)|ry(?:(?:0)?)|yp)|unused_thms|value(?:(?:s(?:(?:_prolog)?))?)|welcome)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(begin|end|theory)\b", KEYWORD),
        Rule::token(r"(?m)\b(ML(?:(?:_file)?)|SML_(?:export|file|import)|a(?:bbreviation|dhoc_overloading|pproximate|rities|t(?:om_decl|tribute_setup)|xiomatization)|b(?:inds|nf_axiomatization|(?:oogie_fi|und)le)|c(?:_(?:(?:def|type)s)|a(?:rtouche|se_of_simps)|lass(?:(?:es|rel)?)|o(?:d(?:atatype|e_(?:abort|c(?:lass|onst)|datatype|i(?:dentifier|n(?:(?:clud|stanc)e))|mo(?:dulename|nad)|printing|re(?:flect|served)|type))|inductive(?:(?:_set)?)|n(?:sts|text)))|d(?:atatype(?:(?:_(?:compat|new(?:(?:_compat)?)))?)|e(?:clar(?:ation|e)|f(?:ault_sort|er_recdef|ini(?:ng|tion)|s))|omain(?:(?:_isomorphism|def)?))|e(?:quivariance|x(?:port_code|tract(?:(?:_type)?)))|f(?:ixrec|ree_constructors|un(?:(?:_cases|ctor)?))|h(?:ide_(?:c(?:lass|onst)|fact|type)|oarestate)|i(?:mport_(?:const_map|file|t(?:(?:pt|ype_ma)p))|n(?:ductive(?:(?:_set)?)|sta(?:ll_C_(?:file|types)|ntiation)))|judgment|l(?:emmas|ifting_(?:forget|update)|ocal(?:_setup|e))|me(?:msafe|thod_setup)|n(?:itpick_params|o(?:_(?:adhoc_overloading|notation|syntax|t(?:ranslations|ype_notation))|minal_(?:datatype|(?:func|termina)tion)|nterminal|t(?:ation|epad)))|o(?:racle|verloading)|p(?:ar(?:(?:se_(?:(?:(?:ast_)?)transla)|tial_func)tion)|ermanent_interpretation|ri(?:m(?:corec|rec(?:(?:_new)?))|nt_(?:(?:(?:ast_)?)translation)))|quickcheck_(?:generator|params)|re(?:aliz(?:ability|ers)|c(?:def|ord)|fute_params)|s(?:etup(?:(?:_lifting)?)|imp(?:roc_setup|s_of_case)|ledgehammer_params|mt2_status|park_(?:end|open(?:(?:_(?:siv|vcg))?)|(?:proof_function|type)s)|tatespace|yntax(?:(?:_declaration)?))|t(?:e(?:rm_cartouche|xt(?:(?:_(?:cartouche|raw))?))|heorems|ranslations|ype(?:_(?:notation|synonym)|d(?:_print_translation|ecl)))|wpc_setup)\b", KEYWORD),
        Rule::token(r"(?m)\b((?:chapt|head)er)\b", GENERIC_HEADING),
        Rule::token(r"(?m)\b(s(?:ect(?:(?:ion)?)|ubs(?:ect(?:(?:ion)?)|ubsect(?:(?:ion)?))))\b", GENERIC_SUBHEADING),
        Rule::token(r"(?m)\b(ax_specification|bnf|c(?:o(?:de_pred|rollary)|podef|runch(?:(?:_ignore)?))|enriched_type|function|in(?:stance|terpretation)|l(?:emma|ift_definition)|nominal_(?:inductive(?:(?:2)?)|primrec)|p(?:cpodef|rimcorecursive)|quotient_(?:definition|type)|re(?:cdef_tc|p_datatype)|s(?:chematic_(?:corollary|lemma|theorem)|p(?:ark_vc|ecification)|ub(?:class|locale))|t(?:ermination|heorem|ypedef)|wrap_free_constructors)\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)\b(inductive_(?:(?:case|simp)s))\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)\b(oops|sorry)\b", GENERIC_ERROR),
        Rule::token(r"(?m)\b(by|done|qed)\b", KEYWORD),
        Rule::token(r"(?m)\b(h(?:(?:av|enc)e)|interpret)\b", KEYWORD),
        Rule::token(r"(?m)\b(next|proof)\b", KEYWORD),
        Rule::token(r"(?m)\b(ML_prf|also|includ(?:e|ing)|let|moreover|note|txt(?:(?:_raw)?)|u(?:(?:nfold|s)ing)|write)\b", KEYWORD),
        Rule::token(r"(?m)\b(f(?:inally|rom)|then|ultimately|with)\b", KEYWORD),
        Rule::token(r"(?m)\b(assume|case|def|fix|presume)\b", KEYWORD),
        Rule::token(r"(?m)\b(guess|obtain|show|thus)\b", KEYWORD),
        Rule::token(r"(?m)\b(apply(?:(?:_(?:end|trace))?)|back|(?:d|pr)efer)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)\\<(\w|\^)*>", TokenType::new(&["Text", "Symbol"])),
        Rule::token(r"(?m)'[^\W\d][.\w']*", TokenType::new(&["Name", "Type"])),
        Rule::token(r"(?m)0[xX][\da-fA-F][\da-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)0[oO][0-7][0-7_]*", NUMBER_OCT),
        Rule::token(r"(?m)0[bB][01][01_]*", NUMBER_BIN),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)`", STRING_OTHER, NewState::Push(vec![r"fact"])),
        Rule::token(r"(?m)[^\s:|\[\]\-()=,+!?{}._][^\s:|\[\]\-()=,+!?{}]*", NAME),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)[^(*)]+", COMMENT),
        Rule::token_to(r"(?m)\(\*", COMMENT, NewState::PushSame),
        Rule::token_to(r"(?m)\*\)", COMMENT, NewState::Pop(1)),
        Rule::token(r"(?m)[(*)]", COMMENT),
    ]);
    m.insert(r"cartouche", vec![
        Rule::token(r"(?m)[^{*}\\‹›]+", STRING),
        Rule::token_to(r"(?m)\\<open>", STRING_SYMBOL, NewState::PushSame),
        Rule::token_to(r"(?m)\{\*|‹", STRING, NewState::PushSame),
        Rule::token_to(r"(?m)\\<close>", STRING_SYMBOL, NewState::Pop(1)),
        Rule::token_to(r"(?m)\*\}|›", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\<(\w|\^)*>", STRING_SYMBOL),
        Rule::token(r"(?m)[{*}\\]", STRING),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)[^"\\]+"#, STRING),
        Rule::token(r"(?m)\\<(\w|\^)*>", STRING_SYMBOL),
        Rule::token(r#"(?m)\\""#, STRING),
        Rule::token(r"(?m)\\", STRING),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
    ]);
    m.insert(r"fact", vec![
        Rule::token(r"(?m)[^`\\]+", STRING_OTHER),
        Rule::token(r"(?m)\\<(\w|\^)*>", STRING_SYMBOL),
        Rule::token(r"(?m)\\`", STRING_OTHER),
        Rule::token(r"(?m)\\", STRING_OTHER),
        Rule::token_to(r"(?m)`", STRING_OTHER, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for IsabelleLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
