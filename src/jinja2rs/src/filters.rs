//! `jinja2rs::filters` — Sphinx built-in Jinja2 filters.
//!
//! Ports the filter functions from `sphinx.jinja2glue`:
//! - `tobool` — coerce string/value to `bool`
//! - `toint` — coerce to `i64`, defaulting to 0
//! - `todim` — format a value as a CSS dimension (`px`)
//! - `slice_index` — partition index entries into N equal columns
//! - `filesizeformat` — format bytes as human-readable file size
//! - `indent` — indent a string by n spaces
//! - `wordwrap` — wrap text at specified width
//! - `xmlattr` — escape string as XML attribute value
//! - `urlencode` — URL-encode a string or dict

use minijinja::Value;

/// `tobool` filter — mirrors `sphinx.jinja2glue._tobool`.
///
/// For string inputs: `"true"`, `"1"`, `"yes"`, `"on"` → `true`; anything
/// else → `false`.  For non-string inputs, delegates to truthiness.
pub fn tobool(val: Value) -> bool {
    if let Some(s) = val.as_str() {
        matches!(s.to_lowercase().as_str(), "true" | "1" | "yes" | "on")
    } else {
        val.is_true()
    }
}

/// `toint` filter — mirrors `sphinx.jinja2glue._toint`.
///
/// Returns an integer parsed from the value, or `0` on failure.
pub fn toint(val: Value) -> i64 {
    if let Some(s) = val.as_str() {
        s.parse::<i64>().unwrap_or(0)
    } else if let Ok(n) = i64::try_from(val.clone()) {
        n
    } else {
        0
    }
}

/// `todim` filter — mirrors `sphinx.jinja2glue._todim`.
///
/// Converts a value to a CSS dimension string:
/// - `None` / undefined → `"initial"`
/// - `0` (integer or `"0"`) → `"0"`
/// - Integer or string representation of integer → `"{n}px"`
/// - Anything else → passes through as-is
pub fn todim(val: Value) -> String {
    if val.is_undefined() || val.is_none() {
        return "initial".to_string();
    }
    if let Some(s) = val.as_str() {
        if let Ok(n) = s.parse::<i64>() {
            return if n == 0 {
                "0".to_string()
            } else {
                format!("{}px", n)
            };
        }
        return s.to_string();
    }
    if let Ok(n) = i64::try_from(val.clone()) {
        return if n == 0 {
            "0".to_string()
        } else {
            format!("{}px", n)
        };
    }
    val.to_string()
}

/// `slice_index` filter — mirrors `sphinx.jinja2glue._slice_index`.
///
/// Distributes a flat list of index entries into `slices` roughly equal
/// column lists, counting sub-entries.  Returns a list of lists.
///
/// Each element of `values` is expected to be a tuple/list `[key, [_, sublist]]`
/// where `sublist` contributes to the column weight.
pub fn slice_index(values: Value, slices: usize) -> Value {
    if slices == 0 {
        return Value::from(Vec::<Value>::new());
    }

    // Count total logical weight (1 per entry + len of subitems).
    let mut total: usize = 0;
    let items: Vec<Value> = (0..values.len().unwrap_or(0))
        .filter_map(|i| values.get_item(&Value::from(i)).ok())
        .collect();

    for item in &items {
        let sub_len = item
            .get_item(&Value::from(1))
            .ok()
            .and_then(|pair| pair.get_item(&Value::from(1)).ok())
            .and_then(|sub| sub.len())
            .unwrap_or(0);
        total += 1 + sub_len;
    }

    let items_per_slice = if total == 0 { 1 } else { (total + slices - 1) / slices };
    let mut result: Vec<Value> = Vec::with_capacity(slices);
    let mut offset = 0usize;

    for slice_idx in 0..slices {
        let start = offset;
        if slice_idx + 1 == slices {
            offset = items.len();
        } else {
            let mut count = 0usize;
            while offset < items.len() {
                let sub_len = items[offset]
                    .get_item(&Value::from(1))
                    .ok()
                    .and_then(|pair| pair.get_item(&Value::from(1)).ok())
                    .and_then(|sub| sub.len())
                    .unwrap_or(0);
                count += 1 + sub_len;
                offset += 1;
                if count >= items_per_slice {
                    break;
                }
            }
        }
        result.push(Value::from(items[start..offset].to_vec()));
    }

    Value::from(result)
}

