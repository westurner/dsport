# Django Template Compatibility Mode Design

## Overview

This document outlines the design for a **Django Template Mode** in `jinja2rs` — a compatibility layer that enables rendering Django templates in Rust while maintaining feature parity with the Django template engine.

Django templates differ significantly from Jinja2, despite both being Python-based templating systems. Django's template language is more restrictive by design, with different syntax, filters, tags, and processing model. A dedicated Django mode will allow `jinja2rs` to:

- Serve as a drop-in replacement for Django's template engine in performance-critical paths.
- Enable mixed Python/Rust template rendering in Django projects.
- Support template migration from Django to Jinja2 or vice versa.
- Power downstream projects like `sphinxdocrs` when rendering Django-based documentation.

## Architecture

### Design Pattern: Composable Compatibility Modes

Following the existing `AnsibleMode` and `KubernetesMode` pattern in `jinja2rs`, **Django mode** will be implemented as a composable configuration struct:

```rust
pub struct DjangoMode {
    /// Whether to enable string-based lookups for reverse URL resolution.
    pub enable_url_resolution: bool,
    
    /// Whether to enable context processors for automatic context injection.
    pub enable_context_processors: bool,
    
    /// Whether to enable custom template tag/filter registration.
    pub enable_custom_tags: bool,
    
    /// Django app directories to search for templates.
    pub app_directories: Vec<PathBuf>,
    
    /// Django settings module (e.g., "myapp.settings").
    pub settings_module: Option<String>,
    
    /// Whether to enable Django ORM query support in templates.
    pub enable_orm_queries: bool,
    
    /// Timezone for date/time filters.
    pub timezone: String,
    
    /// Locale for i18n filters (e.g., "en-US").
    pub locale: String,
}
```

### Integration Points

```
┌─────────────────────────────────────────┐
│        jinja2rs::Environment            │
├─────────────────────────────────────────┤
│  compat::CompatMode enum                │
│  ├─ Jinja2                              │
│  ├─ minijinja                           │
│  ├─ Ansible                             │
│  ├─ Kubernetes                          │
│  └─ [NEW] Django                        │
└─────────────────────────────────────────┘
           │
           ├─→ DjangoEnvironment
           │   ├─ Template loaders (app_directories)
           │   ├─ Context processors
           │   ├─ Custom tags/filters
           │   └─ ORM query handler
           │
           └─→ Django-specific filters
               ├─ date/time filters
               ├─ string filters
               ├─ numeric filters
               └─ and more
```

### Key Differences: Django vs Jinja2

| Feature | Django | Jinja2 | jinja2rs Strategy |
|---------|--------|--------|-------------------|
| **Auto-escaping** | Enabled by default | Opt-in | Configure per environment |
| **Filter syntax** | `{{ value\|filter }}` | Same | Native support |
| **Tag syntax** | `{% tag %}...{% endtag %}` | Same | Translate unsupported tags |
| **Method calls** | `{{ obj.method }}` — discouraged | Encouraged | Emulation layer |
| **Variable access** | Dot notation only; no brackets for method calls | Full Python access | Restrict to dot notation |
| **Built-in filters** | Django-specific set (e.g., `truncatewords`, `urlize`) | Jinja2-specific set | Register Django filters |
| **Template tags** | 50+ built-ins (url, include, etc.) | ~10 built-ins | Implement subset |
| **Context processors** | Automatic context injection | Not supported | Custom context provider |
| **URL resolution** | `{% url "view_name" arg1 arg2 %}` | Not available | Provide via custom function |
| **CSRF tokens** | `{% csrf_token %}` | Not available | Register as global |
| **i18n** | `{% trans %}`, `{% blocktrans %}` | `{% trans %}` similar | Use i18n extension |
| **Comments** | `{# ... #}` | Same | Native support |
| **Numeric formatting** | `{{ value\|floatformat }}` | Different | Map to Django filter |
| **QuerySet iteration** | Direct template iteration of DB results | Not natively supported | Custom value type |

## Features

### Phase 1: Core Infrastructure (MVP)

