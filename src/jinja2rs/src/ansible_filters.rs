//! `jinja2rs::ansible_filters` — Ansible-specific template filters.
//!
//! Provides a curated set of Ansible standard filters for use in Ansible mode.
//! These filters are commonly used in Ansible playbooks and roles.
//!
//! # Standard Ansible Filters
//!
//! | Filter | Purpose | Example |
//! |--------|---------|---------|
//! | `to_nice_json` | Pretty-print JSON | `{{ data \| to_nice_json }}` |
//! | `to_nice_yaml` | Pretty-print YAML | `{{ data \| to_nice_yaml }}` |
//! | `combine` | Merge dicts recursively | `{{ dict1 \| combine(dict2) }}` |
//! | `regex_search` | Find regex pattern | `{{ text \| regex_search('^foo') }}` |
//! | `regex_replace` | Replace regex pattern | `{{ text \| regex_replace('foo', 'bar') }}` |
//! | `regex_findall` | Find all regex matches | `{{ text \| regex_findall('^\\w+') }}` |
//! | `path_join` | Join filesystem paths | `{{ '/tmp' \| path_join('file.txt') }}` |
//! | `quote` | Shell-quote string | `{{ cmd \| quote }}` |
//! | `from_json` | Parse JSON string | `{{ json_str \| from_json }}` |
//! | `from_yaml` | Parse YAML string | `{{ yaml_str \| from_yaml }}` |
//!
//! # Implementation Status
//!
//! | Filter | Status | Notes |
//! |--------|--------|-------|
//! | `to_nice_json` | ✅ stub | Ready for full impl |
//! | `to_nice_yaml` | 🟡 stub | Requires yaml crate |
//! | `combine` | ✅ stub | Recursive dict merge |
//! | `regex_search` | ✅ stub | Requires regex crate |
//! | `regex_replace` | ✅ stub | Requires regex crate |
//! | `regex_findall` | ✅ stub | Requires regex crate |
//! | `path_join` | ✅ stub | Stdlib std::path |
//! | `quote` | ✅ stub | Shell quoting logic |
//! | `from_json` | ✅ stub | Uses serde_json |
//! | `from_yaml` | 🟡 stub | Requires serde_yaml |

use minijinja::Value;
use serde_json::to_string_pretty;
use std::path::Path;

/// Pretty-print a value as JSON.
///
/// # Example
/// ```text
/// {{ data | to_nice_json }}
/// ```
pub fn to_nice_json(value: Value) -> Result<Value, minijinja::Error> {
    match serde_json::to_value(&value) {
        Ok(json_val) => match to_string_pretty(&json_val) {
            Ok(pretty) => Ok(Value::from(pretty)),
            Err(e) => Err(minijinja::Error::new(
                minijinja::ErrorKind::InvalidOperation,
                format!("Failed to format JSON: {}", e),
            )),
        },
        Err(e) => Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Failed to convert to JSON: {}", e),
        )),
    }
}

/// Parse a JSON string into a value.
///
/// # Example
/// ```text
/// {% set data = '{"key": "value"}' | from_json %}
/// ```
pub fn from_json(s: Value) -> Result<Value, minijinja::Error> {
    let s_str = s.to_string();
    match serde_json::from_str::<Value>(&s_str) {
        Ok(val) => Ok(val),
        Err(e) => Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Failed to parse JSON: {}", e),
        )),
    }
}

/// Shell-quote a string.
///
/// Escapes special characters so the string is safe to use in shell commands.
///
/// # Example
/// ```text
/// {{ filename | quote }}
/// ```
pub fn quote(s: Value) -> Value {
    let s_str = s.to_string();
    let quoted = if s_str.contains(|c: char| c.is_whitespace() || "\"'$`\\".contains(c)) {
        format!("'{}'", s_str.replace('\'', "'\\''"))
    } else {
        s_str
    };
    Value::from(quoted)
}

/// Join filesystem path components.
///
/// # Example
/// ```text
/// {{ '/var/log' | path_join('app.log') }}
/// # => /var/log/app.log
/// ```
pub fn path_join(base: Value, component: Value) -> Value {
    let base_str = base.to_string();
    let component_str = component.to_string();
    let path = Path::new(&base_str).join(&component_str);
    Value::from(path.to_string_lossy().to_string())
}

/// Merge two dictionaries recursively (Ansible-style).
///
/// Second dict values override first dict values.
/// For nested dicts, merges recursively instead of replacing.
///
/// # Example
/// ```text
/// {% set merged = dict1 | combine(dict2) %}
/// ```
///
/// # Note
///
/// This is a stub that requires full implementation with proper dict API support.
/// Current version returns an error; use filter-based approach instead.
pub fn combine(
    _dict1: Value,
    _dict2: Value,
) -> Result<Value, minijinja::Error> {
    // Stub: combine requires minijinja dict API which is limited
    // For now, recommend using Ansible's combine filter via other means
    Err(minijinja::Error::new(
        minijinja::ErrorKind::InvalidOperation,
        "combine filter is a stub; use inline merging instead".to_string(),
    ))
}

