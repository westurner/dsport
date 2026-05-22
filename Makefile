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