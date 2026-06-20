//! Django-specific template filters.
//!
//! Ports the Django built-in template filter library to Rust.
//!
//! Reference: <https://docs.djangoproject.com/en/stable/ref/templates/builtins/#built-in-filter-reference>
//!
//! ## Phase 1 — Core string, numeric, list and boolean filters
//!
//! | Filter | Django equiv | Notes |
//! |--------|-------------|-------|
//! | `upper` | `upper` | |
//! | `lower` | `lower` | |
//! | `capfirst` | `capfirst` | Capitalise first char only |
//! | `title` | `title` | Title-case each word |
//! | `slugify` | `slugify` | ASCII slug, hyphens |
//! | `truncatewords` | `truncatewords` | `\| truncatewords:3` |
//! | `truncatechars` | `truncatechars` | `\| truncatechars:20` |
//! | `wordcount` | `wordcount` | |
//! | `wordwrap` | `wordwrap` | `\| wordwrap:79` |
//! | `add` | `add` | Numeric add |
//! | `floatformat` | `floatformat` | `\| floatformat:2` |
//! | `pluralize` | `pluralize` | `\| pluralize` / `\| pluralize:"y,ies"` |
//! | `first` | `first` | First item in list |
//! | `last` | `last` | Last item in list |
//! | `join` | `join` | `\| join:", "` |
//! | `length` | `length` | Len of list or string |
//! | `length_is` | `length_is` | `\| length_is:3` → bool |
//! | `yesno` | `yesno` | `\| yesno:"yes,no,maybe"` |
//! | `default` | `default` | Fallback for falsy values |
//! | `default_if_none` | `default_if_none` | Fallback only for `None` |
//! | `escape` | `escape` | HTML-escape |
//! | `force_escape` | `force_escape` | Always escape |
//! | `safe` | `safe` | Mark as safe (no escape) |
//! | `linebreaks` | `linebreaks` | `\n\n` → `<p>`, `\n` → `<br>` |
//! | `linebreaksbr` | `linebreaksbr` | `\n` → `<br>` |
//! | `striptags` | `striptags` | Strip HTML tags |
//! | `urlencode` | `urlencode` | Percent-encode |

use minijinja::Value;

// ─── String filters ───────────────────────────────────────────────────────────

/// `{{ value|upper }}` — Convert to uppercase.
pub fn upper(value: Value) -> String {
    value.to_string().to_uppercase()
}

/// `{{ value|lower }}` — Convert to lowercase.
pub fn lower(value: Value) -> String {
    value.to_string().to_lowercase()
}

/// `{{ value|capfirst }}` — Capitalise the first character, leave the rest.
pub fn capfirst(value: Value) -> String {
    let s = value.to_string();
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let upper: String = first.to_uppercase().collect();
            upper + chars.as_str()
        }
    }
}

/// `{{ value|title }}` — Title-case every word.
pub fn title(value: Value) -> String {
    let s = value.to_string();
    let mut out = String::with_capacity(s.len());
    let mut capitalise_next = true;
    for c in s.chars() {
        if c.is_whitespace() || c == '-' || c == '_' {
            out.push(c);
            capitalise_next = true;
        } else if capitalise_next {
            for upper in c.to_uppercase() {
                out.push(upper);
            }
            capitalise_next = false;
        } else {
            for lower in c.to_lowercase() {
                out.push(lower);
            }
        }
    }
    out
}

/// `{{ value|slugify }}` — Convert to a URL-friendly ASCII slug.
///
/// - Lowercases
/// - Removes non-alphanumeric chars (except hyphens)
/// - Replaces spaces with hyphens
/// - Strips leading / trailing hyphens
/// - Collapses repeated hyphens
///
/// ```
/// # use jinja2rs::filters::django::slugify;
/// # use minijinja::Value;
/// assert_eq!(slugify(Value::from("Hello, World!")), "hello-world");
/// assert_eq!(slugify(Value::from("  C++ rocks  ")), "c-rocks");
/// ```
pub fn slugify(value: Value) -> String {
    let s = value.to_string().to_lowercase();
    let mut slug = String::with_capacity(s.len());
    let mut last_was_hyphen = true; // leading hyphens are trimmed
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c);
            last_was_hyphen = false;
        } else if c == '-' || c.is_whitespace() || c == '_' {
            if !last_was_hyphen {
                slug.push('-');
                last_was_hyphen = true;
            }
        }
        // all other chars are dropped
    }
    // Trim trailing hyphen
    if slug.ends_with('-') {
        slug.pop();
    }
    slug
}

