# Django Mode Implementation Quick Reference

## Phase 1 Checklist (Core Infrastructure - Week 1-2)

### Files to Modify

#### 1. `src/compat.rs`
```rust
// ADD after KubernetesMode impl block:

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

impl Default for DjangoMode { ... }
impl DjangoMode {
    pub fn full() -> Self { ... }
    pub fn minimal() -> Self { ... }
    pub fn with_app_directory(self, path: impl Into<PathBuf>) -> Self { ... }
    pub fn with_timezone(self, tz: impl Into<String>) -> Self { ... }
    pub fn with_locale(self, locale: impl Into<String>) -> Self { ... }
}

// UPDATE CompatMode enum:
pub enum CompatMode {
    Jinja2,
    Minijinja,
    Ansible(AnsibleMode),
    Kubernetes(KubernetesMode),
    Django(DjangoMode),  // ADD
}
```

#### 2. `src/environment.rs`
```rust
// ADD to Environment struct:
pub(crate) django_config: Option<DjangoMode>,

// ADD method:
pub fn with_django_mode(config: DjangoMode) -> Result<Self, Jinja2Error> {
    let mut env = Self::new();
    env.inner.set_auto_escape_callback(|name| {
        name.ends_with(".html") || name.ends_with(".htm")
    });
    Self::register_django_filters(&mut env, &config)?;
    env.django_config = Some(config);
    Ok(env)
}

// ADD method:
fn register_django_filters(
    env: &mut Environment,
    config: &DjangoMode,
) -> Result<(), Jinja2Error> {
    // Register from filters/django.rs
    Ok(())
}
```

#### 3. `src/filters.rs` (main module)
```rust
// ADD:
pub mod django;
```

#### 4. `src/loaders.rs` (main module)
```rust
// ADD:
pub mod django;
pub use django::DjangoTemplateLoader;
```

#### 5. `src/lib.rs`
```rust
// ADD to mod declarations:
pub mod django;  // If needed for top-level exports

// EXTEND compat exports:
pub use compat::{CompatMode, AnsibleMode, AnsibleInventorySource, DjangoMode};

// UPDATE features() function to add:
"django:filters:string",
"django:filters:numeric",
"django:filters:list",
"django:filters:boolean",
"django:loaders:app-directories",
```

#### 6. `Cargo.toml`
```toml
[dependencies]
# ADD:
html-escape = "0.2"
urlencoding = "2.1"

[features]
# ADD after existing features:
django = []
```

### Files to Create

#### 1. `src/filters/django.rs` (~400 lines)
```rust
//! Django template filters
use minijinja::Value;

// String filters (5)
pub fn upper(value: Value) -> Result<String, minijinja::Error> { ... }
pub fn lower(value: Value) -> Result<String, minijinja::Error> { ... }
pub fn slugify(value: Value) -> Result<String, minijinja::Error> { ... }
pub fn truncatewords(value: Value, args: (Value,)) -> Result<String, minijinja::Error> { ... }
pub fn truncatechars(value: Value, args: (Value,)) -> Result<String, minijinja::Error> { ... }

// Numeric filters (3)
pub fn add(value: Value, arg: i64) -> Result<i64, minijinja::Error> { ... }
pub fn floatformat(value: Value, digits: Option<i64>) -> Result<String, minijinja::Error> { ... }
pub fn pluralize(value: Value, suffix: Option<String>) -> Result<String, minijinja::Error> { ... }

// List filters (4)
pub fn first(value: Value) -> Result<Value, minijinja::Error> { ... }
pub fn last(value: Value) -> Result<Value, minijinja::Error> { ... }
pub fn join(value: Value, separator: Option<String>) -> Result<String, minijinja::Error> { ... }
pub fn length(value: Value) -> Result<usize, minijinja::Error> { ... }

// Boolean filters (4)
pub fn yesno(value: Value, mapping: Option<String>) -> Result<String, minijinja::Error> { ... }
pub fn default(value: Value, default_val: String) -> String { ... }
pub fn escape(value: Value) -> String { ... }
pub fn force_escape(value: Value) -> String { ... }

// Additional filters (1)
pub fn safe(value: Value) -> Value { ... }

// Total: 15 Phase 1 filters
```

#### 2. `src/loaders/django.rs` (~80 lines)
```rust
//! Django template loader with app directories support
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone)]
pub struct DjangoTemplateLoader {
    app_directories: Vec<PathBuf>,
}

impl DjangoTemplateLoader {
    pub fn new(app_dirs: Vec<PathBuf>) -> Self { ... }
    pub fn add_app_directory(&mut self, path: PathBuf) { ... }
    pub fn load_source(&self, name: &str) -> Result<String, String> { ... }
}
```

### Tests to Create

#### `tests/django/` directory structure
```
tests/django/
├── filters/
│   ├── test_string_filters.rs      (100 lines)
│   ├── test_numeric_filters.rs     (80 lines)
│   ├── test_list_filters.rs        (100 lines)
│   └── test_boolean_filters.rs     (100 lines)
├── loaders/
│   └── test_app_directories.rs     (80 lines)
└── environment/
    └── test_django_environment.rs  (60 lines)
```

### Test Template (~40 lines each)

