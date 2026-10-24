# Sphinx (`sphinxdoc`) vs. `sphinxdocrs` LSP API & Test Comparison Report

Generated via `sphinxdocrs` LSP `textDocument/documentSymbol` analysis (`sphinx-source-status --compare-tests`).

## 1. Aggregate Tree Summary

| Tree | Files | Total Symbols | API Callables (`fn`/`def`) | API Types (`class`/`struct`/`enum`/`trait`) | Test Functions |
| --- | ---: | ---: | ---: | ---: | ---: |
| `sphinx` API (`src/sphinx/sphinx`) | 245 | 6893 | 4890 | 768 | 3 |
| `sphinx` tests (`src/sphinx/tests`) | 143 | 2138 | 0 | 0 | 1562 |
| `sphinxdocrs` crate (`src/sphinxdocrs/src`, incl. inline `#[cfg(test)]`) | 104 | 3659 | 1554 | 526 | 1175 |
| `sphinxdocrs` Rust integration tests (`src/sphinxdocrs/tests`) | 48 | 923 | 0 | 0 | 744 |
| `sphinxdocrs` Python bridge tests (`src/tests/test_sphinx*`) | 9 | 91 | 0 | 0 | 83 |
| **`sphinxdocrs` combined** | **161** | **4673** | **1554** | **526** | **2002** |

## 2. Per-Subsystem API & Test Function Breakdown

| Subsystem | `sphinx` API (`fn`/`cls`) | `sphinxdocrs` API (`fn`/`type`) | `sphinx` Test Files | `sphinx` Test Fns | `sphinxdocrs` Test Sources | `sphinxdocrs` Test Fns |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `errors` | 7 / 13 | 40 / 22 | 1 | 2 | 4 | 14 |
| `events` | 37 / 2 | 19 / 11 | 1 | 3 | 4 | 26 |
| `project` | 5 / 1 | 10 / 4 | 1 | 7 | 3 | 14 |
| `addnodes` | 17 / 51 | 10 / 46 | 1 | 1 | 2 | 45 |
| `config` | 48 / 4 | 87 / 13 | 2 | 53 | 2 | 65 |
| `extension & registry` | 43 / 2 | 97 / 31 | 13 | 82 | 8 | 141 |
| `versioning` | 6 / 1 | 9 / 1 | 1 | 10 | 2 | 52 |
| `util` | 502 / 89 | 135 / 23 | 21 | 191 | 16 | 266 |
| `locale & intl` | 26 / 1 | 57 / 14 | 3 | 68 | 4 | 83 |
| `quickstart (C1)` | 20 / 2 | 34 / 14 | 1 | 10 | 6 | 52 |
| `build & cli (C2)` | 51 / 2 | 66 / 33 | 2 | 12 | 8 | 116 |
| `apidoc (C3)` | 24 / 2 | 42 / 14 | 1 | 28 | 3 | 37 |
| `autosummary / autogen (C4)` | 50 / 10 | 21 / 7 | 2 | 30 | 4 | 56 |
| `application` | 85 / 2 | 17 / 10 | 1 | 8 | 2 | 44 |
| `environment & toctree` | 132 / 12 | 113 / 11 | 4 | 28 | 7 | 193 |
| `builders & writers` | 1323 / 82 | 219 / 67 | 29 | 309 | 31 | 401 |
| `roles & directives` | 81 / 44 | 7 / 5 | 13 | 98 | 2 | 46 |
| `domains & source_docs (LSP/H14)` | 1431 / 255 | 331 / 116 | 10 | 209 | 19 | 183 |
| `theming` | 26 / 3 | 70 / 32 | 3 | 16 | 3 | 52 |
| `search & stemming` | 80 / 26 | 43 / 9 | 1 | 15 | 4 | 33 |
| `autodoc & pycode` | 420 / 59 | 99 / 37 | 19 | 262 | 4 | 47 |
| `intersphinx` | 44 / 6 | 18 / 6 | 3 | 34 | 2 | 35 |
| `highlighting & transforms & other ext` | 364 / 82 | 10 / 0 | 11 | 91 | 1 | 1 |
