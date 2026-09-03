use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, Cursor, Read, Write};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crc::Crc;
use hdt::containers::vbyte::{encode_vbyte, read_vbyte};
use hdt::{Hdt, dict_sect_pfc::DictSectPFC};
use oxrdfio::{RdfParser, RdfSerializer};

use crate::{
    CountingReader, CountingWriter, Error, Position, QuadRecord, RdfFormat, Result, Stats,
    parse_single_ntriple, term_to_hdt, term_to_ntriples,
};

/// The graph annotation layout used by HDTQ-java.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AnnotationMode {
    /// One bitmap per graph, with one bit for every SPO triple.
    #[default]
    AnnotatedGraphs,
    /// One bitmap per SPO triple, with one bit for every graph.
    AnnotatedTriples,
}

impl AnnotationMode {
    pub fn parse(value: &str) -> Result<Self> {
        match value.to_ascii_lowercase().as_str() {
            "ag" | "annotated-graphs" | "annotated_graphs" => Ok(Self::AnnotatedGraphs),
            "at" | "annotated-triples" | "annotated_triples" => Ok(Self::AnnotatedTriples),
            other => Err(Error::Hdt(format!(
                "invalid HDTQ annotation mode {other:?}"
            ))),
        }
    }

    fn control_type(self) -> u8 {
        match self {
            Self::AnnotatedGraphs => 5,
            Self::AnnotatedTriples => 6,
        }
    }
}

const ANNOTATION_FORMAT: &str = "<http://purl.org/HDT/hdt#AnnotationMode>";
const DEFAULT_GRAPH: &str = "urn:x-arq:DefaultGraphNode";

/// Parses any Oxigraph RDF format and writes an HDTQ dataset.
pub fn rdf_to_hdtq<R, W>(
    reader: R,
    input_format: RdfFormat,
    base_iri: &str,
    mode: AnnotationMode,
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
    let mut stats = quads_to_hdtq(quads, base_iri, mode, writer)?;
    stats.input_bytes = input_bytes.load(Ordering::Relaxed);
    Ok(stats)
}

/// Writes graph-preserving HDTQ output using the HDTQ-java wire format.
pub fn quads_to_hdtq<I, W>(
    quads: I,
    base_iri: &str,
    mode: AnnotationMode,
    writer: W,
) -> Result<Stats>
where
    I: IntoIterator<Item = Result<QuadRecord>>,
    W: Write,
{
    let quads: Vec<QuadRecord> = quads.into_iter().collect::<Result<_>>()?;
    let mut triples = BTreeSet::<[String; 3]>::new();
    let mut graphs = BTreeSet::<String>::new();
    let mut relations = BTreeSet::<([String; 3], String)>::new();
    for quad in quads {
        let graph = quad.graph_name.unwrap_or_else(|| DEFAULT_GRAPH.to_owned());
        if graph.is_empty() || graph.contains(['<', '>', '\n', '\r']) {
            return Err(Error::InvalidTerm {
                term: graph,
                position: "graph",
            });
        }
        triples.insert(quad.triple.clone());
        graphs.insert(graph.clone());
        relations.insert((quad.triple, graph));
    }

    let triples: Vec<[String; 3]> = triples.into_iter().collect();
    let hdt =
        Hdt::from_triples(triples, base_iri).map_err(|error| Error::Hdt(error.to_string()))?;
    let mut standard = Vec::new();
    hdt.write(&mut standard)
        .map_err(|error| Error::Hdt(error.to_string()))?;

    let dictionary_end = dictionary_end(&standard)?;
    let mut graph_section = Vec::new();
    let graph_refs: BTreeSet<&str> = graphs.iter().map(String::as_str).collect();
    DictSectPFC::compress(&graph_refs, 16)
        .write(&mut graph_section)
        .map_err(|error| Error::Hdt(error.to_string()))?;

    let triple_order: Vec<[String; 3]> = hdt
        .triples_all()
        .map(|triple| triple.map(|term| term.to_string()))
        .collect();
    let triple_ids: BTreeMap<[String; 3], usize> = triple_order
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, triple)| (triple, index))
        .collect();
    let graph_order: Vec<String> = graphs.into_iter().collect();
    let graph_ids: BTreeMap<String, usize> = graph_order
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, graph)| (graph, index))
        .collect();
    let relation_ids: BTreeSet<(usize, usize)> = relations
        .into_iter()
        .map(|(triple, graph)| {
            let triple_id = triple_ids.get(&triple).copied().ok_or_else(|| {
                Error::Hdt("HDT triple order did not contain input triple".to_owned())
            })?;
            let graph_id = graph_ids.get(&graph).copied().ok_or_else(|| {
                Error::Hdt("HDTQ graph order did not contain input graph".to_owned())
            })?;
            Ok((triple_id, graph_id))
        })
        .collect::<Result<_>>()?;

    let mut output = CountingWriter::new(writer);
    output.write_all(&standard[..dictionary_end])?;
    output.write_all(&graph_section)?;
    output.write_all(&standard[dictionary_end..])?;
    write_annotation_control(&mut output, mode)?;
    match mode {
        AnnotationMode::AnnotatedGraphs => {
            for graph_id in 0..graph_order.len() {
                let bits = (0..triple_order.len())
                    .map(|triple_id| relation_ids.contains(&(triple_id, graph_id)))
                    .collect::<Vec<_>>();
                write_bitmap(&mut output, &bits)?;
            }
        }
        AnnotationMode::AnnotatedTriples => {
            for triple_id in 0..triple_order.len() {
                let bits = (0..graph_order.len())
                    .map(|graph_id| relation_ids.contains(&(triple_id, graph_id)))
                    .collect::<Vec<_>>();
                write_bitmap(&mut output, &bits)?;
            }
        }
    }
    output.flush()?;
    Ok(Stats {
        triple_count: triple_order.len() as u64,
        output_bytes: output.count(),
        ..Stats::default()
    })
}

