# jinja2rs Sandbox: Implementation Guide

**Purpose:** Safe patterns and anti-patterns for template rendering in production.

## Quick Start

### Safe Rendering

```rust
use jinja2rs::SandboxedEnvironment;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = SandboxedEnvironment::new();
    
    // Register a trusted template
    env.add_template(
        "greeting.html",
        "Hello, {{ name }}! You have {{ count }} messages."
    )?;
    
    // Prepare trusted context data
    let ctx = json!({
        "name": "Alice",
        "count": 5
    });
    
    // Render
    let template = env.get_template("greeting.html")?;
    let output = template.render(ctx)?;
    println!("{}", output);  // "Hello, Alice! You have 5 messages."
    
    Ok(())
}
```

### Untrusted Template Source

```rust
// User provides template via form/API
let user_template = req.form("template")?;

// Validate before rendering
if user_template.len() > 10_000 {
    return Err("Template too large".into());
}

// Use SandboxedEnvironment (not Environment)
let env = SandboxedEnvironment::new();
let output = env.render_str(&user_template, safe_context)?;
```

---

## Safety Patterns

### 1. Context Data Isolation

```rust
// ✅ GOOD: Only necessary data in context
#[derive(Serialize)]
struct TemplateContext {
    username: String,
    profile_url: String,
    post_count: u32,
}

let ctx = TemplateContext {
    username: user.name.clone(),
    profile_url: format!("/users/{}", user.id),
    post_count: user.posts.len() as u32,
};

env.render_str(template, ctx)?;

// ❌ BAD: Secrets in context
let ctx = json!({
    "user": user,  // Contains password_hash, api_keys, etc.
});
```

### 2. Custom Filter Registration

```rust
use minijinja::filters::{Filter, State};
use minijinja::Value;

// ✅ GOOD: Safe, escaping filter
fn uppercase_safe(val: Value) -> Result<Value, minijinja::Error> {
    let s = val.as_str().unwrap_or("");
    Ok(Value::from(s.to_uppercase()))
}

env.add_filter("uppercase", uppercase_safe);

// ❌ BAD: Unsafe filter that doesn't escape
fn html_injection(val: Value) -> Result<Value, minijinja::Error> {
    // This bypasses HTML escaping!
    Ok(Value::from(format!("<b>{}</b>", val)))
}
```

### 3. Async Rendering with Timeouts

```rust
use std::time::Duration;
use tokio::time::timeout;

async fn render_with_timeout(
    env: &SandboxedEnvironment,
    template: &str,
    ctx: impl Serialize,
) -> Result<String, Box<dyn std::error::Error>> {
    // Wrap render in timeout (prevents infinite loops)
    let result = timeout(
        Duration::from_secs(5),
        async { env.render_str(template, ctx) }
    ).await?;
    
    result.map_err(|e| e.into())
}

// Usage
let output = render_with_timeout(&env, user_template, ctx).await?;
```

### 4. Resource Limits via OS Mechanisms

```bash
# Limit memory to 512 MB
ulimit -v 524288

# Limit CPU time to 10 seconds
ulimit -t 10

# Then run Rust binary
cargo run --release
```

Or programmatically:

```rust
#[cfg(unix)]
fn set_resource_limits() -> Result<(), std::io::Error> {
    use rlimit::{getrlimit, setrlimit, Resource};
    
    // 512 MB memory
    setrlimit(Resource::AS, 512 * 1024 * 1024, 512 * 1024 * 1024)?;
    
    // 10 second CPU time
    setrlimit(Resource::CPU, 10, 10)?;
    
    Ok(())
}
```

---

## Anti-Patterns (What NOT to Do)

### ❌ Don't: Store Secrets in Context

```rust
// NEVER DO THIS
let ctx = json!({
    "api_key": config.anthropic_key,
    "db_password": config.database_password,
    "user_data": user,  // Contains hashed password
});

env.render_str(malicious_template, ctx)?;
```

**Why:** Templates can exfiltrate secrets via:
- Error messages revealing variable names
- Conditional logic testing for presence
- Filter chains leaking type information

**Safe alternative:**
```rust
let ctx = json!({
    "user_name": user.name,
    "user_id": user.id,
    // Don't include: password_hash, auth_token, etc.
});
```

---

### ❌ Don't: Allow `Environment` Instead of `SandboxedEnvironment`

```rust
// BAD: Regular Environment
let env = Environment::new();
env.render_str(untrusted_template, ctx)?;

// GOOD: Sandboxed
let env = SandboxedEnvironment::new();
env.render_str(untrusted_template, ctx)?;
```

**Difference:**
- `Environment` has relaxed undefined behavior (missing vars → empty)
- `SandboxedEnvironment` uses strict mode (missing vars → error)

---

### ❌ Don't: Implement `Object` Unsafely

