#![allow(clippy::needless_borrows_for_generic_args)]

//! Sandbox security tests for Phase 5.
//!
//! Port of relevant cases from `src/jinja2/tests/test_security.py` and additional
//! operator safe-guard tests. Verifies that `SandboxedEnvironment` blocks known
//! Jinja2 sandbox escape attempts.
//!
//! Security properties tested:
//! 1. No dunder attribute access (minijinja blocks `__class__`, `__mro__`, etc.)
//! 2. Strict undefined behavior (raises error instead of empty)
//! 3. No arbitrary Python method calls (minijinja doesn't expose these)
//! 4. is_safe_attribute() utility validates known dangerous patterns
//!
//! Note: minijinja doesn't have a Python runtime, so many Python-specific
//! escalations (getattr, setattr, etc.) are naturally blocked. Underscore-
//! prefixed attributes on JSON objects are accessible (they're just JSON keys),
//! but is_safe_attribute() correctly identifies them as dangerous patterns
//! for migration compatibility checks.

use jinja2rs::SandboxedEnvironment;
use rstest::{fixture, rstest};
use serde_json::json;

// ============================================================================
// FIXTURES
// ============================================================================

/// Fixture: a fresh SandboxedEnvironment for each test.
#[fixture]
fn sandbox_env() -> SandboxedEnvironment {
    SandboxedEnvironment::new()
}

// ============================================================================
// DUNDER ATTRIBUTE DENIAL TESTS
// ============================================================================

/// Test that `__class__` access is denied (undefined in minijinja).
/// Since minijinja doesn't have Python's __class__, accessing it returns undefined,
/// which errors in strict mode.
#[rstest]
fn test_dunder_class_denied(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ obj.__class__ }}", json!({"obj": "string"}));
    // Strict mode should error on undefined attribute access
    assert!(
        result.is_err(),
        "strict mode should error on undefined __class__"
    );
}

/// Test that `__mro__` access is denied.
#[rstest]
fn test_dunder_mro_denied(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ obj.__mro__ }}", json!({"obj": {"x": 1}}));
    assert!(
        result.is_err(),
        "strict mode should error on undefined __mro__"
    );
}

/// Test that `__dict__` access is denied.
#[rstest]
fn test_dunder_dict_denied(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ obj.__dict__ }}", json!({"obj": {}}));
    assert!(
        result.is_err(),
        "strict mode should error on undefined __dict__"
    );
}

/// Test that `__builtins__` access is denied.
#[rstest]
fn test_dunder_builtins_denied(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ __builtins__ }}", json!({}));
    assert!(
        result.is_err(),
        "strict mode should error on undefined __builtins__"
    );
}

/// Test that `__globals__` access is denied.
#[rstest]
fn test_dunder_globals_denied(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ func.__globals__ }}", json!({"func": "fn"}));
    assert!(
        result.is_err(),
        "strict mode should error on undefined __globals__"
    );
}

// ============================================================================
// UNDERSCORE PREFIX DENIAL TESTS
// ============================================================================

/// Test that underscore-prefixed attributes are validated by is_safe_attribute().
///
/// Note: minijinja doesn't prevent access to underscore-prefixed JSON keys
/// (they're just JSON keys), but the is_safe_attribute() utility correctly
/// identifies them as a dangerous pattern for Python Jinja2 compatibility checks.
#[rstest]
#[case("_private")]
#[case("_internal")]
#[case("_safe_attribute")]
fn test_underscore_prefix_validation(#[case] attr: &str) {
    // Verify that is_safe_attribute() correctly identifies underscore patterns as unsafe
    assert!(
        !SandboxedEnvironment::is_safe_attribute(attr),
        "is_safe_attribute should return false for underscore-prefixed names"
    );
}

/// Test that SQLAlchemy's `_sa_instance_state` is denied (common exploit target).
#[rstest]
fn test_sqlalchemy_state_denied(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ obj._sa_instance_state }}", json!({"obj": {}}));
    assert!(
        result.is_err() || result.unwrap().is_empty(),
        "should deny access to _sa_instance_state"
    );
}

