//! Streaming adapters between N-Triples, HDT, and an Oxigraph store.
//!
//! HDT is a triple format. Store export therefore makes the named-graph policy
//! explicit instead of silently discarding graph identity.

use std::fmt::Display;
use std::io::{self, BufRead, Read, Write};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use hdt::Hdt;
use oxrdfio::{RdfParser, RdfSerializer};
use oxttl::NTriplesParser;

pub use oxrdfio::RdfFormat;

mod hdtq;
pub use hdtq::{AnnotationMode, hdtq_to_quads, hdtq_to_rdf, quads_to_hdtq, rdf_to_hdtq};

pub type Result<T> = std::result::Result<T, Error>;

/// Policy used when exporting quads to the triple-only HDT format.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NamedGraphPolicy {
    /// Refuse to export a named graph because HDT cannot retain its identity.
    #[default]
    Reject,
    /// Export the triple and intentionally discard its graph name.
    Flatten,
}

impl NamedGraphPolicy {
    /// Parses the value used by `OXIRS_HDT_NAMED_GRAPH_POLICY`.
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "reject" => Ok(Self::Reject),
            "flatten" => Ok(Self::Flatten),
            other => Err(Error::InvalidPolicy(other.to_owned())),
        }
    }
}

/// Statistics collected during an HDT operation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Stats {
    pub triple_count: u64,
    pub input_bytes: u64,
    pub output_bytes: u64,
}

/// Parses an Oxigraph RDF format from a file extension, media type, or name.
pub fn parse_rdf_format(value: &str) -> Result<RdfFormat> {
    let value = value.trim();
    let extension = value.strip_prefix('.').unwrap_or(value);
    let lowered = value.to_ascii_lowercase();
    let format = match lowered.as_str() {
        "jsonld" | "json-ld" => RdfFormat::from_extension("jsonld"),
        "n3" => RdfFormat::from_extension("n3"),
        "nq" | "n-quads" | "nquads" => RdfFormat::from_extension("nq"),
        "nt" | "n-triples" | "ntriples" => RdfFormat::from_extension("nt"),
        "rdf" | "rdfxml" | "rdf/xml" => RdfFormat::from_extension("rdf"),
        "trig" => RdfFormat::from_extension("trig"),
        "ttl" | "turtle" => RdfFormat::from_extension("ttl"),
        _ => RdfFormat::from_extension(extension)
            .or_else(|| RdfFormat::from_media_type(value))
            .or_else(|| {
                [
                    RdfFormat::N3,
                    RdfFormat::NQuads,
                    RdfFormat::NTriples,
                    RdfFormat::RdfXml,
                    RdfFormat::TriG,
                    RdfFormat::Turtle,
                ]
                .into_iter()
                .find(|format| format.name().eq_ignore_ascii_case(value))
            }),
    };
    format.ok_or_else(|| Error::UnsupportedFormat(value.to_owned()))
}

/// Errors returned by the bridge.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("HDT error: {0}")]
    Hdt(String),
    #[error("N-Triples parse error: {0}")]
    Parse(String),
    #[error("invalid RDF term {term:?} in {position} position")]
    InvalidTerm {
        term: String,
        position: &'static str,
    },
    #[error("named graph {graph:?} cannot be represented in HDT")]
    NamedGraph { graph: String },
    #[error("invalid named-graph policy {0:?}; expected reject or flatten")]
    InvalidPolicy(String),
    #[error("unsupported RDF format {0:?}")]
    UnsupportedFormat(String),
}

#[derive(Clone, Copy)]
enum Position {
    Subject,
    Predicate,
    Object,
    Graph,
}

impl Position {
    const fn name(self) -> &'static str {
        match self {
            Self::Subject => "subject",
            Self::Predicate => "predicate",
            Self::Object => "object",
            Self::Graph => "graph",
        }
    }
}

