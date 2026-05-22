//! `pygments.lexers.data.JsonLexer` — direct port.
//!
//! Upstream is a hand-written character-stream state machine (not a
//! [`RegexLexer`][crate::lexer::engine]) so we do the same. The
//! algorithm and behaviour are mirrored line-for-line from
//! `src/pygments/pygments/lexers/data.py::JsonLexer.get_tokens_unprocessed`.
//!
//! Notable features preserved for byte-parity:
//!
//! * Strings followed by `":"` re-tokenize as `Name.Tag` (object
//!   keys), queued until the colon is observed.
//! * JS-style `//` and `/* … */` comments are accepted even though
//!   they are not part of strict JSON.
//! * Numbers, constants, punctuation runs are coalesced.
//! * Trailing partial tokens are flushed at EOF with the same fallback
//!   rules upstream uses (open string → `Error`, etc.).

use crate::lexer::Lexer;
use crate::token::{
    self, COMMENT_MULTILINE, COMMENT_SINGLE, ERROR, KEYWORD_CONSTANT, NAME_TAG, NUMBER_FLOAT,
    NUMBER_INTEGER, PUNCTUATION, STRING_DOUBLE, TokenType, WHITESPACE,
};

pub struct JsonLexer;

#[inline]
fn is_integer(c: char) -> bool {
    matches!(c, '-' | '0'..='9')
}
#[inline]
fn is_float(c: char) -> bool {
    matches!(c, '.' | 'e' | 'E' | '+')
}
#[inline]
fn is_constant(c: char) -> bool {
    matches!(c, 't' | 'r' | 'u' | 'e' | 'f' | 'a' | 'l' | 's' | 'n')
}
#[inline]
fn is_hex(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F')
}
#[inline]
fn is_punct(c: char) -> bool {
    matches!(c, '{' | '}' | '[' | ']' | ',')
}
#[inline]
fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\n' | '\r' | '\t')
}

