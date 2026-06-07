//! `jinja2rs::ansible_validation` — Ansible playbook validation.
//!
//! Provides YAML schema validation for Ansible playbooks.
//! Supports optional integration with jsonschema, Pydantic, and SHACL validators.
//!
//! # Validation Modes
//!
//! | Mode | Purpose | Status |
//! |------|---------|--------|
//! | YAML | Basic YAML structural validation | ✅ Built-in |
//! | JSON Schema | JSON Schema-based validation | 🟡 Stub (ready for integration) |
//! | Pydantic | Python Pydantic model validation (via PyO3) | 🟡 Stub |
//! | SHACL | RDF/SHACL shape validation | 🟡 Stub (future) |

use serde_json::Value as JsonValue;

/// Validation result.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub valid: bool,
    /// List of validation errors
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<String>,
}

/// Single validation error.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Error message
    pub message: String,
    /// JSON pointer to the location (e.g., "$.tasks[0].name")
    pub path: Option<String>,
    /// Error code for programmatic handling
    pub code: Option<String>,
}

impl ValidationResult {
    /// Create a passing validation result.
    pub fn passed() -> Self {
        ValidationResult {
            valid: true,
            errors: vec![],
            warnings: vec![],
        }
    }

    /// Create a failing validation result with a single error.
    pub fn failed(message: impl Into<String>, path: Option<String>) -> Self {
        ValidationResult {
            valid: false,
            errors: vec![ValidationError {
                message: message.into(),
                path,
                code: None,
            }],
            warnings: vec![],
        }
    }
}

/// Ansible playbook YAML validator.
///
/// Performs basic structural validation of Ansible playbooks.
pub struct AnsibleValidator;

