
# dsport

## Objectives
- port docutils (src/docutils) to rust as docutils.rs
- port sphinxdoc (src/sphinx) to rust as sphinxdoc.rs
- use test parametrization and mocks in rust to keep the tests fast
- full python plugin/extension compatibility
- support automatically using the rust equivalent
  of a python sphinxdoc plugin
  by adding a new metadata attribute to pyproject.toml/setup.py to indicate the name of the equivalent cargo project
- must be able to import docutils.rs and sphinxdoc.rs from Python
- must be able to import and use Python plugins from Rust


## Plan
- create a Cargo.toml
  - install and use `cargo insta`
- port docutils
  - review and port the tests from docutils to docutils.rs
  - parser first
- port sphinxdoc
  - review and port the tests from sphinxdoc to sphinxdoc.rs

