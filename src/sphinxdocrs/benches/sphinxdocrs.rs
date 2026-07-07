//! Criterion benchmarks for `sphinxdocrs`.
//!
//! Run with:
//!   cargo bench -p sphinxdocrs
//!   cargo bench -p sphinxdocrs -- locale   # locale group only
//!
//! HTML reports land in `target/criterion/`.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::io::Write;
use tempfile::TempDir;

// ── fixtures ──────────────────────────────────────────────────────────────────

/// 100-entry synthetic `.po` file (~4 KB).
fn synthetic_po(n: usize) -> String {
    let mut out = String::new();
    for i in 0..n {
        out.push_str(&format!(
            "\nmsgid \"Message number {i} in the test catalog\"\nmsgstr \"Translated message {i}\"\n"
        ));
    }
    out
}

/// Realistic sphinx `.po` file content (the German catalog header + a few entries).
fn de_po_excerpt() -> &'static str {
    r#"
msgid ""
msgstr ""
"Content-Type: text/plain; charset=UTF-8\n"
"Language: de\n"

msgid "Running Sphinx v%s"
msgstr "Sphinx v%s läuft"

msgid "loading translations [%s]... "
msgstr "Lade Übersetzungen [%s]... "

msgid "Build succeeded."
msgstr "Erstellung erfolgreich."

msgid "build finished with problems."
msgstr "Erstellung mit Fehlern abgeschlossen."

msgid "making output directory"
msgstr "Ausgabeverzeichnis wird erstellt"
"#
}

fn simple_rst() -> &'static str {
    r#"
Test Document
=============

A short paragraph with **bold** and *italic* text.

.. note::

   This is a note admonition.

Section Two
-----------

* Bullet one
* Bullet two
* Bullet three

.. code-block:: python

   def hello():
       print("Hello, world!")
"#
}

fn large_rst() -> String {
    let mut out = String::new();
    out.push_str("Large Document\n==============\n\n");
    for i in 0..200 {
        out.push_str(&format!(
            "Section {i}\n{}\n\nParagraph {i}: Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nSed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\n",
            "-".repeat(12 + i.to_string().len()),
        ));
    }
    out
}

// ── locale benchmarks ─────────────────────────────────────────────────────────

fn bench_po_parse(c: &mut Criterion) {
    let po_100 = synthetic_po(100);
    let po_500 = synthetic_po(500);
    let de_po = de_po_excerpt();

    let mut g = c.benchmark_group("locale/po_parse");
    g.bench_function("100_entries", |b| {
        b.iter(|| sphinxdocrs::locale::PoCatalog::parse(black_box(&po_100)))
    });
    g.bench_function("500_entries", |b| {
        b.iter(|| sphinxdocrs::locale::PoCatalog::parse(black_box(&po_500)))
    });
    g.bench_function("de_excerpt", |b| {
        b.iter(|| sphinxdocrs::locale::PoCatalog::parse(black_box(de_po)))
    });
    g.finish();
}

fn bench_tr_lookup(c: &mut Criterion) {
    // Load the synthetic catalog once, then benchmark individual lookups.
    let po = synthetic_po(500);
    let catalog = sphinxdocrs::locale::PoCatalog::parse(&po);

    // Set up translator registry with a unique namespace per bench run.
    let tmp = TempDir::new().unwrap();
    let lc = tmp.path().join("en").join("LC_MESSAGES");
    std::fs::create_dir_all(&lc).unwrap();
    let mut f = std::fs::File::create(lc.join("bench.po")).unwrap();
    f.write_all(po.as_bytes()).unwrap();
    sphinxdocrs::locale::clear_translators();
    sphinxdocrs::locale::init(&[tmp.path()], Some("en"), "bench", "bench_ns");

    let translate = sphinxdocrs::locale::get_translation("bench", "bench_ns");

    let mut g = c.benchmark_group("locale/tr_lookup");
    g.bench_function("hit", |b| {
        b.iter(|| translate(black_box("Message number 42 in the test catalog")))
    });
    g.bench_function("miss", |b| {
        b.iter(|| translate(black_box("Unknown message not in catalog")))
    });
    // Direct PoCatalog::gettext (no registry overhead)
    g.bench_function("direct_gettext_hit", |b| {
        b.iter(|| catalog.gettext(black_box("Message number 42 in the test catalog")))
    });
    g.finish();
}

fn bench_locale_init(c: &mut Criterion) {
    let po = synthetic_po(200);

    let mut g = c.benchmark_group("locale/init");
    for n in [1usize, 3, 10] {
        let tmps: Vec<TempDir> = (0..n).map(|_| TempDir::new().unwrap()).collect();
        for (i, tmp) in tmps.iter().enumerate() {
            let lc = tmp.path().join("en").join("LC_MESSAGES");
            std::fs::create_dir_all(&lc).unwrap();
            let mut f = std::fs::File::create(lc.join("bench.po")).unwrap();
            f.write_all(po.as_bytes()).unwrap();
            drop(i);
        }
        let dirs: Vec<&std::path::Path> = tmps.iter().map(|t| t.path()).collect();
        g.bench_with_input(BenchmarkId::new("dirs", n), &dirs, |b, dirs| {
            b.iter(|| {
                sphinxdocrs::locale::clear_translators();
                sphinxdocrs::locale::init(black_box(dirs), Some("en"), "bench", "bench_init_ns")
            })
        });
    }
    g.finish();
}

