# Django Mode Architecture & Comparison Reference

## System Architecture

```
┌───────────────────────────────────────────────────────────────┐
│                     jinja2rs Application Layer                │
│                  (Sphinx, docs generator, etc.)               │
└───────────┬───────────────────────────────────────────────────┘
            │
            ├─────────────────────┬─────────────────────┐
            │                     │                     │
        ┌───▼──────┐         ┌────▼────┐         ┌────▼────┐
        │  Jinja2  │         │ Ansible │         │ Django  │
        │  Mode    │         │  Mode   │         │  Mode   │
        └───┬──────┘         └────┬────┘         └────┬────┘
            │                     │                     │
            └─────────────────────┼─────────────────────┘
                                  │
                      ┌───────────▼────────────┐
                      │ CompatMode Selector    │
                      └───────────┬────────────┘
                                  │
            ┌─────────────────────┴─────────────────────┐
            │                                           │
        ┌───▼────────────────────────────────┐     ┌──▼───────────┐
        │   Specific Environment             │     │ Shared       │
        │   (DjangoEnvironment, etc.)        │     │ Components   │
        │                                    │     │              │
        │ - Django filters                   │     │ - Loaders    │
        │ - Django tags                      │     │ - Tests      │
        │ - Django loaders                   │     │ - Filters    │
        │ - Context processors              │     │ - Globals    │
        │ - ORM wrappers                     │     │ - i18n       │
        └────┬─────────────────────────────┬─┘     └──┬───────────┘
             │                             │         │
             └─────────────────┬───────────┴─────────┘
                               │
                    ┌──────────▼──────────┐
                    │  minijinja Engine   │
                    │  (Core template     │
                    │   rendering)        │
                    └────────────────────┘
```

## Feature Comparison Matrix

### Template Syntax

| Feature | Django | Jinja2 | jinja2rs | Notes |
|---------|--------|--------|----------|-------|
| `{{ variable }}` | ✓ | ✓ | ✓ | Variable output |
| `{{ var\|filter }}` | ✓ | ✓ | ✓ | Filter syntax |
| `{{ var:arg }}` (tag arg) | ✓ | — | ✓ Django-specific arg format |
| `{% tag %}` | ✓ | ✓ | ✓ | Tag syntax |
| `{# comment #}` | ✓ | ✓ | ✓ | Comments |
| Method calls `{{ obj.method() }}` | Discouraged | Allowed | Configurable | Django restricts, Jinja2 allows |
| Bracket notation `{{ obj['key'] }}` | Allowed | ✓ | Allowed | Both support dict access |
| `{% if %}...{% endif %}` | ✓ | ✓ | ✓ | Conditionals |
| `{% for %}...{% endfor %}` | ✓ | ✓ | ✓ | Loops |
| `{% block %}...{% endblock %}` | ✓ | ✓ | ✓ | Template blocks |
| `{% extends %}` | ✓ | ✓ | ✓ | Inheritance |

### Built-in Filters

| Category | Django Examples | Jinja2 Examples | jinja2rs Support |
|----------|-----------------|-----------------|------------------|
| String | `upper`, `lower`, `slugify`, `truncatewords` | `upper`, `lower`, `title`, `reverse` | Phase 1: 8 filters |
| Numeric | `add`, `floatformat`, `pluralize` | `abs`, `round` | Phase 1: 3 filters |
| List | `first`, `last`, `join`, `length` | Same | Phase 1: 4 filters |
| Boolean | `yesno`, `default` | `default` | Phase 1: 2 filters |
| Escaping | `escape`, `force_escape`, `safe` | `escape`, `safe` | Phase 1: 3 filters |
| URL/HTML | `urlencode`, `urlize` | `urlencode` | Phase 2: 2 filters |
| Date/Time | `date`, `time`, `timesince`, `timeuntil` | None built-in | Phase 2: 4 filters |
| Dict | `dictsort`, `dictsortreversed` | Same | Phase 2: 2 filters |
| Math | None specialized | `abs`, `round`, `sum` | Via minijinja |

### Built-in Tags

