//! `sphinxdocrs::util_uri` — Rust port of `sphinx.util._uri`.
//!
//! URL encoding and URL-detection utilities.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `encode_uri(uri)` | [`encode_uri`] | percent-encodes path/query; IDNA-encodes netloc |
//! | `is_url(url)` | [`is_url`] | checks for `://` scheme |

use std::borrow::Cow;

// ── is_url ────────────────────────────────────────────────────────────────────

/// Return `true` if `url` looks like an absolute URL (has a `://` scheme).
///
/// Mirrors `sphinx.util._uri.is_url`.
///
/// ```rust
/// use sphinxdocrs::util_uri::is_url;
/// assert!(is_url("https://example.com"));
/// assert!(is_url("ftp://files.example.org/file.txt"));
/// assert!(!is_url("example.com"));
/// assert!(!is_url(""));
/// ```
pub fn is_url(url: &str) -> bool {
    !url.is_empty() && url.contains("://")
}

// ── encode_uri ────────────────────────────────────────────────────────────────

/// Percent-encode a URI, preserving slashes in the path and encoding the
/// query string as `application/x-www-form-urlencoded`.
///
/// Mirrors `sphinx.util._uri.encode_uri`:
/// - The netloc is IDNA-encoded (Punycode for international domain names).
/// - The path is percent-encoded with `/` preserved.
/// - The query string is percent-encoded as `key=value` pairs.
/// - The fragment is left unchanged.
///
/// ```rust
/// use sphinxdocrs::util_uri::encode_uri;
/// let uri = "https://ru.wikipedia.org/wiki/Система_управления_базами_данных";
/// let encoded = encode_uri(uri);
/// assert!(encoded.starts_with("https://ru.wikipedia.org/wiki/"));
/// assert!(!encoded.contains('С')); // Cyrillic encoded
/// ```
pub fn encode_uri(uri: &str) -> String {
    // Parse into components.
    let (scheme, rest) = split_scheme(uri);
    if scheme.is_empty() {
        // No scheme — just percent-encode the path portion.
        return percent_encode_path(uri);
    }

    // rest = "//netloc/path?query#fragment" or "//netloc/path"
    let rest = rest.strip_prefix("//").unwrap_or(rest);

    // Split netloc from the remainder.
    let (netloc, path_query_frag) = split_netloc(rest);

    // Split path, query, fragment.
    let (path, query, fragment) = split_path_query_fragment(path_query_frag);

    // IDNA-encode the netloc.
    let encoded_netloc = idna_encode(netloc);

    // Percent-encode path (preserve slashes).
    let encoded_path = percent_encode_path(path);

    // Encode query string.
    let encoded_query = encode_query(query);

    // Rebuild.
    let mut result = format!("{scheme}://{encoded_netloc}{encoded_path}");
    if !encoded_query.is_empty() {
        result.push('?');
        result.push_str(&encoded_query);
    }
    if !fragment.is_empty() {
        result.push('#');
        result.push_str(fragment);
    }
    result
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn split_scheme(uri: &str) -> (&str, &str) {
    if let Some(pos) = uri.find("://") {
        (&uri[..pos], &uri[pos + 3..])
    } else {
        ("", uri)
    }
}

fn split_netloc(s: &str) -> (&str, &str) {
    // netloc ends at first '/' or '?' or '#' or end of string
    let end = s.find(['/', '?', '#']).unwrap_or(s.len());
    (&s[..end], &s[end..])
}

fn split_path_query_fragment(s: &str) -> (&str, &str, &str) {
    // Extract fragment first.
    let (before_frag, fragment) = if let Some(pos) = s.rfind('#') {
        (&s[..pos], &s[pos + 1..])
    } else {
        (s, "")
    };
    // Then query.
    let (path, query) = if let Some(pos) = before_frag.find('?') {
        (&before_frag[..pos], &before_frag[pos + 1..])
    } else {
        (before_frag, "")
    };
    (path, query, fragment)
}

/// IDNA-encode a domain label.
///
/// For domains that are already ASCII this is a no-op. For international
/// domains we encode each label using Punycode (RFC 3492).
///
/// This is a simplified implementation that handles the common case; it
/// does not perform full UTS #46 processing.
fn idna_encode(host: &str) -> Cow<'_, str> {
    // If the host is pure ASCII (or has a port), return it unchanged.
    let (host_part, port) = if let Some(colon) = host.rfind(':') {
        if host[colon + 1..].chars().all(|c| c.is_ascii_digit()) {
            (&host[..colon], &host[colon..])
        } else {
            (host, "")
        }
    } else {
        (host, "")
    };

    if host_part.is_ascii() {
        return Cow::Borrowed(host);
    }

    // Encode each label.
    let encoded: String = host_part
        .split('.')
        .map(|label| {
            if label.is_ascii() {
                label.to_string()
            } else {
                // Punycode encode the label.
                let encoded = punycode_encode(label);
                format!("xn--{encoded}")
            }
        })
        .collect::<Vec<_>>()
        .join(".");
    Cow::Owned(format!("{encoded}{port}"))
}

