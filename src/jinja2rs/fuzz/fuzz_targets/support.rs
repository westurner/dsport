//! Shared generated inputs for the Jinja2rs fuzz targets.
//!
//! [`FuzzCase`] is deliberately serializable so the Rust targets can pass the
//! same nested context into `jinja2rs`, while the Python target converts it to
//! native Python objects. Template generation preserves a raw-input path for
//! parser robustness and supplies structured paths for deeper runtime coverage.

#![allow(dead_code)]

use std::collections::BTreeMap;

use arbitrary::Arbitrary;
use serde::Serialize;

#[derive(Debug, Arbitrary, Serialize)]
/// A recursively nested JSON-compatible value for template contexts.
//+
/// The Python bridge additionally places a `bytes` value beside this value so
/// its non-JSON fallback conversion path is exercised.
pub enum FuzzValue {
    /// A JSON null / Python `None` value.
    None,
    /// A boolean value.
    Bool(bool),
    /// A signed 64-bit integer.
    Integer(i64),
    /// An arbitrary floating-point value, including non-finite values.
    Float(f64),
    /// An arbitrary UTF-8 string.
    String(String),
    /// A recursively nested list.
    List(Vec<FuzzValue>),
    /// A recursively nested string-keyed map.
    Map(BTreeMap<String, FuzzValue>),
}

#[derive(Debug, Arbitrary)]
/// The input shared by the Rust and Python fuzzing targets.
///
/// `template_kind` selects between raw source and generated valid template
/// shapes. `django` selects the Django-compatible Rust environment, while
/// `sandbox` selects the sandbox environment in the sandbox and bridge targets.
pub struct FuzzCase {
    /// Arbitrary source text used directly or embedded in a generated template.
    pub template: String,
    /// Nested context data supplied to the renderer.
    pub context: FuzzValue,
    /// Selects `Environment::with_django_mode` for regular rendering.
    pub django: bool,
    /// Selects `SandboxedEnvironment` for sandbox and bridge rendering.
    pub sandbox: bool,
    /// Selects one of the generated template shapes.
    pub template_kind: u8,
}

/// Return either raw fuzz input or a structured template with valid syntax.
pub fn render_template(case: &FuzzCase) -> String {
    let literal = jinja_string_literal(&case.template);
    match case.template_kind % 6 {
        0 => case.template.clone(),
        1 => format!("{{{{ '{literal}' }}}}"),
        2 => format!("{{% if true %}}{{{{ '{literal}' }}}}{{% endif %}}"),
        3 => format!(
            "{{% for item in [1, 2, 3] %}}{{{{ item }}}}{{% endfor %}}{{{{ '{literal}' }}}}"
        ),
        4 => format!("{{{{ '{literal}'|escape }}}}"),
        _ if case.django => "{{ 'Hello World'|slugify }}".to_string(),
        _ => format!("{{{{ '{literal}'|length }}}}"),
    }
}

/// Return either raw fuzz input or a template targeting strict sandbox behavior.
pub fn sandbox_template(case: &FuzzCase) -> String {
    let literal = jinja_string_literal(&case.template);
    match case.template_kind % 5 {
        0 => case.template.clone(),
        1 => "{{ missing_value }}".to_string(),
        2 => "{{ sandbox_value.__class__ }}".to_string(),
        3 => "{% for item in [1, 2, 3] %}{{ item }}{% endfor %}".to_string(),
        _ => format!("{{{{ '{literal}' }}}}"),
    }
}

fn jinja_string_literal(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '\\' => "\\\\".to_string(),
            '\'' => "\\'".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            character if character.is_control() => " ".to_string(),
            character => character.to_string(),
        })
        .collect()
}
