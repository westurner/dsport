use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use docutilsrs::{html5, parse_rst_with_source};
use std::fs;

#[derive(Parser, Debug)]
#[command(
    name = "rst2html5",
    about = "Generate HTML5 documents from standalone reStructuredText sources.",
    help_template = "Usage\n=====
  {usage}\n\n{about}\n\nOptions\n=======\n{options}"
)]
struct Cli {
    #[arg(short, long, help = "Verbose output.")]
    verbose: bool,

    #[arg(short, long, help = "Quiet output.")]
    quiet: bool,

    #[arg(help = "Source file")]
    source: Option<String>,

    #[arg(help = "Destination file")]
    destination: Option<String>,

    #[arg(long, help = "Call Python script instead of Rust API (optionally specify python binary)", value_name = "PYTHON_BIN_PATH", num_args = 0..=1, default_missing_value = "python")]
    py: Option<String>,

    #[arg(long, hide = true, value_enum)]
    generate_completion: Option<Shell>,

    #[command(flatten)]
    common: docutilsrs::cli::CommonOptions,

    #[command(flatten)]
    html5: docutilsrs::cli::Html5Options,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut py_passed = false;
    let mut python_bin = "python".to_string();
    let mut python_args = Vec::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--py" {
            py_passed = true;
            if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                python_bin = args[i + 1].clone();
                i += 2;
            } else {
                i += 1;
            }
        } else if let Some(val) = args[i].strip_prefix("--py=") {
            py_passed = true;
            python_bin = val.to_string();
            i += 1;
        } else {
            python_args.push(args[i].clone());
            i += 1;
        }
    }

    if py_passed {
        let status = std::process::Command::new(&python_bin)
            .arg("-c")
            .arg(format!("import docutils.core; import sys; sys.argv[0] = '{}'; sys.exit(docutils.core.{}())", "rst2html5", "rst2html5"))
            .args(&python_args)
            .status()
            .unwrap_or_else(|_| panic!("Failed to execute {}", python_bin));
        std::process::exit(status.code().unwrap_or(1));
    }

    let cli = Cli::parse();

    if let Some(shell) = cli.generate_completion {
        let mut cmd = Cli::command();
        let bin_name = cmd.get_name().to_string();
        clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
        return;
    }

    let source_path = cli
        .source
        .unwrap_or_else(|| panic!("<source> is currently required in this early drop-in"));
    let dest_path = cli
        .destination
        .unwrap_or_else(|| panic!("<destination> is currently required in this early drop-in"));

    let source = fs::read_to_string(&source_path).expect("Failed to read input file");
    let tree = parse_rst_with_source(&source, &source_path);
    let output = html5(&tree, &cli.html5, &cli.common);

    let wrapped = format!(
        "<!DOCTYPE html>\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\" lang=\"en\">\n<head>\n<meta charset=\"utf-8\" />\n<title>Docutils HTML5</title>\n</head>\n<body>\n<main>\n{}\n</main>\n</body>\n</html>\n",
        output
    );

    fs::write(&dest_path, wrapped).expect("Failed to write output file");
}
