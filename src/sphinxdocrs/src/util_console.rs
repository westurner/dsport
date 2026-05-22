//! Port of `sphinx.util.console` and its underlying
//! `sphinx._cli.util.colour` + `sphinx._cli.util.errors` colour/escape
//! surface.
//!
//! Pure functions; no global mutable terminal state on the Rust side
//! is exposed to Python (the `_COLOURING_DISABLED` flag lives in a
//! `Mutex<bool>` so toggling from Python via `disable_colour()` /
//! `enable_colour()` mirrors upstream semantics).
//!
//! Parity targets:
//! * upstream `colourise(name, text)`, `terminal_safe`,
//!   `strip_escape_sequences`, every named colour function (`red`,
//!   `bold`, `reset`, …), `disable_colour`, `enable_colour`,
//!   `terminal_supports_colour`.

use std::sync::Mutex;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use regex::Regex;

static COLOURING_DISABLED: Mutex<bool> = Mutex::new(false);

fn colouring_disabled() -> bool {
    *COLOURING_DISABLED.lock().unwrap()
}

/// Wrap `text` with the SGR escape sequence `escape_code` and a
/// trailing reset, matching upstream's
/// `_create_colour_func(escape_code)`.
pub fn wrap(escape_code: &str, text: &str) -> String {
    if colouring_disabled() {
        return text.to_string();
    }
    format!("\x1b[{escape_code}m{text}\x1b[39;49;00m")
}

/// Table of `(name, escape_code)` mirroring upstream's module-level
/// `_create_colour_func` calls.
pub const COLOUR_TABLE: &[(&str, &str)] = &[
    ("reset", "39;49;00"),
    ("bold", "01"),
    ("faint", "02"),
    ("standout", "03"),
    ("underline", "04"),
    ("blink", "05"),
    ("black", "30"),
    ("darkred", "31"),
    ("darkgreen", "32"),
    ("brown", "33"),
    ("darkblue", "34"),
    ("purple", "35"),
    ("turquoise", "36"),
    ("lightgray", "37"),
    ("darkgray", "90"),
    ("red", "91"),
    ("green", "92"),
    ("yellow", "93"),
    ("blue", "94"),
    ("fuchsia", "95"),
    ("teal", "96"),
    ("white", "97"),
];

fn escape_code_for(name: &str) -> Option<&'static str> {
    COLOUR_TABLE
        .iter()
        .find_map(|(n, c)| (*n == name).then_some(*c))
}

/// Direct port of `sphinx._cli.util.colour.colourise`.
pub fn colourise(colour_name: &str, text: &str) -> Result<String, String> {
    if colouring_disabled() {
        return Ok(text.to_string());
    }
    // Upstream blacklists names starting with '_' plus a fixed set
    // pulled from the module's globals; we replicate by checking
    // only against the known colour table.
    let Some(code) = escape_code_for(colour_name) else {
        return Err(format!("Invalid colour name: {colour_name:?}"));
    };
    Ok(wrap(code, text))
}

/// `_ANSI_CODES` re-implementation. Matches the SGR colour code
/// shape `\x1b\[(?:\d+;){0,2}\d*m` or the erase-in-line shape
/// `\x1b\[[012]?K`.
fn ansi_re() -> &'static Regex {
    use std::sync::OnceLock;
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\x1b\[(?:(?:\d+;){0,2}\d*m|[012]?K)").unwrap())
}

/// Port of `sphinx._cli.util.errors.strip_escape_sequences`.
pub fn strip_escape_sequences(text: &str) -> String {
    ansi_re().replace_all(text, "").into_owned()
}

/// Port of `sphinx._cli.util.errors.terminal_safe` (also re-exported
/// as `sphinx.util.console.terminal_safe`). Encodes via ASCII with
/// `backslashreplace`, then decodes back.
pub fn terminal_safe(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if (ch as u32) < 0x80 {
            out.push(ch);
        } else {
            // Replicate Python's `backslashreplace` codec.
            let cp = ch as u32;
            if cp <= 0xFF {
                out.push_str(&format!("\\x{cp:02x}"));
            } else if cp <= 0xFFFF {
                out.push_str(&format!("\\u{cp:04x}"));
            } else {
                out.push_str(&format!("\\U{cp:08x}"));
            }
        }
    }
    out
}

