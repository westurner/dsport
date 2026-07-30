//! Native (PyO3-free) event bus for the [`crate::application::SphinxApp`]
//! read/write pipeline.
//!
//! This is deliberately **not** the same type as [`crate::events::EventManager`]
//! (which is a `#[pyclass]` bound to a Python `app` object and Python
//! callables, used by the hybrid Python bridge). `AppEventManager` instead
//! lets pure-Rust code — `SphinxApp`, `BuildEnvironment` — emit the same
//! core event names in the same upstream order without requiring an active
//! Python interpreter. [`crate::app_facade`] bridges the two: it lets a
//! Python extension's `setup(app)` register a Python callable that gets
//! wrapped as a native listener here.
//!
//! ## What is ported
//!
//! Mirrors the subset of `sphinx.events.EventManager` needed by the H2
//! read/write pipeline: `connect`, `disconnect`, `emit`, priority-ordered
//! dispatch, and the `core_events` name list (`sphinx.application.builtin_extensions`
//! event registrations collapsed into one constant, since this port has no
//! per-extension event registration yet).
//!
//! **Listener errors now propagate** (closes the former "listener errors
//! are swallowed" deviation): [`AppEventManager::emit`] returns
//! `Result<(), EventError>` and stops dispatching to any later listener the
//! moment one fails, mirroring upstream `EventManager.emit`'s fail-fast
//! behavior. [`crate::app_facade::PyAppFacade::connect`] converts a
//! listener's `PyErr` into an [`EventError`] (message includes the
//! callback's `repr()` and the event name, matching the
//! `"Handler {handler!r} for event '{name}' threw an exception"` text the
//! PyO3-facing [`crate::events::EventManager::emit`] already produces) so a
//! failing Python extension callback now aborts the build the same way a
//! failing native listener does. **Remaining accepted deviation:** this
//! native bus has no concept of `allowed_exceptions` / `SphinxError`
//! pass-through or an `app.pdb` short-circuit — every listener error is
//! always wrapped and always propagates. Full fidelity for those two knobs
//! stays on the PyO3-facing [`crate::events::EventManager`], which is a
//! `Bound` Python object and can inspect exception types/`app.pdb`
//! directly; no native (non-Python) listener needs either knob yet.
//! **Also accepted:** listeners must not call `connect`/`disconnect`/`emit`
//! reentrantly on the same [`SharedEvents`] handle while being invoked —
//! `SharedEvents` is a `RefCell`, and a nested mutable borrow panics.
//! Upstream's Python `EventManager` has no such restriction; no native
//! listener needs reentrancy yet.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// Error surfaced when an event listener fails during
/// [`AppEventManager::emit`]. Mirrors the *message shape* of upstream's
/// `ExtensionError`-wrapped listener failure — see this module's
/// "Listener errors now propagate" doc note above for what is and isn't
/// replicated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventError(pub String);

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for EventError {}

/// Core event names, in the order `sphinx.application.Sphinx` fires them
/// across a full build. Mirrors `sphinx.events.EventManager.core_events`
/// (upstream lists these via `builtin_extensions` + `Sphinx.__init__` /
/// `Sphinx.build`).
pub const CORE_EVENTS: &[&str] = &[
    "config-inited",
    "builder-inited",
    "env-get-outdated",
    "env-before-read-docs",
    "source-read",
    "doctree-read",
    "env-updated",
    "env-check-consistency",
    "doctree-resolved",
    "html-page-context",
    "build-finished",
];

/// A positional argument passed to an event listener.
///
/// Mirrors the small subset of argument shapes the native pipeline needs
/// to pass (docnames, docname lists). Richer payloads (config objects,
/// doctrees) are out of scope until the events fire alongside real
/// `Domain`/`Config` Python objects (H3/H5/H6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventArg {
    None,
    Str(String),
    StrList(Vec<String>),
}

/// Boxed listener callback. Not `Send`/`Sync`: listeners may wrap Python
/// callables (via [`crate::app_facade::PyAppFacade`]), which are only
/// safely callable from the thread holding the GIL. [`SharedEvents`] is
/// therefore an `Rc<RefCell<_>>`, not an `Arc<Mutex<_>>` — this bus is not
/// meant to be shared across threads. Returns `Result` so a failing
/// listener (e.g. a Python callback that raised) can abort the rest of the
/// dispatch instead of being silently swallowed — see [`EventError`].
type Handler = Box<dyn FnMut(&[EventArg]) -> Result<(), EventError>>;

struct Listener {
    id: usize,
    priority: i64,
    handler: Handler,
}

/// Native event bus. Records every emitted event name (in order) so tests
/// can assert on emission order, and dispatches to registered listeners in
/// ascending-priority order (matches upstream's stable sort by priority).
#[derive(Default)]
pub struct AppEventManager {
    events: Vec<String>,
    listeners: Vec<(String, Listener)>,
    next_id: usize,
    log: Vec<String>,
}

/// Shared handle to an [`AppEventManager`], cloneable so both `SphinxApp`
/// and a [`crate::app_facade::PyAppFacade`] constructed during
/// `load_extension` can register/emit against the same listener list.
/// Single-threaded (`Rc<RefCell<_>>`) — see [`Handler`].
pub type SharedEvents = Rc<RefCell<AppEventManager>>;

impl AppEventManager {
    pub fn new() -> Self {
        Self {
            events: CORE_EVENTS.iter().map(|s| (*s).to_string()).collect(),
            listeners: Vec::new(),
            next_id: 0,
            log: Vec::new(),
        }
    }

