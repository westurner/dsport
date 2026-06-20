//! AUTO-GENERATED from `pygments.pygments.lexers.shell:BatchLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.shell:BatchLexer:batch

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: batch, bat, dosbatch, winbatch
pub struct BatchLexer;

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
        Rule::token(r"(?im)\)((?=\()|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))(?:(?:[^\n\x1a^]|\^[\n\x1a]?[\w\W])*)", COMMENT_SINGLE),
        Rule::token_to(r"(?im)(?=((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:))", TEXT, NewState::Push(vec![r"follow"])),
        Rule::using_this(r"(?im)(?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)", Some(vec!["root", "text"])),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token(r"(?im)[\n\x1a]+", TEXT),
        Rule::token_to(r"(?im)\(", PUNCTUATION, NewState::Push(vec![r"root/compound"])),
        Rule::token(r"(?im)@+", PUNCTUATION),
        Rule::bygroups_g_to(r#"(?im)((?:for|if|rem)(?:(?=(?:\^[\n\x1a]?)?/)|(?:(?!\^)|(?<=m))(?:(?=\()|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+)?(?:\^[\n\x1a]?)?/(?:\^[\n\x1a]?)?\?)"#, vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })], NewState::Push(vec![r"follow"])),
        Rule::bygroups_g_to(r#"(?im)(goto(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(]))((?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|[^"%\n\x1a&<>|])*(?:\^[\n\x1a]?)?/(?:\^[\n\x1a]?)?\?(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|[^"%\n\x1a&<>|])*)"#, vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })], NewState::Push(vec![r"follow"])),
        Rule::token_to(r"(?im)(assoc|break|c(?:d|hdir|ls|o(?:lor|py))|d(?:ate|el|ir|path)|e(?:cho|ndlocal|rase|xit)|ftype|keys|m(?:d|k(?:dir|link)|ove)|p(?:a(?:th|use)|opd|rompt|ushd)|r(?:d|en(?:(?:ame)?)|mdir)|s(?:etlocal|(?:hif|tar)t)|t(?:(?:i(?:m|tl)|yp)e)|v(?:er(?:(?:ify)?)|ol))(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(])", KEYWORD, NewState::Push(vec![r"follow"])),
        Rule::bygroups_g_to(r"(?im)(call)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)(:)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"call"])),
        Rule::token(r"(?im)call(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(])", KEYWORD),
        Rule::bygroups_g_to(r"(?im)(for(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])(?!\^))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(/f(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD))], NewState::Push(vec![r"for/f", r"for"])),
        Rule::bygroups_g_to(r"(?im)(for(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])(?!\^))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(/l(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD))], NewState::Push(vec![r"for/l", r"for"])),
        Rule::token_to(r"(?im)for(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])(?!\^)", KEYWORD, NewState::Push(vec![r"for2", r"for"])),
        Rule::bygroups_g_to(r"(?im)(goto(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(]))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)(:?)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"label"])),
        Rule::bygroups_g_to(r"(?im)(if(?:(?=\()|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))(?!\^))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)((?:/i(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))?)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)((?:not(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))?)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })], NewState::Push(vec![r"(?", r"if"])),
        Rule::token_to(r#"(?im)rem(((?=\()|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))(?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+)?.*|(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(])(?:(?:[^\n\x1a^]|\^[\n\x1a]?[\w\W])*))"#, COMMENT_SINGLE, NewState::Push(vec![r"follow"])),
        Rule::bygroups_g_to(r"(?im)(set(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(]))((?:(?:\^[\n\x1a]?)?[^\S\n])*)(/a)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD))], NewState::Push(vec![r"arithmetic"])),
        Rule::bygroups_g_to(r#"(?im)(set(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(]))((?:(?:\^[\n\x1a]?)?[^\S\n])*)((?:/p)?)((?:(?:\^[\n\x1a]?)?[^\S\n])*)((?:(?:(?:\^[\n\x1a]?)?[^"\n\x1a&<>|^=]|\^[\n\x1a]?[^"=])+)?)((?:(?:\^[\n\x1a]?)?=)?)"#, vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::UsingThis { state: Some(vec!["root", "variable"]) }), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"follow"])),
        Rule::default(NewState::Push(vec![r"follow"])),
    ]);
    m.insert(r"redirect", vec![
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
    ]);
    m.insert(r"follow", vec![
        Rule::bygroups(r"(?im)((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:)([\t\v\f\r ,;=\xa0]*)((?:(?:[^\s\x1a\xa0,;=&<>|+:^]|\^[\n\x1a]?[\w\W])*))(.*)", vec![Some(TEXT), Some(PUNCTUATION), Some(TEXT), Some(NAME_LABEL), Some(COMMENT_SINGLE)]),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token_to(r"(?im)(?=[\n\x1a])", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?im)\|\|?|&&?", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\s\x1a\xa0,;=&<>|\d)]+|."#, TEXT),
    ]);
    m.insert(r"text", vec![
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\s\x1a\xa0,;=&<>|\d)]+|."#, TEXT),
    ]);
    m.insert(r"variable-or-escape", vec![
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
    ]);
    m.insert(r"arithmetic", vec![
        Rule::token(r"(?im)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?im)0x[\da-f]+", NUMBER_HEX),
        Rule::token(r"(?im)\d+", NUMBER_INTEGER),
        Rule::token(r"(?im)[(),]+", PUNCTUATION),
        Rule::token(r"(?im)([=+\-*/!~]|%|\^\^)+", OPERATOR),
        Rule::using_this(r#"(?im)((?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(\^[\n\x1a]?)?[^()=+\-*/!~%\^"\s\x1a\xa0,;=&<>|]|\^[\s\x1a\xa0,;=]?[\w\W])+"#, Some(vec!["root", "variable"])),
        Rule::token_to(r"(?im)(?=[\x00|&])", TEXT, NewState::Pop(1)),
        Rule::bygroups(r"(?im)((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:)([\t\v\f\r ,;=\xa0]*)((?:(?:[^\s\x1a\xa0,;=&<>|+:^]|\^[\n\x1a]?[\w\W])*))(.*)", vec![Some(TEXT), Some(PUNCTUATION), Some(TEXT), Some(NAME_LABEL), Some(COMMENT_SINGLE)]),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token_to(r"(?im)(?=[\n\x1a])", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?im)\|\|?|&&?", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\s\x1a\xa0,;=&<>|\d)]+|."#, TEXT),
    ]);
    m.insert(
        r"call",
        vec![Rule::bygroups_to(
            r"(?im)(:?)((?:(?:[^\s\x1a\xa0,;=&<>|+:^]|\^[\n\x1a]?[\w\W])*))",
            vec![Some(PUNCTUATION), Some(NAME_LABEL)],
            NewState::Pop(1),
        )],
    );
    m.insert(r"label", vec![
        Rule::bygroups_to(r#"(?im)((?:(?:[^\s\x1a\xa0,;=&<>|+:^]|\^[\n\x1a]?[\w\W])*)?)((?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|\^[\n\x1a]?[\w\W]|[^"%^\n\x1a&<>|])*)"#, vec![Some(NAME_LABEL), Some(COMMENT_SINGLE)], NewState::Pop(1)),
    ]);
    m.insert(r"root/compound", vec![
        Rule::token_to(r"(?im)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?im)(?=((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:))", TEXT, NewState::Push(vec![r"follow/compound"])),
        Rule::using_this(r"(?im)(?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)", Some(vec!["root", "text"])),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|)])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token(r"(?im)[\n\x1a]+", TEXT),
        Rule::token_to(r"(?im)\(", PUNCTUATION, NewState::Push(vec![r"root/compound"])),
        Rule::token(r"(?im)@+", PUNCTUATION),
        Rule::bygroups_g_to(r#"(?im)((?:for|if|rem)(?:(?=(?:\^[\n\x1a]?)?/)|(?:(?!\^)|(?<=m))(?:(?=\()|(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])))))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|)])+)?(?:\^[\n\x1a]?)?/(?:\^[\n\x1a]?)?\?)"#, vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })], NewState::Push(vec![r"follow/compound"])),
        Rule::bygroups_g_to(r#"(?im)(goto(?:(?=\))|(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(])))((?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|[^"%\n\x1a&<>|)])*(?:\^[\n\x1a]?)?/(?:\^[\n\x1a]?)?\?(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|[^"%\n\x1a&<>|)])*)"#, vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })], NewState::Push(vec![r"follow/compound"])),
        Rule::token_to(r"(?im)(assoc|break|c(?:d|hdir|ls|o(?:lor|py))|d(?:ate|el|ir|path)|e(?:cho|ndlocal|rase|xit)|ftype|keys|m(?:d|k(?:dir|link)|ove)|p(?:a(?:th|use)|opd|rompt|ushd)|r(?:d|en(?:(?:ame)?)|mdir)|s(?:etlocal|(?:hif|tar)t)|t(?:(?:i(?:m|tl)|yp)e)|v(?:er(?:(?:ify)?)|ol))(?:(?=\))|(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(]))", KEYWORD, NewState::Push(vec![r"follow/compound"])),
        Rule::bygroups_g_to(r"(?im)(call)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)(:)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"call/compound"])),
        Rule::token(r"(?im)call(?:(?=\))|(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(]))", KEYWORD),
        Rule::bygroups_g_to(r"(?im)(for(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))(?!\^))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(/f(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])))", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD))], NewState::Push(vec![r"for/f", r"for"])),
        Rule::bygroups_g_to(r"(?im)(for(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))(?!\^))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(/l(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])))", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD))], NewState::Push(vec![r"for/l", r"for"])),
        Rule::token_to(r"(?im)for(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))(?!\^)", KEYWORD, NewState::Push(vec![r"for2", r"for"])),
        Rule::bygroups_g_to(r"(?im)(goto(?:(?=\))|(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(])))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)(:?)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"label/compound"])),
        Rule::bygroups_g_to(r"(?im)(if(?:(?=\()|(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])))(?!\^))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)((?:/i(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])))?)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)((?:not(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])))?)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })], NewState::Push(vec![r"(?", r"if"])),
        Rule::token_to(r#"(?im)rem(((?=\()|(?:(?=\))|(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])))(?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+)?.*|(?:(?=\))|(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(]))(?:(?:[^\n\x1a^)]|\^[\n\x1a]?[^)])*))"#, COMMENT_SINGLE, NewState::Push(vec![r"follow/compound"])),
        Rule::bygroups_g_to(r"(?im)(set(?:(?=\))|(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(])))((?:(?:\^[\n\x1a]?)?[^\S\n])*)(/a)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD))], NewState::Push(vec![r"arithmetic/compound"])),
        Rule::bygroups_g_to(r#"(?im)(set(?:(?=\))|(?=(?:\^[\n\x1a]?)?[\t\v\f\r ,;=\xa0+./:\[\\\]]|[\n\x1a&<>|(])))((?:(?:\^[\n\x1a]?)?[^\S\n])*)((?:/p)?)((?:(?:\^[\n\x1a]?)?[^\S\n])*)((?:(?:(?:\^[\n\x1a]?)?[^"\n\x1a&<>|^=)]|\^[\n\x1a]?[^"=])+)?)((?:(?:\^[\n\x1a]?)?=)?)"#, vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::UsingThis { state: Some(vec!["root", "variable"]) }), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"follow/compound"])),
        Rule::default(NewState::Push(vec![r"follow/compound"])),
    ]);
    m.insert(r"redirect/compound", vec![
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|)])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
    ]);
    m.insert(r"follow/compound", vec![
        Rule::token_to(r"(?im)(?=\))", TEXT, NewState::Pop(1)),
        Rule::bygroups(r"(?im)((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:)([\t\v\f\r ,;=\xa0]*)((?:(?:[^\s\x1a\xa0,;=&<>|+:^)]|\^[\n\x1a]?[^)])*))(.*)", vec![Some(TEXT), Some(PUNCTUATION), Some(TEXT), Some(NAME_LABEL), Some(COMMENT_SINGLE)]),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|)])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token_to(r"(?im)(?=[\n\x1a])", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?im)\|\|?|&&?", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\s\x1a\xa0,;=&<>|\d)]+|."#, TEXT),
    ]);
    m.insert(r"arithmetic/compound", vec![
        Rule::token_to(r"(?im)(?=\))", TEXT, NewState::Pop(1)),
        Rule::token(r"(?im)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?im)0x[\da-f]+", NUMBER_HEX),
        Rule::token(r"(?im)\d+", NUMBER_INTEGER),
        Rule::token(r"(?im)[(),]+", PUNCTUATION),
        Rule::token(r"(?im)([=+\-*/!~]|%|\^\^)+", OPERATOR),
        Rule::using_this(r#"(?im)((?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(\^[\n\x1a]?)?[^()=+\-*/!~%\^"\s\x1a\xa0,;=&<>|]|\^[\s\x1a\xa0,;=]?[^)])+"#, Some(vec!["root", "variable"])),
        Rule::token_to(r"(?im)(?=[\x00|&])", TEXT, NewState::Pop(1)),
        Rule::bygroups(r"(?im)((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:)([\t\v\f\r ,;=\xa0]*)((?:(?:[^\s\x1a\xa0,;=&<>|+:^]|\^[\n\x1a]?[\w\W])*))(.*)", vec![Some(TEXT), Some(PUNCTUATION), Some(TEXT), Some(NAME_LABEL), Some(COMMENT_SINGLE)]),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token_to(r"(?im)(?=[\n\x1a])", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?im)\|\|?|&&?", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\s\x1a\xa0,;=&<>|\d)]+|."#, TEXT),
    ]);
    m.insert(
        r"call/compound",
        vec![
            Rule::token_to(r"(?im)(?=\))", TEXT, NewState::Pop(1)),
            Rule::bygroups_to(
                r"(?im)(:?)((?:(?:[^\s\x1a\xa0,;=&<>|+:^)]|\^[\n\x1a]?[^)])*))",
                vec![Some(PUNCTUATION), Some(NAME_LABEL)],
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(r"label/compound", vec![
        Rule::token_to(r"(?im)(?=\))", TEXT, NewState::Pop(1)),
        Rule::bygroups_to(r#"(?im)((?:(?:[^\s\x1a\xa0,;=&<>|+:^)]|\^[\n\x1a]?[^)])*)?)((?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|\^[\n\x1a]?[^)]|[^"%^\n\x1a&<>|)])*)"#, vec![Some(NAME_LABEL), Some(COMMENT_SINGLE)], NewState::Pop(1)),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)\^!|%%", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\n\x1a]+|[%^]"#, STRING_DOUBLE),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"sqstring", vec![
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r"(?im)[^%]+|%", STRING_SINGLE),
    ]);
    m.insert(r"bqstring", vec![
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r"(?im)[^%]+|%", STRING_BACKTICK),
    ]);
    m.insert(r"variable", vec![
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\n\x1a]+|."#, NAME_VARIABLE),
    ]);
    m.insert(r"for", vec![
        Rule::bygroups_g_to(r"(?im)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(in)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(\()", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(PUNCTUATION))], NewState::Pop(1)),
        Rule::bygroups(r"(?im)((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:)([\t\v\f\r ,;=\xa0]*)((?:(?:[^\s\x1a\xa0,;=&<>|+:^]|\^[\n\x1a]?[\w\W])*))(.*)", vec![Some(TEXT), Some(PUNCTUATION), Some(TEXT), Some(NAME_LABEL), Some(COMMENT_SINGLE)]),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token_to(r"(?im)(?=[\n\x1a])", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?im)\|\|?|&&?", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\s\x1a\xa0,;=&<>|\d)]+|."#, TEXT),
    ]);
    m.insert(r"for2", vec![
        Rule::token(r"(?im)\)", PUNCTUATION),
        Rule::bygroups_g_to(r"(?im)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(do(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD))], NewState::Pop(1)),
        Rule::token(r"(?im)[\n\x1a]+", TEXT),
        Rule::bygroups(r"(?im)((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:)([\t\v\f\r ,;=\xa0]*)((?:(?:[^\s\x1a\xa0,;=&<>|+:^]|\^[\n\x1a]?[\w\W])*))(.*)", vec![Some(TEXT), Some(PUNCTUATION), Some(TEXT), Some(NAME_LABEL), Some(COMMENT_SINGLE)]),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token_to(r"(?im)(?=[\n\x1a])", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?im)\|\|?|&&?", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\s\x1a\xa0,;=&<>|\d)]+|."#, TEXT),
    ]);
    m.insert(r"for/f", vec![
        Rule::bygroups_g(r#"(?im)(")((?:(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|[^"])*?")([\s\x1a\xa0,;=]*)(\))"#, vec![Some(GroupAction::Token(STRING_DOUBLE)), Some(GroupAction::UsingThis { state: Some(vec!["root", "string"]) }), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"#pop", r"for2", r"string"])),
        Rule::bygroups_g(r"(?im)('(?:%%|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|[\w\W])*?')([\s\x1a\xa0,;=]*)(\))", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "sqstring"]) }), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?im)(`(?:%%|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|[\w\W])*?`)([\s\x1a\xa0,;=]*)(\))", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "bqstring"]) }), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?im)\)", PUNCTUATION),
        Rule::bygroups_g_to(r"(?im)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(do(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD))], NewState::Pop(1)),
        Rule::token(r"(?im)[\n\x1a]+", TEXT),
        Rule::bygroups(r"(?im)((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:)([\t\v\f\r ,;=\xa0]*)((?:(?:[^\s\x1a\xa0,;=&<>|+:^]|\^[\n\x1a]?[\w\W])*))(.*)", vec![Some(TEXT), Some(PUNCTUATION), Some(TEXT), Some(NAME_LABEL), Some(COMMENT_SINGLE)]),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token_to(r"(?im)(?=[\n\x1a])", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?im)\|\|?|&&?", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\s\x1a\xa0,;=&<>|\d)]+|."#, TEXT),
    ]);
    m.insert(r"for/l", vec![
        Rule::token(r"(?im)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?im)\)", PUNCTUATION),
        Rule::bygroups_g_to(r"(?im)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(do(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(KEYWORD))], NewState::Pop(1)),
        Rule::token(r"(?im)[\n\x1a]+", TEXT),
        Rule::bygroups(r"(?im)((?:(?<=^[^:])|^[^:]?)[\t\v\f\r ,;=\xa0]*)(:)([\t\v\f\r ,;=\xa0]*)((?:(?:[^\s\x1a\xa0,;=&<>|+:^]|\^[\n\x1a]?[\w\W])*))(.*)", vec![Some(TEXT), Some(PUNCTUATION), Some(TEXT), Some(NAME_LABEL), Some(COMMENT_SINGLE)]),
        Rule::bygroups(r"(?im)((?:(?<=[\s\x1a\xa0,;=])\d)?)(>>?&|<&)([\s\x1a\xa0,;=]*)(\d)", vec![Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(TEXT), Some(NUMBER_INTEGER)]),
        Rule::bygroups_g(r#"(?im)((?:(?<=[\s\x1a\xa0,;=])(?<!\^[\n\x1a])\d)?)(>>?|<)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(NUMBER_INTEGER)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })]),
        Rule::token_to(r"(?im)(?=[\n\x1a])", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?im)\|\|?|&&?", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))", NAME_VARIABLE),
        Rule::token(r"(?im)%%|\^[\n\x1a]?(\^!|[\w\W])", STRING_ESCAPE),
        Rule::token(r#"(?im)[^"%^\s\x1a\xa0,;=&<>|\d)]+|."#, TEXT),
    ]);
    m.insert(r"if", vec![
        Rule::bygroups_g_to(r"(?im)((?:cmdextversion|errorlevel)(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))(\d+)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(NUMBER_INTEGER))], NewState::Pop(1)),
        Rule::bygroups_g_to(r#"(?im)(defined(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))((?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::UsingThis { state: Some(vec!["root", "variable"]) })], NewState::Pop(1)),
        Rule::bygroups_g_to(r#"(?im)(exist(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })], NewState::Pop(1)),
        Rule::bygroups_g_to(r"(?im)((?:-?(?:0[0-7]+|0x[\da-f]+|\d+)(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a]))(?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))((?:equ|geq|gtr|leq|lss|neq))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)(?:-?(?:0[0-7]+|0x[\da-f]+|\d+)(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])))", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "arithmetic"]) }), Some(GroupAction::Token(OPERATOR_WORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "arithmetic"]) })], NewState::Pop(1)),
        Rule::using_this_to(r#"(?im)(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+)"#, Some(vec!["root", "text"]), NewState::Push(vec![r"#pop", r"if2"])),
    ]);
    m.insert(r"if2", vec![
        Rule::bygroups_g_to(r#"(?im)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?)(==)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)?(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(OPERATOR)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })], NewState::Pop(1)),
        Rule::bygroups_g_to(r#"(?im)((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+))((?:equ|geq|gtr|leq|lss|neq))((?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)(?:[&<>|]+|(?:(?:"[^\n\x1a"]*(?:"|(?=[\n\x1a])))|(?:(?:%(?:\*|(?:~[a-z]*(?:\$[^:]+:)?)?\d|[^%:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^%\n\x1a^]|\^[^%\n\x1a])[^=\n\x1a]*=(?:[^%\n\x1a^]|\^[^%\n\x1a])*)?)?%))|(?:\^?![^!:\n\x1a]+(?::(?:~(?:-?\d+)?(?:,(?:-?\d+)?)?|(?:[^!\n\x1a^]|\^[^!\n\x1a])[^=\n\x1a]*=(?:[^!\n\x1a^]|\^[^!\n\x1a])*)?)?\^?!))|(?:(?:(?:\^[\n\x1a]?)?[^"\s\x1a\xa0,;=&<>|])+))+))"#, vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) }), Some(GroupAction::Token(OPERATOR_WORD)), Some(GroupAction::UsingThis { state: Some(vec!["root", "text"]) })], NewState::Pop(1)),
    ]);
    m.insert(
        r"(?",
        vec![
            Rule::using_this(
                r"(?im)(?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)",
                Some(vec!["root", "text"]),
            ),
            Rule::token_to(
                r"(?im)\(",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"else?", r"root/compound"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"else?",
        vec![
            Rule::using_this(
                r"(?im)(?:(?:(?:\^[\n\x1a])?[\t\v\f\r ,;=\xa0])+)",
                Some(vec!["root", "text"]),
            ),
            Rule::token_to(
                r"(?im)else(?=\^?[\t\v\f\r ,;=\xa0]|[&<>|\n\x1a])",
                KEYWORD,
                NewState::Pop(1),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for BatchLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
