# Makefile for rspostgresmcp Rust project

.PHONY: build test clippy fmt all

all: fmt clippy build test

build:
	cargo build

test:
	cargo test --all-features

clippy:
	cargo clippy --all-features -- -D warnings

fmt:
	cargo fmt --all
