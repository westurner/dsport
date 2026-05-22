fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let status = if "rst2html4" == "docutils" {
        std::process::Command::new("python")
            .arg("-m")
            .arg("docutils.__main__")
            .args(&args)
            .status()
            .expect("Failed to execute python")
    } else {
        std::process::Command::new("python")
            .arg("-c")
            .arg(format!("import docutils.core; import sys; sys.argv[0] = '{}'; sys.exit(docutils.core.{}())", "rst2html4", "rst2html4"))
            .args(&args)
            .status()
            .expect("Failed to execute python")
    };
    std::process::exit(status.code().unwrap_or(1));
}