/// `{{ value|truncatewords:N }}` — Truncate to N words, appending `"..."`.
///
/// Returns the original string unchanged when it has N or fewer words.
pub fn truncatewords(value: Value, num: i64) -> String {
    let s = value.to_string();
    let n = num.max(0) as usize;
    if n == 0 {
        return String::new();
    }
    let mut words = s.split_whitespace();
    let taken: Vec<&str> = words.by_ref().take(n).collect();
    if words.next().is_none() {
        // Fewer than n words — return as-is.
        taken.join(" ")
    } else {
        format!("{}\u{2026}", taken.join(" ")) // U+2026 HORIZONTAL ELLIPSIS
    }
}

/// `{{ value|truncatechars:N }}` — Truncate to N characters, appending `"…"`.
///
/// The trailing ellipsis counts toward the character limit, so
/// `truncatechars(value, 5)` returns at most 5 chars including the `"…"`.
pub fn truncatechars(value: Value, num: i64) -> String {
    let s = value.to_string();
    let n = num.max(0) as usize;
    if n == 0 {
        return String::new();
    }
    let char_count = s.chars().count();
    if char_count <= n {
        s
    } else {
        // Reserve 1 char for the ellipsis.
        let take = n.saturating_sub(1);
        let truncated: String = s.chars().take(take).collect();
        format!("{}\u{2026}", truncated)
    }
}

/// `{{ value|wordcount }}` — Return the number of words.
pub fn wordcount(value: Value) -> usize {
    value.to_string().split_whitespace().count()
}

/// `{{ value|wordwrap:N }}` — Wrap text at N characters per line.
pub fn wordwrap(value: Value, width: i64) -> String {
    let s = value.to_string();
    let w = (width.max(1)) as usize;
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    for word in s.split_whitespace() {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= w {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line.clone());
            current_line.clear();
            current_line.push_str(word);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines.join("\n")
}

// ─── Numeric filters ──────────────────────────────────────────────────────────

/// `{{ value|add:N }}` — Add an integer to the value.
///
/// Returns an error if the value is not numeric.
pub fn add(value: Value, arg: i64) -> Result<i64, minijinja::Error> {
    if let Some(v) = value.as_i64() {
        Ok(v.saturating_add(arg))
    } else if let Ok(v) = f64::try_from(value.clone()) {
        Ok((v + arg as f64) as i64)
    } else {
        Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("add: expected a number, got {:?}", value),
        ))
    }
}

/// `{{ value|floatformat }}` / `{{ value|floatformat:N }}` — Format a float.
///
/// - `floatformat` (no arg) → strip insignificant zeros, keep at least 1 decimal.
/// - `floatformat:N` (N > 0) → exactly N decimal places.
/// - `floatformat:-N` (N < 0) → N decimal places, but strip trailing zeros.
///
/// Returns an error if the value is not numeric.
pub fn floatformat(value: Value, digits: Option<i64>) -> Result<String, minijinja::Error> {
    let v: f64 = if let Some(i) = value.as_i64() {
        i as f64
    } else if let Ok(f) = f64::try_from(value.clone()) {
        f
    } else {
        return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("floatformat: expected a number, got {:?}", value),
        ));
    };

    let d = digits.unwrap_or(-1);

    if d < 0 {
        // Negative: N places, strip trailing zeros.
        let places = (-d) as usize;
        let formatted = format!("{:.prec$}", v, prec = places);
        // Strip trailing zeros after the decimal point.
        let trimmed = formatted.trim_end_matches('0');
        let trimmed = trimmed.trim_end_matches('.');
        Ok(trimmed.to_string())
    } else if d == 0 {
        Ok(format!("{:.0}", v))
    } else {
        Ok(format!("{:.prec$}", v, prec = d as usize))
    }
}

