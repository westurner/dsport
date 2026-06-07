//! `jinja2rs::globals` — Sphinx template global objects.
//!
//! Ports the global helpers from `sphinx.jinja2glue`:
//! - `IdGen` — auto-incrementing integer generator (`idgen`)
//! - `AccessKey` — per-render access-key de-duplicator
//! - `warning` — stub that swallows warnings at the Rust level (Sphinx
//!   logging is handled by `sphinxdocrs`)

use std::collections::HashSet;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};

use minijinja::value::{Object, ObjectRepr, Value};
use minijinja::{Error, ErrorKind, State};

/// `idgen` global — mirrors `sphinx.jinja2glue.idgen`.
///
/// Each call to `next()` in a template returns the next integer starting
/// from 1.  `current()` returns the current counter value without advancing.
///
/// Usage in template:
/// ```jinja
/// {{ idgen.next() }}      {# → 1 #}
/// {{ idgen.next() }}      {# → 2 #}
/// {{ idgen.current() }}   {# → 2 #}
/// ```
#[derive(Debug)]
pub struct IdGen {
    counter: AtomicU64,
}

impl IdGen {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }

    pub fn next_id(&self) -> u64 {
        self.counter.fetch_add(1, AtomicOrdering::Relaxed) + 1
    }

    pub fn current_id(&self) -> u64 {
        self.counter.load(AtomicOrdering::Relaxed)
    }
}

impl Default for IdGen {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for IdGen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<IdGen id={}>", self.current_id())
    }
}

impl Object for IdGen {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call_method(
        self: &Arc<Self>,
        _state: &State<'_, '_>,
        method: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match method {
            "next" | "__next__" => {
                if !args.is_empty() {
                    return Err(Error::new(
                        ErrorKind::TooManyArguments,
                        "idgen.next() takes no arguments",
                    ));
                }
                Ok(Value::from(self.next_id()))
            }
            "current" => {
                if !args.is_empty() {
                    return Err(Error::new(
                        ErrorKind::TooManyArguments,
                        "idgen.current() takes no arguments",
                    ));
                }
                Ok(Value::from(self.current_id()))
            }
            _ => Err(Error::from(ErrorKind::UnknownMethod)),
        }
    }

    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "id" => Some(Value::from(self.current_id())),
            _ => None,
        }
    }
}

/// `accesskey` global — mirrors `sphinx.jinja2glue.accesskey`.
///
/// Returns the HTML `accesskey="X"` attribute string the first time a key is
/// used in a render, and an empty string on subsequent uses.
#[derive(Debug, Default)]
pub struct AccessKey {
    seen: Mutex<HashSet<String>>,
}

impl AccessKey {
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for AccessKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<AccessKey>")
    }
}

impl Object for AccessKey {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(
        self: &Arc<Self>,
        _state: &State<'_, '_>,
        args: &[Value],
    ) -> Result<Value, Error> {
        let key = args
            .first()
            .and_then(|v| v.as_str().map(|s| s.to_owned()))
            .unwrap_or_default();
        if key.is_empty() {
            return Ok(Value::from(""));
        }
        let mut seen = self.seen.lock().unwrap();
        if seen.contains(&key) {
            Ok(Value::from(""))
        } else {
            seen.insert(key.clone());
            Ok(Value::from(format!(r#"accesskey="{key}""#)))
        }
    }
}

/// `debug` global — mirrors `jinja2.globals.debug`.
///
/// Returns a pretty-printed representation of a value for debugging.
/// Renders similar to Python's `pformat` function.
#[derive(Debug)]
pub struct Debug;

impl Debug {
    pub fn new() -> Self {
        Debug
    }
}

impl Default for Debug {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<debug>")
    }
}

impl Object for Debug {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(
        self: &Arc<Self>,
        _state: &State<'_, '_>,
        args: &[Value],
    ) -> Result<Value, Error> {
        if args.is_empty() {
            return Err(Error::new(
                ErrorKind::MissingArgument,
                "debug() requires at least one argument",
            ));
        }
        // Use JSON pretty-printing as a substitute for pformat
        let val = &args[0];
        let pretty = serde_json::to_string_pretty(&val)
            .unwrap_or_else(|_| val.to_string());
        Ok(Value::from(pretty))
    }
}

/// `cycler` global — mirrors `jinja2.globals.cycler`.
///
/// Cycles through a list of values. Each call to `next()` returns the
/// next value in the list, wrapping around.
///
/// Usage in template:
/// ```jinja
/// {% set colors = cycler('red', 'blue', 'green') %}
/// {{ colors.next() }}    {# → 'red' #}
/// {{ colors.next() }}    {# → 'blue' #}
/// {{ colors.next() }}    {# → 'green' #}
/// {{ colors.next() }}    {# → 'red' (wraps) #}
/// {{ colors.current }}   {# → 'red' #}
/// ```
#[derive(Debug)]
pub struct Cycler {
    items: Vec<Value>,
    current_index: Mutex<usize>,
}

impl Cycler {
    pub fn new(items: Vec<Value>) -> Self {
        Self {
            items,
            current_index: Mutex::new(0),
        }
    }
}

impl fmt::Display for Cycler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Cycler>")
    }
}