```rust
// BAD: Unsafe implementation
struct DangerousObject {
    data: *mut u8,  // Raw pointer!
}

impl Object for DangerousObject {
    fn call_method(&self, state: &State, method: &str, args: &[Value])
        -> Result<Value, minijinja::Error>
    {
        match method {
            "execute" => unsafe {
                // Dereferencing raw pointer is undefined behavior!
                let data = *self.data;
                // ...
            }
            _ => Err(error::UnknownMethod)
        }
    }
}

// GOOD: Safe implementation
struct SafeObject {
    data: Arc<Mutex<Vec<u8>>>,
}

impl Object for SafeObject {
    fn call_method(&self, state: &State, method: &str, args: &[Value])
        -> Result<Value, minijinja::Error>
    {
        match method {
            "get_data" => {
                let guard = self.data.lock().unwrap();
                Ok(Value::from(format!("{:?}", guard.as_slice())))
            }
            _ => Err(error::UnknownMethod)
        }
    }
}
```

---

### ❌ Don't: Render User Templates Without Validation

```rust
// BAD: No validation
let template = req.body().text().await?;
env.render_str(&template, ctx)?;  // No length check, no syntax check

// GOOD: Validate first
let template = req.body().text().await?;

if template.len() > 100_000 {
    return Err("Template exceeds 100 KB limit".into());
}

// Optionally: pre-compile to catch syntax errors early
let compiled = minijinja::Environment::new()
    .compile(&template)?;

env.render_str(&template, ctx)?;
```

---

## Migration from Python Jinja2

### Common Pitfalls

1. **Python objects in context**
   ```python
   # Python Jinja2 — works
   ctx = {"user": user_obj, "posts": posts_list}
   template.render(ctx)
   
   # Rust Jinja2rs — need serializable types
   #[derive(Serialize)]
   struct Context {
       user: UserDTO,
       posts: Vec<PostDTO>,
   }
   ```

2. **Method calls on strings**
   ```jinja2
   {# Python Jinja2 #}
   {{ "hello".upper() }}  → Works
   
   {# jinja2rs #}
   {{ "hello".upper() }}  → Error (no string methods)
   {{ "hello" | upper }}  → Works (use filter instead)
   ```

3. **Attribute access on custom objects**
   ```rust
   // Python: Can access any public attribute
   // Rust: Must implement Object trait explicitly
   
   impl Object for MyType {
       fn get_value(&self, key: &Value) -> Option<Value> {
           match key.as_str()? {
               "name" => Some(Value::from(self.name.clone())),
               "age" => Some(Value::from(self.age)),
               _ => None,  // Other attributes not accessible
           }
       }
   }
   ```

### Test Migration Example

**Before (Python):**
```python
from jinja2 import Environment

def test_greeting():
    env = Environment()
    template = env.from_string("Hello {{ name }}!")
    result = template.render(name="Alice")
    assert result == "Hello Alice!"
```

**After (Rust):**
```rust
use jinja2rs::SandboxedEnvironment;
use serde_json::json;

#[test]
fn test_greeting() {
    let env = SandboxedEnvironment::new();
    let result = env.render_str("Hello {{ name }}!", json!({"name": "Alice"}))
        .expect("render failed");
    assert_eq!(result, "Hello Alice!");
}
```

---

## Performance Tuning

### 1. Cache Compiled Templates

```rust
use std::collections::HashMap;
use std::sync::Arc;

struct TemplateCache {
    templates: Arc<Mutex<HashMap<String, Arc<str>>>>,
    env: SandboxedEnvironment,
}

impl TemplateCache {
    fn render(&self, name: &str, ctx: impl Serialize) -> Result<String> {
        let cache = self.templates.lock().unwrap();
        if let Some(cached) = cache.get(name) {
            // Render from cache (no recompilation)
            return self.env.render_str(cached, ctx);
        }
        Err("Not cached".into())
    }
    
    fn preload(&self, name: &str, template: String) -> Result<()> {
        // Validate syntax
        minijinja::Environment::new().compile(&template)?;
        
        // Cache
        self.templates.lock().unwrap().insert(name, Arc::from(template));
        Ok(())
    }
}
```

### 2. Profile Slow Templates

```rust
use std::time::Instant;

fn render_with_profiling(
    env: &SandboxedEnvironment,
    template: &str,
    ctx: impl Serialize,
) -> Result<(String, u128)> {
    let start = Instant::now();
    let result = env.render_str(template, ctx)?;
    let elapsed = start.elapsed().as_micros();
    
    if elapsed > 1000 {  // > 1 ms
        eprintln!("Slow template: {} µs", elapsed);
    }
    
    Ok((result, elapsed))
}
```

### 3. Batch Rendering

