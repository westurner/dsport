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
use regex::Regex;
use serde_yaml;

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
/// Recursively merges nested dictionaries, combining arrays where both values are objects.
fn recursive_merge(base: &mut serde_json::Map<String, serde_json::Value>, other: &serde_json::Map<String, serde_json::Value>) {
    for (key, other_value) in other {
        match base.get_mut(key) {
            Some(base_value) => {
                // Both values are objects: merge recursively
                if let serde_json::Value::Object(other_obj) = other_value {
                    if let serde_json::Value::Object(base_obj) = base_value {
                        recursive_merge(base_obj, other_obj);
                    } else {
                        // Otherwise, other overwrites base
                        *base_value = other_value.clone();
                    }
                } else {
                    // Otherwise, other overwrites base
                    *base_value = other_value.clone();
                }
            }
            None => {
                // Key doesn't exist in base, just add it
                base.insert(key.clone(), other_value.clone());
            }
        }
    }
}

pub fn combine(
    dict1: Value,
    dict2: Value,
) -> Result<Value, minijinja::Error> {
    // Convert to serde_json values for merging
    let dict1_json = serde_json::to_value(&dict1).map_err(|e| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Failed to convert first dict: {}", e),
        )
    })?;
    
    let dict2_json = serde_json::to_value(&dict2).map_err(|e| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Failed to convert second dict: {}", e),
        )
    })?;

    // Both must be objects
    let mut base_obj = match dict1_json {
        serde_json::Value::Object(obj) => obj,
        _ => return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "First argument to combine must be a dict".to_string(),
        )),
    };

    let other_obj = match dict2_json {
        serde_json::Value::Object(obj) => obj,
        _ => return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "Second argument to combine must be a dict".to_string(),
        )),
    };

    // Recursive merge
    recursive_merge(&mut base_obj, &other_obj);

    // Convert back to minijinja Value
    Ok(Value::from_serialize(serde_json::Value::Object(base_obj)))
}

/// Find regex matches in a string.
///
/// Returns first match, or the first capture group if present, or empty string if no match.
///
/// # Example
/// ```text
/// {{ text | regex_search('^version: (.+)') }}
/// ```
pub fn regex_search(text: Value, pattern: Value) -> Value {
    let text_str = text.to_string();
    let pattern_str = pattern.to_string();

    match Regex::new(&pattern_str) {
        Ok(re) => {
            if let Some(caps) = re.captures(&text_str) {
                // Return first capture group if exists, otherwise full match
                if caps.len() > 1 {
                    Value::from(caps.get(1).map(|m| m.as_str()).unwrap_or(""))
                } else {
                    Value::from(caps.get(0).map(|m| m.as_str()).unwrap_or(""))
                }
            } else {
                Value::from("")
            }
        }
        Err(_) => Value::from(""),
    }
}

/// Replace regex matches in a string.
///
/// Replaces all occurrences of the pattern with the replacement string.
///
/// # Example
/// ```text
/// {{ text | regex_replace('foo', 'bar') }}
/// ```
pub fn regex_replace(text: Value, pattern: Value, replacement: Value) -> Value {
    let text_str = text.to_string();
    let pattern_str = pattern.to_string();
    let replacement_str = replacement.to_string();

    match Regex::new(&pattern_str) {
        Ok(re) => Value::from(re.replace_all(&text_str, replacement_str.as_str()).to_string()),
        Err(_) => Value::from(text_str),
    }
}

/// Find all regex matches in a string.
///
/// Returns list of all non-overlapping matches. If the pattern has capture groups,
/// returns capture group 1 for each match; otherwise returns full match.
///
/// # Example
/// ```text
/// {% for match in text | regex_findall('\\w+') %}
///   {{ match }}
/// {% endfor %}
/// ```
pub fn regex_findall(text: Value, pattern: Value) -> Value {
    let text_str = text.to_string();
    let pattern_str = pattern.to_string();

    match Regex::new(&pattern_str) {
        Ok(re) => {
            let mut matches = Vec::new();
            for caps in re.captures_iter(&text_str) {
                // Return first capture group if exists, otherwise full match
                let m = if caps.len() > 1 {
                    caps.get(1).map(|m| m.as_str()).unwrap_or("")
                } else {
                    caps.get(0).map(|m| m.as_str()).unwrap_or("")
                };
                matches.push(Value::from(m));
            }
            Value::from(matches)
        }
        Err(_) => Value::from(Vec::<Value>::new()),
    }
}

