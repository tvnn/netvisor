use anyhow::Error;
use dhcproto::v4::{self, Decodable, Encodable, Encoder, Message, MessageType};
use rand::{Rng, SeedableRng};
use rsntp::AsyncSntpClient;
use snmp2::{AsyncSession, Oid};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

// Fixed: Actually wait for response and validate it's meaningful
pub async fn send_udp_probe(ip: IpAddr, port: u16) -> Result<Option<u16>, Error> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let target = SocketAddr::new(ip, port);

    // Send a probe packet
    socket.send_to(&[0x00], target).await?;

    // Wait for ANY response (this was missing before)
    let mut buf = [0u8; 1024];
    match timeout(Duration::from_millis(1000), socket.recv_from(&mut buf)).await {
        Ok(Ok((len, _))) if len > 0 => {
            tracing::debug!("✅ Got response from {}:{}", ip, port);
            Ok(Some(port))
        }
        _ => {
            tracing::debug!("❌ No response from {}:{}", ip, port);
            Ok(None)
        }
    }
}

// Fixed: Use simpler DNS resolver that doesn't have API issues
pub async fn test_dns_service(ip: IpAddr) -> Result<Option<u16>, Error> {
    // Use the simpler approach - create resolver with custom config directly
    let mut config = ResolverConfig::new();
    let name_server = NameServerConfig::new(SocketAddr::new(ip, 53), Protocol::Udp);
    config.add_name_server(name_server);

    let test_resolver = TokioAsyncResolver::tokio(config, ResolverOpts::default());

    match timeout(
        Duration::from_millis(2000),
        test_resolver.lookup_ip("google.com"),
    )
    .await
    {
        Ok(Ok(_)) => {
            tracing::debug!("✅ DNS server responding at {}:53", ip);
            Ok(Some(53))
        }
        _ => {
            tracing::debug!("❌ DNS server not responding at {}:53", ip);
            Ok(None)
        }
    }
}

pub async fn test_ntp_service(ip: IpAddr) -> Result<Option<u16>, Error> {
    let client = AsyncSntpClient::new();
    let server_addr = format!("{}:123", ip);

    match timeout(
        Duration::from_millis(2000),
        client.synchronize(&server_addr),
    )
    .await
    {
        Ok(Ok(result)) => {
            // Validate that we got a meaningful time response
            if let Ok(datetime) = result.datetime().unix_timestamp() {
                if datetime > Duration::from_secs(0) {
                    // Sanity check for valid timestamp
                    tracing::debug!(
                        "✅ NTP server responding at {}:123 with time {}",
                        ip,
                        datetime.as_millis()
                    );
                    Ok(Some(123))
                } else {
                    tracing::debug!("❌ Invalid NTP response from {}:123", ip);
                    Ok(None)
                }
            } else {
                tracing::debug!("❌ Invalid NTP response from {}:123", ip);
                Ok(None)
            }
        }
        Ok(Err(e)) => {
            tracing::debug!("❌ NTP error from {}:123 - {}", ip, e);
            Ok(None)
        }
        Err(_) => {
            tracing::debug!("❌ NTP timeout from {}:123", ip);
            Ok(None)
        }
    }
}

// Fixed: Add proper error handling and response validation
pub async fn test_snmp_service(ip: IpAddr) -> Result<Option<u16>, Error> {
    let target = format!("{}:161", ip);
    let community = b"public";

    match AsyncSession::new_v2c(&target, community, 0).await {
        Ok(mut session) => {
            let sys_descr_oid = Oid::from(&[1, 3, 6, 1, 2, 1, 1, 1, 0]).unwrap();

            match timeout(Duration::from_millis(2000), session.get(&sys_descr_oid)).await {
                Ok(Ok(mut response)) => {
                    if let Some(_varbind) = response.varbinds.next() {
                        tracing::debug!("✅ SNMP server responding at {}:161", ip);
                        Ok(Some(161))
                    } else {
                        tracing::debug!("❌ Empty SNMP response from {}:161", ip);
                        Ok(None)
                    }
                }
                Ok(Err(e)) => {
                    tracing::debug!("❌ SNMP error from {}:161 - {}", ip, e);
                    Ok(None)
                }
                Err(_) => {
                    tracing::debug!("❌ SNMP timeout from {}:161", ip);
                    Ok(None)
                }
            }
        }
        Err(e) => {
            tracing::debug!("❌ SNMP session creation failed for {}:161 - {}", ip, e);
            Ok(None)
        }
    }
}

/// Test if a host is running a DHCP server on port 67
pub async fn test_dhcp_service(ip: IpAddr) -> Result<Option<u16>, Error> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let target = SocketAddr::new(ip, 67);

    // Create a minimal DHCP DISCOVER message
    let mut rng = rand::rngs::StdRng::from_os_rng();
    let mac_addr: [u8; 6] = rng.random();
    let transaction_id = rng.random::<u32>();

    let mut msg = Message::default();
    msg.set_opcode(v4::Opcode::BootRequest)
        .set_htype(v4::HType::Eth)
        .set_xid(transaction_id)
        .set_flags(v4::Flags::default().set_broadcast())
        .set_chaddr(&mac_addr);

    msg.opts_mut()
        .insert(v4::DhcpOption::MessageType(MessageType::Discover));

    // Encode and send DHCP DISCOVER packet
    let mut buf = Vec::new();
    let mut encoder = Encoder::new(&mut buf);
    msg.encode(&mut encoder)?;
    socket.send_to(&buf, target).await?;

    // Wait for DHCP OFFER response
    let mut response_buf = [0u8; 1500];
    match timeout(
        Duration::from_millis(2000),
        socket.recv_from(&mut response_buf),
    )
    .await
    {
        Ok(Ok((len, _))) if len > 0 => {
            // Try to parse as DHCP message and validate response type
            match Message::decode(&mut dhcproto::Decoder::new(&response_buf[..len])) {
                Ok(response_msg) => {
                    let is_valid_response = response_msg.opts().iter().any(|(_, opt)| {
                        matches!(
                            opt,
                            v4::DhcpOption::MessageType(MessageType::Offer)
                                | v4::DhcpOption::MessageType(MessageType::Ack)
                        )
                    });

                    if is_valid_response {
                        tracing::debug!("✅ DHCP server responding at {}:67", ip);
                        Ok(Some(67))
                    } else {
                        tracing::debug!("❌ Invalid DHCP response from {}:67", ip);
                        Ok(None)
                    }
                }
                Err(_) => {
                    tracing::debug!("❌ Failed to parse DHCP response from {}:67", ip);
                    Ok(None)
                }
            }
        }
        _ => {
            tracing::debug!("❌ DHCP timeout from {}:67", ip);
            Ok(None)
        }
    }
}