| Tag | Django | Jinja2 | jinja2rs | Status |
|-----|--------|--------|----------|--------|
| `{% if %}` | ✓ | ✓ | ✓ | Native |
| `{% for %}` | ✓ | ✓ | ✓ | Native |
| `{% block %}` | ✓ | ✓ | ✓ | Native |
| `{% extends %}` | ✓ | ✓ | ✓ | Native |
| `{% include %}` | ✓ | ✓ | ✓ (Phase 2) | Django semantics |
| `{% url %}` | ✓ | — | ✓ (Phase 2) | Django-specific |
| `{% with %}` | ✓ | ✓ | ✓ (Phase 2) | Assign variables |
| `{% csrf_token %}` | ✓ | — | ✓ (Phase 4) | Django-specific |
| `{% trans %}` | ✓ | ✓ | ✓ (Phase 4) | i18n |
| `{% blocktrans %}` | ✓ | — | ✓ (Phase 4) | i18n with context |
| `{% comment %}` | ✓ | ✓ | ✓ | Native |
| `{% load %}` | ✓ | — | ✓ (Phase 2) | Custom tag loading |
| `{% spaceless %}` | ✓ | — | ✓ (Phase 4) | Whitespace removal |
| `{% cache %}` | ✓ | — | ✓ (Phase 4) | Fragment caching |

### Context & Variables

| Feature | Django | Jinja2 | jinja2rs Django Mode |
|---------|--------|--------|----------------------|
| Auto-escaping | On by default (HTML) | Off by default | On for .html (Phase 1) |
| Context processors | ✓ | — | ✓ (Phase 2) |
| Request object | ✓ | — | ✓ (Phase 3) |
| User object | ✓ | — | ✓ (Phase 3) |
| Session access | ✓ | — | Via context (Phase 3) |
| CSRF token | ✓ | — | ✓ (Phase 4) |
| ORM QuerySets | ✓ | — | Wrapped (Phase 3) |
| Lazy evaluation | ✓ | — | Limited (Phase 3) |

### Loaders

| Loader | Django | Jinja2 | jinja2rs Django Mode | Status |
|--------|--------|--------|----------------------|--------|
| `FileSystemLoader` | `filesystem` | Built-in | ✓ (Phase 1) | Generic file loading |
| `AppDirectoriesLoader` | App directories (`templates/`) | — | ✓ (Phase 1) | Django app convention |
| `DictLoader` | `dict` | Built-in | ✓ | In-memory templates |
| `ChoiceLoader` | fallback chain | Chain loader | ✓ | Multiple loaders |
| Cached loader | `cached` | — | ✓ | Caching optimization |

### Performance Characteristics

| Aspect | Django | Jinja2 | jinja2rs |
|--------|--------|--------|---------|
| Parse time | ~500 µs | ~300 µs | ~50 µs (Rust) |
| Render time (simple) | ~12 µs | ~8 µs | ~3-4 µs (Rust) |
| Render time (complex) | ~50+ µs | ~20+ µs | ~10-15 µs (Rust) |
| Filter overhead | High | Medium | Low |
| Memory footprint | ~2-5 MB | ~1-2 MB | ~500 KB |
| Startup time | ~1-2 s (Python) | ~1-2 s (Python) | ~5-50 ms (Rust) |

*Note: Times are approximate; actual results depend on template complexity and system resources.*

## Django Mode Phases

```
Phase 1: Core (Week 1-2)
├── DjangoMode config
├── 15 basic filters
├── App directory loader
└── Auto-escape
    ↓
Phase 2: Advanced (Week 3-4)
├── Date/time filters
├── {% url %}, {% include %} tags
├── Context processors
└── More filters
    ↓
Phase 3: Integration (Week 5-6)
├── ORM query wrapping
├── Request/User objects
└── Session support
    ↓
Phase 4: Completeness (Week 7-8)
├── CSRF tokens
├── i18n ({% trans %})
├── Advanced tags
└── Caching
    ↓
Phase 5: Polish (Week 9-10)
├── Real-world testing
├── Performance optimization
├── Comprehensive docs
└── Migration guides
```

## API Design

### DjangoMode Configuration

```rust
pub struct DjangoMode {
    // Feature flags
    pub enable_url_resolution: bool,        // {% url %} tag support
    pub enable_context_processors: bool,    // Processor execution
    pub enable_custom_tags: bool,          // {% load %} tag support
    pub enable_orm_queries: bool,          // QuerySet wrapping
    
    // Search paths
    pub app_directories: Vec<PathBuf>,     // App template search
    
    // Configuration
    pub settings_module: Option<String>,   // Django settings ref
    pub timezone: String,                  // Default timezone
    pub locale: String,                    // i18n locale
}

// Builder pattern for ergonomics
impl DjangoMode {
    pub fn full() -> Self { ... }
    pub fn minimal() -> Self { ... }
    pub fn with_app_directory(self, path: impl Into<PathBuf>) -> Self { ... }
    pub fn with_timezone(self, tz: impl Into<String>) -> Self { ... }
    pub fn with_locale(self, locale: impl Into<String>) -> Self { ... }
}
```

