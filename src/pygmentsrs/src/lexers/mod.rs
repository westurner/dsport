//! Lexer registry — `get_lexer_by_name(alias)` mirrors
//! `pygments.lexers.get_lexer_by_name`.
//!
//! Phase 0 ships only the `text` passthrough so end-to-end builds
//! work; Phase 1 adds the Python lexer; Phase 2 widens the set.

pub mod delegating;
pub mod diff;
pub mod generated;
pub mod json;
pub mod json_ld;
pub mod python;
pub mod registry;
pub mod text;
pub mod yaml_ld;

use crate::lexer::Lexer;
use crate::token::{self, TokenType};

/// Port of `pygments.lexer.DelegatingLexer`.
///
/// Combines a *language* lexer (e.g. a template engine like Jinja/Twig) with
/// a *root* lexer (e.g. HTML).  Algorithm:
///
/// 1. Run `language.get_tokens` over the entire input.
/// 2. Collect every token whose type **is** the `needle` type (default
///    `Token.Other`) into a plain-text buffer, recording where each
///    non-needle span falls (the *insertions*).
/// 3. Run `root.get_tokens` over the buffered plain text.
/// 4. Splice the language tokens back into the root token stream at the
///    recorded byte positions (`do_insertions`).
///
/// This is byte-for-byte compatible with Python's upstream implementation
/// — the `do_insertions` logic mirrors `pygments.lexer.do_insertions`.
pub struct DelegatingLexer {
    pub root: Box<dyn Lexer>,
    pub language: Box<dyn Lexer>,
    pub needle: TokenType,
}

impl DelegatingLexer {
    pub fn new(root: Box<dyn Lexer>, language: Box<dyn Lexer>) -> Self {
        Self {
            root,
            language,
            needle: token::OTHER,
        }
    }
    pub fn with_needle(root: Box<dyn Lexer>, language: Box<dyn Lexer>, needle: TokenType) -> Self {
        Self {
            root,
            language,
            needle,
        }
    }
}

impl Lexer for DelegatingLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let language_tokens = self.language.get_tokens(code);

        let mut buffered = String::new();
        let mut insertions: Vec<(usize, Vec<(TokenType, String)>)> = Vec::new();
        let mut lng_buffer: Vec<(TokenType, String)> = Vec::new();

        for (t, v) in language_tokens {
            if t == self.needle {
                if !lng_buffer.is_empty() {
                    insertions.push((buffered.len(), std::mem::take(&mut lng_buffer)));
                }
                buffered.push_str(&v);
            } else {
                lng_buffer.push((t, v));
            }
        }
        if !lng_buffer.is_empty() {
            insertions.push((buffered.len(), lng_buffer));
        }

        let root_tokens = self.root.get_tokens(&buffered);
        do_insertions(insertions, root_tokens)
    }
}

/// Splice language-token insertions back into a root token stream.
///
/// Public wrapper used by the `DispatchCodeBlock` engine action for
/// RST-style indented code blocks.  See the private `do_insertions`
/// below for full documentation.
pub fn do_insertions_owned(
    insertions: Vec<(usize, Vec<(TokenType, String)>)>,
    root_tokens: Vec<(TokenType, String)>,
) -> Vec<(TokenType, String)> {
    do_insertions(insertions, root_tokens)
}

/// Splice language-token insertions back into a root token stream.
///
/// `insertions` is a list of `(byte_index_in_buffered, lang_tokens)` pairs;
/// `root_tokens` is the root-lexer output over the buffered plain text.
/// The result merges both streams in order.
///
/// Mirrors `pygments.lexer.do_insertions` exactly, including the handling
/// of root tokens that span an insertion boundary (split into two pieces).
fn do_insertions(
    insertions: Vec<(usize, Vec<(TokenType, String)>)>,
    root_tokens: Vec<(TokenType, String)>,
) -> Vec<(TokenType, String)> {
    let mut out: Vec<(TokenType, String)> = Vec::with_capacity(
        root_tokens.len() + insertions.iter().map(|(_, v)| v.len()).sum::<usize>(),
    );
    let mut ins_iter = insertions.into_iter().peekable();
    let mut current_offset: usize = 0;

    for (t, v) in root_tokens {
        let v_len = v.len();
        let mut oldi = 0usize;

        // Drain all insertions whose byte index falls within this token.
        while let Some((idx, _)) = ins_iter.peek() {
            let next_index = *idx;
            if current_offset + (v_len - oldi) < next_index - current_offset + oldi {
                break;
            }
            // More precisely: does the insertion fall at or before end of this token?
            let end_of_token = current_offset + v_len;
            if next_index > end_of_token {
                break;
            }
            // Emit the root-token slice up to the insertion point.
            let split = next_index - current_offset;
            if split > oldi {
                let piece = &v[oldi..split];
                if !piece.is_empty() {
                    out.push((t, piece.to_string()));
                }
                oldi = split;
            }
            // Emit the language tokens at this insertion.
            let (_, itokens) = ins_iter.next().unwrap();
            for item in itokens {
                out.push(item);
            }
        }

        // Emit the remainder of this root token.
        if oldi < v_len {
            out.push((t, v[oldi..].to_string()));
        }
        current_offset += v_len;
    }

    // Any insertions past the end of the root stream (e.g. template tokens
    // at the very end of the document) are emitted in order.
    for (_, itokens) in ins_iter {
        for item in itokens {
            out.push(item);
        }
    }

    out
}
