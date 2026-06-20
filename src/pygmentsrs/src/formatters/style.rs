//! Token style extraction — TokenType → (color, bold, italic, underline).
//!
//! Maps Pygments token types to foreground color, bold, italic, and underline attributes.
//! Used by terminal, ANSI, and markup formatters.

use crate::token::TokenType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg_color: Option<(u8, u8, u8)>, // RGB
    pub bg_color: Option<(u8, u8, u8)>, // RGB (rarely used)
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Style {
    pub fn new() -> Self {
        Self {
            fg_color: None,
            bg_color: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }

    /// Extract style from a Pygments token type.
    /// Hard-coded for now; in a full port, would read from a style file.
    pub fn from_token(token: TokenType) -> Self {
        // Map token types to default styles
        // This is a simplified version; Pygments has full style sheets.
        use crate::token::*;

        if token == COMMENT
            || token == COMMENT_SINGLE
            || token == COMMENT_MULTILINE
            || token == COMMENT_PREPROC
        {
            return Style {
                fg_color: Some((150, 150, 150)), // gray
                bold: false,
                italic: false,
                underline: false,
                bg_color: None,
            };
        }

        if token == KEYWORD || token == KEYWORD_NAMESPACE {
            return Style {
                fg_color: Some((0, 0, 255)), // blue
                bold: true,
                italic: false,
                underline: false,
                bg_color: None,
            };
        }

        if token == NAME {
            return Style {
                fg_color: None,
                bold: false,
                italic: false,
                underline: false,
                bg_color: None,
            };
        }

        if token == NAME_FUNCTION {
            return Style {
                fg_color: Some((0, 128, 0)), // green
                bold: false,
                italic: false,
                underline: false,
                bg_color: None,
            };
        }

        if token == STRING || token == STRING_DOUBLE || token == STRING_SINGLE {
            return Style {
                fg_color: Some((200, 0, 0)), // red
                bold: false,
                italic: false,
                underline: false,
                bg_color: None,
            };
        }

        if token == NUMBER || token == NUMBER_INTEGER {
            return Style {
                fg_color: Some((139, 69, 19)), // brown
                bold: false,
                italic: false,
                underline: false,
                bg_color: None,
            };
        }

        if token == OPERATOR {
            return Style {
                fg_color: Some((255, 127, 0)), // orange
                bold: false,
                italic: false,
                underline: false,
                bg_color: None,
            };
        }

        // Default: no style
        Style::new()
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// ANSI SGR (Select Graphic Rendition) code builder.
/// Constructs escape sequences like ESC[1;31m for bold red.
pub struct AnsiBuf;

impl AnsiBuf {
    pub fn new() -> Self {
        Self
    }

    /// Add SGR codes for the given style (foreground color + attributes).
    /// Returns the escape sequence as a string.
    pub fn escape_for_style(style: Style, color_fn: fn(u8, u8, u8) -> u8) -> String {
        let mut codes = Vec::new();

        // Bold
        if style.bold {
            codes.push(1);
        }

        // Italic (non-standard; 3 is less portable)
        if style.italic {
            codes.push(3);
        }

        // Underline
        if style.underline {
            codes.push(4);
        }

        // Foreground color
        if let Some((r, g, b)) = style.fg_color {
            let color_idx = color_fn(r, g, b);
            if color_idx < 8 {
                // Standard colors (0-7): use codes 30-37
                codes.push(30 + color_idx);
            } else {
                // Bright colors (8-15) or beyond: not handled here, use 256-color instead
                codes.push(90 + (color_idx - 8));
            }
        }

        // Build escape sequence
        if codes.is_empty() {
            String::new()
        } else {
            let mut result = String::from("\x1b[");
            for (i, code) in codes.iter().enumerate() {
                if i > 0 {
                    result.push(';');
                }
                result.push_str(&code.to_string());
            }
            result.push('m');
            result
        }
    }

    /// ANSI 256-color escape sequence for foreground.
    pub fn escape_256(idx: u8) -> String {
        format!("\x1b[38;5;{}m", idx)
    }

    /// ANSI truecolor (24-bit) escape sequence for foreground.
    pub fn escape_truecolor(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }

    /// Reset all attributes.
    pub fn escape_reset() -> &'static str {
        "\x1b[0m"
    }
}

impl Default for AnsiBuf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn test_style_from_token() {
        let style = Style::from_token(COMMENT);
        assert!(style.fg_color.is_some());
    }

    #[test]
    fn test_ansi_escape() {
        let seq = AnsiBuf::escape_256(196);
        assert!(seq.contains("38;5;196"));
        assert!(seq.starts_with("\x1b["));
    }

    #[test]
    fn test_ansi_truecolor() {
        let seq = AnsiBuf::escape_truecolor(255, 0, 0);
        assert!(seq.contains("38;2;255;0;0"));
    }
}
