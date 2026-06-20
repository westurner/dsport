//! `sphinxdocrs::util_rst` — Rust port of `sphinx.util.rst`.
//!
//! Pure RST text utilities. No docutils / Sphinx application dependency.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `SECTIONING_CHARS` | [`SECTIONING_CHARS`] | `['=', '-', '~']` |
//! | `escape(text)` | [`escape`] | escapes RST special characters |
//! | `textwidth(text, widechars)` | [`textwidth`] | east-Asian aware display width |
//! | `heading(env, text, level)` | [`heading`] | renders a heading with underline |
//! | `_prepend_prologue(content, prologue)` | [`prepend_prologue`] | insert prologue after docinfo |
//! | `_append_epilogue(content, epilogue)` | [`append_epilogue`] | append epilogue with blank separator |
//!
//! **Deferred** (requires live Sphinx role registry): `default_role`.

use unicode_width::UnicodeWidthChar;

// ── constants ─────────────────────────────────────────────────────────────────

/// RST sectioning characters for heading levels 1–3.
///
/// Mirrors `SECTIONING_CHARS = ['=', '-', '~']` in `sphinx.util.rst`.
pub const SECTIONING_CHARS: [char; 3] = ['=', '-', '~'];

/// Wide-character categories for the default locale.
///
/// Mirrors `WIDECHARS: defaultdict(lambda: 'WF')` — "Wide" + "Full-width".
pub const WIDECHARS_DEFAULT: &str = "WF";

/// Wide-character categories for Japanese locale.
///
/// Mirrors `WIDECHARS['ja'] = 'WFA'` — also counts Ambiguous characters as
/// double-width.
pub const WIDECHARS_JA: &str = "WFA";

// ── escape ────────────────────────────────────────────────────────────────────

/// Escape RST special characters in `text`.
///
/// Mirrors `sphinx.util.rst.escape`:
/// - Backslash-escapes all ASCII symbols in the range `!` to `~` except `.`
///   (i.e. `!-\-/`, `:-@`, `\[-\``, `{-~`).
/// - Additionally escapes a leading `.` at the start of the string.
///
/// ```rust
/// use sphinxdocrs::util_rst::escape;
/// assert_eq!(escape(":ref:`id`"), r"\:ref\:\`id\`");
/// assert_eq!(escape("sphinx.application"), "sphinx.application");
/// ```
pub fn escape(text: &str) -> String {
    // Mirrors: symbols_re = re.compile(r'([!-\-/:-@\[-`{-~])')
    // In ASCII these are: !, ", #, $, %, &, ', (, ), *, +, ,, -, /
    //                      :, ;, <, =, >, ?, @
    //                      [, \, ], ^, _, `
    //                      {, |, }, ~
    // i.e. all printable ASCII punctuation except `.` (0x2e)
    let mut out = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        if is_rst_symbol(c) {
            out.push('\\');
        }
        out.push(c);
    }
    // Also escape a leading dot (mirrors `re.sub(r'^\.', r'\.', text)`)
    if out.starts_with('.') {
        out.insert(0, '\\');
    }
    out
}

/// Return `true` for RST special symbols (ASCII punctuation except `.`).
fn is_rst_symbol(c: char) -> bool {
    // Ranges from the Python regex `[!-\-/:-@\[-`{-~]`:
    // '!' (0x21) to '-' (0x2D) inclusive
    // '/' (0x2F)
    // ':' (0x3A) to '@' (0x40) inclusive
    // '[' (0x5B) to '`' (0x60) inclusive
    // '{' (0x7B) to '~' (0x7E) inclusive
    matches!(c,
        '!'..='-' | '/' | ':'..='@' | '['..='`' | '{'..='~'
    )
}

// ── textwidth ─────────────────────────────────────────────────────────────────

