#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteAdhocTestRequest {
    pub node_id: String,
    pub test_type: TestType,
    pub test_config: TestConfiguration,
}