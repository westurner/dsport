//! `pygments.regexopt` port.
//!
//! Generates the same optimized alternation regex that
//! `pygments.lexer.words(...)` produces at class-build time, so a
//! transpiled `words(...)` rule compiles to a **byte-identical** pattern
//! string (and therefore byte-identical match behavior) versus upstream.
//!
//! Faithful line-by-line port of `src/pygments/pygments/regexopt.py`,
//! including Python's `re.escape` (3.7+) semantics, which the
//! `regex`-crate `escape` does not match.

/// Characters escaped by Python 3.7+ `re.escape`.
/// Mirrors `re._special_chars_map` =
/// `b'()[]{}?*+-|^$\\.&~# \t\n\r\v\f'`.
const RE_SPECIAL: &str = "()[]{}?*+-|^$\\.&~# \t\n\r\u{0b}\u{0c}";

/// Port of Python's `re.escape` (3.7+ "escape only special chars").
fn re_escape(s: &[char]) -> String {
    let mut out = String::with_capacity(s.len());
    for &c in s {
        if RE_SPECIAL.contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Port of `make_charset`: wrap letters in `[...]`, escaping the
/// charset-significant chars `[ ^ \ - ]` (mirrors `CS_ESCAPE`).
fn make_charset(letters: &[char]) -> String {
    let mut inner = String::with_capacity(letters.len());
    for &c in letters {
        if matches!(c, '[' | '^' | '\\' | '-' | ']') {
            inner.push('\\');
        }
        inner.push(c);
    }
    format!("[{inner}]")
}

/// Port of `commonprefix`: longest common leading substring of an
/// iterable of strings, comparing `min` and `max` element char-by-char.
fn common_prefix(strings: &[Vec<char>]) -> Vec<char> {
    if strings.is_empty() {
        return Vec::new();
    }
    let s1 = strings.iter().min().unwrap();
    let s2 = strings.iter().max().unwrap();
    for (i, c) in s1.iter().enumerate() {
        if *c != s2[i] {
            return s1[..i].to_vec();
        }
    }
    s1.clone()
}

/// Port of `regex_opt_inner`.
fn regex_opt_inner(strings: &[Vec<char>], open_paren: &str) -> String {
    let close_paren = if open_paren.is_empty() { "" } else { ")" };
    if strings.is_empty() {
        return String::new();
    }
    let first = &strings[0];
    if strings.len() == 1 {
        return format!("{open_paren}{}{close_paren}", re_escape(first));
    }
    if first.is_empty() {
        return format!(
            "{open_paren}{}?{close_paren}",
            regex_opt_inner(&strings[1..], "(?:")
        );
    }
    if first.len() == 1 {
        // Multiple one-char strings? make a charset.
        let mut oneletter: Vec<char> = Vec::new();
        let mut rest: Vec<Vec<char>> = Vec::new();
        for s in strings {
            if s.len() == 1 {
                oneletter.push(s[0]);
            } else {
                rest.push(s.clone());
            }
        }
        if oneletter.len() > 1 {
            if !rest.is_empty() {
                return format!(
                    "{open_paren}{}|{}{close_paren}",
                    regex_opt_inner(&rest, ""),
                    make_charset(&oneletter)
                );
            }
            return format!("{open_paren}{}{close_paren}", make_charset(&oneletter));
        }
    }
    let prefix = common_prefix(strings);
    if !prefix.is_empty() {
        let plen = prefix.len();
        let tails: Vec<Vec<char>> = strings.iter().map(|s| s[plen..].to_vec()).collect();
        return format!(
            "{open_paren}{}{}{close_paren}",
            re_escape(&prefix),
            regex_opt_inner(&tails, "(?:")
        );
    }
    // Is there a suffix?
    let strings_rev: Vec<Vec<char>> = strings
        .iter()
        .map(|s| s.iter().rev().copied().collect())
        .collect();
    let suffix_rev = common_prefix(&strings_rev);
    if !suffix_rev.is_empty() {
        let slen = suffix_rev.len();
        let mut heads: Vec<Vec<char>> = strings
            .iter()
            .map(|s| s[..s.len() - slen].to_vec())
            .collect();
        heads.sort();
        let suffix: Vec<char> = suffix_rev.iter().rev().copied().collect();
        return format!(
            "{open_paren}{}{}{close_paren}",
            regex_opt_inner(&heads, "(?:"),
            re_escape(&suffix)
        );
    }
    // Last resort: recurse on consecutive groups keyed by whether the
    // string shares `first`'s first char. Mirrors
    // `groupby(strings, key=lambda s: s[0] == first[0])`.
    let first_char = first[0];
    let mut groups: Vec<String> = Vec::new();
    let mut i = 0;
    while i < strings.len() {
        let key = strings[i][0] == first_char;
        let mut j = i;
        while j < strings.len() && (strings[j][0] == first_char) == key {
            j += 1;
        }
        groups.push(regex_opt_inner(&strings[i..j], ""));
        i = j;
    }
    format!("{open_paren}{}{close_paren}", groups.join("|"))
}

/// Port of `regex_opt(strings, prefix='', suffix='')`.
///
/// Sorts `strings`, then wraps the optimized alternation in
/// `prefix` … `suffix`. Equivalent to what `pygments.lexer.words(...)`
/// stores as its compiled pattern.
pub fn regex_opt(words: &[&str], prefix: &str, suffix: &str) -> String {
    let mut strings: Vec<Vec<char>> = words.iter().map(|w| w.chars().collect()).collect();
    strings.sort();
    format!("{prefix}{}{suffix}", regex_opt_inner(&strings, "("))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opt(words: &[&str], prefix: &str, suffix: &str) -> String {
        regex_opt(words, prefix, suffix)
    }

    #[test]
    fn golden_vectors_match_upstream() {
        // Captured from `pygments.regexopt.regex_opt`.
        assert_eq!(opt(&["if", "else", "elif"], "", ""), "(el(?:if|se)|if)");
        assert_eq!(
            opt(&["def", "del", "class"], r"\b", r"\b"),
            r"\b(class|de(?:[fl]))\b"
        );
        assert_eq!(opt(&["a", "b", "c", "ab"], "", ""), "(ab|[abc])");
        assert_eq!(
            opt(&["True", "False", "None"], r"(?![\w])", ""),
            r"(?![\w])((?:Fals|Non|Tru)e)"
        );
        assert_eq!(
            opt(&["int", "float", "str", "list", "dict", "set"], "", ""),
            "(dict|float|int|list|s(?:et|tr))"
        );
        assert_eq!(
            opt(&["foo-bar", "foo.baz"], "", ""),
            r"(foo(?:\-bar|\.baz))"
        );
        assert_eq!(opt(&["x"], "", ""), "(x)");
        assert_eq!(opt(&["self", "cls"], "", ""), "(cls|self)");
    }

    #[test]
    fn re_escape_matches_python() {
        let chars: Vec<char> = "()[]{}?*+-|^$\\.&~# \t".chars().collect();
        assert_eq!(
            re_escape(&chars),
            "\\(\\)\\[\\]\\{\\}\\?\\*\\+\\-\\|\\^\\$\\\\\\.\\&\\~\\#\\ \\\t"
        );
    }

    #[test]
    fn generated_pattern_compiles_and_matches() {
        // The optimized pattern is a valid regex matching exactly the
        // input words and nothing else.
        let pat = opt(&["def", "del", "class"], "", "");
        let re = fancy_regex::Regex::new(&format!("^(?:{pat})$")).unwrap();
        for w in ["def", "del", "class"] {
            assert!(re.is_match(w).unwrap(), "should match {w}");
        }
        for w in ["de", "clas", "xyz"] {
            assert!(!re.is_match(w).unwrap(), "should not match {w}");
        }
    }
}
