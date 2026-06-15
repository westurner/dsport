# Django Mode Usage Guide

## Quick Start

### Basic Setup

```rust
use jinja2rs::Environment;
use jinja2rs::compat::DjangoMode;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Django-compatible environment
    let django_mode = DjangoMode::default();
    let mut env = Environment::with_django_mode(django_mode)?;

    // Add a template
    env.add_template("hello.html", 
        "Hello, {{ name|upper }}!")?;

    // Render
    let tmpl = env.get_template("hello.html")?;
    let output = tmpl.render(json!({"name": "django"}))?;
    
    assert_eq!(output, "Hello, DJANGO!");
    println!("{}", output);
    
    Ok(())
}
```

## Filter Examples

### String Filters

```rust
let env = Environment::with_django_mode(DjangoMode::default())?;

// upper / lower
env.render_str("{{ text|upper }}", json!({"text": "hello"}))?;
// Output: "HELLO"

// slugify - convert to URL-safe slug
env.render_str("{{ title|slugify }}", json!({
    "title": "Hello World!"
}))?;
// Output: "hello-world"

// truncatewords - limit to N words
env.render_str("{{ text|truncatewords:2 }}", json!({
    "text": "The quick brown fox jumps"
}))?;
// Output: "The quick ..."

// truncatechars - limit to N characters
env.render_str("{{ text|truncatechars:10 }}", json!({
    "text": "Hello, world!"
}))?;
// Output: "Hello, ..."

// wordwrap - wrap at N characters per line
env.render_str("{{ text|wordwrap:20 }}", json!({
    "text": "This is a very long line of text that needs wrapping"
}))?;
// Output includes line breaks
```

### Numeric Filters

```rust
// add - add number to value
env.render_str("{{ count|add:5 }}", json!({"count": 10}))?;
// Output: "15"

// floatformat - format float to N decimal places
env.render_str("{{ pi|floatformat:2 }}", json!({"pi": 3.14159}))?;
// Output: "3.14"

// floatformat:-1 - round to nearest integer
env.render_str("{{ value|floatformat:-1 }}", json!({"value": 3.14159}))?;
// Output: "3"

// pluralize - add suffix if count != 1
env.render_str("{{ count }} item{{ count|pluralize }}", json!({"count": 5}))?;
// Output: "5 items"

// pluralize with custom suffix
env.render_str("{{ count }} cand{{ count|pluralize:'y,ies' }}", 
    json!({"count": 3}))?;
// Output: "3 candies"
```

### List Filters

```rust
// first / last
env.render_str("{{ items|first }}", json!({
    "items": [1, 2, 3]
}))?;
// Output: "1"

// join - join list items with separator
env.render_str("{{ items|join:', ' }}", json!({
    "items": ["apple", "banana", "cherry"]
}))?;
// Output: "apple, banana, cherry"

// length - get length
env.render_str("{{ items|length }}", json!({
    "items": [1, 2, 3, 4, 5]
}))?;
// Output: "5"
```

### Boolean Filters

```rust
// yesno - convert boolean to text
env.render_str("{{ is_active|yesno }}", json!({"is_active": true}))?;
// Output: "yes"

// yesno with custom mapping
env.render_str("{{ is_active|yesno:'on,off' }}", json!({"is_active": false}))?;
// Output: "off"

// yesno with three values (for None/null)
env.render_str("{{ value|yesno:'yes,no,maybe' }}", json!({"value": null}))?;
// Output: "maybe"

// default - use fallback if empty
env.render_str("{{ name|default:'Anonymous' }}", json!({"name": ""}))?;
// Output: "Anonymous"

// escape - HTML escape
env.render_str("{{ text|escape }}", json!({
    "text": "<script>alert('xss')</script>"
}))?;
// Output: "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"

// safe - mark as safe (no escaping)
// Note: Applied automatically for HTML templates
```

## Template Inheritance