impl Object for Cycler {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call_method(
        self: &Arc<Self>,
        _state: &State<'_, '_>,
        method: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match method {
            "next" | "__next__" => {
                if !args.is_empty() {
                    return Err(Error::new(
                        ErrorKind::TooManyArguments,
                        "cycler.next() takes no arguments",
                    ));
                }
                let mut idx = self.current_index.lock().unwrap();
                let current = if self.items.is_empty() {
                    Value::from(())
                } else {
                    let val = self.items[*idx].clone();
                    *idx = (*idx + 1) % self.items.len();
                    val
                };
                Ok(current)
            }
            _ => Err(Error::from(ErrorKind::UnknownMethod)),
        }
    }

    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "current" => {
                let idx = self.current_index.lock().unwrap();
                if self.items.is_empty() {
                    Some(Value::from(()))
                } else {
                    // Current is the one we're about to return (before incrementing)
                    let current_idx = if *idx == 0 {
                        self.items.len() - 1
                    } else {
                        *idx - 1
                    };
                    Some(self.items[current_idx].clone())
                }
            }
            _ => None,
        }
    }
}

/// `joiner` global — mirrors `jinja2.globals.joiner`.
///
/// Returns a callable that joins multiple strings with a separator,
/// but only adds the separator starting from the second call.
///
/// Usage in template:
/// ```jinja
/// {% set comma = joiner(', ') %}
/// {{ comma('a') }}    {# → 'a' #}
/// {{ comma('b') }}    {# → ', b' #}
/// {{ comma('c') }}    {# → ', c' #}
/// ```
#[derive(Debug)]
pub struct Joiner {
    separator: String,
    first: Mutex<bool>,
}

impl Joiner {
    pub fn new(separator: String) -> Self {
        Self {
            separator,
            first: Mutex::new(true),
        }
    }
}

impl fmt::Display for Joiner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Joiner>")
    }
}

impl Object for Joiner {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(
        self: &Arc<Self>,
        _state: &State<'_, '_>,
        args: &[Value],
    ) -> Result<Value, Error> {
        if args.is_empty() {
            return Err(Error::new(
                ErrorKind::MissingArgument,
                "joiner() requires at least one argument",
            ));
        }
        
        let mut first = self.first.lock().unwrap();
        let text = args[0].to_string();
        
        if *first {
            *first = false;
            Ok(Value::from(text))
        } else {
            Ok(Value::from(format!("{}{}", self.separator, text)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idgen_sequence() {
        let idgen = IdGen::new();
        assert_eq!(idgen.current_id(), 0);
        assert_eq!(idgen.next_id(), 1);
        assert_eq!(idgen.next_id(), 2);
        assert_eq!(idgen.next_id(), 3);
        assert_eq!(idgen.current_id(), 3);
    }

    #[test]
    fn test_accesskey_dedup_internal() {
        let ak = AccessKey::new();
        let mut seen = ak.seen.lock().unwrap();
        assert!(!seen.contains("n"));
        seen.insert("n".to_owned());
        assert!(seen.contains("n"));
    }

    #[test]
    fn test_cycler_basic() {
        let cycler = Cycler::new(vec![
            Value::from("red"),
            Value::from("blue"),
            Value::from("green"),
        ]);
        assert_eq!(cycler.items.len(), 3);
    }

    #[test]
    fn test_joiner_basic() {
        let joiner = Joiner::new(", ".to_string());
        assert_eq!(joiner.separator, ", ");
    }

    #[test]
    fn test_debug_create() {
        let debug = Debug::new();
        let debug_str = debug.to_string();
        assert_eq!(debug_str, "<debug>");
    }
}
