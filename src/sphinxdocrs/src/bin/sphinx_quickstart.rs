fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mod_path = match "sphinx_quickstart".replace("_", "").as_str() {
        "sphinxbuild" => "sphinx.cmd.build",
        "sphinxquickstart" => "sphinx.cmd.quickstart",
        "sphinxapidoc" => "sphinx.ext.apidoc",
        "sphinxautogen" => "sphinx.ext.autosummary.generate",
        _ => "sphinx.cmd.build",
    };
    let py_code = format!("import sys; from {} import main; sys.exit(main())", mod_path);
    let status = std::process::Command::new("python")
        .arg("-c")
        .arg(&py_code)
        .args(&args)
        .status()
        .expect("Failed to execute python");
    std::process::exit(status.code().unwrap_or(1));
}
