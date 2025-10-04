# NetVisor

**Automatically discover and visually document network topology.**

NetVisor scans your network, identifies hosts and services, and generates an interactive visualization showing how everything connects, letting you easily create and maintain network documentation.

<p align="center">
  <img src="./media/example_visualization.png" width="1200" alt="Example Visualization">
</p>

## Table of Contents

- [Installation](#installation)
- [Network Discovery](#network-discovery)
- [Network Organization](#network-organization)
- [Topology Visualization](#topology-visualization)
- [Configuration](#configuration)
  - [Daemon Configuration](#daemon-configuration)
  - [Server Configuration](#server-configuration)
- [Uninstall Daemon](#uninstall-daemon)
- [FAQ](#faq)

## Installation

### 0. Install Requirements

#### Daemon
- **Linux**: Docker with host networking support, OR binary installation
- **Mac/Windows**: Binary installation only (Docker Desktop does not support host networking)

#### Server (Docker - Recommended)
- Docker
- Docker Compose

#### Server (Building from source)
- Rust 1.90 or later
- Node.js 20 or later

### 1. Start the Server

```bash
curl -O https://raw.githubusercontent.com/mayanayza/netvisor/refs/heads/main/docker-compose.yml && docker compose up -d
```

The server collects data from the daemon(s) and generates the topology visualization. The server can support multiple daemon instances in case you want to collect data from the perspective of multiple hosts on your network (ie - mapping out vlans).

### 2. Install the Daemon  

The daemon scans the network and requires access to host network interfaces. 

**Linux**: Can run in Docker with `--network host` and `--privileged` flags, or as a binary directly on the host.

**Mac/Windows**: Docker Desktop does not support host networking, so the daemon must run as a binary directly on the host (not containerized).

#### Linux (Docker - Recommended)

Make sure to replace YOUR_SERVER_IP

```bash
curl -O https://raw.githubusercontent.com/mayanayza/netvisor/refs/heads/main/docker-compose.daemon.yml && \
NETVISOR_SERVER_TARGET=YOUR_SERVER_IP docker compose -f docker-compose.daemon.yml up -d
```

#### Linux (Binary)

```bash
curl -sSL https://raw.githubusercontent.com/mayanayza/netvisor/refs/heads/main/install.sh | bash
```

#### Mac / Windows (Binary)

```bash
curl -sSL https://raw.githubusercontent.com/mayanayza/netvisor/refs/heads/main/install.sh | bash
```

### 3. Connect Daemon to Server

If you installed the daemon as a binary (not using Docker), run:

```bash
netvisor-daemon --server-target YOUR_SERVER_IP --server-port 60072
```

If you used the Docker command above, the daemon is already connected and running.

## Network Discovery

Once you connect a daemon to the server, a host will be created with a discovery button. Click to start discovery.

<p align="center">
  <img src="./media/discovery_host.png" width="400" alt="Run Discovery">
</p>

The NetVisor Daemon discovers hosts on your network by scanning all IPv4 addresses on subnets that the host it runs on has a network interface with. For each IP on the network, the daemon:

- **Detects services**: Uses rule based pattern matching to recognize running services using open ports, HTTP endpoints responses, and other data from the host.
- **Maps interfaces**: Detects host network interfaces and their subnet membership

Discovery creates hosts with their interfaces, services, and subnet relationships.

Discovery can take 5-10+ minutes depending on how many subnets the daemon's host is connected and the network mask for those subnets, as it needs to scan every IP address on the subnet.

## Network Organization

### Consolidating Hosts

The discovery process does its best to merge duplicate hosts, but this isn't always possible. You can consolidate hosts that actually represent multiple interfaces or services on the same host using the Consolidate feature. This migrates all ports, interfaces, and services to a single host record.

<p align="center">
  <img src="./media/consolidate_host.png" width="400" alt="Consolidate Host">
</p>

### Subnets

Subnets organize your network into logical segments of hosts. Subnets are automatically created during discovery.

<p align="center">
  <img src="./media/subnet.png" width="400" alt="Subnet">
</p>

**Infrastructure Services**: The discovery process identifies hosts providing infrastructure services (DNS, gateway, reverse proxy) on a subnet and flags them. You can edit this in the subnet if needed.

**Organizational Subnets**: Subnets with 0.0.0.0/0 CIDR can be used to organize external resources (like internet services or remote hosts) that aren't on your local network but you want to include in your topology.

### Groups

Groups let you visualize logical connections between services, such as a web app talking to its database, or representing network paths between different parts of your infrastructure. You'll need to create groups manually.

<p align="center">
  <img src="./media/group.png" width="400" alt="Group">
</p>

## Topology Visualization

The topology auto-generates from your hosts, subnets, and service groups, creating living documentation that updates as your network changes.

You can customize the visualization:
- **Anchor points**: Click edges to change where they connect to nodes
- **Subnet sizing**: Drag subnet boundaries to resize
- **Layout**: Drag hosts and subnets to organize your topology

## Configuration

Both the server and daemon support multiple configuration methods with the following priority order (highest to lowest):

1. **Command-line arguments** (highest priority)
2. **Environment variables**
3. **Configuration file** (daemon only)
4. **Default values** (lowest priority)

### Daemon Configuration

The daemon supports the following configuration options:

| Parameter | CLI Flag | Environment Variable | Config File | Default | Description |
|-----------|----------|---------------------|-------------|---------|-------------|
| Server Target | `--server-target` | `NETVISOR_SERVER_TARGET` | `server_target` | `None` | IP address or hostname of the NetVisor server (required) |
| Server Port | `--server-port` | `NETVISOR_SERVER_PORT` | `server_port` | `60072` | Port the NetVisor server is listening on |
| Daemon Port | `--port` or `-p` | `NETVISOR_PORT` | `port` | `60073` | Port for the daemon to listen on |
| Bind Address | `--bind-address` | `NETVISOR_BIND_ADDRESS` | `bind_address` | `0.0.0.0` | IP address to bind the daemon to (useful for restricting to specific interfaces when using host networking) |
| Daemon Name | `--name` | `NETVISOR_NAME` | `name` | `netvisor-daemon` | Human-readable name for this daemon instance |
| Log Level | `--log-level` | `NETVISOR_LOG_LEVEL` | `log_level` | `info` | Logging verbosity (`trace`, `debug`, `info`, `warn`, `error`) |
| Heartbeat Interval | `--heartbeat-interval` | `NETVISOR_HEARTBEAT_INTERVAL` | `heartbeat_interval` | `30` | Seconds between heartbeat updates to the server |

#### Configuration File Location

The daemon automatically creates and maintains a configuration file at:
- **Linux**: `~/.config/netvisor/daemon/config.json`
- **macOS**: `~/Library/Application Support/com.netvisor.daemon/config.json`
- **Windows**: `%APPDATA%\netvisor\daemon\config.json`

The configuration file persists runtime state (daemon ID, host ID, last heartbeat) alongside your configured settings.

### Server Configuration

The server supports the following configuration options:

| Parameter | CLI Flag | Environment Variable | Default | Description |
|-----------|----------|---------------------|---------|-------------|
| Port | `--port` | `NETVISOR_PORT` | `60072` | Port for the server to listen on |
| Log Level | `--log-level` | `NETVISOR_LOG_LEVEL` | `info` | Logging verbosity (`trace`, `debug`, `info`, `warn`, `error`) |
| Database Path | `--database-path` | `NETVISOR_DATABASE_PATH` | `./netvisor.db` | Path to the SQLite database file |
| Web Assets Path | `--web-external-path` | `NETVISOR_WEB_EXTERNAL_PATH` | `None` | Path to static web UI assets (used in Docker builds) |

## Uninstall Daemon

#### Linux (Docker)

```bash
docker stop netvisor-daemon
docker rm netvisor-daemon
docker volume rm netvisor_daemon-config  # Optional: remove persisted config
```

#### Linux (Binary)

```bash
sudo rm /usr/local/bin/netvisor-daemon
rm -rf ~/.config/netvisor/daemon
```

#### Mac (Binary)

```bash
sudo rm /usr/local/bin/netvisor-daemon
rm -rf ~/Library/Application\ Support/com.netvisor.daemon
```

## FAQ

### Are VLANs supported?

Yes, you can collect information from hosts on multiple vlans by using multiple Daemon deployments. Simply install a daemon on a host that interfaces with each vlan you want to include in the visualization. You will probably need to use the [Consolidate](#consolidating-hosts) feature to combine IPs which are detected as distinct hosts.

### Is IPv6 supported?

Not currently. Future plans to support IPv6 will focus on collecting a host's IPv6 address during discovery and/or allowing manual entry of it during editing. Scanning the entire IPv6 space of a discovered subnet will never be supported as it will take too long to do so.

### How do I change the daemon configuration after installation?

You can change daemon configuration in three ways:

1. **Pass new CLI arguments** when starting the daemon
2. **Set environment variables** before starting the daemon
3. **Edit the config file** at `~/.config/netvisor/daemon/config.json` (Linux) or equivalent location for your OS

Changes take effect the next time the daemon starts. The daemon must be restarted for configuration changes to apply.
