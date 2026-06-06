import subprocess
import pytest
import os
import shutil

RUST_BINS = [
    ("docutilsrs", [
        "docutils-rs", "rst2html-rs", "rst2html4-rs", "rst2html5-rs",
        "rst2latex-rs", "rst2man-rs", "rst2odt-rs", "rst2pseudoxml-rs",
        "rst2s5-rs", "rst2xetex-rs"
    ]),
    ("pygmentsrs", ["pygmentize-rs"]),
    ("sphinxdocrs", [
        "sphinx-build-rs", "sphinx-quickstart-rs", "sphinx-apidoc-rs", "sphinx-autogen-rs"
    ])
]

PYTHON_SCRIPTS = [
    # TODO: are these necessary?
    # "docutils-rs-py",
    # "rst2html-rs-py",
    # "rst2html4-rs-py",
    # "rst2html5-rs-py",
    # "rst2latex-rs-py",
    # "rst2man-rs-py",
    # "rst2odt-rs-py",
    # "rst2pseudoxml-rs-py",
    # "rst2s5-rs-py",
    # "rst2xetex-rs-py",
    # "pygmentize-rs-py",
    # "sphinx-build-rs-py",
    # "sphinx-quickstart-rs-py",
    # "sphinx-apidoc-rs-py",
    # "sphinx-autogen-rs-py",
]

@pytest.mark.parametrize("pkg,bin_name", [(pkg, bin_name) for pkg, bins in RUST_BINS for bin_name in bins])
def test_rust_binary_runs(pkg, bin_name):
    """Test that the stub Cargo binaries can be run and produce output."""
    output = subprocess.check_output(["cargo", "run", "-q", "-p", pkg, "--bin", bin_name, "--", "--help"], text=True, stderr=subprocess.STDOUT)
    assert output.strip(), f"No output produced by {bin_name}"

@pytest.mark.parametrize("script", PYTHON_SCRIPTS)
def test_python_script_runs(script):
    """Test that the installed pip entry point commands run successfully and produce output."""
    if not shutil.which(script):
        pytest.fail(f"{script} not found in PATH")
    output = subprocess.check_output([script, "--help"], text=True, stderr=subprocess.STDOUT)
    assert output.strip(), f"No output produced by {script}"
