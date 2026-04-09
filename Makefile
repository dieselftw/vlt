.PHONY: fmt fmt-check lint test check run

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-targets

check: fmt-check lint test

run:
	cargo run -- $(ARGS)