```rust
async fn render_batch(
    env: Arc<SandboxedEnvironment>,
    jobs: Vec<(String, serde_json::Value)>,
) -> Result<Vec<String>> {
    let mut tasks = vec![];
    
    for (template, ctx) in jobs {
        let env = Arc::clone(&env);
        let task = tokio::spawn(async move {
            env.render_str(&template, ctx)
        });
        tasks.push(task);
    }
    
    let mut results = vec![];
    for task in tasks {
        results.push(task.await??);
    }
    
    Ok(results)
}
```

---

## Monitoring and Alerting

### 1. Log Template Rendering Errors

```rust
fn render_with_logging(
    env: &SandboxedEnvironment,
    template: &str,
    ctx: impl Serialize,
) -> Result<String> {
    match env.render_str(template, ctx) {
        Ok(result) => {
            tracing::debug!(template_len = template.len(), "template rendered");
            Ok(result)
        }
        Err(e) => {
            tracing::warn!(error = ?e, template_len = template.len(), "render failed");
            Err(e)
        }
    }
}
```

### 2. Metrics (Prometheus)

```rust
use prometheus::{Counter, Histogram, Registry};

struct TemplateMetrics {
    render_count: Counter,
    render_errors: Counter,
    render_duration: Histogram,
}

impl TemplateMetrics {
    fn record_render(&self, success: bool, duration_ms: f64) {
        self.render_count.inc();
        if !success {
            self.render_errors.inc();
        }
        self.render_duration.observe(duration_ms);
    }
}
```

### 3. Alerting Thresholds

- **Error rate > 5%:** Possible attack or malformed templates
- **P95 latency > 100 ms:** Possible DoS via complex template
- **Memory spike > 100 MB:** Possible resource exhaustion
- **CPU > 80% for 1+ minute:** Possible infinite loop or pathological template

---

## Compliance and Audit

### Checklist for SOC 2 / ISO 27001

- [ ] **Template source control:** All templates in version control with signatures
- [ ] **Change management:** Template changes reviewed and approved
- [ ] **Access control:** Only authorized users can modify templates
- [ ] **Audit logging:** All render failures logged with context
- [ ] **Security testing:** Fuzz tests in CI/CD
- [ ] **Incident response:** Plan for template injection incidents
- [ ] **Key rotation:** API keys in context rotated regularly
- [ ] **Backup/recovery:** Templates backed up; recovery tested

### Logging Format

```json
{
  "timestamp": "2026-06-07T10:30:45.123Z",
  "level": "warn",
  "event": "template_render_error",
  "template_hash": "sha256:...",
  "error": "undefined variable: secret_key",
  "context_vars": ["user", "posts"],
  "duration_us": 245,
  "source_ip": "192.0.2.1"
}
```

---

## FAQ

### Q: Can jinja2rs templates access environment variables?

**A:** No. The template has no access to `os.environ` or similar. Only data explicitly passed in the context is available.

```rust
// Environment variable is NOT accessible
std::env::set_var("SECRET_KEY", "my-secret");
env.render_str("{{ SECRET_KEY }}", json!({}))?;
// Error: undefined variable
```

---

### Q: What about symlink attacks or path traversal?

**A:** Not applicable. jinja2rs templates can't invoke filesystem APIs. Loader implementations (like `FileSystemLoader`) should validate paths, but the template itself has no filesystem access.

```rust
let loader = FileSystemLoader::new("templates");
// Even if template says {{ open("/etc/passwd") }}, it fails:
// Error: getattr not available
```

---

### Q: Can templates cause a denial of service?

**A:** Yes, if unbounded:
- Infinite loops via `{% for i in range(inf) %}`
- Memory exhaustion via list concatenation in loops
- Pathological template nesting

**Mitigation:** Use OS resource limits + render timeout.

---

### Q: Is jinja2rs compatible with Sphinx themes?

**A:** Mostly. Sphinx themes use:
- Filters: `tobool`, `toint`, `todim`, `slice_index` ✅ implemented
- Globals: `idgen`, `accesskey` ✅ implemented
- Methods: `.items()`, `.values()` ⚠️ Use `|items`, `|values` filters

See the porting-plan.md for full compatibility matrix.

---

### Q: Can I use jinja2rs as a drop-in replacement for Python Jinja2?

**A:** Not directly. Differences:
- No method calls on strings (`use |filter instead`)
- No Python stdlib available
- Strict undefined by default (not configurable in current version)
- Custom Object types must implement `Object` trait

Migration guide available in MIGRATION.md.

---

## Further Reading

- [jinja2rs Architecture](../docs/adr/)
- [Sandbox Security Analysis](SANDBOX_SECURITY_ANALYSIS.md)
- [minijinja Documentation](https://docs.rs/minijinja/)
- [OWASP Template Injection](https://owasp.org/www-community/Server-Side_Template_Injection)
- [CWE-94: Improper Control of Generation of Code](https://cwe.mitre.org/data/definitions/94.html)

---

**Last Updated:** 2026-06-07  
**Maintainer:** Security Team
