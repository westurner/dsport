DSport Rust libraries
=====================

The following are the first-party packages declared in the root Cargo
workspace. Each package is cited here and listed on the project's
:doc:`bibliography` page.

Documentation and rendering
----------------------------

* ``docutilsrs`` provides the Rust docutils port. :cite:`docutilsrs, docutils`
* ``sphinxdocrs`` is the Rust Sphinx port and native documentation builder.
  :cite:`sphinxdocrs, sphinx`
* ``pygmentsrs`` provides Rust syntax highlighting. :cite:`pygmentsrs, pygments`
* ``myst-md-rs`` parses MyST Markdown. :cite:`myst-md-rs, myst_parser`
* ``jinja2rs`` provides the Sphinx-compatible Jinja API. :cite:`jinja2rs, jinja2`
* ``markupsafers`` provides safe HTML and XML markup strings.
  :cite:`markupsafers, markupsafe`
* ``mathrenderrs`` selects math rendering backends. :cite:`mathrenderrs`

DocIndex and data formats
-------------------------

* ``docindexrs-core`` contains the portable indexing and search primitives.
  :cite:`docindexrs-core`
* ``docindexrs-native`` provides filesystem and artifact adapters.
  :cite:`docindexrs-native`
* ``docindexrs-cli`` is the DocIndex command-line interface.
  :cite:`docindexrs-cli`
* ``docindexrs-wasm`` provides WebAssembly bindings for local DocIndex
  artifacts. :cite:`docindexrs-wasm`
* ``nbconvertrs`` provides optional Markdown and notebook transforms for
  DocIndex. :cite:`nbconvertrs`
* ``rdfhdt`` provides RDF and HDT artifact adapters. :cite:`rdfhdt`

Compatibility
-------------

* ``smartstring-compat`` re-exports the compatible SmartString API used by
  citeproc-rs. :cite:`smartstring-compat`