```rust
#[cfg(test)]
mod tests {
    use jinja2rs::Environment;
    use jinja2rs::compat::DjangoMode;
    use serde_json::json;

    #[test]
    fn test_upper_filter() {
        let env = Environment::with_django_mode(DjangoMode::default()).unwrap();
        let result = env.render_str("{{ text|upper }}", json!({"text": "hello"})).unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_truncatewords_filter() {
        let env = Environment::with_django_mode(DjangoMode::default()).unwrap();
        let result = env.render_str(
            "{{ text|truncatewords:2 }}", 
            json!({"text": "one two three four"})
        ).unwrap();
        assert_eq!(result, "one two ...");
    }
    
    // More tests...
}
```

## Implementation Order

1. **Day 1**: Modify `src/compat.rs`, `src/environment.rs`
2. **Day 2**: Create `src/filters/django.rs` with first 5 string filters
3. **Day 3**: Add numeric (3) and list (4) filters
4. **Day 4**: Add boolean (4) filters and safe filter
5. **Day 5**: Create `src/loaders/django.rs`
6. **Day 6**: Write comprehensive filter tests (~500 lines total)
7. **Day 7**: Write loader tests, integration tests
8. **Day 8**: Performance profiling, documentation
9. **Day 9**: Code review, refinements
10. **Day 10**: Buffer for issues, polish

## Key Implementation Notes

### Filter Registration Pattern
```rust
// In DjangoEnvironment::register_django_filters()
env.inner.add_filter("upper", filters::django::upper);
env.inner.add_filter("lower", filters::django::lower);
env.inner.add_filter("slugify", filters::django::slugify);
// ... etc
```

### Error Handling for Filters
```rust
// Graceful degradation: return empty string or 0 instead of error
pub fn add(value: Value, arg: i64) -> Result<i64, minijinja::Error> {
    match (value.as_i64(), value.as_f64()) {
        (Some(v), _) => Ok(v + arg),
        (None, Some(v)) => Ok((v + arg as f64) as i64),
        _ => Err("Cannot add to non-numeric value".into()),
    }
}
```

### Auto-Escape Configuration
```rust
// In with_django_mode():
env.inner.set_auto_escape_callback(|name| {
    // Enable escaping for .html and .xml files (Django convention)
    name.ends_with(".html") 
        || name.ends_with(".htm")
        || name.ends_with(".xml")
});
```

### App Directory Loader Pattern
```rust
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
```

## Verification Checklist

- [ ] `cargo check` passes without warnings
- [ ] `cargo test --features django` passes all tests
- [ ] At least 80% code coverage for django-specific code
- [ ] `cargo doc` builds without errors
- [ ] Example code compiles and runs
- [ ] Benchmark shows 4-6x speedup vs Django for Phase 1 filters
- [ ] Real Django templates render identically
- [ ] Documentation complete and accurate

## Phase 2 Preview (After Phase 1 Complete)

### New Filters (~4 filters)
- `date` - date formatting
- `time` - time formatting
- `timesince` - time elapsed
- `timeuntil` - time remaining

### New Tags (~3 tags)
- `{% url %}` - URL reversing
- `{% include %}` - template inclusion
- `{% with %}` - variable assignment

### New Components
- Context processor framework
- URL resolver interface
- Date/time utilities

## Performance Targets for Phase 1

| Operation | Target | Threshold |
|-----------|--------|-----------|
| Simple filter | < 1 µs | < 2 µs |
| Filter chain (5) | < 5 µs | < 10 µs |
| Environment creation | < 100 ms | < 200 ms |
| Template compilation | < 50 µs | < 100 µs |
| Template render (10 filters) | < 10 µs | < 20 µs |

## Debugging Tips

### Common Issues

**Issue**: Filter not found
```rust
// Solution: Check it's registered in register_django_filters()
env.inner.add_filter("slugify", filters::django::slugify);
```

**Issue**: Auto-escape not working
```rust
// Solution: Ensure callback is set
env.inner.set_auto_escape_callback(|name| name.ends_with(".html"));
```

**Issue**: Template not found
```rust
// Solution: Check app directory structure
// Expected: /app/templates/template_name.html
// NOT: /app/template_name.html (missing templates/)
```

**Issue**: Filter receives wrong type
```rust
// Solution: Add type checking
if let Some(val) = value.as_i64() {
    // Process integer
} else if let Some(val) = value.as_f64() {
    // Process float
} else {
    // Error handling
}
```

## Resources During Implementation

- **Django Docs**: https://docs.djangoproject.com/en/stable/ref/templates/builtins/
- **minijinja Docs**: https://github.com/mitsuhiko/minijinja
- **HTML Escape Crate**: https://docs.rs/html-escape/
- **URL Encoding Crate**: https://docs.rs/urlencoding/

## Communication & Review

### Weekly Standup
- [ ] What filters/features were completed
- [ ] What blockers exist
- [ ] Test coverage achieved
- [ ] Performance metrics

### Code Review Checklist
- [ ] Tests pass (100% of test suite)
- [ ] Coverage >= 85%
- [ ] No clippy warnings
- [ ] Documentation complete
- [ ] Examples work
- [ ] Performance acceptable

### Merge Criteria
- [ ] All tests pass
- [ ] Coverage >= 85%
- [ ] Code review approved
- [ ] Performance benchmarked
- [ ] Documentation updated
- [ ] No breaking changes to existing APIs

---

**Print this page for quick reference while implementing Phase 1!**