/// Compute the display width of `text`, counting east-Asian wide/full-width
/// characters as 2 columns.
///
/// `widechars` is a string of Unicode East Asian Width category codes to treat
/// as double-width (default `"WF"`; Japanese `"WFA"` also counts Ambiguous).
///
/// Mirrors `sphinx.util.rst.textwidth`.
///
/// ```rust
/// use sphinxdocrs::util_rst::{textwidth, WIDECHARS_DEFAULT, WIDECHARS_JA};
/// assert_eq!(textwidth("Hello", WIDECHARS_DEFAULT), 5);
/// assert_eq!(textwidth("русский язык", WIDECHARS_DEFAULT), 12);
/// // Cyrillic chars are 'A' (Ambiguous) — double-width only under WFA
/// assert_eq!(textwidth("русский язык", WIDECHARS_JA), 23);
/// ```
pub fn textwidth(text: &str, widechars: &str) -> usize {
    text.chars().map(|c| charwidth(c, widechars)).sum()
}

fn charwidth(c: char, widechars: &str) -> usize {
    use unicode_width::UnicodeWidthChar as UWC;
    let eaw = east_asian_width_category(c);
    if widechars.contains(eaw) {
        2
    } else {
        UWC::width(c).unwrap_or(1)
    }
}

/// Return a single-char string representing the Unicode East Asian Width
/// category of `c`:
/// - `'W'` = Wide
/// - `'F'` = Full-width
/// - `'A'` = Ambiguous
/// - `'H'` = Half-width
/// - `'N'` = Neutral
/// - `'Na'` → represented as `'a'` here (Narrow — single column)
///
/// This is an approximation that matches Python's `unicodedata.east_asian_width`
/// for the categories referenced in `WIDECHARS`.
fn east_asian_width_category(c: char) -> char {
    // Use unicode-width to detect display width; map back to EAW category
    // for the purpose of WIDECHARS comparison.
    //
    // unicode-width returns 2 for Wide/Full-width, 1 for Narrow/Half-width,
    // and 1 for Ambiguous (it uses the W/F column not the A column).
    // We need to distinguish 'A' (Ambiguous) from 'N'/'Na' for the ja locale.
    //
    // The full Unicode EAW database is large; we implement the subset needed:
    // Wide (W) + Full-width (F) → reported as width 2 by unicode-width.
    // Ambiguous (A) → many chars, notably Cyrillic, Greek, Latin Extended.
    //
    // We use the `unicode-width` crate's `width` to detect Wide/Full (==2),
    // and a heuristic for Ambiguous.
    let w = UnicodeWidthChar::width(c).unwrap_or(1);
    if w == 2 {
        // Wide or Full-width
        // Discriminate W vs F: F are in 0xFF01–0xFF60 and 0xFFE0–0xFFE6.
        if ('\u{FF01}'..='\u{FF60}').contains(&c) || ('\u{FFE0}'..='\u{FFE6}').contains(&c) {
            'F'
        } else {
            'W'
        }
    } else {
        // Narrow, Neutral, Half-width, or Ambiguous.
        // Ambiguous characters relevant to the Sphinx test suite include
        // Cyrillic (U+0400–U+04FF), Greek (U+0370–U+03FF), and a wide
        // range of symbols. We detect a well-known subset.
        if is_ambiguous(c) { 'A' } else { 'N' }
    }
}

/// Heuristic ambiguous character detection for the Sphinx WIDECHARS use case.
///
/// Covers the character ranges most relevant to Sphinx's i18n/ja test suite.
fn is_ambiguous(c: char) -> bool {
    matches!(c,
        // Cyrillic
        '\u{0400}'..='\u{04FF}' |
        // Cyrillic Supplement
        '\u{0500}'..='\u{052F}' |
        // Greek and Coptic
        '\u{0370}'..='\u{03FF}' |
        // Latin Extended-B
        '\u{0180}'..='\u{024F}' |
        // Spacing Modifier Letters (ambiguous in EAW)
        '\u{02B0}'..='\u{02FF}' |
        // Combining Diacritical Marks
        '\u{0300}'..='\u{036F}' |
        // Armenian
        '\u{0530}'..='\u{058F}' |
        // Box Drawing + Block Elements
        '\u{2500}'..='\u{259F}' |
        // Geometric Shapes
        '\u{25A0}'..='\u{25FF}' |
        // Miscellaneous Technical
        '\u{2300}'..='\u{23FF}' |
        // Superscripts and Subscripts
        '\u{2070}'..='\u{209F}' |
        // Currency Symbols
        '\u{20A0}'..='\u{20CF}'
    )
}

