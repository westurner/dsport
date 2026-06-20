//! AUTO-GENERATED from `pygments.pygments.lexers.varnish:VCLSnippetLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.varnish:VCLSnippetLexer:vclsnippets

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: vclsnippets, vclsnippet
pub struct VclsnippetsLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"snippetspre", vec![
        Rule::token(r"(?m)\.\.\.+", COMMENT),
        Rule::token(r"(?m)(bereq|req|req_top|resp|beresp|obj|client|server|local|remote|storage)($|\.\*)", NAME_VARIABLE),
    ]);
    m.insert(
        r"snippetspost",
        vec![Rule::token(r"(?m)(backend)\b", KEYWORD_RESERVED)],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)\.\.\.+", COMMENT),
        Rule::token(r"(?m)(bereq|req|req_top|resp|beresp|obj|client|server|local|remote|storage)($|\.\*)", NAME_VARIABLE),
        Rule::token_to(r#"(?m)L?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token_to(r#"(?m)\{""#, STRING, NewState::Push(vec![r"multistring"])),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)//.*$", COMMENT),
        Rule::token(r"(?m)(\d\.)?\d+[sdwhmy]", LITERAL_DATE),
        Rule::token(r"(?m)(\d\.)?\d+ms", LITERAL_DATE),
        Rule::token(r"(?m)(vcl_pass|vcl_hash|vcl_hit|vcl_init|vcl_backend_fetch|vcl_pipe|vcl_backend_response|vcl_synth|vcl_deliver|vcl_backend_error|vcl_fini|vcl_recv|vcl_purge|vcl_miss)\b", NAME_FUNCTION),
        Rule::token(r"(?m)(pipe|retry|hash|synth|deliver|purge|abandon|lookup|pass|fail|ok|miss|fetch|restart)\b", NAME_CONSTANT),
        Rule::token(r"(?m)(beresp|obj|resp|req|req_top|bereq)\.http\.[a-zA-Z_-]+\b", NAME_VARIABLE),
        Rule::token(r"(?m)(bere(?:q\.(?:b(?:ackend|etween_bytes_timeout)|connect_timeout|first_byte_timeout|method|proto|retries|u(?:ncacheable|rl)|xid)|sp\.(?:age|backend(?:(?:\.(?:ip|name))?)|do_(?:esi|g(?:(?:(?:un)?)zip)|stream)|grace|keep|proto|reason|st(?:atus|orage_hint)|ttl|uncacheable|was_304))|client\.i(?:dentity|p)|local\.ip|now|obj\.(?:age|grace|hits|keep|proto|reason|status|ttl|uncacheable)|re(?:mote\.ip|q(?:\.(?:backend_hint|can_gzip|esi(?:(?:_level)?)|hash_(?:always_miss|ignore_busy)|method|proto|restarts|ttl|url|xid)|_top\.(?:method|proto|url))|sp\.(?:is_streaming|proto|reason|status))|server\.(?:hostname|i(?:dentity|p)))\b", NAME_VARIABLE),
        Rule::token(r"(?m)[!%&+*\-,/<.}{>=|~]+", OPERATOR),
        Rule::token(r"(?m)[();]", PUNCTUATION),
        Rule::token(r"(?m)[,]+", PUNCTUATION),
        Rule::token(r"(?m)(ban|call|el(?:if|s(?:e|if))|hash_data|i(?:f|mport|nclude)|new|r(?:e(?:gsub(?:(?:all)?)|turn)|ollback)|s(?:et|ynth(?:(?:etic)?))|unset)\b", KEYWORD),
        Rule::token(r"(?m)storage\.\w+\.\w+\b", NAME_VARIABLE),
        Rule::token(r"(?m)((?:fals|tru)e)", NAME_BUILTIN),
        Rule::token(r"(?m)\d+\b", NUMBER),
        Rule::bygroups_to(r"(?m)(backend)(\s+\w+)(\s*\{)", vec![Some(KEYWORD), Some(NAME_VARIABLE_GLOBAL), Some(PUNCTUATION)], NewState::Push(vec![r"backend"])),
        Rule::bygroups_to(r"(?m)(probe\s)(\s*\w+\s)(\{)", vec![Some(KEYWORD), Some(NAME_VARIABLE_GLOBAL), Some(PUNCTUATION)], NewState::Push(vec![r"probe"])),
        Rule::bygroups_to(r"(?m)(acl\s)(\s*\w+\s)(\{)", vec![Some(KEYWORD), Some(NAME_VARIABLE_GLOBAL), Some(PUNCTUATION)], NewState::Push(vec![r"acl"])),
        Rule::bygroups(r"(?m)(vcl )(4.0)(;)$", vec![Some(KEYWORD_RESERVED), Some(NAME_CONSTANT), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(sub\s+)([a-zA-Z]\w*)(\s*\{)", vec![Some(KEYWORD), Some(NAME_FUNCTION), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?m)([a-zA-Z_]\w*)(\.)([a-zA-Z_]\w*)(\s*\(.*\))", vec![Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::UsingThis { state: None })]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)(backend)\b", KEYWORD_RESERVED),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::token_to(r#"(?m)L?""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token_to(r#"(?m)\{""#, STRING, NewState::Push(vec![r"multistring"])),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\\\n", TEXT),
        ],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token(r"(?m)//.*$", COMMENT),
        ],
    );
    m.insert(r"statements", vec![
        Rule::token(r"(?m)(\d\.)?\d+[sdwhmy]", LITERAL_DATE),
        Rule::token(r"(?m)(\d\.)?\d+ms", LITERAL_DATE),
        Rule::token(r"(?m)(vcl_pass|vcl_hash|vcl_hit|vcl_init|vcl_backend_fetch|vcl_pipe|vcl_backend_response|vcl_synth|vcl_deliver|vcl_backend_error|vcl_fini|vcl_recv|vcl_purge|vcl_miss)\b", NAME_FUNCTION),
        Rule::token(r"(?m)(pipe|retry|hash|synth|deliver|purge|abandon|lookup|pass|fail|ok|miss|fetch|restart)\b", NAME_CONSTANT),
        Rule::token(r"(?m)(beresp|obj|resp|req|req_top|bereq)\.http\.[a-zA-Z_-]+\b", NAME_VARIABLE),
        Rule::token(r"(?m)(bere(?:q\.(?:b(?:ackend|etween_bytes_timeout)|connect_timeout|first_byte_timeout|method|proto|retries|u(?:ncacheable|rl)|xid)|sp\.(?:age|backend(?:(?:\.(?:ip|name))?)|do_(?:esi|g(?:(?:(?:un)?)zip)|stream)|grace|keep|proto|reason|st(?:atus|orage_hint)|ttl|uncacheable|was_304))|client\.i(?:dentity|p)|local\.ip|now|obj\.(?:age|grace|hits|keep|proto|reason|status|ttl|uncacheable)|re(?:mote\.ip|q(?:\.(?:backend_hint|can_gzip|esi(?:(?:_level)?)|hash_(?:always_miss|ignore_busy)|method|proto|restarts|ttl|url|xid)|_top\.(?:method|proto|url))|sp\.(?:is_streaming|proto|reason|status))|server\.(?:hostname|i(?:dentity|p)))\b", NAME_VARIABLE),
        Rule::token(r"(?m)[!%&+*\-,/<.}{>=|~]+", OPERATOR),
        Rule::token(r"(?m)[();]", PUNCTUATION),
        Rule::token(r"(?m)[,]+", PUNCTUATION),
        Rule::token(r"(?m)(ban|call|el(?:if|s(?:e|if))|hash_data|i(?:f|mport|nclude)|new|r(?:e(?:gsub(?:(?:all)?)|turn)|ollback)|s(?:et|ynth(?:(?:etic)?))|unset)\b", KEYWORD),
        Rule::token(r"(?m)storage\.\w+\.\w+\b", NAME_VARIABLE),
        Rule::token(r"(?m)((?:fals|tru)e)", NAME_BUILTIN),
        Rule::token(r"(?m)\d+\b", NUMBER),
        Rule::bygroups_to(r"(?m)(backend)(\s+\w+)(\s*\{)", vec![Some(KEYWORD), Some(NAME_VARIABLE_GLOBAL), Some(PUNCTUATION)], NewState::Push(vec![r"backend"])),
        Rule::bygroups_to(r"(?m)(probe\s)(\s*\w+\s)(\{)", vec![Some(KEYWORD), Some(NAME_VARIABLE_GLOBAL), Some(PUNCTUATION)], NewState::Push(vec![r"probe"])),
        Rule::bygroups_to(r"(?m)(acl\s)(\s*\w+\s)(\{)", vec![Some(KEYWORD), Some(NAME_VARIABLE_GLOBAL), Some(PUNCTUATION)], NewState::Push(vec![r"acl"])),
        Rule::bygroups(r"(?m)(vcl )(4.0)(;)$", vec![Some(KEYWORD_RESERVED), Some(NAME_CONSTANT), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(sub\s+)([a-zA-Z]\w*)(\s*\{)", vec![Some(KEYWORD), Some(NAME_FUNCTION), Some(PUNCTUATION)]),
        Rule::bygroups_g(r"(?m)([a-zA-Z_]\w*)(\.)([a-zA-Z_]\w*)(\s*\(.*\))", vec![Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::UsingThis { state: None })]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(
        r"probe",
        vec![
            Rule::token_to(r#"(?m)L?""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token_to(r#"(?m)\{""#, STRING, NewState::Push(vec![r"multistring"])),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\\\n", TEXT),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token(r"(?m)//.*$", COMMENT),
            Rule::bygroups_g(
                r"(?m)(\.\w+)(\s*=\s*)([^;]*)(;)",
                vec![
                    Some(GroupAction::Token(NAME_ATTRIBUTE)),
                    Some(GroupAction::Token(OPERATOR)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"acl",
        vec![
            Rule::token_to(r#"(?m)L?""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token_to(r#"(?m)\{""#, STRING, NewState::Push(vec![r"multistring"])),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\\\n", TEXT),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token(r"(?m)//.*$", COMMENT),
            Rule::token(r"(?m)[!/]+", OPERATOR),
            Rule::token(r"(?m);", PUNCTUATION),
            Rule::token(r"(?m)\d+", NUMBER),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"backend",
        vec![
            Rule::token_to(r#"(?m)L?""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token_to(r#"(?m)\{""#, STRING, NewState::Push(vec![r"multistring"])),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\\\n", TEXT),
            Rule::bygroups(
                r"(?m)(\.probe)(\s*=\s*)(\w+)(;)",
                vec![
                    Some(NAME_ATTRIBUTE),
                    Some(OPERATOR),
                    Some(NAME_VARIABLE_GLOBAL),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_to(
                r"(?m)(\.probe)(\s*=\s*)(\{)",
                vec![Some(NAME_ATTRIBUTE), Some(OPERATOR), Some(PUNCTUATION)],
                NewState::Push(vec![r"probe"]),
            ),
            Rule::bygroups_g(
                r"(?m)(\.\w+\b)(\s*=\s*)([^;\s]*)(\s*;)",
                vec![
                    Some(GroupAction::Token(NAME_ATTRIBUTE)),
                    Some(GroupAction::Token(OPERATOR)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(PUNCTUATION)),
                ],
            ),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"\n]+"#, STRING),
        ],
    );
    m.insert(
        r"multistring",
        vec![
            Rule::token(r#"(?m)[^"}]"#, STRING),
            Rule::token_to(r#"(?m)"\}"#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)["}]"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for VclsnippetsLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
