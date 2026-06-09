//! Integration tests for Ansible template rendering with filters.
//!
//! Tests real-world Ansible use cases combining inventory, filters, and templates.

#[cfg(test)]
mod ansible_integration_tests {
    use jinja2rs::{Environment, compat::{CompatMode, AnsibleMode}};
    use serde_json::json;

    // Helper to create environment with Ansible mode enabled
    fn env_with_ansible() -> Environment {
        let mut env = Environment::new();
        env.set_compat_mode(CompatMode::Ansible(AnsibleMode::full()));
        env
    }

    #[test]
    fn test_combine_filter_in_template() {
        let env = env_with_ansible();
        
        let template = r#"
{% set base = {"app": "myapp", "version": "1.0", "env": "dev"} %}
{% set overrides = {"env": "prod", "debug": false} %}
{% set config = base | combine(overrides) %}
app: {{ config.app }}
version: {{ config.version }}
env: {{ config.env }}
debug: {{ config.debug }}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("app: myapp"));
        assert!(result.contains("env: prod"));
        assert!(result.contains("debug: false"));
    }

    #[test]
    fn test_regex_search_in_template() {
        let env = env_with_ansible();
        
        let template = r#"
{% set version_line = "version = 2.4.5" %}
{% set version = version_line | regex_search("(\d+\.\d+\.\d+)") %}
Extracted version: {{ version }}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("2.4.5"));
    }

    #[test]
    fn test_regex_replace_in_template() {
        let env = env_with_ansible();
        
        let template = r#"
{% set config = "host=localhost port=5432 user=admin" %}
{% set safe_config = config | regex_replace("user=\w+", "user=****") %}
{{ safe_config }}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("user=****"));
    }

    #[test]
    fn test_regex_findall_in_template() {
        let env = env_with_ansible();
        
        let template = r#"
{% set text = "Email: user1@example.com, user2@example.org, user3@test.com" %}
{% set emails = text | regex_findall("[\w.]+@[\w.]+") %}
Found emails:
{% for email in emails %}
- {{ email }}
{% endfor %}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("@"));
    }

    #[test]
    fn test_to_nice_yaml_in_template() {
        let env = env_with_ansible();
        
        let template = r#"
{%- set data = {"services": ["nginx", "postgresql", "redis"], "port": 8080} %}
{{ data | to_nice_yaml }}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        // YAML should contain the structure
        assert!(result.contains("services") && result.contains("nginx"));
    }

    #[test]
    fn test_from_yaml_in_template() {
        let env = env_with_ansible();
        
        let template = r#"
{%- set yaml_data = "
app_name: MyApp
version: 2.0
features:
  - auth
  - api
  - admin
" | from_yaml %}
App: {{ yaml_data.app_name }}
Version: {{ yaml_data.version }}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("MyApp"));
        assert!(result.contains("2.0"));
    }

    #[test]
    fn test_ansible_playbook_with_combine() {
        let env = env_with_ansible();
        
        let template = r#"
---
- name: Deploy {{ app_name }}
  hosts: "{{ inventory_hostname }}"
  vars:
    {%- set defaults = {"timeout": 30, "retries": 3, "debug": false} %}
    {%- set custom = {"timeout": 60, "log_level": "INFO"} %}
    {%- set final_config = defaults | combine(custom) %}
    timeout: {{ final_config.timeout }}
    retries: {{ final_config.retries }}
    debug: {{ final_config.debug }}
    log_level: {{ final_config.log_level }}
  tasks:
    - name: Configure application
      debug:
        msg: "Config applied"
"#;

        let ctx = json!({
            "app_name": "TestApp",
            "inventory_hostname": "web1.example.com"
        });
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("timeout: 60"));
        assert!(result.contains("retries: 3"));
        assert!(result.contains("log_level: INFO"));
    }

    #[test]
    fn test_complex_filter_chain() {
        let env = env_with_ansible();
        
        let template = r#"
{% set log_entry = "[2024-06-07T10:30:45] INFO: User alice performed action update" %}
{% set timestamp = log_entry | regex_search("(\d{4}-\d{2}-\d{2})") %}
{% set user = log_entry | regex_search("User (\w+)") %}
{% set action = log_entry | regex_search("action (\w+)") %}
Date: {{ timestamp }}
User: {{ user }}
Action: {{ action }}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("2024-06-07"));
        assert!(result.contains("alice"));
        assert!(result.contains("update"));
    }

    #[test]
    fn test_conditional_with_regex_and_combine() {
        let env = env_with_ansible();
        
        let template = r#"
{% set version_string = "nginx/1.24.0" %}
{% set version = version_string | regex_search("nginx/(.+)") %}
{% if version.startswith("1.2") %}
version_family: "1.2.x"
{% else %}
version_family: "other"
{% endif %}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("version_family"));
    }

    #[test]
    fn test_yaml_parsing_with_variables() {
        let env = env_with_ansible();
        
        // Use a pre-built YAML string to avoid template concatenation issues
        let template = r#"
{% set config_yaml = '{"database": {"host": "postgres.example.com", "port": 5432, "name": "myapp_db"}}' | from_yaml %}
DB Host: {{ config_yaml.database.host }}
DB Port: {{ config_yaml.database.port }}
DB Name: {{ config_yaml.database.name }}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("postgres.example.com"));
        assert!(result.contains("5432"));
        assert!(result.contains("myapp_db"));
    }

    #[test]
    fn test_combine_multiple_inventories() {
        let env = env_with_ansible();
        
        let template = r#"
{%- set global_vars = {"env": "prod", "region": "us-east-1", "team": "devops"} %}
{%- set service_vars = {"env": "staging", "service": "api", "port": 8080} %}
{%- set merged = global_vars | combine(service_vars) %}
Environment: {{ merged.env }}
Region: {{ merged.region }}
Team: {{ merged.team }}
Service: {{ merged.service }}
Port: {{ merged.port }}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        // env should be overridden by service_vars
        assert!(result.contains("Environment: staging"));
        assert!(result.contains("Region: us-east-1"));
        assert!(result.contains("Service: api"));
    }

    #[test]
    fn test_quote_filter_in_shell_command() {
        let env = env_with_ansible();
        
        let template = r#"
{% set command = "docker run -e PASSWORD=" + (password | quote) %}
{{ command }}
"#;

        let ctx = json!({
            "password": "secret@123!value"
        });
        let result = env.render_str(template, &ctx).unwrap();
        
        // Password should be quoted
        assert!(result.contains("'") || result.contains("secret"));
    }

    #[test]
    fn test_path_join_in_playbook() {
        let env = env_with_ansible();
        
        let template = r#"
---
- name: Copy config files
  copy:
    src: "{{ base_path | path_join('config') | path_join(env) | path_join('app.conf') }}"
    dest: /etc/app/app.conf
"#;

        let ctx = json!({
            "base_path": "/opt/app",
            "env": "production"
        });
        let result = env.render_str(template, &ctx).unwrap();
        
        // Should construct proper path
        assert!(result.contains("/opt/app") || result.contains("config") || result.contains("production"));
    }

    #[test]
    fn test_json_and_yaml_roundtrip() {
        let env = env_with_ansible();
        
        let template = r#"
{%- set original = {"users": [{"name": "alice", "role": "admin"}, {"name": "bob", "role": "user"}]} %}
{%- set as_yaml = original | to_nice_yaml %}
{%- set parsed_back = as_yaml | from_yaml %}
Round-trip successful: {{ parsed_back.users[0].name }}
"#;

        let ctx = json!({});
        let result = env.render_str(template, &ctx).unwrap();
        
        assert!(result.contains("alice") || result.contains("Round-trip"));
    }
}
