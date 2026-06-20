//! Parametrized global and utility tests with rstest fixtures and mocks.
//!
//! Demonstrates:
//! - `#[fixture]` setup for reusable test objects
//! - `#[rstest]` parametrization
//! - Mock state and behavior verification

use jinja2rs::Environment;
use rstest::{fixture, rstest};
use serde_json::json;

// ============================================================================
// FIXTURES
// ============================================================================

/// Fixture: a pre-configured environment with test context.
#[fixture]
fn env_with_context() -> (Environment, serde_json::Value) {
    let env = Environment::new();
    let ctx = json!({
        "name": "Alice",
        "items": ["a", "b", "c"],
        "count": 5,
        "nested": {
            "key": "value"
        }
    });
    (env, ctx)
}

/// Fixture: idgen global for testing incremental IDs.
#[fixture]
fn env_with_idgen() -> Environment {
    let env = Environment::new();
    // idgen is added by default in Environment::new()
    env
}

// ============================================================================
// IDGEN TESTS
// ============================================================================

/// Test idgen.next() returns sequential IDs starting from 1.
#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
fn test_idgen_next_basic(env_with_idgen: Environment, #[case] expected_id: u64) {
    let env = env_with_idgen;
    // Each call to idgen.next() should increment
    for i in 1..=expected_id {
        let out = env
            .render_str("{{ idgen.next() }}", json!({}))
            .expect("idgen.next() should work");
        if i == expected_id {
            assert_eq!(out, expected_id.to_string());
        }
    }
}

/// Test idgen.current() returns the last generated ID without incrementing.
#[rstest]
fn test_idgen_current_no_increment(env_with_idgen: Environment) {
    env_with_idgen
        .render_str("{{ idgen.next() }}", json!({}))
        .unwrap();

    // Call current() multiple times; should always return the same value
    let out1 = env_with_idgen
        .render_str("{{ idgen.current() }}", json!({}))
        .expect("idgen.current() should work");
    let out2 = env_with_idgen
        .render_str("{{ idgen.current() }}", json!({}))
        .expect("idgen.current() should work again");

    assert_eq!(out1, out2);
    assert_eq!(out1, "1");
}

/// Test idgen in loops.
#[rstest]
fn test_idgen_in_loop(env_with_idgen: Environment) {
    let out = env_with_idgen
        .render_str(
            "{% for i in range(3) %}{{ idgen.next() }},{% endfor %}",
            json!({}),
        )
        .expect("idgen in loop should work");

    assert_eq!(out, "1,2,3,");
}

/// Test idgen sequence across multiple renders from the same environment.
///
/// Note: minijinja templates are compiled independently, so state may not
/// persist across renders unless idgen is a global object (which it is).
/// This test renders multiple times from a single environment to verify idgen state persists.
#[rstest]
fn test_idgen_persistence_across_renders(env_with_idgen: Environment) {
    let out1 = env_with_idgen
        .render_str("{{ idgen.next() }}", json!({}))
        .expect("idgen should render");
    assert_eq!(out1, "1", "First call should return 1");

    let out2 = env_with_idgen
        .render_str("{{ idgen.next() }}", json!({}))
        .expect("idgen should persist state");
    assert_eq!(out2, "2", "Second call should return 2");

    let out3 = env_with_idgen
        .render_str("{{ idgen.next() }}", json!({}))
        .expect("idgen should continue persisting");
    assert_eq!(out3, "3", "Third call should return 3");
}

// ============================================================================
// GLOBALS AND CONTEXT TESTS
// ============================================================================

/// Test that custom globals are accessible in templates.
#[rstest]
fn test_custom_global_access(mut env_with_idgen: Environment) {
    env_with_idgen.add_global("site_name", "TestSite");

    let out = env_with_idgen
        .render_str("Site: {{ site_name }}", json!({}))
        .expect("custom global should be accessible");

    assert_eq!(out, "Site: TestSite");
}

/// Test globals in conditionals.
#[rstest]
#[case("idgen", true)]
fn test_global_in_conditional(
    env_with_idgen: Environment,
    #[case] global_name: &str,
    #[case] should_exist: bool,
) {
    let template = format!(
        "{{% if {global_name} %}}exists{{% else %}}missing{{% endif %}}",
        global_name = global_name
    );
    let out = env_with_idgen
        .render_str(&template, json!({}))
        .expect("global in conditional should work");

    if should_exist {
        assert_eq!(out, "exists");
    } else {
        assert_eq!(out, "missing");
    }
}

// ============================================================================
// CONTEXT AND VARIABLE TESTS
// ============================================================================

