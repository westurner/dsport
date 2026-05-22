# ADR 0004 — Doctree representation

**Status**: accepted (M1)
**Date**: 2026-05-22

## Context
The doctree (node tree) is the contract every subsystem touches: parsers
produce it, transforms mutate it, writers consume it, plugins read and
modify it. Its representation directly bounds the achievable performance and
shapes the FFI boundary.

Two candidates:

- **(a) Owned Rust tree**: a native Rust data structure (arena + `NodeId`
  indices, enum-dispatched node kinds). Converters translate to/from
  `docutils.nodes` at the FFI boundary.
- **(b) Wrapper over Python nodes**: Rust holds `Py<PyAny>` handles and
  every traversal/mutation goes through PyO3.

## Decision
**(a) Owned Rust tree with FFI converters.**

## Rationale
- Transforms and writers are traversal-heavy. (b) pays GIL acquisition +
  attribute lookup per child access; the cost is paid on every pass, of
  which there are many.
- An arena layout gives cache-friendly iteration and cheap parent/sibling
  navigation via integer indices.
- The Rust type system can encode node-kind invariants via enums; (b)
  forces runtime `isinstance`-equivalent checks.
- (a) decouples Rust code from `docutils.nodes` internals. Upstream layout
  changes ripple only through the converter, not the whole crate.
- Hybrid mode (ADR-pending, phase 3) is not blocked: converters run O(n) at
  each boundary crossing, versus (b)'s per-access cost.

## Mitigations
- Converter correctness is a parity risk surface. Guard with round-trip
  snapshot tests: Python → Rust → Python, asserted via pseudo-XML.
- Define the node enum carefully; adding a new node kind should be a single
  enum variant addition, not a refactor.

## Consequences
- M1 does not implement the doctree; phase 1 does.
- The converter is the first piece of FFI code to land in phase 1, before
  any parsing.