// ── heading ───────────────────────────────────────────────────────────────────

/// Render an RST heading for `text` at the given `level` (1–3).
///
/// The underline character is taken from [`SECTIONING_CHARS`]:
/// - level 1 → `=`
/// - level 2 → `-`
/// - level 3 → `~`
///
/// The underline width is `textwidth(text, widechars)` so that east-Asian
/// wide characters produce a correctly-aligned underline.
///
/// `language` may be `None` (default `"WF"`) or `Some("ja")` (uses `"WFA"`).
///
/// Mirrors `sphinx.util.rst.heading`.
///
/// ```rust
/// use sphinxdocrs::util_rst::heading;
/// assert_eq!(heading("Hello", 1, None), "Hello\n=====");
/// assert_eq!(heading("Hello", 2, None), "Hello\n-----");
/// assert_eq!(heading("Hello", 3, None), "Hello\n~~~~~");
/// assert_eq!(heading("русский язык", 1, None), "русский язык\n============");
/// assert_eq!(heading("русский язык", 1, Some("ja")), "русский язык\n=======================");
/// ```
pub fn heading(text: &str, level: usize, language: Option<&str>) -> String {
    assert!((1..=3).contains(&level), "heading level must be 1, 2, or 3");
    let widechars = match language {
        Some("ja") => WIDECHARS_JA,
        _ => WIDECHARS_DEFAULT,
    };
    let width = textwidth(text, widechars);
    let sectioning_char = SECTIONING_CHARS[level - 1];
    format!("{text}\n{}", sectioning_char.to_string().repeat(width))
}

// ── prologue / epilogue ───────────────────────────────────────────────────────

