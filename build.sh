#!/bin/bash
VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: ./release.sh v1.0.0"
    exit 1
fi

echo "Creating NetVisor release ${VERSION}"

# Setup Docker buildx
docker buildx create --name multiarch --use --bootstrap 2>/dev/null || docker buildx use multiarch

# Build and push Docker images
docker buildx build --platform linux/amd64,linux/arm64 \
  -t mayanayza/netvisor-ui:${VERSION} \
  -t mayanayza/netvisor-ui:latest \
  --push ../netvisor-ui

docker buildx build --platform linux/amd64,linux/arm64 \
  -t mayanayza/netvisor-server:${VERSION} \
  -t mayanayza/netvisor-server:latest \
  --push .

echo "Docker images pushed. Now create GitHub release ${VERSION} and Actions will build binaries."