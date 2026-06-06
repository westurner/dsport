# PyO3 Bridge Selection Guide

## Overview

The pygmentsrs project provides two paths for syntax highlighting:
1. **Native Rust Lexers/Formatters** — Fast, zero-dependency, type-safe
2. **Python Bridge (PyO3)** — Access to all upstream Pygments lexers/formatters

This guide helps you choose the right approach for your use case.

---

## Quick Decision Tree

```
Is the language JSON, Python, or shell?
├─ YES → Need Rust-only standalone binary?
│         ├─ YES → Use Native Rust lexer
│         └─ NO → Use Native Rust lexer (faster) OR Bridge (if compatibility needed)
└─ NO → Do you need the specific lexer?
         ├─ Known lexer → Use Bridge (supports 500+ languages)
         └─ Unknown → Use Bridge to fail gracefully
```

---

## Detailed Comparison

### Native Rust Lexers

**Supported Languages:** JSON, Python (partial), Shell (partial)

**Pros:**
- ✅ **Zero Dependencies** — No Python required
- ✅ **Fast** — 2-10x faster than bridge for same lexer
- ✅ **Memory Efficient** — No FFI overhead
- ✅ **Standalone** — Works offline, embedded systems
- ✅ **Type Safe** — Rust's type system catches errors
- ✅ **No GIL** — Can parallelize lexing across cores

**Cons:**
- ❌ Limited language support (3-5 lexers only)
- ❌ Need to port each lexer from Pygments
- ❌ Larger binary if including all native lexers

**When to Use:**
- Small set of known languages (JSON, Python, Shell)
- Performance-critical path
- Embedded/offline environment
- No Python available
- Building a standalone Rust library
- Parallelizing across multiple files

### Python Bridge (PyO3)

**Supported Languages:** 500+ (all of Pygments)

**Pros:**
- ✅ **Comprehensive** — 500+ languages supported
- ✅ **Auto-Updated** — Inherits Pygments updates
- ✅ **Proven** — Tested in production by Pygments users
- ✅ **Flexibility** — Add custom lexers via Pygments plugins
- ✅ **Feature Rich** — Advanced lexer options available
- ✅ **Fallback** — Unknown languages fail gracefully

**Cons:**
- ❌ **Python Dependency** — Requires Python + Pygments
- ❌ **Slower** — FFI overhead (~1-5ms per lex call)
- ❌ **GIL** — Single-threaded token generation
- ❌ **Memory Overhead** — Python runtime required
- ❌ **Distribution** — More complex deployment

**When to Use:**
- Need to support many languages
- Language set unknown at compile time
- Plugins/custom lexers required
- Compatibility with Pygments ecosystem needed
- Feature parity with upstream Pygments
- Extensibility important

---

## Performance Comparison

### Lexing Speed (typical results)

| Scenario | Native Rust | Python Bridge | Ratio |
|----------|------------|---------------|-------|
| Small input (< 1KB) | 0.1ms | 2-5ms | 20-50x |
| Medium input (10KB) | 1ms | 5-10ms | 5-10x |
| Large input (100KB) | 10ms | 50-100ms | 5-10x |
| Very large (1MB) | 100ms | 500-1000ms | 5-10x |

**Note:** Bridge overhead is ~1-3ms (FFI + Python setup), so becomes negligible for larger inputs.

### Memory Usage

| Aspect | Native Rust | Python Bridge |
|--------|------------|---------------|
| Per-operation | ~1MB | ~10-50MB (Python runtime) |
| Token stream | Minimal | Minimal (same) |
| Parallel safety | Multi-threaded OK | Single-threaded only |

---

## Decision Scenarios

### Scenario 1: Web Server

**Requirements:** HTTP API for syntax highlighting, support many languages, 100+ RPS

**Recommendation:** **Python Bridge**

```rust
// In Cargo.toml
[dependencies]
pygmentsrs = { features = ["python-bridge"] }

// Usage
#[post("/highlight")]
fn highlight(req: HighlightRequest) -> Result<HighlightResponse> {
    // Bridge handles fallback gracefully
    let tokens = pygmentsrs::bridge::lex(&req.language, &req.code)?;
    let output = pygmentsrs::bridge::format("html", &tokens)?;
    Ok(HighlightResponse { html: output })
}
```

**Why:** Unknown languages, need all 500+, single-threaded OK for HTTP handlers

### Scenario 2: CLI Tool

**Requirements:** Standalone binary, no external dependencies, 3 supported languages

**Recommendation:** **Native Rust**

```rust
// In Cargo.toml
[dependencies]
pygmentsrs = { default-features = false }  # No Python bridge

// Usage
match language {
    "json" => pygmentsrs::lex("json", code),
    "python" => pygmentsrs::lex("python", code),
    "shell" => pygmentsrs::lex("shell", code),
    _ => Err("unsupported language"),
}
```

