use crate::{
    components::{
        tests::{
            types::{TestType, TestConfiguration, TestResult, Timer},
            implementations::*,
        },
    },
};

pub struct TestService {}

impl TestService {
    pub fn new() -> Self { 
        Self {}
    }

    pub async fn execute_test(
        &self,
        test_type: &TestType,
        config: &TestConfiguration,
    ) -> TestResult {

        let timer = Timer::now();

        // Execute test based on type and configuration
        let test_result = match (test_type, config) {
            (TestType::Connectivity, TestConfiguration::Connectivity(config)) => {
                connectivity::execute_connectivity_test(config, &timer).await
            },
            (TestType::DirectIp, TestConfiguration::DirectIp(config)) => {
                connectivity::execute_direct_ip_test(config, &timer).await
            },
            (TestType::Ping, TestConfiguration::Ping(config)) => {
                connectivity::execute_ping_test(config, &timer).await
            },
            (TestType::WellknownIp, TestConfiguration::WellknownIp(config)) => {
                connectivity::execute_wellknown_ip_test(config, &timer).await
            },
            (TestType::DnsResolution, TestConfiguration::DnsResolution(config)) => {
                dns::execute_dns_resolution_test(config, &timer).await
            },
            (TestType::DnsOverHttps, TestConfiguration::DnsOverHttps(config)) => {
                dns::execute_dns_over_https_test(config, &timer).await
            },
            (TestType::VpnConnectivity, TestConfiguration::VpnConnectivity(config)) => {
                vpn::execute_vpn_connectivity_test(config, &timer).await
            },
            (TestType::VpnTunnel, TestConfiguration::VpnTunnel(config)) => {
                vpn::execute_vpn_tunnel_test(config, &timer).await
            },
            (TestType::ServiceHealth, TestConfiguration::ServiceHealth(config)) => {
                service::execute_service_health_test(config, &timer).await
            },
            // (TestType::DaemonCommand, TestConfiguration::DaemonCommand(config)) => {
            //     daemon::execute_daemon_command_test(config).await
            // },
            // (TestType::SshScript, TestConfiguration::SshScript(config)) => {
            //     daemon::execute_ssh_script_test(config).await
            // },
            // Type safety ensures this should never happen, but handle gracefully
            _ => {
                Err(anyhow::anyhow!(
                    "Provided configuration does not match test type {:?}",
                    test_type
                ))
            }
        };

        match test_result {
            Ok(result) => result,
            Err(e) => TestResult::error_result(test_type, e, timer)
        }
    }

}

