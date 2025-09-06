use crate::server::services::types::{endpoints::{Endpoint, EndpointResponse}, ports::Port};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    AnyPort(Vec<Port>),      // Match if at least one port is open
    AllPort(Vec<Port>),           // Match if ALL of these ports are open
    AnyResponse(Vec<EndpointResponse>), // Match if at least one endpoint response contains the response string
    None,
}

impl Pattern {
    pub fn matches(&self, open_ports: Vec<Port>, responses: Vec<EndpointResponse>) -> bool {
        match self {
            Pattern::AnyPort(ports) => ports.iter().any(|p| open_ports.contains(p)),
            Pattern::AllPort(ports) => ports.iter().all(|p| open_ports.contains(p)),
            Pattern::AnyResponse(endpoint_responses) => {
                endpoint_responses.iter().any(|expected| {
                    responses.iter().any(|actual| {
                        actual.endpoint == expected.endpoint && 
                        actual.response.contains(&expected.response)
                    })
                })
            },
            Pattern::None => false
        }
    }

    pub fn ports(&self) -> Vec<Port> {
        match self {
            Pattern::AnyPort(ports) => ports.to_vec(),
            Pattern::AllPort(ports) => ports.to_vec(),
            Pattern::AnyResponse(_) => vec!(),
            Pattern::None => vec!()
        }
    }

    pub fn endpoints(&self) -> Vec<Endpoint> {
        match self {
            Pattern::AnyPort(_) => vec!(),
            Pattern::AllPort(_) => vec!(),
            Pattern::AnyResponse(endpoint_response) => endpoint_response.iter().map(|er| er.endpoint.clone()).collect(),
            Pattern::None => vec!()
        }
    }
}