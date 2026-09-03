# rdfhdt

`rdfhdt` provides a Rust bridge between HDT/HDTQ and the RDF formats supported
by Oxigraph: N-Triples, N-Quads, Turtle, TriG, N3, RDF/XML, and JSON-LD. The
library exposes parser/serializer-backed APIs as well as an RDF-store-neutral
quad adapter.

The command-line interface accepts `-` for stdin or stdout:

```sh
printf '<https://example.test/s> <https://example.test/p> "ok" .\n' \
  | rdfhdt export - dataset.hdt
rdfhdt import dataset.hdt -

rdfhdt export data.trig dataset.hdtq \
  --input-format trig --output-format hdtq --annotation-mode ag
rdfhdt import dataset.hdtq data.nq \
  --input-format hdtq --output-format nq
```

Standard HDT construction uses the local `hdt-rs` fork's `nt`-feature
`Hdt::from_triples` API. HDT is triple-only, so named graphs are rejected by
default and can only be discarded through the explicit `--graph-policy
flatten` policy.

HDTQ preserves named and default graphs using the HDTQ-java-compatible fifth
graph dictionary section and plain CRC-protected bitmaps. `ag` writes one
bitmap per graph; `at` writes one bitmap per SPO triple. Both modes are read by
the library. Roaring bitmap payloads are not supported.

The generic library entry points are `rdf_to_hdt`, `hdt_to_rdf`, `rdf_to_hdtq`,
and `hdtq_to_rdf`. Conversion materializes the input records while building
the compressed HDT dictionary; parsing and serialization themselves use the
Oxigraph streaming IO interfaces.