/// Reads an HDTQ file and returns its graph-preserving records.
pub fn hdtq_to_quads<R: Read>(mut reader: R) -> Result<Vec<QuadRecord>> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    let dictionary_end = dictionary_end(&bytes)?;
    let mut graph_reader = Cursor::new(&bytes[dictionary_end..]);
    let graph_section = read_pfc(&mut graph_reader)?;
    let graph_section_end = dictionary_end + graph_reader.position() as usize;
    let mut triples_reader = Cursor::new(&bytes[graph_section_end..]);
    hdt::triples::TriplesBitmap::read_sect(&mut triples_reader)
        .map_err(|error| Error::Hdt(error.to_string()))?;
    let annotation_start = graph_section_end + triples_reader.position() as usize;

    let graph_names = (1..=graph_section.num_strings)
        .map(|id| {
            graph_section
                .extract(id)
                .map_err(|error| Error::Hdt(error.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;
    let mut standard = Vec::with_capacity(annotation_start);
    standard.extend_from_slice(&bytes[..dictionary_end]);
    standard.extend_from_slice(&bytes[graph_section_end..annotation_start]);
    let hdt = Hdt::read(Cursor::new(standard)).map_err(|error| Error::Hdt(error.to_string()))?;
    let triples: Vec<[String; 3]> = hdt
        .triples_all()
        .map(|triple| triple.map(|term| term.to_string()))
        .collect();

    let mut annotation_reader = Cursor::new(&bytes[annotation_start..]);
    let mode = read_annotation_control(&mut annotation_reader)?;
    let bitmap_count = match mode {
        AnnotationMode::AnnotatedGraphs => graph_names.len(),
        AnnotationMode::AnnotatedTriples => triples.len(),
    };
    let mut bitmaps = Vec::with_capacity(bitmap_count);
    for _ in 0..bitmap_count {
        bitmaps.push(read_bitmap(&mut annotation_reader)?);
    }
    if annotation_reader.position() as usize != bytes[annotation_start..].len() {
        return Err(Error::Hdt(
            "trailing bytes after HDTQ annotation bitmaps".to_owned(),
        ));
    }

    let mut quads = Vec::new();
    match mode {
        AnnotationMode::AnnotatedGraphs => {
            if bitmaps.iter().any(|bitmap| bitmap.len() != triples.len()) {
                return Err(Error::Hdt(
                    "HDTQ graph bitmap length does not match triple count".to_owned(),
                ));
            }
            for (graph_id, bits) in bitmaps.iter().enumerate() {
                for (triple_id, present) in bits.iter().copied().enumerate() {
                    if present {
                        quads.push(QuadRecord {
                            triple: triples[triple_id].clone(),
                            graph_name: Some(graph_names[graph_id].clone()),
                        });
                    }
                }
            }
        }
        AnnotationMode::AnnotatedTriples => {
            if bitmaps
                .iter()
                .any(|bitmap| bitmap.len() != graph_names.len())
            {
                return Err(Error::Hdt(
                    "HDTQ triple bitmap length does not match graph count".to_owned(),
                ));
            }
            for (triple_id, bits) in bitmaps.iter().enumerate() {
                for (graph_id, present) in bits.iter().copied().enumerate() {
                    if present {
                        quads.push(QuadRecord {
                            triple: triples[triple_id].clone(),
                            graph_name: Some(graph_names[graph_id].clone()),
                        });
                    }
                }
            }
        }
    }
    for quad in &mut quads {
        if quad.graph_name.as_deref() == Some(DEFAULT_GRAPH) {
            quad.graph_name = None;
        }
    }
    Ok(quads)
}

/// Reads HDTQ and serializes its quads to any Oxigraph RDF format.
pub fn hdtq_to_rdf<R, W>(reader: R, output_format: RdfFormat, writer: W) -> Result<Stats>
where
    R: Read,
    W: Write,
{
    let quads = hdtq_to_quads(reader)?;
    let mut writer = CountingWriter::new(writer);
    let mut serializer = RdfSerializer::from_format(output_format).for_writer(writer);
    for quad in &quads {
        let line = if let Some(graph) = &quad.graph_name {
            format!(
                "{} {} {} {} .\n",
                term_to_ntriples(&quad.triple[0], Position::Subject)?,
                term_to_ntriples(&quad.triple[1], Position::Predicate)?,
                term_to_ntriples(&quad.triple[2], Position::Object)?,
                term_to_ntriples(graph, Position::Graph)?,
            )
        } else {
            format!(
                "{} {} {} .\n",
                term_to_ntriples(&quad.triple[0], Position::Subject)?,
                term_to_ntriples(&quad.triple[1], Position::Predicate)?,
                term_to_ntriples(&quad.triple[2], Position::Object)?,
            )
        };
        let parsed = if quad.graph_name.is_some() {
            let mut parser = RdfParser::from_format(RdfFormat::NQuads).for_reader(line.as_bytes());
            parser
                .next()
                .ok_or_else(|| Error::Parse("generated N-Quads line was empty".to_owned()))?
                .map_err(|error| Error::Parse(error.to_string()))?
        } else {
            parse_single_ntriple(&line)?
        };
        serializer.serialize_quad(&parsed)?;
    }
    writer = serializer.finish()?;
    writer.flush()?;
    Ok(Stats {
        triple_count: quads.len() as u64,
        output_bytes: writer.count(),
        ..Stats::default()
    })
}

fn dictionary_end(bytes: &[u8]) -> Result<usize> {
    let mut reader = Cursor::new(bytes);
    Hdt::read_header(&mut reader).map_err(|error| Error::Hdt(error.to_string()))?;
    let dict = hdt::four_sect_dict::FourSectDict::read(&mut reader)
        .map_err(|error| Error::Hdt(error.to_string()))?;
    dict.validate()
        .map_err(|error| Error::Hdt(error.to_string()))?;
    Ok(reader.position() as usize)
}

fn read_pfc(reader: &mut Cursor<&[u8]>) -> Result<DictSectPFC> {
    let section = DictSectPFC::read(reader).map_err(|error| Error::Hdt(error.to_string()))?;
    section
        .join()
        .map_err(|_| Error::Hdt("graph dictionary validation thread panicked".to_owned()))?
        .map_err(|error| Error::Hdt(error.to_string()))
}

fn write_annotation_control(writer: &mut impl Write, mode: AnnotationMode) -> Result<()> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"$HDT");
    bytes.push(mode.control_type());
    bytes.extend_from_slice(ANNOTATION_FORMAT.as_bytes());
    bytes.push(0);
    bytes.push(0);
    let crc = Crc::<u16>::new(&crc::CRC_16_ARC).checksum(&bytes);
    bytes.extend_from_slice(&crc.to_le_bytes());
    writer.write_all(&bytes)?;
    Ok(())
}

fn read_annotation_control(reader: &mut Cursor<&[u8]>) -> Result<AnnotationMode> {
    let mut cookie = [0; 4];
    reader.read_exact(&mut cookie)?;
    if cookie != *b"$HDT" {
        return Err(Error::Hdt(
            "missing HDTQ annotation control cookie".to_owned(),
        ));
    }
    let mut control_type = [0];
    reader.read_exact(&mut control_type)?;
    let mode = match control_type[0] {
        5 => AnnotationMode::AnnotatedGraphs,
        6 => AnnotationMode::AnnotatedTriples,
        other => return Err(Error::Hdt(format!("unsupported HDTQ control type {other}"))),
    };
    let mut format = Vec::new();
    reader.read_until(0, &mut format)?;
    if format.pop() != Some(0) || format != ANNOTATION_FORMAT.as_bytes() {
        return Err(Error::Hdt("invalid HDTQ annotation format".to_owned()));
    }
    let mut properties = Vec::new();
    reader.read_until(0, &mut properties)?;
    if properties.pop() != Some(0) {
        return Err(Error::Hdt(
            "unterminated HDTQ annotation properties".to_owned(),
        ));
    }
    let mut checksum = [0; 2];
    reader.read_exact(&mut checksum)?;
    let mut header = Vec::new();
    header.extend_from_slice(b"$HDT");
    header.push(control_type[0]);
    header.extend_from_slice(ANNOTATION_FORMAT.as_bytes());
    header.push(0);
    header.extend_from_slice(&properties);
    header.push(0);
    let expected = Crc::<u16>::new(&crc::CRC_16_ARC).checksum(&header);
    if u16::from_le_bytes(checksum) != expected {
        return Err(Error::Hdt(
            "invalid HDTQ annotation control checksum".to_owned(),
        ));
    }
    Ok(mode)
}

fn write_bitmap(writer: &mut impl Write, bits: &[bool]) -> Result<()> {
    let mut header = vec![1];
    header.extend_from_slice(&encode_vbyte(bits.len()));
    let crc8 = Crc::<u8>::new(&crc::CRC_8_SMBUS).checksum(&header);
    writer.write_all(&header)?;
    writer.write_all(&[crc8])?;
    let mut body = vec![0; bits.len().div_ceil(8)];
    for (index, bit) in bits.iter().copied().enumerate() {
        if bit {
            body[index / 8] |= 1 << (index % 8);
        }
    }
    writer.write_all(&body)?;
    let crc32 = Crc::<u32>::new(&crc::CRC_32_ISCSI).checksum(&body);
    writer.write_all(&crc32.to_le_bytes())?;
    Ok(())
}

fn read_bitmap(reader: &mut Cursor<&[u8]>) -> Result<Vec<bool>> {
    let mut bitmap_type = [0];
    reader.read_exact(&mut bitmap_type)?;
    if bitmap_type[0] != 1 {
        return Err(Error::Hdt(format!(
            "unsupported HDTQ bitmap type {}",
            bitmap_type[0]
        )));
    }
    let (num_bits, encoded_bits) = read_vbyte(reader)?;
    let mut crc_header = vec![1];
    crc_header.extend_from_slice(&encoded_bits);
    let mut checksum8 = [0];
    reader.read_exact(&mut checksum8)?;
    let expected8 = Crc::<u8>::new(&crc::CRC_8_SMBUS).checksum(&crc_header);
    if checksum8[0] != expected8 {
        return Err(Error::Hdt("invalid HDTQ bitmap header checksum".to_owned()));
    }
    let byte_count = num_bits
        .checked_add(7)
        .ok_or_else(|| Error::Hdt("HDTQ bitmap size overflow".to_owned()))?
        / 8;
    let mut body = vec![0; byte_count];
    reader.read_exact(&mut body)?;
    let mut checksum32 = [0; 4];
    reader.read_exact(&mut checksum32)?;
    let expected32 = Crc::<u32>::new(&crc::CRC_32_ISCSI).checksum(&body);
    if u32::from_le_bytes(checksum32) != expected32 {
        return Err(Error::Hdt("invalid HDTQ bitmap body checksum".to_owned()));
    }
    Ok((0..num_bits)
        .map(|index| body[index / 8] & (1 << (index % 8)) != 0)
        .collect())
}
