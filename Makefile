run:
	cargo run --bin cli

build:
	cargo build --release

fmt:
	cargo fmt 

lint:
	cargo clippy

publish:
	cargo publish