- [x] Define `DjangoMode` config struct
- [ ] Implement `DjangoEnvironment` wrapper
- [ ] Register Django-specific filters (Phase 1 subset)
- [ ] Implement basic context processor interface
- [ ] Create Django template loader with app directories
- [ ] Add Django-specific error messages
- [ ] Unit tests for core features

**Filters (Phase 1):**
- String filters: `upper`, `lower`, `slugify`, `truncatewords`, `truncatechars`, `wordwrap`
- Numeric filters: `add`, `floatformat`, `pluralize`
- List filters: `first`, `last`, `join`, `length`
- Boolean: `yesno`, `default`
- Escaping: `escape`, `force_escape`, `safe` (mark-safe equivalent)

**Tags (Phase 1):**
- `{% comment %}...{% endcomment %}` (already supported via Jinja2)
- `{% if %}...{% endif %}` (native support)
- `{% for %}...{% endfor %}` (native support)
- `{% block %}...{% endblock %}` (native support)
- `{% extends %}` (native support)

### Phase 2: Advanced Filters & Tags

- [ ] Date/time filters: `date`, `time`, `timesince`, `timeuntil`
- [ ] URL filters: `urlencode`, `urlize`
- [ ] List filters: `dictsort`, `dictsortreversed`
- [ ] Template tags: `{% url %}`, `{% include %}` (with Django-specific behavior)
- [ ] `{% load %}` tag for custom tag registration
- [ ] Query string parsing in context

**Filters (Phase 2):**
- `date`, `time`, `timesince`, `timeuntil`
- `urlencode`, `urlize`, `escape`
- `make_list` (convert to list)

**Tags (Phase 2):**
- `{% url "view-name" arg1 arg2 %}`
- `{% include "template-name" %}`
- `{% load %}`
- `{% with %}...{% endwith %}`

### Phase 3: Context Processors & ORM

- [ ] Context processor registration and execution
- [ ] ORM query result wrapping (QuerySet-like interface)
- [ ] Lazy evaluation support
- [ ] Request object in context

**Components:**
- Request object fields (user, method, GET/POST, META)
- ORM query wrapper (map to table/view queries)
- User object (permissions, groups)

### Phase 4: CSRF, i18n, & Advanced Features

- [ ] CSRF token injection
- [ ] `{% trans %}` and `{% blocktrans %}` tags
- [ ] Message and warning frameworks
- [ ] Custom template tag/filter registration API
- [ ] Cache integration (template fragment caching)

**Tags (Phase 4):**
- `{% csrf_token %}`
- `{% trans "text" %}`
- `{% blocktrans %}...{% endblocktrans %}`
- `{% cache %}`
- `{% spaceless %}`

### Phase 5: Compatibility & Migration

- [ ] Django template dialect detection (auto-mode selection)
- [ ] Template error messages mapped to Django format
- [ ] Migration guide: Django → jinja2rs
- [ ] Performance benchmarks vs Django

## Implementation Details

### 1. DjangoMode Configuration

```rust
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DjangoMode {
    pub enable_url_resolution: bool,
    pub enable_context_processors: bool,
    pub enable_custom_tags: bool,
    pub app_directories: Vec<PathBuf>,
    pub settings_module: Option<String>,
    pub enable_orm_queries: bool,
    pub timezone: String,
    pub locale: String,
}

impl Default for DjangoMode {
    fn default() -> Self {
        DjangoMode {
            enable_url_resolution: true,
            enable_context_processors: true,
            enable_custom_tags: true,
            app_directories: Vec::new(),
            settings_module: None,
            enable_orm_queries: false,
            timezone: "UTC".to_string(),
            locale: "en-US".to_string(),
        }
    }
}

impl DjangoMode {
    pub fn full() -> Self {
        DjangoMode::default()
    }

    pub fn minimal() -> Self {
        DjangoMode {
            enable_url_resolution: false,
            enable_context_processors: false,
            enable_custom_tags: false,
            app_directories: Vec::new(),
            settings_module: None,
            enable_orm_queries: false,
            timezone: "UTC".to_string(),
            locale: "en-US".to_string(),
        }
    }

    pub fn with_app_directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.app_directories.push(path.into());
        self
    }

    pub fn with_timezone(mut self, tz: impl Into<String>) -> Self {
        self.timezone = tz.into();
        self
    }

    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = locale.into();
        self
    }
}
```

