.PHONY: pre-check lint test

pre-check: lint test

lint:
	cargo check --workspace --all-features
	cargo clippy --workspace --all-features --all-targets -- -D warnings
	cargo fmt --all
test:
	cargo test --workspace --all-features
