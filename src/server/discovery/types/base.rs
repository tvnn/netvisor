use strum_macros::EnumIter;
use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive, Clone, Copy, EnumIter, PartialEq, Eq)]
#[repr(u16)]
pub enum DiscoveryPort {
    Ssh = 22,
    Dns = 53,
    Http = 80,
    Https = 443,
    IpsecIke = 500,
    OpenVpn = 1194,
    Pptp = 1723,
    IpsecNat = 4500,
    WireGuard = 51820,
    Snmp = 161,
    SnmpTrap = 162,
    Rdp = 3389,
    HttpAlt = 8080,
    HttpsAlt = 8443,
    Telnet = 23,
}