### 2. DjangoEnvironment Wrapper

```rust
use crate::environment::Environment;
use minijinja::Value;

pub struct DjangoEnvironment {
    inner: Environment,
    config: DjangoMode,
    context_processors: Vec<Box<dyn Fn(&mut Value) -> Result<(), String>>>,
}

impl DjangoEnvironment {
    pub fn new(config: DjangoMode) -> Self {
        let mut env = Environment::new();
        
        // Register Django filters
        Self::register_django_filters(&mut env, &config);
        
        // Set auto-escape to True by default (Django style)
        env.inner.set_auto_escape_callback(|name| {
            name.ends_with(".html") || name.ends_with(".htm")
        });

        DjangoEnvironment {
            inner: env,
            config,
            context_processors: Vec::new(),
        }
    }

    pub fn add_context_processor<F>(&mut self, processor: F)
    where
        F: Fn(&mut Value) -> Result<(), String> + 'static,
    {
        self.context_processors.push(Box::new(processor));
    }

    fn register_django_filters(env: &mut Environment, config: &DjangoMode) {
        // Phase 1 filters
        env.add_filter("upper", crate::filters::django::upper);
        env.add_filter("lower", crate::filters::django::lower);
        env.add_filter("slugify", crate::filters::django::slugify);
        env.add_filter("truncatewords", crate::filters::django::truncatewords);
        env.add_filter("truncatechars", crate::filters::django::truncatechars);
        env.add_filter("wordwrap", crate::filters::django::wordwrap);
        env.add_filter("add", crate::filters::django::add);
        env.add_filter("floatformat", crate::filters::django::floatformat);
        env.add_filter("pluralize", crate::filters::django::pluralize);
        env.add_filter("first", crate::filters::django::first);
        env.add_filter("last", crate::filters::django::last);
        env.add_filter("join", crate::filters::django::join);
        env.add_filter("length", crate::filters::django::length);
        env.add_filter("yesno", crate::filters::django::yesno);
        env.add_filter("default", crate::filters::django::default);
        env.add_filter("escape", crate::filters::django::escape);
        env.add_filter("force_escape", crate::filters::django::force_escape);
        env.add_filter("safe", crate::filters::django::safe);

        if config.enable_url_resolution {
            env.add_filter("urlencode", crate::filters::django::urlencode);
        }
    }
}
```

### 3. Django Filters Module

Create `src/jinja2rs/src/filters/django.rs`:

