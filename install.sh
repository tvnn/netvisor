#!/bin/bash
set -e

BINARY_URL="https://github.com/mayanayza/netvisor-server/releases/latest/download/netvisor-daemon-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)"

echo "Downloading NetVisor daemon..."
curl -L "$BINARY_URL" -o netvisor-daemon
chmod +x netvisor-daemon

echo "Installing to /usr/local/bin (may require sudo)..."
if [ -w "/usr/local/bin" ]; then
    mv netvisor-daemon /usr/local/bin/
else
    sudo mv netvisor-daemon /usr/local/bin/
fi

echo "✓ NetVisor daemon installed!"
echo "Run with: netvisor-daemon --server-ip YOUR_SERVER_IP --server-port 60072"