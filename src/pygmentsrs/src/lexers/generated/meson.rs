#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.meson:MesonLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.meson:MesonLexer:meson

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: meson, meson.build
pub struct MesonLexer;

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
        Rule::token(r"(?m)#.*?$", COMMENT),
        Rule::token(r"(?m)'''.*'''", STRING_SINGLE),
        Rule::token(r"(?m)[1-9][0-9]*", NUMBER_INTEGER),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)[']{3}([']{0,2}([^\\']|\\(.|\n)))*[']{3}", STRING),
        Rule::token(r"(?m)'.*?(?<!\\)(\\\\)*?'", STRING),
        Rule::token(r"(?m)(break|continue|e(?:l(?:if|se)|nd(?:foreach|if))|foreach|if)\b", KEYWORD),
        Rule::token(r"(?m)(in|and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)([\*/%\+-]=?|==|!=|=)", OPERATOR),
        Rule::token(r"(?m)[\[\]{}:().,?]", PUNCTUATION),
        Rule::token(r"(?m)((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?<!\.)(a(?:dd_(?:global_(?:(?:(?:link_)?)arguments)|languages|project_(?:(?:(?:link_)?)arguments)|test_setup)|ssert)|b(?:enchmark|oth_libraries|uild_target)|c(?:onfigur(?:ation_data|e_file)|ustom_target)|d(?:e(?:(?:(?:clare_de)?)pendency)|isabler)|e(?:nvironment|rror|xecutable)|fi(?:les|nd_(?:library|program))|ge(?:nerator|t_(?:option|variable))|i(?:n(?:clude_directories|stall_(?:data|headers|man|subdir))|s_(?:disabler|variable))|j(?:ar|oin_paths)|library|message|project|r(?:ange|un_command)|s(?:et_variable|hared_(?:library|module)|tatic_library|u(?:b(?:dir(?:(?:_done)?)|project)|mmary))|test|(?:vcs_ta|warnin)g)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)import\b", NAME_NAMESPACE),
        Rule::token(r"(?m)(build_machine|host_machine|meson|target_machine)\b", NAME_VARIABLE_MAGIC),
        Rule::token(r"(?m)[a-zA-Z_][a-zA-Z_0-9]*", NAME),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token(r"(?m)[']{3}([']{0,2}([^\\']|\\(.|\n)))*[']{3}", STRING),
            Rule::token(r"(?m)'.*?(?<!\\)(\\\\)*?'", STRING),
        ],
    );
    m.insert(
        r"keywords",
        vec![Rule::token(
            r"(?m)(break|continue|e(?:l(?:if|se)|nd(?:foreach|if))|foreach|if)\b",
            KEYWORD,
        )],
    );
    m.insert(r"expr", vec![
        Rule::token(r"(?m)(in|and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)([\*/%\+-]=?|==|!=|=)", OPERATOR),
        Rule::token(r"(?m)[\[\]{}:().,?]", PUNCTUATION),
        Rule::token(r"(?m)((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?<!\.)(a(?:dd_(?:global_(?:(?:(?:link_)?)arguments)|languages|project_(?:(?:(?:link_)?)arguments)|test_setup)|ssert)|b(?:enchmark|oth_libraries|uild_target)|c(?:onfigur(?:ation_data|e_file)|ustom_target)|d(?:e(?:(?:(?:clare_de)?)pendency)|isabler)|e(?:nvironment|rror|xecutable)|fi(?:les|nd_(?:library|program))|ge(?:nerator|t_(?:option|variable))|i(?:n(?:clude_directories|stall_(?:data|headers|man|subdir))|s_(?:disabler|variable))|j(?:ar|oin_paths)|library|message|project|r(?:ange|un_command)|s(?:et_variable|hared_(?:library|module)|tatic_library|u(?:b(?:dir(?:(?:_done)?)|project)|mmary))|test|(?:vcs_ta|warnin)g)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)import\b", NAME_NAMESPACE),
        Rule::token(r"(?m)(build_machine|host_machine|meson|target_machine)\b", NAME_VARIABLE_MAGIC),
    ]);
    m.insert(r"builtins", vec![
        Rule::token(r"(?m)(?<!\.)(a(?:dd_(?:global_(?:(?:(?:link_)?)arguments)|languages|project_(?:(?:(?:link_)?)arguments)|test_setup)|ssert)|b(?:enchmark|oth_libraries|uild_target)|c(?:onfigur(?:ation_data|e_file)|ustom_target)|d(?:e(?:(?:(?:clare_de)?)pendency)|isabler)|e(?:nvironment|rror|xecutable)|fi(?:les|nd_(?:library|program))|ge(?:nerator|t_(?:option|variable))|i(?:n(?:clude_directories|stall_(?:data|headers|man|subdir))|s_(?:disabler|variable))|j(?:ar|oin_paths)|library|message|project|r(?:ange|un_command)|s(?:et_variable|hared_(?:library|module)|tatic_library|u(?:b(?:dir(?:(?:_done)?)|project)|mmary))|test|(?:vcs_ta|warnin)g)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)import\b", NAME_NAMESPACE),
    ]);
    Table(m)
}

impl Lexer for MesonLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