/// Builds and writes an HDT document from an iterator of HDT lexical triples.
///
/// The iterator is consumed without sorting or canonicalization. Dictionary
/// construction and SPO ordering remain the responsibility of `hdt-rs`.
pub fn write_hdt<I, W>(triples: I, base_iri: &str, writer: W) -> Result<Stats>
where
    I: IntoIterator<Item = Result<[String; 3]>>,
    W: Write,
{
    let triples: Vec<[String; 3]> = triples.into_iter().collect::<Result<_>>()?;
    let hdt =
        Hdt::from_triples(triples, base_iri).map_err(|error| Error::Hdt(error.to_string()))?;
    let triple_count =
        u64::try_from(hdt.triples.len()).map_err(|_| io::Error::other("triple count overflow"))?;
    let mut writer = CountingWriter::new(writer);
    hdt.write(&mut writer)
        .map_err(|error| Error::Hdt(error.to_string()))?;
    Ok(Stats {
        triple_count,
        output_bytes: writer.count(),
        ..Stats::default()
    })
}

/// A triple plus an optional graph name in HDT lexical form.
///
/// This small record lets Oxigraph, OxiRS, or another RDF store provide its
/// own iterator without making this bridge depend on that store's workspace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuadRecord {
    pub triple: [String; 3],
    pub graph_name: Option<String>,
}

impl QuadRecord {
    pub fn default_graph(triple: [String; 3]) -> Self {
        Self {
            triple,
            graph_name: None,
        }
    }

    pub fn named_graph(triple: [String; 3], graph_name: impl Into<String>) -> Self {
        Self {
            triple,
            graph_name: Some(graph_name.into()),
        }
    }
}

/// Exports a quad iterator while applying the explicit HDT named-graph policy.
pub fn quads_to_hdt<I, W>(
    quads: I,
    base_iri: &str,
    policy: NamedGraphPolicy,
    writer: W,
) -> Result<Stats>
where
    I: IntoIterator<Item = Result<QuadRecord>>,
    W: Write,
{
    let triples = quads.into_iter().map(|quad| {
        let quad = quad?;
        if let Some(graph) = quad.graph_name {
            if policy == NamedGraphPolicy::Reject {
                return Err(Error::NamedGraph { graph });
            }
        }
        Ok(quad.triple)
    });
    write_hdt(triples, base_iri, writer)
}

/// Parses any RDF format supported by Oxigraph and writes a triple-only HDT.
/// Named graphs are rejected or flattened according to `policy`.
pub fn rdf_to_hdt<R, W>(
    reader: R,
    input_format: RdfFormat,
    base_iri: &str,
    policy: NamedGraphPolicy,
    writer: W,
) -> Result<Stats>
where
    R: Read,
    W: Write,
{
    let input_bytes = Arc::new(AtomicU64::new(0));
    let reader = CountingReader::new(reader, Arc::clone(&input_bytes));
    let parser = RdfParser::from_format(input_format)
        .with_base_iri(base_iri)
        .map_err(|error| Error::Parse(error.to_string()))?;
    let quads = parser.for_reader(reader).map(|item| {
        let quad = item.map_err(|error| Error::Parse(error.to_string()))?;
        Ok(QuadRecord {
            triple: [
                term_to_hdt(&quad.subject, Position::Subject)?,
                term_to_hdt(&quad.predicate, Position::Predicate)?,
                term_to_hdt(&quad.object, Position::Object)?,
            ],
            graph_name: if quad.graph_name.is_default_graph() {
                None
            } else {
                Some(term_to_hdt(&quad.graph_name, Position::Graph)?)
            },
        })
    });
    let mut stats = quads_to_hdt(quads, base_iri, policy, writer)?;
    stats.input_bytes = input_bytes.load(Ordering::Relaxed);
    Ok(stats)
}

