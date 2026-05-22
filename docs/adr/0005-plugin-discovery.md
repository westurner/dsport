# ADR 0005 — Plugin discovery

**Status**: accepted (M1)
**Date**: 2026-05-22

## Context
The project must let a Python user install a third-party Sphinx extension
and have its Rust equivalent (when one exists) used transparently, with
fallback to the Python implementation when the Rust equivalent is missing
or incompatible.

A previously-considered approach added a freeform key to the plugin's
`pyproject.toml` naming the Rust crate. That requires every plugin author to
opt into our schema and gives us no way to discover already-installed
plugins.

## Decision
Use Python entry points (PEP 621 `[project.entry-points]`), group name
`docutilsrs.equivalents` (and `sphinxdocrs.equivalents`).

An entry point maps a Python dotted name (the plugin the user thinks they
are installing) to a Rust extension module + symbol that provides the
equivalent:

```toml
[project.entry-points."sphinxdocrs.equivalents"]
"sphinx.ext.autodoc" = "sphinx_autodoc_rs:register"
```

Resolver order at load time:

1. Look up the requested plugin name in the entry-point group.
2. If found, import the Rust module and call the registration symbol.
3. If the Rust module's declared compatible upstream version range does
   not include the active upstream pin, log a warning and fall back.
4. Otherwise, load the Python plugin normally.

## Consequences
- Works with already-installed packages: third parties ship the entry point
  in their own `pyproject.toml`; no central registry.
- Version skew is handled explicitly, not silently.
- Implementation lands in phase 5, not M1. ADR is recorded now so the
  resolver isn't designed twice.
