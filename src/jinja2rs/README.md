# jinja2rs

Rust port of [Jinja2](https://jinja.palletsprojects.com/), powered by
[minijinja](https://github.com/mitsuhiko/minijinja) and its
[`minijinja-contrib` pycompat layer](https://github.com/mitsuhiko/minijinja/tree/main/minijinja-contrib),
with a Sphinx-compatible API and Django template compatibility mode.

## Quick start

```rust
use jinja2rs::Environment;
use serde_json::json;

let mut env = Environment::new();
env.add_template("hello.html", "Hello, {{ name }}!").unwrap();
let tmpl = env.get_template("hello.html").unwrap();
assert_eq!(tmpl.render(json!({"name": "Sphinx"})).unwrap(), "Hello, Sphinx!");
```

## Compatibility modes

| Mode | Description |
|------|-------------|
| `CompatMode::Jinja2` (default) | Drop-in Python Jinja2; enables `.items()`, `.upper()`, etc. |
| `CompatMode::Minijinja` | Strict minijinja; use filters instead of methods |
| `CompatMode::Ansible(…)` | Ansible playbook filters and inventory |
| `CompatMode::Kubernetes(…)` | Kubernetes manifest filters |
| `CompatMode::Django(…)` | Django template language (filters, app-directory loader, auto-escape) |


## Features

| Cargo feature | Default | Description |
|---------------|---------|-------------|
| `sphinx` | off | Sphinx-specific glue (`jinja2glue.py` equivalent) |
| `sandbox` | off | Path/attribute/method sandboxing |
| `seccomp` | off | Linux syscall filtering (requires `libseccomp`) |
| `resource-limits` | off | `ulimit`-based memory/CPU limits |
| `django` | off | Django template filters and app-directory loader |
| `i18n` | off | `gettext`/`ngettext` translation support |


## Django mode

```rust
use jinja2rs::{Environment, DjangoMode};
use serde_json::json;

let env = Environment::with_django_mode(
    DjangoMode::default().with_app_directory("/myproject/myapp"),
);
let html = env
    .render_str("{{ title|slugify }}", json!({"title": "Hello, World!"}))
    .unwrap();
assert_eq!(html, "hello-world");
```


## Licenses

This crate incorporates design patterns and API conventions from several
open-source projects. Their licenses are kept at the workspace root:

| Integration Type | License Source | Project | License |
|---|---|---------|---------|
| Wraps | External | minijinja | Apache-2.0 |
| Reimplements/Ports/Clones | [`LICENSE.jinja2`](../../LICENSE.jinja2) | Jinja2 | BSD-3-Clause |
| Reimplements/Ports/Clones | [`LICENSE.Django`](../../LICENSE.Django) | Django | BSD-3-Clause |
| Reimplements/Ports/Clones | [`LICENSE.sphinx`](../../LICENSE.sphinx) | Sphinx | BSD-2-Clause |
| Writes config files of | External | Ansible | GPL-3.0 |
| Reads/Writes config files of | External | Docker Compose | Apache-2.0 |
| Reads/Writes config files of | External | Kubernetes | Apache-2.0 |

The `jinja2rs` crate itself is licensed under **BSD-3-Clause AND Apache-2.0**
(see `Cargo.toml`).

## Citation

If you use jinja2rs in research or wish to cite the upstream projects, please
use the following entries.

For Django, Jinja2, minijinja:

```bibtex
@software{django,
  author       = {{Django Software Foundation}},
  title        = {Django: The Web Framework for Perfectionists with Deadlines},
  year         = {2005},
  howpublished = {\url{https://www.djangoproject.com/}},
  note         = {BSD-3-Clause license.
                  Source: \url{https://github.com/django/django}},
}

@software{jinja2,
  author       = {Armin Ronacher and contributors},
  title        = {Jinja2: A Full-Featured Template Engine for Python},
  year         = {2008},
  url          = {https://jinja.palletsprojects.com/},
  note         = {BSD-3-Clause license.
                  Source: \url{https://github.com/pallets/jinja}},
}

@software{markupsafe,
  author       = {Armin Ronacher and contributors},
  title        = {MarkupSafe: Safely Convert Markup to Bytes},
  year         = {2010},
  url          = {https://markupsafe.palletsprojects.com/},
  note         = {BSD-3-Clause license.
                  Source: \url{https://github.com/pallets/markupsafe}.
                  Provides safe HTML string handling and auto-escaping.
                  jinja2rs includes \texttt{markupsafers}, a Rust port of MarkupSafe.},
}

@software{minijinja,
  author       = {Armin Ronacher},
  title        = {MiniJinja: Jinja2 Template Engine for Rust},
  year         = {2021},
  url          = {https://github.com/mitsuhiko/minijinja},
  note         = {Apache-2.0 license.
                  jinja2rs is built on minijinja and minijinja python compat.},
}
```

For jinja2rs:

```
@software{jinja2rs,
  author       = {Westurner},
  title        = {jinja2rs: Rust port of Jinja2 with Django compatibility},
  year         = {2026},
  url          = {https://github.com/westurner/dsport/tree/main/src/jinja2rs},
  note         = {Powered by minijinja. BSD-3-Clause AND Apache-2.0.},
}
```

For Sphinxdoc, Docker Compose, Ansible, Kubernetes (template-engine consumers):

```bibtex
@software{sphinx,
  author       = {Georg Brandl and contributors},
  title        = {Sphinx: Python Documentation Generator},
  year         = {2007},
  url          = {https://www.sphinx-doc.org/},
  note         = {BSD-2-Clause license.
                  Source: \url{https://github.com/sphinx-doc/sphinx}.
                  Sphinx uses Jinja2 as its HTML theme template engine.},
}

@software{docker_compose,
  author       = {{Docker, Inc.}},
  title        = {Docker Compose: Defining and Running Multi-Container Applications},
  year         = {2020},
  url          = {https://docs.docker.com/compose/},
  note         = {Apache-2.0 license.
                  Source: \url{https://github.com/docker/compose}.
                  Docker Compose uses environment variable substitution and
                  template-like expressions in compose.yml files for service
                  configuration.
                  jinja2rs provides a \texttt{CompatMode::Docker} (via general
                  environment support) for pre-processing and rendering
                  compose.yml files with full Jinja2 template power, enabling
                  dynamic service definitions, conditional blocks, and
                  macro-based composition before passing to Docker.},
}

@software{ansible,
  author       = {Michael DeHaan and {Red Hat, Inc.} and contributors},
  title        = {Ansible: Radically Simple IT Automation},
  year         = {2012},
  url          = {https://www.ansible.com/},
  note         = {GPL-3.0 license.
                  Source: \url{https://github.com/ansible/ansible}.
                  Ansible uses Jinja2 as its template engine for playbooks,
                  roles, and inventory files.
                  jinja2rs provides \texttt{CompatMode::Ansible} for rendering
                  Ansible-compatible templates with standard filters and
                  inventory variable support.},
}

@software{kubernetes,
  author       = {{The Kubernetes Authors}},
  title        = {Kubernetes: Production-Grade Container Orchestration},
  year         = {2014},
  url          = {https://kubernetes.io/},
  note         = {Apache-2.0 license.
                  Source: \url{https://github.com/kubernetes/kubernetes}.
                  Helm charts, Kustomize, and many Kubernetes deployment tools
                  use Jinja2-style templating for manifest generation.
                  jinja2rs provides \texttt{CompatMode::Kubernetes} for rendering
                  Kubernetes manifests with resource filters and inventory support.},
}
```


## See also

- [porting-plan.md](porting-plan.md) — architecture and design decisions
- [markupsafers](../markupsafers/) — companion crate; Rust port of MarkupSafe
- [sphinxdocrs](../sphinxdocrs/) — Sphinx port that consumes this crate
