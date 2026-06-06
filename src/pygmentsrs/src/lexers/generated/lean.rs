//! AUTO-GENERATED from `pygments.pygments.lexers.lean:Lean3Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lean:Lean3Lexer:lean

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: lean, lean3
pub struct LeanLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"expression", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)/--", STRING_DOC, NewState::Push(vec![r"docstring"])),
        Rule::token_to(r"(?m)/-", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)--.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)\b(Pi|assume|calc|do|else|f(?:orall|rom|un)|have|i(?:[fn])|let|match|s(?:how|uffices)|then|with)\b", KEYWORD),
        Rule::token(r"(?m)\b(admit|sorry)\b", GENERIC_ERROR),
        Rule::token(r"(?m)\b(Prop|Sort|Type)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(:=|[(),:\[\]{}‹›⟨⟩⦃⦄])", OPERATOR),
        Rule::token(r"(?m)(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*(\.(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*)*", NAME),
        Rule::token(r"(?m)``?(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*(\.(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*)*", STRING_SYMBOL),
        Rule::token(r"(?m)0x[A-Za-z0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)0b[01]+", NUMBER_INTEGER),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r#"(?m)'(?:(\\[\\\"'nt])|(\\x[0-9a-fA-F]{2})|(\\u[0-9a-fA-F]{4})|.)'"#, STRING_CHAR),
        Rule::token(r"(?m)[~?][a-z][\w\']*:", NAME_VARIABLE),
        Rule::token(r"(?m)\S", NAME_BUILTIN_PSEUDO),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)\b(attribute|export|hiding|i(?:mport|nclude)|local|namespace|o(?:mit|pen)|pr(?:ivate|otected(?:(?:)?))|renaming|section(?:(?:)?))\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)\b(a(?:bbreviation|ttribute|xiom(?:(?:s)?))|b(?:egin|y)|c(?:lass|o(?:inductive|nstant(?:(?:s)?)))|def(?:(?:inition)?)|e(?:nd|x(?:ample|tends))|in(?:ductive|fix(?:(?:[lr])?)|stance)|lemma|m(?:eta|utual)|no(?:ncomputable(?:(?:\ theory)?)|tation)|p(?:arameter(?:(?:s)?)|ostfix|re(?:cedence|fix))|r(?:eserve|un_cmd)|s(?:et_option|tructure)|theorem|universe(?:(?:s)?)|variable(?:(?:s)?))\b", KEYWORD_DECLARATION),
        Rule::token_to(r"(?m)@\[", KEYWORD_DECLARATION, NewState::Push(vec![r"attribute"])),
        Rule::token(r"(?m)(\#(?:check|e(?:val|xit)|help|print|reduce))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)/--", STRING_DOC, NewState::Push(vec![r"docstring"])),
        Rule::token_to(r"(?m)/-", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)--.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)\b(Pi|assume|calc|do|else|f(?:orall|rom|un)|have|i(?:[fn])|let|match|s(?:how|uffices)|then|with)\b", KEYWORD),
        Rule::token(r"(?m)\b(admit|sorry)\b", GENERIC_ERROR),
        Rule::token(r"(?m)\b(Prop|Sort|Type)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(:=|[(),:\[\]{}‹›⟨⟩⦃⦄])", OPERATOR),
        Rule::token(r"(?m)(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*(\.(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*)*", NAME),
        Rule::token(r"(?m)``?(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*(\.(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*)*", STRING_SYMBOL),
        Rule::token(r"(?m)0x[A-Za-z0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)0b[01]+", NUMBER_INTEGER),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r#"(?m)'(?:(\\[\\\"'nt])|(\\x[0-9a-fA-F]{2})|(\\u[0-9a-fA-F]{4})|.)'"#, STRING_CHAR),
        Rule::token(r"(?m)[~?][a-z][\w\']*:", NAME_VARIABLE),
        Rule::token(r"(?m)\S", NAME_BUILTIN_PSEUDO),
    ]);
    m.insert(r"attribute", vec![
        Rule::token_to(r"(?m)\]", KEYWORD_DECLARATION, NewState::Pop(1)),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)/--", STRING_DOC, NewState::Push(vec![r"docstring"])),
        Rule::token_to(r"(?m)/-", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)--.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)\b(Pi|assume|calc|do|else|f(?:orall|rom|un)|have|i(?:[fn])|let|match|s(?:how|uffices)|then|with)\b", KEYWORD),
        Rule::token(r"(?m)\b(admit|sorry)\b", GENERIC_ERROR),
        Rule::token(r"(?m)\b(Prop|Sort|Type)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(:=|[(),:\[\]{}‹›⟨⟩⦃⦄])", OPERATOR),
        Rule::token(r"(?m)(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*(\.(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*)*", NAME),
        Rule::token(r"(?m)``?(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*(\.(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟](?:(?![λΠΣ])[_a-zA-Zα-ωΑ-Ωϊ-ϻἀ-῾℀-⅏𝒜-𝖟0-9'ⁿ-₉ₐ-ₜᵢ-ᵪ])*)*", STRING_SYMBOL),
        Rule::token(r"(?m)0x[A-Za-z0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)0b[01]+", NUMBER_INTEGER),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r#"(?m)'(?:(\\[\\\"'nt])|(\\x[0-9a-fA-F]{2})|(\\u[0-9a-fA-F]{4})|.)'"#, STRING_CHAR),
        Rule::token(r"(?m)[~?][a-z][\w\']*:", NAME_VARIABLE),
        Rule::token(r"(?m)\S", NAME_BUILTIN_PSEUDO),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)[^/-]+", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/-", COMMENT_MULTILINE, NewState::PushSame),
        Rule::token_to(r"(?m)-/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[/-]", COMMENT_MULTILINE),
    ]);
    m.insert(r"docstring", vec![
        Rule::token(r"(?m)[^/-]+", STRING_DOC),
        Rule::token_to(r"(?m)-/", STRING_DOC, NewState::Pop(1)),
        Rule::token(r"(?m)[/-]", STRING_DOC),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)[^\\"]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)(?:(\\[\\\"'nt])|(\\x[0-9a-fA-F]{2})|(\\u[0-9a-fA-F]{4}))"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for LeanLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
