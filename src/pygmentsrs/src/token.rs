//! Token-type hierarchy — port of `pygments/token.py`.
//!
//! Pygments tokens are tuple-like singletons (`Token.Literal.String`)
//! with `repr() == "Token.Literal.String"` and a `split()` returning
//! the chain of ancestors. Subtype tests use `in`:
//! `String in Literal` is true because `String[:len(Literal)] == Literal`.
//!
//! Here a [`TokenType`] is a `'static` slice of name segments
//! (`&["Literal", "String"]`). `repr()` joins with `"Token"` +
//! `'.'.join(segments)`. `contains(other)` is the prefix check.

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenType {
    pub path: &'static [&'static str],
}

impl TokenType {
    pub const fn new(path: &'static [&'static str]) -> Self {
        Self { path }
    }

    /// `repr(ttype)` — matches `pygments.token._TokenType.__repr__`.
    /// `Token` itself reprs as `"Token"`; any subtype reprs as
    /// `"Token.X.Y..."`.
    pub fn repr(&self) -> String {
        if self.path.is_empty() {
            "Token".to_string()
        } else {
            let mut s = String::from("Token");
            for seg in self.path {
                s.push('.');
                s.push_str(seg);
            }
            s
        }
    }

    /// Mirrors `other in self` semantics: `other` is a subtype of
    /// (or equal to) `self`.
    pub fn contains(&self, other: TokenType) -> bool {
        if other.path.len() < self.path.len() {
            return false;
        }
        other.path[..self.path.len()] == *self.path
    }

    /// Walk from the root `Token` down to `self`. Mirrors
    /// `_TokenType.split()`.
    pub fn split(&self) -> Vec<TokenType> {
        let mut out = Vec::with_capacity(self.path.len() + 1);
        for i in 0..=self.path.len() {
            out.push(TokenType {
                path: &self.path[..i],
            });
        }
        out
    }

    /// Short name used by the HTML formatter's `STANDARD_TYPES`
    /// table. Mirrors `pygments.token.STANDARD_TYPES` exactly. For
    /// subtypes not present in the table, returns the short-name of
    /// the nearest ancestor that IS present (matching
    /// `HtmlFormatter._get_css_classes`'s walk-to-parent loop, but
    /// flattened to a single short-name since we currently emit one
    /// span per token rather than the space-joined CSS-class chain).
    pub fn short_name(&self) -> String {
        let mut path: &[&'static str] = self.path;
        loop {
            if let Some(s) = standard_short(path) {
                return s.to_string();
            }
            if path.is_empty() {
                return String::new();
            }
            path = &path[..path.len() - 1];
        }
    }
}

/// Direct port of `pygments.token.STANDARD_TYPES`. `None` means the
/// dotted type is not in the table — caller walks to parent.
fn standard_short(path: &[&str]) -> Option<&'static str> {
    Some(match path {
        [] => "",
        ["Text"] => "",
        ["Text", "Whitespace"] => "w",
        ["Escape"] => "esc",
        ["Error"] => "err",
        ["Other"] => "x",

        ["Keyword"] => "k",
        ["Keyword", "Constant"] => "kc",
        ["Keyword", "Declaration"] => "kd",
        ["Keyword", "Namespace"] => "kn",
        ["Keyword", "Pseudo"] => "kp",
        ["Keyword", "Reserved"] => "kr",
        ["Keyword", "Type"] => "kt",

        ["Name"] => "n",
        ["Name", "Attribute"] => "na",
        ["Name", "Builtin"] => "nb",
        ["Name", "Builtin", "Pseudo"] => "bp",
        ["Name", "Class"] => "nc",
        ["Name", "Constant"] => "no",
        ["Name", "Decorator"] => "nd",
        ["Name", "Entity"] => "ni",
        ["Name", "Exception"] => "ne",
        ["Name", "Function"] => "nf",
        ["Name", "Function", "Magic"] => "fm",
        ["Name", "Property"] => "py",
        ["Name", "Label"] => "nl",
        ["Name", "Namespace"] => "nn",
        ["Name", "Other"] => "nx",
        ["Name", "Tag"] => "nt",
        ["Name", "Variable"] => "nv",
        ["Name", "Variable", "Class"] => "vc",
        ["Name", "Variable", "Global"] => "vg",
        ["Name", "Variable", "Instance"] => "vi",
        ["Name", "Variable", "Magic"] => "vm",

        ["Literal"] => "l",
        ["Literal", "Date"] => "ld",

        ["Literal", "String"] => "s",
        ["Literal", "String", "Affix"] => "sa",
        ["Literal", "String", "Backtick"] => "sb",
        ["Literal", "String", "Char"] => "sc",
        ["Literal", "String", "Delimiter"] => "dl",
        ["Literal", "String", "Doc"] => "sd",
        ["Literal", "String", "Double"] => "s2",
        ["Literal", "String", "Escape"] => "se",
        ["Literal", "String", "Heredoc"] => "sh",
        ["Literal", "String", "Interpol"] => "si",
        ["Literal", "String", "Other"] => "sx",
        ["Literal", "String", "Regex"] => "sr",
        ["Literal", "String", "Single"] => "s1",
        ["Literal", "String", "Symbol"] => "ss",

        ["Literal", "Number"] => "m",
        ["Literal", "Number", "Bin"] => "mb",
        ["Literal", "Number", "Float"] => "mf",
        ["Literal", "Number", "Hex"] => "mh",
        ["Literal", "Number", "Integer"] => "mi",
        ["Literal", "Number", "Integer", "Long"] => "il",
        ["Literal", "Number", "Oct"] => "mo",

        ["Operator"] => "o",
        ["Operator", "Word"] => "ow",

        ["Punctuation"] => "p",
        ["Punctuation", "Marker"] => "pm",

        ["Comment"] => "c",
        ["Comment", "Hashbang"] => "ch",
        ["Comment", "Multiline"] => "cm",
        ["Comment", "Preproc"] => "cp",
        ["Comment", "PreprocFile"] => "cpf",
        ["Comment", "Single"] => "c1",
        ["Comment", "Special"] => "cs",

        ["Generic"] => "g",
        ["Generic", "Deleted"] => "gd",
        ["Generic", "Emph"] => "ge",
        ["Generic", "Error"] => "gr",
        ["Generic", "Heading"] => "gh",
        ["Generic", "Inserted"] => "gi",
        ["Generic", "Output"] => "go",
        ["Generic", "Prompt"] => "gp",
        ["Generic", "Strong"] => "gs",
        ["Generic", "Subheading"] => "gu",
        ["Generic", "EmphStrong"] => "ges",
        ["Generic", "Traceback"] => "gt",

        _ => return None,
    })
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.repr())
    }
}

