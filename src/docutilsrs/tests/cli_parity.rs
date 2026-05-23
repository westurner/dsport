use std::collections::HashSet;
use std::process::Command;

fn get_rust_help(bin: &str) -> String {
    let output = Command::new("cargo")
        .args(["run", "-q", "--bin", bin, "--", "--help"])
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_python_help(bin_name: &str) -> String {
    let python_cmd = format!(
        "import docutils.core; import sys; sys.argv=['{}.py', '--help']; sys.exit(docutils.core.{}())",
        bin_name,
        bin_name.replace("-rs", "")
    );
    let output = Command::new("python3")
        .env("PYTHONPATH", "../docutils/docutils")
        .args(["-c", &python_cmd])
        .output()
        .expect("Failed to execute python");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn extract_options(help_text: &str) -> HashSet<String> {
    let mut options = HashSet::new();
    for line in help_text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("-") {
            if trimmed.chars().all(|c| c == '-') {
                continue;
            }
            
            for part in trimmed.split_whitespace() {
                if !part.starts_with("-") {
                    break;
                }
                
                let clean = part.trim_matches(',');
                let clean = clean.split('=').next().unwrap();
                let clean = clean.trim_end_matches('.');
                let clean = clean.trim_matches('"');
                let clean = clean.trim_matches('\'');
                
                if clean.starts_with("-") && clean.len() >= 2 && !clean.chars().skip(1).all(|c| c == '-') {
                    options.insert(clean.to_string());
                }
            }
        }
    }
    options
}

macro_rules! generate_parity_test {
    ($test_name:ident, $bin_name:expr, $py_name:expr) => {
        #[test]
        fn $test_name() {
            let rust_help = get_rust_help($bin_name);
            let python_help = get_python_help($py_name);

            let rust_options = extract_options(&rust_help);
            let python_options = extract_options(&python_help);

            let mut missing = Vec::new();
            for opt in python_options.iter() {
                if !rust_options.contains(opt) {
                    missing.push(opt.clone());
                }
            }

            missing.sort();
            if !missing.is_empty() {
                panic!("{} is missing the following options present in {}:\n{:#?}", $bin_name, $py_name, missing);
            }
        }
    };
}

generate_parity_test!(test_rst2html5_cli_parity, "rst2html5-rs", "rst2html5");
generate_parity_test!(test_rst2latex_cli_parity, "rst2latex-rs", "rst2latex");
generate_parity_test!(test_rst2man_cli_parity, "rst2man-rs", "rst2man");
generate_parity_test!(test_rst2odt_cli_parity, "rst2odt-rs", "rst2odt");
generate_parity_test!(test_rst2pseudoxml_cli_parity, "rst2pseudoxml-rs", "rst2pseudoxml");
