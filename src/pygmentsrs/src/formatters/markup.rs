//! Markup formatters — F2 phase.
//!
//! `GroffFormatter` — troff escape sequences
//! `PangoMarkupFormatter` — Pango XML markup
//! `LatexFormatter` — LaTeX macros
//! `RtfFormatter` — Rich Text Format

use super::color::rgb_to_hex;
use super::style::Style;
use crate::token::TokenType;
use std::collections::HashMap;

pub struct GroffFormatter {
    color_map: HashMap<String, usize>,
    next_color_id: usize,
}

impl GroffFormatter {
    pub fn new() -> Self {
        Self {
            color_map: HashMap::new(),
            next_color_id: 1,
        }
    }

    pub fn format(&mut self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();
        out.push_str(".nf\n"); // no-fill mode (preserve formatting)

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);

            if let Some((r, g, b)) = style.fg_color {
                let hex = rgb_to_hex(r, g, b);
                if !self.color_map.contains_key(&hex) {
                    let id = self.next_color_id;
                    self.color_map.insert(hex.clone(), id);
                    self.next_color_id += 1;
                    out.push_str(&format!(
                        ".defcolor {} rgb {} {} {}\n",
                        id,
                        r as f32 / 255.0,
                        g as f32 / 255.0,
                        b as f32 / 255.0
                    ));
                }
                let color_id = self.color_map[&hex];
                out.push_str(&format!(".mcolor {}\n", color_id));
            }

            if style.bold {
                out.push_str(".ft B\n");
            }

            out.push_str(value);

            if style.bold || style.fg_color.is_some() {
                out.push_str(".ft R\n");
            }
        }

        out.push_str(".fi\n");
        out
    }
}

impl Default for GroffFormatter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PangoMarkupFormatter;

impl PangoMarkupFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);
            let mut attrs = Vec::new();

            if let Some((r, g, b)) = style.fg_color {
                let hex = rgb_to_hex(r, g, b);
                attrs.push(format!("foreground='{}'", hex));
            }

            if style.bold {
                attrs.push("weight='bold'".to_string());
            }

            if style.italic {
                attrs.push("style='italic'".to_string());
            }

            if style.underline {
                attrs.push("underline='single'".to_string());
            }

            if !attrs.is_empty() {
                out.push_str("<span ");
                out.push_str(&attrs.join(" "));
                out.push_str(">");
            }

            // Escape XML entities
            for c in value.chars() {
                match c {
                    '&' => out.push_str("&amp;"),
                    '<' => out.push_str("&lt;"),
                    '>' => out.push_str("&gt;"),
                    '"' => out.push_str("&quot;"),
                    '\'' => out.push_str("&apos;"),
                    _ => out.push(c),
                }
            }

            if !attrs.is_empty() {
                out.push_str("</span>");
            }
        }

        out
    }
}

pub struct LatexFormatter;

impl LatexFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::from("\\documentclass{article}\n");
        out.push_str("\\usepackage{xcolor}\n");
        out.push_str("\\usepackage{listings}\n");
        out.push_str("\\begin{document}\n");
        out.push_str("\\begin{lstlisting}\n");

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);
            let escaped = Self::escape_latex(value);

            if style.bold || style.italic || style.fg_color.is_some() {
                if style.bold {
                    out.push_str("\\textbf{");
                }
                if style.italic {
                    out.push_str("\\textit{");
                }

                if let Some((r, g, b)) = style.fg_color {
                    let hex = rgb_to_hex(r, g, b);
                    out.push_str(&format!("\\textcolor{{{}}}{{{}}}", hex, escaped));
                } else {
                    out.push_str(&escaped);
                }

                if style.italic {
                    out.push_str("}");
                }
                if style.bold {
                    out.push_str("}");
                }
            } else {
                out.push_str(&escaped);
            }
        }

        out.push_str("\\end{lstlisting}\n");
        out.push_str("\\end{document}\n");
        out
    }

    fn escape_latex(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '\\' => out.push_str("\\textbackslash{}"),
                '{' => out.push_str("\\{"),
                '}' => out.push_str("\\}"),
                '$' => out.push_str("\\$"),
                '&' => out.push_str("\\&"),
                '%' => out.push_str("\\%"),
                '#' => out.push_str("\\#"),
                '_' => out.push_str("\\_"),
                '^' => out.push_str("\\^{}"),
                '~' => out.push_str("\\textasciitilde{}"),
                '`' => out.push_str("\\textasciigrave{}"),
                '|' => out.push_str("\\textbar{}"),
                // Escape control chars (0x00-0x1F) as \textasciigrave or similar
                c if c.is_control() => {
                    // Replace control chars with visible placeholder
                    out.push_str(&format!("{{\\small [{:02X}]}}", c as u8));
                }
                _ => out.push(c),
            }
        }
        out
    }
}