```rust
//! Django-specific template filters.
//!
//! Maps Django filter semantics to minijinja values.

use minijinja::Value;
use regex::Regex;

/// Convert to uppercase (Django: `{{ value|upper }}`)
pub fn upper(value: Value) -> Result<String, minijinja::Error> {
    Ok(value.to_string().to_uppercase())
}

/// Convert to lowercase
pub fn lower(value: Value) -> Result<String, minijinja::Error> {
    Ok(value.to_string().to_lowercase())
}

/// Convert string to URL slug
pub fn slugify(value: Value) -> Result<String, minijinja::Error> {
    let s = value.to_string();
    // Replace spaces with hyphens, remove non-alphanumeric, lowercase
    let slug = s
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else if c.is_whitespace() { '-' } else { ' ' })
        .collect::<String>()
        .replace("  ", " ")
        .replace(' ', "-")
        .trim_matches('-')
        .to_string();
    Ok(slug)
}

/// Truncate string to word count
pub fn truncatewords(value: Value, words: Option<i64>) -> Result<String, minijinja::Error> {
    let s = value.to_string();
    let word_count = words.unwrap_or(3) as usize;
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() > word_count {
        Ok(format!("{}...", parts[..word_count].join(" ")))
    } else {
        Ok(s)
    }
}

/// Truncate string to character count
pub fn truncatechars(value: Value, chars: Option<i64>) -> Result<String, minijinja::Error> {
    let s = value.to_string();
    let char_count = chars.unwrap_or(3) as usize;
    if s.len() > char_count {
        Ok(format!("{}...", &s[..char_count.saturating_sub(3)]))
    } else {
        Ok(s)
    }
}

/// Wrap text to given width
pub fn wordwrap(value: Value, width: Option<i64>) -> Result<String, minijinja::Error> {
    let s = value.to_string();
    let w = width.unwrap_or(79) as usize;
    let mut result = String::new();
    let mut line = String::new();
    for word in s.split_whitespace() {
        if line.len() + word.len() + 1 > w && !line.is_empty() {
            result.push_str(&line);
            result.push('\n');
            line = word.to_string();
        } else if !line.is_empty() {
            line.push(' ');
            line.push_str(word);
        } else {
            line = word.to_string();
        }
    }
    if !line.is_empty() {
        result.push_str(&line);
    }
    Ok(result)
}

/// Add numeric value (Django: `{{ value|add:5 }}`)
pub fn add(value: Value, arg: i64) -> Result<i64, minijinja::Error> {
    if let Some(v) = value.as_i64() {
        Ok(v + arg)
    } else if let Some(v) = value.as_f64() {
        Ok((v + arg as f64) as i64)
    } else {
        Err("Cannot add to non-numeric value".into())
    }
}

/// Format floating point number (Django: `{{ value|floatformat:2 }}`)
pub fn floatformat(value: Value, digits: Option<i64>) -> Result<String, minijinja::Error> {
    let d = digits.unwrap_or(-1) as i32;
    let v = value.as_f64()
        .or_else(|| value.as_i64().map(|i| i as f64))
        .ok_or_else(|| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, "not a number"))?;
    
    if d == -1 {
        Ok(format!("{}", v))
    } else {
        Ok(format!("{:.prec$}", v, prec = d as usize))
    }
}

/// Pluralize word based on count (Django: `{{ count|pluralize }}`)
pub fn pluralize(value: Value, suffix: Option<String>) -> Result<String, minijinja::Error> {
    let count = value.as_i64().unwrap_or(0);
    let suf = suffix.unwrap_or_else(|| "s".to_string());
    Ok(if count == 1 { "".to_string() } else { suf })
}

/// Get first item in list
pub fn first(value: Value) -> Result<Value, minijinja::Error> {
    if let Some(seq) = value.as_seq() {
        Ok(seq.iter().next().unwrap_or(Value::Undefined))
    } else {
        Ok(Value::Undefined)
    }
}

/// Get last item in list
pub fn last(value: Value) -> Result<Value, minijinja::Error> {
    if let Some(seq) = value.as_seq() {
        Ok(seq.iter().last().unwrap_or(Value::Undefined))
    } else {
        Ok(Value::Undefined)
    }
}

/// Join list items
pub fn join(value: Value, separator: Option<String>) -> Result<String, minijinja::Error> {
    let sep = separator.unwrap_or_else(|| ", ".to_string());
    if let Some(seq) = value.as_seq() {
        Ok(seq.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(&sep))
    } else {
        Ok(value.to_string())
    }
}

/// Get length of value
pub fn length(value: Value) -> Result<usize, minijinja::Error> {
    if let Some(seq) = value.as_seq() {
        Ok(seq.iter().count())
    } else if let Some(map) = value.as_object() {
        Ok(map.iter().count())
    } else {
        Ok(value.to_string().len())
    }
}

/// Convert boolean to yes/no/maybe (Django: `{{ value|yesno:"yes,no,maybe" }}`)
pub fn yesno(value: Value, mapping: Option<String>) -> Result<String, minijinja::Error> {
    let map = mapping.unwrap_or_else(|| "yes,no,maybe".to_string());
    let parts: Vec<&str> = map.split(',').collect();
    
    let index = if value.is_true() {
        0
    } else if value.is_false() {
        1
    } else {
        2
    };
    
    Ok(parts.get(index).unwrap_or(&"").to_string())
}

/// Provide default value if none given
pub fn default(value: Value, default_val: String) -> String {
    if value.is_undefined() || value.is_none() {
        default_val
    } else {
        value.to_string()
    }
}

/// Escape HTML special characters
pub fn escape(value: Value) -> String {
    html_escape::encode_text(&value.to_string()).to_string()
}

/// Force escape even if marked safe
pub fn force_escape(value: Value) -> String {
    html_escape::encode_text(&value.to_string()).to_string()
}

/// Mark value as safe (no escaping)
pub fn safe(value: Value) -> Value {
    // In minijinja, this would be handled by the autoescape setting
    // For now, just return the value as-is
    value
}

/// URL encode
pub fn urlencode(value: Value) -> String {
    urlencoding::encode(&value.to_string()).to_string()
}
```