// ── HTML builder benchmarks ───────────────────────────────────────────────────

fn bench_html_render(c: &mut Criterion) {
    use sphinxdocrs::builders::Builder;
    use sphinxdocrs::builders::html::HtmlBuilder;
    use tempfile::TempDir;

    let builder = HtmlBuilder::new();
    let tmp = TempDir::new().unwrap();

    let small = simple_rst();
    let large = large_rst();

    let mut g = c.benchmark_group("builders/html");
    g.bench_function("build_doc_small", |b| {
        b.iter(|| {
            builder
                .build_doc(black_box("index"), black_box(small), tmp.path())
                .unwrap()
        })
    });
    g.bench_function("build_doc_large_200_sections", |b| {
        b.iter(|| {
            builder
                .build_doc(black_box("index"), black_box(large.as_str()), tmp.path())
                .unwrap()
        })
    });
    g.finish();
}

fn bench_html_build_all(c: &mut Criterion) {
    use sphinxdocrs::builders::Builder;
    use sphinxdocrs::builders::html::HtmlBuilder;
    use sphinxdocrs::environment::{BuildEnvironment, EnvProject};
    use tempfile::TempDir;

    let builder = HtmlBuilder::new();

    // Synthetic project: 20 RST files.
    let src = TempDir::new().unwrap();
    for i in 0..20 {
        let rst = format!("Page {i}\n{}\n\nContent of page {i}.\n", "=".repeat(10));
        std::fs::write(src.path().join(format!("page{i}.rst")), rst.as_bytes()).unwrap();
    }
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(
        sphinxdocrs::config::SphinxConfig::new_defaults(),
        project,
        src.path(),
        src.path().join(".doctrees"),
    );

    let mut g = c.benchmark_group("builders/html_build_all");
    g.bench_function("20_docs", |b| {
        let out = TempDir::new().unwrap();
        b.iter(|| {
            builder
                .build_all(
                    black_box(src.path()),
                    black_box(out.path()),
                    black_box(&env),
                )
                .unwrap()
        })
    });
    g.finish();
}

// ── RST util benchmarks ───────────────────────────────────────────────────────

fn bench_util_rst(c: &mut Criterion) {
    use sphinxdocrs::util_rst::{escape, heading, textwidth};

    let long_text = "Hello *world* &amp; <friends> — testing «escape»!".repeat(50);
    let heading_text = "My Section Title";

    let mut g = c.benchmark_group("util_rst");
    g.bench_function("escape_long", |b| b.iter(|| escape(black_box(&long_text))));
    g.bench_function("heading", |b| {
        b.iter(|| heading(black_box(heading_text), black_box(1usize), None))
    });
    g.bench_function("textwidth_ascii", |b| {
        b.iter(|| {
            textwidth(
                black_box(heading_text),
                sphinxdocrs::util_rst::WIDECHARS_DEFAULT,
            )
        })
    });
    g.bench_function("textwidth_unicode", |b| {
        b.iter(|| {
            textwidth(
                black_box("日本語テキストの幅測定"),
                sphinxdocrs::util_rst::WIDECHARS_DEFAULT,
            )
        })
    });
    g.finish();
}

// ── intl benchmarks ───────────────────────────────────────────────────────────

fn bench_intl(c: &mut Criterion) {
    use sphinxdocrs::intl::{CatalogRepository, docname_to_domain, ustrftime_to_babel};

    // Build a small project locale tree.
    let tmp = TempDir::new().unwrap();
    let locale = tmp.path().join("locale");
    let lc = locale.join("de").join("LC_MESSAGES");
    std::fs::create_dir_all(&lc).unwrap();
    let po = synthetic_po(50);
    for i in 0..5 {
        let mut f = std::fs::File::create(lc.join(format!("doc{i}.po"))).unwrap();
        f.write_all(po.as_bytes()).unwrap();
    }

    let repo = CatalogRepository::new(tmp.path(), vec!["locale".to_owned()], "de", "utf-8");

    let mut g = c.benchmark_group("intl");
    g.bench_function("catalog_repository_catalogs", |b| {
        b.iter(|| repo.catalogs())
    });
    g.bench_function("docname_to_domain_compact", |b| {
        b.iter(|| docname_to_domain(black_box("guide/intro/deep"), black_box(Some(""))))
    });
    g.bench_function("ustrftime_to_babel", |b| {
        b.iter(|| ustrftime_to_babel(black_box("%Y-%m-%d %H:%M:%S")))
    });
    g.finish();
}

// ── benchmark groups ─────────────────────────────────────────────────────────

criterion_group!(
    locale_benches,
    bench_po_parse,
    bench_tr_lookup,
    bench_locale_init,
);

criterion_group!(builder_benches, bench_html_render, bench_html_build_all,);

criterion_group!(util_benches, bench_util_rst, bench_intl,);

criterion_main!(locale_benches, builder_benches, util_benches);
