# NetVisor

## Quick Start

### 1. Start the Server
`docker compose up -d`

### 2. Install the Daemon  
`curl -sSL https://install.netvisor.com | bash`

### 3. Connect Daemon to Server
`netvisor-daemon --server-target YOUR_SERVER_IP --server-port 60072`