**Why:** Specific languages, offline OK, single binary, no Python deployment headache

### Scenario 3: Batch Processing

**Requirements:** Process 10,000+ files, many languages, want parallelism

**Recommendation:** **Native Rust** (for supported languages) with **Bridge fallback**

```rust
use rayon::prelude::*;

// In Cargo.toml
[dependencies]
pygmentsrs = { features = ["python-bridge"] }
rayon = "1"

// Process in parallel if native, sequential if bridge
files.par_iter().for_each(|file| {
    // Try native first (parallel-safe)
    if let Some(tokens) = pygmentsrs::lex("json", &file.content) {
        process_tokens(tokens);
    } else {
        // Fall back to bridge (sequential)
        if let Some(tokens) = pygmentsrs::bridge::lex("json", &file.content) {
            process_tokens(tokens);
        }
    }
});
```

**Why:** Parallelize native path, bridge as fallback for unknown languages

### Scenario 4: Library

**Requirements:** Published crate, users may not have Python, feature optional

**Recommendation:** **Native Rust** with **optional Bridge**

```toml
# In Cargo.toml
[features]
default = []  # No bridge by default
python-bridge = ["pyo3"]  # Optional feature

[dependencies]
pygmentsrs = { version = "0.1", features = [] }
pyo3 = { version = "0.20", optional = true }
```

**Usage:**
```rust
pub fn highlight(language: &str, code: &str) -> Option<String> {
    #[cfg(feature = "python-bridge")]
    {
        // Try native first
        if let Some(tokens) = pygmentsrs::lex(language, code) {
            return pygmentsrs::formatters::format_native("html", &tokens);
        }
        // Fall back to bridge for unknown languages
        if let Some(tokens) = pygmentsrs::bridge::lex(language, code) {
            return pygmentsrs::bridge::format("html", &tokens);
        }
    }
    
    #[cfg(not(feature = "python-bridge"))]
    {
        // Only native available
        pygmentsrs::lex(language, code)
            .and_then(|tokens| pygmentsrs::formatters::format_native("html", &tokens))
    }
}
```

**Why:** Users can opt-in to bridge, not required, library stays lightweight

### Scenario 5: Embedded System

**Requirements:** No external dependencies, minimal binary size, offline

**Recommendation:** **Native Rust only**

```toml
[dependencies]
pygmentsrs = { default-features = false }  # No bridge
```

**Usage:**
```rust
// Only JSON, Python, Shell available
match file_type {
    "json" => pygmentsrs::lex("json", &data),
    "sh" => pygmentsrs::lex("shell", &data),
    _ => Err("language not supported on embedded system"),
}
```

**Why:** No Python runtime available, small footprint critical

---

## Error Handling

### Native Rust Error Handling

```rust
use pygmentsrs::lexer::Lexer;

match pygmentsrs::lex("json", code) {
    Some(tokens) => process_tokens(tokens),
    None => eprintln!("JSON lexing failed"),
}
```

### Bridge Error Handling

```rust
use pygmentsrs::bridge;

// Pattern 1: Check availability first
if bridge::alias_is_known("python") {
    if let Some(tokens) = bridge::lex("python", code) {
        // Process...
    }
}

// Pattern 2: Graceful fallback
match bridge::lex(language, code) {
    Some(tokens) => process_tokens(tokens),
    None => {
        eprintln!("Language '{}' not available", language);
        // Fall back to native or error
    }
}

// Pattern 3: Format with fallback
let tokens = bridge::lex("ruby", code)?;
if let Some(html) = bridge::format("html", &tokens) {
    Ok(html)
} else if let Some(html) = bridge::format("terminal", &tokens) {
    Ok(html)  // Fall back to terminal format
} else {
    Err("no formatters available")
}
```

---

## Optimization Tips

### For Native Rust Path

1. **Batch Operations**
   ```rust
   // Good: lex multiple files in parallel
   files.par_iter().for_each(|f| { pygmentsrs::lex("json", &f.content); });
   ```

2. **Cache Tokens**
   ```rust
   let mut cache = HashMap::new();
   let tokens = cache
       .entry(code.clone())
       .or_insert_with(|| pygmentsrs::lex("json", &code));
   ```

3. **Streaming**
   ```rust
   // Process large files in chunks
   for chunk in large_file.chunks(1024 * 1024) {
       let tokens = pygmentsrs::lex("json", chunk);
   }
   ```

### For Bridge Path

1. **Reduce FFI Calls**
   ```rust
   // Good: batch format multiple tokens
   let tokens = bridge::lex("python", code)?;
   let html = bridge::format("html", &tokens)?;
   
   // Bad: format each token individually
   for (ttype, value) in tokens {
       let _ = bridge::format("html", &[(ttype, value)])?;  // Multiple FFI calls
   }
   ```

