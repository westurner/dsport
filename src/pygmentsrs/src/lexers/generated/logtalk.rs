#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.prolog:LogtalkLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.prolog:LogtalkLexer:logtalk

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: logtalk
pub struct LogtalkLexer;

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
        Rule::token_to(r"(?m)^\s*:-\s", PUNCTUATION, NewState::Push(vec![r"directive"])),
        Rule::token(r"(?m)%.*?\n", COMMENT),
        Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
        Rule::token(r"(?m)\n", TEXT),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)0'[\\]?.", NUMBER),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+\.?\d*((e|E)(\+|-)?\d+)?", NUMBER),
        Rule::token(r"(?m)([A-Z_][a-zA-Z0-9_]*)", NAME_VARIABLE),
        Rule::token(r"(?m)(after|before)(?=[(])", KEYWORD),
        Rule::token(r"(?m)forward(?=[(])", KEYWORD),
        Rule::token(r"(?m)(context|parameter|this|se(lf|nder))(?=[(])", KEYWORD),
        Rule::token(r"(?m)(current_predicate|predicate_property)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(expand_(goal|term)|(goal|term)_expansion|phrase)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(abolish|c(reate|urrent))_(object|protocol|category)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(object|protocol|category)_property(?=[(])", KEYWORD),
        Rule::token(r"(?m)co(mplements_object|nforms_to_protocol)(?=[(])", KEYWORD),
        Rule::token(r"(?m)extends_(object|protocol|category)(?=[(])", KEYWORD),
        Rule::token(r"(?m)imp(lements_protocol|orts_category)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(instantiat|specializ)es_class(?=[(])", KEYWORD),
        Rule::token(r"(?m)(current_event|(abolish|define)_events)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(create|current|set)_logtalk_flag(?=[(])", KEYWORD),
        Rule::token(r"(?m)logtalk_(compile|l(ibrary_path|oad|oad_context)|make(_target_action)?)(?=[(])", KEYWORD),
        Rule::token(r"(?m)\blogtalk_make\b", KEYWORD),
        Rule::token(r"(?m)(clause|retract(all)?)(?=[(])", KEYWORD),
        Rule::token(r"(?m)a(bolish|ssert(a|z))(?=[(])", KEYWORD),
        Rule::token(r"(?m)(ca(ll|tch)|throw)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(fa(il|lse)|true|(instantiation|system)_error)\b", KEYWORD),
        Rule::token(r"(?m)(uninstantiation|type|domain|existence|permission|representation|evaluation|resource|syntax)_error(?=[(])", KEYWORD),
        Rule::token(r"(?m)((bag|set)of|f(ind|or)all)(?=[(])", KEYWORD),
        Rule::token(r"(?m)threaded(_(ca(ll|ncel)|once|ignore|exit|peek|wait|notify))?(?=[(])", KEYWORD),
        Rule::token(r"(?m)threaded_engine(_(create|destroy|self|next|next_reified|yield|post|fetch))?(?=[(])", KEYWORD),
        Rule::token(r"(?m)(subsumes_term|unify_with_occurs_check)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(functor|arg|copy_term|numbervars|term_variables)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(div|rem|m(ax|in|od)|abs|sign)(?=[(])", KEYWORD),
        Rule::token(r"(?m)float(_(integer|fractional)_part)?(?=[(])", KEYWORD),
        Rule::token(r"(?m)(floor|t(an|runcate)|round|ceiling)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(cos|a(cos|sin|tan|tan2)|exp|log|s(in|qrt)|xor)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(var|atom(ic)?|integer|float|c(allable|ompound)|n(onvar|umber)|ground|acyclic_term)(?=[(])", KEYWORD),
        Rule::token(r"(?m)compare(?=[(])", KEYWORD),
        Rule::token(r"(?m)(curren|se)t_(in|out)put(?=[(])", KEYWORD),
        Rule::token(r"(?m)(open|close)(?=[(])", KEYWORD),
        Rule::token(r"(?m)flush_output(?=[(])", KEYWORD),
        Rule::token(r"(?m)(at_end_of_stream|flush_output)\b", KEYWORD),
        Rule::token(r"(?m)(stream_property|at_end_of_stream|set_stream_position)(?=[(])", KEYWORD),
        Rule::token(r"(?m)(nl|(get|peek|put)_(byte|c(har|ode)))(?=[(])", KEYWORD),
        Rule::token(r"(?m)\bnl\b", KEYWORD),
        Rule::token(r"(?m)read(_term)?(?=[(])", KEYWORD),
        Rule::token(r"(?m)write(q|_(canonical|term))?(?=[(])", KEYWORD),
        Rule::token(r"(?m)(current_)?op(?=[(])", KEYWORD),
        Rule::token(r"(?m)(current_)?char_conversion(?=[(])", KEYWORD),
        Rule::token(r"(?m)atom_(length|c(hars|o(ncat|des)))(?=[(])", KEYWORD),
        Rule::token(r"(?m)(char_code|sub_atom)(?=[(])", KEYWORD),
        Rule::token(r"(?m)number_c(har|ode)s(?=[(])", KEYWORD),
        Rule::token(r"(?m)(se|curren)t_prolog_flag(?=[(])", KEYWORD),
        Rule::token(r"(?m)\bhalt\b", KEYWORD),
        Rule::token(r"(?m)halt(?=[(])", KEYWORD),
        Rule::token(r"(?m)(::|:|\^\^)", OPERATOR),
        Rule::token(r"(?m)[{}]", KEYWORD),
        Rule::token(r"(?m)(ignore|once)(?=[(])", KEYWORD),
        Rule::token(r"(?m)\brepeat\b", KEYWORD),
        Rule::token(r"(?m)(key)?sort(?=[(])", KEYWORD),
        Rule::token(r"(?m)(>>|<<|/\\|\\\\|\\)", OPERATOR),
        Rule::token(r"(?m)\bas\b", OPERATOR),
        Rule::token(r"(?m)\bis\b", KEYWORD),
        Rule::token(r"(?m)(=:=|=\\=|<|=<|>=|>)", OPERATOR),
        Rule::token(r"(?m)=\.\.", OPERATOR),
        Rule::token(r"(?m)(=|\\=)", OPERATOR),
        Rule::token(r"(?m)(==|\\==|@=<|@<|@>=|@>)", OPERATOR),
        Rule::token(r"(?m)(//|[-+*/])", OPERATOR),
        Rule::token(r"(?m)\b(e|pi|div|mod|rem)\b", OPERATOR),
        Rule::token(r"(?m)\b\*\*\b", OPERATOR),
        Rule::token(r"(?m)-->", OPERATOR),
        Rule::token(r"(?m)([!;]|->)", OPERATOR),
        Rule::token(r"(?m)\\+", OPERATOR),
        Rule::token(r"(?m)[?@]", OPERATOR),
        Rule::token(r"(?m)\^", OPERATOR),
        Rule::token(r"(?m)[()\[\],.|]", TEXT),
        Rule::token(r"(?m)[a-z][a-zA-Z0-9_]*", TEXT),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"quoted_atom"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"double_quoted_term"])),
    ]);
    m.insert(
        r"quoted_atom",
        vec![
            Rule::token(r"(?m)''", STRING),
            Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\\([\\abfnrtv"\']|(x[a-fA-F0-9]+|[0-7]+)\\)"#,
                STRING_ESCAPE,
            ),
            Rule::token(r"(?m)[^\\'\n]+", STRING),
            Rule::token(r"(?m)\\", STRING),
        ],
    );
    m.insert(
        r"double_quoted_term",
        vec![
            Rule::token(r#"(?m)"""#, STRING),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\\([\\abfnrtv"\']|(x[a-fA-F0-9]+|[0-7]+)\\)"#,
                STRING_ESCAPE,
            ),
            Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
            Rule::token(r"(?m)\\", STRING),
        ],
    );
    m.insert(r"directive", vec![
        Rule::token_to(r"(?m)(el)?if(?=[(])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)(e(lse|ndif))(?=[.])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)(category|object|protocol)(?=[(])", KEYWORD, NewState::Push(vec![r"entityrelations"])),
        Rule::token_to(r"(?m)(end_(category|object|protocol))(?=[.])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)(public|protected|private)(?=[(])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)e(n(coding|sure_loaded)|xport)(?=[(])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)in(clude|itialization|fo)(?=[(])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)(built_in|dynamic|synchronized|threaded)(?=[.])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)(alias|d(ynamic|iscontiguous)|m(eta_(non_terminal|predicate)|ode|ultifile)|s(et_(logtalk|prolog)_flag|ynchronized))(?=[(])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)op(?=[(])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)(c(alls|oinductive)|module|reexport|use(s|_module))(?=[(])", KEYWORD, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)[a-z][a-zA-Z0-9_]*(?=[(])", TEXT, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?m)[a-z][a-zA-Z0-9_]*(?=[.])", TEXT, NewState::Push(vec![r"root"])),
    ]);
    m.insert(
        r"entityrelations",
        vec![
            Rule::token(
                r"(?m)(complements|extends|i(nstantiates|mp(lements|orts))|specializes)(?=[(])",
                KEYWORD,
            ),
            Rule::token(r"(?m)0'[\\]?.", NUMBER),
            Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
            Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
            Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
            Rule::token(r"(?m)\d+\.?\d*((e|E)(\+|-)?\d+)?", NUMBER),
            Rule::token(r"(?m)([A-Z_][a-zA-Z0-9_]*)", NAME_VARIABLE),
            Rule::token(r"(?m)[a-z][a-zA-Z0-9_]*", TEXT),
            Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"quoted_atom"])),
            Rule::token_to(
                r#"(?m)""#,
                STRING,
                NewState::Push(vec![r"double_quoted_term"]),
            ),
            Rule::token_to(r"(?m)([)]\.)", TEXT, NewState::Push(vec![r"root"])),
            Rule::token(r"(?m)(::)", OPERATOR),
            Rule::token(r"(?m)[()\[\],.|]", TEXT),
            Rule::token(r"(?m)%.*?\n", COMMENT),
            Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
            Rule::token(r"(?m)\n", TEXT),
            Rule::token(r"(?m)\s+", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for LogtalkLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
