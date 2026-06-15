# Django Mode Implementation Roadmap

## Quick Reference

| Component | File | Status | Phase | Priority |
|-----------|------|--------|-------|----------|
| DjangoMode config | `src/compat.rs` | TODO | 1 | P0 |
| DjangoEnvironment | `src/environment.rs` | TODO | 1 | P0 |
| Django filters module | `src/filters/django.rs` | TODO | 1 | P0 |
| Django loader | `src/loaders/django.rs` | TODO | 1 | P0 |
| Phase 1 filter tests | `tests/django/filters/` | TODO | 1 | P0 |
| Phase 2 tag support | `src/tags/django.rs` | TODO | 2 | P1 |
| Context processors | `src/context.rs` | TODO | 2 | P1 |
| ORM wrapping | `src/orm.rs` | TODO | 3 | P2 |
| Date/time filters | `src/filters/datetime.rs` | TODO | 2 | P1 |
| CSRF support | `src/tags/csrf.rs` | TODO | 4 | P2 |
| i18n extensions | `src/i18n_django.rs` | TODO | 4 | P2 |
| Integration tests | `tests/django/integration/` | TODO | 5 | P1 |
| Documentation | `docs/DJANGO_USAGE.md` | TODO | 5 | P1 |

## Phase 1: Core Infrastructure (Week 1-2)

### 1.1 Extend `src/compat.rs`

**Goal**: Add `DjangoMode` configuration struct

**Changes**:
```rust
// In src/compat.rs, after the KubernetesMode impl block

/// Django template compatibility mode.
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
    pub fn with_app_directory(mut self, path: impl Into<PathBuf>) -> Self { ... }
    pub fn with_timezone(mut self, tz: impl Into<String>) -> Self { ... }
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self { ... }
}
```

**Add to `CompatMode` enum**:
```rust
pub enum CompatMode {
    Jinja2,
    Minijinja,
    Ansible(AnsibleMode),
    Kubernetes(KubernetesMode),
    Django(DjangoMode),  // NEW
}
```

**Tests**:
- `tests/django/config/test_django_mode_builder.rs`
- Verify builder methods chain correctly
- Test default values

### 1.2 Update `src/environment.rs`

**Goal**: Add `with_django_mode()` constructor

**Changes**:
```rust
impl Environment {
    pub fn with_django_mode(config: DjangoMode) -> Result<Self, Jinja2Error> {
        let mut env = Self::new();
        
        // Set auto-escape to true by default (Django behavior)
        env.inner.set_auto_escape_callback(|name| {
            name.ends_with(".html") || name.ends_with(".htm")
        });
        
        // Register Django filters
        Self::register_django_filters(&mut env, &config)?;
        
        // Store config for later use
        env.django_config = Some(config);
        
        Ok(env)
    }
    
    fn register_django_filters(
        env: &mut Environment,
        config: &DjangoMode,
    ) -> Result<(), Jinja2Error> {
        // Register filters from filters/django.rs
        // See next section
        Ok(())
    }
}
```

**Add field to Environment**:
```rust
pub struct Environment {
    pub(crate) inner: MiniEnv<'static>,
    pub(crate) search_paths: Vec<PathBuf>,
    pub(crate) django_config: Option<DjangoMode>,  // NEW
}
```

**Tests**:
- `tests/django/environment/test_django_environment_creation.rs`
- Verify filters are registered
- Check auto-escape is enabled

### 1.3 Create `src/filters/django.rs`

**Goal**: Implement Phase 1 Django filters

**Structure**:
```rust
//! Django-specific template filters.
//!
//! Implements Django filter semantics for use in jinja2rs templates.

use minijinja::Value;

pub fn upper(value: Value) -> Result<String, minijinja::Error> { ... }
pub fn lower(value: Value) -> Result<String, minijinja::Error> { ... }
pub fn slugify(value: Value) -> Result<String, minijinja::Error> { ... }
pub fn truncatewords(
    value: Value,
    args: (Value,),
) -> Result<String, minijinja::Error> { ... }
// ... more filters
```

