//! `jinja2rs::filters` — Sphinx built-in Jinja2 filters.
//!
//! Ports the filter functions from `sphinx.jinja2glue`:
//! - `tobool` — coerce string/value to `bool`
//! - `toint` — coerce to `i64`, defaulting to 0
//! - `todim` — format a value as a CSS dimension (`px`)
//! - `slice_index` — partition index entries into N equal columns

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
}