/// Minimal Punycode encoder (RFC 3492) for IDNA label encoding.
fn punycode_encode(s: &str) -> String {
    // Adapted from the reference algorithm in RFC 3492 §6.3.
    const BASE: u32 = 36;
    const TMIN: u32 = 1;
    const TMAX: u32 = 26;
    const SKEW: u32 = 38;
    const DAMP: u32 = 700;
    const INITIAL_BIAS: u32 = 72;
    const INITIAL_N: u32 = 128;

    fn adapt(mut delta: u32, num_points: u32, first_time: bool) -> u32 {
        delta = if first_time { delta / DAMP } else { delta / 2 };
        delta += delta / num_points;
        let mut k = 0u32;
        while delta > ((BASE - TMIN) * TMAX) / 2 {
            delta /= BASE - TMIN;
            k += BASE;
        }
        k + ((BASE - TMIN + 1) * delta) / (delta + SKEW)
    }

    fn digit_to_char(d: u32) -> char {
        if d < 26 {
            (b'a' + d as u8) as char
        } else {
            (b'0' + (d - 26) as u8) as char
        }
    }

    let input: Vec<char> = s.chars().collect();
    let mut output = String::new();

    // Output basic code points.
    let basics: Vec<char> = input.iter().filter(|c| c.is_ascii()).copied().collect();
    let b = basics.len() as u32;
    for &c in &basics {
        output.push(c);
    }
    if b > 0 {
        output.push('-');
    }

    let mut n = INITIAL_N;
    let mut delta: u32 = 0;
    let mut bias = INITIAL_BIAS;
    let mut h = b;
    let len = input.len() as u32;

    while h < len {
        // Find next smallest non-basic code point >= n.
        let m = input
            .iter()
            .filter(|&&c| c as u32 >= n)
            .map(|&c| c as u32)
            .min()
            .unwrap();
        delta = delta.saturating_add((m - n).saturating_mul(h + 1));
        n = m;

        for &c in &input {
            let cp = c as u32;
            if cp < n || cp < INITIAL_N {
                delta = delta.saturating_add(1);
            }
            if cp == n {
                let mut q = delta;
                let mut k = BASE;
                loop {
                    let t = if k <= bias + TMIN {
                        TMIN
                    } else if k >= bias + TMAX {
                        TMAX
                    } else {
                        k - bias
                    };
                    if q < t {
                        break;
                    }
                    output.push(digit_to_char(t + (q - t) % (BASE - t)));
                    q = (q - t) / (BASE - t);
                    k += BASE;
                }
                output.push(digit_to_char(q));
                bias = adapt(delta, h + 1, h == b);
                delta = 0;
                h += 1;
            }
        }
        delta += 1;
        n += 1;
    }
    output
}

/// Percent-encode a URL path, preserving forward slashes.
fn percent_encode_path(path: &str) -> String {
    // Characters that do NOT need encoding in a URI path segment:
    // unreserved: A-Z a-z 0-9 - _ . ~
    // sub-delims: ! $ & ' ( ) * + , ; =
    // pchar extras: : @
    // We also preserve /
    path.chars()
        .flat_map(|c| {
            if is_safe_path_char(c) {
                vec![c]
            } else {
                // Encode each byte of the UTF-8 representation.
                c.to_string()
                    .bytes()
                    .flat_map(|b| {
                        let hi = (b >> 4) & 0xF;
                        let lo = b & 0xF;
                        vec!['%', hex_digit(hi), hex_digit(lo)]
                    })
                    .collect::<Vec<_>>()
            }
        })
        .collect()
}

fn is_safe_path_char(c: char) -> bool {
    matches!(c,
        'A'..='Z' | 'a'..='z' | '0'..='9' |
        '-' | '_' | '.' | '~' |   // unreserved
        '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=' | // sub-delims
        ':' | '@' | '/'  // pchar extras + slash
    )
}

