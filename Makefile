setup: install-extism
	rustup target add wasm32-unknown-unknown
	cargo install wasm-opt

# Build with optimizations and minimal size
build: setup
	cargo build --release --features extism --target wasm32-unknown-unknown
	wasm-opt -Oz -o ./target/wasm32-unknown-unknown/release/diff_html.wasm ./target/wasm32-unknown-unknown/release/diff_html_rs.wasm

# Clean build artifacts
clean:
	cargo clean

# Install Extism CLI if not present
install-extism:
	which extism || curl -fsSL https://extism.org/install.sh | bash

# Clean build artifacts
clean:
	cargo clean
	rm -f diff_html_rs.wasm

.PHONY: build clean install-extism test
