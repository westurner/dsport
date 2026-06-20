//! SVG formatter — F3 phase.
//!
//! Renders tokens to SVG with monospace layout.
//! Assumes fixed-width font and line height.

use super::style::Style;
use crate::token::TokenType;

pub struct SvgFormatter {
    font_width: f32,  // pixels per character (monospace)
    line_height: f32, // pixels per line
}

impl SvgFormatter {
    pub fn new() -> Self {
        Self {
            font_width: 7.2, // Courier New, typical
            line_height: 14.0,
        }
    }

    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        // Measure dimensions
        let mut max_width: f32 = 0.0;
        let mut line_count = 1;
        let mut col = 0.0;

        for (_, value) in tokens {
            for c in value.chars() {
                if c == '\n' {
                    line_count += 1;
                    col = 0.0;
                } else {
                    col += self.font_width;
                    max_width = max_width.max(col);
                }
            }
        }

        let width = max_width + 20.0; // padding
        let height = (line_count as f32 * self.line_height) + 20.0;

        let mut out = String::new();
        out.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\n",
            width as u32, height as u32, width as u32, height as u32
        ));

        // Background
        out.push_str(&format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"white\"/>\n",
            width as u32, height as u32
        ));

        // CSS styles
        out.push_str("<style>\n");
        out.push_str("text { font-family: 'Courier New', monospace; font-size: 12px; }\n");
        out.push_str(".tok-keyword { fill: #0000ff; font-weight: bold; }\n");
        out.push_str(".tok-string { fill: #c80000; }\n");
        out.push_str(".tok-comment { fill: #969696; }\n");
        out.push_str(".tok-number { fill: #8b4513; }\n");
        out.push_str("</style>\n");

        // Render tokens
        let mut x = 10.0;
        let mut y = 10.0 + self.line_height * 0.8; // baseline

        for (ttype, value) in tokens {
            let style = Style::from_token(*ttype);

            // Split text at newlines to handle multi-line tokens
            for (line_idx, line) in value.split('\n').enumerate() {
                if line_idx > 0 {
                    // Move to next line
                    x = 10.0;
                    y += self.line_height;
                }

                if line.is_empty() {
                    continue;
                }

                // Start text element with color
                if let Some((r, g, b)) = style.fg_color {
                    let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
                    out.push_str(&format!(
                        "<text x=\"{}\" y=\"{}\" style=\"fill: {};\">",
                        x as u32, y as u32, hex
                    ));
                } else {
                    out.push_str(&format!("<text x=\"{}\" y=\"{}\">", x as u32, y as u32));
                }

                // Apply text decorations
                if style.bold {
                    out.push_str("<tspan style=\"font-weight: bold;\">");
                }
                if style.italic {
                    out.push_str("<tspan style=\"font-style: italic;\">");
                }
                if style.underline {
                    out.push_str("<tspan style=\"text-decoration: underline;\">");
                }

                // Escape XML entities
                for c in line.chars() {
                    match c {
                        '&' => out.push_str("&amp;"),
                        '<' => out.push_str("&lt;"),
                        '>' => out.push_str("&gt;"),
                        '"' => out.push_str("&quot;"),
                        _ => out.push(c),
                    }
                }

                // Close decorations and text
                if style.underline {
                    out.push_str("</tspan>");
                }
                if style.italic {
                    out.push_str("</tspan>");
                }
                if style.bold {
                    out.push_str("</tspan>");
                }

                out.push_str("</text>\n");

                // Update x position for next character
                x += line.len() as f32 * self.font_width;
            }
        }

        out.push_str("</svg>\n");
        out
    }
}

impl Default for SvgFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn test_svg_formatter() {
        let formatter = SvgFormatter::new();
        let tokens = vec![
            (KEYWORD, "let".to_string()),
            (TEXT, " ".to_string()),
            (NAME, "x".to_string()),
        ];
        let result = formatter.format(&tokens);
        assert!(result.contains("<svg"));
        assert!(result.contains("</svg>"));
        assert!(result.contains("Courier New"));
    }

    #[test]
    fn test_svg_with_newline() {
        let formatter = SvgFormatter::new();
        let tokens = vec![(KEYWORD, "let\n".to_string()), (NAME, "x".to_string())];
        let result = formatter.format(&tokens);
        assert!(result.contains("</text>"));
        assert!(result.contains("<text"));
    }
}
