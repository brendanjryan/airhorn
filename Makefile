.PHONY: check test build fix clean doc run

check:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo test
	cargo build --release

test:
	cargo test --verbose

build:
	cargo build --release

fix:
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged

clean:
	cargo clean

doc:
	cargo doc --open

run:
	cargo run --release