impl Lexer for JsonLexer {
    fn get_tokens(&self, text: &str) -> Vec<(TokenType, String)> {
        let mut out: Vec<(TokenType, String)> = Vec::new();
        let mut queue: Vec<(usize, TokenType, String)> = Vec::new();

        let mut in_string = false;
        let mut in_escape = false;
        let mut in_unicode_escape: u8 = 0;
        let mut in_whitespace = false;
        let mut in_constant = false;
        let mut in_number = false;
        let mut in_float = false;
        let mut in_punctuation = false;
        let mut in_comment_single = false;
        let mut in_comment_multiline = false;
        let mut expecting_second_comment_opener = false;
        let mut expecting_second_comment_closer = false;

        let mut start = 0usize;

        // Step over each char by its byte index so slicing matches
        // pygments' Python str slicing (which is by code point but
        // for this lexer's byte-level operations on ASCII matches).
        // JSON content can contain non-ASCII bytes inside strings;
        // we operate on chars and track byte indices.
        let chars: Vec<(usize, char)> = text.char_indices().collect();
        let push = |out: &mut Vec<(TokenType, String)>,
                    queue: &mut Vec<(usize, TokenType, String)>,
                    tok: (usize, TokenType, String)| {
            if queue.is_empty() {
                out.push((tok.1, tok.2));
            } else {
                queue.push(tok);
            }
        };

        let flush_queue = |out: &mut Vec<(TokenType, String)>,
                           queue: &mut Vec<(usize, TokenType, String)>| {
            for (_, t, v) in queue.drain(..) {
                out.push((t, v));
            }
        };

        for i in 0..chars.len() {
            let (stop, character) = chars[i];
            // ---- continuation branches first (mirror upstream) ----
            if in_string {
                if in_unicode_escape > 0 {
                    if is_hex(character) {
                        in_unicode_escape -= 1;
                        if in_unicode_escape == 0 {
                            in_escape = false;
                        }
                    } else {
                        in_unicode_escape = 0;
                        in_escape = false;
                    }
                } else if in_escape {
                    if character == 'u' {
                        in_unicode_escape = 4;
                    } else {
                        in_escape = false;
                    }
                } else if character == '\\' {
                    in_escape = true;
                } else if character == '"' {
                    let end = stop + character.len_utf8();
                    let slice = text[start..end].to_string();
                    queue.push((start, STRING_DOUBLE, slice));
                    in_string = false;
                    in_escape = false;
                    in_unicode_escape = 0;
                }
                continue;
            } else if in_whitespace {
                if is_whitespace(character) {
                    continue;
                }
                let slice = text[start..stop].to_string();
                push(&mut out, &mut queue, (start, WHITESPACE, slice));
                in_whitespace = false;
                // fall through
            } else if in_constant {
                if is_constant(character) {
                    continue;
                }
                out.push((KEYWORD_CONSTANT, text[start..stop].to_string()));
                in_constant = false;
            } else if in_number {
                if is_integer(character) {
                    continue;
                } else if is_float(character) {
                    in_float = true;
                    continue;
                }
                let t = if in_float { NUMBER_FLOAT } else { NUMBER_INTEGER };
                out.push((t, text[start..stop].to_string()));
                in_number = false;
                in_float = false;
            } else if in_punctuation {
                if is_punct(character) {
                    continue;
                }
                out.push((PUNCTUATION, text[start..stop].to_string()));
                in_punctuation = false;
            } else if in_comment_single {
                if character != '\n' {
                    continue;
                }
                let slice = text[start..stop].to_string();
                push(&mut out, &mut queue, (start, COMMENT_SINGLE, slice));
                in_comment_single = false;
            } else if in_comment_multiline {
                if character == '*' {
                    expecting_second_comment_closer = true;
                } else if expecting_second_comment_closer {
                    expecting_second_comment_closer = false;
                    if character == '/' {
                        let end = stop + character.len_utf8();
                        let slice = text[start..end].to_string();
                        push(&mut out, &mut queue, (start, COMMENT_MULTILINE, slice));
                        in_comment_multiline = false;
                    }
                }
                continue;
            } else if expecting_second_comment_opener {
                expecting_second_comment_opener = false;
                if character == '/' {
                    in_comment_single = true;
                    continue;
                } else if character == '*' {
                    in_comment_multiline = true;
                    continue;
                }
                flush_queue(&mut out, &mut queue);
                out.push((ERROR, text[start..stop].to_string()));
                // fall through
            }

            start = stop;

            if character == '"' {
                in_string = true;
            } else if is_whitespace(character) {
                in_whitespace = true;
            } else if matches!(character, 'f' | 'n' | 't') {
                flush_queue(&mut out, &mut queue);
                in_constant = true;
            } else if is_integer(character) {
                flush_queue(&mut out, &mut queue);
                in_number = true;
            } else if character == ':' {
                // Replace any quoted-string tokens in the queue with Name.Tag.
                for (_s, tok, val) in queue.drain(..) {
                    if tok == STRING_DOUBLE {
                        out.push((NAME_TAG, val));
                    } else {
                        out.push((tok, val));
                    }
                }
                in_punctuation = true;
            } else if is_punct(character) {
                flush_queue(&mut out, &mut queue);
                in_punctuation = true;
            } else if character == '/' {
                expecting_second_comment_opener = true;
            } else {
                flush_queue(&mut out, &mut queue);
                out.push((ERROR, character.to_string()));
            }
        }

        // Flush queue + open partial tokens at EOF.
        flush_queue(&mut out, &mut queue);
        let tail = || text[start..].to_string();
        if in_string {
            out.push((ERROR, tail()));
        } else if in_float {
            out.push((NUMBER_FLOAT, tail()));
        } else if in_number {
            out.push((NUMBER_INTEGER, tail()));
        } else if in_constant {
            out.push((KEYWORD_CONSTANT, tail()));
        } else if in_whitespace {
            out.push((WHITESPACE, tail()));
        } else if in_punctuation {
            out.push((PUNCTUATION, tail()));
        } else if in_comment_single {
            out.push((COMMENT_SINGLE, tail()));
        } else if in_comment_multiline {
            out.push((ERROR, tail()));
        } else if expecting_second_comment_opener {
            out.push((ERROR, tail()));
        }

        // Silence unused-import if token::TOKEN ever goes unused.
        let _ = token::TOKEN;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(src: &str) -> Vec<(String, String)> {
        JsonLexer
            .get_tokens(src)
            .into_iter()
            .map(|(t, v)| (t.repr(), v))
            .collect()
    }

    #[test]
    fn simple_object() {
        let toks = lex("{\"k\": 1}");
        // Object key reclassified as Name.Tag.
        assert_eq!(
            toks,
            vec![
                ("Token.Punctuation".to_string(), "{".to_string()),
                ("Token.Name.Tag".to_string(), "\"k\"".to_string()),
                ("Token.Punctuation".to_string(), ":".to_string()),
                ("Token.Text.Whitespace".to_string(), " ".to_string()),
                ("Token.Literal.Number.Integer".to_string(), "1".to_string()),
                ("Token.Punctuation".to_string(), "}".to_string()),
            ]
        );
    }

    #[test]
    fn constants_and_string_value() {
        let toks = lex("[true, false, null, \"x\"]");
        assert!(toks.contains(&(
            "Token.Keyword.Constant".to_string(),
            "true".to_string()
        )));
        assert!(toks.contains(&(
            "Token.Literal.String.Double".to_string(),
            "\"x\"".to_string()
        )));
    }

    #[test]
    fn numbers_int_and_float() {
        let toks = lex("[1, 2.5, -3e10]");
        let kinds: Vec<&str> = toks.iter().map(|(t, _)| t.as_str()).collect();
        assert!(kinds.contains(&"Token.Literal.Number.Integer"));
        assert!(kinds.contains(&"Token.Literal.Number.Float"));
    }

    #[test]
    fn line_and_block_comments() {
        let toks = lex("// hi\n/* multi */\n1");
        assert!(toks.iter().any(|(t, _)| t == "Token.Comment.Single"));
        assert!(toks.iter().any(|(t, _)| t == "Token.Comment.Multiline"));
    }
}
