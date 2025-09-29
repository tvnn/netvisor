.PHONY: help dev build test clean install-dev

help:
	@echo "NetVisor Development Commands"
	@echo ""
	@echo "  make dev          - Start development environment"
	@echo "  make dev          - Start containerized development environment using docker-compose.dev.yml"
	@echo "  make build        - Build production Docker images"
	@echo "  make test         - Run all tests"
	@echo "  make lint         - Run all linters"
	@echo "  make clean        - Clean build artifacts and containers"
	@echo "  make install-dev  - Install local development dependencies"

dev:
	cd server && cargo run
	cd ui && npm run dev

dev-container:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml up

build:
	docker compose build

test:
	@echo "Testing Server..."
	cd server && cargo test

lint:
	@echo "Linting Server..."
	cd server && cargo fmt -- --check && cargo clippy -- -D warnings
	@echo "Linting UI..."
	cd ui && npm run lint && npm run check

clean:
	docker compose down -v
	cd server && cargo clean
	cd ui && rm -rf node_modules dist build .svelte-kit

install-dev:
	@echo "Installing Rust toolchain..."
	rustup install stable
	rustup component add rustfmt clippy
	@echo "Installing Node.js dependencies..."
	cd ui && npm install
	@echo "Installing cargo-watch for hot reload..."
	cargo install cargo-watch
	@echo "Development dependencies installed!"