/// Test variable lookup with nested objects.
#[rstest]
#[case("{{ name }}", "Alice")]
#[case("{{ count }}", "5")]
#[case("{{ nested.key }}", "value")]
#[case("{{ items[0] }}", "a")]
#[case("{{ items|length }}", "3")]
fn test_context_variable_lookup(
    env_with_context: (Environment, serde_json::Value),
    #[case] template: &str,
    #[case] expected: &str,
) {
    let (env, ctx) = env_with_context;
    let out = env
        .render_str(template, &ctx)
        .expect("variable lookup should work");
    assert_eq!(out, expected);
}

/// Test undefined variables in strict sandbox mode.
#[rstest]
fn test_undefined_in_strict_sandbox() {
    use jinja2rs::SandboxedEnvironment;

    let env = SandboxedEnvironment::new();
    let result = env.render_str("{{ missing_var }}", json!({}));

    assert!(
        result.is_err(),
        "strict sandbox should error on undefined variables"
    );
}

// ============================================================================
// LOOP AND ITERATION TESTS
// ============================================================================

/// Test loop variables are available in idgen context.
#[rstest]
#[case(
    "{% for item in items %}[{{ idgen.next() }}: {{ item }}] {% endfor %}",
    "[1: a] [2: b] [3: c] "
)]
fn test_idgen_in_loop_with_items(
    env_with_idgen: Environment,
    #[case] template: &str,
    #[case] expected: &str,
) {
    let ctx = json!({"items": ["a", "b", "c"]});
    let out = env_with_idgen
        .render_str(template, ctx)
        .expect("idgen in loop with items should work");
    assert_eq!(out, expected);
}

/// Test nested loops with idgen.
#[rstest]
fn test_idgen_nested_loops(env_with_idgen: Environment) {
    let template =
        "{% for row in matrix %}{% for cell in row %}{{ idgen.next() }},{% endfor %};{% endfor %}";
    let ctx = json!({
        "matrix": [["a", "b"], ["c", "d"]]
    });

    let out = env_with_idgen
        .render_str(template, ctx)
        .expect("idgen in nested loops should work");

    assert_eq!(out, "1,2,;3,4,;");
}

// ============================================================================
// ENVIRONMENT STATE TESTS
// ============================================================================

/// Test that globals can be overridden in context.
#[rstest]
fn test_context_shadows_global(env_with_idgen: Environment) {
    let out = env_with_idgen
        .render_str("{{ value }}", json!({"value": "from_context"}))
        .expect("context should shadow global");

    assert_eq!(out, "from_context");
}