```rust
let mut env = Environment::with_django_mode(DjangoMode::default())?;

// Base template
env.add_template("base.html", r#"
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}My Site{% endblock %}</title>
</head>
<body>
    {% block content %}{% endblock %}
</body>
</html>
"#)?;

// Child template
env.add_template("page.html", r#"
{% extends "base.html" %}
{% block title %}My Page{% endblock %}
{% block content %}
    <h1>Welcome!</h1>
    <p>This is my page.</p>
{% endblock %}
"#)?;

let tmpl = env.get_template("page.html")?;
let output = tmpl.render(json!({}))?;
// Output includes full HTML structure from base.html
```

## File-based Loaders

### App Directory Convention

```rust
use jinja2rs::compat::DjangoMode;
use std::path::PathBuf;

let django_mode = DjangoMode::full()
    .with_app_directory(PathBuf::from("/home/app/accounts/"))
    .with_app_directory(PathBuf::from("/home/app/core/"))
    .with_app_directory(PathBuf::from("/home/app/common/"));

// Templates are searched in:
// 1. /home/app/accounts/templates/
// 2. /home/app/core/templates/
// 3. /home/app/common/templates/
// 
// First match found is used (precedence order)

let mut env = Environment::with_django_mode(django_mode)?;

// Load "admin/dashboard.html" from first matching location
let tmpl = env.get_template("admin/dashboard.html")?;
```

**Directory structure**:
```
/home/app/accounts/
├── models.py
├── views.py
└── templates/
    ├── admin/
    │   └── dashboard.html
    └── profile.html

/home/app/core/
├── models.py
└── templates/
    ├── admin/
    │   └── dashboard.html
    ├── base.html
    └── footer.html

/home/app/common/
├── models.py
└── templates/
    ├── macros.html
    └── widgets.html
```

## Context Processors

Context processors automatically inject variables into every template:

```rust
use minijinja::Value;

let mut env = Environment::with_django_mode(DjangoMode::full())?;

// Add a context processor that injects CSRF token
env.add_context_processor(|ctx: &mut Value| {
    // ctx is a mutable reference to the template context
    // Add CSRF token to all templates
    if let Ok(obj) = ctx.as_object_mut() {
        obj.insert("csrf_token".to_string(), 
            Value::from("abc123xyz789"));
    }
    Ok(())
});

// Add a context processor that injects current user
env.add_context_processor(|ctx: &mut Value| {
    if let Ok(obj) = ctx.as_object_mut() {
        obj.insert("current_user".to_string(),
            Value::from_object(UserObject {
                username: "john_doe".to_string(),
                is_authenticated: true,
            }));
    }
    Ok(())
});

// Now render a template
env.add_template("dashboard.html", r#"
<form method="post">
    <input type="hidden" name="csrfmiddlewaretoken" 
           value="{{ csrf_token }}">
    <p>Welcome, {{ current_user.username }}!</p>
    <button type="submit">Submit</button>
</form>
"#)?;

let tmpl = env.get_template("dashboard.html")?;
let output = tmpl.render(json!({}))?;
// csrf_token and current_user are automatically available
```

## Auto-escaping

Django automatically escapes HTML in templates (safety default):

```rust
let env = Environment::with_django_mode(DjangoMode::default())?;

// HTML content is auto-escaped
env.render_str(
    "{{ html_content }}", 
    json!({"html_content": "<script>alert('xss')</script>"})
)?;
// Output: "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"

// Force escape explicitly
env.render_str(
    "{{ html_content|force_escape }}", 
    json!({"html_content": "text"})
)?;
// Output: "text" (already safe, no change)

// Mark as safe (no escaping)
env.render_str(
    "{{ html_content|safe }}", 
    json!({"html_content": "<b>bold</b>"})
)?;
// Output: "<b>bold</b>" (not escaped)
```

## Custom Filters

