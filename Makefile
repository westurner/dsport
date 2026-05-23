.PHONY: all build test develop clean

all: build

# ========== BUILD ==========
build: build-docutils build-pygments build-sphinx

build-docutils:
	cd src/docutilsrs && cargo build

build-pygments:
	cd src/pygmentsrs && cargo build

build-sphinx:
	cd src/sphinxdocrs && cargo build

# ========== TEST ==========
test: test-cargo test-python

test-cargo: test-cargo-docutils test-cargo-pygments test-cargo-sphinx

test-cargo-docutils:
	cd src/docutilsrs && cargo test

test-cargo-pygments:
	cd src/pygmentsrs && cargo test

test-cargo-sphinx:
	cd src/sphinxdocrs && cargo test

test-python:
	cd src && .venv/bin/pytest tests/

# ========== DEVELOP (maturin) ==========
develop: develop-docutils develop-pygments develop-sphinx

develop-docutils:
	cd src && .venv/bin/maturin develop --manifest-path docutilsrs/Cargo.toml --release

develop-pygments:
	cd src && .venv/bin/maturin develop --manifest-path pygmentsrs/Cargo.toml --release

develop-sphinx:
	cd src && .venv/bin/maturin develop --manifest-path sphinxdocrs/Cargo.toml --release

# ========== TODO  ==========
test-all: \
	test-rst2html-build-docs-docutils-readme \
	test-rst2html5-build-docs-docutils-readme \
	test-sphinxdocrs-build-docs-sphinx


SPHINXRS_OUTPUT ?= ${PWD}/build/tests/sphinxdocrs
DOCUTILSRS_OUTPUT ?= ${PWD}/build/tests/docutilsrs

test-sphinxdocrs-build-docs-sphinx:
	mkdir -p "${SPHINXRS_OUTPUT}"
	set -x; \
	cd src/sphinxdocrs; time cargo run -q -p sphinxdocrs --bin \
		sphinx-build-rs -- ../sphinx/doc "${SPHINXRS_OUTPUT}" 2>&1 \
		| tee "${SPHINXRS_OUTPUT}"/sphinx-build.log.txt
	test -d "${SPHINXRS_OUTPUT}"
	test -e "${SPHINXRS_OUTPUT}"/index.html
	test -e "${SPHINXRS_OUTPUT}"/sphinx-build.log.txt

test-rst2html-build-docs-docutils-readme: 
	mkdir -p "${DOCUTILSRS_OUTPUT}"
	set -x; \
	cd src/docutilsrs; time cargo run -q -p docutilsrs --bin \
		rst2html-rs -- ../docutils/docutils/README.rst "${DOCUTILSRS_OUTPUT}/"README.rst2html.html
	test -d "${DOCUTILSRS_OUTPUT}"
	test -e "${DOCUTILSRS_OUTPUT}"/README.rst2html.html

test-rst2html5-build-docs-docutils-readme: 
	mkdir -p "${DOCUTILSRS_OUTPUT}"
	set -x; \
	cd src/docutilsrs; time cargo run -q -p docutilsrs --bin \
		rst2html5-rs -- ../docutils/docutils/README.rst "${DOCUTILSRS_OUTPUT}/"README.rst2html5.html
	test -e "${DOCUTILSRS_OUTPUT}"/README.rst2html5.html

# ========== MATH DEMO ==========
# Demonstrate end-to-end math rendering: RaTeX is the default backend
# for docutilsrs, myst-md-rs, and sphinxdocrs; sphinxdocrs also reads
# `mathjax_path` / `imgmath_*` from a sphinx `conf.py`.
#
# Each example takes a sphinx-build-style `-o OUTDIR` flag. Override
# `MATH_DEMO_OUT` on the make command line to change the build path:
#
#   make demo-math MATH_DEMO_OUT=/tmp/math-demo
#
# Default output tree (per ${PWD} at make invocation):
#
#   ${PWD}/build/tests/math-demo/
#   ├── mathrenderrs/   backends.html, test.log.txt
#   ├── docutilsrs/     input.rst, output.html, output.pseudoxml
#   ├── myst-md-rs/     input.md, ratex.html, mathjax.html, imgmath.html
#   └── sphinxdocrs/    {mathjax,imgmath,ratex}/conf.py + report.txt

MATH_DEMO_OUT      ?= ${PWD}/build/tests/math-demo
MATH_DEMO_DOCUTILS ?= $(MATH_DEMO_OUT)/docutilsrs
MATH_DEMO_MYST     ?= $(MATH_DEMO_OUT)/myst-md-rs
MATH_DEMO_SPHINX   ?= $(MATH_DEMO_OUT)/sphinxdocrs
MATH_DEMO_RATEX    ?= $(MATH_DEMO_OUT)/mathrenderrs

