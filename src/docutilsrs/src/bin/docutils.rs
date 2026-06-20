fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let status = std::process::Command::new("python")
        .arg("-m")
        .arg("docutils.__main__")
        .args(&args)
        .status()
        .expect("Failed to execute python");
    std::process::exit(status.code().unwrap_or(1));
}