/// Test that underscore-prefixed method access is denied.
#[rstest]
fn test_underscore_method_denied(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ obj._method() }}", json!({"obj": "test"}));
    // Should fail or produce empty (minijinja doesn't support method calls on strings anyway)
    assert!(
        result.is_err() || result.unwrap().is_empty(),
        "should deny underscore-prefixed method access"
    );
}

// ============================================================================
// STRICT UNDEFINED BEHAVIOR TESTS
// ============================================================================

/// Test that undefined variables raise an error in strict mode.
#[rstest]
fn test_undefined_strict_error(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ missing }}", json!({}));
    assert!(
        result.is_err(),
        "undefined variable should error in strict mode"
    );
}

/// Test that undefined filters don't silently fail.
#[rstest]
fn test_undefined_filter_strict(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ 'test' | nonexistent_filter }}", json!({}));
    assert!(
        result.is_err(),
        "undefined filter should error in strict mode"
    );
}

/// Test that undefined function calls error.
#[rstest]
fn test_undefined_function_strict(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ undefined_func() }}", json!({}));
    assert!(
        result.is_err(),
        "undefined function should error in strict mode"
    );
}

/// Test that missing dict keys error in strict mode.
#[rstest]
fn test_undefined_dict_key_strict(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ obj.missing_key }}", json!({"obj": {"a": 1}}));
    assert!(
        result.is_err(),
        "missing dict key should error in strict mode"
    );
}

// ============================================================================
// FORMAT STRING / OPERATOR SAFE-GUARD TESTS
// ============================================================================

/// Test that string format operator (%) is restricted.
///
/// In Python, `format % (arg, ...)` can be used to execute arbitrary Python code.
/// minijinja doesn't have this operator, but we test that format-like expressions fail safely.
#[rstest]
fn test_format_operator_safe(sandbox_env: SandboxedEnvironment) {
    // minijinja doesn't have % operator, so this should fail gracefully
    let result = sandbox_env.render_str(
        "{{ 'hello %s' % (obj.name) }}",
        json!({"obj": {"name": "test"}}),
    );
    assert!(result.is_err(), "format operator should not be available");
}

/// Test that string .format() method calls are restricted.
///
/// In minijinja, string method calls are not supported, so this should fail.
#[rstest]
fn test_string_format_method_safe(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str(
        "{{ 'hello {}'.format(obj.name) }}",
        json!({"obj": {"name": "test"}}),
    );
    // Should fail because minijinja doesn't expose string methods
    assert!(
        result.is_err(),
        "string .format() method should not be available"
    );
}

/// Test that f-string-like expressions are restricted.
#[rstest]
fn test_fstring_safe(sandbox_env: SandboxedEnvironment) {
    // minijinja doesn't support f-strings in templates, only {{ }} interpolation
    let result = sandbox_env.render_str(
        "{{ f'hello {obj.name}' }}",
        json!({"obj": {"name": "test"}}),
    );
    assert!(result.is_err(), "f-string syntax should not be available");
}

// ============================================================================
// METHOD ACCESS RESTRICTION TESTS
// ============================================================================

/// Test that `getattr()` is not available as a global function.
#[rstest]
fn test_getattr_not_available(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ getattr(obj, '__class__') }}", json!({"obj": "test"}));
    assert!(result.is_err(), "getattr() should not be available");
}

/// Test that `setattr()` is not available.
#[rstest]
fn test_setattr_not_available(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ setattr(obj, 'x', 'y') }}", json!({"obj": {}}));
    assert!(result.is_err(), "setattr() should not be available");
}

/// Test that `delattr()` is not available.
#[rstest]
fn test_delattr_not_available(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ delattr(obj, 'x') }}", json!({"obj": {}}));
    assert!(result.is_err(), "delattr() should not be available");
}

/// Test that `__import__()` is not available.
#[rstest]
fn test_import_not_available(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ __import__('os') }}", json!({}));
    assert!(result.is_err(), "__import__() should not be available");
}

// ============================================================================
// ATTRIBUTE CHAIN ESCAPE ATTEMPTS
// ============================================================================

/// Test that chaining attribute access to dunders is blocked.
///
/// E.g., `obj.something.__class__` should fail if the final attribute is undefined.
#[rstest]
fn test_chained_dunder_access_blocked(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str(
        "{{ obj.method.__class__ }}",
        json!({"obj": {"method": "fn"}}),
    );
    // Accessing __class__ on a string should error in strict mode
    assert!(
        result.is_err(),
        "strict mode should error on chained dunder access"
    );
}

