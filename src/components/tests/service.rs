use crate::components::{
        nodes::types::{base::Node, tests::AssignedTest}, tests::types::{execution::{TestResult, Timer}}
    };

pub struct TestService {}

impl TestService {
    pub fn new() -> Self { 
        Self {}
    }

    // pub async fn execute_test(&self, test: &Test, node: &Node) -> TestResult {
    //     let timer = Timer::now();
    //     let test_result = test.execute(&timer, &node).await;

    //     match test_result {
    //         Ok(result) => result,
    //         Err(e) => TestResult::error_result(e, None, timer)
    //     }
    // }

    pub async fn execute_assigned_test(&self, assigned_test: &AssignedTest, node: &Node) -> TestResult {

        let test = &assigned_test.test;
        let criticality = &assigned_test.criticality;
        let timer = Timer::now();
        let test_result = test.execute(&timer, &node).await;

        match test_result {
            Ok(mut result) => {
                result.criticality = Some(criticality.clone());
                result
            },
            Err(e) => TestResult::error_result(e, Some(criticality.clone()), timer)
        }
    }

}