/// `filesizeformat` filter — mirrors `jinja2.filters.do_filesizeformat`.
///
/// Converts a file size in bytes to a human-readable format (B, KB, MB, GB, TB).
/// By default, `binary=true` uses 1024-based units; set `binary=false` for 1000-based SI units.
///
/// Examples:
/// - `1024|filesizeformat` → `"1.0 KiB"`
/// - `1000|filesizeformat` → `"1000 B"`
/// - `1000|filesizeformat(false)` → `"1.0 kB"`
/// - `1536|filesizeformat` → `"1.5 KiB"`
pub fn filesizeformat(value: Value, binary: Option<bool>) -> String {
    let bytes = match value.as_i64() {
        Some(n) => (n as f64).max(0.0),
        None => return value.to_string(),
    };

    const BINARY_UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    const DECIMAL_UNITS: &[&str] = &["B", "kB", "MB", "GB", "TB"];

    let (divisor, units) = if binary.unwrap_or(true) {
        (1024.0, BINARY_UNITS)
    } else {
        (1000.0, DECIMAL_UNITS)
    };

    if bytes < divisor {
        return format!("{:.0} {}", bytes, units[0]);
    }

    let mut size = bytes;
    for i in 1..units.len() {
        size /= divisor;
        if size < divisor || i == units.len() - 1 {
            return format!("{:.1} {}", size, units[i]);
        }
    }

    format!("{:.1} {}", size, units[units.len() - 1])
}

/// `indent` filter — mirrors `jinja2.filters.do_indent`.
///
/// Indents a string with the given number of spaces. By default, does not
/// indent the first line unless `first` is set to `true`. Can also indent
/// blank lines by setting `blank` to `true`.
///
/// Examples:
/// - `"hello\nworld"|indent(2)` → `"hello\n  world"`
/// - `"hello\nworld"|indent(2, true)` → `"  hello\n  world"`
pub fn indent(value: Value, width: Option<u64>, first: Option<bool>, blank: Option<bool>) -> String {
    let s = value.to_string();
    let width = width.unwrap_or(4) as usize;
    let first = first.unwrap_or(false);
    let blank = blank.unwrap_or(false);

    let indent_str = " ".repeat(width);
    let lines: Vec<&str> = s.lines().collect();
    let mut result = String::new();

    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            result.push('\n');
        }
        let should_indent = (i > 0 || first) && (!line.is_empty() || blank);
        if should_indent {
            result.push_str(&indent_str);
        }
        result.push_str(line);
    }

    result
}

/// `wordwrap` filter — mirrors `jinja2.filters.do_wordwrap`.
///
/// Wraps text at the specified width. Preserves word boundaries.
/// Uses `minijinja_contrib::filters::wordwrap` internally.
///
/// Examples:
/// - `"hello world test"|wordwrap(5)` → `"hello\nworld\ntest"`
pub fn wordwrap(value: &Value, width: Option<u64>, break_long_words: Option<bool>) -> Result<String, minijinja::Error> {
    let s = value.to_string();
    let width = width.unwrap_or(79) as usize;
    let break_long = break_long_words.unwrap_or(false);

    if width == 0 {
        return Ok(s);
    }

    let mut result = String::new();
    let mut current_line = String::new();
    let mut word_buffer = String::new();

    for ch in s.chars() {
        if ch.is_whitespace() {
            if !word_buffer.is_empty() {
                // Add word to line
                if current_line.is_empty() {
                    current_line.push_str(&word_buffer);
                } else if current_line.len() + 1 + word_buffer.len() <= width {
                    current_line.push(' ');
                    current_line.push_str(&word_buffer);
                } else {
                    result.push_str(&current_line);
                    result.push('\n');
                    current_line = word_buffer.clone();
                }
                word_buffer.clear();
            }
            if ch == '\n' {
                result.push_str(&current_line);
                result.push('\n');
                current_line.clear();
            }
        } else {
            word_buffer.push(ch);
            if break_long && word_buffer.len() > width {
                if !current_line.is_empty() {
                    result.push_str(&current_line);
                    result.push('\n');
                    current_line.clear();
                }
                result.push_str(&word_buffer);
                result.push('\n');
                word_buffer.clear();
            }
        }
    }

    if !word_buffer.is_empty() {
        if current_line.is_empty() {
            current_line.push_str(&word_buffer);
        } else if current_line.len() + 1 + word_buffer.len() <= width {
            current_line.push(' ');
            current_line.push_str(&word_buffer);
        } else {
            result.push_str(&current_line);
            result.push('\n');
            current_line = word_buffer;
        }
    }

    if !current_line.is_empty() {
        result.push_str(&current_line);
    }

    Ok(result)
}

