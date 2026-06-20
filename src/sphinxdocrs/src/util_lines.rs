//! `sphinxdocrs::util_lines` — Rust port of `sphinx.util._lines`.
//!
//! Parses line number specifications like `"1,2,4-6"` into 0-based index lists.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `parse_line_num_spec(spec, total)` | [`parse_line_num_spec`] | mirrors exact semantics including half-open ranges |

/// Parse a line number specification and return a list of 0-based line indices.
///
/// The `spec` is a comma-separated list of:
/// - A single number `N` → 0-based index `N-1`.
/// - A range `M-N` → indices `M-1` through `N-1` inclusive.
/// - A half-open left range `-N` → indices `0` through `N-1` (start defaults to 1).
/// - A half-open right range `N-` → indices `N-1` through `total-1` (end defaults
///   to `max(N, total)`).
///
/// Mirrors `sphinx.util._lines.parse_line_num_spec`.
///
/// # Errors
/// Returns `Err` with an `"invalid line number spec: ..."` message for any
/// malformed part.
///
/// ```rust
/// use sphinxdocrs::util_lines::parse_line_num_spec;
/// assert_eq!(parse_line_num_spec("1,2,3", 10).unwrap(), vec![0, 1, 2]);
/// assert_eq!(parse_line_num_spec("7-9", 10).unwrap(), vec![6, 7, 8]);
/// assert_eq!(parse_line_num_spec("-4", 10).unwrap(), vec![0, 1, 2, 3]);
/// assert_eq!(parse_line_num_spec("7-", 10).unwrap(), vec![6, 7, 8, 9]);
/// ```
pub fn parse_line_num_spec(spec: &str, total: usize) -> Result<Vec<usize>, String> {
    let mut items = Vec::new();
    for part in spec.split(',') {
        let part = part.trim();
        let begend: Vec<&str> = part.splitn(3, '-').collect();

        // Detect the split: we need to handle leading '-' for half-open left.
        // Python splits on '-', so 'M-N' → ['M','N'], '-N' → ['','N'],
        // 'N-' → ['N',''], '--' or '-' → ['',''] which is invalid.
        // We replicate this by splitting on the first '-' only.
        let result = parse_part(part, total);
        match result {
            Ok(indices) => items.extend(indices),
            Err(_) => {
                return Err(format!("invalid line number spec: {spec:?}"));
            }
        }
        let _ = begend; // used above for context
    }
    Ok(items)
}

fn parse_part(part: &str, total: usize) -> Result<Vec<usize>, ()> {
    // Find the split point.  We need to split on '-' but a leading '-'
    // is the left-open marker, not a sign.
    // Strategy: find '-' after position 0 for ranges; a bare '-' at pos 0
    // means left-open.
    let parts = split_range(part);
    match parts.as_slice() {
        [single] => {
            // Single number
            let n: usize = single.parse::<usize>().map_err(|_| ())?;
            if n == 0 {
                return Err(()); // 0 is not a valid 1-based line number
            }
            Ok(vec![n - 1])
        }
        [left, right] => {
            // Range M-N  or  -N  (left=="") or  N-  (right=="")
            if left.is_empty() && right.is_empty() {
                // bare '-' → invalid
                return Err(());
            }
            let start: usize = if left.is_empty() {
                1
            } else {
                left.parse::<usize>().map_err(|_| ())?
            };
            let end: usize = if right.is_empty() {
                start.max(total)
            } else {
                right.parse::<usize>().map_err(|_| ())?
            };
            if start > end {
                return Err(());
            }
            Ok((start - 1..end).collect())
        }
        _ => Err(()), // e.g. "1-2-3" → 3 parts
    }
}

/// Split a range part on the first '-' that is not at position 0.
///
/// - `"7-9"` → `["7", "9"]`
/// - `"-4"` → `["", "4"]` (left-open)
/// - `"7-"` → `["7", ""]` (right-open)
/// - `"-"` → `["", ""]` (invalid)
/// - `"7"` → `["7"]` (single)
/// - `"1-2-3"` → `["1", "2", "3"]` (too many → error)
fn split_range(s: &str) -> Vec<&str> {
    // Count explicit '-' characters; a leading '-' is the left-empty marker.
    if s.is_empty() {
        return vec![s];
    }
    let bytes = s.as_bytes();

    // Check for leading '-'
    if bytes[0] == b'-' {
        // Left-open range "-N" → ["", rest]
        let rest = &s[1..];
        // rest may itself contain '-' → extra splits → error
        if rest.contains('-') {
            // e.g. "-2-3"
            let mut parts = vec![""];
            parts.extend(rest.split('-'));
            return parts;
        }
        return vec!["", rest];
    }

    // No leading '-': split on first '-'
    match s.find('-') {
        None => vec![s],
        Some(pos) => {
            let left = &s[..pos];
            let right = &s[pos + 1..];
            // right may still contain '-' → "1-2-3"
            if right.contains('-') {
                let mut parts = vec![left];
                parts.extend(right.split('-'));
                return parts;
            }
            vec![left, right]
        }
    }
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Mirrors test_parse_line_num_spec from test_util_lines.py

    #[test]
    fn single_numbers() {
        assert_eq!(parse_line_num_spec("1,2,3", 10).unwrap(), vec![0, 1, 2]);
    }

    #[test]
    fn single_numbers_with_spaces() {
        assert_eq!(parse_line_num_spec("4, 5, 6", 10).unwrap(), vec![3, 4, 5]);
    }

    #[test]
    fn left_open_range() {
        assert_eq!(parse_line_num_spec("-4", 10).unwrap(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn closed_range() {
        assert_eq!(parse_line_num_spec("7-9", 10).unwrap(), vec![6, 7, 8]);
    }

    #[test]
    fn right_open_range() {
        assert_eq!(parse_line_num_spec("7-", 10).unwrap(), vec![6, 7, 8, 9]);
    }

    #[test]
    fn mixed_single_and_right_open() {
        assert_eq!(
            parse_line_num_spec("1,7-", 10).unwrap(),
            vec![0, 6, 7, 8, 9]
        );
    }

    #[test]
    fn single_element_range() {
        assert_eq!(parse_line_num_spec("7-7", 10).unwrap(), vec![6]);
    }

    #[test]
    fn right_open_beyond_total() {
        // "11-" with total=10: start=11 > total=10, so end=max(11,10)=11 → [10]
        assert_eq!(parse_line_num_spec("11-", 10).unwrap(), vec![10]);
    }

    #[test]
    fn error_too_many_dashes() {
        let err = parse_line_num_spec("1-2-3", 10).unwrap_err();
        assert!(err.contains("invalid line number spec"), "got: {err}");
        assert!(err.contains("1-2-3"), "got: {err}");
    }

    #[test]
    fn error_non_numeric() {
        let err = parse_line_num_spec("abc-def", 10).unwrap_err();
        assert!(err.contains("invalid line number spec"), "got: {err}");
    }

    #[test]
    fn error_bare_dash() {
        let err = parse_line_num_spec("-", 10).unwrap_err();
        assert!(err.contains("invalid line number spec"), "got: {err}");
    }

    #[test]
    fn error_inverted_range() {
        let err = parse_line_num_spec("3-1", 10).unwrap_err();
        assert!(err.contains("invalid line number spec"), "got: {err}");
    }
}
