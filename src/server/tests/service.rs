use crate::server::{
        capabilities::types::base::{Capability, CapabilityTest}, nodes::{service::NodeService, types::base::Node}, tests::types::execution::{TestResult, Timer}
    };

pub struct TestService {}

impl TestService {
    pub fn new() -> Self { 
        Self {}
    }

    pub async fn execute_test(&self, capability_test: &CapabilityTest, node: &Node, capability: &Capability, node_service: &NodeService) -> TestResult {

        let test = &capability_test.test;
        let criticality = &capability_test.criticality;
        let timer = Timer::now();

        let test_result = test.execute(&timer, &node, capability, &node_service).await;

        match test_result {
            Ok(mut result) => {
                result.criticality = Some(criticality.clone());
                result
            },
            Err(e) => TestResult::error_result(e, Some(criticality.clone()), timer)
        }
    }

}

