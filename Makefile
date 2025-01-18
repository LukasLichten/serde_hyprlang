.phony: build test help

all: test

build:
	cargo build

test:
	cargo test

help:
	@echo "Hyprlang config language parser for rust/serde:"
	@echo "make:       (Currently) runs tests"
	@echo "make build: Builds the crate in debug"
	@echo "make test:  Runs the tests"
	@echo "make help:  Prints this information"
