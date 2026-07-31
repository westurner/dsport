use docutilsrs::{html5, parse_rst};

fn render_html(src: &str) -> String {
    let tree = parse_rst(src);
    html5(
        &tree,
        &docutilsrs::cli::Html5Options::default(),
        &docutilsrs::cli::CommonOptions::default(),
    )
}

#[test]
fn test_core_directives_parsing() {
    let src = r#"
.. rubric:: Title of Rubric

.. contents:: Table of Contents

.. versionadded:: 1.2
   New feature added.

.. only:: html

   Content for HTML build.

.. todo::

   Fix this issue.

.. glossary::

   Term 1
      Definition 1.

.. list-table::
   * - Cell 1
     - Cell 2

.. csv-table::
   Header 1, Header 2
   Val 1, Val 2
"#;

    let html = render_html(src);

    assert!(html.contains("Title of Rubric"), "rubric check: {html}");
    assert!(html.contains("topic contents"), "contents check: {html}");
    assert!(
        html.contains("versionmodified added"),
        "versionadded check: {html}"
    );
    assert!(html.contains("only html"), "only check: {html}");
    assert!(html.contains("todo"), "todo check: {html}");
    assert!(html.contains("glossary"), "glossary check: {html}");
    assert!(html.contains("<table>"), "table check: {html}");
}

#[test]
fn test_domain_and_extension_directives() {
    let src = r#"
.. py:function:: foo(a, b=1)

   Python function docstring.

.. autoclass:: MyClass

.. graphviz::

   digraph { A -> B; }
"#;

    let html = render_html(src);

    assert!(
        html.contains("py:function foo(a, b=1)"),
        "HTML output: {html}"
    );
    assert!(html.contains("py:autoclass MyClass"), "HTML output: {html}");
    assert!(html.contains("graphviz"), "HTML output: {html}");
}

#[test]
fn test_formatting_and_domain_roles() {
    let src = r#"
Text with :abbr:`LIFO (Last In First Out)`, :sub:`2`, :sup:`10`, :kbd:`Ctrl+C`.
Security references: :cve:`2021-12345`, :cwe:`79`.
Cross references: :ref:`target-label`, :doc:`index`, :py:func:`my_func`.
Code and docutils links: :code-py:`x = 1`, :dudir:`include`.
"#;

    let html = render_html(src);

    assert!(
        html.contains("<abbr title=\"Last In First Out\">LIFO</abbr>"),
        "HTML output: {html}"
    );
    assert!(html.contains("<sub>2</sub>"), "HTML output: {html}");
    assert!(html.contains("<sup>10</sup>"), "HTML output: {html}");
    assert!(
        html.contains("<kbd class=\"kbd\">Ctrl+C</kbd>"),
        "HTML output: {html}"
    );
    assert!(html.contains("CVE-2021-12345"), "HTML output: {html}");
    assert!(html.contains("CWE-79"), "HTML output: {html}");
    assert!(html.contains("class=\"xref ref\""), "HTML output: {html}");
    assert!(html.contains("class=\"xref func\""), "HTML output: {html}");
    assert!(
        html.contains("class=\"code python\""),
        "HTML output: {html}"
    );
    assert!(
        html.contains("https://docutils.sourceforge.io/docs/ref/rst/directives.html#include"),
        "HTML output: {html}"
    );
}
