use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "rdfhdt",
    about = "Convert streaming N-Triples and HDT artifacts"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Convert N-Triples to HDT.
    Export {
        /// Input N-Triples path, or `-` for stdin.
        input: PathBuf,
        /// Output HDT path, or `-` for stdout.
        output: PathBuf,
        /// Dataset IRI written to the HDT header.
        #[arg(long, default_value = "https://example.invalid/rdfhdt-dataset")]
        base_iri: String,
    },
    /// Convert HDT to N-Triples.
    Import {
        /// Input HDT path, or `-` for stdin.
        input: PathBuf,
        /// Output N-Triples path, or `-` for stdout.
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let stats = match cli.command {
        Command::Export {
            input,
            output,
            base_iri,
        } => {
            let input_size = input.metadata().ok().map(|metadata| metadata.len());
            let input = open_input(&input)?;
            let mut output = open_output(&output)?;
            let stats = rdfhdt::ntriples_to_hdt(input, &base_iri, &mut output)?;
            if let Some(input_size) = input_size {
                debug_assert_eq!(stats.input_bytes, input_size);
            }
            stats
        }
        Command::Import { input, output } => {
            let input = open_input(&input)?;
            let mut output = open_output(&output)?;
            rdfhdt::hdt_to_ntriples(BufReader::new(input), &mut output)?
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
