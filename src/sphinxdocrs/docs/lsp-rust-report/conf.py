# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

import os
from pathlib import Path

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'SphinxDocRS LSP Report'
copyright = '2026, SphinxDocRS'
author = 'SphinxDocRS'

version = '0.1.0'
release = '0.1.0'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = []

# LSP is explicitly protected and configured per language. The API page is
# generated before the HTML build by sphinx-autodoc-rs; this build only renders
# that checked-in RST report.
source_backend = 'lsp'
source_lsp_sandbox = 'protected-lsp'
cargo_home = Path(os.environ.get('CARGO_HOME', str(Path.home() / '.cargo'))).resolve()
rustup_home = Path(os.environ.get('RUSTUP_HOME', str(Path.home() / '.rustup'))).resolve()
rust_analyzer = os.environ.get('SPHINXDOCRS_RUST_ANALYZER', 'rust-analyzer')
rust_analyzer_path = Path(rust_analyzer)
if rust_analyzer_path.is_absolute():
	toolchain_bin = rust_analyzer_path.resolve().parent
	toolchain_lib = toolchain_bin.parent / 'lib'
	rust_server_argv = [
		'/usr/bin/env',
		f'LD_LIBRARY_PATH={toolchain_lib}',
		str(rust_analyzer_path.resolve()),
	]
else:
	toolchain_bin = None
	rust_server_argv = [rust_analyzer]

source_lsp_servers = {'rust': rust_server_argv}
source_lsp_server_environment = {
	'rust': {
		'HOME': str(Path.home()),
		'CARGO_HOME': str(cargo_home),
		'RUSTUP_HOME': str(rustup_home),
		'CARGO_TARGET_DIR': '/tmp/sphinxdocrs-lsp-target',
		'PATH': os.pathsep.join(
			str(path)
			for path in [cargo_home / 'bin', toolchain_bin, Path('/usr/bin'), Path('/bin')]
			if path is not None
		),
	},
}
source_lsp_server_read_only_roots = {
	'rust': [
		str(cargo_home),
		str(rustup_home),
	],
}

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']



# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'alabaster'
html_static_path = ['_static']