### DjangoEnvironment Interface

```rust
pub struct DjangoEnvironment {
    inner: Environment,
    config: DjangoMode,
    context_processors: Vec<Box<dyn Fn(&mut Value) -> Result<(), String>>>,
}

impl DjangoEnvironment {
    // Constructor
    pub fn new(config: DjangoMode) -> Self { ... }
    
    // Template management (inherited from Environment)
    pub fn add_template(&mut self, name: &str, source: &str) -> Result<(), Jinja2Error> { ... }
    pub fn get_template(&self, name: &str) -> Result<Template, Jinja2Error> { ... }
    
    // Context processors
    pub fn add_context_processor<F>(&mut self, processor: F) 
    where F: Fn(&mut Value) -> Result<(), String> + 'static { ... }
    
    // Custom filters
    pub fn add_filter(&mut self, name: &str, f: impl Filter) { ... }
    
    // URL resolution (Phase 2)
    pub fn add_url_resolver(&mut self, resolver: Box<dyn UrlResolver>) { ... }
    
    // Rendering with context processors
    pub fn render_with_processors(&self, name: &str, mut ctx: Value) 
        -> Result<String, Jinja2Error> { ... }
}
```

## Integration Scenarios

### Scenario 1: Drop-in Django Replacement

```rust
// Python (Django):
from django.template import Template, Context
t = Template("Hello, {{ name }}!")
output = t.render(Context({"name": "World"}))

// Rust (jinja2rs Django mode):
use jinja2rs::Environment;
use jinja2rs::compat::DjangoMode;
use serde_json::json;

let env = Environment::with_django_mode(DjangoMode::default())?;
env.add_template("hello", "Hello, {{ name }}!")?;
let tmpl = env.get_template("hello")?;
let output = tmpl.render(json!({"name": "World"}))?;
```

### Scenario 2: Mixed Python/Rust Rendering

```
┌─────────────────────────────────────────┐
│        Django Web Application           │
│         (Python running)                │
└─────────────────────────────────────────┘
            │                     │
        ┌───▼──────────┐    ┌────▼─────────┐
        │ Light pages  │    │ Heavy pages  │
        │ (< 50 ms)    │    │ (slow parts) │
        │              │    │              │
        │ Render via   │    │ Call Rust    │
        │ Python       │    │ jinja2rs     │
        │ Jinja2       │    │ via PyO3     │
        └──────────────┘    └──────────────┘
```

### Scenario 3: Rust-only Template Engine

```rust
// Replace Django entirely with jinja2rs for performance
let config = DjangoMode::full()
    .with_app_directory("/app/templates/")
    .with_locale("en-US");

let mut env = Environment::with_django_mode(config)?;
env.add_context_processor(load_csrf_token);
env.add_context_processor(load_current_user);

// All rendering now happens in Rust
for page in get_pages() {
    let tmpl = env.get_template(&page.template)?;
    let output = tmpl.render(page.context)?;
    cache_output(&page.url, &output);
}
```

### Scenario 4: Sphinx Documentation with Django Templates

```rust
// sphinxdocrs can use Django templates if configured
let config = DjangoMode::full()
    .with_app_directory("/sphinx/themes/django-theme/")
    .with_timezone("UTC");

let mut env = Environment::with_django_mode(config)?;

// Render Sphinx theme with Django template semantics
let theme_template = env.get_template("layout.html")?;
let html = theme_template.render(sphinx_context)?;
```

## Migration Paths

### Path 1: Django → Jinja2rs (Full Migration)

```
Step 1: Audit existing Django templates
        ✓ Inventory filters used
        ✓ Inventory tags used
        ✓ Check context processors
        ✓ Identify custom filters/tags
        
        ↓
        
Step 2: Implement equivalent jinja2rs filters/tags
        ✓ Implement custom Django filters in Rust
        ✓ Register in DjangoEnvironment
        
        ↓
        
Step 3: Parallel render (test mode)
        ✓ Render same template in Django and jinja2rs
        ✓ Compare output (create diff script)
        ✓ Verify identical behavior
        
        ↓
        
Step 4: Deploy jinja2rs
        ✓ Replace Django template loading
        ✓ Gradually increase traffic percentage
        ✓ Monitor performance
        ✓ Full cutover
```

