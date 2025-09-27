#!/bin/bash
set -e

echo "Building NetVisor daemon..."

# Build for your current platform
cargo build --release --bin daemon

# Copy to a release directory
mkdir -p releases
cp target/release/daemon releases/netvisor-daemon-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)

echo "Binary ready: releases/netvisor-daemon-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)"