/// `xmlattr` filter — mirrors `jinja2.filters.do_xmlattr`.
///
/// Converts a dict/object to XML attribute list. Values are XML-escaped
/// and quoted. The filter should be used inside tag markup.
///
/// Examples:
/// - `{"class": "foo", "id": "bar"}|xmlattr` → ` class="foo" id="bar"`
/// - `{"data": "a & b"}|xmlattr` → ` data="a &amp; b"`
pub fn xmlattr(attrs: Value) -> String {
    let mut result = String::new();

    // Convert value to JSON string and parse it back as JSON for iteration
    if let Ok(json_str) = serde_json::to_string(&attrs) {
        if let Ok(serde_json::Value::Object(map)) = serde_json::from_str(&json_str) {
            for (key, val) in map.iter() {
                let escaped = val.to_string()
                    .trim_matches('"')  // JSON serialization adds quotes
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;")
                    .replace('\'', "&#x27;");
                result.push(' ');
                result.push_str(key);
                result.push_str("=\"");
                result.push_str(&escaped);
                result.push('"');
            }
        }
    }

    result
}

/// `urlencode` filter — mirrors `jinja2.filters.do_urlencode`.
///
/// URL-encodes a string or dict. For dicts, formats as query string (key=value&key=value).
///
/// Examples:
/// - `"hello world"|urlencode` → `"hello%20world"`
/// - `{"q": "hello world"}|urlencode` → `"q=hello+world"`
pub fn urlencode(value: &Value) -> Result<String, minijinja::Error> {
    if let Some(s) = value.as_str() {
        Ok(urlencoding::encode(s).into_owned())
    } else {
        // Convert to JSON string and parse as JSON for dict handling
        if let Ok(json_str) = serde_json::to_string(&value) {
            if let Ok(serde_json::Value::Object(map)) = serde_json::from_str(&json_str) {
                let mut pairs = Vec::new();
                for (k, v) in map.iter() {
                    let key = urlencoding::encode(k).into_owned();
                    let val_str = if let serde_json::Value::String(s) = v {
                        s.clone()
                    } else {
                        v.to_string().trim_matches('"').to_string()
                    };
                    let val = urlencoding::encode(&val_str).into_owned();
                    pairs.push(format!("{}={}", key, val));
                }
                return Ok(pairs.join("&"));
            }
        }
        Ok(urlencoding::encode(&value.to_string()).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use minijinja::Value;

    #[test]
    fn test_tobool_strings() {
        assert!(tobool(Value::from("true")));
        assert!(tobool(Value::from("True")));
        assert!(tobool(Value::from("TRUE")));
        assert!(tobool(Value::from("1")));
        assert!(tobool(Value::from("yes")));
        assert!(tobool(Value::from("on")));
        assert!(!tobool(Value::from("false")));
        assert!(!tobool(Value::from("0")));
        assert!(!tobool(Value::from("no")));
        assert!(!tobool(Value::from("")));
    }

    #[test]
    fn test_tobool_non_strings() {
        assert!(tobool(Value::from(true)));
        assert!(tobool(Value::from(42i64)));
        assert!(!tobool(Value::from(false)));
        assert!(!tobool(Value::from(0i64)));
    }

    #[test]
    fn test_toint_strings() {
        assert_eq!(toint(Value::from("42")), 42);
        assert_eq!(toint(Value::from("-7")), -7);
        assert_eq!(toint(Value::from("notanint")), 0);
        assert_eq!(toint(Value::from("")), 0);
    }

    #[test]
    fn test_toint_numbers() {
        assert_eq!(toint(Value::from(10i64)), 10);
        assert_eq!(toint(Value::from(0i64)), 0);
    }

    #[test]
    fn test_todim_none() {
        assert_eq!(todim(Value::UNDEFINED), "initial");
    }

    #[test]
    fn test_todim_zero() {
        assert_eq!(todim(Value::from(0i64)), "0");
        assert_eq!(todim(Value::from("0")), "0");
    }

    #[test]
    fn test_todim_integer() {
        assert_eq!(todim(Value::from(320i64)), "320px");
        assert_eq!(todim(Value::from("200")), "200px");
    }

    #[test]
    fn test_todim_passthrough() {
        assert_eq!(todim(Value::from("2em")), "2em");
        assert_eq!(todim(Value::from("auto")), "auto");
    }

    #[test]
    fn test_filesizeformat_binary() {
        assert_eq!(filesizeformat(Value::from(0.0), Some(true)), "0 B");
        assert_eq!(filesizeformat(Value::from(512.0), Some(true)), "512 B");
        assert_eq!(filesizeformat(Value::from(1024.0), Some(true)), "1.0 KiB");
        assert_eq!(filesizeformat(Value::from(1536.0), Some(true)), "1.5 KiB");
        assert_eq!(filesizeformat(Value::from(1048576.0), Some(true)), "1.0 MiB");
    }

    #[test]
    fn test_filesizeformat_decimal() {
        assert_eq!(filesizeformat(Value::from(1000.0), Some(false)), "1.0 kB");
        assert_eq!(filesizeformat(Value::from(1500.0), Some(false)), "1.5 kB");
        assert_eq!(filesizeformat(Value::from(1000000.0), Some(false)), "1.0 MB");
    }

    #[test]
    fn test_filesizeformat_large() {
        assert_eq!(filesizeformat(Value::from(1099511627776.0), Some(true)), "1.0 TiB");
        assert_eq!(filesizeformat(Value::from(1000000000000.0), Some(false)), "1.0 TB");
    }

    #[test]
    fn test_filesizeformat_negative() {
        // Negative values should be treated as 0
        assert_eq!(filesizeformat(Value::from(-100.0), Some(true)), "0 B");
    }

    #[test]
    fn test_indent_default() {
        assert_eq!(indent(Value::from("hello\nworld"), None, None, None), "hello\n    world");
        assert_eq!(indent(Value::from("hello\nworld"), Some(2), None, None), "hello\n  world");
    }

    #[test]
    fn test_indent_first_line() {
        assert_eq!(indent(Value::from("hello\nworld"), Some(2), Some(true), None), "  hello\n  world");
    }

    #[test]
    fn test_indent_blank_lines() {
        let input = "hello\n\nworld";
        assert_eq!(indent(Value::from(input), Some(2), Some(false), Some(false)), "hello\n\n  world");
        assert_eq!(indent(Value::from(input), Some(2), Some(false), Some(true)), "hello\n  \n  world");
    }

    #[test]
    fn test_wordwrap_basic() {
        let result = wordwrap(&Value::from("hello world test"), Some(10), None);
        assert!(result.is_ok());
        // Word wrap should keep words on separate lines if they exceed width
        let wrapped = result.unwrap();
        assert!(wrapped.contains('\n'));
    }

    #[test]
    fn test_wordwrap_long_word() {
        let result = wordwrap(&Value::from("verylongword short"), Some(5), Some(true));
        assert!(result.is_ok());
    }

    #[test]
    fn test_xmlattr_basic() {
        use std::collections::BTreeMap;
        let mut attrs = BTreeMap::new();
        attrs.insert("class", "foo");
        attrs.insert("id", "bar");
        let val = minijinja::Value::from_object(attrs);
        let result = xmlattr(val);
        assert!(result.contains("class=\"foo\""));
        assert!(result.contains("id=\"bar\""));
    }

    #[test]
    fn test_xmlattr_escaping() {
        use std::collections::BTreeMap;
        let mut attrs = BTreeMap::new();
        attrs.insert("data", "a & b");
        let val = minijinja::Value::from_object(attrs);
        let result = xmlattr(val);
        assert!(result.contains("&amp;"));
    }

    #[test]
    fn test_xmlattr_quote_escaping() {
        use std::collections::BTreeMap;
        let mut attrs = BTreeMap::new();
        attrs.insert("title", "a \"quoted\" word");
        let val = minijinja::Value::from_object(attrs);
        let result = xmlattr(val);
        assert!(result.contains("&quot;"));
    }

    #[test]
    fn test_urlencode_string() {
        let result = urlencode(&Value::from("hello world"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello%20world");
    }

    #[test]
    fn test_urlencode_special_chars() {
        let result = urlencode(&Value::from("a&b=c"));
        assert!(result.is_ok());
        let encoded = result.unwrap();
        assert!(encoded.contains("%26")); // & encoded
        assert!(encoded.contains("%3D")); // = encoded
    }

    #[test]
    fn test_urlencode_dict() {
        use std::collections::BTreeMap;
        let mut dict = BTreeMap::new();
        dict.insert("q", "hello");
        dict.insert("lang", "en");
        let val = minijinja::Value::from_object(dict);
        let result = urlencode(&val);
        assert!(result.is_ok());
        let encoded = result.unwrap();
        assert!(encoded.contains("q=hello"));
        assert!(encoded.contains("lang=en"));
    }
}

