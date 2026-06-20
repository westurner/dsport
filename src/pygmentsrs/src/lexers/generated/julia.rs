#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.julia:JuliaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.julia:JuliaLexer:julia

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: julia, jl
pub struct JuliaLexer;

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
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token_to(r"(?m)#=", COMMENT_MULTILINE, NewState::Push(vec![r"blockcomment"])),
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m)[\[\](),;]", PUNCTUATION),
        Rule::bygroups(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\s*)(:)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(NAME), Some(WHITESPACE), Some(OPERATOR), Some(NAME)]),
        Rule::token(r"(?m)(?<![\]):<>\d.])(:(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", STRING_SYMBOL),
        Rule::bygroups(r"(?m)(?<=::)(\s*)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))\b(?![(\[])", vec![Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\s*)([<>]:)(\s*)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))\b(?![(\[])", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)([<>]:)(\s*)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))\b(?![(\[])", vec![Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)\b((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\s*)([<>]:)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)(!=(?:(?:=)?)|\$=|%=|\&(?:[&=])|\*=|\+(?:[+=])|\-(?:\->|[=>])|/(?:/=|[/=])|:(?:[:=])|<(?:\-\-(?:(?:>)?)|<=|[:<=|])|=(?:==|[=>])|>(?:>(?:>=|[=>])|[:=>])|\\=|\^=|\|(?:[=>|])|÷=|≕'|⊻=|[!$%&*+\-/:<=>?\\\^|~¦¬±×÷…⁝⅋↑→↓↔↚↛↜↝↞↠↢↣↤↦↩↪↫↬↮↶↷↺↻↼↽⇀⇁⇄⇆⇇⇉⇋⇌⇍⇎⇏⇐⇒⇔⇚⇛⇜⇝⇠⇢⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿∈∉∊∋∌∍∓∔∗∘∙√∛∜∝∤∥∦∧∨∩∪∷∸∺∻∽∾≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊞⊟⊠⊡⊢⊣⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⊻⊼⊽⋄⋅⋆⋇⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌿▷⟂⟈⟉⟑⟒⟕⟖⟗⟰⟱⟵⟶⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤈⤉⤊⤋⤌⤍⤎⤏⤐⤑⤒⤓⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥉⥊⥋⥌⥍⥎⥏⥐⥑⥒⥓⥔⥕⥖⥗⥘⥙⥚⥛⥜⥝⥞⥟⥠⥡⥢⥣⥤⥥⥦⥧⥨⥩⥪⥫⥬⥭⥮⥯⥰⦷⦸⦼⦾⦿⧀⧁⧡⧣⧤⧥⧴⧶⧷⧺⧻⨇⨈⨝⨟⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨰⨱⨲⨳⨴⨵⨶⨷⨸⨹⨺⨻⨼⨽⩀⩁⩂⩃⩄⩅⩊⩋⩌⩍⩎⩏⩐⩑⩒⩓⩔⩕⩖⩗⩘⩚⩛⩜⩝⩞⩟⩠⩡⩢⩣⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫛⫷⫸⫹⫺⬰⬱⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￪￫￬])[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*", OPERATOR),
        Rule::token(r"(?m)(\.(?:!=(?:(?:=)?)|%=|\&=|\*=|\+(?:[+=])|\-(?:\->|=)|/(?:/=|[/=])|<(?:\-\-(?:(?:>)?)|<=|[:<=|])|=(?:==|[=>])|>(?:>(?:>=|[=>])|[:=>])|\\=|\^=|\|(?:[=>])|÷=|≕'|⊻=|[!%&*+\-/<=>\\\^|~¦¬±×÷…⁝⅋↑→↓↔↚↛↜↝↞↠↢↣↤↦↩↪↫↬↮↶↷↺↻↼↽⇀⇁⇄⇆⇇⇉⇋⇌⇍⇎⇏⇐⇒⇔⇚⇛⇜⇝⇠⇢⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿∈∉∊∋∌∍∓∔∗∘∙√∛∜∝∤∥∦∧∨∩∪∷∸∺∻∽∾≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊞⊟⊠⊡⊢⊣⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⊻⊼⊽⋄⋅⋆⋇⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌿▷⟂⟈⟉⟑⟒⟕⟖⟗⟰⟱⟵⟶⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤈⤉⤊⤋⤌⤍⤎⤏⤐⤑⤒⤓⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥉⥊⥋⥌⥍⥎⥏⥐⥑⥒⥓⥔⥕⥖⥗⥘⥙⥚⥛⥜⥝⥞⥟⥠⥡⥢⥣⥤⥥⥦⥧⥨⥩⥪⥫⥬⥭⥮⥯⥰⦷⦸⦼⦾⦿⧀⧁⧡⧣⧤⧥⧴⧶⧷⧺⧻⨇⨈⨝⨟⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨰⨱⨲⨳⨴⨵⨶⨷⨸⨹⨺⨻⨼⨽⩀⩁⩂⩃⩄⩅⩊⩋⩌⩍⩎⩏⩐⩑⩒⩓⩔⩕⩖⩗⩘⩚⩛⩜⩝⩞⩟⩠⩡⩢⩣⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫛⫷⫸⫹⫺⬰⬱⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￪￫￬]))[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*", OPERATOR),
        Rule::token(r"(?m)(\.\.(?:(?:\.)?))", OPERATOR),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,3}|\\u[a-fA-F0-9]{1,4}|\\U[a-fA-F0-9]{1,6}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(?<=[.\w)\]])(\'[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*)+", OPERATOR),
        Rule::bygroups_to(r#"(?m)(raw)(""")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"tqrawstring"])),
        Rule::bygroups_to(r#"(?m)(raw)(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"rawstring"])),
        Rule::bygroups_to(r#"(?m)(r)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_REGEX)], NewState::Push(vec![r"tqregex"])),
        Rule::bygroups_to(r#"(?m)(r)(")"#, vec![Some(STRING_AFFIX), Some(STRING_REGEX)], NewState::Push(vec![r"regex"])),
        Rule::bygroups_to(r#"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(""")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"tqstring"])),
        Rule::bygroups_to(r#"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::bygroups_to(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(```)", vec![Some(STRING_AFFIX), Some(STRING_BACKTICK)], NewState::Push(vec![r"tqcommand"])),
        Rule::bygroups_to(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(`)", vec![Some(STRING_AFFIX), Some(STRING_BACKTICK)], NewState::Push(vec![r"command"])),
        Rule::bygroups_to(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\{)", vec![Some(KEYWORD_TYPE), Some(PUNCTUATION)], NewState::Push(vec![r"curly"])),
        Rule::bygroups(r"(?m)(where)(\s+)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::token_to(r"(?m)(\{)", PUNCTUATION, NewState::Push(vec![r"curly"])),
        Rule::bygroups(r"(?m)(abstract|primitive)([ \t]+)(type\b)([\s()]+)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(TEXT), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)(mutable(?=[ \t]))?([ \t]+)?(struct\b)([\s()]+)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(TEXT), Some(KEYWORD_TYPE)]),
        Rule::token(r"(?m)@(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", NAME_DECORATOR),
        Rule::token(r"(?m)@(!=(?:(?:=)?)|\$=|%=|\&(?:[&=])|\*=|\+(?:[+=])|\-(?:\->|[=>])|\.\.|/(?:/=|[/=])|:(?:[:=])|<(?:\-\-(?:(?:>)?)|<=|[:<=|])|=(?:==|[=>])|>(?:>(?:>=|[=>])|[:=>])|\\=|\^=|\|(?:[=>|])|÷=|≕'|⊻=|[!$%&*+\-./:<=>?\\\^|~¦¬±×÷…⁝⅋↑→↓↔↚↛↜↝↞↠↢↣↤↦↩↪↫↬↮↶↷↺↻↼↽⇀⇁⇄⇆⇇⇉⇋⇌⇍⇎⇏⇐⇒⇔⇚⇛⇜⇝⇠⇢⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿∈∉∊∋∌∍∓∔∗∘∙√∛∜∝∤∥∦∧∨∩∪∷∸∺∻∽∾≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊞⊟⊠⊡⊢⊣⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⊻⊼⊽⋄⋅⋆⋇⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌿▷⟂⟈⟉⟑⟒⟕⟖⟗⟰⟱⟵⟶⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤈⤉⤊⤋⤌⤍⤎⤏⤐⤑⤒⤓⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥉⥊⥋⥌⥍⥎⥏⥐⥑⥒⥓⥔⥕⥖⥗⥘⥙⥚⥛⥜⥝⥞⥟⥠⥡⥢⥣⥤⥥⥦⥧⥨⥩⥪⥫⥬⥭⥮⥯⥰⦷⦸⦼⦾⦿⧀⧁⧡⧣⧤⧥⧴⧶⧷⧺⧻⨇⨈⨝⨟⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨰⨱⨲⨳⨴⨵⨶⨷⨸⨹⨺⨻⨼⨽⩀⩁⩂⩃⩄⩅⩊⩋⩌⩍⩎⩏⩐⩑⩒⩓⩔⩕⩖⩗⩘⩚⩛⩜⩝⩞⩟⩠⩡⩢⩣⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫛⫷⫸⫹⫺⬰⬱⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￪￫￬])[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*", NAME_DECORATOR),
        Rule::token(r"(?m)(b(?:aremodule|egin|reak)|c(?:atch|call|on(?:st|tinue))|do|e(?:lse(?:(?:if)?)|nd|xport)|f(?:inally|or|unction)|global|i(?:mport|sa|[fn])|l(?:et|ocal)|m(?:acro|odule)|quote|return|try|using|wh(?:(?:er|il)e))\b", KEYWORD),
        Rule::token(r"(?m)(A(?:bstract(?:Array|Cha(?:nnel|r)|Di(?:ct|splay)|Float|Irrational|Mat(?:ch|rix)|Pattern|Range|S(?:et|tring)|UnitRange|Vec(?:OrMat|tor))|ny|r(?:gumentError|ray)|ssertionError)|B(?:i(?:g(?:(?:Floa|In)t)|t(?:Array|Matrix|Set|Vector))|o(?:ol|undsError))|C(?:a(?:pturedException|rtesianInd(?:ex|ices))|char|double|float|ha(?:nnel|r)|int(?:(?:max_t)?)|long(?:(?:long)?)|md|o(?:lon|mp(?:lex(?:(?:F(?:16|32|64))?)|os(?:(?:edFunc|iteExcep)tion))|ndition)|ptrdiff_t|s(?:hort|ize_t|size_t|tring)|u(?:char|int(?:(?:max_t)?)|long(?:(?:long)?)|short)|void|w(?:char_t|string))|D(?:ataType|ense(?:Array|Matrix|Vec(?:OrMat|tor))|i(?:ct|m(?:ensionMismatch|s)|videError)|omainError)|E(?:OFError|num|rrorException|x(?:ception|p(?:onentialBackOff|r)))|F(?:loat(?:16|32|64)|unction)|GlobalRef|HTML|I(?:O(?:(?:Buffer|Context|Stream)?)|dDict|n(?:dex(?:Cartesian|Linear|Style)|exactError|itError|t(?:(?:1(?:28|6)|32|64|8|e(?:ger|rruptException))?)|validStateException)|rrational)|KeyError|L(?:in(?:Range|e(?:NumberNode|arIndices))|oadError)|M(?:IME|atrix|ethod(?:(?:Error)?)|issing(?:(?:Exception)?)|odule)|N(?:Tuple|amedTuple|othing|umber)|O(?:rdinalRange|(?:utOfMemory|verflow)Error)|P(?:a(?:ir|rtialQuickSort)|ermutedDimsArray|ipe|rocessFailedException|tr)|QuoteNode|R(?:a(?:tional|wFD)|e(?:a(?:dOnlyMemoryError|l)|entrantLock|f|gex(?:(?:Match)?))|oundingMode)|S(?:e(?:(?:(?:gmentationFaul)?)t)|igned|ome|t(?:ackOverflowError|epRange(?:(?:Len)?)|ri(?:ded(?:Array|Matrix|Vec(?:OrMat|tor))|ng(?:(?:IndexError)?)))|ub(?:Array|(?:(?:stitution)?)String)|y(?:mbol|stemError))|T(?:ask(?:(?:FailedException)?)|ext(?:(?:Display)?)|imer|uple|ype(?:(?:(?:Erro|Va)r)?))|U(?:Int(?:(?:1(?:28|6)|32|64|8)?)|n(?:def(?:(?:Initialize|(?:Keyword|Ref|Var)Erro)r)|i(?:on(?:(?:All)?)|tRange)|signed))|V(?:a(?:l|rarg)|e(?:c(?:Element|OrMat|tor)|rsionNumber))|Weak(?:KeyDict|Ref))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(ARGS|C_NULL|DEPOT_PATH|EN(?:DIAN_BOM|V)|In(?:f(?:(?:16|32|64)?)|sertionSort)|LOAD_PATH|MergeSort|NaN(?:(?:16|32|64)?)|PROGRAM_FILE|QuickSort|Round(?:Down|FromZero|Nearest(?:(?:Ties(?:Away|Up))?)|ToZero|Up)|VERSION|devnull|false|im|missing|nothing|pi|std(?:err|in|out)|true|undef|[πℯ])\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", NAME),
        Rule::token(r"(?m)(\d+((_\d+)+)?\.(?!\.)(\d+((_\d+)+)?)?|\.\d+((_\d+)+)?)([eEf][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+((_\d+)+)?[eEf][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[a-fA-F0-9]+((_[a-fA-F0-9]+)+)?(\.([a-fA-F0-9]+((_[a-fA-F0-9]+)+)?)?)?p[+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)0b[01]+((_[01]+)+)?", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+((_[0-7]+)+)?", NUMBER_OCT),
        Rule::token(r"(?m)0x[a-fA-F0-9]+((_[a-fA-F0-9]+)+)?", NUMBER_HEX),
        Rule::token(r"(?m)\d+((_\d+)+)?", NUMBER_INTEGER),
        Rule::token(r"(?m)(\.)", OPERATOR),
    ]);
    m.insert(
        r"blockcomment",
        vec![
            Rule::token(r"(?m)[^=#]", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)#=", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)=#", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[=#]", COMMENT_MULTILINE),
        ],
    );
    m.insert(r"curly", vec![
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", KEYWORD_TYPE),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token_to(r"(?m)#=", COMMENT_MULTILINE, NewState::Push(vec![r"blockcomment"])),
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m)[\[\](),;]", PUNCTUATION),
        Rule::bygroups(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\s*)(:)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(NAME), Some(WHITESPACE), Some(OPERATOR), Some(NAME)]),
        Rule::token(r"(?m)(?<![\]):<>\d.])(:(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", STRING_SYMBOL),
        Rule::bygroups(r"(?m)(?<=::)(\s*)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))\b(?![(\[])", vec![Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\s*)([<>]:)(\s*)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))\b(?![(\[])", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)([<>]:)(\s*)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))\b(?![(\[])", vec![Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)\b((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\s*)([<>]:)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)(!=(?:(?:=)?)|\$=|%=|\&(?:[&=])|\*=|\+(?:[+=])|\-(?:\->|[=>])|/(?:/=|[/=])|:(?:[:=])|<(?:\-\-(?:(?:>)?)|<=|[:<=|])|=(?:==|[=>])|>(?:>(?:>=|[=>])|[:=>])|\\=|\^=|\|(?:[=>|])|÷=|≕'|⊻=|[!$%&*+\-/:<=>?\\\^|~¦¬±×÷…⁝⅋↑→↓↔↚↛↜↝↞↠↢↣↤↦↩↪↫↬↮↶↷↺↻↼↽⇀⇁⇄⇆⇇⇉⇋⇌⇍⇎⇏⇐⇒⇔⇚⇛⇜⇝⇠⇢⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿∈∉∊∋∌∍∓∔∗∘∙√∛∜∝∤∥∦∧∨∩∪∷∸∺∻∽∾≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊞⊟⊠⊡⊢⊣⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⊻⊼⊽⋄⋅⋆⋇⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌿▷⟂⟈⟉⟑⟒⟕⟖⟗⟰⟱⟵⟶⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤈⤉⤊⤋⤌⤍⤎⤏⤐⤑⤒⤓⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥉⥊⥋⥌⥍⥎⥏⥐⥑⥒⥓⥔⥕⥖⥗⥘⥙⥚⥛⥜⥝⥞⥟⥠⥡⥢⥣⥤⥥⥦⥧⥨⥩⥪⥫⥬⥭⥮⥯⥰⦷⦸⦼⦾⦿⧀⧁⧡⧣⧤⧥⧴⧶⧷⧺⧻⨇⨈⨝⨟⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨰⨱⨲⨳⨴⨵⨶⨷⨸⨹⨺⨻⨼⨽⩀⩁⩂⩃⩄⩅⩊⩋⩌⩍⩎⩏⩐⩑⩒⩓⩔⩕⩖⩗⩘⩚⩛⩜⩝⩞⩟⩠⩡⩢⩣⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫛⫷⫸⫹⫺⬰⬱⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￪￫￬])[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*", OPERATOR),
        Rule::token(r"(?m)(\.(?:!=(?:(?:=)?)|%=|\&=|\*=|\+(?:[+=])|\-(?:\->|=)|/(?:/=|[/=])|<(?:\-\-(?:(?:>)?)|<=|[:<=|])|=(?:==|[=>])|>(?:>(?:>=|[=>])|[:=>])|\\=|\^=|\|(?:[=>])|÷=|≕'|⊻=|[!%&*+\-/<=>\\\^|~¦¬±×÷…⁝⅋↑→↓↔↚↛↜↝↞↠↢↣↤↦↩↪↫↬↮↶↷↺↻↼↽⇀⇁⇄⇆⇇⇉⇋⇌⇍⇎⇏⇐⇒⇔⇚⇛⇜⇝⇠⇢⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿∈∉∊∋∌∍∓∔∗∘∙√∛∜∝∤∥∦∧∨∩∪∷∸∺∻∽∾≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊞⊟⊠⊡⊢⊣⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⊻⊼⊽⋄⋅⋆⋇⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌿▷⟂⟈⟉⟑⟒⟕⟖⟗⟰⟱⟵⟶⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤈⤉⤊⤋⤌⤍⤎⤏⤐⤑⤒⤓⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥉⥊⥋⥌⥍⥎⥏⥐⥑⥒⥓⥔⥕⥖⥗⥘⥙⥚⥛⥜⥝⥞⥟⥠⥡⥢⥣⥤⥥⥦⥧⥨⥩⥪⥫⥬⥭⥮⥯⥰⦷⦸⦼⦾⦿⧀⧁⧡⧣⧤⧥⧴⧶⧷⧺⧻⨇⨈⨝⨟⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨰⨱⨲⨳⨴⨵⨶⨷⨸⨹⨺⨻⨼⨽⩀⩁⩂⩃⩄⩅⩊⩋⩌⩍⩎⩏⩐⩑⩒⩓⩔⩕⩖⩗⩘⩚⩛⩜⩝⩞⩟⩠⩡⩢⩣⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫛⫷⫸⫹⫺⬰⬱⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￪￫￬]))[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*", OPERATOR),
        Rule::token(r"(?m)(\.\.(?:(?:\.)?))", OPERATOR),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,3}|\\u[a-fA-F0-9]{1,4}|\\U[a-fA-F0-9]{1,6}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(?<=[.\w)\]])(\'[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*)+", OPERATOR),
        Rule::bygroups_to(r#"(?m)(raw)(""")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"tqrawstring"])),
        Rule::bygroups_to(r#"(?m)(raw)(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"rawstring"])),
        Rule::bygroups_to(r#"(?m)(r)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_REGEX)], NewState::Push(vec![r"tqregex"])),
        Rule::bygroups_to(r#"(?m)(r)(")"#, vec![Some(STRING_AFFIX), Some(STRING_REGEX)], NewState::Push(vec![r"regex"])),
        Rule::bygroups_to(r#"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(""")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"tqstring"])),
        Rule::bygroups_to(r#"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::bygroups_to(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(```)", vec![Some(STRING_AFFIX), Some(STRING_BACKTICK)], NewState::Push(vec![r"tqcommand"])),
        Rule::bygroups_to(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(`)", vec![Some(STRING_AFFIX), Some(STRING_BACKTICK)], NewState::Push(vec![r"command"])),
        Rule::bygroups_to(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\{)", vec![Some(KEYWORD_TYPE), Some(PUNCTUATION)], NewState::Push(vec![r"curly"])),
        Rule::bygroups(r"(?m)(where)(\s+)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::token_to(r"(?m)(\{)", PUNCTUATION, NewState::Push(vec![r"curly"])),
        Rule::bygroups(r"(?m)(abstract|primitive)([ \t]+)(type\b)([\s()]+)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(TEXT), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)(mutable(?=[ \t]))?([ \t]+)?(struct\b)([\s()]+)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(TEXT), Some(KEYWORD_TYPE)]),
        Rule::token(r"(?m)@(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", NAME_DECORATOR),
        Rule::token(r"(?m)@(!=(?:(?:=)?)|\$=|%=|\&(?:[&=])|\*=|\+(?:[+=])|\-(?:\->|[=>])|\.\.|/(?:/=|[/=])|:(?:[:=])|<(?:\-\-(?:(?:>)?)|<=|[:<=|])|=(?:==|[=>])|>(?:>(?:>=|[=>])|[:=>])|\\=|\^=|\|(?:[=>|])|÷=|≕'|⊻=|[!$%&*+\-./:<=>?\\\^|~¦¬±×÷…⁝⅋↑→↓↔↚↛↜↝↞↠↢↣↤↦↩↪↫↬↮↶↷↺↻↼↽⇀⇁⇄⇆⇇⇉⇋⇌⇍⇎⇏⇐⇒⇔⇚⇛⇜⇝⇠⇢⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿∈∉∊∋∌∍∓∔∗∘∙√∛∜∝∤∥∦∧∨∩∪∷∸∺∻∽∾≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊞⊟⊠⊡⊢⊣⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⊻⊼⊽⋄⋅⋆⋇⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌿▷⟂⟈⟉⟑⟒⟕⟖⟗⟰⟱⟵⟶⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤈⤉⤊⤋⤌⤍⤎⤏⤐⤑⤒⤓⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥉⥊⥋⥌⥍⥎⥏⥐⥑⥒⥓⥔⥕⥖⥗⥘⥙⥚⥛⥜⥝⥞⥟⥠⥡⥢⥣⥤⥥⥦⥧⥨⥩⥪⥫⥬⥭⥮⥯⥰⦷⦸⦼⦾⦿⧀⧁⧡⧣⧤⧥⧴⧶⧷⧺⧻⨇⨈⨝⨟⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨰⨱⨲⨳⨴⨵⨶⨷⨸⨹⨺⨻⨼⨽⩀⩁⩂⩃⩄⩅⩊⩋⩌⩍⩎⩏⩐⩑⩒⩓⩔⩕⩖⩗⩘⩚⩛⩜⩝⩞⩟⩠⩡⩢⩣⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫛⫷⫸⫹⫺⬰⬱⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￪￫￬])[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*", NAME_DECORATOR),
        Rule::token(r"(?m)(b(?:aremodule|egin|reak)|c(?:atch|call|on(?:st|tinue))|do|e(?:lse(?:(?:if)?)|nd|xport)|f(?:inally|or|unction)|global|i(?:mport|sa|[fn])|l(?:et|ocal)|m(?:acro|odule)|quote|return|try|using|wh(?:(?:er|il)e))\b", KEYWORD),
        Rule::token(r"(?m)(A(?:bstract(?:Array|Cha(?:nnel|r)|Di(?:ct|splay)|Float|Irrational|Mat(?:ch|rix)|Pattern|Range|S(?:et|tring)|UnitRange|Vec(?:OrMat|tor))|ny|r(?:gumentError|ray)|ssertionError)|B(?:i(?:g(?:(?:Floa|In)t)|t(?:Array|Matrix|Set|Vector))|o(?:ol|undsError))|C(?:a(?:pturedException|rtesianInd(?:ex|ices))|char|double|float|ha(?:nnel|r)|int(?:(?:max_t)?)|long(?:(?:long)?)|md|o(?:lon|mp(?:lex(?:(?:F(?:16|32|64))?)|os(?:(?:edFunc|iteExcep)tion))|ndition)|ptrdiff_t|s(?:hort|ize_t|size_t|tring)|u(?:char|int(?:(?:max_t)?)|long(?:(?:long)?)|short)|void|w(?:char_t|string))|D(?:ataType|ense(?:Array|Matrix|Vec(?:OrMat|tor))|i(?:ct|m(?:ensionMismatch|s)|videError)|omainError)|E(?:OFError|num|rrorException|x(?:ception|p(?:onentialBackOff|r)))|F(?:loat(?:16|32|64)|unction)|GlobalRef|HTML|I(?:O(?:(?:Buffer|Context|Stream)?)|dDict|n(?:dex(?:Cartesian|Linear|Style)|exactError|itError|t(?:(?:1(?:28|6)|32|64|8|e(?:ger|rruptException))?)|validStateException)|rrational)|KeyError|L(?:in(?:Range|e(?:NumberNode|arIndices))|oadError)|M(?:IME|atrix|ethod(?:(?:Error)?)|issing(?:(?:Exception)?)|odule)|N(?:Tuple|amedTuple|othing|umber)|O(?:rdinalRange|(?:utOfMemory|verflow)Error)|P(?:a(?:ir|rtialQuickSort)|ermutedDimsArray|ipe|rocessFailedException|tr)|QuoteNode|R(?:a(?:tional|wFD)|e(?:a(?:dOnlyMemoryError|l)|entrantLock|f|gex(?:(?:Match)?))|oundingMode)|S(?:e(?:(?:(?:gmentationFaul)?)t)|igned|ome|t(?:ackOverflowError|epRange(?:(?:Len)?)|ri(?:ded(?:Array|Matrix|Vec(?:OrMat|tor))|ng(?:(?:IndexError)?)))|ub(?:Array|(?:(?:stitution)?)String)|y(?:mbol|stemError))|T(?:ask(?:(?:FailedException)?)|ext(?:(?:Display)?)|imer|uple|ype(?:(?:(?:Erro|Va)r)?))|U(?:Int(?:(?:1(?:28|6)|32|64|8)?)|n(?:def(?:(?:Initialize|(?:Keyword|Ref|Var)Erro)r)|i(?:on(?:(?:All)?)|tRange)|signed))|V(?:a(?:l|rarg)|e(?:c(?:Element|OrMat|tor)|rsionNumber))|Weak(?:KeyDict|Ref))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(ARGS|C_NULL|DEPOT_PATH|EN(?:DIAN_BOM|V)|In(?:f(?:(?:16|32|64)?)|sertionSort)|LOAD_PATH|MergeSort|NaN(?:(?:16|32|64)?)|PROGRAM_FILE|QuickSort|Round(?:Down|FromZero|Nearest(?:(?:Ties(?:Away|Up))?)|ToZero|Up)|VERSION|devnull|false|im|missing|nothing|pi|std(?:err|in|out)|true|undef|[πℯ])\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", NAME),
        Rule::token(r"(?m)(\d+((_\d+)+)?\.(?!\.)(\d+((_\d+)+)?)?|\.\d+((_\d+)+)?)([eEf][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+((_\d+)+)?[eEf][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[a-fA-F0-9]+((_[a-fA-F0-9]+)+)?(\.([a-fA-F0-9]+((_[a-fA-F0-9]+)+)?)?)?p[+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)0b[01]+((_[01]+)+)?", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+((_[0-7]+)+)?", NUMBER_OCT),
        Rule::token(r"(?m)0x[a-fA-F0-9]+((_[a-fA-F0-9]+)+)?", NUMBER_HEX),
        Rule::token(r"(?m)\d+((_\d+)+)?", NUMBER_INTEGER),
        Rule::token(r"(?m)(\.)", OPERATOR),
    ]);
    m.insert(
        r"tqrawstring",
        vec![
            Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)([^"]|"[^"][^"])+"#, STRING),
        ],
    );
    m.insert(
        r"rawstring",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)\\""#, STRING_ESCAPE),
            Rule::token(r#"(?m)([^"\\]|\\[^"])+"#, STRING),
        ],
    );
    m.insert(
        r"interp",
        vec![
            Rule::token(r"(?m)\$(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", STRING_INTERPOL),
            Rule::bygroups_to(
                r"(?m)(\$)(\()",
                vec![Some(STRING_INTERPOL), Some(PUNCTUATION)],
                NewState::Push(vec![r"in-intp"]),
            ),
        ],
    );
    m.insert(r"in-intp", vec![
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token_to(r"(?m)#=", COMMENT_MULTILINE, NewState::Push(vec![r"blockcomment"])),
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m)[\[\](),;]", PUNCTUATION),
        Rule::bygroups(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\s*)(:)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(NAME), Some(WHITESPACE), Some(OPERATOR), Some(NAME)]),
        Rule::token(r"(?m)(?<![\]):<>\d.])(:(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", STRING_SYMBOL),
        Rule::bygroups(r"(?m)(?<=::)(\s*)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))\b(?![(\[])", vec![Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\s*)([<>]:)(\s*)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))\b(?![(\[])", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)([<>]:)(\s*)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))\b(?![(\[])", vec![Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)\b((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\s*)([<>]:)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)(!=(?:(?:=)?)|\$=|%=|\&(?:[&=])|\*=|\+(?:[+=])|\-(?:\->|[=>])|/(?:/=|[/=])|:(?:[:=])|<(?:\-\-(?:(?:>)?)|<=|[:<=|])|=(?:==|[=>])|>(?:>(?:>=|[=>])|[:=>])|\\=|\^=|\|(?:[=>|])|÷=|≕'|⊻=|[!$%&*+\-/:<=>?\\\^|~¦¬±×÷…⁝⅋↑→↓↔↚↛↜↝↞↠↢↣↤↦↩↪↫↬↮↶↷↺↻↼↽⇀⇁⇄⇆⇇⇉⇋⇌⇍⇎⇏⇐⇒⇔⇚⇛⇜⇝⇠⇢⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿∈∉∊∋∌∍∓∔∗∘∙√∛∜∝∤∥∦∧∨∩∪∷∸∺∻∽∾≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊞⊟⊠⊡⊢⊣⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⊻⊼⊽⋄⋅⋆⋇⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌿▷⟂⟈⟉⟑⟒⟕⟖⟗⟰⟱⟵⟶⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤈⤉⤊⤋⤌⤍⤎⤏⤐⤑⤒⤓⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥉⥊⥋⥌⥍⥎⥏⥐⥑⥒⥓⥔⥕⥖⥗⥘⥙⥚⥛⥜⥝⥞⥟⥠⥡⥢⥣⥤⥥⥦⥧⥨⥩⥪⥫⥬⥭⥮⥯⥰⦷⦸⦼⦾⦿⧀⧁⧡⧣⧤⧥⧴⧶⧷⧺⧻⨇⨈⨝⨟⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨰⨱⨲⨳⨴⨵⨶⨷⨸⨹⨺⨻⨼⨽⩀⩁⩂⩃⩄⩅⩊⩋⩌⩍⩎⩏⩐⩑⩒⩓⩔⩕⩖⩗⩘⩚⩛⩜⩝⩞⩟⩠⩡⩢⩣⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫛⫷⫸⫹⫺⬰⬱⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￪￫￬])[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*", OPERATOR),
        Rule::token(r"(?m)(\.(?:!=(?:(?:=)?)|%=|\&=|\*=|\+(?:[+=])|\-(?:\->|=)|/(?:/=|[/=])|<(?:\-\-(?:(?:>)?)|<=|[:<=|])|=(?:==|[=>])|>(?:>(?:>=|[=>])|[:=>])|\\=|\^=|\|(?:[=>])|÷=|≕'|⊻=|[!%&*+\-/<=>\\\^|~¦¬±×÷…⁝⅋↑→↓↔↚↛↜↝↞↠↢↣↤↦↩↪↫↬↮↶↷↺↻↼↽⇀⇁⇄⇆⇇⇉⇋⇌⇍⇎⇏⇐⇒⇔⇚⇛⇜⇝⇠⇢⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿∈∉∊∋∌∍∓∔∗∘∙√∛∜∝∤∥∦∧∨∩∪∷∸∺∻∽∾≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊞⊟⊠⊡⊢⊣⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⊻⊼⊽⋄⋅⋆⋇⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌿▷⟂⟈⟉⟑⟒⟕⟖⟗⟰⟱⟵⟶⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤈⤉⤊⤋⤌⤍⤎⤏⤐⤑⤒⤓⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥉⥊⥋⥌⥍⥎⥏⥐⥑⥒⥓⥔⥕⥖⥗⥘⥙⥚⥛⥜⥝⥞⥟⥠⥡⥢⥣⥤⥥⥦⥧⥨⥩⥪⥫⥬⥭⥮⥯⥰⦷⦸⦼⦾⦿⧀⧁⧡⧣⧤⧥⧴⧶⧷⧺⧻⨇⨈⨝⨟⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨰⨱⨲⨳⨴⨵⨶⨷⨸⨹⨺⨻⨼⨽⩀⩁⩂⩃⩄⩅⩊⩋⩌⩍⩎⩏⩐⩑⩒⩓⩔⩕⩖⩗⩘⩚⩛⩜⩝⩞⩟⩠⩡⩢⩣⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫛⫷⫸⫹⫺⬰⬱⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￪￫￬]))[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*", OPERATOR),
        Rule::token(r"(?m)(\.\.(?:(?:\.)?))", OPERATOR),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,3}|\\u[a-fA-F0-9]{1,4}|\\U[a-fA-F0-9]{1,6}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(?<=[.\w)\]])(\'[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*)+", OPERATOR),
        Rule::bygroups_to(r#"(?m)(raw)(""")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"tqrawstring"])),
        Rule::bygroups_to(r#"(?m)(raw)(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"rawstring"])),
        Rule::bygroups_to(r#"(?m)(r)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_REGEX)], NewState::Push(vec![r"tqregex"])),
        Rule::bygroups_to(r#"(?m)(r)(")"#, vec![Some(STRING_AFFIX), Some(STRING_REGEX)], NewState::Push(vec![r"regex"])),
        Rule::bygroups_to(r#"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(""")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"tqstring"])),
        Rule::bygroups_to(r#"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::bygroups_to(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(```)", vec![Some(STRING_AFFIX), Some(STRING_BACKTICK)], NewState::Push(vec![r"tqcommand"])),
        Rule::bygroups_to(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))?(`)", vec![Some(STRING_AFFIX), Some(STRING_BACKTICK)], NewState::Push(vec![r"command"])),
        Rule::bygroups_to(r"(?m)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))(\{)", vec![Some(KEYWORD_TYPE), Some(PUNCTUATION)], NewState::Push(vec![r"curly"])),
        Rule::bygroups(r"(?m)(where)(\s+)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::token_to(r"(?m)(\{)", PUNCTUATION, NewState::Push(vec![r"curly"])),
        Rule::bygroups(r"(?m)(abstract|primitive)([ \t]+)(type\b)([\s()]+)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(TEXT), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?m)(mutable(?=[ \t]))?([ \t]+)?(struct\b)([\s()]+)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*))", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(TEXT), Some(KEYWORD_TYPE)]),
        Rule::token(r"(?m)@(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", NAME_DECORATOR),
        Rule::token(r"(?m)@(!=(?:(?:=)?)|\$=|%=|\&(?:[&=])|\*=|\+(?:[+=])|\-(?:\->|[=>])|\.\.|/(?:/=|[/=])|:(?:[:=])|<(?:\-\-(?:(?:>)?)|<=|[:<=|])|=(?:==|[=>])|>(?:>(?:>=|[=>])|[:=>])|\\=|\^=|\|(?:[=>|])|÷=|≕'|⊻=|[!$%&*+\-./:<=>?\\\^|~¦¬±×÷…⁝⅋↑→↓↔↚↛↜↝↞↠↢↣↤↦↩↪↫↬↮↶↷↺↻↼↽⇀⇁⇄⇆⇇⇉⇋⇌⇍⇎⇏⇐⇒⇔⇚⇛⇜⇝⇠⇢⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿∈∉∊∋∌∍∓∔∗∘∙√∛∜∝∤∥∦∧∨∩∪∷∸∺∻∽∾≀≁≂≃≄≅≆≇≈≉≊≋≌≍≎≏≐≑≒≓≔≖≗≘≙≚≛≜≝≞≟≠≡≢≣≤≥≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊞⊟⊠⊡⊢⊣⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⊻⊼⊽⋄⋅⋆⋇⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌿▷⟂⟈⟉⟑⟒⟕⟖⟗⟰⟱⟵⟶⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤈⤉⤊⤋⤌⤍⤎⤏⤐⤑⤒⤓⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥉⥊⥋⥌⥍⥎⥏⥐⥑⥒⥓⥔⥕⥖⥗⥘⥙⥚⥛⥜⥝⥞⥟⥠⥡⥢⥣⥤⥥⥦⥧⥨⥩⥪⥫⥬⥭⥮⥯⥰⦷⦸⦼⦾⦿⧀⧁⧡⧣⧤⧥⧴⧶⧷⧺⧻⨇⨈⨝⨟⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨰⨱⨲⨳⨴⨵⨶⨷⨸⨹⨺⨻⨼⨽⩀⩁⩂⩃⩄⩅⩊⩋⩌⩍⩎⩏⩐⩑⩒⩓⩔⩕⩖⩗⩘⩚⩛⩜⩝⩞⩟⩠⩡⩢⩣⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫛⫷⫸⫹⫺⬰⬱⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￪￫￬])[²³¹ʰʲʳʷʸˡˢˣᴬᴮᴰᴱᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿᵀᵁᵂᵃᵇᵈᵉᵍᵏᵐᵒᵖᵗᵘᵛᵝᵞᵟᵠᵡᵢᵣᵤᵥᵦᵧᵨᵩᵪᶜᶠᶥᶦᶫᶰᶸᶻᶿ′″‴‵‶‷⁗⁰ⁱ⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿ₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎ₐₑₒₓₕₖₗₘₙₚₛₜⱼⱽ]*", NAME_DECORATOR),
        Rule::token(r"(?m)(b(?:aremodule|egin|reak)|c(?:atch|call|on(?:st|tinue))|do|e(?:lse(?:(?:if)?)|nd|xport)|f(?:inally|or|unction)|global|i(?:mport|sa|[fn])|l(?:et|ocal)|m(?:acro|odule)|quote|return|try|using|wh(?:(?:er|il)e))\b", KEYWORD),
        Rule::token(r"(?m)(A(?:bstract(?:Array|Cha(?:nnel|r)|Di(?:ct|splay)|Float|Irrational|Mat(?:ch|rix)|Pattern|Range|S(?:et|tring)|UnitRange|Vec(?:OrMat|tor))|ny|r(?:gumentError|ray)|ssertionError)|B(?:i(?:g(?:(?:Floa|In)t)|t(?:Array|Matrix|Set|Vector))|o(?:ol|undsError))|C(?:a(?:pturedException|rtesianInd(?:ex|ices))|char|double|float|ha(?:nnel|r)|int(?:(?:max_t)?)|long(?:(?:long)?)|md|o(?:lon|mp(?:lex(?:(?:F(?:16|32|64))?)|os(?:(?:edFunc|iteExcep)tion))|ndition)|ptrdiff_t|s(?:hort|ize_t|size_t|tring)|u(?:char|int(?:(?:max_t)?)|long(?:(?:long)?)|short)|void|w(?:char_t|string))|D(?:ataType|ense(?:Array|Matrix|Vec(?:OrMat|tor))|i(?:ct|m(?:ensionMismatch|s)|videError)|omainError)|E(?:OFError|num|rrorException|x(?:ception|p(?:onentialBackOff|r)))|F(?:loat(?:16|32|64)|unction)|GlobalRef|HTML|I(?:O(?:(?:Buffer|Context|Stream)?)|dDict|n(?:dex(?:Cartesian|Linear|Style)|exactError|itError|t(?:(?:1(?:28|6)|32|64|8|e(?:ger|rruptException))?)|validStateException)|rrational)|KeyError|L(?:in(?:Range|e(?:NumberNode|arIndices))|oadError)|M(?:IME|atrix|ethod(?:(?:Error)?)|issing(?:(?:Exception)?)|odule)|N(?:Tuple|amedTuple|othing|umber)|O(?:rdinalRange|(?:utOfMemory|verflow)Error)|P(?:a(?:ir|rtialQuickSort)|ermutedDimsArray|ipe|rocessFailedException|tr)|QuoteNode|R(?:a(?:tional|wFD)|e(?:a(?:dOnlyMemoryError|l)|entrantLock|f|gex(?:(?:Match)?))|oundingMode)|S(?:e(?:(?:(?:gmentationFaul)?)t)|igned|ome|t(?:ackOverflowError|epRange(?:(?:Len)?)|ri(?:ded(?:Array|Matrix|Vec(?:OrMat|tor))|ng(?:(?:IndexError)?)))|ub(?:Array|(?:(?:stitution)?)String)|y(?:mbol|stemError))|T(?:ask(?:(?:FailedException)?)|ext(?:(?:Display)?)|imer|uple|ype(?:(?:(?:Erro|Va)r)?))|U(?:Int(?:(?:1(?:28|6)|32|64|8)?)|n(?:def(?:(?:Initialize|(?:Keyword|Ref|Var)Erro)r)|i(?:on(?:(?:All)?)|tRange)|signed))|V(?:a(?:l|rarg)|e(?:c(?:Element|OrMat|tor)|rsionNumber))|Weak(?:KeyDict|Ref))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(ARGS|C_NULL|DEPOT_PATH|EN(?:DIAN_BOM|V)|In(?:f(?:(?:16|32|64)?)|sertionSort)|LOAD_PATH|MergeSort|NaN(?:(?:16|32|64)?)|PROGRAM_FILE|QuickSort|Round(?:Down|FromZero|Nearest(?:(?:Ties(?:Away|Up))?)|ToZero|Up)|VERSION|devnull|false|im|missing|nothing|pi|std(?:err|in|out)|true|undef|[πℯ])\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", NAME),
        Rule::token(r"(?m)(\d+((_\d+)+)?\.(?!\.)(\d+((_\d+)+)?)?|\.\d+((_\d+)+)?)([eEf][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+((_\d+)+)?[eEf][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[a-fA-F0-9]+((_[a-fA-F0-9]+)+)?(\.([a-fA-F0-9]+((_[a-fA-F0-9]+)+)?)?)?p[+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)0b[01]+((_[01]+)+)?", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+((_[0-7]+)+)?", NUMBER_OCT),
        Rule::token(r"(?m)0x[a-fA-F0-9]+((_[a-fA-F0-9]+)+)?", NUMBER_HEX),
        Rule::token(r"(?m)\d+((_\d+)+)?", NUMBER_INTEGER),
        Rule::token(r"(?m)(\.)", OPERATOR),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::bygroups_to(
                r#"(?m)(")((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)|\d+)?"#,
                vec![Some(STRING), Some(STRING_AFFIX)],
                NewState::Pop(1),
            ),
            Rule::token(
                r#"(?m)\\([\\"\'$nrbtfav]|(x|u|U)[a-fA-F0-9]+|\d+)"#,
                STRING_ESCAPE,
            ),
            Rule::token(r"(?m)\$(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", STRING_INTERPOL),
            Rule::bygroups_to(
                r"(?m)(\$)(\()",
                vec![Some(STRING_INTERPOL), Some(PUNCTUATION)],
                NewState::Push(vec![r"in-intp"]),
            ),
            Rule::token(
                r"(?m)%[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]",
                STRING_INTERPOL,
            ),
            Rule::token(r#"(?m)[^"$%\\]+"#, STRING),
            Rule::token(r"(?m).", STRING),
        ],
    );
    m.insert(
        r"tqstring",
        vec![
            Rule::bygroups_to(
                r#"(?m)(""")((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)|\d+)?"#,
                vec![Some(STRING), Some(STRING_AFFIX)],
                NewState::Pop(1),
            ),
            Rule::token(
                r#"(?m)\\([\\"\'$nrbtfav]|(x|u|U)[a-fA-F0-9]+|\d+)"#,
                STRING_ESCAPE,
            ),
            Rule::token(r"(?m)\$(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", STRING_INTERPOL),
            Rule::bygroups_to(
                r"(?m)(\$)(\()",
                vec![Some(STRING_INTERPOL), Some(PUNCTUATION)],
                NewState::Push(vec![r"in-intp"]),
            ),
            Rule::token(r#"(?m)[^"$%\\]+"#, STRING),
            Rule::token(r"(?m).", STRING),
        ],
    );
    m.insert(
        r"regex",
        vec![
            Rule::bygroups_to(
                r#"(?m)(")([imsxa]*)?"#,
                vec![Some(STRING_REGEX), Some(STRING_AFFIX)],
                NewState::Pop(1),
            ),
            Rule::token(r#"(?m)\\""#, STRING_REGEX),
            Rule::token(r#"(?m)[^\\"]+"#, STRING_REGEX),
        ],
    );
    m.insert(
        r"tqregex",
        vec![
            Rule::bygroups_to(
                r#"(?m)(""")([imsxa]*)?"#,
                vec![Some(STRING_REGEX), Some(STRING_AFFIX)],
                NewState::Pop(1),
            ),
            Rule::token(r#"(?m)[^"]+"#, STRING_REGEX),
        ],
    );
    m.insert(
        r"command",
        vec![
            Rule::bygroups_to(
                r"(?m)(`)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)|\d+)?",
                vec![Some(STRING_BACKTICK), Some(STRING_AFFIX)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)\\[`$]", STRING_ESCAPE),
            Rule::token(r"(?m)\$(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", STRING_INTERPOL),
            Rule::bygroups_to(
                r"(?m)(\$)(\()",
                vec![Some(STRING_INTERPOL), Some(PUNCTUATION)],
                NewState::Push(vec![r"in-intp"]),
            ),
            Rule::token(r"(?m)[^\\`$]+", STRING_BACKTICK),
            Rule::token(r"(?m).", STRING_BACKTICK),
        ],
    );
    m.insert(
        r"tqcommand",
        vec![
            Rule::bygroups_to(
                r"(?m)(```)((?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)|\d+)?",
                vec![Some(STRING_BACKTICK), Some(STRING_AFFIX)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)\\\$", STRING_ESCAPE),
            Rule::token(r"(?m)\$(?:[a-zA-Z_¡-􏿿][a-zA-Z_0-9!¡-􏿿]*)", STRING_INTERPOL),
            Rule::bygroups_to(
                r"(?m)(\$)(\()",
                vec![Some(STRING_INTERPOL), Some(PUNCTUATION)],
                NewState::Push(vec![r"in-intp"]),
            ),
            Rule::token(r"(?m)[^\\`$]+", STRING_BACKTICK),
            Rule::token(r"(?m).", STRING_BACKTICK),
        ],
    );
    Table(m)
}

impl Lexer for JuliaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