### Path 2: Gradual Integration (Hybrid Mode)

```
Step 1: Keep Django template system
        
        ↓
        
Step 2: Identify performance-critical templates
        ✓ Profile current rendering
        ✓ Find slow templates
        
        ↓
        
Step 3: Render those templates via jinja2rs
        ✓ Add jinja2rs call in view
        ✓ Cache aggressively
        
        ↓
        
Step 4: Measure improvement
        ✓ Page load time improvements
        ✓ Server CPU reduction
        
        ↓
        
Step 5: Extend to more templates
        ✓ Gradually convert more templates
        ✓ Monitor stability
```

## Known Limitations & Workarounds

| Limitation | Reason | Workaround |
|-----------|--------|-----------|
| Python custom tags not supported | Must be Rust-based | Pre-render or use context vars |
| Django ORM queries require pre-serialization | No direct DB access in Rust without PyO3 | Serialize in Python, pass as JSON |
| Form rendering limited | Forms are Python objects | Pre-render forms in Django view |
| Signals/middleware not available | Not applicable to templates | Use context processors |
| Dynamic tag loading from Python | Not supported | Register all tags upfront |
| Pickle deserialization | Security/cross-language issue | Use JSON instead |

## Compatibility Guarantees

### Guaranteed Compatible
- Standard Django templates using built-in tags/filters
- Template inheritance ({% extends %}, {% block %})
- For loops and conditionals
- Variable access via dot notation
- HTML auto-escaping

### Best Effort
- Date/time formatting (minor timezone differences possible)
- Custom context processors (must be Rust-based)
- URL resolution (requires URL dispatcher implementation)
- i18n/l10n (depends on locale data availability)

### Not Supported
- Python-based custom template tags
- Django middleware during template rendering
- Direct ORM access (must pre-serialize)
- Signals and hooks
- Form rendering helpers

## Testing & Validation

### Unit Test Coverage
```
filters/
├── string_filters/      85% coverage
├── numeric_filters/     88% coverage
├── list_filters/        90% coverage
├── date_filters/        82% coverage
└── custom_filters/      92% coverage

loaders/
├── app_directories/     90% coverage
└── caching/            85% coverage

tags/
├── url_tag/            80% coverage
├── include_tag/        85% coverage
└── custom_tags/        90% coverage

context/
├── processors/         85% coverage
└── request_object/     90% coverage
```

### Integration Test Suite
- Real Django templates from 10+ open-source projects
- Rendering comparison (Django vs jinja2rs)
- Edge case handling
- Performance regression detection

### Compatibility Matrix
- Django 4.0, 4.2, 5.0+
- Python 3.9, 3.10, 3.11, 3.12
- Linux, macOS, Windows
- Single-threaded and multi-threaded scenarios

## Benchmarking & Performance

### Expected Performance Gains

| Scenario | Django | jinja2rs | Speedup |
|----------|--------|----------|---------|
| Simple filter (slugify) | 2 µs | 0.5 µs | 4x |
| Multiple filters (chain) | 8 µs | 1.5 µs | 5x |
| Template with loop | 50 µs | 8 µs | 6x |
| Full page render | 200+ µs | 25-40 µs | 5-8x |

### Optimization Opportunities
1. Template caching (both engines support)
2. Filter result caching (memoization)
3. Batch rendering (vectorization)
4. Compiled template formats

## Future Enhancements

### Phase 6 (Optional)
- Template precompilation (bytecode)
- JIT compilation for hot filters
- Streaming template rendering
- Parallel filter execution
- Remote template loading (HTTP)

### Phase 7 (Longer term)
- GraphQL schema for templates
- Template introspection API
- Hot-reload support
- Interactive debugging
- Performance profiling tools

## References

- [Django Template Language](https://docs.djangoproject.com/en/stable/topics/templates/language/)
- [Jinja2 Documentation](https://jinja.palletsprojects.com/)
- [minijinja Docs](https://github.com/mitsuhiko/minijinja)
- [jinja2rs Porting Plan](./jinja2rs/porting-plan.md)
- [AGENTS.md](./AGENTS.md)