**Filters to implement (Phase 1)**:
1. String: `upper`, `lower`, `slugify`, `truncatewords`, `truncatechars`
2. Numeric: `add`, `floatformat`, `pluralize`
3. List: `first`, `last`, `join`, `length`
4. Boolean: `yesno`, `default`, `escape`, `force_escape`, `safe`

**Tests**:
- `tests/django/filters/test_string_filters.rs`
- `tests/django/filters/test_numeric_filters.rs`
- `tests/django/filters/test_list_filters.rs`
- Each filter with normal + edge cases
- Compare output to Django reference implementation

### 1.4 Create `src/loaders/django.rs`

**Goal**: Django-specific template loader with app directories

**Structure**:
```rust
//! Django template loader supporting app directories convention.

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

**Update `src/loaders.rs`**:
```rust
pub mod django;
pub use django::DjangoTemplateLoader;
```

**Tests**:
- `tests/django/loaders/test_app_directories.rs`
- Directory precedence (first match wins)
- Template not found handling
- Symlink handling

### 1.5 Update `src/filters.rs`

**Add module declaration**:
```rust
pub mod django;
```

**Update `lib.rs` features**:
```rust
pub fn features() -> &'static [&'static str] {
    &[
        // ... existing features ...
        "django:filters:string",
        "django:filters:numeric",
        "django:filters:list",
        "django:filters:boolean",
    ]
}
```

## Phase 2: Advanced Filters & Tags (Week 3-4)

### 2.1 Date/Time Filters

**Create**: `src/filters/django_datetime.rs`

```rust
pub fn date(
    value: Value,
    args: (Value,),
) -> Result<String, minijinja::Error> {
    // Parse format string, render datetime
}

pub fn time(
    value: Value,
    args: (Value,),
) -> Result<String, minijinja::Error> { ... }

pub fn timesince(
    value: Value,
    args: (Value,),
) -> Result<String, minijinja::Error> { ... }

pub fn timeuntil(
    value: Value,
    args: (Value,),
) -> Result<String, minijinja::Error> { ... }
```

**Dependencies**:
```toml
chrono = "0.4"
humantime = "2.1"
```

**Tests**:
- `tests/django/filters/test_date_filters.rs`
- Test with Django datetime format strings
- Timezone handling
- Locale-aware formatting

### 2.2 Template Tag Support: `{% url %}`

**Create**: `src/tags/django_url.rs`

```rust
/// Parse and execute Django's {% url %} tag
pub struct UrlTag {
    view_name: String,
    args: Vec<String>,
}

impl UrlTag {
    pub fn parse(input: &str) -> Result<Self, String> { ... }
    pub fn execute(&self, resolver: &UrlResolver) -> Result<String, String> { ... }
}

pub trait UrlResolver {
    fn reverse(&self, view_name: &str, args: &[&str]) -> Result<String, String>;
}
```

**Tests**:
- `tests/django/tags/test_url_tag.rs`
- URL resolution with arguments
- Missing view handling
- Reverse URL generation

### 2.3 Template Tag Support: `{% include %}`

**Django-specific behavior**:
```rust
pub struct IncludeTag {
    template_name: String,
    context_vars: Vec<String>,
    isolated: bool,  // `isolated` keyword
}
```

**Tests**:
- `tests/django/tags/test_include_tag.rs`
- Include with context
- Isolated context mode
- Circular include protection

### 2.4 Context Processors

**Create**: `src/context.rs`

```rust
pub trait ContextProcessor {
    fn process(&self) -> Result<serde_json::Value, String>;
}

pub struct DjangoContextManager {
    processors: Vec<Box<dyn ContextProcessor>>,
}

impl DjangoContextManager {
    pub fn add_processor(&mut self, processor: Box<dyn ContextProcessor>) { ... }
    pub fn apply(&self, ctx: &mut Value) -> Result<(), String> { ... }
}
```

**Tests**:
- `tests/django/context/test_context_processors.rs`
- Processor execution order
- Error propagation
- Conflicting keys handling

## Phase 3: ORM & Request Context (Week 5-6)

### 3.1 ORM Query Result Wrapping

**Create**: `src/orm.rs`

```rust
/// Wrapper for Django ORM QuerySet-like results
pub struct QuerySetWrapper {
    rows: Vec<serde_json::Value>,
    model_name: String,
}

