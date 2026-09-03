# rdfhdt

`rdfhdt` provides a small Rust bridge for converting streaming N-Triples to
HDT and HDT back to validated N-Triples. It also exposes an RDF-store-neutral
quad adapter with explicit named-graph handling.

The command-line interface accepts `-` for stdin or stdout:

```sh
printf '<https://example.test/s> <https://example.test/p> "ok" .\n' \
  | rdfhdt export - dataset.hdt
rdfhdt import dataset.hdt -
```

HDT construction uses the local `hdt-rs` fork's `nt`-feature
`Hdt::from_triples` API. HDT is triple-only, so named graphs are rejected by
default and can only be discarded through the explicit `flatten` policy.