### 4. Django Template Loader

Create `src/jinja2rs/src/loaders/django.rs`:

```rust
//! Django-specific template loader with app directories support.

use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone)]
pub struct DjangoTemplateLoader {
    app_directories: Vec<PathBuf>,
}

impl DjangoTemplateLoader {
    pub fn new(app_dirs: Vec<PathBuf>) -> Self {
        DjangoTemplateLoader {
            app_directories: app_dirs,
        }
    }

    pub fn add_app_directory(&mut self, path: PathBuf) {
        self.app_directories.push(path);
    }

    /// Load template from app directories following Django conventions:
    /// - Search in each app's `templates/` subdirectory
    /// - Return first match found
    pub fn load_source(&self, name: &str) -> Result<String, String> {
        for app_dir in &self.app_directories {
            let template_path = app_dir.join("templates").join(name);
            if template_path.exists() {
                return fs::read_to_string(&template_path)
                    .map_err(|e| format!("Failed to read template {}: {}", name, e));
            }
        }
        Err(format!("Template '{}' not found in app directories", name))
    }
}
```

## Usage Examples

### Basic Django Mode

```rust
use jinja2rs::Environment;
use jinja2rs::compat::DjangoMode;
use serde_json::json;

// Create environment with Django compatibility
let django_mode = DjangoMode::default();
let mut env = Environment::with_django_mode(django_mode)?;

// Add template
env.add_template("hello.html", 
    "Hello, {{ name|upper }}!")?;

let tmpl = env.get_template("hello.html")?;
let output = tmpl.render(json!({"name": "world"}))?;
assert_eq!(output, "Hello, WORLD!");
```

### Django Mode with App Directories

```rust
use jinja2rs::compat::DjangoMode;
use std::path::PathBuf;

let django_mode = DjangoMode::full()
    .with_app_directory(PathBuf::from("/app/myapp"))
    .with_app_directory(PathBuf::from("/app/common"))
    .with_locale("fr-FR")
    .with_timezone("Europe/Paris");

let mut env = Environment::with_django_mode(django_mode)?;

// Templates loaded from /app/myapp/templates/ or /app/common/templates/
let tmpl = env.get_template("base.html")?;
```

### Context Processors

```rust
let mut env = Environment::with_django_mode(DjangoMode::full())?;

env.add_context_processor(|ctx| {
    // Add CSRF token to all templates
    ctx.set_attr("csrf_token", Value::from("token123"));
    Ok(())
});

env.add_context_processor(|ctx| {
    // Add current user to all templates
    ctx.set_attr("user", Value::from_object(UserObject::new()));
    Ok(())
});
```

### Custom Filters

```rust
env.add_filter("custom", |value: Value| {
    Ok(format!("Custom: {}", value))
});
```

## Testing Strategy

### Unit Tests

- Filter behavior (truncate, pluralize, date formatting, etc.)
- Context processor execution order
- Template loader app directory searching
- Error handling for unsupported Django features

### Integration Tests

- Django template rendering vs actual Django output
- Performance benchmarks
- Mixed Python/Rust rendering

