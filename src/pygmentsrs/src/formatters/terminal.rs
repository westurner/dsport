//! ANSI terminal formatters — F1 phase.
//!
//! `TerminalFormatter` — ANSI 16-color (3-bit)
//! `Terminal256Formatter` — ANSI 256-color (8-bit)
//! `TerminalTrueColorFormatter` — ANSI 24-bit truecolor
//! `IRCFormatter` — mIRC color codes
//! `BBCodeFormatter` — BBCode markup

use super::color::{rgb_to_ansi16, rgb_to_ansi256, rgb_to_hex, rgb_to_mirc};
use super::style::Style;
use crate::token::TokenType;

pub struct TerminalFormatter;

impl TerminalFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);
            if let Some((r, g, b)) = style.fg_color {
                let ansi_idx = rgb_to_ansi16(r, g, b);
                let escape = if ansi_idx < 8 {
                    // Standard colors (0-7)
                    format!("\x1b[{}m", 30 + ansi_idx)
                } else {
                    // Bright colors (8-15)
                    format!("\x1b[{}m", 90 + (ansi_idx - 8))
                };
                out.push_str(&escape);

                if style.bold {
                    out.push_str("\x1b[1m");
                }
                if style.italic {
                    out.push_str("\x1b[3m");
                }
                if style.underline {
                    out.push_str("\x1b[4m");
                }
            }

            out.push_str(value);

            if style.fg_color.is_some() || style.bold || style.italic || style.underline {
                out.push_str("\x1b[0m");
            }
        }

        out
    }
}

pub struct Terminal256Formatter;

impl Terminal256Formatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);
            if let Some((r, g, b)) = style.fg_color {
                let ansi_idx = rgb_to_ansi256(r, g, b);
                let escape = format!("\x1b[38;5;{}m", ansi_idx);
                out.push_str(&escape);

                if style.bold {
                    out.push_str("\x1b[1m");
                }
                if style.italic {
                    out.push_str("\x1b[3m");
                }
                if style.underline {
                    out.push_str("\x1b[4m");
                }
            }

            out.push_str(value);

            if style.fg_color.is_some() || style.bold || style.italic || style.underline {
                out.push_str("\x1b[0m");
            }
        }

        out
    }
}

pub struct TerminalTrueColorFormatter;

impl TerminalTrueColorFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);
            if let Some((r, g, b)) = style.fg_color {
                let escape = format!("\x1b[38;2;{};{};{}m", r, g, b);
                out.push_str(&escape);

                if style.bold {
                    out.push_str("\x1b[1m");
                }
                if style.italic {
                    out.push_str("\x1b[3m");
                }
                if style.underline {
                    out.push_str("\x1b[4m");
                }
            }

            out.push_str(value);

            if style.fg_color.is_some() || style.bold || style.italic || style.underline {
                out.push_str("\x1b[0m");
            }
        }

        out
    }
}

pub struct IRCFormatter;

impl IRCFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);

            if let Some((r, g, b)) = style.fg_color {
                let mirc_idx = rgb_to_mirc(r, g, b);
                // IRC color code: ^C foreground[,background]
                out.push_str(&format!("\x03{:02}", mirc_idx));

                if style.bold {
                    out.push('\x02'); // bold
                }
                if style.italic {
                    out.push('\x1d'); // italic
                }
                if style.underline {
                    out.push('\x1f'); // underline
                }
            }

            out.push_str(value);

            if style.fg_color.is_some() || style.bold || style.italic || style.underline {
                out.push('\x03'); // reset IRC codes
            }
        }

        out
    }
}

pub struct BBCodeFormatter;

impl BBCodeFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);

            if let Some((r, g, b)) = style.fg_color {
                let hex = rgb_to_hex(r, g, b);
                out.push_str(&format!("[color={}]", hex));
            }

            if style.bold {
                out.push_str("[b]");
            }

            if style.italic {
                out.push_str("[i]");
            }

            if style.underline {
                out.push_str("[u]");
            }

            // Escape BBCode special chars
            for c in value.chars() {
                match c {
                    '[' => out.push_str("&#91;"),
                    ']' => out.push_str("&#93;"),
                    _ => out.push(c),
                }
            }

            if style.underline {
                out.push_str("[/u]");
            }
            if style.italic {
                out.push_str("[/i]");
            }
            if style.bold {
                out.push_str("[/b]");
            }
            if let Some(_) = style.fg_color {
                out.push_str("[/color]");
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    fn make_tokens() -> Vec<(TokenType, String)> {
        vec![
            (KEYWORD, "let".to_string()),
            (TEXT, " ".to_string()),
            (NAME, "x".to_string()),
            (TEXT, " ".to_string()),
            (OPERATOR, "=".to_string()),
            (TEXT, " ".to_string()),
            (NUMBER_INTEGER, "42".to_string()),
            (TEXT, "\n".to_string()),
        ]
    }

    #[test]
    fn test_terminal_formatter() {
        let tokens = make_tokens();
        let result = TerminalFormatter.format(&tokens);
        assert!(!result.is_empty());
        assert!(result.contains("\x1b[")); // ANSI escape
    }

    #[test]
    fn test_terminal256_formatter() {
        let tokens = make_tokens();
        let result = Terminal256Formatter.format(&tokens);
        assert!(!result.is_empty());
        assert!(result.contains("38;5")); // 256-color code
    }

    #[test]
    fn test_terminal_truecolor_formatter() {
        let tokens = make_tokens();
        let result = TerminalTrueColorFormatter.format(&tokens);
        assert!(!result.is_empty());
        assert!(result.contains("38;2")); // Truecolor code
    }

    #[test]
    fn test_irc_formatter() {
        let tokens = make_tokens();
        let result = IRCFormatter.format(&tokens);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_bbcode_formatter() {
        let tokens = make_tokens();
        let result = BBCodeFormatter.format(&tokens);
        assert!(!result.is_empty());
        assert!(result.contains("["));
    }
}