// ----- Standard token tree (mirrors `pygments.token` module-level
// constants). Add nodes here as lexers need them.

pub const TOKEN: TokenType = TokenType::new(&[]);

pub const TEXT: TokenType = TokenType::new(&["Text"]);
pub const WHITESPACE: TokenType = TokenType::new(&["Text", "Whitespace"]);
pub const ESCAPE: TokenType = TokenType::new(&["Escape"]);
pub const ERROR: TokenType = TokenType::new(&["Error"]);
pub const OTHER: TokenType = TokenType::new(&["Other"]);

pub const KEYWORD: TokenType = TokenType::new(&["Keyword"]);
pub const KEYWORD_CONSTANT: TokenType = TokenType::new(&["Keyword", "Constant"]);
pub const KEYWORD_DECLARATION: TokenType = TokenType::new(&["Keyword", "Declaration"]);
pub const KEYWORD_NAMESPACE: TokenType = TokenType::new(&["Keyword", "Namespace"]);
pub const KEYWORD_PSEUDO: TokenType = TokenType::new(&["Keyword", "Pseudo"]);
pub const KEYWORD_RESERVED: TokenType = TokenType::new(&["Keyword", "Reserved"]);
pub const KEYWORD_TYPE: TokenType = TokenType::new(&["Keyword", "Type"]);

pub const NAME: TokenType = TokenType::new(&["Name"]);
pub const NAME_ATTRIBUTE: TokenType = TokenType::new(&["Name", "Attribute"]);
pub const NAME_BUILTIN: TokenType = TokenType::new(&["Name", "Builtin"]);
pub const NAME_BUILTIN_PSEUDO: TokenType = TokenType::new(&["Name", "Builtin", "Pseudo"]);
pub const NAME_CLASS: TokenType = TokenType::new(&["Name", "Class"]);
pub const NAME_CONSTANT: TokenType = TokenType::new(&["Name", "Constant"]);
pub const NAME_DECORATOR: TokenType = TokenType::new(&["Name", "Decorator"]);
pub const NAME_ENTITY: TokenType = TokenType::new(&["Name", "Entity"]);
pub const NAME_EXCEPTION: TokenType = TokenType::new(&["Name", "Exception"]);
pub const NAME_FUNCTION: TokenType = TokenType::new(&["Name", "Function"]);
pub const NAME_FUNCTION_MAGIC: TokenType = TokenType::new(&["Name", "Function", "Magic"]);
pub const NAME_PROPERTY: TokenType = TokenType::new(&["Name", "Property"]);
pub const NAME_LABEL: TokenType = TokenType::new(&["Name", "Label"]);
pub const NAME_NAMESPACE: TokenType = TokenType::new(&["Name", "Namespace"]);
pub const NAME_OTHER: TokenType = TokenType::new(&["Name", "Other"]);
pub const NAME_TAG: TokenType = TokenType::new(&["Name", "Tag"]);
pub const NAME_VARIABLE: TokenType = TokenType::new(&["Name", "Variable"]);
pub const NAME_VARIABLE_CLASS: TokenType = TokenType::new(&["Name", "Variable", "Class"]);
pub const NAME_VARIABLE_GLOBAL: TokenType = TokenType::new(&["Name", "Variable", "Global"]);
pub const NAME_VARIABLE_INSTANCE: TokenType = TokenType::new(&["Name", "Variable", "Instance"]);
pub const NAME_VARIABLE_MAGIC: TokenType = TokenType::new(&["Name", "Variable", "Magic"]);

