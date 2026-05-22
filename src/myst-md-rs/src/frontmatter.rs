//! YAML front matter extraction.
//!
//! MyST (like Jekyll/Hugo) supports a YAML block fenced by `---` lines at the
//! very top of a document. We strip it before handing the body to
//! `pulldown-cmark` and expose the parsed mapping separately.

use serde_yaml::Value;

/// Result of splitting a document into front matter and body.
#[derive(Debug, Clone, Default)]
pub struct Split<'a> {
    pub front_matter: Option<Value>,
    pub raw_front_matter: Option<&'a str>,
    pub body: &'a str,
}

/// Split `source` into optional YAML front matter and the remaining body.
///
/// A front matter block must start on line 1 with `---` and is terminated by
/// the next line that is exactly `---` or `...`. If parsing the YAML fails we
/// still strip the block but leave `front_matter` as `None`.
pub fn split(source: &str) -> Split<'_> {
    let mut lines = source.split_inclusive('\n');
    let Some(first) = lines.next() else {
        return Split {
            front_matter: None,
            raw_front_matter: None,
            body: source,
        };
    };
    if first.trim_end_matches(['\r', '\n']) != "---" {
        return Split {
            front_matter: None,
            raw_front_matter: None,
            body: source,
        };
    }

    let after_open = source.len() - lines.clone().map(str::len).sum::<usize>();
    let mut cursor = after_open;
    for line in lines {
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed == "---" || trimmed == "..." {
            let raw = &source[after_open..cursor];
            let body_start = cursor + line.len();
            let body = source.get(body_start..).unwrap_or("");
            let parsed = serde_yaml::from_str::<Value>(raw).ok();
            return Split {
                front_matter: parsed,
                raw_front_matter: Some(raw),
                body,
            };
        }
        cursor += line.len();
    }

    // Unterminated front matter — treat the whole document as body.
    Split {
        front_matter: None,
        raw_front_matter: None,
        body: source,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_front_matter() {
        let s = split("hello\nworld\n");
        assert!(s.front_matter.is_none());
        assert_eq!(s.body, "hello\nworld\n");
    }

    #[test]
    fn parses_front_matter() {
        let src = "---\ntitle: hi\ncount: 3\n---\nbody\n";
        let s = split(src);
        assert_eq!(s.body, "body\n");
        let v = s.front_matter.expect("yaml");
        assert_eq!(v["title"], Value::String("hi".into()));
        assert_eq!(v["count"], Value::Number(3.into()));
    }

    #[test]
    fn unterminated_front_matter_is_body() {
        let src = "---\ntitle: hi\nbody\n";
        let s = split(src);
        assert!(s.front_matter.is_none());
        assert_eq!(s.body, src);
    }
}
