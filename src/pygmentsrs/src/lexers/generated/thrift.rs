//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:ThriftLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:ThriftLexer:thrift

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: thrift
pub struct ThriftLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m)//.*?\n", COMMENT),
        Rule::token(r"(?m)/\*[\w\W]*?\*/", COMMENT_MULTILINE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r"(?m)\'", STRING_SINGLE, NewState::Push(vec![r"_tmp_1"])),
        Rule::bygroups_to(r"(?m)(namespace)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"namespace"])),
        Rule::bygroups_to(r"(?m)(enum|union|struct|service|exception)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"class"])),
        Rule::bygroups_g(r"(?m)((?:(?:[^\W\d]|\$)[\w.\[\]$<>]*\s+)+?)((?:[^\W\d]|\$)[\w$]*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::token(r"(?m)(async|oneway|extends|throws|required|optional)\b", KEYWORD),
        Rule::token(r"(?m)(true|false)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(const|typedef)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(c(?:ocoa_prefix|(?:pp_(?:includ|namespac|typ)|sharp_namespac)e)|delphi_namespace|include|java_package|p(?:(?:erl_packag|hp_namespac|y_modul)e)|ruby_namespace|smalltalk_(?:category|prefix)|xsd_(?:a(?:ll|ttrs)|n(?:(?:amespac|illabl)e)|optional))\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(b(?:inary|ool|yte)|double|i(?:16|32|64)|list|map|s(?:e(?:num|t)|list|tring)|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(BEGIN|END|__(?:(?:CLASS|DIR|F(?:ILE|UNCTION)|LINE|METHOD|NAMESPACE)__)|a(?:bstract|lias|nd|rgs|s(?:(?:sert)?))|b(?:egin|reak)|c(?:a(?:se|tch)|l(?:ass|one)|ontinue)|d(?:e(?:clare|fault|lete|[fl])|o|ynamic)|e(?:l(?:if|s(?:e(?:(?:if)?)|if))|n(?:d(?:(?:declare|for(?:(?:each)?)|if|switch|while)?)|sure)|x(?:cept|ec))|f(?:inally|loat|or(?:(?:each)?)|unction)|g(?:lobal|oto)|i(?:mp(?:lements|ort)|n(?:line|stanceof|terface)|[fns])|lambda|module|n(?:ative|e(?:w|xt)|il|ot)|or|p(?:ass|r(?:i(?:nt|vate)|otected)|ublic)|r(?:aise|e(?:do|gister|scue|t(?:ry|urn)))|s(?:elf|izeof|tatic|uper|witch|ynchronized)|t(?:h(?:en|is|row)|r(?:ansient|y))|u(?:n(?:def|less|signed|til)|se)|v(?:ar|irtual|olatile)|w(?:h(?:en|ile)|ith)|xor|yield)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)[+-]?(\d+\.\d+([eE][+-]?\d+)?|\.?\d+[eE][+-]?\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?0x[0-9A-Fa-f]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)[&=]", OPERATOR),
        Rule::token(r"(?m)[:;,{}()<>\[\]]", PUNCTUATION),
        Rule::token(r"(?m)[a-zA-Z_](\.\w|\w)*", NAME),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"comments", vec![
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m)//.*?\n", COMMENT),
        Rule::token(r"(?m)/\*[\w\W]*?\*/", COMMENT_MULTILINE),
    ]);
    m.insert(r"stringescape", vec![
        Rule::token(r#"(?m)\\([\\nrt"\'])"#, STRING_ESCAPE),
    ]);
    m.insert(r"dqs", vec![
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING_DOUBLE),
    ]);
    m.insert(r"_tmp_0", vec![
        Rule::token(r#"(?m)\\([\\nrt"\'])"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING_DOUBLE),
    ]);
    m.insert(r"sqs", vec![
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)[^\\\'\n]+", STRING_SINGLE),
    ]);
    m.insert(r"_tmp_1", vec![
        Rule::token(r#"(?m)\\([\\nrt"\'])"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)[^\\\'\n]+", STRING_SINGLE),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(async|oneway|extends|throws|required|optional)\b", KEYWORD),
        Rule::token(r"(?m)(true|false)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(const|typedef)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(c(?:ocoa_prefix|(?:pp_(?:includ|namespac|typ)|sharp_namespac)e)|delphi_namespace|include|java_package|p(?:(?:erl_packag|hp_namespac|y_modul)e)|ruby_namespace|smalltalk_(?:category|prefix)|xsd_(?:a(?:ll|ttrs)|n(?:(?:amespac|illabl)e)|optional))\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(b(?:inary|ool|yte)|double|i(?:16|32|64)|list|map|s(?:e(?:num|t)|list|tring)|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(BEGIN|END|__(?:(?:CLASS|DIR|F(?:ILE|UNCTION)|LINE|METHOD|NAMESPACE)__)|a(?:bstract|lias|nd|rgs|s(?:(?:sert)?))|b(?:egin|reak)|c(?:a(?:se|tch)|l(?:ass|one)|ontinue)|d(?:e(?:clare|fault|lete|[fl])|o|ynamic)|e(?:l(?:if|s(?:e(?:(?:if)?)|if))|n(?:d(?:(?:declare|for(?:(?:each)?)|if|switch|while)?)|sure)|x(?:cept|ec))|f(?:inally|loat|or(?:(?:each)?)|unction)|g(?:lobal|oto)|i(?:mp(?:lements|ort)|n(?:line|stanceof|terface)|[fns])|lambda|module|n(?:ative|e(?:w|xt)|il|ot)|or|p(?:ass|r(?:i(?:nt|vate)|otected)|ublic)|r(?:aise|e(?:do|gister|scue|t(?:ry|urn)))|s(?:elf|izeof|tatic|uper|witch|ynchronized)|t(?:h(?:en|is|row)|r(?:ansient|y))|u(?:n(?:def|less|signed|til)|se)|v(?:ar|irtual|olatile)|w(?:h(?:en|ile)|ith)|xor|yield)\b", KEYWORD_RESERVED),
    ]);
    m.insert(r"numbers", vec![
        Rule::token(r"(?m)[+-]?(\d+\.\d+([eE][+-]?\d+)?|\.?\d+[eE][+-]?\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?0x[0-9A-Fa-f]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[0-9]+", NUMBER_INTEGER),
    ]);
    m.insert(r"namespace", vec![
        Rule::token_to(r"(?m)[a-z*](\.\w|\w)*", NAME_NAMESPACE, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"class", vec![
        Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_CLASS, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for ThriftLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