.PHONY: demo-math demo-math-docutils demo-math-myst demo-math-sphinx \
	demo-math-mathrenderrs demo-math-fetch-mathjax

demo-math: demo-math-mathrenderrs demo-math-docutils demo-math-sphinx \
	demo-math-fetch-mathjax demo-math-myst
	@echo "=== math demos written to $(MATH_DEMO_OUT) ==="
	@ls -1 $(MATH_DEMO_OUT)/*/

# --- mathrenderrs: backend matrix (Ratex / MathJax / ImgMath) -------
demo-math-mathrenderrs:
	mkdir -p "$(MATH_DEMO_RATEX)"
	cd src && cargo test -p mathrenderrs -- --nocapture 2>&1 \
		| tee "$(MATH_DEMO_RATEX)"/test.log.txt
	cd src && cargo run -q --example backend_matrix -p mathrenderrs -- \
		-o "$(MATH_DEMO_RATEX)"
	@test -e "$(MATH_DEMO_RATEX)"/backends.html
	@echo "  -> $(MATH_DEMO_RATEX)/"

# --- docutilsrs: .. math:: directive + :math: role through rst2html5 -
demo-math-docutils:
	mkdir -p "$(MATH_DEMO_DOCUTILS)"
	printf '%s\n' \
		'Math demo' \
		'=========' \
		'' \
		'Inline: :math:`E = mc^2` in a paragraph.' \
		'' \
		'.. math::' \
		'' \
		'   \int_0^1 x^2 \, dx = \frac{1}{3}' \
		'' \
		> "$(MATH_DEMO_DOCUTILS)"/input.rst
	cd src/docutilsrs && cargo run -q -p docutilsrs --example math_demo -- \
		-o "$(MATH_DEMO_DOCUTILS)" \
		"$(MATH_DEMO_DOCUTILS)"/input.rst
	@grep -q 'data-renderer="ratex"' "$(MATH_DEMO_DOCUTILS)"/output.html \
		&& echo "  -> RaTeX SVG embedded in $(MATH_DEMO_DOCUTILS)/output.html" \
		|| (echo "FAIL: no RaTeX output" && exit 1)
	@grep -q '<math' "$(MATH_DEMO_DOCUTILS)"/output.pseudoxml \
		&& echo "  -> <math>/<math_block> in $(MATH_DEMO_DOCUTILS)/output.pseudoxml"

# --- myst-md-rs: $…$ inline, $$…$$ display, ```math fenced ----------
# NOTE: literal `$` in a Make recipe must be doubled (`$$`), and a
# literal `$$` must be quadrupled (`$$$$`); otherwise Make expands
# `$E` -> empty and `$$` -> `$` before printf ever runs, which
# silently strips the dollar-math delimiters from the fixture.
demo-math-myst:
	mkdir -p "$(MATH_DEMO_MYST)"
	printf '%s\n' \
		'# Math demo' \
		'' \
		'Inline math: $$E = mc^2$$.' \
		'' \
		'Display math:' \
		'' \
		'$$$$' \
		'\int_0^1 x^2 \, dx = \frac{1}{3}' \
		'$$$$' \
		'' \
		'Fenced math block:' \
		'' \
		'```math' \
		'a^2 + b^2 = c^2' \
		'```' \
		'' \
		> "$(MATH_DEMO_MYST)"/input.md
	MATHJAX_SRC=$$(awk -F'"' '/"url"/{print $$4; exit}' \
		"$(MATH_DEMO_SPHINX)"/mathjax.mathjax.json 2>/dev/null) ; \
	MATHJAX_SRI=$$(awk -F'"' '/"integrity"/{print $$4; exit}' \
		"$(MATH_DEMO_SPHINX)"/mathjax.mathjax.json 2>/dev/null) ; \
	EXTRA= ; \
	if [ -n "$$MATHJAX_SRC" ]; then EXTRA="--mathjax-src $$MATHJAX_SRC" ; fi ; \
	if [ -n "$$MATHJAX_SRI" ]; then EXTRA="$$EXTRA --mathjax-integrity $$MATHJAX_SRI" ; fi ; \
	cd src && cargo run -q --example math_demo -p myst-md-rs -- \
		-o "$(MATH_DEMO_MYST)" $$EXTRA \
		"$(MATH_DEMO_MYST)"/input.md
	@for f in ratex.html mathjax.html imgmath.html; do \
		test -e "$(MATH_DEMO_MYST)"/$$f && \
			echo "  -> $(MATH_DEMO_MYST)/$$f" || \
			(echo "FAIL: missing $$f" && exit 1); \
	done
	@grep -q 'data-renderer="ratex"' "$(MATH_DEMO_MYST)"/ratex.html
	@grep -q 'class="math notranslate' "$(MATH_DEMO_MYST)"/mathjax.html
	@grep -q '<script defer src=' "$(MATH_DEMO_MYST)"/mathjax.html
	@grep -q 'data:image/svg+xml;base64' "$(MATH_DEMO_MYST)"/imgmath.html

# --- sphinxdocrs: read conf.py + pick math renderer ------------------
demo-math-sphinx:
	mkdir -p "$(MATH_DEMO_SPHINX)/mathjax" "$(MATH_DEMO_SPHINX)/imgmath" \
		"$(MATH_DEMO_SPHINX)/ratex"
	printf '%s\n' \
		"extensions = ['sphinx.ext.mathjax']" \
		"mathjax_path = 'https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js'" \
		"mathjax_options = {'async': 'async'}" \
		> "$(MATH_DEMO_SPHINX)"/mathjax/conf.py
	printf '%s\n' \
		"extensions = ['sphinx.ext.imgmath']" \
		"imgmath_image_format = 'svg'" \
		"imgmath_latex = '/usr/bin/latex'" \
		> "$(MATH_DEMO_SPHINX)"/imgmath/conf.py
	printf '%s\n' \
		"import sys, os" \
		"sys.path.insert(0, os.path.abspath('.'))" \
		"extensions = ['dsport.ext.ratex']" \
		> "$(MATH_DEMO_SPHINX)"/ratex/conf.py
	mkdir -p "$(MATH_DEMO_SPHINX)"/ratex/dsport/ext
	touch "$(MATH_DEMO_SPHINX)"/ratex/dsport/__init__.py "$(MATH_DEMO_SPHINX)"/ratex/dsport/ext/__init__.py
	echo "def setup(app): pass" > "$(MATH_DEMO_SPHINX)"/ratex/dsport/ext/ratex.py
	printf '%s\n' \
		'Math demo' \
		'=========' \
		'' \
		'Inline: :math:`E = mc^2` in a paragraph.' \
		'' \
		'.. math::' \
		'' \
		'   \int_0^1 x^2 \, dx = \frac{1}{3}' \
		'' \
		> "$(MATH_DEMO_SPHINX)"/ratex/index.rst
	cd src && cargo run -q --example read_conf -p sphinxdocrs -- \
		-o "$(MATH_DEMO_SPHINX)" \
		"$(MATH_DEMO_SPHINX)"/mathjax/conf.py \
		"$(MATH_DEMO_SPHINX)"/imgmath/conf.py \
		"$(MATH_DEMO_SPHINX)"/ratex/conf.py
	@grep -q "effective_math_renderer=mathjax" "$(MATH_DEMO_SPHINX)"/report.txt
	@grep -q "effective_math_renderer=imgmath" "$(MATH_DEMO_SPHINX)"/report.txt
	@grep -q "effective_math_renderer=ratex"   "$(MATH_DEMO_SPHINX)"/report.txt
	cd src/sphinxdocrs && cargo run -q -p sphinxdocrs --bin \
		sphinx-build-rs -- "$(MATH_DEMO_SPHINX)"/ratex "$(MATH_DEMO_SPHINX)"/html
	@echo "  -> $(MATH_DEMO_SPHINX)/report.txt"
	@echo "  -> $(MATH_DEMO_SPHINX)/html"

# --- sphinxdocrs: fetch+cache MathJax bundle & compute SRI integrity -
# Uses Python stdlib (urllib.request + hashlib + base64) under the hood
# so no extra Rust crates are pulled in. Output:
#
#   $(MATH_DEMO_SPHINX)/mathjax.mathjax.json   <-- {url,path,algo,integrity}
#   $(MATH_DEMO_SPHINX)/mathjax.mathjax.html   <-- <script defer ... integrity="...">
#   $(MATH_DEMO_SPHINX)/_cache/<sha>/<file>    <-- cached bundle bytes
#
# These match the Sphinx 1.8+ `app.add_js_file(filename, integrity=...)`
# API (note: `add_javascript` / `add_stylesheet` were the pre-1.8 names
# and were removed in Sphinx 4.0; use `add_js_file` / `add_css_file`).
demo-math-fetch-mathjax: demo-math-sphinx
	cd src && cargo run -q --example fetch_mathjax -p sphinxdocrs -- \
		-o "$(MATH_DEMO_SPHINX)" \
		"$(MATH_DEMO_SPHINX)"/mathjax/conf.py
	@test -e "$(MATH_DEMO_SPHINX)"/mathjax.mathjax.json
	@grep -q '"integrity": "sha384-' "$(MATH_DEMO_SPHINX)"/mathjax.mathjax.json
	@echo "  -> $(MATH_DEMO_SPHINX)/mathjax.mathjax.{json,html}"

# ========== CLEAN ==========
clean:
	cd src && cargo clean

# ========== CLONE UPSTREAM ==========

CLONE_MY_REPOS_INSTEAD ?= 1
CLONE_MY_REPOS_PREFIX ?= https://github.com/westurner

ifeq ($(CLONE_MY_REPOS_INSTEAD),1)
DOCUTILS_REPO ?= $(CLONE_MY_REPOS_PREFIX)/docutils
PYGMENTS_REPO ?= $(CLONE_MY_REPOS_PREFIX)/pygments
SPHINX_REPO ?= $(CLONE_MY_REPOS_PREFIX)/sphinx
RATEX_REPO ?= $(CLONE_MY_REPOS_PREFIX)/RaTeX
MYST_PARSER_REPO ?= $(CLONE_MY_REPOS_PREFIX)/MyST-Parser
MARKDOWN_IT_REPO ?= $(CLONE_MY_REPOS_PREFIX)/markdown-it-py
MINIJINJA_REPO ?= $(CLONE_MY_REPOS_PREFIX)/minijinja
JINJA2_REPO ?= $(CLONE_MY_REPOS_PREFIX)/jinja2
else
DOCUTILS_REPO ?= https://github.com/docutils/docutils
PYGMENTS_REPO ?= https://github.com/pygments/pygments
SPHINX_REPO ?= https://github.com/sphinx-doc/sphinx
RATEX_REPO ?= https://github.com/erweixin/RaTeX
MYST_PARSER_REPO ?= https://github.com/executablebooks/MyST-Parser
MARKDOWN_IT_REPO ?= https://github.com/executablebooks/markdown-it-py
MINIJINJA_REPO ?= https://github.com/mitsuhiko/minijinja
JINJA2_REPO ?= https://github.com/pallets/jinja2
endif

# DOCUTILS_REPO ?=    https://github.com/westurner/docutils
# PYGMENTS_REPO ?=    https://github.com/westurner/pygments
# SPHINX_REPO ?=      https://github.com/westurner/sphinx
# RATEX_REPO ?=       https://github.com/westurner/RaTeX
# MYST_PARSER_REPO ?= https://github.com/westurner/MyST-Parser
# MARKDOWN_IT_REPO ?= https://github.com/westurner/markdown-it-py
# MINIJINJA_REPO ?=   https://github.com/westurner/minijinja
# JINJA2_REPO ?=      https://github.com/westurner/jinja2

clone-upstream: clone-docutils clone-pygments clone-sphinx clone-ratex \
	clone-myst-parser \
	clone-markdown-it-py \
	clone-minijinja \
	clone-jinja2

clone-docutils:
	@if [ ! -d "src/docutils" ]; then \
		git clone $(DOCUTILS_REPO) src/docutils; \
	else \
		echo "src/docutils already exists."; \
	fi

clone-pygments:
	@if [ ! -d "src/pygments" ]; then \
		git clone $(PYGMENTS_REPO) src/pygments; \
	else \
		echo "src/pygments already exists."; \
	fi

clone-sphinx:
	@if [ ! -d "src/sphinx" ]; then \
		git clone $(SPHINX_REPO) src/sphinx; \
	else \
		echo "src/sphinx already exists."; \
	fi

RATEX_PATH ?= src/RaTeX
clone-ratex:
	@if [ ! -d "$(RATEX_PATH)" ]; then \
		git clone $(RATEX_REPO) $(RATEX_PATH); \
	else \
		echo "$(RATEX_PATH) already exists."; \
	fi


MYST_PARSER_PATH ?= src/MyST-Parser
clone-myst-parser:
	@if [ ! -d "$(MYST_PARSER_PATH)" ]; then \
		git clone $(MYST_PARSER_REPO) $(MYST_PARSER_PATH); \
	else \
		echo "$(MYST_PARSER_PATH) already exists."; \
	fi


MARKDOWN_IT_PATH ?= src/markdown-it-py
clone-markdown-it-py:
	@if [ ! -d "$(MARKDOWN_IT_PATH)" ]; then \
		git clone $(MARKDOWN_IT_REPO) $(MARKDOWN_IT_PATH); \
	else \
		echo "$(MARKDOWN_IT_PATH) already exists."; \
	fi


MINIJINJA_PATH ?= src/minijinja
clone-minijinja:
	@if [ ! -d "$(MINIJINJA_PATH)" ]; then \
		git clone $(MINIJINJA_REPO) $(MINIJINJA_PATH); \
	else \
		echo "$(MINIJINJA_PATH) already exists."; \
	fi


JINJA2_PATH ?= src/jinja2
clone-jinja2:
	@if [ ! -d "$(JINJA2_PATH)" ]; then \
		git clone $(JINJA2_REPO) $(JINJA2_PATH); \
	else \
		echo "$(JINJA2_PATH) already exists."; \
	fi