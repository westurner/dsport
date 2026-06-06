//! Color utilities — RGB ↔ ANSI/IRC/BBCode conversion.
//!
//! Shared by terminal, terminal256, irc, bbcode formatters.
//! Implements:
//! - RGB → ANSI 16-color (3-bit + bright bit)
//! - RGB → ANSI 256-color (6×6×6 cube + grayscale)
//! - RGB → mIRC color index
//! - RGB → hex string

/// Parse CSS color string ("#RRGGBB" or name) to (R, G, B).
/// Returns black (0, 0, 0) if unrecognized.
pub fn parse_color(color: &str) -> (u8, u8, u8) {
    if let Some(hex) = color.strip_prefix('#') {
        if hex.len() == 6 {
            if let Ok(val) = u32::from_str_radix(hex, 16) {
                let r = ((val >> 16) & 0xFF) as u8;
                let g = ((val >> 8) & 0xFF) as u8;
                let b = (val & 0xFF) as u8;
                return (r, g, b);
            }
        }
    }
    // Named colors — common subset
    match color.to_lowercase().as_str() {
        "black" => (0, 0, 0),
        "red" => (255, 0, 0),
        "green" => (0, 128, 0),
        "yellow" => (255, 255, 0),
        "blue" => (0, 0, 255),
        "magenta" | "purple" => (255, 0, 255),
        "cyan" => (0, 255, 255),
        "white" => (255, 255, 255),
        "gray" | "grey" => (128, 128, 128),
        _ => (0, 0, 0),
    }
}

/// Convert RGB to hex string "#RRGGBB".
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// 16 ANSI color palette (indices 0-15).
const ANSI_16: [(u8, u8, u8); 16] = [
    (0, 0, 0),           // 0: black
    (128, 0, 0),         // 1: maroon
    (0, 128, 0),         // 2: green
    (128, 128, 0),       // 3: olive
    (0, 0, 128),         // 4: navy
    (128, 0, 128),       // 5: purple
    (0, 128, 128),       // 6: teal
    (192, 192, 192),     // 7: silver
    (128, 128, 128),     // 8: gray
    (255, 0, 0),         // 9: red
    (0, 255, 0),         // 10: lime
    (255, 255, 0),       // 11: yellow
    (0, 0, 255),         // 12: blue
    (255, 0, 255),       // 13: magenta
    (0, 255, 255),       // 14: cyan
    (255, 255, 255),     // 15: white
];

/// Convert RGB to ANSI 16-color index (0-15) using nearest neighbor.
pub fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> u8 {
    let mut best_idx = 0;
    let mut best_dist = u32::MAX;
    
    for (idx, &(cr, cg, cb)) in ANSI_16.iter().enumerate() {
        let dr = (r as i32 - cr as i32).abs() as u32;
        let dg = (g as i32 - cg as i32).abs() as u32;
        let db = (b as i32 - cb as i32).abs() as u32;
        let dist = dr * dr + dg * dg + db * db;
        
        if dist < best_dist {
            best_dist = dist;
            best_idx = idx;
        }
    }
    
    best_idx as u8
}

/// Convert RGB to ANSI 256-color index (0-255).
/// Uses 6×6×6 RGB cube (16-231) with fallback to grayscale (232-255).
pub fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    // Grayscale fallback: if R=G=B (or very close), use grayscale ramp (232-255)
    if (r as i32 - g as i32).abs() < 5 && (g as i32 - b as i32).abs() < 5 {
        // 24-step grayscale: 232-255 covers #080808 to #EEEEEE
        let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;
        if gray < 48 {
            return 16; // near black
        }
        if gray > 238 {
            return 231; // near white
        }
        // Map to 232-255 range (use u32 to avoid overflow in intermediate computation)
        let step = ((gray as u32 - 48) * 24) / 208;
        return (232 + step.min(23)) as u8;
    }
    
    // 6×6×6 RGB cube: map each channel to 0-5
    let r_idx = (r as u32 * 6) / 256;
    let g_idx = (g as u32 * 6) / 256;
    let b_idx = (b as u32 * 6) / 256;
    
    // Cube formula: 16 + 36*r + 6*g + b
    (16 + 36 * r_idx + 6 * g_idx + b_idx) as u8
}

/// 16 mIRC standard color codes.
const MIRC_COLORS: [(u8, u8, u8); 16] = [
    (255, 255, 255),     // 00: white
    (0, 0, 0),           // 01: black
    (0, 0, 127),         // 02: blue
    (0, 147, 0),         // 03: green
    (255, 0, 0),         // 04: red
    (127, 0, 0),         // 05: brown
    (156, 0, 156),       // 06: magenta
    (252, 127, 0),       // 07: orange
    (255, 255, 0),       // 08: yellow
    (0, 252, 0),         // 09: light green
    (0, 147, 147),       // 10: teal
    (0, 255, 255),       // 11: cyan
    (0, 0, 252),         // 12: light blue
    (255, 0, 255),       // 13: light magenta
    (127, 127, 127),     // 14: gray
    (192, 192, 192),     // 15: light gray
];

/// Convert RGB to mIRC color index (0-15) using nearest neighbor.
pub fn rgb_to_mirc(r: u8, g: u8, b: u8) -> u8 {
    let mut best_idx = 0;
    let mut best_dist = u32::MAX;
    
    for (idx, &(cr, cg, cb)) in MIRC_COLORS.iter().enumerate() {
        let dr = (r as i32 - cr as i32).abs() as u32;
        let dg = (g as i32 - cg as i32).abs() as u32;
        let db = (b as i32 - cb as i32).abs() as u32;
        let dist = dr * dr + dg * dg + db * db;
        
        if dist < best_dist {
            best_dist = dist;
            best_idx = idx;
        }
    }
    
    best_idx as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("#FF0000"), (255, 0, 0));
        assert_eq!(parse_color("#00FF00"), (0, 255, 0));
        assert_eq!(parse_color("red"), (255, 0, 0));
        assert_eq!(parse_color("black"), (0, 0, 0));
    }

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex(255, 0, 0), "#ff0000");
        assert_eq!(rgb_to_hex(0, 255, 0), "#00ff00");
    }

    #[test]
    fn test_rgb_to_ansi16() {
        let idx = rgb_to_ansi16(255, 0, 0);
        assert!(idx < 16);
    }

    #[test]
    fn test_rgb_to_ansi256() {
        let idx = rgb_to_ansi256(255, 0, 0);
        // idx is u8, so it's always < 256 by type. Just verify it returns some value.
        let _ = idx;
    }

    #[test]
    fn test_rgb_to_mirc() {
        let idx = rgb_to_mirc(255, 0, 0);
        assert!(idx < 16);
    }
}