```rust
let mut env = Environment::with_django_mode(DjangoMode::default())?;

// Add custom filter
env.add_filter("reverse_string", |value: Value| -> Result<String, minijinja::Error> {
    Ok(value.to_string().chars().rev().collect())
});

env.render_str("{{ text|reverse_string }}", json!({"text": "hello"}))?;
// Output: "olleh"

// Chaining filters
env.render_str(
    "{{ text|reverse_string|upper }}", 
    json!({"text": "hello"})
)?;
// Output: "OLLEH"
```

## Request and User Objects

```rust
// Example context with request object
let context = json!({
    "request": {
        "method": "POST",
        "path": "/api/users/",
        "user": {
            "username": "alice",
            "is_authenticated": true,
            "permissions": ["users.add_user", "users.change_user"]
        }
    }
});

env.add_template("admin.html", r#"
{% if request.user.is_authenticated %}
    <p>Hello, {{ request.user.username }}!</p>
    {% if request.method == "POST" %}
        <p>Processing form submission...</p>
    {% endif %}
{% else %}
    <p>Please log in</p>
{% endif %}
"#)?;

let tmpl = env.get_template("admin.html")?;
let output = tmpl.render(context)?;
```

## QuerySet-like Iteration

Pre-serialized query results can be rendered as if they were Django ORM QuerySets:

```rust
// Simulate Django ORM query results (pre-serialized from database)
let context = json!({
    "users": [
        {"id": 1, "name": "Alice", "email": "alice@example.com"},
        {"id": 2, "name": "Bob", "email": "bob@example.com"},
        {"id": 3, "name": "Charlie", "email": "charlie@example.com"}
    ]
});

env.add_template("user_list.html", r#"
<ul>
{% for user in users %}
    <li>{{ user.name }} ({{ user.email }})</li>
{% endfor %}
</ul>
"#)?;

let tmpl = env.get_template("user_list.html")?;
let output = tmpl.render(context)?;
// Output: HTML list of users
```

## Comparisons with Jinja2 Mode

### Feature Differences

```rust
use jinja2rs::Environment;
use jinja2rs::compat::DjangoMode;

// Django mode
let django_env = Environment::with_django_mode(DjangoMode::default())?;

// Jinja2 mode (for comparison)
let jinja2_env = Environment::new();

// Same features:
// - Template inheritance ({% extends %}, {% block %})
// - For loops, if statements
// - Variable access via dot notation

// Different features:
// Django:
// - HTML auto-escape by default
// - Django-specific filters (slugify, truncatewords, etc.)
// - App directory loader convention
// - Context processors

// Jinja2:
// - Optional auto-escape
// - Jinja2-specific filters
// - Generic file system loader
// - No context processors
```

### Migration Example

```rust
// Before: Python Django template
let django_template = r#"
<h1>{{ title|slugify }}</h1>
<p>{{ description|truncatewords:20 }}</p>
<ul>
{% for item in items %}
    <li>{{ item.name|upper }}</li>
{% endfor %}
</ul>
"#;

// After: Can use same template in jinja2rs
let env = Environment::with_django_mode(DjangoMode::default())?;
env.add_template("page.html", django_template)?;

let context = json!({
    "title": "My Django Project",
    "description": "A long description of the project that should be truncated",
    "items": [
        {"name": "item one"},
        {"name": "item two"}
    ]
});

let tmpl = env.get_template("page.html")?;
let output = tmpl.render(context)?;
```

## Performance Tips

### 1. Reuse Environment

```rust
// Good: Create once, reuse
let env = Environment::with_django_mode(DjangoMode::default())?;
for template_name in templates {
    let tmpl = env.get_template(template_name)?;
    // render...
}

// Avoid: Creating new environment each time
for template_name in templates {
    let env = Environment::with_django_mode(DjangoMode::default())?;
    let tmpl = env.get_template(template_name)?;
    // render...
}
```

### 2. Pre-serialize Data