pub const LITERAL: TokenType = TokenType::new(&["Literal"]);
pub const LITERAL_DATE: TokenType = TokenType::new(&["Literal", "Date"]);

pub const STRING: TokenType = TokenType::new(&["Literal", "String"]);
pub const STRING_AFFIX: TokenType = TokenType::new(&["Literal", "String", "Affix"]);
pub const STRING_BACKTICK: TokenType = TokenType::new(&["Literal", "String", "Backtick"]);
pub const STRING_CHAR: TokenType = TokenType::new(&["Literal", "String", "Char"]);
pub const STRING_DELIMITER: TokenType = TokenType::new(&["Literal", "String", "Delimiter"]);
pub const STRING_DOC: TokenType = TokenType::new(&["Literal", "String", "Doc"]);
pub const STRING_DOUBLE: TokenType = TokenType::new(&["Literal", "String", "Double"]);
pub const STRING_ESCAPE: TokenType = TokenType::new(&["Literal", "String", "Escape"]);
pub const STRING_HEREDOC: TokenType = TokenType::new(&["Literal", "String", "Heredoc"]);
pub const STRING_INTERPOL: TokenType = TokenType::new(&["Literal", "String", "Interpol"]);
pub const STRING_OTHER: TokenType = TokenType::new(&["Literal", "String", "Other"]);
pub const STRING_REGEX: TokenType = TokenType::new(&["Literal", "String", "Regex"]);
pub const STRING_SINGLE: TokenType = TokenType::new(&["Literal", "String", "Single"]);
pub const STRING_SYMBOL: TokenType = TokenType::new(&["Literal", "String", "Symbol"]);

pub const NUMBER: TokenType = TokenType::new(&["Literal", "Number"]);
pub const NUMBER_BIN: TokenType = TokenType::new(&["Literal", "Number", "Bin"]);
pub const NUMBER_FLOAT: TokenType = TokenType::new(&["Literal", "Number", "Float"]);
pub const NUMBER_HEX: TokenType = TokenType::new(&["Literal", "Number", "Hex"]);
pub const NUMBER_INTEGER: TokenType = TokenType::new(&["Literal", "Number", "Integer"]);
pub const NUMBER_INTEGER_LONG: TokenType =
    TokenType::new(&["Literal", "Number", "Integer", "Long"]);
pub const NUMBER_OCT: TokenType = TokenType::new(&["Literal", "Number", "Oct"]);

pub const PUNCTUATION: TokenType = TokenType::new(&["Punctuation"]);
pub const PUNCTUATION_MARKER: TokenType = TokenType::new(&["Punctuation", "Marker"]);

pub const OPERATOR: TokenType = TokenType::new(&["Operator"]);
pub const OPERATOR_WORD: TokenType = TokenType::new(&["Operator", "Word"]);

pub const COMMENT: TokenType = TokenType::new(&["Comment"]);
pub const COMMENT_HASHBANG: TokenType = TokenType::new(&["Comment", "Hashbang"]);
pub const COMMENT_MULTILINE: TokenType = TokenType::new(&["Comment", "Multiline"]);
pub const COMMENT_PREPROC: TokenType = TokenType::new(&["Comment", "Preproc"]);
pub const COMMENT_PREPROCFILE: TokenType = TokenType::new(&["Comment", "PreprocFile"]);
pub const COMMENT_SINGLE: TokenType = TokenType::new(&["Comment", "Single"]);
pub const COMMENT_SPECIAL: TokenType = TokenType::new(&["Comment", "Special"]);

pub const GENERIC: TokenType = TokenType::new(&["Generic"]);
pub const GENERIC_DELETED: TokenType = TokenType::new(&["Generic", "Deleted"]);
pub const GENERIC_EMPH: TokenType = TokenType::new(&["Generic", "Emph"]);
pub const GENERIC_ERROR: TokenType = TokenType::new(&["Generic", "Error"]);
pub const GENERIC_HEADING: TokenType = TokenType::new(&["Generic", "Heading"]);
pub const GENERIC_INSERTED: TokenType = TokenType::new(&["Generic", "Inserted"]);
pub const GENERIC_OUTPUT: TokenType = TokenType::new(&["Generic", "Output"]);
pub const GENERIC_PROMPT: TokenType = TokenType::new(&["Generic", "Prompt"]);
pub const GENERIC_STRONG: TokenType = TokenType::new(&["Generic", "Strong"]);
pub const GENERIC_SUBHEADING: TokenType = TokenType::new(&["Generic", "Subheading"]);
pub const GENERIC_TRACEBACK: TokenType = TokenType::new(&["Generic", "Traceback"]);