/// Reads a triple-only HDT and serializes it to any Oxigraph RDF format.
/// The output is always in the default graph because standard HDT has no graph
/// identity to restore.
pub fn hdt_to_rdf<R, W>(reader: R, output_format: RdfFormat, writer: W) -> Result<Stats>
where
    R: BufRead,
    W: Write,
{
    let hdt = Hdt::read(reader).map_err(|error| Error::Hdt(error.to_string()))?;
    let mut writer = CountingWriter::new(writer);
    let mut serializer = RdfSerializer::from_format(output_format).for_writer(writer);
    let mut triple_count = 0_u64;
    for triple in hdt.triples_all() {
        let line = format!(
            "{} {} {} .\n",
            term_to_ntriples(&triple[0], Position::Subject)?,
            term_to_ntriples(&triple[1], Position::Predicate)?,
            term_to_ntriples(&triple[2], Position::Object)?,
        );
        let quad = parse_single_ntriple(&line)?;
        serializer.serialize_quad(&quad).map_err(Error::Io)?;
        triple_count += 1;
    }
    writer = serializer.finish().map_err(Error::Io)?;
    writer.flush()?;
    Ok(Stats {
        triple_count,
        output_bytes: writer.count(),
        ..Stats::default()
    })
}

/// Parses a streaming N-Triples input and writes the resulting HDT document.
pub fn ntriples_to_hdt<R, W>(reader: R, base_iri: &str, writer: W) -> Result<Stats>
where
    R: Read,
    W: Write,
{
    let input_bytes = Arc::new(AtomicU64::new(0));
    let reader = CountingReader::new(reader, Arc::clone(&input_bytes));
    let triples = NTriplesParser::new().for_reader(reader).map(|item| {
        let triple = item.map_err(|error| Error::Parse(error.to_string()))?;
        Ok([
            term_to_hdt(&triple.subject, Position::Subject)?,
            term_to_hdt(&triple.predicate, Position::Predicate)?,
            term_to_hdt(&triple.object, Position::Object)?,
        ])
    });
    let mut stats = write_hdt(triples, base_iri, writer)?;
    stats.input_bytes = input_bytes.load(Ordering::Relaxed);
    Ok(stats)
}

/// Reads an HDT document and writes validated N-Triples.
pub fn hdt_to_ntriples<R, W>(reader: R, writer: W) -> Result<Stats>
where
    R: BufRead,
    W: Write,
{
    let hdt = Hdt::read(reader).map_err(|error| Error::Hdt(error.to_string()))?;
    let mut writer = CountingWriter::new(writer);
    let mut triple_count = 0_u64;
    for triple in hdt.triples_all() {
        let line = format!(
            "{} {} {} .\n",
            term_to_ntriples(&triple[0], Position::Subject)?,
            term_to_ntriples(&triple[1], Position::Predicate)?,
            term_to_ntriples(&triple[2], Position::Object)?,
        );
        validate_ntriples(&line)?;
        writer.write_all(line.as_bytes())?;
        triple_count += 1;
    }
    writer.flush()?;
    Ok(Stats {
        triple_count,
        output_bytes: writer.count(),
        ..Stats::default()
    })
}

fn term_to_hdt(term: &impl Display, position: Position) -> Result<String> {
    let term = term.to_string();
    if let Some(iri) = term
        .strip_prefix('<')
        .and_then(|value| value.strip_suffix('>'))
    {
        if iri.is_empty() {
            return Err(Error::InvalidTerm {
                term,
                position: position.name(),
            });
        }
        return Ok(iri.to_owned());
    }
    if term.starts_with("_:")
        && matches!(
            position,
            Position::Subject | Position::Object | Position::Graph
        )
    {
        return Ok(term);
    }
    if term.starts_with('"') && matches!(position, Position::Object) {
        return Ok(term);
    }
    Err(Error::InvalidTerm {
        term,
        position: position.name(),
    })
}

fn term_to_ntriples(term: &str, position: Position) -> Result<String> {
    if term.starts_with("_:")
        && matches!(
            position,
            Position::Subject | Position::Object | Position::Graph
        )
    {
        return Ok(term.to_owned());
    }
    if term.starts_with('"') && matches!(position, Position::Object) {
        return Ok(term.to_owned());
    }
    if term.starts_with('<') && term.ends_with('>') && term.len() > 2 {
        return Ok(term.to_owned());
    }
    if !term.is_empty() && !term.contains(['<', '>', '\n', '\r']) {
        return Ok(format!("<{term}>"));
    }
    Err(Error::InvalidTerm {
        term: term.to_owned(),
        position: position.name(),
    })
}

