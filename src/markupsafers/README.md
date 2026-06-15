# markupsafers

A Rust port of Python [`MarkupSafe`](https://markupsafe.palletsprojects.com/) with identical Python API and native Rust library integration.

## What is MarkupSafe?

`MarkupSafe` solves the double-escaping problem in HTML templates:

- Plain strings are **escaped** when inserted into a template.
- `Markup` strings are already safe HTML — they pass through unchanged.
- All string operations on `Markup` values **automatically escape** any plain-string operands before combining them.

## Quick Start — Rust

```rust
use markupsafers::{Markup, escape};

// Escape untrusted input.
let user = "<script>alert('xss')</script>";
let safe = escape(user);
assert_eq!(safe.as_str(), "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");

// Trusted content — wrap without escaping.
let html = Markup::from_safe("<b>bold</b>");

// Concatenation escapes the plain side.
let result = html + "<em>emphasis</em>";
assert_eq!(result.as_str(), "<b>bold</b>&lt;em&gt;emphasis&lt;/em&gt;");

// Unescape back to raw text.
let raw = Markup::from_safe("&lt;b&gt;bold&lt;/b&gt;").unescape();
assert_eq!(raw, "<b>bold</b>");
```

## Quick Start — Python

```python
from markupsafers import Markup, escape

# Escape untrusted input.
safe = escape("<script>alert('xss')</script>")
# Markup("&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;")

# Already safe? escape() is a no-op.
html = Markup("<b>bold</b>")
assert escape(html) is html   # Not actually 'is', but equal and safe

# Concatenation escapes plain strings.
result = Markup("<b>") + "<em>"
assert result == Markup("<b>&lt;em&gt;")
```

## API Reference

### Rust

| Symbol | Description |
|--------|-------------|
| `Markup::from_safe(s)` | Wrap without escaping. Caller guarantees `s` is safe HTML. |
| `Markup::escape(s)` | Escape `s` and return `Markup`. |
| `Markup::unescape()` | Reverse HTML entities (`&amp;` → `&`, etc.). |
| `Markup + &str` | Append, escaping the `&str` first. |
| `Markup + Markup` | Append without extra escaping. |
| `Markup::join(iter)` | Join items, escaping each one. |
| `Markup::join_markup(iter)` | Join `Markup` items without escaping. |
| `Markup::format_args(args)` | `{}`-style format with argument escaping. |
| `escape(s)` | Free function; equals `Markup::escape`. |
| `escape_value(m)` | Clone of already-safe `Markup`. |
| `escape_silent(opt)` | Like `escape`, but returns empty `Markup` for `None`. |
| `soft_str(m)` | Return `&str` reference without escaping. |
| `MarkupEscapeWriter` | `fmt::Write` adapter that escapes writes. |

### Python

| Symbol | Description |
|--------|-------------|
| `Markup(s)` | Escape `s` and wrap. |
| `Markup.__new_safe__(s)` | Wrap without escaping. |
| `Markup.__html__()` | Return the inner string (the `__html__` protocol). |
| `Markup.escape(s)` | Class method; escape and wrap. |
| `Markup.unescape()` | Reverse HTML entities. |
| `Markup.format(*args)` | `{}`-style format with argument escaping. |
| `escape(s)` | Module-level; escapes or no-ops if `s.__html__` exists. |
| `escape_silent(s)` | Like `escape`; empty `Markup` for `None`. |
| `soft_str(s)` | Return `str` without escaping. |

## Differences from Python MarkupSafe

| Aspect | Python MarkupSafe | markupsafers |
|--------|-------------------|--------------|
| Implementation language | C extension + Python fallback | Pure Rust |
| Python API compatibility | Authoritative | Drop-in compatible |
| `__html__()` protocol | `str` subclass | Delegation via `PyO3` |
| Thread safety | GIL-protected | `Send + Sync` |
| Allocation | CPython object overhead | Thin `String` wrapper |
| `Markup % args` | Supported | Not yet (use `format_args`) |

## minijinja Integration

With the `minijinja` Cargo feature enabled:

```rust
use markupsafers::minijinja_compat::{
    markup_auto_escape_callback,
    escape_filter,
    safe_filter,
    markup_to_value,
};
use minijinja::Environment;

let mut env = Environment::new();

// Enable HTML auto-escaping on .html / .htm / .xml templates.
env.set_auto_escape_callback(markup_auto_escape_callback);

// Register escape filters.
env.add_filter("escape", escape_filter);
env.add_filter("e", escape_filter);
env.add_filter("safe", safe_filter);

// Insert a pre-safe value into context.
let safe_html = markup_to_value(Markup::from_safe("<b>trusted</b>"));
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `extension-module` | off | Build PyO3 Python extension wheel |
| `minijinja` | off | Enable minijinja `Object` impl and filters |

## HTML Escaping Rules

Matches the Python MarkupSafe escaping table exactly:

| Character | Entity |
|-----------|--------|
| `&`  | `&amp;` |
| `<`  | `&lt;` |
| `>`  | `&gt;` |
| `"`  | `&#34;` |
| `'`  | `&#39;` |

## License

BSD-3-Clause — same license as the upstream Python `MarkupSafe` package.
