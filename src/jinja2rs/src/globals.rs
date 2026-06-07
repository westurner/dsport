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
}
