#![allow(clippy::needless_borrows_for_generic_args)]


#[cfg(test)]
mod compat_mode_tests {
    use jinja2rs::{CompatMode, Environment};
    use serde_json::json;

    #[test]
    fn test_jinja2_compat_mode_dict_items() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str(
            "{% for k, v in user.items() %}{{ k }}:{{ v }},{% endfor %}",
            &json!({"user": {"name": "Alice", "age": 30}}),
        );

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("name:Alice") || output.contains("age:30"));
    }

    #[test]
    fn test_jinja2_compat_mode_dict_values() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str(
            "{% for v in user.values() %}{{ v }},{% endfor %}",
            &json!({"user": {"name": "Alice", "age": 30}}),
        );

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Alice") || output.contains("30"));
    }

    #[test]
    fn test_jinja2_compat_mode_dict_keys() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str(
            "{% for k in user.keys() %}{{ k }},{% endfor %}",
            &json!({"user": {"name": "Alice", "age": 30}}),
        );

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("name") || output.contains("age"));
    }

    #[test]
    fn test_jinja2_compat_mode_dict_get() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str(
            "{{ user.get('name', 'Unknown') }}",
            &json!({"user": {"name": "Alice"}}),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Alice");
    }

    #[test]
    fn test_jinja2_compat_mode_string_upper() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str("{{ name.upper() }}", &json!({"name": "alice"}));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ALICE");
    }

    #[test]
    fn test_jinja2_compat_mode_string_lower() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str("{{ name.lower() }}", &json!({"name": "ALICE"}));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "alice");
    }

    #[test]
    fn test_jinja2_compat_mode_string_split() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str(
            "{% for part in text.split(',') %}{{ part }};{% endfor %}",
            &json!({"text": "a,b,c"}),
        );

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("a") && output.contains("b") && output.contains("c"));
    }

    #[test]
    fn test_jinja2_compat_mode_string_replace() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str("{{ text.replace('a', 'X') }}", &json!({"text": "banana"}));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "bXnXnX");
    }

    #[test]
    fn test_minijinja_compat_mode_no_dict_items() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Minijinja);

        // In minijinja mode, .items() should not work
        let result = env.render_str(
            "{% for k, v in user.items() %}{{ k }}:{{ v }},{% endfor %}",
            &json!({"user": {"name": "Alice", "age": 30}}),
        );

        // Should fail because .items() is not available
        assert!(result.is_err());
    }

    #[test]
    fn test_minijinja_compat_mode_use_filter_items() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Minijinja);

        // In minijinja mode, use |items filter instead
        let result = env.render_str(
            "{% for item in user | items %}{{ item[0] }}:{{ item[1] }},{% endfor %}",
            &json!({"user": {"name": "Alice", "age": 30}}),
        );

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("name:Alice") || output.contains("age:30"));
    }

    #[test]
    fn test_minijinja_compat_mode_no_string_upper() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Minijinja);

        // In minijinja mode, .upper() should not work
        let result = env.render_str("{{ name.upper() }}", &json!({"name": "alice"}));

        // Should fail because .upper() is not available
        assert!(result.is_err());
    }

    #[test]
    fn test_minijinja_compat_mode_use_filter_upper() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Minijinja);

        // In minijinja mode, use |upper filter instead
        let result = env.render_str("{{ name | upper }}", &json!({"name": "alice"}));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ALICE");
    }

    #[test]
    fn test_enable_jinja2_compat_explicitly() {
        let mut env = Environment::new();
        env.enable_jinja2_compat();

        let result = env.render_str("{{ text.upper() }}", &json!({"text": "hello"}));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[test]
    fn test_enable_minijinja_compat_explicitly() {
        let mut env = Environment::new();
        env.enable_minijinja_compat();

        // minijinja mode doesn't support method calls
        let result = env.render_str("{{ text.upper() }}", &json!({"text": "hello"}));

        assert!(result.is_err());
    }

    #[test]
    fn test_compat_mode_equality() {
        assert_eq!(CompatMode::Jinja2, CompatMode::Jinja2);
        assert_eq!(CompatMode::Minijinja, CompatMode::Minijinja);
        assert_ne!(CompatMode::Jinja2, CompatMode::Minijinja);
    }

    #[test]
    fn test_compat_mode_default_is_jinja2() {
        assert_eq!(CompatMode::default(), CompatMode::Jinja2);
    }

    #[test]
    fn test_compat_mode_is_checks() {
        assert!(CompatMode::Jinja2.is_jinja2());
        assert!(!CompatMode::Jinja2.is_minijinja());

        assert!(CompatMode::Minijinja.is_minijinja());
        assert!(!CompatMode::Minijinja.is_jinja2());
    }

    #[test]
    fn test_jinja2_compat_mode_chained_methods() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str(
            "{{ text.lower().replace('a', 'X') }}",
            &json!({"text": "BANANA"}),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "bXnXnX");
    }

    #[test]
    fn test_jinja2_compat_mode_list_count() {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Jinja2);

        let result = env.render_str(
            "{{ items.count(2) }}",
            &json!({"items": [1, 2, 3, 2, 4, 2]}),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3");
    }
}
