.PHONY: help build test clean install-dev dev-server dev-daemon dev-container dev-container-rebuild dev-container-rebuild-clean format

help:
	@echo "NetVisor Development Commands"
	@echo ""
	@echo "  make dev-server     - Start development environment (server + ui)"
	@echo "  make dev-daemon     - Start daemon"
	@echo "  make dev-container  - Start containerized development environment using docker-compose.dev.yml (server + ui + daemon)"
	@echo "  make dev-container-rebuild  - Rebuild and start containerized dev environment"
	@echo "  make dev-container-rebuild-clean  - Rebuild, clean, and start containerized dev environment"
	@echo "  make build          - Build production Docker images (server + ui + daemon)"
	@echo "  make test           - Run all tests"
	@echo "  make lint           - Run all linters"
	@echo "  make format         - Format all code"
	@echo "  make clean          - Clean build artifacts and containers"
	@echo "  make install-dev    - Install local development dependencies"

dev-server:
	cd backend && cargo run --bin server
	cd ui && npm run dev

dev-daemon:
	cd backend && cargo run --bin daemon -- --server-target 127.0.0.1 --server-port 60072

dev-container:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml up

dev-container-rebuild:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml up --build --force-recreate

dev-container-rebuild-clean:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml build --no-cache
	docker compose -f docker-compose.yml -f docker-compose.dev.yml up

build:
	@echo "Building server Docker image..."
	docker compose build
	@echo "Building daemon Docker image..."
	docker build -f backend/Dockerfile.daemon -t mayanayza/netvisor-daemon:latest server/
	@echo "✓ Daemon image built: mayanayza/netvisor-daemon:latest"

test:
	rm -rf ./data/*
	cd backend && cargo test

format:
	@echo "Formatting Server..."
	cd backend && cargo fmt
	@echo "Formatting UI..."
	cd ui && npm run format
	@echo "All code formatted!"

lint:
	@echo "Linting Server..."
	cd backend && cargo fmt -- --check && cargo clippy --bin server -- -D warnings
	@echo "Linting Daemon..."
	cd backend && cargo clippy --bin daemon -- -D warnings
	@echo "Linting UI..."
	cd ui && npm run lint && npm run format -- --check && npm run check

clean:
	docker compose down -v
	cd backend && cargo clean
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