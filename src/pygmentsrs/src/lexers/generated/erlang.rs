#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.erlang:ErlangLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.erlang:ErlangLexer:erlang

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: erlang
pub struct ErlangLexer;

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
        Rule::bygroups(r"(?m)(%.*)(\n)", vec![Some(COMMENT), Some(WHITESPACE)]),
        Rule::token(r"(?m)(after|begin|c(?:a(?:se|tch)|ond)|end|fun|if|let|of|query|receive|try|when)\b", KEYWORD),
        Rule::token(r"(?m)(a(?:bs|pp(?:end_element|ly)|tom_to_list)|b(?:i(?:nary_to_(?:list|term)|t(?:_size|string_to_list))|ump_reductions|yte_size)|c(?:ancel_timer|heck_process_code)|d(?:e(?:lete_module|monitor)|is(?:connect_node|play))|e(?:lement|rase|xit)|f(?:loat(?:(?:_to_list)?)|un(?:_(?:info|to_list)|ction_exported))|g(?:arbage_collect|et(?:(?:_keys)?)|roup_leader)|h(?:ash|d)|i(?:nteger_to_list|olist_(?:size|to_binary)|s_(?:atom|b(?:i(?:nary|tstring)|(?:oolea|uilti)n)|f(?:loat|unction)|integer|list|number|p(?:id|ort|rocess_alive)|re(?:cord|ference)|tuple))|l(?:ength|i(?:nk|st_to_(?:atom|bi(?:nary|tstring)|existing_atom|float|integer|pid|tuple))|o(?:(?:ad_modul|caltime_to_universaltim)e))|m(?:ake_tuple|d5(?:(?:_(?:final|update))?)|emory|o(?:dule_loaded|nitor(?:(?:_node)?)))|node(?:(?:s)?)|open_port|p(?:hash(?:(?:2)?)|id_to_list|ort_(?:c(?:all|lose|o(?:mmand|n(?:nect|trol)))|info|to_list)|rocess_(?:display|flag|info)|u(?:rge_module|t))|r(?:e(?:ad_timer|f_to_list|gister|sume_process)|ound)|s(?:e(?:nd(?:(?:_(?:after|nosuspend))?)|t(?:_cookie|element))|ize|p(?:awn(?:(?:_(?:link|monitor|opt))?)|lit_binary)|ta(?:rt_timer|tistics)|uspend_process|ystem_(?:flag|info|monitor|profile))|t(?:erm_to_binary|l|r(?:ace(?:(?:_(?:delivered|info|pattern))?)|unc)|uple_(?:size|to_list))|un(?:iversaltime_to_localtime|link|register)|whereis)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(and(?:(?:also)?)|b(?:and|not|or|s(?:[lr])|xor)|div|not|or(?:(?:else)?)|rem|xor)\b", OPERATOR_WORD),
        Rule::token_to(r"(?m)^-", PUNCTUATION, NewState::Push(vec![r"directive"])),
        Rule::token(r"(?m)(\+\+?|--?|\*|/|<|>|/=|=:=|=/=|=<|>=|==?|<-|!|\?)", OPERATOR),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)<<", NAME_LABEL),
        Rule::token(r"(?m)>>", NAME_LABEL),
        Rule::bygroups(r"(?m)((?:[a-z]\w*|'[^\n']*[^\\]'))(:)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?:^|(?<=:))((?:[a-z]\w*|'[^\n']*[^\\]'))(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[+-]?(?:[2-9]|[12][0-9]|3[0-6])#[0-9a-zA-Z]+", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-]?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-]?\d+.\d+", NUMBER_FLOAT),
        Rule::token(r#"(?m)[]\[:_@\".{}()|;,]"#, PUNCTUATION),
        Rule::token(r"(?m)(?:[A-Z_]\w*)", NAME_VARIABLE),
        Rule::token(r"(?m)(?:[a-z]\w*|'[^\n']*[^\\]')", NAME),
        Rule::token(r"(?m)\?(?:(?:[A-Z_]\w*)|(?:[a-z]\w*|'[^\n']*[^\\]'))", NAME_CONSTANT),
        Rule::token(r#"(?m)\$(?:(?:\\(?:[bdefnrstv\'"\\]|[0-7][0-7]?[0-7]?|(?:x[0-9a-fA-F]{2}|x\{[0-9a-fA-F]+\})|\^[a-zA-Z]))|\\[ %]|[^\\])"#, STRING_CHAR),
        Rule::token(r"(?m)#(?:[a-z]\w*|'[^\n']*[^\\]')(:?\.(?:[a-z]\w*|'[^\n']*[^\\]'))?", NAME_LABEL),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"map_key"])),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)(?:\\(?:[bdefnrstv\'"\\]|[0-7][0-7]?[0-7]?|(?:x[0-9a-fA-F]{2}|x\{[0-9a-fA-F]+\})|\^[a-zA-Z]))"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?m)~[0-9.*]*[~#+BPWXb-ginpswx]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^"\\~]+"#, STRING),
        Rule::token(r"(?m)~", STRING),
    ]);
    m.insert(
        r"directive",
        vec![
            Rule::bygroups_to(
                r"(?m)(define)(\s*)(\()((?:(?:[A-Z_]\w*)|(?:[a-z]\w*|'[^\n']*[^\\]')))",
                vec![
                    Some(NAME_ENTITY),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(NAME_CONSTANT),
                ],
                NewState::Pop(1),
            ),
            Rule::bygroups_to(
                r"(?m)(record)(\s*)(\()((?:(?:[A-Z_]\w*)|(?:[a-z]\w*|'[^\n']*[^\\]')))",
                vec![
                    Some(NAME_ENTITY),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(NAME_LABEL),
                ],
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)(?:[a-z]\w*|'[^\n']*[^\\]')",
                NAME_ENTITY,
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(r"map_key", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(%.*)(\n)", vec![Some(COMMENT), Some(WHITESPACE)]),
        Rule::token(r"(?m)(after|begin|c(?:a(?:se|tch)|ond)|end|fun|if|let|of|query|receive|try|when)\b", KEYWORD),
        Rule::token(r"(?m)(a(?:bs|pp(?:end_element|ly)|tom_to_list)|b(?:i(?:nary_to_(?:list|term)|t(?:_size|string_to_list))|ump_reductions|yte_size)|c(?:ancel_timer|heck_process_code)|d(?:e(?:lete_module|monitor)|is(?:connect_node|play))|e(?:lement|rase|xit)|f(?:loat(?:(?:_to_list)?)|un(?:_(?:info|to_list)|ction_exported))|g(?:arbage_collect|et(?:(?:_keys)?)|roup_leader)|h(?:ash|d)|i(?:nteger_to_list|olist_(?:size|to_binary)|s_(?:atom|b(?:i(?:nary|tstring)|(?:oolea|uilti)n)|f(?:loat|unction)|integer|list|number|p(?:id|ort|rocess_alive)|re(?:cord|ference)|tuple))|l(?:ength|i(?:nk|st_to_(?:atom|bi(?:nary|tstring)|existing_atom|float|integer|pid|tuple))|o(?:(?:ad_modul|caltime_to_universaltim)e))|m(?:ake_tuple|d5(?:(?:_(?:final|update))?)|emory|o(?:dule_loaded|nitor(?:(?:_node)?)))|node(?:(?:s)?)|open_port|p(?:hash(?:(?:2)?)|id_to_list|ort_(?:c(?:all|lose|o(?:mmand|n(?:nect|trol)))|info|to_list)|rocess_(?:display|flag|info)|u(?:rge_module|t))|r(?:e(?:ad_timer|f_to_list|gister|sume_process)|ound)|s(?:e(?:nd(?:(?:_(?:after|nosuspend))?)|t(?:_cookie|element))|ize|p(?:awn(?:(?:_(?:link|monitor|opt))?)|lit_binary)|ta(?:rt_timer|tistics)|uspend_process|ystem_(?:flag|info|monitor|profile))|t(?:erm_to_binary|l|r(?:ace(?:(?:_(?:delivered|info|pattern))?)|unc)|uple_(?:size|to_list))|un(?:iversaltime_to_localtime|link|register)|whereis)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(and(?:(?:also)?)|b(?:and|not|or|s(?:[lr])|xor)|div|not|or(?:(?:else)?)|rem|xor)\b", OPERATOR_WORD),
        Rule::token_to(r"(?m)^-", PUNCTUATION, NewState::Push(vec![r"directive"])),
        Rule::token(r"(?m)(\+\+?|--?|\*|/|<|>|/=|=:=|=/=|=<|>=|==?|<-|!|\?)", OPERATOR),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)<<", NAME_LABEL),
        Rule::token(r"(?m)>>", NAME_LABEL),
        Rule::bygroups(r"(?m)((?:[a-z]\w*|'[^\n']*[^\\]'))(:)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?:^|(?<=:))((?:[a-z]\w*|'[^\n']*[^\\]'))(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[+-]?(?:[2-9]|[12][0-9]|3[0-6])#[0-9a-zA-Z]+", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-]?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-]?\d+.\d+", NUMBER_FLOAT),
        Rule::token(r#"(?m)[]\[:_@\".{}()|;,]"#, PUNCTUATION),
        Rule::token(r"(?m)(?:[A-Z_]\w*)", NAME_VARIABLE),
        Rule::token(r"(?m)(?:[a-z]\w*|'[^\n']*[^\\]')", NAME),
        Rule::token(r"(?m)\?(?:(?:[A-Z_]\w*)|(?:[a-z]\w*|'[^\n']*[^\\]'))", NAME_CONSTANT),
        Rule::token(r#"(?m)\$(?:(?:\\(?:[bdefnrstv\'"\\]|[0-7][0-7]?[0-7]?|(?:x[0-9a-fA-F]{2}|x\{[0-9a-fA-F]+\})|\^[a-zA-Z]))|\\[ %]|[^\\])"#, STRING_CHAR),
        Rule::token(r"(?m)#(?:[a-z]\w*|'[^\n']*[^\\]')(:?\.(?:[a-z]\w*|'[^\n']*[^\\]'))?", NAME_LABEL),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"map_key"])),
        Rule::token_to(r"(?m)=>", PUNCTUATION, NewState::Push(vec![r"map_val"])),
        Rule::token_to(r"(?m):=", PUNCTUATION, NewState::Push(vec![r"map_val"])),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"map_val", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(%.*)(\n)", vec![Some(COMMENT), Some(WHITESPACE)]),
        Rule::token(r"(?m)(after|begin|c(?:a(?:se|tch)|ond)|end|fun|if|let|of|query|receive|try|when)\b", KEYWORD),
        Rule::token(r"(?m)(a(?:bs|pp(?:end_element|ly)|tom_to_list)|b(?:i(?:nary_to_(?:list|term)|t(?:_size|string_to_list))|ump_reductions|yte_size)|c(?:ancel_timer|heck_process_code)|d(?:e(?:lete_module|monitor)|is(?:connect_node|play))|e(?:lement|rase|xit)|f(?:loat(?:(?:_to_list)?)|un(?:_(?:info|to_list)|ction_exported))|g(?:arbage_collect|et(?:(?:_keys)?)|roup_leader)|h(?:ash|d)|i(?:nteger_to_list|olist_(?:size|to_binary)|s_(?:atom|b(?:i(?:nary|tstring)|(?:oolea|uilti)n)|f(?:loat|unction)|integer|list|number|p(?:id|ort|rocess_alive)|re(?:cord|ference)|tuple))|l(?:ength|i(?:nk|st_to_(?:atom|bi(?:nary|tstring)|existing_atom|float|integer|pid|tuple))|o(?:(?:ad_modul|caltime_to_universaltim)e))|m(?:ake_tuple|d5(?:(?:_(?:final|update))?)|emory|o(?:dule_loaded|nitor(?:(?:_node)?)))|node(?:(?:s)?)|open_port|p(?:hash(?:(?:2)?)|id_to_list|ort_(?:c(?:all|lose|o(?:mmand|n(?:nect|trol)))|info|to_list)|rocess_(?:display|flag|info)|u(?:rge_module|t))|r(?:e(?:ad_timer|f_to_list|gister|sume_process)|ound)|s(?:e(?:nd(?:(?:_(?:after|nosuspend))?)|t(?:_cookie|element))|ize|p(?:awn(?:(?:_(?:link|monitor|opt))?)|lit_binary)|ta(?:rt_timer|tistics)|uspend_process|ystem_(?:flag|info|monitor|profile))|t(?:erm_to_binary|l|r(?:ace(?:(?:_(?:delivered|info|pattern))?)|unc)|uple_(?:size|to_list))|un(?:iversaltime_to_localtime|link|register)|whereis)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(and(?:(?:also)?)|b(?:and|not|or|s(?:[lr])|xor)|div|not|or(?:(?:else)?)|rem|xor)\b", OPERATOR_WORD),
        Rule::token_to(r"(?m)^-", PUNCTUATION, NewState::Push(vec![r"directive"])),
        Rule::token(r"(?m)(\+\+?|--?|\*|/|<|>|/=|=:=|=/=|=<|>=|==?|<-|!|\?)", OPERATOR),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)<<", NAME_LABEL),
        Rule::token(r"(?m)>>", NAME_LABEL),
        Rule::bygroups(r"(?m)((?:[a-z]\w*|'[^\n']*[^\\]'))(:)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?:^|(?<=:))((?:[a-z]\w*|'[^\n']*[^\\]'))(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[+-]?(?:[2-9]|[12][0-9]|3[0-6])#[0-9a-zA-Z]+", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-]?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-]?\d+.\d+", NUMBER_FLOAT),
        Rule::token(r#"(?m)[]\[:_@\".{}()|;,]"#, PUNCTUATION),
        Rule::token(r"(?m)(?:[A-Z_]\w*)", NAME_VARIABLE),
        Rule::token(r"(?m)(?:[a-z]\w*|'[^\n']*[^\\]')", NAME),
        Rule::token(r"(?m)\?(?:(?:[A-Z_]\w*)|(?:[a-z]\w*|'[^\n']*[^\\]'))", NAME_CONSTANT),
        Rule::token(r#"(?m)\$(?:(?:\\(?:[bdefnrstv\'"\\]|[0-7][0-7]?[0-7]?|(?:x[0-9a-fA-F]{2}|x\{[0-9a-fA-F]+\})|\^[a-zA-Z]))|\\[ %]|[^\\])"#, STRING_CHAR),
        Rule::token(r"(?m)#(?:[a-z]\w*|'[^\n']*[^\\]')(:?\.(?:[a-z]\w*|'[^\n']*[^\\]'))?", NAME_LABEL),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"map_key"])),
        Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)(?=\})", PUNCTUATION, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for ErlangLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