/// Mirror upstream's `terminal_supports_colour` env logic. The
/// `isatty` check upstream does on `sys.stdout` is **not** performed
/// here — callers that need that can do it on the Python side; the
/// Rust port answers purely from the environment so it is
/// deterministic in tests.
pub fn terminal_supports_colour_from_env<F: Fn(&str) -> Option<String>>(getenv: F) -> bool {
    if getenv("NO_COLOUR").is_some() || getenv("NO_COLOR").is_some() {
        return false;
    }
    if getenv("FORCE_COLOUR").is_some() || getenv("FORCE_COLOR").is_some() {
        return true;
    }
    if let Some(v) = getenv("CI") {
        let v = v.to_lowercase();
        if v == "true" || v == "1" {
            return true;
        }
    }
    let term = getenv("TERM").unwrap_or_else(|| "unknown".to_string());
    !matches!(term.to_lowercase().as_str(), "dumb" | "unknown")
}

// ---- PyO3 surface -----------------------------------------------

#[pyfunction(name = "disable_colour")]
fn py_disable_colour() {
    *COLOURING_DISABLED.lock().unwrap() = true;
}

#[pyfunction(name = "enable_colour")]
fn py_enable_colour() {
    *COLOURING_DISABLED.lock().unwrap() = false;
}

#[pyfunction(name = "colourise")]
fn py_colourise(colour_name: &str, text: &str) -> PyResult<String> {
    colourise(colour_name, text).map_err(PyValueError::new_err)
}

#[pyfunction(name = "strip_escape_sequences")]
fn py_strip_escape_sequences(text: &str) -> String {
    strip_escape_sequences(text)
}

#[pyfunction(name = "terminal_safe")]
fn py_terminal_safe(s: &str) -> String {
    terminal_safe(s)
}

#[pyfunction(name = "colour_names")]
fn py_colour_names() -> Vec<&'static str> {
    COLOUR_TABLE.iter().map(|(n, _)| *n).collect()
}

#[pyfunction(name = "colour_escape_code")]
fn py_colour_escape_code(name: &str) -> Option<&'static str> {
    escape_code_for(name)
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_disable_colour, m)?)?;
    m.add_function(wrap_pyfunction!(py_enable_colour, m)?)?;
    m.add_function(wrap_pyfunction!(py_colourise, m)?)?;
    m.add_function(wrap_pyfunction!(py_strip_escape_sequences, m)?)?;
    m.add_function(wrap_pyfunction!(py_terminal_safe, m)?)?;
    m.add_function(wrap_pyfunction!(py_colour_names, m)?)?;
    m.add_function(wrap_pyfunction!(py_colour_escape_code, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_matches_upstream_shape() {
        // Reset state in case other tests toggled it.
        *COLOURING_DISABLED.lock().unwrap() = false;
        assert_eq!(wrap("91", "x"), "\x1b[91mx\x1b[39;49;00m");
    }

    #[test]
    fn strip_strips_colours_and_el() {
        let input = "\x1b[91mred\x1b[39;49;00m and \x1b[2Kerased";
        assert_eq!(strip_escape_sequences(input), "red and erased");
    }

    #[test]
    fn terminal_safe_replaces_non_ascii() {
        assert_eq!(terminal_safe("café"), "caf\\xe9");
    }

    #[test]
    fn env_logic_no_colour() {
        let env = |k: &str| {
            if k == "NO_COLOR" {
                Some("1".to_string())
            } else {
                None
            }
        };
        assert!(!terminal_supports_colour_from_env(env));
    }

    #[test]
    fn env_logic_force_colour() {
        let env = |k: &str| (k == "FORCE_COLOR").then(|| "1".to_string());
        assert!(terminal_supports_colour_from_env(env));
    }

    #[test]
    fn env_logic_dumb_term() {
        let env = |k: &str| (k == "TERM").then(|| "dumb".to_string());
        assert!(!terminal_supports_colour_from_env(env));
    }

    #[test]
    fn colourise_unknown_errors() {
        *COLOURING_DISABLED.lock().unwrap() = false;
        assert!(colourise("not_a_colour", "x").is_err());
    }
}
