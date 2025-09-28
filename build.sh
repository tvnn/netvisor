#!/bin/bash
VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: ./release.sh v1.0.0"
    exit 1
fi

echo "🚀 Creating NetVisor release ${VERSION}"

# Build daemon with platform-specific name
cargo build --release --bin daemon
mkdir -p releases
cp target/release/daemon releases/netvisor-daemon-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)

# Build and push Docker images
docker build -t mayanayza/netvisor-ui:${VERSION} ../netvisor-ui
docker build -t mayanayza/netvisor-server:${VERSION} .

docker push mayanayza/netvisor-ui:${VERSION}
docker push mayanayza/netvisor-server:${VERSION}

# Also tag as latest
docker tag mayanayza/netvisor-ui:${VERSION} mayanayza/netvisor-ui:latest
docker tag mayanayza/netvisor-server:${VERSION} mayanayza/netvisor-server:latest
docker push mayanayza/netvisor-ui:latest
docker push mayanayza/netvisor-server:latest

echo "✅ Release ${VERSION} ready"
echo "📦 Upload this binary to GitHub release: releases/netvisor-daemon-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)"