/// Parse `"Literal.String.Double"` (with or without leading `Token.`)
/// into the static `TokenType` if known. Returns `None` for unknown
/// dotted names — the caller can fall back to constructing an
/// owned `Vec<String>`-backed type when full dynamic support lands.
pub fn from_dotted(name: &str) -> Option<TokenType> {
    let trimmed = name.strip_prefix("Token.").unwrap_or(name);
    let trimmed = trimmed.strip_prefix("Token").unwrap_or(trimmed);
    if trimmed.is_empty() {
        return Some(TOKEN);
    }
    let segs: Vec<&str> = trimmed.split('.').collect();
    match segs.as_slice() {
        ["Text"] => Some(TEXT),
        ["Text", "Whitespace"] => Some(WHITESPACE),
        ["Escape"] => Some(ESCAPE),
        ["Error"] => Some(ERROR),
        ["Other"] => Some(OTHER),
        ["Keyword"] => Some(KEYWORD),
        ["Name"] => Some(NAME),
        ["Name", "Function"] => Some(NAME_FUNCTION),
        ["Literal"] => Some(LITERAL),
        ["Literal", "String"] => Some(STRING),
        ["Literal", "Number"] => Some(NUMBER),
        ["Literal", "Number", "Integer"] => Some(NUMBER_INTEGER),
        ["Punctuation"] => Some(PUNCTUATION),
        ["Operator"] => Some(OPERATOR),
        ["Comment"] => Some(COMMENT),
        ["Generic"] => Some(GENERIC),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repr_root_and_subtypes() {
        assert_eq!(TOKEN.repr(), "Token");
        assert_eq!(TEXT.repr(), "Token.Text");
        assert_eq!(NAME_FUNCTION.repr(), "Token.Name.Function");
        assert_eq!(STRING_DOUBLE.repr(), "Token.Literal.String.Double");
    }

    #[test]
    fn contains_subtypes() {
        assert!(LITERAL.contains(STRING));
        assert!(STRING.contains(STRING_DOUBLE));
        assert!(TOKEN.contains(NAME_FUNCTION));
        assert!(NAME_FUNCTION.contains(NAME_FUNCTION));
        assert!(!NAME_FUNCTION.contains(NAME));
        assert!(!STRING.contains(NUMBER));
    }

    #[test]
    fn split_walks_ancestry() {
        let chain = NAME_FUNCTION.split();
        let reprs: Vec<String> = chain.iter().map(|t| t.repr()).collect();
        assert_eq!(reprs, vec!["Token", "Token.Name", "Token.Name.Function"]);
    }

    #[test]
    fn short_name_matches_standard_types() {
        // Spot-check rows from pygments.token.STANDARD_TYPES.
        assert_eq!(TOKEN.short_name(), "");
        assert_eq!(TEXT.short_name(), "");
        assert_eq!(WHITESPACE.short_name(), "w");
        assert_eq!(ERROR.short_name(), "err");
        assert_eq!(KEYWORD.short_name(), "k");
        assert_eq!(KEYWORD_CONSTANT.short_name(), "kc");
        assert_eq!(NAME_FUNCTION.short_name(), "nf");
        assert_eq!(NAME_FUNCTION_MAGIC.short_name(), "fm");
        assert_eq!(NAME_BUILTIN_PSEUDO.short_name(), "bp");
        assert_eq!(STRING.short_name(), "s");
        assert_eq!(STRING_DOUBLE.short_name(), "s2");
        assert_eq!(STRING_ESCAPE.short_name(), "se");
        assert_eq!(NUMBER_INTEGER.short_name(), "mi");
        assert_eq!(OPERATOR_WORD.short_name(), "ow");
        assert_eq!(COMMENT_SINGLE.short_name(), "c1");
        assert_eq!(PUNCTUATION.short_name(), "p");
    }

    #[test]
    fn short_name_walks_to_known_ancestor() {
        // `Name.Variable.Magic` IS in STANDARD_TYPES (`vm`) but a
        // hypothetical `Name.Variable.Magic.Extra` would fall back
        // to its nearest known ancestor.
        let extra =
            TokenType::new(&["Name", "Variable", "Magic", "Extra"]);
        assert_eq!(extra.short_name(), "vm");
        let totally_unknown = TokenType::new(&["NotAToken"]);
        assert_eq!(totally_unknown.short_name(), "");
    }
}