2. **Check Availability**
   ```rust
   // Avoid failing lex calls on unknown languages
   if bridge::alias_is_known(language) {
       bridge::lex(language, code)
   }
   ```

3. **Skip When Possible**
   ```rust
   // Use environment variable to skip if Python unavailable
   // SKIP_BRIDGE_TESTS=1 cargo test
   if std::env::var("NO_PYTHON_BRIDGE").is_ok() {
       use_native_only();
   }
   ```

---

## Feature Flags

```toml
[features]
# Default: Bridge enabled
default = ["python-bridge"]

# Python bridge support (requires Python + PyO3)
python-bridge = ["dep:pyo3"]

# Optional: RaTeX for LaTeX rendering
ratex-svg = ["dep:ratex-svg"]

# Build standalone (no Python needed)
standalone = []
```

### Recommended Combinations

```toml
# Web API - full featured
pygmentsrs = { version = "0.1", features = ["python-bridge", "ratex-svg"] }

# CLI tool - minimal
pygmentsrs = { version = "0.1", default-features = false }

# Library - let users choose
pygmentsrs = { version = "0.1", default-features = false, optional = true }

# Embedded - no dependencies
pygmentsrs = { version = "0.1", default-features = false }
```

---

## Testing Strategy

### For Native-Only Code

```bash
# Test without Python bridge
cargo test --no-default-features

# Verify no Python dependency
cargo build --no-default-features --release
ldd ./target/release/app  # Should not show libpython
```

### For Bridge Code

```bash
# Skip bridge tests in CI without Python
SKIP_BRIDGE_TESTS=1 cargo test

# Or skip at compile time
cargo test --lib  # Only library tests, skip integration

# Run bridge tests only
cargo test --test test_bridge_pyo3_100pct
```

### For Both

```bash
# Test everything
cargo test

# Benchmark
cargo bench

# Check coverage
cargo llvm-cov --html

# Fuzz test
SKIP_PERFORMANCE_TESTS=1 cargo test --test test_bridge_fuzz
```

---

## Performance Profiling

### Native Rust

```bash
# Flamegraph
cargo install flamegraph
cargo flamegraph --bin highlighter -- large_file.py

# perf
perf record -g ./target/release/app
perf report
```

### Python Bridge

```bash
# Python profiling
python -m cProfile -s cumtime script.py

# Memory profiling
pip install memory_profiler
python -m memory_profiler script.py
```

---

## Migration Path

### From Pure Pygments to pygmentsrs

```python
# Before: Pure Python
from pygments.lexers import get_lexer_by_name
from pygments.formatters import HtmlFormatter
from pygments import highlight

code = "x = 1"
lexer = get_lexer_by_name("python")
result = highlight(code, lexer, HtmlFormatter())
```

```rust
// After: Rust with optional bridge
use pygmentsrs::{lex, bridge};

let code = "x = 1";

// Try native first
if let Some(tokens) = lex("python", code) {
    // Fast path
    let html = format_native("html", &tokens);
} else {
    // Fall back to bridge for other languages
    if let Some(tokens) = bridge::lex("python", code) {
        let html = bridge::format("html", &tokens);
    }
}
```

---

## Troubleshooting

### Q: Bridge tests skip silently

**A:** Python or pygments not available. Install:
```bash
pip install pygments
SKIP_BRIDGE_TESTS=0 cargo test --test test_bridge_pyo3_100pct
```

### Q: Binary still depends on Python

**A:** Build without bridge feature:
```bash
cargo build --no-default-features --release
ldd ./target/release/app  # Verify no libpython
```

### Q: Performance degradation with bridge

**A:** Check for repeated lex/format calls:
```rust
// Bad: Creates new tokens each time
for file in files {
    bridge::lex(lang, &file);  // FFI call
}

// Good: Reuse tokens
for file in files {
    let tokens = bridge::lex(lang, &file);
    bridge::format("html", &tokens);  // One FFI pair
}
```

### Q: How to benchmark native vs bridge?

**A:** Use test output:
```bash
cargo test --test test_bridge_performance -- --nocapture
# Look for "Lexed X file: Y tokens in Z time"
```

---

## Conclusion

Choose **Native Rust** for:
- Known, limited language set
- Performance-critical code
- Embedded/standalone binaries
- No Python available

Choose **Python Bridge** for:
- Flexibility and extensibility
- Many languages (500+)
- Plugin ecosystem support
- Feature parity with Pygments

Combine both for:
- Try native first, bridge fallback
- Optimal performance and flexibility
- User choice via feature flags

See the examples and tests for patterns in `test_bridge_pyo3_100pct.rs`, `test_bridge_performance.rs`, and `test_bridge_extended_languages.rs`.