impl QuerySetWrapper {
    pub fn from_json(data: serde_json::Value, model_name: &str) -> Self { ... }
    pub fn filter(&self, predicate: impl Fn(&Value) -> bool) -> Self { ... }
    pub fn count(&self) -> usize { ... }
}
```

**Tests**:
- `tests/django/orm/test_queryset_wrapper.rs`
- Iteration, filtering, counting
- Lazy evaluation approximation

### 3.2 Request Object Context

**Create**: `src/context/request.rs`

```rust
pub struct DjangoRequest {
    method: String,
    get_params: serde_json::Value,
    post_data: serde_json::Value,
    user: Option<DjangoUser>,
    meta: serde_json::Value,
}

pub struct DjangoUser {
    username: String,
    permissions: Vec<String>,
    groups: Vec<String>,
    is_authenticated: bool,
}
```

**Tests**:
- `tests/django/context/test_request_object.rs`
- Request data access
- User permissions
- Meta information

## Phase 4: CSRF & i18n (Week 7-8)

### 4.1 CSRF Token Support

**Create**: `src/tags/csrf_token.rs`

```rust
pub struct CsrfTokenTag {
    token: String,
}

impl CsrfTokenTag {
    pub fn new(token: String) -> Self { ... }
    pub fn render(&self) -> String {
        format!(r#"<input type="hidden" name="csrfmiddlewaretoken" value="{}">"#, self.token)
    }
}
```

**Integration**:
```rust
env.add_global("csrf_token", Value::from("token123"));
```

**Tests**:
- `tests/django/tags/test_csrf_tag.rs`

### 4.2 Internationalization

**Create**: `src/i18n_django.rs`

```rust
pub struct DjangoI18n {
    locale: String,
    translations: HashMap<String, HashMap<String, String>>,
}

impl DjangoI18n {
    pub fn trans(&self, msgid: &str) -> String { ... }
    pub fn blocktrans(&self, msgid: &str, context: &Value) -> String { ... }
}
```

**Tests**:
- `tests/django/i18n/test_trans_tag.rs`
- Pluralization
- Context variables in messages

## Phase 5: Testing & Documentation (Week 9-10)

### 5.1 Compatibility Test Suite

**Create**: `tests/django/compatibility/`

```rust
// tests/django/compatibility/test_django_templates.rs
mod django_templates {
    #[test]
    fn test_django_real_world_template_1() { ... }
    
    #[test]
    fn test_django_real_world_template_2() { ... }
    
