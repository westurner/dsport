use docutilsrs::{parse_rst_with_source, pseudo_xml};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.rst> <output.xml>", args[0]);
        std::process::exit(1);
    }
    
    let input_path = &args[1];
    let output_path = &args[2];
    
    let source = fs::read_to_string(input_path).expect("Failed to read input file");
    let tree = parse_rst_with_source(&source, input_path);
    let xml = pseudo_xml(&tree);
    
    fs::write(output_path, xml).expect("Failed to write output file");
}