```rust
// Good: Pass already-serialized JSON
let context = json!({
    "users": fetch_users_from_db()  // Already JSON
});

let tmpl = env.get_template("list.html")?;
let output = tmpl.render(context)?;

// Avoid: Re-serializing repeatedly
for user in users {
    let context = json!({"user": user});
    // render...
}
```

### 3. Use Appropriate Features

```rust
// Minimal mode for simple rendering
let env = Environment::with_django_mode(
    DjangoMode::minimal()
)?;

// Full mode only if needed
let env = Environment::with_django_mode(
    DjangoMode::full()
)?;
```

## Error Handling

```rust
use jinja2rs::Environment;
use jinja2rs::compat::DjangoMode;
use jinja2rs::Jinja2Error;

match Environment::with_django_mode(DjangoMode::default()) {
    Ok(mut env) => {
        match env.add_template("my.html", "{{ undefined_var }}") {
            Ok(_) => {
                match env.get_template("my.html") {
                    Ok(tmpl) => {
                        match tmpl.render(json!({})) {
                            Ok(output) => println!("{}", output),
                            Err(Jinja2Error::Render(e)) => {
                                eprintln!("Render error: {}", e);
                            }
                            Err(e) => eprintln!("Error: {:?}", e),
                        }
                    }
                    Err(Jinja2Error::TemplateNotFound(name)) => {
                        eprintln!("Template not found: {}", name);
                    }
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }
            Err(e) => eprintln!("Failed to add template: {:?}", e),
        }
    }
    Err(e) => eprintln!("Failed to create environment: {:?}", e),
}
```

## Configuration Examples

### Multi-app Django Project

```rust
use jinja2rs::compat::DjangoMode;
use std::path::PathBuf;

let config = DjangoMode::full()
    .with_app_directory(PathBuf::from("/django/apps/users/"))
    .with_app_directory(PathBuf::from("/django/apps/posts/"))
    .with_app_directory(PathBuf::from("/django/apps/comments/"))
    .with_app_directory(PathBuf::from("/django/apps/common/"))
    .with_locale("en-US")
    .with_timezone("America/New_York");

let mut env = Environment::with_django_mode(config)?;
```

### Internationalized Django Project

```rust
let config = DjangoMode::full()
    .with_app_directory(PathBuf::from("/app/"))
    .with_locale("fr-FR")
    .with_timezone("Europe/Paris");

let mut env = Environment::with_django_mode(config)?;

// Translations would be loaded from compiled message files
```

## Troubleshooting

### Template Not Found

```rust
// Error: Template 'admin/dashboard.html' not found in app directories

// Solution: Ensure app directory structure is correct
// Expected: /app/templates/admin/dashboard.html
// NOT: /app/admin/dashboard.html (missing templates/)

let config = DjangoMode::full()
    .with_app_directory(PathBuf::from("/app/"));  // Correct
    
// Check directory contents:
// /app/
// └── templates/
//     └── admin/
//         └── dashboard.html
```

### Variable Undefined

```rust
// Template references undefined variable
env.render_str("{{ undefined_var }}", json!({}))?;
// Result: renders empty string (Django default)

// To catch undefined variables:
env.add_template("test.html", "{{ user.name }}")?;
// Will render empty if user doesn't exist
```

### Filter Not Found

```rust
// Error: Unknown filter 'my_filter'

// Solution: Register custom filters
env.add_filter("my_filter", |value: Value| {
    Ok(format!("custom: {}", value))
});

env.render_str("{{ text|my_filter }}", json!({"text": "hello"}))?;
// Output: "custom: hello"
```

## Comparing Output with Django

When testing compatibility, you can compare jinja2rs output with actual Django:

```bash
# Generate baseline from actual Django
python manage.py shell < generate_templates.py > django_output.txt

# Generate from jinja2rs
cargo test --test django_compat > jinja2rs_output.txt

# Compare
diff django_output.txt jinja2rs_output.txt
```
