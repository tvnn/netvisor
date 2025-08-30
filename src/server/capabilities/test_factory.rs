use crate::server::{capabilities::types::base::CapabilityTest, nodes::types::criticality::TestCriticality, tests::types::{base::Test, configs::{ConnectivityConfig, DnsLookupConfig, DnsResolutionConfig, ReverseDnsConfig, ServiceHealthConfig, VpnSubnetAccessConfig}}};

pub struct CapabilityTestFactory {}

impl CapabilityTestFactory {
    pub fn connectivity() -> CapabilityTest {
        CapabilityTest {
            test: Test::Connectivity(ConnectivityConfig::default()),
            criticality: TestCriticality::Important,
            enabled: false
        }
    }

    pub fn service_health() -> CapabilityTest {
        CapabilityTest {
            test: Test::ServiceHealth(ServiceHealthConfig::default()),
            enabled: false,
            criticality: TestCriticality::Important
        }
    }

    pub fn reverse_dns() -> CapabilityTest {
        CapabilityTest {
            test: Test::ReverseDns(ReverseDnsConfig::default()),
            criticality: TestCriticality::Important,
            enabled: false,
        }
    }

    pub fn dns_lookup() -> CapabilityTest {
        CapabilityTest { 
            test: Test::DnsLookup(DnsLookupConfig::default()), 
            criticality: TestCriticality::Important,
            enabled: false,
        }
    }

    pub fn dns_resolution() -> CapabilityTest {
        CapabilityTest { 
            test: Test::DnsResolution(DnsResolutionConfig::default()), 
            criticality: TestCriticality::Important,
            enabled: false,
        }
    }

    pub fn vpn_subnet() -> CapabilityTest {
        CapabilityTest { 
            test: Test::VpnSubnetAccess(VpnSubnetAccessConfig::default()), 
            criticality: TestCriticality::Important,
            enabled: false,
        }
    }
}