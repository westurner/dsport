# AGENTS — sphinxdocrs

Guidance for coding agents and contributors working inside
`src/sphinxdocrs/`.

---

## Quick orientation

```
src/sphinxdocrs/
├── Cargo.toml          # crate manifest; see [features] for "parity"
├── locale/             # symlink → ../../sphinx/sphinx/locale (70 .po files)
├── assets/             # vendored Jinja2 templates (quickstart, apidoc, autosummary)
├── src/
│   ├── lib.rs          # pub mod declarations + pymodule entry point
│   ├── locale.rs       # sphinx.locale port — PO parser, Translator, tr!
│   ├── intl.rs         # sphinx.util.i18n — CatalogInfo, CatalogRepository
│   ├── application.rs  # SphinxApp (minimal Sphinx.__init__ + build)
│   ├── builders/       # html.rs, latex.rs, manpage.rs, json.rs
│   ├── build/          # CLI arg parsing, make_mode, logging
│   ├── quickstart/     # sphinx-quickstart-rs native implementation
│   ├── apidoc/         # sphinx-apidoc-rs native implementation
│   ├── autogen/        # sphinx-autogen-rs native implementation
│   └── …               # config, environment, registry, roles, util_*, …
├── tests/
│   ├── parity.rs       # cross-language parity tests (feature = "parity")
│   ├── locale.rs       # sphinx.locale integration tests
│   ├── intl.rs         # sphinx.util.i18n integration tests
│   └── …               # per-module integration tests
└── benches/
    └── sphinxdocrs.rs  # criterion benchmarks
```

---

## Locale / translation support

### Overview

`locale.rs` is a pure-Rust port of `sphinx.locale`. It provides:

| item | Rust | Python equivalent |
|---|---|---|
| `.po` file parser | `PoCatalog::parse(content)` | `babel.messages.pofile.read_po` |
| Translator chain | `Translator` + `add_fallback` | `NullTranslations.add_fallback` |
| Global registry | `TRANSLATORS` (`OnceLock<Mutex<HashMap>>`) | `sphinx.locale.translators` |
| Load catalog | `init(locale_dirs, language, catalog, namespace)` | `sphinx.locale.init` |
| Console init | `init_console(locale_dir, catalog)` | `sphinx.locale.init_console` |
| Get translator | `get_translator(catalog, namespace)` | `sphinx.locale.get_translator` |
| Translation fn | `get_translation(catalog, namespace)` → `Fn` | `sphinx.locale.get_translation` |
| Shortcut `_` | `tr(msgid)` or `tr!("msgid")` macro | `_ = get_translation('sphinx')` |
| Shortcut `__` | `tr_console(msgid)` or `tr_c!("msg")` | `__ = get_translation('sphinx','console')` |
| Admonition labels | `admonition_labels()` | `sphinx.locale.admonitionlabels` |

### The `locale/` symlink

```
src/sphinxdocrs/locale  →  ../../sphinx/sphinx/locale
```

This relative symlink gives the crate access to all 70 upstream Sphinx
`.po` files (one per language) without copying them.  The file
`builtin_locale_dir()` resolves this path at runtime.

To load the built-in sphinx catalog for a given language:

```rust
use sphinxdocrs::locale::{init, tr};

// Load German translations from the bundled locale/ symlink
let locale_dir = sphinxdocrs::locale::builtin_locale_dir().unwrap();
init(&[locale_dir], Some("de"), "sphinx", "general");

// Translate a message
let msg = tr("Running Sphinx v%s");
// → "Sphinx v%s läuft" (if the German catalog has a translation)
```

### Adding translations from an extension

```rust
use sphinxdocrs::locale::{init, get_translation};
use std::path::PathBuf;

// Extension registers its own message catalog
let locale_dir = PathBuf::from("myext/locale");
init(&[locale_dir], Some("ja"), "myext", "myext_ns");

let translate = get_translation("myext", "myext_ns");
println!("{}", translate("Hello from myext"));
```

### Key design decisions

1. **Runtime (not build-time) loading** — `init()` reads `.po` files on
   demand.  This is intentional: Sphinx extensions register their catalogs
   at runtime, and tests inject temporary `TempDir`s.  If you want
   build-time embedding for the built-in strings, see the note in
   `docs/sphinx-port-inventory.md` about `include-po` + `tr`.

