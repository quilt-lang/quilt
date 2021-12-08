.ONESHELL:
.PHONY: check clean build fmt clippy test

check: build fmt clippy test

clean:
	cargo clean

build:
	cargo build

fmt:
	cargo fmt -- --check --verbose

clippy:
	cargo clippy --all-targets -- --forbid warnings

test:
	cargo test -- --nocapture