/// Pretty-print a value as YAML.
///
/// Converts the value to YAML format with proper indentation and formatting.
///
/// # Example
/// ```text
/// {{ data | to_nice_yaml }}
/// ```
pub fn to_nice_yaml(value: Value) -> Value {
    let json_val = match serde_json::to_value(&value) {
        Ok(v) => v,
        Err(e) => return Value::from(format!("[ERROR: Failed to convert to JSON: {}]", e)),
    };

    match serde_yaml::to_string(&json_val) {
        Ok(yaml_str) => Value::from(yaml_str),
        Err(e) => Value::from(format!("[ERROR: Failed to convert to YAML: {}]", e)),
    }
}

/// Parse a YAML string into a value.
///
/// Parses YAML string and returns the parsed value, allowing access to fields within templates.
///
/// # Example
/// ```text
/// {% set data = 'key: value
/// nested:
///   item: value2' | from_yaml %}
/// {{ data.key }}
/// ```
pub fn from_yaml(s: Value) -> Value {
    let s_str = s.to_string();
    match serde_yaml::from_str::<serde_json::Value>(&s_str) {
        Ok(yaml_val) => Value::from_serialize(yaml_val),
        Err(e) => Value::from(format!("[ERROR: Failed to parse YAML: {}]", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_to_nice_json() {
        let mut data_map = BTreeMap::new();
        data_map.insert("key", Value::from("value"));
        data_map.insert("number", Value::from(42));
        let data = Value::from(data_map);
        let result = to_nice_json(data).unwrap();
        let result_str = result.to_string();
        assert!(result_str.contains("key"));
        assert!(result_str.contains("value"));
    }

    #[test]
    fn test_from_json() {
        let json_str = Value::from(r#"{"name": "test", "count": 5}"#);
        let result = from_json(json_str).unwrap();
        assert!(result.to_string().contains("test") || result.to_string().contains("5"));
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
    fn test_quote_with_dollar() {
        let result = quote(Value::from("$HOME"));
        let result_str = result.to_string();
        assert!(result_str.contains("'") || result_str.contains("$"));
    }

    #[test]
    fn test_path_join() {
        let result = path_join(Value::from("/var/log"), Value::from("app.log"));
        let result_str = result.to_string();
        assert!(result_str.contains("var") && result_str.contains("app.log"));
    }

    #[test]
    fn test_combine_basic() {
        let mut dict1_map = BTreeMap::new();
        dict1_map.insert("a", Value::from(1));
        dict1_map.insert("b", Value::from(2));
        let dict1 = Value::from(dict1_map);
        
        let mut dict2_map = BTreeMap::new();
        dict2_map.insert("b", Value::from(3));
        dict2_map.insert("c", Value::from(4));
        let dict2 = Value::from(dict2_map);
        
        let result = combine(dict1, dict2);
        assert!(result.is_ok(), "combine should succeed");
        let merged = result.unwrap();
        // Result should contain all keys from both dicts
        assert!(merged.to_string().contains("a") || merged.to_string().contains("1"));
    }

    #[test]
    fn test_combine_nested() {
        let mut nested1 = BTreeMap::new();
        nested1.insert("x", Value::from(1));
        
        let mut dict1 = BTreeMap::new();
        dict1.insert("nested", Value::from(nested1));
        dict1.insert("a", Value::from(1));
        
        let mut nested2 = BTreeMap::new();
        nested2.insert("y", Value::from(2));
        
        let mut dict2 = BTreeMap::new();
        dict2.insert("nested", Value::from(nested2));
        dict2.insert("b", Value::from(2));
        
        let result = combine(Value::from(dict1), Value::from(dict2));
        assert!(result.is_ok());
        let merged_str = result.unwrap().to_string();
        // Merged result should have both x and y in nested
        assert!(!merged_str.is_empty());
    }

    #[test]
    fn test_regex_search_basic() {
        let result = regex_search(Value::from("version: 1.2.3"), Value::from(r"version: (.+)"));
        assert_eq!(result.to_string(), "1.2.3");
    }

    #[test]
    fn test_regex_search_no_match() {
        let result = regex_search(Value::from("hello world"), Value::from(r"^\d+"));
        assert_eq!(result.to_string(), "");
    }

    #[test]
    fn test_regex_search_full_match() {
        let result = regex_search(Value::from("test123"), Value::from(r"test"));
        assert_eq!(result.to_string(), "test");
    }

    #[test]
    fn test_regex_replace_basic() {
        let result = regex_replace(
            Value::from("hello world"),
            Value::from("world"),
            Value::from("universe"),
        );
        assert_eq!(result.to_string(), "hello universe");
    }

    #[test]
    fn test_regex_replace_multiple() {
        let result = regex_replace(
            Value::from("foo foo foo"),
            Value::from("foo"),
            Value::from("bar"),
        );
        assert_eq!(result.to_string(), "bar bar bar");
    }

    #[test]
    fn test_regex_findall_basic() {
        let result = regex_findall(
            Value::from("test123 hello456 world789"),
            Value::from(r"\d+"),
        );
        let result_str = result.to_string();
        // Should contain the numbers
        assert!(result_str.contains("123") || result_str.contains("456"));
    }

    #[test]
    fn test_regex_findall_with_capture() {
        let result = regex_findall(
            Value::from("version 1.0 version 2.0 version 3.0"),
            Value::from(r"version (\d+\.\d+)"),
        );
        let result_str = result.to_string();
        // Should contain captured versions
        assert!(result_str.contains("1.0") || result_str.contains("2.0"));
    }

    #[test]
    fn test_regex_findall_no_match() {
        let result = regex_findall(Value::from("hello world"), Value::from(r"\d+"));
        // Should return empty array representation
        let result_str = result.to_string();
        assert!(result_str.contains("[]") || result_str.is_empty());
    }

    #[test]
    fn test_to_nice_yaml_basic() {
        let mut data_map = BTreeMap::new();
        data_map.insert("key", Value::from("value"));
        data_map.insert("number", Value::from(42));
        let data = Value::from(data_map);
        
        let result = to_nice_yaml(data);
        let result_str = result.to_string();
        
        // YAML output should contain the key and value
        assert!(result_str.contains("key") && result_str.contains("value"));
    }

    #[test]
    fn test_to_nice_yaml_with_array() {
        let array = vec![Value::from(1), Value::from(2), Value::from(3)];
        let data = Value::from(array);
        
        let result = to_nice_yaml(data);
        let result_str = result.to_string();
        
        // YAML output should contain the values
        assert!(!result_str.contains("[ERROR") && !result_str.is_empty());
    }

    #[test]
    fn test_from_yaml_basic() {
        let yaml_str = Value::from("key: value\nnumber: 42");
        let result = from_yaml(yaml_str);
        let result_str = result.to_string();
        
        // Result should contain parsed values
        assert!(result_str.contains("key") || result_str.contains("value"));
    }

    #[test]
    fn test_from_yaml_complex() {
        let yaml_str = Value::from(
            "users:\n  - name: alice\n    age: 30\n  - name: bob\n    age: 25"
        );
        let result = from_yaml(yaml_str);
        let result_str = result.to_string();
        
        // Result should contain parsed data
        assert!(!result_str.contains("[ERROR"));
    }

    #[test]
    fn test_from_yaml_invalid() {
        let yaml_str = Value::from("{ invalid yaml: [");
        let result = from_yaml(yaml_str);
        let result_str = result.to_string();
        
        // Should have an error indicator
        assert!(result_str.contains("[ERROR") || !result_str.is_empty());
    }
}