fn parse_single_ntriple(line: &str) -> Result<oxrdf::Quad> {
    let mut parser = RdfParser::from_format(RdfFormat::NTriples).for_reader(line.as_bytes());
    let quad = parser
        .next()
        .ok_or_else(|| Error::Parse("generated N-Triples line was empty".to_owned()))?
        .map_err(|error| Error::Parse(error.to_string()))?;
    if parser.next().is_some() {
        return Err(Error::Parse(
            "generated N-Triples line contained multiple triples".to_owned(),
        ));
    }
    Ok(quad)
}

fn validate_ntriples(line: &str) -> Result<()> {
    let mut parser = NTriplesParser::new().for_slice(line.as_bytes());
    parser
        .next()
        .ok_or_else(|| Error::Parse("generated N-Triples line was empty".to_owned()))?
        .map_err(|error| Error::Parse(error.to_string()))?;
    if parser.next().is_some() {
        return Err(Error::Parse(
            "generated N-Triples line contained multiple triples".to_owned(),
        ));
    }
    Ok(())
}

struct CountingReader<R> {
    inner: R,
    count: Arc<AtomicU64>,
}

impl<R> CountingReader<R> {
    fn new(inner: R, count: Arc<AtomicU64>) -> Self {
        Self { inner, count }
    }
}

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let read = self.inner.read(buffer)?;
        self.count.fetch_add(read as u64, Ordering::Relaxed);
        Ok(read)
    }
}

struct CountingWriter<W> {
    inner: W,
    count: u64,
}

impl<W> CountingWriter<W> {
    fn new(inner: W) -> Self {
        Self { inner, count: 0 }
    }

    fn count(&self) -> u64 {
        self.count
    }
}

