use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "rdfhdt",
    about = "Convert Oxigraph RDF formats and HDT artifacts"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Convert an RDF input to HDT or HDTQ.
    Export {
        /// Input RDF path, or `-` for stdin.
        input: PathBuf,
        /// Output HDT/HDTQ path, or `-` for stdout.
        output: PathBuf,
        /// Input format name, extension, or media type.
        #[arg(long, default_value = "nt")]
        input_format: String,
        /// Output artifact: `hdt` or `hdtq`.
        #[arg(long, default_value = "hdt")]
        output_format: String,
        /// Dataset IRI written to the HDT header.
        #[arg(long, default_value = "https://example.invalid/rdfhdt-dataset")]
        base_iri: String,
        /// Standard HDT named graph policy: `reject` or `flatten`.
        #[arg(long, default_value = "reject")]
        graph_policy: String,
        /// HDTQ annotation mode: `ag` or `at`.
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
        /// Output RDF format name, extension, or media type.
        #[arg(long, default_value = "nt")]
        output_format: String,
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
            let input_format = rdfhdt::parse_rdf_format(&input_format)?;
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
            let output_format = rdfhdt::parse_rdf_format(&output_format)?;
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
