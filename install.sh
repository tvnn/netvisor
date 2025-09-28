#!/bin/bash
set -e

REPO="mayanayza/netvisor-server"
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
BINARY_NAME="netvisor-daemon-${PLATFORM}-${ARCH}"

echo "📥 Installing NetVisor daemon..."

# Download latest binary
BINARY_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"
curl -L "$BINARY_URL" -o netvisor-daemon
chmod +x netvisor-daemon

# Install to system
echo "🔐 Installing to /usr/local/bin (may require sudo)..."
if [ -w "/usr/local/bin" ]; then
    mv netvisor-daemon /usr/local/bin/
else
    sudo mv netvisor-daemon /usr/local/bin/
fi

echo "✅ NetVisor daemon installed!"
echo ""
echo "Next steps:"
echo "1. Start NetVisor server: docker compose up -d"
echo "2. Run daemon: netvisor-daemon --server-target YOUR_SERVER_IP --server-port 60072"
echo ""
echo "Need help? Visit: https://github.com/${REPO}#readme"