/// Encode a query string as `application/x-www-form-urlencoded`.
///
/// Mirrors Python's `urlencode([(q, v.encode()) for (q, v) in parse_qsl(query)])`:
/// 1. Parse key=value pairs, URL-decoding values (including `+` → space).
/// 2. Re-encode values with percent-encoding + `+` for spaces.
fn encode_query(query: &str) -> String {
    if query.is_empty() {
        return String::new();
    }
    query
        .split('&')
        .map(|pair| {
            if let Some(eq) = pair.find('=') {
                let key = &pair[..eq];
                let raw_val = &pair[eq + 1..];
                // URL-decode the value (parse_qsl decodes percent sequences and +).
                let decoded = percent_decode_str(raw_val);
                // Re-encode using form encoding.
                format!("{}={}", key, percent_encode_query_value(&decoded))
            } else {
                pair.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("&")
}

/// Percent-decode a query string value: `%XX` → byte, `+` → space.
fn percent_decode_str(s: &str) -> String {
    let mut bytes = Vec::with_capacity(s.len());
    let mut chars = s.bytes().peekable();
    while let Some(b) = chars.next() {
        if b == b'+' {
            bytes.push(b' ');
        } else if b == b'%' {
            let hi = chars.next().unwrap_or(b'%');
            let lo = chars.next().unwrap_or(b'0');
            if let (Some(h), Some(l)) = (hex_val(hi), hex_val(lo)) {
                bytes.push((h << 4) | l);
            } else {
                bytes.push(b'%');
                bytes.push(hi);
                bytes.push(lo);
            }
        } else {
            bytes.push(b);
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn percent_encode_query_value(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if is_safe_query_char(c) {
                vec![c]
            } else if c == ' ' {
                vec!['+']
            } else {
                c.to_string()
                    .bytes()
                    .flat_map(|b| vec!['%', hex_digit((b >> 4) & 0xF), hex_digit(b & 0xF)])
                    .collect::<Vec<_>>()
            }
        })
        .collect()
}

fn is_safe_query_char(c: char) -> bool {
    matches!(c,
        'A'..='Z' | 'a'..='z' | '0'..='9' |
        '-' | '_' | '.' | '~' | '*'
    )
}

fn hex_digit(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'A' + n - 10) as char,
        _ => unreachable!(),
    }
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_url ────────────────────────────────────────────────────────────────

    #[test]
    fn is_url_https() {
        assert!(is_url("https://example.com"));
    }

    #[test]
    fn is_url_ftp() {
        assert!(is_url("ftp://files.example.org/file.txt"));
    }

    #[test]
    fn is_url_plain_path() {
        assert!(!is_url("example.com"));
    }

    #[test]
    fn is_url_empty() {
        assert!(!is_url(""));
    }

    #[test]
    fn is_url_relative_path() {
        assert!(!is_url("docs/index.rst"));
    }

    // ── encode_uri ────────────────────────────────────────────────────────────

    #[test]
    fn encode_uri_cyrillic_path() {
        // Mirrors test_encode_uri() from test_util_uri.py
        let uri = "https://ru.wikipedia.org/wiki/Система_управления_базами_данных";
        let encoded = encode_uri(uri);
        let expected = concat!(
            "https://ru.wikipedia.org/wiki/",
            "%D0%A1%D0%B8%D1%81%D1%82%D0%B5%D0%BC%D0%B0_",
            "%D1%83%D0%BF%D1%80%D0%B0%D0%B2%D0%BB%D0%B5%D0%BD%D0%B8%D1%8F_",
            "%D0%B1%D0%B0%D0%B7%D0%B0%D0%BC%D0%B8_%D0%B4%D0%B0%D0%BD%D0%BD%D1%8B%D1%85"
        );
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encode_uri_already_encoded_query() {
        // Python parse_qsl decodes percent-encoded values then urlencode re-encodes.
        // Net effect: already-encoded ASCII query values pass through unchanged.
        let uri = "https://github.com/search?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+is%3Asprint-friendly+user%3Ajupyter&type=Issues&ref=searchresults";
        let encoded = encode_uri(uri);
        assert_eq!(encoded, uri, "already-encoded URI should be unchanged");
    }

    #[test]
    fn encode_uri_plain_ascii_unchanged() {
        let uri = "https://example.com/docs/index.html";
        assert_eq!(encode_uri(uri), uri);
    }

    #[test]
    fn encode_uri_with_fragment() {
        let uri = "https://example.com/page#section";
        let encoded = encode_uri(uri);
        assert!(encoded.ends_with("#section"), "got: {encoded}");
    }
}
