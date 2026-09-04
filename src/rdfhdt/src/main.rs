use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "rdfhdt",
    about = "Convert Oxigraph RDF formats and HDT artifacts",
    after_help = "RDF formats: jsonld, n3, nq, nt, rdf, trig, ttl (aliases, extensions, and media types are accepted).\n\nExamples:\n  # Convert Turtle to HDT and write N-Triples\n  rdfhdt export input.ttl output.hdt --input-format ttl\n  rdfhdt import output.hdt output.nt --output-format nt\n\n  # Preserve named graphs in HDTQ using annotated triples\n  rdfhdt export dataset.trig dataset.hdtq --input-format trig --output-format hdtq --annotation-mode at\n  rdfhdt import dataset.hdtq restored.nq --input-format hdtq --output-format nq"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Convert an RDF input to RDFHDT or RDFHDTQ.
    ///
    /// RDF Header, Dictionary, Triples (RDFHDT) is a binary read-only compressed representation for RDF.
    /// RDFHDTQ is RDFHDT with Named Graph Quads per HDTQ-java.
    Export {
        /// Input RDF path, or `-` for stdin.
        input: PathBuf,
        /// Output HDT/HDTQ path, or `-` for stdout.
        output: PathBuf,
        /// Input RDF format: jsonld, n3, nq, nt, rdf, trig, or ttl. Aliases,
        /// extensions, and media types are also accepted.
        #[arg(
            long,
            default_value = "nt",
            value_parser = parse_rdf_format_arg
        )]
        input_format: rdfhdt::RdfFormat,
        /// Output artifact: `hdt` or `hdtq`.
        #[arg(long, default_value = "hdt")]
        output_format: String,
        /// Dataset IRI written to the HDT header.
        #[arg(long, default_value = "https://example.invalid/rdfhdt-dataset")]
        base_iri: String,
        /// Standard HDT named-graph policy. Use `--output-format hdtq` to
        /// preserve named graphs instead. `reject` fails when a named graph is
        /// encountered; `flatten` writes its triples and discards graph names.
        /// The default graph is unaffected. Only applies to HDT; there is no
        /// `store` value because named graphs are preserved by selecting HDTQ.
        #[arg(long, default_value = "reject")]
        graph_policy: String,
        /// HDTQ annotation mode for `--output-format hdtq`: `ag` stores one
        /// bitmap per graph, marking
        /// triples in that graph; `at` stores one bitmap per triple, marking
        /// graphs containing that triple. Only applies to HDTQ.
        #[arg(long, default_value = "ag")]
        annotation_mode: String,
    },
    /// Convert HDT or HDTQ to an RDF format.
    Import {
        /// Input HDT/HDTQ path, or `-` for stdin.
        input: PathBuf,
        /// Output RDF path, or `-` for stdout.
        output: PathBuf,
        /// Input artifact: `hdt` or `hdtq`.
        #[arg(long, default_value = "hdt")]
        input_format: String,
        /// Output RDF format: jsonld, n3, nq, nt, rdf, trig, or ttl. Aliases,
        /// extensions, and media types are also accepted.
        #[arg(
            long,
            default_value = "nt",
            value_parser = parse_rdf_format_arg
        )]
        output_format: rdfhdt::RdfFormat,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let stats = match cli.command {
        Command::Export {
            input,
            output,
            input_format,
            output_format,
            base_iri,
            graph_policy,
            annotation_mode,
        } => {
            let input_size = input.metadata().ok().map(|metadata| metadata.len());
            let input = open_input(&input)?;
            let mut output = open_output(&output)?;
            let stats = match output_format.to_ascii_lowercase().as_str() {
                "hdt" => rdfhdt::rdf_to_hdt(
                    input,
                    input_format,
                    &base_iri,
                    rdfhdt::NamedGraphPolicy::parse(&graph_policy)?,
                    &mut output,
                )?,
                "hdtq" => rdfhdt::rdf_to_hdtq(
                    input,
                    input_format,
                    &base_iri,
                    rdfhdt::AnnotationMode::parse(&annotation_mode)?,
                    &mut output,
                )?,
                other => {
                    return Err(format!(
                        "unsupported output artifact {other:?}; expected hdt or hdtq"
                    )
                    .into());
                }
            };
            if let Some(input_size) = input_size {
                debug_assert_eq!(stats.input_bytes, input_size);
            }
            stats
        }
        Command::Import {
            input,
            output,
            input_format,
            output_format,
        } => {
            let input = open_input(&input)?;
            let mut output = open_output(&output)?;
            match input_format.to_ascii_lowercase().as_str() {
                "hdt" => rdfhdt::hdt_to_rdf(BufReader::new(input), output_format, &mut output)?,
                "hdtq" => rdfhdt::hdtq_to_rdf(input, output_format, &mut output)?,
                other => {
                    return Err(format!(
                        "unsupported input artifact {other:?}; expected hdt or hdtq"
                    )
                    .into());
                }
            }
        }
    };
    eprintln!(
        "triples={} input_bytes={} output_bytes={}",
        stats.triple_count, stats.input_bytes, stats.output_bytes
    );
    Ok(())
}

fn parse_rdf_format_arg(value: &str) -> Result<rdfhdt::RdfFormat, String> {
    rdfhdt::parse_rdf_format(value).map_err(|error| error.to_string())
}

fn open_input(path: &PathBuf) -> io::Result<Box<dyn io::Read>> {
    if path == "-" {
        return Ok(Box::new(io::stdin()));
    }
    Ok(Box::new(File::open(path)?))
}

fn open_output(path: &PathBuf) -> io::Result<Box<dyn io::Write>> {
    if path == "-" {
        return Ok(Box::new(io::stdout()));
    }
    Ok(Box::new(BufWriter::new(File::create(path)?)))
}
