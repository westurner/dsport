use docutilsrs::{parse_rst_with_source, html5};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.rst> <output.html>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let source = fs::read_to_string(input_path).expect("Failed to read input file");
    let tree = parse_rst_with_source(&source, input_path);
    let fragment = html5(&tree);

    let html = format!(
        "<!DOCTYPE html>\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\" lang=\"en\">\n<head>\n<meta charset=\"utf-8\" />\n<title>Docutils HTML5</title>\n</head>\n<body>\n<main>\n{}\n</main>\n</body>\n</html>\n",
        fragment
    );

    fs::write(output_path, html).expect("Failed to write output file");
}
