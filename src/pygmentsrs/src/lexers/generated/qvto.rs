//! AUTO-GENERATED from `pygments.pygments.lexers.qvt:QVToLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.qvt:QVToLexer:qvto

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: qvto, qvt
pub struct QvtoLexer;

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
        Rule::token(r"(?m)\n", TEXT),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::bygroups(r"(?m)(--|//)(\s*)(directive:)?(.*)$", vec![Some(COMMENT), Some(COMMENT), Some(COMMENT_PREPROC), Some(COMMENT)]),
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)(and|not|or|xor|##?)\b", OPERATOR_WORD),
        Rule::token(r"(?m)(:{1,2}=|[-+]=)\b", OPERATOR_WORD),
        Rule::token(r"(?m)(@|<<|>>)\b", KEYWORD),
        Rule::token(r"(?m)!=|<>|==|=|!->|->|>=|<=|[.]{3}|[+/*%=<>&|.~]", OPERATOR),
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::token(r"(?m)(true|false|unlimited|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(this|self|result)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(var)\b", KEYWORD_DECLARATION),
        Rule::token_to(r"(?m)(from|import)\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"fromimport"])),
        Rule::bygroups(r"(?m)(metamodel|class|exception|primitive|enum|transformation|library)(\s+)(\w+)", vec![Some(TokenType::new(&["Keyword", "Word"])), Some(TEXT), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(exception)(\s+)(\w+)", vec![Some(TokenType::new(&["Keyword", "Word"])), Some(TEXT), Some(NAME_EXCEPTION)]),
        Rule::token(r"(?m)(main)\b", NAME_FUNCTION),
        Rule::bygroups_to(r"(?m)(mapping|helper|query)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(TEXT)], NewState::Push(vec![r"operation"])),
        Rule::bygroups_to(r"(?m)(assert)(\s+)\b", vec![Some(KEYWORD), Some(TEXT)], NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(Bag|Collection|Dict|OrderedSet|Sequence|Set|Tuple|List)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(a(?:bstract|ccess|ny|ssert)|b(?:lackbox|reak)|c(?:ase|o(?:llect(?:(?:Nested|One|select(?:(?:One)?))?)|mp(?:oses|ute)|n(?:figuration|structor|tinue)))|d(?:atatype|e(?:fault|rived)|isjuncts|o)|e(?:l(?:if|se)|nd(?:(?:if)?)|x(?:cept|(?:ist|tend)s))|f(?:or(?:All|Each|One)|rom)|i(?:mplies|n(?:herits|it|out|termediate|vresolve(?:(?:In|one(?:(?:In)?))?))|(?:sUniqu|terat)e|[fn])|l(?:ate|et|iteral|og)|m(?:ap|erges|odeltype)|new|o(?:bject|ne|rdered|ut)|p(?:ackage|opulation|roperty)|r(?:aise|e(?:adonly|f(?:(?:erenc|in)es)|ject|solve(?:(?:In|one(?:(?:In)?))?)|turn))|s(?:elect(?:(?:One)?)|ortedBy|tatic|witch)|t(?:ag|hen|ry|ypedef)|u(?:nlimited|ses)|w(?:h(?:e(?:n|re)|ile)|ith)|x(?:collect|map|select))\b", KEYWORD),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"_tmp_1"])),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(a(?:bstract|ccess|ny|ssert)|b(?:lackbox|reak)|c(?:ase|o(?:llect(?:(?:Nested|One|select(?:(?:One)?))?)|mp(?:oses|ute)|n(?:figuration|structor|tinue)))|d(?:atatype|e(?:fault|rived)|isjuncts|o)|e(?:l(?:if|se)|nd(?:(?:if)?)|x(?:cept|(?:ist|tend)s))|f(?:or(?:All|Each|One)|rom)|i(?:mplies|n(?:herits|it|out|termediate|vresolve(?:(?:In|one(?:(?:In)?))?))|(?:sUniqu|terat)e|[fn])|l(?:ate|et|iteral|og)|m(?:ap|erges|odeltype)|new|o(?:bject|ne|rdered|ut)|p(?:ackage|opulation|roperty)|r(?:aise|e(?:adonly|f(?:(?:erenc|in)es)|ject|solve(?:(?:In|one(?:(?:In)?))?)|turn))|s(?:elect(?:(?:One)?)|ortedBy|tatic|witch)|t(?:ag|hen|ry|ypedef)|u(?:nlimited|ses)|w(?:h(?:e(?:n|re)|ile)|ith)|x(?:collect|map|select))\b", KEYWORD),
    ]);
    m.insert(r"stringescape", vec![
        Rule::token(r#"(?m)\\([\\btnfr"\']|u[0-3][0-7]{2}|u[0-7]{1,2})"#, STRING_ESCAPE),
    ]);
    m.insert(r"dqs", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\""#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
    ]);
    m.insert(r"strings", vec![
        Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
    ]);
    m.insert(r"_tmp_0", vec![
        Rule::token(r#"(?m)\\([\\btnfr"\']|u[0-3][0-7]{2}|u[0-7]{1,2})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\""#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
    ]);
    m.insert(r"sqs", vec![
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'", STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
    ]);
    m.insert(r"_tmp_1", vec![
        Rule::token(r#"(?m)\\([\\btnfr"\']|u[0-3][0-7]{2}|u[0-7]{1,2})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'", STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
    ]);
    m.insert(r"name", vec![
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(r"numbers", vec![
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
    ]);
    m.insert(r"fromimport", vec![
        Rule::token(r"(?m)(?:[ \t]|\\\n)+", TEXT),
        Rule::token(r"(?m)[a-zA-Z_][\w.]*", NAME_NAMESPACE),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"operation", vec![
        Rule::token(r"(?m)::", TEXT),
        Rule::bygroups_to(r"(?m)(.*::)([a-zA-Z_]\w*)([ \t]*)(\()", vec![Some(TEXT), Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)], NewState::Pop(1)),
    ]);
    m.insert(r"assert", vec![
        Rule::token_to(r"(?m)(warning|error|fatal)\b", KEYWORD, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for QvtoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