    // Collect real Django templates from projects like:
    // - Django admin templates
    // - Django Wagtail
    // - Open-source Django projects
}
```

### 5.2 Performance Benchmarks

**Create**: `benches/django_filters.rs`

```rust
use criterion::{black_box, criterion_group, criterion_benchmark};

fn bench_django_filters(c: &mut Criterion) {
    c.bench_function("truncatewords", |b| {
        b.iter(|| {
            // Render template with truncatewords filter
        })
    });
}
```

### 5.3 Documentation

**Create**: `docs/DJANGO_USAGE.md`

- Quick start guide
- Filter reference with examples
- Tag reference
- Context processor examples
- Migration guide: Django → jinja2rs
- Troubleshooting

**Create**: `docs/DJANGO_COMPATIBILITY_MATRIX.md`

- Feature matrix: Django vs jinja2rs
- Version compatibility
- Known limitations
- Workarounds

## File Structure After Implementation

```
src/jinja2rs/
├── src/
│   ├── lib.rs                 # Updated features()
│   ├── compat.rs              # Added DjangoMode
│   ├── environment.rs          # Added with_django_mode()
│   ├── filters/
│   │   ├── mod.rs             # Updated with django module
│   │   ├── django.rs           # NEW: Phase 1-2 filters
│   │   ├── django_datetime.rs  # NEW: Date/time filters
│   │   └── ...
│   ├── loaders/
│   │   ├── mod.rs             # Updated
│   │   ├── django.rs           # NEW: DjangoTemplateLoader
│   │   └── ...
│   ├── tags/
│   │   ├── mod.rs             # NEW module
│   │   ├── django_url.rs       # NEW: {% url %} tag
│   │   ├── django_include.rs   # NEW: {% include %} tag
│   │   ├── csrf_token.rs       # NEW: {% csrf_token %}
│   │   └── ...
│   ├── context.rs              # NEW: Context processors
│   ├── orm.rs                  # NEW: QuerySet wrapper
│   ├── i18n_django.rs          # NEW: Django i18n
│   └── ...
│
├── tests/
│   ├── django/
│   │   ├── filters/
│   │   │   ├── test_string_filters.rs
│   │   │   ├── test_numeric_filters.rs
│   │   │   ├── test_date_filters.rs
│   │   │   └── ...
│   │   ├── tags/
│   │   │   ├── test_url_tag.rs
│   │   │   ├── test_include_tag.rs
│   │   │   ├── test_csrf_tag.rs
│   │   │   └── ...
│   │   ├── loaders/
│   │   │   └── test_app_directories.rs
│   │   ├── context/
│   │   │   ├── test_context_processors.rs
│   │   │   └── test_request_object.rs
│   │   ├── compatibility/
│   │   │   └── test_django_templates.rs
│   │   ├── integration/
│   │   │   └── test_django_rendering.rs
│   │   └── ...
│   └── ...
│
├── benches/
│   ├── django_filters.rs
│   └── ...
│
└── docs/
    ├── DJANGO_MODE_DESIGN.md       (created)
    ├── DJANGO_USAGE.md              (NEW)
    ├── DJANGO_COMPATIBILITY_MATRIX.md (NEW)
    └── ...
```

## Dependencies to Add

**Update `Cargo.toml`**:

```toml
[dependencies]
# Phase 1
html-escape = "0.2"
urlencoding = "2.1"

# Phase 2 (date/time)
chrono = { version = "0.4", optional = true }
humantime = { version = "2.1", optional = true }

# Phase 3 (ORM)
serde_json = "1.0"

# Phase 4 (i18n)
fluent = { version = "0.16", optional = true }

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
assert_matches = "1.5"

[features]
default = ["environment:basic"]
django = []
django-orm = ["django"]
django-i18n = ["django", "fluent"]
```

## Rollout Checklist

- [ ] Phase 1: Core infrastructure
  - [ ] DjangoMode config struct
  - [ ] DjangoEnvironment wrapper
  - [ ] Phase 1 filters (15 filters)
  - [ ] Django template loader
  - [ ] Unit tests
  - [ ] Feature flag in Cargo.toml

- [ ] Phase 2: Advanced filters & tags
  - [ ] Date/time filters (4 filters)
  - [ ] {% url %} tag
  - [ ] {% include %} tag with Django semantics
  - [ ] {% with %} tag
  - [ ] Context processor system

- [ ] Phase 3: ORM & request context
  - [ ] QuerySet wrapper
  - [ ] Request object
  - [ ] User object
  - [ ] Integration with context processors

- [ ] Phase 4: CSRF & i18n
  - [ ] CSRF token injection
  - [ ] {% trans %} tag
  - [ ] {% blocktrans %} tag
  - [ ] Pluralization support

- [ ] Phase 5: Polish & launch
  - [ ] Real-world template testing
  - [ ] Performance benchmarks
  - [ ] Comprehensive documentation
  - [ ] Migration guide
  - [ ] Examples and tutorials

## Success Criteria

1. **Compatibility**: >90% of Django templates render identically
2. **Performance**: ≥3× faster than Django template engine
3. **Coverage**: 15+ Phase 1 filters + 4 Phase 2 filters + major tags
4. **Test coverage**: ≥85% line coverage for Django-specific code
5. **Documentation**: Complete usage guide + API docs + examples
6. **Real-world testing**: Verified on 3+ open-source Django projects

## Next Steps

1. Start with Phase 1 core infrastructure
2. Get community feedback on design
3. Implement incrementally per phase
4. Gather real-world template examples for compatibility testing
5. Benchmark against Django at each phase