2. **Empty `msgstr` → falls back to `msgid`** — matches Python gettext
   semantics.  A `.po` entry with `msgstr ""` is treated as untranslated.

3. **Namespace isolation** — the registry key is `(namespace, catalog)`.
   The `"general"` namespace holds documentation-language translations;
   `"console"` holds output-language translations.

4. **`clear_translators()`** must be called between tests that use
   `init()` — the registry is process-global.  All `tests/locale.rs`
   fixtures call `clear_translators()` before registering.

### `intl.rs` — document-level catalog discovery

`CatalogRepository` maps a project's `locale_dirs` config setting to
individual `.po` files:

```rust
use sphinxdocrs::intl::{CatalogRepository, docname_to_domain};

let repo = CatalogRepository::new(
    srcdir,                       // project source root
    vec!["locale".to_owned()],    // conf.py locale_dirs
    "ja",                         // conf.py language
    "utf-8",                      // charset
);
for cat in repo.catalogs() {
    println!("{} (outdated={})", cat.domain, cat.is_outdated());
}
```

---

## Running tests

```bash
# All unit + integration tests
cargo test -p sphinxdocrs

# Locale / intl integration tests only
cargo test -p sphinxdocrs --test locale --test intl

# Cross-language parity tests (requires Python + sphinx in PATH)
# Safe: json_parity tests excluded; Python processes capped at 2 concurrent
cargo test -p sphinxdocrs --features parity --test parity

# Also run the JSON builder parity tests (memory-intensive; each spawns sphinx-build)
cargo test -p sphinxdocrs --features parity,parity-network --test parity

# Benchmarks
cargo bench -p sphinxdocrs
```

### Memory notes for parity tests

- Each `sphinx-build` (Python) subprocess uses ~150–300 MB.
- A global `Semaphore` in `parity.rs` caps concurrent Python processes at
  `MAX_PY_PROCS = 2`, bounding peak memory to ~600 MB during test execution.
- Cargo compilation on a 20-core machine uses ~4–8 GB (normal); the binary
  is cached after the first build, so subsequent runs are cheap.
- The `json_parity_*` tests are gated behind `--features parity-network`
  (which implies `parity`) to keep the default run lean.

---

## Parity discipline

Every module in `src/` that mirrors a Python subsystem must have a
corresponding row in `docs/sphinx-port-inventory.md` with one of:

- **mirrored** — Rust-side test exists, behavior checked against upstream
- **partial** — subset ported; remaining deferred items listed explicitly
- **deferred** — not yet ported; Python bridge is the only implementation
- **done** — complete parity including edge-cases

When you change the status of a row, update the inventory file in the
same commit.

---

## Parity gap: `make -C src/sphinx/doc SPHINXBUILD=sphinx-build-rs html`

Running the Sphinx documentation through `sphinx-build-rs` currently
produces HTML pages but lacks the following features present in the
Python output:

| missing | status | gate |
|---|---|---|
| Theme CSS / JS (`_static/`) | partial — minimal embedded CSS added | full theming deferred (Jinja2 / theming.py) |
| Document title from RST | **fixed** — extracted from `NodeKind::Document` | — |
| `objects.inv` (intersphinx inventory) | stub emitted | full serialization deferred |
| `genindex.html` | stub emitted | domains / index pipeline deferred |
| `search.html` / `searchindex.js` | deferred | search subsystem P3 |
| `_sources/` (source copies) | deferred | config `html_copy_source` P3 |
| `_images/` | deferred | image handling P3 |
| `py-modindex.html` | deferred | Python domain P3 |
| Cross-references resolved | deferred | environment resolve_references P3 |

The `tests/parity.rs` file contains skippable snapshot tests
(`cargo test --features parity`) that document these gaps as named
`insta` snapshots so regressions are caught automatically.

---

## Adding a new native builder

1. Create `src/builders/<name>.rs` implementing the `Builder` trait.
2. Add it to `NATIVE_BUILDERS` in `application.rs`.
3. Add it to the `match` in `SphinxApp::new`.
4. Add inline tests and an entry to `docs/sphinx-port-inventory.md`.
