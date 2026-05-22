//! `pygments.regexopt` port — Phase 0 stub.
//!
//! The full suffix-tree optimization for `words(...)` alternations
//! lands in Phase 1.

/// Naive baseline: join alternatives with `|`, escaping each one.
/// The optimized version (matching `pygments.regexopt.regex_opt`)
/// replaces this when fixtures need the byte-identical compiled
/// regex. The semantics are equivalent — only the compiled regex
/// shape differs.
pub fn regex_opt(words: &[&str], prefix: &str, suffix: &str) -> String {
    let mut parts: Vec<String> = words.iter().map(|w| regex::escape(w)).collect();
    parts.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));
    format!("{prefix}(?:{})\\b{suffix}", parts.join("|"))
}