/// `{{ count|pluralize }}` / `{{ count|pluralize:"y,ies" }}` — Pluralise suffix.
///
/// - No argument: return `""` for count == 1, `"s"` otherwise.
/// - `"suffix"`: return `""` for count == 1, `suffix` otherwise.
/// - `"singular,plural"`: select suffix based on count.
pub fn pluralize(value: Value, suffix: Option<String>) -> String {
    let count = value.as_i64().unwrap_or(0).abs();
    match suffix {
        None => {
            // Default: empty singular, "s" plural
            if count == 1 {
                "".to_string()
            } else {
                "s".to_string()
            }
        }
        Some(suf) => {
            let parts: Vec<&str> = suf.splitn(2, ',').collect();
            if parts.len() == 1 {
                // Single suffix: empty singular, suffix plural
                if count == 1 {
                    "".to_string()
                } else {
                    parts[0].to_string()
                }
            } else {
                // Two-part: first=singular, second=plural
                if count == 1 {
                    parts[0].to_string()
                } else {
                    parts.get(1).copied().unwrap_or("").to_string()
                }
            }
        }
    }
}

// ─── List filters ─────────────────────────────────────────────────────────────

/// `{{ list|first }}` — Return the first item in the list.
pub fn first(value: Value) -> Value {
    if let Ok(mut iter) = value.try_iter() {
        iter.next().unwrap_or(Value::UNDEFINED)
    } else {
        Value::UNDEFINED
    }
}

/// `{{ list|last }}` — Return the last item in the list.
pub fn last(value: Value) -> Value {
    if let Ok(iter) = value.try_iter() {
        iter.last().unwrap_or(Value::UNDEFINED)
    } else {
        Value::UNDEFINED
    }
}

/// `{{ list|join:", " }}` — Join list items with a separator.
pub fn join(value: Value, separator: Option<String>) -> String {
    let sep = separator.unwrap_or_else(|| "".to_string());
    if let Ok(iter) = value.try_iter() {
        iter.map(|v| v.to_string()).collect::<Vec<_>>().join(&sep)
    } else {
        value.to_string()
    }
}

/// `{{ value|length }}` — Return the length of a list, string, or mapping.
pub fn length(value: Value) -> usize {
    if let Some(s) = value.as_str() {
        s.chars().count()
    } else if let Ok(iter) = value.try_iter() {
        iter.count()
    } else {
        value.to_string().chars().count()
    }
}

/// `{{ value|length_is:N }}` — Return `true` if the length equals N.
pub fn length_is(value: Value, n: i64) -> bool {
    length(value) == n.max(0) as usize
}

// ─── Boolean / fallback filters ───────────────────────────────────────────────

/// `{{ value|yesno }}` / `{{ value|yesno:"yes,no,maybe" }}` — Map to yes/no string.
///
/// - Truthy → first token
/// - Falsy (but not None/undefined) → second token
/// - None / undefined → third token (if provided), else second token
pub fn yesno(value: Value, mapping: Option<String>) -> String {
    let m = mapping.unwrap_or_else(|| "yes,no,maybe".to_string());
    let tokens: Vec<&str> = m.split(',').collect();
    let yes = tokens.first().copied().unwrap_or("yes");
    let no = tokens.get(1).copied().unwrap_or("no");
    let maybe = tokens.get(2).copied().unwrap_or(no);

    if value.is_undefined() || value.is_none() {
        maybe.to_string()
    } else if value.is_true() {
        yes.to_string()
    } else {
        no.to_string()
    }
}

/// `{{ value|default:"fallback" }}` — Return fallback for any falsy value.
pub fn default(value: Value, fallback: String) -> String {
    if value.is_undefined() || value.is_none() || !value.is_true() {
        fallback
    } else {
        value.to_string()
    }
}

/// `{{ value|default_if_none:"fallback" }}` — Return fallback only for `None`.
pub fn default_if_none(value: Value, fallback: String) -> String {
    if value.is_undefined() || value.is_none() {
        fallback
    } else {
        value.to_string()
    }
}

// ─── HTML / escaping filters ──────────────────────────────────────────────────

/// `{{ value|escape }}` — HTML-escape the value.
///
/// Returns a safe HTML string.  Wraps the output in minijinja's safe-string
/// mechanism so the auto-escaper will not double-escape it.
pub fn escape(value: Value) -> minijinja::Value {
    let s = value.to_string();
    let escaped = html_escape_str(&s);
    minijinja::Value::from_safe_string(escaped)
}

/// `{{ value|force_escape }}` — Always HTML-escape, even if already safe.
pub fn force_escape(value: Value) -> minijinja::Value {
    let s = value.to_string();
    let escaped = html_escape_str(&s);
    minijinja::Value::from_safe_string(escaped)
}

