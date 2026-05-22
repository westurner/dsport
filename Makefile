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
else
DOCUTILS_REPO ?= https://github.com/docutils/docutils
PYGMENTS_REPO ?= https://github.com/pygments/pygments
SPHINX_REPO ?= https://github.com/sphinx-doc/sphinx
RATEX_REPO ?= https://github.com/erweixin/RaTeX
endif
# DOCUTILS_REPO ?= https://github.com/westurner/docutils
# PYGMENTS_REPO ?= https://github.com/westurner/pygments
# SPHINX_REPO ?= https://github.com/westurner/sphinx
# RATEX_REPO ?= https://github.com/westurner/RaTeX

clone-upstream: clone-docutils clone-pygments clone-sphinx clone-ratex

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

clone-ratex:
	@if [ ! -d "src/RaTeX" ]; then \
		git clone $(RATEX_REPO) src/RaTeX; \
	else \
		echo "src/RaTeX already exists."; \
	fi