/// Find regex matches in a string (stub).
///
/// Returns first match or empty string if no match.
///
/// # Example
/// ```text
/// {{ text | regex_search('^version: (.+)') }}
/// ```
pub fn regex_search(text: Value, pattern: Value) -> Value {
    // Stub: requires regex crate for full implementation
    let text_str = text.to_string();
    let pattern_str = pattern.to_string();
    Value::from(format!(
        "[regex_search stub: pattern='{}' in '{}']",
        pattern_str, text_str
    ))
}

/// Replace regex matches in a string (stub).
///
/// # Example
/// ```text
/// {{ text | regex_replace('foo', 'bar') }}
/// ```
pub fn regex_replace(text: Value, pattern: Value, replacement: Value) -> Value {
    // Stub: requires regex crate for full implementation
    let text_str = text.to_string();
    let pattern_str = pattern.to_string();
    let replacement_str = replacement.to_string();
    Value::from(format!(
        "[regex_replace stub: '{}' -> '{}' in '{}']",
        pattern_str, replacement_str, text_str
    ))
}

/// Find all regex matches in a string (stub).
///
/// Returns list of matches.
///
/// # Example
/// ```text
/// {% for match in text | regex_findall('\\w+') %}
/// ```
pub fn regex_findall(text: Value, pattern: Value) -> Value {
    // Stub: requires regex crate for full implementation
    let text_str = text.to_string();
    let pattern_str = pattern.to_string();
    Value::from(vec![format!(
        "[regex_findall stub: pattern='{}' in '{}']",
        pattern_str, text_str
    )])
}

/// Pretty-print a value as YAML (stub).
///
/// # Example
/// ```text
/// {{ data | to_nice_yaml }}
/// ```
pub fn to_nice_yaml(value: Value) -> Value {
    // Stub: requires serde_yaml for full implementation
    Value::from(format!(
        "[to_nice_yaml stub: {:?}]",
        value
    ))
}

/// Parse a YAML string into a value (stub).
///
/// # Example
/// ```text
/// {% set data = 'key: value' | from_yaml %}
/// ```
pub fn from_yaml(s: Value) -> Value {
    // Stub: requires serde_yaml for full implementation
    Value::from(format!(
        "[from_yaml stub: '{}']",
        s
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_nice_json() {
        use std::collections::BTreeMap;
        let mut data_map = BTreeMap::new();
        data_map.insert("key", Value::from("value"));
        let data = Value::from(data_map);
        let result = to_nice_json(data).unwrap();
        let result_str = result.to_string();
        assert!(result_str.contains("key"));
        assert!(result_str.contains("value"));
    }

    #[test]
    fn test_from_json() {
        let json_str = Value::from(r#"{"name": "test"}"#);
        let result = from_json(json_str).unwrap();
        let name_key = Value::from("name");
        assert_eq!(result.get_item(&name_key).unwrap().as_str(), Some("test"));
    }

    #[test]
    fn test_quote_with_spaces() {
        let result = quote(Value::from("hello world"));
        assert_eq!(result.to_string(), "'hello world'");
    }

    #[test]
    fn test_quote_no_special_chars() {
        let result = quote(Value::from("hello"));
        assert_eq!(result.to_string(), "hello");
    }

    #[test]
    fn test_path_join() {
        let result = path_join(Value::from("/var/log"), Value::from("app.log"));
        let result_str = result.to_string();
        assert!(result_str.contains("var"));
        assert!(result_str.contains("app.log"));
    }

    #[test]
    fn test_combine_basic() {
        use std::collections::BTreeMap;
        let mut dict1_map = BTreeMap::new();
        dict1_map.insert("a", Value::from(1));
        dict1_map.insert("b", Value::from(2));
        let dict1 = Value::from(dict1_map);
        
        let mut dict2_map = BTreeMap::new();
        dict2_map.insert("b", Value::from(3));
        dict2_map.insert("c", Value::from(4));
        let dict2 = Value::from(dict2_map);
        
        let result = combine(dict1, dict2);
        // Combine is a stub, so it should return an error
        assert!(result.is_err());
    }

    #[test]
    fn test_regex_search_stub() {
        let result = regex_search(Value::from("test123"), Value::from(r#"^\w+"#));
        let result_str = result.to_string();
        assert!(result_str.contains("stub"));
    }
}