/// `{{ value|safe }}` — Mark value as safe HTML (bypass auto-escaping).
pub fn safe(value: Value) -> minijinja::Value {
    minijinja::Value::from_safe_string(value.to_string())
}

/// `{{ value|striptags }}` — Remove all HTML/XML tags from the value.
///
/// This is a simple state-machine parser; it does not validate HTML.
pub fn striptags(value: Value) -> String {
    let s = value.to_string();
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

/// `{{ value|linebreaks }}` — Replace newlines with `<p>` and `<br>` tags.
///
/// - A blank line (`\n\n`) starts a new `<p>`.
/// - A single `\n` becomes a `<br>`.
pub fn linebreaks(value: Value) -> minijinja::Value {
    let s = html_escape_str(&value.to_string());
    let paragraphs: Vec<&str> = s.split("\n\n").collect();
    let html: Vec<String> = paragraphs
        .iter()
        .map(|p| format!("<p>{}</p>", p.replace('\n', "<br>")))
        .collect();
    minijinja::Value::from_safe_string(html.join("\n\n"))
}

/// `{{ value|linebreaksbr }}` — Replace each `\n` with a `<br>`.
pub fn linebreaksbr(value: Value) -> minijinja::Value {
    let s = html_escape_str(&value.to_string());
    minijinja::Value::from_safe_string(s.replace('\n', "<br>"))
}

/// `{{ value|urlencode }}` — Percent-encode the value for safe URL inclusion.
///
/// Slashes are preserved (mirrors Django behaviour; use `urlencode_strict` to
/// also encode slashes).
pub fn urlencode(value: Value) -> String {
    let s = value.to_string();
    // Encode everything except unreserved chars and '/'
    let mut out = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~' | '/') {
            out.push(c);
        } else {
            for byte in c.to_string().as_bytes() {
                out.push('%');
                out.push_str(&format!("{:02X}", byte));
            }
        }
    }
    out
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Escape the five HTML special characters.
pub(crate) fn html_escape_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&#34;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── String filters ────────────────────────────────────────────────────────

    #[test]
    fn test_upper() {
        assert_eq!(upper(Value::from("hello")), "HELLO");
    }

    #[test]
    fn test_lower() {
        assert_eq!(lower(Value::from("HELLO")), "hello");
    }

    #[test]
    fn test_capfirst() {
        assert_eq!(capfirst(Value::from("hello world")), "Hello world");
        assert_eq!(capfirst(Value::from("")), "");
    }

    #[test]
    fn test_title() {
        assert_eq!(title(Value::from("hello world")), "Hello World");
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify(Value::from("Hello, World!")), "hello-world");
        assert_eq!(slugify(Value::from("  C++ rocks  ")), "c-rocks");
        assert_eq!(slugify(Value::from("my blog post")), "my-blog-post");
        assert_eq!(slugify(Value::from("   ")), "");
    }

    #[test]
    fn test_truncatewords() {
        assert_eq!(
            truncatewords(Value::from("one two three four"), 2),
            "one two\u{2026}"
        );
        assert_eq!(truncatewords(Value::from("short"), 5), "short");
        assert_eq!(truncatewords(Value::from("anything"), 0), "");
    }

    #[test]
    fn test_truncatechars() {
        assert_eq!(
            truncatechars(Value::from("Hello, World!"), 8),
            "Hello, \u{2026}"
        );
        assert_eq!(truncatechars(Value::from("short"), 10), "short");
        assert_eq!(truncatechars(Value::from("hi"), 0), "");
    }

    #[test]
    fn test_wordcount() {
        assert_eq!(wordcount(Value::from("one two three")), 3);
        assert_eq!(wordcount(Value::from("")), 0);
    }

    #[test]
    fn test_wordwrap() {
        let s = wordwrap(Value::from("The quick brown fox jumps"), 10);
        for line in s.lines() {
            assert!(line.len() <= 10, "line too long: {:?}", line);
        }
    }

    // ── Numeric filters ───────────────────────────────────────────────────────

    #[test]
    fn test_add() {
        assert_eq!(add(Value::from(5), 3).unwrap(), 8);
        assert_eq!(add(Value::from(-1), 1).unwrap(), 0);
        assert!(add(Value::from("not a number"), 1).is_err());
    }

    #[test]
    fn test_floatformat_default() {
        // default (-1): round to 1 decimal place, strip trailing zeros
        // Mirrors Django: {{ 3.14159|floatformat }} → "3.1"
        assert_eq!(floatformat(Value::from(3.14159_f64), None).unwrap(), "3.1");
        assert_eq!(floatformat(Value::from(3.0_f64), None).unwrap(), "3");
        assert_eq!(floatformat(Value::from(3.10_f64), None).unwrap(), "3.1");
    }

    #[test]
    fn test_floatformat_fixed() {
        assert_eq!(
            floatformat(Value::from(3.14159_f64), Some(2)).unwrap(),
            "3.14"
        );
    }

    #[test]
    fn test_floatformat_strip() {
        assert_eq!(
            floatformat(Value::from(3.14000_f64), Some(-2)).unwrap(),
            "3.14"
        );
        assert_eq!(floatformat(Value::from(3.00_f64), Some(-2)).unwrap(), "3");
    }

    #[test]
    fn test_pluralize_default() {
        assert_eq!(pluralize(Value::from(1), None), "");
        assert_eq!(pluralize(Value::from(2), None), "s");
        assert_eq!(pluralize(Value::from(0), None), "s");
    }

    #[test]
    fn test_pluralize_custom() {
        assert_eq!(pluralize(Value::from(1), Some("y,ies".into())), "y");
        assert_eq!(pluralize(Value::from(3), Some("y,ies".into())), "ies");
        // Single suffix: empty singular, suffix plural
        assert_eq!(pluralize(Value::from(1), Some("s".into())), "");
        assert_eq!(pluralize(Value::from(2), Some("s".into())), "s");
    }

    // ── List filters ──────────────────────────────────────────────────────────

    #[test]
    fn test_first() {
        let list = Value::from(vec![1i64, 2, 3]);
        assert_eq!(first(list), Value::from(1i64));
    }

    #[test]
    fn test_last() {
        let list = Value::from(vec![1i64, 2, 3]);
        assert_eq!(last(list), Value::from(3i64));
    }

    #[test]
    fn test_join() {
        let list = Value::from(vec!["a", "b", "c"]);
        assert_eq!(join(list, Some(", ".into())), "a, b, c");
    }

    #[test]
    fn test_length_list() {
        let list = Value::from(vec![1i64, 2, 3, 4]);
        assert_eq!(length(list), 4);
    }

    #[test]
    fn test_length_str() {
        assert_eq!(length(Value::from("hello")), 5);
    }

    #[test]
    fn test_length_is() {
        let list = Value::from(vec![1i64, 2, 3]);
        assert!(length_is(list, 3));
    }

    // ── Boolean filters ───────────────────────────────────────────────────────

    #[test]
    fn test_yesno() {
        assert_eq!(yesno(Value::from(true), None), "yes");
        assert_eq!(yesno(Value::from(false), None), "no");
        assert_eq!(yesno(Value::UNDEFINED, None), "maybe");
    }

    #[test]
    fn test_yesno_custom() {
        assert_eq!(yesno(Value::from(true), Some("on,off".into())), "on");
        assert_eq!(yesno(Value::from(false), Some("on,off".into())), "off");
    }

    #[test]
    fn test_default() {
        assert_eq!(default(Value::from(""), "fallback".into()), "fallback");
        assert_eq!(default(Value::from("hello"), "fallback".into()), "hello");
        assert_eq!(default(Value::UNDEFINED, "fallback".into()), "fallback");
    }

    #[test]
    fn test_default_if_none() {
        assert_eq!(
            default_if_none(Value::UNDEFINED, "fallback".into()),
            "fallback"
        );
        assert_eq!(default_if_none(Value::from(""), "fallback".into()), "");
    }

    // ── HTML filters ──────────────────────────────────────────────────────────

    #[test]
    fn test_striptags() {
        assert_eq!(striptags(Value::from("<b>bold</b>")), "bold");
        assert_eq!(striptags(Value::from("<p>One<br>Two</p>")), "OneTwo");
    }

    #[test]
    fn test_urlencode() {
        assert_eq!(urlencode(Value::from("hello world")), "hello%20world");
        assert_eq!(urlencode(Value::from("/path/to")), "/path/to");
        assert_eq!(urlencode(Value::from("a=1&b=2")), "a%3D1%26b%3D2");
    }

    #[test]
    fn test_html_escape_str() {
        assert_eq!(html_escape_str("< > & \" '"), "&lt; &gt; &amp; &#34; &#39;");
        assert_eq!(html_escape_str("safe text"), "safe text");
    }
}