impl<W: Write> Write for CountingWriter<W> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let written = self.inner.write(buffer)?;
        self.count += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const INPUT: &str = concat!(
        "<https://example.test/s> <https://example.test/p> \"plain\" .\n",
        "<https://example.test/s> <https://example.test/lang> \"bonjour\"@fr .\n",
        "_:b <https://example.test/type> \"7\"^^<http://www.w3.org/2001/XMLSchema#integer> .\n",
    );

    #[test]
    fn ntriples_round_trip_preserves_terms() {
        let mut hdt = Vec::new();
        let exported =
            ntriples_to_hdt(INPUT.as_bytes(), "https://example.test/dataset", &mut hdt).unwrap();
        assert_eq!(exported.triple_count, 3);
        assert!(exported.input_bytes > 0);
        assert!(exported.output_bytes > 0);

        let mut output = Vec::new();
        let imported = hdt_to_ntriples(Cursor::new(hdt), &mut output).unwrap();
        assert_eq!(imported.triple_count, 3);
        let output = String::from_utf8(output).unwrap();
        for expected in INPUT.lines() {
            assert!(
                output.lines().any(|line| line == expected),
                "missing {expected:?} in {output:?}"
            );
        }
    }

    #[test]
    fn empty_dataset_round_trips() {
        let mut hdt = Vec::new();
        let stats = ntriples_to_hdt(
            Cursor::new(Vec::<u8>::new()),
            "https://example.test/empty",
            &mut hdt,
        )
        .unwrap();
        assert_eq!(stats.triple_count, 0);
        let mut output = Vec::new();
        let stats = hdt_to_ntriples(Cursor::new(hdt), &mut output).unwrap();
        assert_eq!(stats.triple_count, 0);
        assert!(output.is_empty());
    }

    #[test]
    fn malformed_ntriples_is_rejected() {
        let error = ntriples_to_hdt(
            "not RDF".as_bytes(),
            "https://example.test/dataset",
            Vec::new(),
        )
        .unwrap_err();
        assert!(matches!(error, Error::Parse(_)));
    }

    #[test]
    fn malformed_hdt_term_is_rejected_on_import() {
        let malformed = [[
            "https://example.test/s".to_owned(),
            "https://example.test/p".to_owned(),
            "<not-a-valid-hdt-term>".to_owned(),
        ]];
        let mut hdt = Vec::new();
        write_hdt(
            malformed.into_iter().map(Ok),
            "https://example.test/dataset",
            &mut hdt,
        )
        .unwrap();
        let error = hdt_to_ntriples(std::io::Cursor::new(hdt), Vec::new()).unwrap_err();
        assert!(matches!(error, Error::Parse(_) | Error::InvalidTerm { .. }));
    }

    #[test]
    fn malformed_hdt_container_is_rejected() {
        let error = hdt_to_ntriples(std::io::Cursor::new(b"not an HDT"), Vec::new()).unwrap_err();
        assert!(matches!(error, Error::Hdt(_)));
    }

    #[test]
    fn export_count_matches_hdt_deduplication() {
        let triple = [
            "https://example.test/s".to_owned(),
            "https://example.test/p".to_owned(),
            "\"o\"".to_owned(),
        ];
        let mut hdt = Vec::new();
        let stats = write_hdt(
            [Ok(triple.clone()), Ok(triple)],
            "https://example.test/dataset",
            &mut hdt,
        )
        .unwrap();
        assert_eq!(stats.triple_count, 1);
    }

    #[test]
    fn policies_are_explicit() {
        assert_eq!(
            NamedGraphPolicy::parse("reject").unwrap(),
            NamedGraphPolicy::Reject
        );
        assert_eq!(
            NamedGraphPolicy::parse("flatten").unwrap(),
            NamedGraphPolicy::Flatten
        );
        assert!(matches!(
            NamedGraphPolicy::parse("keep"),
            Err(Error::InvalidPolicy(_))
        ));
    }

    #[test]
    fn named_graph_policy_rejects_or_flattens() {
        let quad = QuadRecord::named_graph(
            [
                "https://example.test/s".to_owned(),
                "https://example.test/p".to_owned(),
                "\"o\"".to_owned(),
            ],
            "https://example.test/graph",
        );
        let error = quads_to_hdt(
            std::iter::once(Ok(quad.clone())),
            "https://example.test/dataset",
            NamedGraphPolicy::Reject,
            Vec::new(),
        )
        .unwrap_err();
        assert!(matches!(error, Error::NamedGraph { .. }));

        let mut hdt = Vec::new();
        let stats = quads_to_hdt(
            std::iter::once(Ok(quad)),
            "https://example.test/dataset",
            NamedGraphPolicy::Flatten,
            &mut hdt,
        )
        .unwrap();
        assert_eq!(stats.triple_count, 1);
    }

    fn parse_count(data: &[u8], format: RdfFormat) -> usize {
        RdfParser::from_format(format)
            .for_reader(data)
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap()
            .len()
    }

    #[test]
    fn every_oxigraph_format_round_trips_through_hdt() {
        let jsonld = RdfFormat::from_extension("jsonld").unwrap();
        let cases = [
            (
                RdfFormat::NTriples,
                "<https://example.test/s> <https://example.test/p> <https://example.test/o> .\n",
            ),
            (
                RdfFormat::NQuads,
                "<https://example.test/s> <https://example.test/p> <https://example.test/o> .\n",
            ),
            (
                RdfFormat::Turtle,
                "@prefix ex: <https://example.test/> . ex:s ex:p ex:o .\n",
            ),
            (
                RdfFormat::TriG,
                "@prefix ex: <https://example.test/> . ex:s ex:p ex:o .\n",
            ),
            (
                RdfFormat::N3,
                "@prefix ex: <https://example.test/> . ex:s ex:p ex:o .\n",
            ),
            (
                RdfFormat::RdfXml,
                "<?xml version=\"1.0\"?><rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\" xmlns:ex=\"https://example.test/\"><rdf:Description rdf:about=\"https://example.test/s\"><ex:p rdf:resource=\"https://example.test/o\"/></rdf:Description></rdf:RDF>",
            ),
            (
                jsonld,
                r#"{"@context":{"ex":"https://example.test/"},"@id":"ex:s","ex:p":{"@id":"ex:o"}}"#,
            ),
        ];
        for (format, input) in cases {
            let mut hdt = Vec::new();
            let policy = if format.supports_datasets() {
                NamedGraphPolicy::Flatten
            } else {
                NamedGraphPolicy::Reject
            };
            let stats = rdf_to_hdt(
                input.as_bytes(),
                format,
                "https://example.test/base",
                policy,
                &mut hdt,
            )
            .unwrap();
            assert_eq!(stats.triple_count, 1, "{}", format.name());
            let mut output = Vec::new();
            hdt_to_rdf(Cursor::new(hdt), format, &mut output).unwrap();
            assert_eq!(parse_count(&output, format), 1, "{}", format.name());
        }
    }

    #[test]
    fn format_parser_accepts_names_extensions_and_media_types() {
        assert_eq!(parse_rdf_format("n-triples").unwrap(), RdfFormat::NTriples);
        assert_eq!(parse_rdf_format(".ttl").unwrap(), RdfFormat::Turtle);
        assert_eq!(
            parse_rdf_format("application/n-quads").unwrap(),
            RdfFormat::NQuads
        );
        assert!(matches!(
            parse_rdf_format("nope"),
            Err(Error::UnsupportedFormat(_))
        ));
    }

    #[test]
    fn hdtq_preserves_graphs_in_both_annotation_modes() {
        let triple = [
            "https://example.test/s".to_owned(),
            "https://example.test/p".to_owned(),
            "https://example.test/o".to_owned(),
        ];
        let quads = [
            QuadRecord::named_graph(triple.clone(), "https://example.test/g1"),
            QuadRecord::named_graph(triple.clone(), "https://example.test/g2"),
            QuadRecord::default_graph([
                "https://example.test/s2".to_owned(),
                "https://example.test/p".to_owned(),
                "\"default\"".to_owned(),
            ]),
        ];
        for mode in [
            AnnotationMode::AnnotatedGraphs,
            AnnotationMode::AnnotatedTriples,
        ] {
            let mut hdtq = Vec::new();
            let stats = quads_to_hdtq(
                quads.iter().cloned().map(Ok),
                "https://example.test/dataset",
                mode,
                &mut hdtq,
            )
            .unwrap();
            assert_eq!(stats.triple_count, 2);
            let restored = hdtq_to_quads(Cursor::new(&hdtq)).unwrap();
            assert_eq!(restored.len(), 3);
            assert!(restored.contains(&QuadRecord::named_graph(
                triple.clone(),
                "https://example.test/g1",
            )));
            assert!(restored.contains(&QuadRecord::named_graph(
                triple.clone(),
                "https://example.test/g2",
            )));
            let mut nquads = Vec::new();
            hdtq_to_rdf(Cursor::new(hdtq), RdfFormat::NQuads, &mut nquads).unwrap();
            assert_eq!(parse_count(&nquads, RdfFormat::NQuads), 3);
        }
    }

    #[test]
    fn hdtq_rejects_corrupt_annotation_bitmap() {
        let mut hdtq = Vec::new();
        quads_to_hdtq(
            [Ok(QuadRecord::named_graph(
                [
                    "https://example.test/s".to_owned(),
                    "https://example.test/p".to_owned(),
                    "https://example.test/o".to_owned(),
                ],
                "https://example.test/g",
            ))],
            "https://example.test/dataset",
            AnnotationMode::AnnotatedGraphs,
            &mut hdtq,
        )
        .unwrap();
        *hdtq.last_mut().unwrap() ^= 1;
        assert!(matches!(
            hdtq_to_quads(Cursor::new(hdtq)),
            Err(Error::Hdt(_))
        ));
    }
}