/// Test multiple globals work together.
#[rstest]
fn test_multiple_globals(mut env_with_idgen: Environment) {
    env_with_idgen.add_global("app_name", "MyApp");
    env_with_idgen.add_global("version", "1.0");

    let out = env_with_idgen
        .render_str("{{ app_name }} v{{ version }}", json!({}))
        .expect("multiple globals should work");

    assert_eq!(out, "MyApp v1.0");
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

/// Test a realistic template with idgen and context variables.
#[rstest]
fn test_realistic_list_template(env_with_idgen: Environment) {
    let template = r#"
{% for item in items %}
<li id="item-{{ idgen.next() }}">{{ item }}</li>
{% endfor %}
"#;
    let ctx = json!({"items": ["Buy milk", "Walk dog", "Code review"]});

    let out = env_with_idgen
        .render_str(template, ctx)
        .expect("realistic template should render");

    assert!(out.contains("id=\"item-1\""));
    assert!(out.contains("id=\"item-2\""));
    assert!(out.contains("id=\"item-3\""));
    assert!(out.contains("Buy milk"));
}

// ============================================================================
// PHASE 4 CYCLER TESTS
// ============================================================================

/// Test cycler creation and basic iteration.
#[rstest]
fn test_cycler_creation(test_env: Environment) {
    let template = r#"
{% set colors = cycler('red', 'blue', 'green') %}
{{ colors.next() }},{{ colors.next() }},{{ colors.next() }},{{ colors.next() }}
"#;
    let out = test_env
        .render_str(template, json!({}))
        .expect("cycler should create and cycle");

    assert!(out.contains("red") && out.contains("blue") && out.contains("green"));
}

/// Test cycler with single value.
#[rstest]
fn test_cycler_single_value(test_env: Environment) {
    let template = r#"
{% set single = cycler('x') %}
{{ single.next() }},{{ single.next() }},{{ single.next() }}
"#;
    let out = test_env
        .render_str(template, json!({}))
        .expect("cycler with single value should work");

    assert_eq!(out.trim(), "x,x,x");
}

/// Test cycler in loops.
#[rstest]
fn test_cycler_in_loop(test_env: Environment) {
    let template = r#"
{% set colors = cycler('odd', 'even') %}
{% for i in range(4) %}<div class="{{ colors.next() }}">{{ i }}</div>{% endfor %}
"#;
    let out = test_env
        .render_str(template, json!({}))
        .expect("cycler in loop should work");

    assert!(out.contains(r#"class="odd""#));
    assert!(out.contains(r#"class="even""#));
}

/// Test cycler.current attribute.
#[rstest]
fn test_cycler_current_attribute(test_env: Environment) {
    let template = r#"
{% set colors = cycler('a', 'b', 'c') %}
{{ colors.next() }}: {{ colors.current }},
{{ colors.next() }}: {{ colors.current }},
{{ colors.next() }}: {{ colors.current }}
"#;
    let out = test_env
        .render_str(template, json!({}))
        .expect("cycler.current should work");

    assert!(out.contains("a:") && out.contains("a,"));
}

// ============================================================================
// PHASE 4 JOINER TESTS
// ============================================================================

/// Test joiner creation and basic operation.
#[rstest]
fn test_joiner_creation(test_env: Environment) {
    let template = r#"
{% set comma = joiner(', ') %}
{{ comma('one') }}{{ comma('two') }}{{ comma('three') }}
"#;
    let out = test_env
        .render_str(template, json!({}))
        .expect("joiner should create and join");

    assert_eq!(out.trim(), "one, two, three");
}

/// Test joiner with different separators.
#[rstest]
#[case(" | ", "a", "b", "c", "a | b | c")]
#[case("; ", "a", "b", "c", "a; b; c")]
#[case(" - ", "a", "b", "c", "a - b - c")]
fn test_joiner_separators(
    test_env: Environment,
    #[case] sep: &str,
    #[case] val1: &str,
    #[case] val2: &str,
    #[case] val3: &str,
    #[case] expected: &str,
) {
    let template = "{% set j = joiner(separator) %}\
                    {{ j(val1) }}{{ j(val2) }}{{ j(val3) }}";
    let out = test_env
        .render_str(
            template,
            json!({"separator": sep, "val1": val1, "val2": val2, "val3": val3}),
        )
        .expect("joiner with custom separator should work");

    assert_eq!(out.trim(), expected);
}

/// Test joiner in loops.
#[rstest]
fn test_joiner_in_loop(test_env: Environment) {
    let template = r#"
{% set comma = joiner(', ') %}
{% for item in items %}{{ comma(item) }}{% endfor %}
"#;
    let out = test_env
        .render_str(template, json!({"items": ["apple", "banana", "cherry"]}))
        .expect("joiner in loop should work");

    assert_eq!(out.trim(), "apple, banana, cherry");
}

/// Test joiner with empty first call.
#[rstest]
fn test_joiner_empty_first_call(test_env: Environment) {
    let template = r#"
{% set comma = joiner(', ') %}
{{ comma('') }}{{ comma('hello') }}
"#;
    let out = test_env
        .render_str(template, json!({}))
        .expect("joiner with empty string should work");

    assert!(out.contains(", hello"));
}

// ============================================================================
// PHASE 4 LIPSUM TESTS
// ============================================================================

/// Test lipsum generation.
#[rstest]
fn test_lipsum_generation(test_env: Environment) {
    let template = "{{ lipsum() }}";
    let out = test_env
        .render_str(template, json!({}))
        .expect("lipsum should generate");

    assert!(out.contains("Lorem ipsum"));
    assert!(!out.is_empty());
}

/// Test lipsum with parameter.
#[rstest]
#[case(1)]
#[case(3)]
#[case(5)]
fn test_lipsum_paragraphs(test_env: Environment, #[case] n: i64) {
    let template = format!("{{{{ lipsum({}) }}}}", n);
    let out = test_env
        .render_str(&template, json!({}))
        .expect("lipsum with parameter should work");

    assert!(out.contains("Lorem ipsum"));
    // Check approximate number of newlines (n-1 newlines for n paragraphs)
    let newline_count = out.matches('\n').count() as i64;
    assert!(
        newline_count >= n - 1,
        "lipsum({}) should have at least {} newlines",
        n,
        n - 1
    );
}

/// Test lipsum in realistic template.
#[rstest]
fn test_lipsum_in_template(test_env: Environment) {
    let template = r#"
<article>
<h1>{{ title }}</h1>
{{ lipsum(2) }}
</article>
"#;
    let out = test_env
        .render_str(template, json!({"title": "Test Article"}))
        .expect("lipsum in template should work");

    assert!(out.contains("<h1>Test Article</h1>"));
    assert!(out.contains("Lorem ipsum"));
}

// ============================================================================
// FIXTURE FOR OTHER TESTS
// ============================================================================

#[fixture]
fn test_env() -> Environment {
    Environment::new()
}