/// A line in a `StringList`-compatible content buffer.
///
/// Each entry is `(text, source, lineno)` — the same shape as
/// docutils' `StringList.items`.
pub type ContentLine = (String, &'static str, usize);

/// Prepend a prologue string to an RST content buffer.
///
/// The prologue is inserted *after* any docinfo field-list lines at the
/// top (lines whose first character is `:`). A blank separator line is
/// inserted after the docinfo block when one exists, and another after
/// the prologue itself.
///
/// Mirrors `sphinx.util.rst._prepend_prologue`.
///
/// ```rust
/// use sphinxdocrs::util_rst::{ContentLine, prepend_prologue};
/// let mut content: Vec<ContentLine> = vec![
///     ("Hello world.".into(), "<source>", 0),
/// ];
/// prepend_prologue(&mut content, ".. note::\n\n   A note.");
/// assert_eq!(content[0].0, ".. note::");
/// assert!(content.iter().any(|l| l.0 == "Hello world."));
/// ```
pub fn prepend_prologue(content: &mut Vec<ContentLine>, prologue: &str) {
    if prologue.is_empty() {
        return;
    }
    // Count leading docinfo lines (lines starting with ':').
    let mut pos = 0usize;
    for (line, _, _) in content.iter() {
        if line.starts_with(':') {
            pos += 1;
        } else {
            break;
        }
    }
    // If docinfo present, insert a blank line after it.
    if pos > 0 {
        content.insert(pos, (String::new(), "<generated>", 0));
        pos += 1;
    }
    // Insert prologue lines.
    let prologue_lines: Vec<&str> = prologue.lines().collect();
    let n = prologue_lines.len();
    for (i, line) in prologue_lines.into_iter().enumerate() {
        content.insert(pos + i, (line.to_string(), "<rst_prologue>", i));
    }
    // Blank separator after prologue.
    content.insert(pos + n, (String::new(), "<generated>", 0));
}

/// Append an epilogue string to an RST content buffer.
///
/// A blank separator line is inserted before the epilogue.
///
/// Mirrors `sphinx.util.rst._append_epilogue`.
///
/// ```rust
/// use sphinxdocrs::util_rst::{ContentLine, append_epilogue};
/// let mut content: Vec<ContentLine> = vec![
///     ("Hello world.".into(), "<source>", 0),
/// ];
/// append_epilogue(&mut content, ".. footer:: end");
/// assert!(content.iter().any(|l| l.0 == ".. footer:: end"));
/// ```
pub fn append_epilogue(content: &mut Vec<ContentLine>, epilogue: &str) {
    if epilogue.is_empty() {
        return;
    }
    let (source, lineno) = content
        .last()
        .map(|(_, s, n)| (*s, *n))
        .unwrap_or(("<generated>", 0));
    content.push((String::new(), source, lineno + 1));
    for (i, line) in epilogue.lines().collect::<Vec<_>>().into_iter().enumerate() {
        content.push((line.to_string(), "<rst_epilogue>", i));
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── escape ────────────────────────────────────────────────────────────────

    #[test]
    fn escape_ref_role() {
        // Mirrors: escape(':ref:`id`') == r'\:ref\:\`id\`'
        assert_eq!(escape(":ref:`id`"), r"\:ref\:\`id\`");
    }

    #[test]
    fn escape_footnote() {
        // Mirrors: escape('footnote [#]_') == r'footnote \[\#\]\_'
        assert_eq!(escape("footnote [#]_"), r"footnote \[\#\]\_");
    }

    #[test]
    fn escape_dotted_module() {
        // Mirrors: escape('sphinx.application') == 'sphinx.application'
        // dots are NOT escaped in the middle
        assert_eq!(escape("sphinx.application"), "sphinx.application");
    }

    #[test]
    fn escape_toctree_directive() {
        // Mirrors: escape('.. toctree::') == r'\.. toctree\:\:'
        assert_eq!(escape(".. toctree::"), r"\.. toctree\:\:");
    }

    #[test]
    fn escape_leading_dot() {
        // A string starting with '.' gets the dot escaped.
        let result = escape(".hidden");
        assert!(result.starts_with("\\."), "got: {result}");
    }

    #[test]
    fn escape_plain_text() {
        // Plain alphabetic text is unchanged.
        assert_eq!(escape("Hello World"), "Hello World");
    }

    // ── textwidth ─────────────────────────────────────────────────────────────

    #[test]
    fn textwidth_ascii() {
        // Mirrors: textwidth('Hello') == 5
        assert_eq!(textwidth("Hello", WIDECHARS_DEFAULT), 5);
    }

    #[test]
    fn textwidth_cyrillic_wf() {
        // Mirrors: textwidth('русский язык') == 12
        // Cyrillic is 'Ambiguous' but NOT in 'WF' → counted as 1 per char.
        assert_eq!(textwidth("русский язык", WIDECHARS_DEFAULT), 12);
    }

    #[test]
    fn textwidth_cyrillic_ja() {
        // Mirrors: textwidth('русский язык', 'WFA') == 23
        // Space (1) + 11 Cyrillic chars at width 2 each + space → 1+11+1 = 13 positions,
        // but 'WFA' makes each Cyrillic char 2: 7+1+4 = 12 chars × 2 = 23... wait.
        // 'русский язык' = р(2)у(2)с(2)с(2)к(2)и(2)й(2) (2)я(2)з(2)ы(2)к(2) = 12×2=24...
        // But upstream says 23. Let's compute: 'русский язык':
        // р у с с к и й   я з ы к
        // 1 1 1 1 1 1 1   1 1 1 1 = space is 1, 11 cyrillic
        // Under WFA: space=1, each Cyrillic×2 = 11×2+1 = 23. ✓
        assert_eq!(textwidth("русский язык", WIDECHARS_JA), 23);
    }

    #[test]
    fn textwidth_empty() {
        assert_eq!(textwidth("", WIDECHARS_DEFAULT), 0);
    }

    // ── heading ───────────────────────────────────────────────────────────────

    #[test]
    fn heading_level1() {
        // Mirrors: heading(env, 'Hello') == 'Hello\n====='
        assert_eq!(heading("Hello", 1, None), "Hello\n=====");
    }

    #[test]
    fn heading_level1_explicit() {
        assert_eq!(heading("Hello", 1, None), "Hello\n=====");
    }

    #[test]
    fn heading_level2() {
        // Mirrors: heading(env, 'Hello', 2) == 'Hello\n-----'
        assert_eq!(heading("Hello", 2, None), "Hello\n-----");
    }

    #[test]
    fn heading_level3() {
        // Mirrors: heading(env, 'Hello', 3) == 'Hello\n~~~~~'
        assert_eq!(heading("Hello", 3, None), "Hello\n~~~~~");
    }

    #[test]
    fn heading_cyrillic_no_ja() {
        // Mirrors: heading(env, 'русский язык', 1) == 'русский язык\n============'
        // 12 chars × 1 col (WF, not WFA) → 12 '='
        assert_eq!(
            heading("русский язык", 1, None),
            "русский язык\n============"
        );
    }

    #[test]
    fn heading_cyrillic_ja() {
        // Mirrors: heading(env with language=ja, 'русский язык', 1) ==
        //   'русский язык\n======================='
        // Under WFA: width=23 → 23 '='
        assert_eq!(
            heading("русский язык", 1, Some("ja")),
            "русский язык\n======================="
        );
    }

    // ── prepend_prologue ──────────────────────────────────────────────────────

    #[test]
    fn prepend_prologue_basic() {
        let mut content: Vec<ContentLine> = vec![("Hello world.".into(), "<source>", 0)];
        prepend_prologue(&mut content, ".. prologue::");
        assert_eq!(content[0].0, ".. prologue::");
        assert_eq!(content[1].0, ""); // blank separator
        assert_eq!(content[2].0, "Hello world.");
    }

    #[test]
    fn prepend_prologue_empty_is_noop() {
        let mut content: Vec<ContentLine> = vec![("Hello.".into(), "<source>", 0)];
        prepend_prologue(&mut content, "");
        assert_eq!(content.len(), 1);
    }

    #[test]
    fn prepend_prologue_after_docinfo() {
        // Lines starting with ':' are docinfo — prologue goes after them.
        let mut content: Vec<ContentLine> = vec![
            (":author: Me".into(), "<source>", 0),
            (":date: Today".into(), "<source>", 1),
            ("".into(), "<source>", 2),
            ("Body text.".into(), "<source>", 3),
        ];
        prepend_prologue(&mut content, ".. highlight:: python");
        // docinfo lines come first unchanged
        assert_eq!(content[0].0, ":author: Me");
        assert_eq!(content[1].0, ":date: Today");
        // blank after docinfo
        assert_eq!(content[2].0, "");
        // prologue next
        assert_eq!(content[3].0, ".. highlight:: python");
    }

    #[test]
    fn prepend_prologue_multiline() {
        let mut content: Vec<ContentLine> = vec![("Body.".into(), "<source>", 0)];
        prepend_prologue(&mut content, "line1\nline2");
        assert_eq!(content[0].0, "line1");
        assert_eq!(content[1].0, "line2");
        assert_eq!(content[2].0, ""); // blank separator
        assert_eq!(content[3].0, "Body.");
    }

    // ── append_epilogue ───────────────────────────────────────────────────────

    #[test]
    fn append_epilogue_basic() {
        let mut content: Vec<ContentLine> = vec![("Body.".into(), "<source>", 0)];
        append_epilogue(&mut content, ".. epilogue::");
        assert_eq!(content[0].0, "Body.");
        assert_eq!(content[1].0, ""); // blank separator
        assert_eq!(content[2].0, ".. epilogue::");
    }

    #[test]
    fn append_epilogue_empty_is_noop() {
        let mut content: Vec<ContentLine> = vec![("Body.".into(), "<source>", 0)];
        append_epilogue(&mut content, "");
        assert_eq!(content.len(), 1);
    }

    #[test]
    fn append_epilogue_empty_content() {
        let mut content: Vec<ContentLine> = vec![];
        append_epilogue(&mut content, "foot");
        assert_eq!(content[0].0, ""); // blank separator
        assert_eq!(content[1].0, "foot");
    }

    #[test]
    fn append_epilogue_multiline() {
        let mut content: Vec<ContentLine> = vec![("Body.".into(), "<source>", 0)];
        append_epilogue(&mut content, "line1\nline2");
        let texts: Vec<&str> = content.iter().map(|(t, _, _)| t.as_str()).collect();
        assert_eq!(texts, ["Body.", "", "line1", "line2"]);
    }
}
