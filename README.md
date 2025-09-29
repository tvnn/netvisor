# NetVisor

## Quick Start

### 1. Start the Server

`curl -O https://raw.githubusercontent.com/mayanayza/netvisor-server/refs/heads/main/docker-compose.yml && docker compose up -d`

### 2. Install the Daemon  
`curl -sSL https://raw.githubusercontent.com/mayanayza/netvisor-server/refs/heads/main/install.sh | bash`

### 3. Connect Daemon to Server
`netvisor-daemon --server-target YOUR_SERVER_IP --server-port 60072`