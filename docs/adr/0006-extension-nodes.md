# ADR 0006 — Extension (`add_node`) doctree nodes

**Status**: proposed
**Date**: 2026-07-28

## Context
Real Sphinx extensions commonly define their own docutils node classes and
register them via `Sphinx.add_node(node_cls, override=False, **kwargs)`,
e.g.:

```python
class todo_node(nodes.Admonition, nodes.Element):
    pass

def html_visit_todo(self, node): ...
def html_depart_todo(self, node): ...

def setup(app):
    app.add_node(
        todo_node,
        html=(html_visit_todo, html_depart_todo),
        latex=(latex_visit_todo, latex_depart_todo),
    )
```

Each `kwargs` entry is keyed by a builder *format* name (`html`, `latex`,
`text`, `man`, ...) and maps to a `(visit_fn, depart_fn)` pair. Upstream's
`SphinxTranslator` dispatches to these dynamically (`visit_<node class
name>`/`depart_<node class name>`, falling back to `visit_/depart_
Admonition`-style base-class dispatch, then to `unknown_visit`/a
`SkipNode`-raising default). A document containing an instance of the
custom node class then renders correctly for every builder that
registered a pair, and degrades gracefully (renders children only, or is
skipped) for builders that didn't.

`docutilsrs::doctree::NodeKind` is a **closed Rust enum** (see [ADR
0004](0004-doctree-representation.md) for why: cache-friendly arena
traversal, enum-encoded invariants, no per-access PyO3 cost). Every
writer (`html5_writer.rs`, `latex_writer.rs`, `manpage_writer.rs`,
`odt_writer.rs`, `text_writer.rs`, `xml_writer.rs`, plus the shared
`writer.rs` dispatch and `python.rs`'s doctree introspection bridge) has
an exhaustive `match &node.kind { ... }`. There is currently no variant —
and no per-builder visitor-callable registry — a custom node's rendering
logic could hook into. `app.add_node` is presently a no-op stub for
exactly this reason.

This ADR is scoped to the *representation and dispatch* question only,
not to a full implementation.

## Decision drivers
- Must not force every existing writer to reason about arbitrary Python
  objects on the traversal hot path (ADR 0004's whole rationale).
- Must support upstream's actual per-*builder-format*, per-*node-class*
  `(visit, depart)` pair model reasonably faithfully — not just "run
  Python for every node," which would reopen the ADR 0004 tradeoff.
- Must degrade gracefully (skip / render children) for a builder format
  the extension never registered a pair for, matching upstream.
- Should not require inventing a full generic Python-node round-trip
  bridge (arbitrary attribute get/set, `findall`, `replace_self`, etc.) —
  that is a separate, much larger project (see "Related/out of scope"
  below) that this ADR does not attempt to solve.

## Options considered

### A. Closed catch-all `NodeKind::Extension` variant + per-format visitor registry
Add one new variant:

```rust
NodeKind::Extension {
    /// The registered node class's Python `__qualname__` (used both as
    /// the node's `dispatch_key` and as the docname-scoped identity a
    /// caller uses to look up its own per-instance data, since this
    /// variant carries no other Python-visible state).
    class_name: String,
},
```

A new registry (`docutilsrs::plugins`, alongside the existing
`register_directive`/transform registries) stores, per `(class_name,
builder_format)`, an optional `(visit, depart)` `Py<PyAny>` pair —
mirroring `SphinxComponentRegistry::add_node`'s upstream shape closely
enough for `sphinxdocrs::app_facade::PyAppFacade::add_node` to populate
it directly from `**kwargs`.

Each writer's exhaustive match gains exactly one new arm:
`NodeKind::Extension { class_name } => { look up (class_name,
<this writer's format>) in the registry; if found, call visit, render
children (recursively re-entering the same writer, exactly like every
other container node), call depart; if not found, render children only
(closest Rust analogue of upstream's base-class/`unknown_visit`
fallback chain) }`.

**Pros**
- Single, bounded enum change (one variant), matching the "adding a new
  node kind should be a single enum variant addition" guardrail ADR 0004
  already commits to.
- Reuses the existing "process-global `Mutex`-guarded registry consulted
  from the parser/writer, invoked via a short `Python::attach` call"
  pattern already established for directives/transforms
  (`docutilsrs::plugins`) — no new architectural idiom.
- Per-builder-format fallback-to-children-only behavior is cheap to
  implement once per writer and is a reasonable, honest approximation of
  upstream's dispatch chain.
- Node *identity* (which custom class an instance is) survives
  `Doctree`'s existing serialization/snapshot machinery
  (`doctree.rs`'s `NodeKindData`-based (de)serialization used for pseudo-
  XML snapshots and persisted-environment round-trips) as a plain string,
  unlike a `Py<PyAny>` handle would.

**Cons**
- The node carries **no other Python-visible attributes** beyond its
  class name — a real docutils `Element` subclass can carry arbitrary
  `node['foo'] = bar` attributes and arbitrary child content assembled by
  the extension's own directive `run()`. Representing arbitrary
  attributes would need a second field (e.g. `attrs:
  HashMap<String, String>`, mirroring the existing string-typed
  simplification already accepted for `domaindata`/`temp_data`/
  `ref_context` — see `app_facade.rs`'s `PyEnvFacade` doc comment) —
  proposed as part of this same variant, with the same "**accepted
  deviation**: string-valued attributes only" caveat.
- Still does not solve *constructing* such a node from within a custom
  directive's `run()` (that return value is a list of real
  `docutils.nodes.Node` objects upstream, which still has nowhere to go
  in this arena — see "Related/out of scope").
- The visit/depart callables still need *something* real to operate on.
  Upstream's `visit_todo(self, node)` reads/writes arbitrary `node[...]`
  attributes and calls `self.body.append(...)` directly against the
  translator's live output buffer — this needs its own small bridge (a
  `PyNodeHandle`-style wrapper exposing `attrs`, a `body.append(str)`-
  shaped write sink, and nothing else) rather than a generic `PyDoctree`
  round-trip. Scoped as an implementation detail of this ADR, not a
  separate ADR, since it doesn't touch `NodeKind`'s shape.

### B. Side-channel map (`NodeId -> Py<PyAny>`), no `NodeKind` change
Keep `NodeKind` untouched. Maintain a side table
`HashMap<NodeId, Py<PyAny>>` on `Doctree` for nodes that are "really" a
live Python object; writers check this table before falling into their
normal match.

**Pros**: zero changes to any existing writer's exhaustive match arms
(only an extra lookup at the top of each `match` block, or even outside
it).
**Cons**: every writer still needs that lookup-and-special-case
anyway (so the "single variant addition" saving is smaller than it
looks); `Py<PyAny>` in a table alongside an otherwise `Send`-agnostic
arena reopens exactly the per-access-PyO3-cost/GIL-coupling ADR 0004
rejected for the *whole tree*, just scoped to a subset of nodes instead —
same category of tradeoff, harder to reason about locally since two
representations of "a node" now coexist.

### C. Status quo — keep `add_node` a stub
No code changes. Simplest, but leaves a real and common extension
pattern (`sphinx.ext.todo`, `sphinx-design`, many others) permanently
unable to register at all without crashing `setup()` — worse than the
current bookkeeping-only treatment already given to `add_directive`/
`add_role`/`add_domain`/`add_builder`, all of which at least avoid
`AttributeError`.

## Decision
**(A) Closed catch-all `NodeKind::Extension { class_name, attrs }`
variant, plus a per-`(class_name, builder_format)` visitor registry in
`docutilsrs::plugins`.** Consistent with ADR 0004's arena-based design and
its explicit guidance that adding a node kind should be a single enum
variant, not a refactor.

This ADR does not by itself unblock a custom *directive* that
constructs such nodes (that still needs the directive to be able to push
an `Extension`-kind node into the tree it's currently building — see
"Related/out of scope"), only the **rendering** half:
`app.add_node(cls, **format_pairs)` recording real visit/depart callables
against `cls.__name__`, and every writer's fallback-to-children (or
skip) behavior for an `Extension` node with no matching pair.

