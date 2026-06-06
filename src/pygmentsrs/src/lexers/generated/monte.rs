//! AUTO-GENERATED from `pygments.pygments.lexers.monte:MonteLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.monte:MonteLexer:monte

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: monte
pub struct MonteLexer;

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
        Rule::token(r"(?m)#[^\n]*\n", COMMENT),
        Rule::token(r"(?m)/\*\*.*?\*/", STRING_DOC),
        Rule::token_to(r"(?m)\bvar\b", KEYWORD_DECLARATION, NewState::Push(vec![r"var"])),
        Rule::token_to(r"(?m)\binterface\b", KEYWORD_DECLARATION, NewState::Push(vec![r"interface"])),
        Rule::token_to(r"(?m)\b(method|to)\b", KEYWORD, NewState::Push(vec![r"method"])),
        Rule::token(r"(?m)\b(bind|def|fn|object)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(as|break|c(?:atch|ontinue)|e(?:lse|scape|x(?:it|(?:port|tend)s))|f(?:inally|or)|guards|i(?:mp(?:lements|ort)|[fn])|m(?:atch|eta)|pass|return|switch|try|via|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?m)[+-]?0x[_0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[_0-9]+\.[_0-9]*([eE][+-]?[_0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?[_0-9]+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING_DOUBLE, NewState::Push(vec![r"char"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"ql"])),
        Rule::token(r"(?m)(!(?:[=~])|%=|\&=|\*(?:\*=|[*=])|\+=|\-(?:[=>])|/=|:=|<(?:<=|=>|[\-<=])|=(?:[=>~])|>(?:>=|[=>])|(?:[\^|])=|[!%&*+\-./<>?\^|~])", OPERATOR),
        Rule::token(r"(?m)[_a-zA-Z]\w*=", OPERATOR_WORD),
        Rule::token(r"(?m)\b(Infinity|M|NaN|Ref|false|null|t(?:hrow|r(?:aceln|ue)))\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)\b(Any|B(?:inding|ool|ytes)|Char|D(?:eepFrozen|ouble)|Empty|Int|List|Map|N(?:ear|ullOk)|S(?:ame|e(?:lfless|t)|tr|ubrangeGuard)|Transparent|Void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(_(?:a(?:ccumulate(?:List|Map)|uditedBy)|b(?:ind|ooleanFlow)|comparer|equalizer|iterForever|loop|ma(?:ke(?:Bytes|Double|FinalSlot|Int|List|M(?:ap|essageDesc)|OrderedSpace|P(?:(?:aram|rotocol)Desc)|S(?:ourceSpan|tring)|V(?:(?:arSlo|erbFace)t))|pExtract|tchSame)|quasiMatcher|s(?:lotToBinding|plitList|uchThat|witchFailed)|validateFor)|b__quasiParser|eval|import|m(?:__quasiParser|ake(?:BrandPair|LazySlot))|s(?:afeScope|imple__quasiParser))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[_a-zA-Z]\w*", NAME),
        Rule::token(r"(?m)\(|\)|\{|\}|\[|\]|:|,", PUNCTUATION),
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token(r"(?m)=", ERROR),
    ]);
    m.insert(r"char", vec![
        Rule::token_to(r"(?m)'", ERROR, NewState::Push(vec![r"root"])),
        Rule::token_to(r#"(?m)(?:\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|\\["\'\\bftnr])"#, STRING_ESCAPE, NewState::Push(vec![r"charEnd"])),
        Rule::token_to(r"(?m).", STRING_CHAR, NewState::Push(vec![r"charEnd"])),
    ]);
    m.insert(r"charEnd", vec![
        Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Pop(2)),
        Rule::token(r"(?m).", ERROR),
    ]);
    m.insert(r"interface", vec![
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token_to(r"(?m)[_a-zA-Z]\w*", NAME_CLASS, NewState::Pop(1)),
        Rule::token(r"(?m)#[^\n]*\n", COMMENT),
        Rule::token(r"(?m)/\*\*.*?\*/", STRING_DOC),
        Rule::token_to(r"(?m)\bvar\b", KEYWORD_DECLARATION, NewState::Push(vec![r"var"])),
        Rule::token_to(r"(?m)\binterface\b", KEYWORD_DECLARATION, NewState::Push(vec![r"interface"])),
        Rule::token_to(r"(?m)\b(method|to)\b", KEYWORD, NewState::Push(vec![r"method"])),
        Rule::token(r"(?m)\b(bind|def|fn|object)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(as|break|c(?:atch|ontinue)|e(?:lse|scape|x(?:it|(?:port|tend)s))|f(?:inally|or)|guards|i(?:mp(?:lements|ort)|[fn])|m(?:atch|eta)|pass|return|switch|try|via|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?m)[+-]?0x[_0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[_0-9]+\.[_0-9]*([eE][+-]?[_0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?[_0-9]+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING_DOUBLE, NewState::Push(vec![r"char"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"ql"])),
        Rule::token(r"(?m)(!(?:[=~])|%=|\&=|\*(?:\*=|[*=])|\+=|\-(?:[=>])|/=|:=|<(?:<=|=>|[\-<=])|=(?:[=>~])|>(?:>=|[=>])|(?:[\^|])=|[!%&*+\-./<>?\^|~])", OPERATOR),
        Rule::token(r"(?m)[_a-zA-Z]\w*=", OPERATOR_WORD),
        Rule::token(r"(?m)\b(Infinity|M|NaN|Ref|false|null|t(?:hrow|r(?:aceln|ue)))\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)\b(Any|B(?:inding|ool|ytes)|Char|D(?:eepFrozen|ouble)|Empty|Int|List|Map|N(?:ear|ullOk)|S(?:ame|e(?:lfless|t)|tr|ubrangeGuard)|Transparent|Void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(_(?:a(?:ccumulate(?:List|Map)|uditedBy)|b(?:ind|ooleanFlow)|comparer|equalizer|iterForever|loop|ma(?:ke(?:Bytes|Double|FinalSlot|Int|List|M(?:ap|essageDesc)|OrderedSpace|P(?:(?:aram|rotocol)Desc)|S(?:ourceSpan|tring)|V(?:(?:arSlo|erbFace)t))|pExtract|tchSame)|quasiMatcher|s(?:lotToBinding|plitList|uchThat|witchFailed)|validateFor)|b__quasiParser|eval|import|m(?:__quasiParser|ake(?:BrandPair|LazySlot))|s(?:afeScope|imple__quasiParser))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[_a-zA-Z]\w*", NAME),
        Rule::token(r"(?m)\(|\)|\{|\}|\[|\]|:|,", PUNCTUATION),
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token(r"(?m)=", ERROR),
    ]);
    m.insert(r"method", vec![
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token_to(r"(?m)[_a-zA-Z]\w*", NAME_FUNCTION, NewState::Pop(1)),
        Rule::token(r"(?m)#[^\n]*\n", COMMENT),
        Rule::token(r"(?m)/\*\*.*?\*/", STRING_DOC),
        Rule::token_to(r"(?m)\bvar\b", KEYWORD_DECLARATION, NewState::Push(vec![r"var"])),
        Rule::token_to(r"(?m)\binterface\b", KEYWORD_DECLARATION, NewState::Push(vec![r"interface"])),
        Rule::token_to(r"(?m)\b(method|to)\b", KEYWORD, NewState::Push(vec![r"method"])),
        Rule::token(r"(?m)\b(bind|def|fn|object)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(as|break|c(?:atch|ontinue)|e(?:lse|scape|x(?:it|(?:port|tend)s))|f(?:inally|or)|guards|i(?:mp(?:lements|ort)|[fn])|m(?:atch|eta)|pass|return|switch|try|via|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?m)[+-]?0x[_0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[_0-9]+\.[_0-9]*([eE][+-]?[_0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?[_0-9]+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING_DOUBLE, NewState::Push(vec![r"char"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"ql"])),
        Rule::token(r"(?m)(!(?:[=~])|%=|\&=|\*(?:\*=|[*=])|\+=|\-(?:[=>])|/=|:=|<(?:<=|=>|[\-<=])|=(?:[=>~])|>(?:>=|[=>])|(?:[\^|])=|[!%&*+\-./<>?\^|~])", OPERATOR),
        Rule::token(r"(?m)[_a-zA-Z]\w*=", OPERATOR_WORD),
        Rule::token(r"(?m)\b(Infinity|M|NaN|Ref|false|null|t(?:hrow|r(?:aceln|ue)))\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)\b(Any|B(?:inding|ool|ytes)|Char|D(?:eepFrozen|ouble)|Empty|Int|List|Map|N(?:ear|ullOk)|S(?:ame|e(?:lfless|t)|tr|ubrangeGuard)|Transparent|Void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(_(?:a(?:ccumulate(?:List|Map)|uditedBy)|b(?:ind|ooleanFlow)|comparer|equalizer|iterForever|loop|ma(?:ke(?:Bytes|Double|FinalSlot|Int|List|M(?:ap|essageDesc)|OrderedSpace|P(?:(?:aram|rotocol)Desc)|S(?:ourceSpan|tring)|V(?:(?:arSlo|erbFace)t))|pExtract|tchSame)|quasiMatcher|s(?:lotToBinding|plitList|uchThat|witchFailed)|validateFor)|b__quasiParser|eval|import|m(?:__quasiParser|ake(?:BrandPair|LazySlot))|s(?:afeScope|imple__quasiParser))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[_a-zA-Z]\w*", NAME),
        Rule::token(r"(?m)\(|\)|\{|\}|\[|\]|:|,", PUNCTUATION),
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token(r"(?m)=", ERROR),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"root"])),
        Rule::token(r#"(?m)(?:\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|\\["\'\\bftnr])"#, STRING_ESCAPE),
        Rule::token(r"(?m)\n", STRING_DOUBLE),
        Rule::token(r"(?m).", STRING_DOUBLE),
    ]);
    m.insert(r"ql", vec![
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"root"])),
        Rule::token(r#"(?m)\$(?:\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|\\["\'\\bftnr])"#, STRING_ESCAPE),
        Rule::token(r"(?m)\$\$", STRING_ESCAPE),
        Rule::token(r"(?m)@@", STRING_ESCAPE),
        Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"qlNest"])),
        Rule::token_to(r"(?m)@\{", STRING_INTERPOL, NewState::Push(vec![r"qlNest"])),
        Rule::token(r"(?m)\$[_a-zA-Z]\w*", NAME),
        Rule::token(r"(?m)@[_a-zA-Z]\w*", NAME),
        Rule::token(r"(?m).", STRING_BACKTICK),
    ]);
    m.insert(r"qlNest", vec![
        Rule::token_to(r"(?m)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?m)#[^\n]*\n", COMMENT),
        Rule::token(r"(?m)/\*\*.*?\*/", STRING_DOC),
        Rule::token_to(r"(?m)\bvar\b", KEYWORD_DECLARATION, NewState::Push(vec![r"var"])),
        Rule::token_to(r"(?m)\binterface\b", KEYWORD_DECLARATION, NewState::Push(vec![r"interface"])),
        Rule::token_to(r"(?m)\b(method|to)\b", KEYWORD, NewState::Push(vec![r"method"])),
        Rule::token(r"(?m)\b(bind|def|fn|object)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(as|break|c(?:atch|ontinue)|e(?:lse|scape|x(?:it|(?:port|tend)s))|f(?:inally|or)|guards|i(?:mp(?:lements|ort)|[fn])|m(?:atch|eta)|pass|return|switch|try|via|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?m)[+-]?0x[_0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[_0-9]+\.[_0-9]*([eE][+-]?[_0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?[_0-9]+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING_DOUBLE, NewState::Push(vec![r"char"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"ql"])),
        Rule::token(r"(?m)(!(?:[=~])|%=|\&=|\*(?:\*=|[*=])|\+=|\-(?:[=>])|/=|:=|<(?:<=|=>|[\-<=])|=(?:[=>~])|>(?:>=|[=>])|(?:[\^|])=|[!%&*+\-./<>?\^|~])", OPERATOR),
        Rule::token(r"(?m)[_a-zA-Z]\w*=", OPERATOR_WORD),
        Rule::token(r"(?m)\b(Infinity|M|NaN|Ref|false|null|t(?:hrow|r(?:aceln|ue)))\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)\b(Any|B(?:inding|ool|ytes)|Char|D(?:eepFrozen|ouble)|Empty|Int|List|Map|N(?:ear|ullOk)|S(?:ame|e(?:lfless|t)|tr|ubrangeGuard)|Transparent|Void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(_(?:a(?:ccumulate(?:List|Map)|uditedBy)|b(?:ind|ooleanFlow)|comparer|equalizer|iterForever|loop|ma(?:ke(?:Bytes|Double|FinalSlot|Int|List|M(?:ap|essageDesc)|OrderedSpace|P(?:(?:aram|rotocol)Desc)|S(?:ourceSpan|tring)|V(?:(?:arSlo|erbFace)t))|pExtract|tchSame)|quasiMatcher|s(?:lotToBinding|plitList|uchThat|witchFailed)|validateFor)|b__quasiParser|eval|import|m(?:__quasiParser|ake(?:BrandPair|LazySlot))|s(?:afeScope|imple__quasiParser))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[_a-zA-Z]\w*", NAME),
        Rule::token(r"(?m)\(|\)|\{|\}|\[|\]|:|,", PUNCTUATION),
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token(r"(?m)=", ERROR),
    ]);
    m.insert(r"var", vec![
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token_to(r"(?m)[_a-zA-Z]\w*", NAME_VARIABLE, NewState::Pop(1)),
        Rule::token(r"(?m)#[^\n]*\n", COMMENT),
        Rule::token(r"(?m)/\*\*.*?\*/", STRING_DOC),
        Rule::token_to(r"(?m)\bvar\b", KEYWORD_DECLARATION, NewState::Push(vec![r"var"])),
        Rule::token_to(r"(?m)\binterface\b", KEYWORD_DECLARATION, NewState::Push(vec![r"interface"])),
        Rule::token_to(r"(?m)\b(method|to)\b", KEYWORD, NewState::Push(vec![r"method"])),
        Rule::token(r"(?m)\b(bind|def|fn|object)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(as|break|c(?:atch|ontinue)|e(?:lse|scape|x(?:it|(?:port|tend)s))|f(?:inally|or)|guards|i(?:mp(?:lements|ort)|[fn])|m(?:atch|eta)|pass|return|switch|try|via|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?m)[+-]?0x[_0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[_0-9]+\.[_0-9]*([eE][+-]?[_0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?[_0-9]+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING_DOUBLE, NewState::Push(vec![r"char"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"ql"])),
        Rule::token(r"(?m)(!(?:[=~])|%=|\&=|\*(?:\*=|[*=])|\+=|\-(?:[=>])|/=|:=|<(?:<=|=>|[\-<=])|=(?:[=>~])|>(?:>=|[=>])|(?:[\^|])=|[!%&*+\-./<>?\^|~])", OPERATOR),
        Rule::token(r"(?m)[_a-zA-Z]\w*=", OPERATOR_WORD),
        Rule::token(r"(?m)\b(Infinity|M|NaN|Ref|false|null|t(?:hrow|r(?:aceln|ue)))\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)\b(Any|B(?:inding|ool|ytes)|Char|D(?:eepFrozen|ouble)|Empty|Int|List|Map|N(?:ear|ullOk)|S(?:ame|e(?:lfless|t)|tr|ubrangeGuard)|Transparent|Void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(_(?:a(?:ccumulate(?:List|Map)|uditedBy)|b(?:ind|ooleanFlow)|comparer|equalizer|iterForever|loop|ma(?:ke(?:Bytes|Double|FinalSlot|Int|List|M(?:ap|essageDesc)|OrderedSpace|P(?:(?:aram|rotocol)Desc)|S(?:ourceSpan|tring)|V(?:(?:arSlo|erbFace)t))|pExtract|tchSame)|quasiMatcher|s(?:lotToBinding|plitList|uchThat|witchFailed)|validateFor)|b__quasiParser|eval|import|m(?:__quasiParser|ake(?:BrandPair|LazySlot))|s(?:afeScope|imple__quasiParser))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[_a-zA-Z]\w*", NAME),
        Rule::token(r"(?m)\(|\)|\{|\}|\[|\]|:|,", PUNCTUATION),
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token(r"(?m)=", ERROR),
    ]);
    Table(m)
}

impl Lexer for MonteLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
