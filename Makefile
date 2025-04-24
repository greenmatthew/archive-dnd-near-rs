.PHONY: help dev build release serve clean clean-all  clippy clippy-pedantic clippy-all install-deps deploy-env

# Default target
help:
	@echo "D&D Helper Leptos Application Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make help        - Show this help message"
	@echo "  make dev         - Run development server with auto-reload"
	@echo "  make build       - Build the project with clippy pedantic warnings (debug)"
	@echo "  make release     - Build the project with clippy pedantic warnings (release)"
	@echo "  make serve       - Run the server without auto-reload"
	@echo "  make csr         - Run client-side rendering via trunk"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make clean-all   - Clean build artifacts and other files"
	@echo "  make clippy      - Run clippy with default settings"
	@echo "  make clippy-pedantic - Run clippy with pedantic settings"
	@echo "  make clippy-all  - Run clippy with all lints enabled"
	@echo "  make deploy-env - Deploy environment"
	@echo "  make install-deps - Install project dependencies"
	

# Development server with auto-reload and clippy checks
dev:
	cargo clippy -- -W clippy::pedantic
	cargo leptos watch

# Build the project (debug mode)
build:
	cargo clippy -- -W clippy::pedantic
	cargo leptos build

# Build for release
release:
	cargo clippy -- -W clippy::pedantic
	cargo leptos build --release

serve:
	@echo "Running server on http://127.0.0.1:3000"
	./target/debug/dnd-near-rs

# Run client-side rendering via trunk (useful for Tauri integration)
csr:
	trunk serve --open --features csr

# Clean build artifacts
clean:
	cargo clean

# Clean build artifacts and other files
clean-all:
	cargo clean
	rm -rf target/site

# Clippy commands
clippy:
	cargo clippy

clippy-pedantic:
	cargo clippy -- -W clippy::pedantic

clippy-all:
	cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery

# Install dependencies
install-deps:
	@echo "Installing dependencies..."
	rustup toolchain install nightly --allow-downgrade
	rustup target add wasm32-unknown-unknown
	cargo install cargo-leptos --locked
	rustup component add clippy
	cargo install cargo-watch
	@echo "Dependencies installed successfully"

# Set environment variables for deployment
deploy-env:
	@echo "export LEPTOS_OUTPUT_NAME=\"dnd-near-rs\"" > .env
	@echo "export LEPTOS_SITE_ROOT=\"site\"" >> .env
	@echo "export LEPTOS_SITE_PKG_DIR=\"pkg\"" >> .env
	@echo "export LEPTOS_SITE_ADDR=\"127.0.0.1:3000\"" >> .env
	@echo "export LEPTOS_RELOAD_PORT=\"3001\"" >> .env
	@echo "Environment variables written to .env file"
	@echo "Source this file before running the server binary on a deployment machine"