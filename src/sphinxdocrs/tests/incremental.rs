//! **H8a/H8b** — incremental rebuild integration tests.
//!
//! Each test constructs a fresh [`SphinxApp`] against the same
//! `srcdir`/`outdir`/`doctreedir` more than once (mirroring separate
//! `sphinx-build-rs` process invocations against the same project),
//! exercising `SphinxApp::read`'s persisted-environment reload
//! (`BuildEnvironment::load_persisted`/`save_persisted`, **H8b**) and
//! outdated-document filtering (`BuildEnvironment::get_outdated`,
//! **H8a**): an unchanged rebuild should re-read nothing, touching one
//! file should re-read only that file, a newly-added file should be
//! read once, a removed file should be purged, and `-E`/config changes
//! should force a full re-read.

use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, SystemTime};

use sphinxdocrs::app_events::EventArg;
use sphinxdocrs::application::SphinxApp;
use tempfile::TempDir;

/// Connect a listener to `event` that records the docname argument (the
/// first `EventArg::Str`) of every emission, returning a handle to the
/// captured list.
fn capture_docname_event(
    app: &mut SphinxApp,
    event: &str,
) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    let captured = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let captured_for_closure = captured.clone();
    app.events
        .borrow_mut()
        .connect(event.to_string(), 500, move |args: &[EventArg]| {
            if let Some(EventArg::Str(name)) = args.first() {
                captured_for_closure.borrow_mut().push(name.clone());
            }
        });
    captured
}

/// Set `path`'s mtime comfortably in the future so a follow-up build's
/// mtime comparison against the previous `all_docs` read-time is
/// unambiguous, regardless of filesystem mtime resolution or how fast
/// the test itself runs.
fn touch_forward(path: &Path) {
    let file = std::fs::File::options().write(true).open(path).unwrap();
    file.set_modified(SystemTime::now() + Duration::from_secs(120))
        .unwrap();
}

fn write_project(src: &Path) {
    std::fs::write(
        src.join("index.rst"),
        "Welcome\n=======\n\nHome.\n\n.. toctree::\n\n   guide\n",
    )
    .unwrap();
    std::fs::write(src.join("guide.rst"), "Guide\n=====\n\nBody.\n").unwrap();
}

#[test]
fn second_build_of_unchanged_project_rereads_nothing() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    write_project(src.path());

    let mut app1 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    let reads1 = capture_docname_event(&mut app1, "source-read");
    app1.build().unwrap();
    assert_eq!(
        reads1.borrow().len(),
        2,
        "first build should read both documents; got {:?}",
        reads1.borrow()
    );

    let mut app2 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    let reads2 = capture_docname_event(&mut app2, "source-read");
    app2.build().unwrap();
    assert!(
        reads2.borrow().is_empty(),
        "second build of an unchanged project should re-read nothing; got {:?}",
        reads2.borrow()
    );
}

#[test]
fn touching_one_file_rereads_only_that_file() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    write_project(src.path());
    let index_path = src.path().join("index.rst");

    let mut app1 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    app1.build().unwrap();

    std::fs::write(
        &index_path,
        "Welcome\n=======\n\nHome, updated.\n\n.. toctree::\n\n   guide\n",
    )
    .unwrap();
    touch_forward(&index_path);

    let mut app2 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    let reads2 = capture_docname_event(&mut app2, "source-read");
    app2.build().unwrap();
    assert_eq!(
        *reads2.borrow(),
        vec!["index".to_string()],
        "only the touched document should be re-read"
    );
}

#[test]
fn newly_added_file_is_read_once() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    write_project(src.path());

    let mut app1 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    app1.build().unwrap();

    std::fs::write(src.path().join("extra.rst"), "Extra\n=====\n\nMore.\n").unwrap();

    let mut app2 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    let reads2 = capture_docname_event(&mut app2, "source-read");
    app2.build().unwrap();
    assert_eq!(
        *reads2.borrow(),
        vec!["extra".to_string()],
        "only the newly-added document should be read"
    );
    assert!(app2.env.all_docs.contains_key("extra"));
}

#[test]
fn removed_file_is_purged_from_env_and_doctree_store() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    write_project(src.path());
    std::fs::write(src.path().join("extra.rst"), "Extra\n=====\n\nMore.\n").unwrap();

    let mut app1 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    app1.build().unwrap();
    assert!(app1.env.has_stored_doctree("extra"));

    std::fs::remove_file(src.path().join("extra.rst")).unwrap();

    let mut app2 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    let reads2 = capture_docname_event(&mut app2, "source-read");
    app2.build().unwrap();
    assert!(
        reads2.borrow().is_empty(),
        "removing a document shouldn't cause anything to be re-read"
    );
    assert!(
        !app2.env.all_docs.contains_key("extra"),
        "removed document must be purged from all_docs"
    );
    assert!(
        !app2.env.has_stored_doctree("extra"),
        "removed document's persisted doctree must be deleted"
    );
}

#[test]
fn freshenv_forces_full_reread_even_when_unchanged() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    write_project(src.path());

    let mut app1 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    app1.build().unwrap();

    let mut app2 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    app2.set_incremental_options(true, false);
    let reads2 = capture_docname_event(&mut app2, "source-read");
    app2.build().unwrap();
    assert_eq!(
        reads2.borrow().len(),
        2,
        "-E/--fresh-env should force re-reading every document"
    );
}

#[test]
fn config_change_forces_full_reread() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let dt = TempDir::new().unwrap();
    write_project(src.path());
    std::fs::write(src.path().join("conf.py"), "project = 'Original'\n").unwrap();

    let mut app1 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    app1.build().unwrap();

    std::fs::write(src.path().join("conf.py"), "project = 'Changed'\n").unwrap();

    let mut app2 =
        SphinxApp::new(src.path(), out.path(), dt.path(), "html", HashMap::new()).unwrap();
    let reads2 = capture_docname_event(&mut app2, "source-read");
    app2.build().unwrap();
    assert_eq!(
        reads2.borrow().len(),
        2,
        "a conf.py change should force re-reading every document"
    );
}