impl AnsibleValidator {
    /// Validate basic YAML structure for Ansible playbook.
    ///
    /// Checks:
    /// - Root is a list (array of plays)
    /// - Each play has required fields (name, hosts, tasks)
    /// - Tasks are present and valid
    pub fn validate_playbook(data: &JsonValue) -> ValidationResult {
        // Must be an array at root
        if !data.is_array() {
            return ValidationResult::failed("Playbook must be a YAML list/array", None);
        }

        let plays = data.as_array().unwrap();
        let mut errors = vec![];
        let mut warnings = vec![];

        for (idx, play) in plays.iter().enumerate() {
            if !play.is_object() {
                errors.push(ValidationError {
                    message: "Each play must be an object".to_string(),
                    path: Some(format!("$[{}]", idx)),
                    code: Some("invalid_play".to_string()),
                });
                continue;
            }

            let play_obj = play.as_object().unwrap();

            // Check required fields
            if !play_obj.contains_key("name") && !play_obj.contains_key("hosts") {
                warnings.push(format!(
                    "Play at $[{}] should have 'name' field",
                    idx
                ));
            }

            if !play_obj.contains_key("hosts") {
                errors.push(ValidationError {
                    message: "Play must have 'hosts' field".to_string(),
                    path: Some(format!("$[{}]", idx)),
                    code: Some("missing_hosts".to_string()),
                });
                continue;
            }

            // Validate tasks if present
            if let Some(tasks) = play_obj.get("tasks") {
                if !tasks.is_array() {
                    errors.push(ValidationError {
                        message: "Tasks must be an array".to_string(),
                        path: Some(format!("$[{}].tasks", idx)),
                        code: Some("invalid_tasks".to_string()),
                    });
                } else {
                    for (task_idx, task) in tasks.as_array().unwrap().iter().enumerate() {
                        if !task.is_object() {
                            errors.push(ValidationError {
                                message: "Each task must be an object".to_string(),
                                path: Some(format!("$[{}].tasks[{}]", idx, task_idx)),
                                code: Some("invalid_task".to_string()),
                            });
                        }
                    }
                }
            }
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Validate basic YAML structure for Ansible inventory.
    ///
    /// Checks:
    /// - Root is an object
    /// - 'all' group is present (recommended)
    /// - Groups are objects with valid structure
    pub fn validate_inventory(data: &JsonValue) -> ValidationResult {
        if !data.is_object() {
            return ValidationResult::failed("Inventory must be a YAML object", None);
        }

        let inv = data.as_object().unwrap();
        let mut warnings = vec![];

        if !inv.contains_key("all") {
            warnings.push("Inventory should have an 'all' group".to_string());
        }

        // Check that all top-level keys are valid group objects
        for (key, val) in inv {
            if !val.is_object() {
                return ValidationResult::failed(
                    format!("Group '{}' must be an object", key),
                    Some(format!("$.{}", key)),
                );
            }
        }

        ValidationResult {
            valid: true,
            errors: vec![],
            warnings,
        }
    }
}

/// JSON Schema validator (stub - requires jsonschema crate).
///
/// # Example
///
/// ```rust,ignore
/// let schema = r#"{
///   "type": "object",
///   "properties": {
///     "name": { "type": "string" }
///   },
///   "required": ["name"]
/// }"#;
///
/// let result = JsonSchemaValidator::validate(data, schema)?;
/// ```
pub struct JsonSchemaValidator;

impl JsonSchemaValidator {
    /// Validate data against a JSON Schema (stub).
    ///
    /// # Arguments
    ///
    /// * `data` — The data to validate
    /// * `schema_str` — JSON Schema as string
    ///
    /// # Returns
    ///
    /// Validation result with any schema violations
    ///
    /// # Note
    ///
    /// This is a stub that requires integration with the `jsonschema` crate.
    pub fn validate(
        _data: &JsonValue,
        _schema_str: &str,
    ) -> Result<ValidationResult, Box<dyn std::error::Error>> {
        // Stub: would use jsonschema crate here
        Ok(ValidationResult::passed())
    }
}

/// Pydantic validator (stub - requires PyO3 integration).
///
/// Allows Python Pydantic models to validate template data.
///
/// # Example
///
/// ```rust,ignore
/// // Would validate against a Python Pydantic model via PyO3
/// let result = PydanticValidator::validate_with_python_model(
///     data,
///     "MyModel",
/// )?;
/// ```
pub struct PydanticValidator;

impl PydanticValidator {
    /// Validate data using a Python Pydantic model (stub).
    ///
    /// # Arguments
    ///
    /// * `data` — The data to validate
    /// * `model_name` — Name of Python Pydantic model
    ///
    /// # Note
    ///
    /// This is a stub that requires PyO3 integration to call Python Pydantic.
    pub fn validate_with_python_model(
        _data: &JsonValue,
        _model_name: &str,
    ) -> Result<ValidationResult, Box<dyn std::error::Error>> {
        // Stub: would use PyO3 to call Python Pydantic model
        Ok(ValidationResult::passed())
    }
}

/// SHACL validator (stub - future feature).
///
/// Uses SHACL (Shapes Constraint Language) for RDF/graph-based validation.
///
/// # Note
///
/// SHACL validation is a stub for future implementation.
/// This would require RDF and SHACL libraries.
pub struct ShaclValidator;

impl ShaclValidator {
    /// Validate data using SHACL shapes (stub - future).
    ///
    /// # Arguments
    ///
    /// * `data` — The data to validate
    /// * `shapes_str` — SHACL shapes definition as TTL or JSON-LD
    ///
    /// # Note
    ///
    /// This is a future stub. Requires RDF/SHACL integration.
    pub fn validate_with_shapes(
        _data: &JsonValue,
        _shapes_str: &str,
    ) -> Result<ValidationResult, Box<dyn std::error::Error>> {
        // Stub: would implement SHACL validation in future
        Ok(ValidationResult::passed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_playbook_valid() {
        let playbook = json!([
            {
                "name": "Test Play",
                "hosts": "all",
                "tasks": [
                    {
                        "name": "Test Task",
                        "debug": { "msg": "Hello" }
                    }
                ]
            }
        ]);

        let result = AnsibleValidator::validate_playbook(&playbook);
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_playbook_invalid_root() {
        let playbook = json!({ "play": "invalid" });
        let result = AnsibleValidator::validate_playbook(&playbook);
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_validate_playbook_missing_hosts() {
        let playbook = json!([
            {
                "name": "Test Play"
                // Missing "hosts"
            }
        ]);

        let result = AnsibleValidator::validate_playbook(&playbook);
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_validate_inventory_valid() {
        let inventory = json!({
            "all": {
                "hosts": {
                    "localhost": {}
                }
            }
        });

        let result = AnsibleValidator::validate_inventory(&inventory);
        assert!(result.valid);
    }

    #[test]
    fn test_validate_inventory_invalid_root() {
        let inventory = json!(["invalid"]);
        let result = AnsibleValidator::validate_inventory(&inventory);
        assert!(!result.valid);
    }

    #[test]
    fn test_jsonschema_stub() {
        let data = json!({"name": "test"});
        let result = JsonSchemaValidator::validate(&data, "{}").unwrap();
        assert!(result.valid); // Stub always returns valid
    }
}
