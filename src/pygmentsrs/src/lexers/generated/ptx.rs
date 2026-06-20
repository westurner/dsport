#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.ptx:PtxLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ptx:PtxLexer:ptx

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ptx
pub struct PtxLexer;

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
        Rule::token(r"(?m)(\n|\s+)+", WHITESPACE),
        Rule::token(r"(?m)//.*?\n", COMMENT),
        Rule::token(r#"(?m)(([-a-zA-Z$._][\w\-$.]*|"[^"]*?")|(\d+))\s*:"#, NAME_LABEL),
        Rule::token(r"(?m)(a(?:bs|ctivemask|dd(?:(?:c)?)|lloca|nd|pplypriority|tom)|b(?:ar(?:(?:rier)?)|f(?:ind|[ei])|msk|r(?:ev|kpt|[ax]))|c(?:all|lz|not|o(?:pysign|s)|p|reatepolicy|vt(?:(?:a)?))|d(?:i(?:scard|v)|p(?:(?:[24])a))|e(?:lect|x(?:2|it))|f(?:ence|ma|ns)|g(?:etctarank|riddepcontrol)|is(?:(?:spac|typ)ep)|l(?:d(?:(?:matrix|u)?)|g2|op3)|m(?:a(?:d(?:24|c)|pa|tch|[dx])|barrier|embar|in|ma|ov(?:(?:matrix)?)|ul(?:(?:24|timem)?))|n(?:anosleep|eg|ot)|or|p(?:mevent|opc|r(?:efetch(?:(?:u)?)|mt))|r(?:cp|e(?:dux|[dmt])|sqrt)|s(?:ad|e(?:lp|t(?:(?:maxnreg|p)?))|h(?:fl|[flr])|in|lct|qrt|t(?:(?:ack(?:(?:restor|sav)e)|matrix)?)|u(?:bc|ld|red|st|[bq])|zext)|t(?:anh|e(?:stp|x)|ld4|rap|xq)|v(?:a(?:bsdiff(?:(?:[24])?)|dd(?:(?:[24])?)|vrg(?:[24]))|m(?:a(?:x(?:[24])|[dx])|in(?:(?:[24])?))|ote|s(?:et(?:(?:[24])?)|h(?:[lr])|ub(?:(?:[24])?)))|w(?:(?:(?:g)?)mma)|xor)", KEYWORD),
        Rule::token(r"(?m)(\.(?:const|global|loc(?:(?:al)?)|param|s(?:hared|reg)|tex|wide)|reg)", KEYWORD_PSEUDO),
        Rule::token(r"(?m)(\.(?:a(?:ddress_size|li(?:as|gn))|branchtargets|c(?:all(?:prototype|targets)|o(?:mmon|nst))|e(?:ntry|x(?:plicitcluster|tern))|f(?:ile|unc)|global|loc(?:(?:al)?)|m(?:ax(?:clusterrank|n(?:ctapersm|reg|tid))|innctapersm)|noreturn|p(?:aram|ragma)|re(?:g|qn(?:ctapercluster|tid))|s(?:ection|hared|reg)|t(?:arget|ex)|v(?:ersion|isible)|weak))", KEYWORD_RESERVED),
        Rule::token(r"(?m)(\.(?:b(?:16|32|64|8)|f(?:16(?:(?:x2)?)|32|64)|pred|s(?:16|32|64|8)|u(?:16|32|64|8)))", KEYWORD_TYPE),
        Rule::token(r#"(?m)%([-a-zA-Z$._][\w\-$.]*|"[^"]*?")"#, NAME_VARIABLE),
        Rule::token(r"(?m)%\d+", TokenType::new(&["Name", "Variable", "Anonymous"])),
        Rule::token(r#"(?m)c?"[^"]*?""#, STRING),
        Rule::token(r#"(?m)([-a-zA-Z$._][\w\-$.]*|"[^"]*?")"#, NAME_VARIABLE),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)[*+-/]", OPERATOR),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER),
        Rule::token(r"(?m)-?\d+(?:[.]\d+)?(?:[eE][-+]?\d+(?:[.]\d+)?)?", NUMBER),
        Rule::token(r"(?m)[=<>{}\[\]()*.,!]|x\b", PUNCTUATION),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)(\n|\s+)+", WHITESPACE),
            Rule::token(r"(?m)//.*?\n", COMMENT),
        ],
    );
    m.insert(r"keyword", vec![
        Rule::token(r"(?m)(a(?:bs|ctivemask|dd(?:(?:c)?)|lloca|nd|pplypriority|tom)|b(?:ar(?:(?:rier)?)|f(?:ind|[ei])|msk|r(?:ev|kpt|[ax]))|c(?:all|lz|not|o(?:pysign|s)|p|reatepolicy|vt(?:(?:a)?))|d(?:i(?:scard|v)|p(?:(?:[24])a))|e(?:lect|x(?:2|it))|f(?:ence|ma|ns)|g(?:etctarank|riddepcontrol)|is(?:(?:spac|typ)ep)|l(?:d(?:(?:matrix|u)?)|g2|op3)|m(?:a(?:d(?:24|c)|pa|tch|[dx])|barrier|embar|in|ma|ov(?:(?:matrix)?)|ul(?:(?:24|timem)?))|n(?:anosleep|eg|ot)|or|p(?:mevent|opc|r(?:efetch(?:(?:u)?)|mt))|r(?:cp|e(?:dux|[dmt])|sqrt)|s(?:ad|e(?:lp|t(?:(?:maxnreg|p)?))|h(?:fl|[flr])|in|lct|qrt|t(?:(?:ack(?:(?:restor|sav)e)|matrix)?)|u(?:bc|ld|red|st|[bq])|zext)|t(?:anh|e(?:stp|x)|ld4|rap|xq)|v(?:a(?:bsdiff(?:(?:[24])?)|dd(?:(?:[24])?)|vrg(?:[24]))|m(?:a(?:x(?:[24])|[dx])|in(?:(?:[24])?))|ote|s(?:et(?:(?:[24])?)|h(?:[lr])|ub(?:(?:[24])?)))|w(?:(?:(?:g)?)mma)|xor)", KEYWORD),
        Rule::token(r"(?m)(\.(?:const|global|loc(?:(?:al)?)|param|s(?:hared|reg)|tex|wide)|reg)", KEYWORD_PSEUDO),
        Rule::token(r"(?m)(\.(?:a(?:ddress_size|li(?:as|gn))|branchtargets|c(?:all(?:prototype|targets)|o(?:mmon|nst))|e(?:ntry|x(?:plicitcluster|tern))|f(?:ile|unc)|global|loc(?:(?:al)?)|m(?:ax(?:clusterrank|n(?:ctapersm|reg|tid))|innctapersm)|noreturn|p(?:aram|ragma)|re(?:g|qn(?:ctapercluster|tid))|s(?:ection|hared|reg)|t(?:arget|ex)|v(?:ersion|isible)|weak))", KEYWORD_RESERVED),
        Rule::token(r"(?m)(\.(?:b(?:16|32|64|8)|f(?:16(?:(?:x2)?)|32|64)|pred|s(?:16|32|64|8)|u(?:16|32|64|8)))", KEYWORD_TYPE),
    ]);
    Table(m)
}

impl Lexer for PtxLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