pub struct RtfFormatter {
    color_table: Vec<(u8, u8, u8)>,
}

impl RtfFormatter {
    pub fn new() -> Self {
        Self {
            color_table: vec![(0, 0, 0)], // Start with black
        }
    }

    pub fn format(&mut self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::new();

        // RTF header
        out.push_str("{\\rtf1\\ansi\\ansicpg1252\n");
        out.push_str("{\\colortbl;");

        // Collect all colors first
        let mut color_map = HashMap::new();
        for (ttype, _) in tokens {
            let style = Style::from_token(*ttype);
            if let Some((r, g, b)) = style.fg_color {
                if !color_map.contains_key(&(r, g, b)) {
                    color_map.insert((r, g, b), self.color_table.len());
                    self.color_table.push((r, g, b));
                }
            }
        }

        // Write color table
        for (r, g, b) in &self.color_table {
            out.push_str(&format!("\\red{}\\green{}\\blue{};", r, g, b));
        }
        out.push_str("}\n");

        // Font table
        out.push_str("{\\fonttbl{\\f0\\fmodern Courier New;}}\n");

        // Body
        out.push_str("\\f0\\fs20\n");

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);

            if style.bold {
                out.push_str("\\b ");
            }
            if style.italic {
                out.push_str("\\i ");
            }
            if style.underline {
                out.push_str("\\ul ");
            }

            if let Some(color) = style.fg_color {
                if let Some(&color_idx) = color_map.get(&color) {
                    out.push_str(&format!("\\cf{} ", color_idx + 1)); // +1 because RTF is 1-indexed
                }
            }

            // Escape RTF special chars and control chars
            for c in value.chars() {
                match c {
                    '\\' => out.push_str("\\\\"),
                    '{' => out.push_str("\\{"),
                    '}' => out.push_str("\\}"),
                    '\n' => out.push_str("\\par\n"),
                    '\r' => {}
                    // Escape control chars (0x00-0x1F, except newline/carriage return already handled)
                    c if c.is_control() => {
                        // RTF hex escape: \'HH
                        let code = c as u8;
                        out.push_str(&format!("\\'{:02x}", code));
                    }
                    _ => out.push(c),
                }
            }

            if style.underline {
                out.push_str("\\ul0 ");
            }
            if style.italic {
                out.push_str("\\i0 ");
            }
            if style.bold {
                out.push_str("\\b0 ");
            }
        }

        out.push_str("}\n");
        out
    }
}

impl Default for RtfFormatter {
    fn default() -> Self {
        Self::new()
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
            (STRING_DOUBLE, "hello".to_string()),
        ]
    }

    #[test]
    fn test_groff_formatter() {
        let mut formatter = GroffFormatter::new();
        let tokens = make_tokens();
        let result = formatter.format(&tokens);
        assert!(result.contains(".nf"));
        assert!(result.contains(".fi"));
    }

    #[test]
    fn test_pango_formatter() {
        let tokens = make_tokens();
        let result = PangoMarkupFormatter.format(&tokens);
        assert!(result.contains("<span"));
        assert!(result.contains("</span>"));
    }

    #[test]
    fn test_latex_formatter() {
        let tokens = make_tokens();
        let result = LatexFormatter.format(&tokens);
        assert!(result.contains("\\documentclass"));
        assert!(result.contains("\\begin{lstlisting}"));
    }

    #[test]
    fn test_rtf_formatter() {
        let mut formatter = RtfFormatter::new();
        let tokens = make_tokens();
        let result = formatter.format(&tokens);
        assert!(result.contains("{\\rtf1"));
        assert!(result.contains("\\colortbl"));
    }
}