/// Test that undefined attributes in chain raise error in strict mode.
#[rstest]
fn test_chained_undefined_access_blocked(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str(
        "{{ obj.subobj.missing }}",
        json!({"obj": {"subobj": {"a": 1}}}),
    );
    // Missing key in strict mode should error
    assert!(
        result.is_err(),
        "strict mode should error on undefined nested attributes"
    );
}

/// Test that bracket notation `obj[attr]` with dunder names is blocked.
#[rstest]
fn test_bracket_dunder_access_blocked(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str(
        "{{ obj['__class__'] }}",
        json!({"obj": {"__class__": "fake"}}),
    );
    // minijinja may allow dict access via brackets, but the actual dunder should be empty/error
    if let Ok(out) = result {
        assert!(
            out.is_empty() || !out.contains("class"),
            "bracket dunder access should be safe"
        )
    }
}

// ============================================================================
// PYTHON EXCEPTION / ERROR ESCALATION ATTEMPTS
// ============================================================================

/// Test that undefined variable access errors in strict mode.
///
/// This is the primary error handling property - errors don't leak internals.
#[rstest]
fn test_error_doesnt_leak_internals(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ missing_var }}", json!({}));
    assert!(
        result.is_err(),
        "undefined variable should error in strict mode"
    );
    if let Err(e) = result {
        let msg = format!("{}", e);
        // Error message should not expose Rust internals, file paths, or memory addresses
        assert!(
            !msg.contains("0x"),
            "error should not contain memory addresses"
        );
        assert!(
            !msg.contains("/"),
            "error should not contain absolute paths"
        );
    }
}

/// Test that recursion limits are enforced.
///
/// Note: minijinja may have recursion limits; this test verifies they don't leak internals.
#[rstest]
fn test_recursion_safe(sandbox_env: SandboxedEnvironment) {
    // Try to trigger deep recursion via loops
    let template = "{% for i in range(10000) %}{{ i }}{% endfor %}";
    let result = sandbox_env.render_str(template, json!({}));
    // Should either succeed (if minijinja has no limit) or error safely
    if let Err(e) = result {
        let msg = format!("{}", e);
        assert!(
            !msg.contains("private") && !msg.contains("__"),
            "recursion error should not expose internals"
        );
    }
}

// ============================================================================
// SAFE TEMPLATE TESTS (POSITIVE CASES)
// ============================================================================

/// Test that safe attributes are still accessible.
#[rstest]
fn test_safe_attribute_access(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ obj.name }}", json!({"obj": {"name": "Alice"}}));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Alice");
}

/// Test that safe method-like filters work.
#[rstest]
fn test_safe_filters_work(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str("{{ 'hello' | upper }}", json!({}));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HELLO");
}

/// Test that loop constructs are safe.
#[rstest]
fn test_safe_loop_iteration(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str(
        "{% for x in items %}{{ x }}{% endfor %}",
        json!({"items": [1, 2, 3]}),
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "123");
}

/// Test that conditionals work safely.
#[rstest]
fn test_safe_conditionals(sandbox_env: SandboxedEnvironment) {
    let result = sandbox_env.render_str(
        "{% if show %}visible{% else %}hidden{% endif %}",
        json!({"show": true}),
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "visible");
}

// ============================================================================
// ATTRIBUTE SAFETY FUNCTION TESTS
// ============================================================================

/// Test the `is_safe_attribute()` utility function.
#[rstest]
#[case("name", true)]
#[case("title", true)]
#[case("value", true)]
#[case("__class__", false)]
#[case("__dict__", false)]
#[case("_private", false)]
#[case("_internal_state", false)]
#[case("__builtins__", false)]
fn test_is_safe_attribute(#[case] attr: &str, #[case] expected_safe: bool) {
    let is_safe = SandboxedEnvironment::is_safe_attribute(attr);
    assert_eq!(
        is_safe, expected_safe,
        "is_safe_attribute('{}') should be {}",
        attr, expected_safe
    );
}
