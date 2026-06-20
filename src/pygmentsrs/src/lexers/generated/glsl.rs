#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.graphics:GLShaderLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.graphics:GLShaderLexer:glsl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: glsl
pub struct GlslLexer;

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
        Rule::token(r"(?m)#(?:.*\\\n)*.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)\+|-|~|!=?|\*|/|%|<<|>>|<=?|>=?|==?|&&?|\^|\|\|?", OPERATOR),
        Rule::token(r"(?m)[?:]", OPERATOR),
        Rule::token(r"(?m)\bdefined\b", OPERATOR),
        Rule::token(r"(?m)[;{}(),\[\]]", PUNCTUATION),
        Rule::token(r"(?m)[+-]?\d*\.\d+([eE][-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d+\.\d*([eE][-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F]*", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]*", NUMBER_OCT),
        Rule::token(r"(?m)[1-9][0-9]*", NUMBER_INTEGER),
        Rule::token(r"(?m)\b(attribute|b(?:reak|uffer)|c(?:ase|entroid|o(?:herent|n(?:st|tinue)))|d(?:efault|iscard|o)|else|f(?:lat|or)|highp|i(?:n(?:(?:ou|varian)t)|[fn])|l(?:ayout|owp)|mediump|noperspective|out|p(?:atch|recis(?:e|ion))|re(?:adonly|strict|turn)|s(?:ample|hared|mooth|truct|ubroutine|witch)|uniform|v(?:arying|olatile)|w(?:hile|riteonly))\b", KEYWORD),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\b(atomic_uint|b(?:ool|vec(?:[234]))|d(?:mat(?:2x(?:[234])|3x(?:[234])|4x(?:[234])|[234])|ouble|vec(?:[234]))|float|i(?:image(?:1D(?:(?:Array)?)|2D(?:(?:Array|MS(?:(?:Array)?)|Rect)?)|3D|Buffer|Cube(?:(?:Array)?))|mage(?:1D(?:(?:Array)?)|2D(?:(?:Array|MS(?:(?:Array)?)|Rect)?)|3D|Buffer|Cube(?:(?:Array)?))|nt|sampler(?:1D(?:(?:Array)?)|2D(?:(?:Array|MS(?:(?:Array)?)|Rect)?)|3D|Buffer|Cube(?:(?:Array)?))|vec(?:[234]))|mat(?:2x(?:[234])|3x(?:[234])|4x(?:[234])|[234])|sampler(?:1D(?:(?:Array(?:(?:Shadow)?)|Shadow)?)|2D(?:(?:Array(?:(?:Shadow)?)|MS(?:(?:Array)?)|Rect(?:(?:Shadow)?)|Shadow)?)|3D|Buffer|Cube(?:(?:Array(?:(?:Shadow)?)|Shadow)?))|u(?:i(?:mage(?:1D(?:(?:Array)?)|2D(?:(?:Array|MS(?:(?:Array)?)|Rect)?)|3D|Buffer|Cube(?:(?:Array)?))|nt)|sampler(?:1D(?:(?:Array)?)|2D(?:(?:Array|MS(?:(?:Array)?)|Rect)?)|3D|Buffer|Cube(?:(?:Array)?))|vec(?:[234]))|v(?:ec(?:[234])|oid))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(a(?:ctive|sm)|c(?:ast|lass|ommon)|e(?:num|xtern(?:(?:al)?))|f(?:i(?:lter|xed)|vec(?:[234]))|goto|h(?:alf|vec(?:[234]))|in(?:line|put|terface)|long|n(?:(?:amespac|oinlin)e)|output|p(?:artition|ublic)|resource|s(?:ampler3DRect|hort|izeof|tatic|uperp)|t(?:emplate|his|ypedef)|u(?:n(?:ion|signed)|sing))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)gl_\w*", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)\.", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for GlslLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
