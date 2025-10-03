use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use local_ip_address::local_ip;
use pnet::datalink::NetworkInterface;
use std::net::IpAddr;

pub trait NetworkUtils {
    fn new() -> Self;

    fn get_own_interfaces(&self) -> Vec<NetworkInterface> {
        pnet::datalink::interfaces()
    }

    fn get_own_ip_address(&self) -> Result<IpAddr, Error> {
        local_ip().map_err(|e| anyhow!("Failed to get local IP address: {}", e))
    }
}

pub struct ServerNetworkUtils {}

impl NetworkUtils for ServerNetworkUtils {
    fn new() -> Self {
        Self {}
    }
}
