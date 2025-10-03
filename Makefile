.PHONY: help dev build test clean install-dev dev-container format dev-daemon build-daemon

help:
	@echo "NetVisor Development Commands"
	@echo ""
	@echo "  make dev            - Start development environment (server + ui)"
	@echo "  make dev-container  - Start containerized development environment using docker-compose.dev.yml (server + ui)"
	@echo "  make dev-daemon     - Start daemon"
	@echo "  make build          - Build production Docker images (server + ui)"
	@echo "  make build-daemon   - Build daemon Docker image"
	@echo "  make build-all      - Build all Docker images (server + ui + daemon)"
	@echo "  make test           - Run all tests"
	@echo "  make lint           - Run all linters"
	@echo "  make format         - Format all code"
	@echo "  make clean          - Clean build artifacts and containers"
	@echo "  make install-dev    - Install local development dependencies"

dev:
	cd server && cargo run --bin server
	cd ui && npm run dev

dev-daemon:
	cd server && cargo run --bin daemon -- --server-target 127.0.0.1 --server-port 60072

dev-container:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml up

build:
	docker compose build

build-daemon:
	@echo "Building daemon Docker image..."
	docker build -f server/Dockerfile.daemon -t mayanayza/netvisor-daemon:latest server/
	@echo "✓ Daemon image built: mayanayza/netvisor-daemon:latest"

build-all: build build-daemon
	@echo "✓ All images built"

test:
	@echo "Testing Server..."
	cd server && cargo test --bin server
	@echo "Testing Daemon..."
	cd server && cargo test --bin daemon

format:
	@echo "Formatting Server..."
	cd server && cargo fmt
	@echo "Formatting UI..."
	cd ui && npm run format
	@echo "All code formatted!"

lint:
	@echo "Linting Server..."
	cd server && cargo fmt -- --check && cargo clippy --bin server -- -D warnings
	@echo "Linting Daemon..."
	cd server && cargo clippy --bin daemon -- -D warnings
	@echo "Linting UI..."
	cd ui && npm run lint && npm run format -- --check && npm run check

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