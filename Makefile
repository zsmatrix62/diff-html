setup: install-extism
	rustup target add wasm32-unknown-unknown
	cargo install wasm-opt

# Build with optimizations and minimal size
build: 
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

.PHONY: build clean install-extism test tag

# Create git tag from Cargo.toml version and push to remote
tag:
	@version=$$(grep -m 1 '^version =' Cargo.toml | awk -F'"' '{print $$2}') && \
	git tag -a v$$version -m "Version $$version" && \
	git push origin v$$version