    /// Wrap a fresh manager in a [`SharedEvents`] handle.
    pub fn shared() -> SharedEvents {
        Rc::new(RefCell::new(Self::new()))
    }

    /// Register a custom event name (no-op if already known, unlike
    /// upstream's `ExtensionError` — kept permissive since native callers
    /// don't need strict duplicate detection).
    pub fn add(&mut self, name: impl Into<String>) {
        let name = name.into();
        if !self.events.contains(&name) {
            self.events.push(name);
        }
    }

    /// Connect a listener to `name`. Returns a listener id usable with
    /// [`disconnect`](Self::disconnect).
    pub fn connect(
        &mut self,
        name: impl Into<String>,
        priority: i64,
        handler: impl FnMut(&[EventArg]) -> Result<(), EventError> + 'static,
    ) -> usize {
        let name = name.into();
        self.add(name.clone());
        let id = self.next_id;
        self.next_id += 1;
        self.listeners.push((
            name,
            Listener {
                id,
                priority,
                handler: Box::new(handler),
            },
        ));
        id
    }

    /// Remove a listener previously registered with [`connect`](Self::connect).
    pub fn disconnect(&mut self, listener_id: usize) {
        self.listeners.retain(|(_, l)| l.id != listener_id);
    }

    /// Emit `name` with `args`, calling every matching listener in
    /// ascending-priority order. Records `name` in the emission log
    /// regardless of whether any listener is registered.
    ///
    /// Stops at (and returns) the first listener error — mirrors upstream
    /// `EventManager.emit`'s fail-fast behavior: a later listener never
    /// runs once an earlier one has failed.
    pub fn emit(&mut self, name: &str, args: &[EventArg]) -> Result<(), EventError> {
        self.add(name);
        self.log.push(name.to_string());

        let mut matching: Vec<usize> = self
            .listeners
            .iter()
            .enumerate()
            .filter(|(_, (n, _))| n == name)
            .map(|(i, _)| i)
            .collect();
        matching.sort_by_key(|&i| self.listeners[i].1.priority);

        for i in matching {
            (self.listeners[i].1.handler)(args)?;
        }
        Ok(())
    }

    /// The full emission log, in call order (may contain duplicates —
    /// `source-read`/`doctree-read` fire once per document).
    pub fn emitted(&self) -> &[String] {
        &self.log
    }

    /// Known event names, in registration order.
    pub fn known_events(&self) -> &[String] {
        &self.events
    }

    /// Number of currently registered listeners (across all events).
    pub fn listener_count(&self) -> usize {
        self.listeners.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn emit_records_log_even_without_listeners() {
        let mut mgr = AppEventManager::new();
        mgr.emit("config-inited", &[]).unwrap();
        mgr.emit("builder-inited", &[]).unwrap();
        assert_eq!(mgr.emitted(), &["config-inited", "builder-inited"]);
    }

    #[test]
    fn connect_dispatches_in_priority_order() {
        let mut mgr = AppEventManager::new();
        let order = Rc::new(RefCell::new(Vec::<i64>::new()));

        let o1 = order.clone();
        mgr.connect("doctree-resolved", 900, move |_| {
            o1.borrow_mut().push(900);
            Ok(())
        });
        let o2 = order.clone();
        mgr.connect("doctree-resolved", 100, move |_| {
            o2.borrow_mut().push(100);
            Ok(())
        });
        let o3 = order.clone();
        mgr.connect("doctree-resolved", 500, move |_| {
            o3.borrow_mut().push(500);
            Ok(())
        });

        mgr.emit("doctree-resolved", &[]).unwrap();
        assert_eq!(*order.borrow(), vec![100, 500, 900]);
    }

    #[test]
    fn disconnect_removes_listener() {
        let mut mgr = AppEventManager::new();
        let calls = Rc::new(RefCell::new(0));
        let c = calls.clone();
        let id = mgr.connect("build-finished", 0, move |_| {
            *c.borrow_mut() += 1;
            Ok(())
        });
        mgr.emit("build-finished", &[]).unwrap();
        mgr.disconnect(id);
        mgr.emit("build-finished", &[]).unwrap();
        assert_eq!(*calls.borrow(), 1);
    }

    #[test]
    fn emit_stops_at_first_listener_error_and_propagates_it() {
        let mut mgr = AppEventManager::new();
        let calls = Rc::new(RefCell::new(Vec::<&'static str>::new()));

        let c1 = calls.clone();
        mgr.connect("build-finished", 0, move |_| {
            c1.borrow_mut().push("first");
            Err(EventError("boom".to_string()))
        });
        let c2 = calls.clone();
        mgr.connect("build-finished", 10, move |_| {
            c2.borrow_mut().push("second");
            Ok(())
        });

        let err = mgr.emit("build-finished", &[]).unwrap_err();
        assert_eq!(err.0, "boom");
        // The second (lower-priority) listener never ran once the first failed.
        assert_eq!(*calls.borrow(), vec!["first"]);
    }

    #[test]
    fn unknown_event_names_are_recorded_too() {
        let mut mgr = AppEventManager::new();
        mgr.emit("my-custom-event", &[]).unwrap();
        assert!(mgr.known_events().iter().any(|n| n == "my-custom-event"));
    }

    #[test]
    fn core_events_known_by_default() {
        let mgr = AppEventManager::new();
        for name in CORE_EVENTS {
            assert!(mgr.known_events().iter().any(|n| n == name));
        }
    }
}
