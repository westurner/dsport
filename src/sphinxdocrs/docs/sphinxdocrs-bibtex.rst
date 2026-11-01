sphinxdocrs::extensions::bibtex BibTeX and CSL citations
==========================================================

``sphinxdocrs`` includes a native BibTeX extension for citation processing.
It is available in the Rust build path and is separate from the Python
``sphinxcontrib-bibtex`` package.

Enable the extension in ``conf.py`` and list one or more BibTeX files:

.. code-block:: python

   extensions = ["sphinxdocrs::extensions::bibtex"]
   bibtex_bibfiles = ["references.bib"]

Bibliography paths are resolved relative to the documentation source
directory; absolute paths are also accepted. Citation keys must match the IDs
in the configured BibTeX records.

The extension's citation formatting and bibliography exports require building
``sphinxdocrs`` with the ``csl-citeproc`` Cargo feature. For example:

.. code-block:: console

   sphinx-build-rs -b html docs _build/html

   # To (re)compile first:
   cargo run -p sphinxdocrs --features csl-citeproc --bin sphinx-build-rs -- -b html docs _build/html

Configuration
-------------

``bibtex_bibfiles``
   A list of BibTeX source paths. The default is an empty list.

``bibtex_csl_style``
   An optional path to a local CSL style XML file, relative to the source
   directory or absolute. When unset or empty, the built-in style is used.

``bibtex_include_uncited``
   If true, include records that are not cited anywhere in the project in
   generated bibliography lists. The default is false. The standalone exports
   described below include every record in the configured BibTeX files.

Citing records
--------------

Use the citation key as the role content. The ``:cite:`` and ``:cite:p:``
roles render parenthetical citations; ``:cite:t:`` renders a textual citation.
Separate multiple keys with commas:

.. code-block:: rst

   See :cite:t:`smith2024` for an overview.

   The result is consistent with earlier work :cite:p:`smith2024,jones2025`.

The ``:footcite:``, ``:footcite:p:``, and ``:footcite:t:`` roles provide the
corresponding footnote citations. Add a ``footbibliography`` directive to the
document to render their notes:

.. code-block:: rst

   See :footcite:t:`smith2024` for an overview.

   .. footbibliography::

In MyST Markdown, write roles with ``{role}`content`` syntax and directives
as fenced blocks:

.. code-block:: markdown

   See {cite:t}`smith2024` for an overview.

   The result is consistent with earlier work {cite:p}`smith2024,jones2025`.

   See {footcite:t}`smith2024` for an overview.

   ```{footbibliography}
   ```

Bibliography directives
-----------------------

Place ``bibliography`` where the formatted list of cited records should
appear:

.. code-block:: rst

   .. bibliography::

The equivalent MyST directive is:

.. code-block:: markdown

   ```{bibliography}
   ```

The native extension currently accepts no directive options. Bibliography
records are formatted with citeproc and the selected CSL style.

Generated exports
-----------------

When the configured bibliography contains records, a successful build writes
two additional files to the output directory:

``bibliography.tex``
   A standalone LaTeX document containing a ``thebibliography`` list formatted
   with citeproc.

``bibliography.jsonld``
   A JSON-LD document with a Schema.org ``@graph`` describing the configured
   BibTeX records. DOI values are emitted as DOI URLs; records without a DOI or
   URL receive a ``urn:bibtex:`` identifier.

The names and output formats are currently fixed. No export files are written
when the configured bibliography is empty.