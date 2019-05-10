.PHONY: clean-build docs clean

help:
	@echo "all - local build including docs"
	@echo "clean - remove all build, test, coverage and Rust artifacts"
	@echo "clean-build - remove build artifacts"
	@echo "clean-doc - remove test and coverage artifacts"
	@echo "lint - check style with rustfmt"
	@echo "format - automatically correct style with rustfmt"
	@echo "clip - more extensive linting using clippy"
	@echo "test - build and run unit tests"
	@echo "docs - generate rust documentation"
	@echo "docs-watch - regenerate rust documentation automatically"

# NOTE: docs requires rustdoc.  docs-watch is linux specific, requires
# inotifywait (from inotify-tools) be installed

all: docs build

clean: clean-build clean-doc

clean-build:
	cargo clean

clean-doc:
	rm -fr doc/

lint:
	cargo check

clip:
	cargo clippy

format:
	cargo fmt

test: build
	cargo test

build:
	cargo build

docs:
	rm -f docs
	rustdoc src/*.rs

docs-watch:
	while inotifywait src/*.rs ; do make docs ; done