### Compatibility Matrix

Create test suite comparing output against:
- Django 4.0+
- Django 5.0+
- Real-world Django templates from popular projects

Test structure:
```
tests/
├── django/
│   ├── filters/
│   │   ├── test_string_filters.rs
│   │   ├── test_numeric_filters.rs
│   │   ├── test_date_filters.rs
│   │   └── test_list_filters.rs
│   ├── tags/
│   │   ├── test_url_tag.rs
│   │   ├── test_include_tag.rs
│   │   └── test_csrf_tag.rs
│   ├── loaders/
│   │   └── test_app_directories.rs
│   ├── context/
│   │   ├── test_context_processors.rs
│   │   └── test_orm_queries.rs
│   └── compatibility/
│       └── test_django_templates.rs
```

## Rollout Plan

### Week 1-2: Foundation (Phase 1)
- [ ] Implement `DjangoMode` struct
- [ ] Create `DjangoEnvironment` wrapper
- [ ] Register Phase 1 filters
- [ ] Write unit tests for Phase 1 filters
- [ ] Document basic usage

### Week 3-4: Loaders & Context (Phase 1-2)
- [ ] Implement Django template loader with app directories
- [ ] Implement context processor registration
- [ ] Add `{% url %}` tag support
- [ ] Implement basic `{% include %}` Django-style behavior
- [ ] Integration tests

### Week 5-6: Date/Time & Advanced (Phase 2-3)
- [ ] Implement date/time filters
- [ ] ORM query result wrapping
- [ ] Request object in context
- [ ] Performance benchmarks vs Django

### Week 7-8: Polish & Documentation (Phase 4)
- [ ] CSRF token support
- [ ] Custom tag registration API
- [ ] Migration guide: Django → jinja2rs
- [ ] Comprehensive documentation

## Feature Gaps & Limitations

### Intentional Non-Support

1. **Django ORM Integration**: While we can wrap query results, full ORM access would require PyO3 binding to Django models. Instead, we'll support pre-serialized data.

2. **Middleware**: Django's middleware system doesn't apply in template rendering context.

3. **Form Rendering**: Django forms (`.as_p()`, `.as_ul()`) are Python-specific. Recommend rendering forms outside templates or via custom filters.

4. **Custom Template Tags (Python)**: Python-based custom tags can't be loaded. Support only for Rust-based tag registration.

### Workarounds

| Django Feature | Workaround |
|---|---|
| Custom Python tags | Pre-process or use context variables |
| Form rendering | Pre-render forms in Rust or pass as context |
| Signals/hooks | Use context processors instead |
| Template tags with complex logic | Render in Rust, pass results as context |

## Compatibility Notes

### Django Version Support

- **Target**: Django 4.0+
- **Testing**: Django 4.0, 4.2, 5.0
- **Compatibility level**: High fidelity for 90%+ of real-world templates

### Known Incompatibilities

1. **Variable attributes**: Django strictly uses dot notation; jinja2rs allows bracket access
2. **Method calls**: Django discourages methods in templates; jinja2rs allows them
3. **Custom filters/tags**: Only Rust implementations supported, not Python
4. **Lazy objects**: Limited support for Django's lazy evaluation

## Configuration in Cargo.toml

```toml
[features]
# ... existing features ...
django = []  # Enable Django template mode
django-orm = ["django"]  # Enable ORM query wrapping (requires PyO3)
```

Activate with:
```bash
cargo test --features django
cargo build --features django,django-orm
```

## References

- [Django Template Language Documentation](https://docs.djangoproject.com/en/stable/topics/templates/language/)
- [Django Built-in Template Filters](https://docs.djangoproject.com/en/stable/ref/templates/builtins/#built-in-filter-reference)
- [Django Built-in Template Tags](https://docs.djangoproject.com/en/stable/ref/templates/builtins/#built-in-tag-reference)
- [jinja2rs Porting Plan](./jinja2rs/../jinja2rs/porting-plan.md)
- [AGENTS.md](./AGENTS.md) — Django mode compatibility rules