## Consequences
- `app_facade.rs::PyAppFacade::add_node` becomes a real implementation:
  extract `cls.__name__`, then for each `(format, (visit_fn,
  depart_fn))` pair in `**kwargs` (plus the 2-tuple `override`-adjacent
  positional form upstream also accepts), register into the new
  `docutilsrs::plugins` visitor table.
- Every writer (`html5_writer.rs`, `latex_writer.rs`,
  `manpage_writer.rs`, `odt_writer.rs`, `text_writer.rs`,
  `xml_writer.rs`, `writer.rs`) gains one new match arm plus a small
  shared helper (in `docutilsrs::plugins`) for "look up + invoke +
  fallback-to-children."
- `doctree.rs`'s `NodeKindData`/pseudo-XML snapshot (de)serialization
  needs one new case (round-trips `class_name`/`attrs` as plain
  strings — no `Py<PyAny>` ever touches the arena or its snapshots).
- **Still out of scope even after this ADR's decision is implemented**:
  a custom *directive*'s `run()` producing an `Extension` node in the
  first place. That is squarely the same, larger "doctree/Node Python
  bridge" gap already identified as blocking `app.events` +
  `doctree-read` listeners (e.g. `sphinx.ext.viewcode`) during the
  `make otherdocs-sphinx-rs` real-world validation pass — tracked as a
  separate, follow-up body of work, not folded into `add_node` itself.
  Until that lands, `add_node`'s value is limited to "a registered
  extension no longer crashes `setup()`, and if some *other* mechanism
  ever manages to place an `Extension`-kind node into a doctree, it
  renders reasonably" — real end-to-end custom-node authoring (write a
  directive that emits your own node, see it rendered) needs that
  follow-up too.
- Mitigate the attribute-value simplification (`HashMap<String,
  String>`, matching `domaindata`/`temp_data`/`ref_context`'s already-
  accepted deviation) by documenting it directly on the new variant, the
  same way those existing fields are documented.

## Related/out of scope
- The full docutils doctree/Node Python bridge (real, mutable,
  `findall`/`replace_self`-capable node objects reachable from
  `doctree-read` listeners and from a custom directive's `run()`) is a
  separate, larger undertaking that this ADR deliberately does not
  attempt to solve. See `/memories/repo/build-environment.md`'s
  "Known next wall" entry (2026-07-28+) for where that was first
  identified, via `sphinx.ext.viewcode`'s `doctree_read(app, doctree)`
  listener during a real `make otherdocs-sphinx-rs` build.
- `add_autodocumenter` remains explicitly out of scope (per prior
  direction), independent of this ADR.
