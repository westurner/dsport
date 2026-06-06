//! AUTO-GENERATED from `pygments.pygments.lexers.typoscript:TypoScriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.typoscript:TypoScriptLexer:typoscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: typoscript
pub struct TyposcriptLexer;

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
        Rule::token(r#"(?ms)(?<!(#|\'|"))(?:#(?!(?:[a-fA-F0-9]{6}|[a-fA-F0-9]{3}))[^\n#]+|//[^\n]*)"#, COMMENT),
        Rule::token(r"(?ms)/\*(?:(?!\*/).)*\*/", COMMENT),
        Rule::token(r"(?ms)(\s*#\s*\n)", COMMENT),
        Rule::bygroups(r"(?ms)(\{)(\$)((?:[\w\-]+\.)*)([\w\-]+)(\})", vec![Some(STRING_SYMBOL), Some(OPERATOR), Some(NAME_CONSTANT), Some(NAME_CONSTANT), Some(STRING_SYMBOL)]),
        Rule::bygroups(r"(?ms)(\{)([\w\-]+)(\s*:\s*)([\w\-]+)(\})", vec![Some(STRING_SYMBOL), Some(NAME_CONSTANT), Some(OPERATOR), Some(NAME_CONSTANT), Some(STRING_SYMBOL)]),
        Rule::token(r"(?ms)(#[a-fA-F0-9]{6}\b|#[a-fA-F0-9]{3}\b)", STRING_CHAR),
        Rule::using_lexer(r"(?ms)<\S[^\n>]*>", "typoscripthtmldata", None),
        Rule::token(r"(?ms)&[^;\n]*;", STRING),
        Rule::bygroups_g(r"(?ms)(?s)(_CSS_DEFAULT_STYLE)(\s*)(\()(.*(?=\n\)))", vec![Some(GroupAction::Token(NAME_CLASS)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(STRING_SYMBOL)), Some(GroupAction::UsingLexer { alias: "typoscriptcssdata", state: None })]),
        Rule::token(r#"(?ms)(EXT|FILE|LLL):[^}\n"]*"#, STRING),
        Rule::bygroups(r"(?ms)(?![^\w\-])([\w\-]+(?:/[\w\-]+)+/?)(\S*\n)", vec![Some(STRING), Some(STRING)]),
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::bygroups(r"(?ms)(?i)(\[)(browser|compatVersion|dayofmonth|dayofweek|dayofyear|device|ELSE|END|GLOBAL|globalString|globalVar|hostname|hour|IP|language|loginUser|loginuser|minute|month|page|PIDinRootline|PIDupinRootline|system|treeLevel|useragent|userFunc|usergroup|version)([^\]]*)(\])", vec![Some(STRING_SYMBOL), Some(NAME_CONSTANT), Some(TEXT), Some(STRING_SYMBOL)]),
        Rule::token(r"(?ms)(?=[\w\-])(HTMLparser|HTMLparser_tags|addParams|cache|encapsLines|filelink|if|imageLinkWrap|imgResource|makelinks|numRows|numberFormat|parseFunc|replacement|round|select|split|stdWrap|strPad|tableStyle|tags|textStyle|typolink)(?![\w\-])", NAME_FUNCTION),
        Rule::bygroups(r"(?ms)(?:(=?\s*<?\s+|^\s*))(cObj|field|config|content|constants|FEData|file|frameset|includeLibs|lib|page|plugin|register|resources|sitemap|sitetitle|styles|temp|tt_[^:.\s]*|types|xmlnews|INCLUDE_TYPOSCRIPT|_CSS_DEFAULT_STYLE|_DEFAULT_PI_VARS|_LOCAL_LANG)(?![\w\-])", vec![Some(OPERATOR), Some(NAME_BUILTIN)]),
        Rule::token(r"(?ms)(?=[\w\-])(CASE|CLEARGIF|COA|COA_INT|COBJ_ARRAY|COLUMNS|CONTENT|CTABLE|EDITPANEL|FILE|FILES|FLUIDTEMPLATE|FORM|HMENU|HRULER|HTML|IMAGE|IMGTEXT|IMG_RESOURCE|LOAD_REGISTER|MEDIA|MULTIMEDIA|OTABLE|PAGE|QTOBJECT|RECORDS|RESTORE_REGISTER|SEARCHRESULT|SVG|SWFOBJECT|TEMPLATE|TEXT|USER|USER_INT)(?![\w\-])", NAME_CLASS),
        Rule::token(r"(?ms)(?=[\w\-])(ACTIFSUBRO|ACTIFSUB|ACTRO|ACT|CURIFSUBRO|CURIFSUB|CURRO|CUR|IFSUBRO|IFSUB|NO|SPC|USERDEF1RO|USERDEF1|USERDEF2RO|USERDEF2|USRRO|USR)", NAME_CLASS),
        Rule::token(r"(?ms)(?=[\w\-])(GMENU_FOLDOUT|GMENU_LAYERS|GMENU|IMGMENUITEM|IMGMENU|JSMENUITEM|JSMENU|TMENUITEM|TMENU_LAYERS|TMENU)", NAME_CLASS),
        Rule::token(r"(?ms)(?=[\w\-])(PHP_SCRIPT(_EXT|_INT)?)", NAME_CLASS),
        Rule::token(r"(?ms)(?=[\w\-])(userFunc)(?![\w\-])", NAME_FUNCTION),
        Rule::token(r"(?ms)[,.]", PUNCTUATION),
        Rule::token(r"(?ms)[<>,:=.*%+|]", OPERATOR),
        Rule::token(r"(?ms)[{}()\[\]\\]", STRING_SYMBOL),
        Rule::token(r"(?ms)0x[0-9A-Fa-f]+t?", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?ms)(###\w+###)", NAME_CONSTANT),
        Rule::token(r#"(?ms)[\w"\-!/&;]+"#, TEXT),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r#"(?ms)(?<!(#|\'|"))(?:#(?!(?:[a-fA-F0-9]{6}|[a-fA-F0-9]{3}))[^\n#]+|//[^\n]*)"#, COMMENT),
        Rule::token(r"(?ms)/\*(?:(?!\*/).)*\*/", COMMENT),
        Rule::token(r"(?ms)(\s*#\s*\n)", COMMENT),
    ]);
    m.insert(r"constant", vec![
        Rule::bygroups(r"(?ms)(\{)(\$)((?:[\w\-]+\.)*)([\w\-]+)(\})", vec![Some(STRING_SYMBOL), Some(OPERATOR), Some(NAME_CONSTANT), Some(NAME_CONSTANT), Some(STRING_SYMBOL)]),
        Rule::bygroups(r"(?ms)(\{)([\w\-]+)(\s*:\s*)([\w\-]+)(\})", vec![Some(STRING_SYMBOL), Some(NAME_CONSTANT), Some(OPERATOR), Some(NAME_CONSTANT), Some(STRING_SYMBOL)]),
        Rule::token(r"(?ms)(#[a-fA-F0-9]{6}\b|#[a-fA-F0-9]{3}\b)", STRING_CHAR),
    ]);
    m.insert(r"html", vec![
        Rule::using_lexer(r"(?ms)<\S[^\n>]*>", "typoscripthtmldata", None),
        Rule::token(r"(?ms)&[^;\n]*;", STRING),
        Rule::bygroups_g(r"(?ms)(?s)(_CSS_DEFAULT_STYLE)(\s*)(\()(.*(?=\n\)))", vec![Some(GroupAction::Token(NAME_CLASS)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(STRING_SYMBOL)), Some(GroupAction::UsingLexer { alias: "typoscriptcssdata", state: None })]),
    ]);
    m.insert(r"label", vec![
        Rule::token(r#"(?ms)(EXT|FILE|LLL):[^}\n"]*"#, STRING),
        Rule::bygroups(r"(?ms)(?![^\w\-])([\w\-]+(?:/[\w\-]+)+/?)(\S*\n)", vec![Some(STRING), Some(STRING)]),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?ms)\s+", TEXT),
    ]);
    m.insert(r"keywords", vec![
        Rule::bygroups(r"(?ms)(?i)(\[)(browser|compatVersion|dayofmonth|dayofweek|dayofyear|device|ELSE|END|GLOBAL|globalString|globalVar|hostname|hour|IP|language|loginUser|loginuser|minute|month|page|PIDinRootline|PIDupinRootline|system|treeLevel|useragent|userFunc|usergroup|version)([^\]]*)(\])", vec![Some(STRING_SYMBOL), Some(NAME_CONSTANT), Some(TEXT), Some(STRING_SYMBOL)]),
        Rule::token(r"(?ms)(?=[\w\-])(HTMLparser|HTMLparser_tags|addParams|cache|encapsLines|filelink|if|imageLinkWrap|imgResource|makelinks|numRows|numberFormat|parseFunc|replacement|round|select|split|stdWrap|strPad|tableStyle|tags|textStyle|typolink)(?![\w\-])", NAME_FUNCTION),
        Rule::bygroups(r"(?ms)(?:(=?\s*<?\s+|^\s*))(cObj|field|config|content|constants|FEData|file|frameset|includeLibs|lib|page|plugin|register|resources|sitemap|sitetitle|styles|temp|tt_[^:.\s]*|types|xmlnews|INCLUDE_TYPOSCRIPT|_CSS_DEFAULT_STYLE|_DEFAULT_PI_VARS|_LOCAL_LANG)(?![\w\-])", vec![Some(OPERATOR), Some(NAME_BUILTIN)]),
        Rule::token(r"(?ms)(?=[\w\-])(CASE|CLEARGIF|COA|COA_INT|COBJ_ARRAY|COLUMNS|CONTENT|CTABLE|EDITPANEL|FILE|FILES|FLUIDTEMPLATE|FORM|HMENU|HRULER|HTML|IMAGE|IMGTEXT|IMG_RESOURCE|LOAD_REGISTER|MEDIA|MULTIMEDIA|OTABLE|PAGE|QTOBJECT|RECORDS|RESTORE_REGISTER|SEARCHRESULT|SVG|SWFOBJECT|TEMPLATE|TEXT|USER|USER_INT)(?![\w\-])", NAME_CLASS),
        Rule::token(r"(?ms)(?=[\w\-])(ACTIFSUBRO|ACTIFSUB|ACTRO|ACT|CURIFSUBRO|CURIFSUB|CURRO|CUR|IFSUBRO|IFSUB|NO|SPC|USERDEF1RO|USERDEF1|USERDEF2RO|USERDEF2|USRRO|USR)", NAME_CLASS),
        Rule::token(r"(?ms)(?=[\w\-])(GMENU_FOLDOUT|GMENU_LAYERS|GMENU|IMGMENUITEM|IMGMENU|JSMENUITEM|JSMENU|TMENUITEM|TMENU_LAYERS|TMENU)", NAME_CLASS),
        Rule::token(r"(?ms)(?=[\w\-])(PHP_SCRIPT(_EXT|_INT)?)", NAME_CLASS),
        Rule::token(r"(?ms)(?=[\w\-])(userFunc)(?![\w\-])", NAME_FUNCTION),
    ]);
    m.insert(r"punctuation", vec![
        Rule::token(r"(?ms)[,.]", PUNCTUATION),
    ]);
    m.insert(r"operator", vec![
        Rule::token(r"(?ms)[<>,:=.*%+|]", OPERATOR),
    ]);
    m.insert(r"structure", vec![
        Rule::token(r"(?ms)[{}()\[\]\\]", STRING_SYMBOL),
    ]);
    m.insert(r"literal", vec![
        Rule::token(r"(?ms)0x[0-9A-Fa-f]+t?", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?ms)(###\w+###)", NAME_CONSTANT),
    ]);
    m.insert(r"other", vec![
        Rule::token(r#"(?ms)[\w"\-!/&;]+"#, TEXT),
    ]);
    Table(m)
}

impl Lexer for TyposcriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
