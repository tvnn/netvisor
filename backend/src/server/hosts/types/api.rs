use serde::{Deserialize, Serialize};

use crate::server::{hosts::types::base::Host, services::types::base::Service};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostWithServicesRequest {
    pub host: Host,
    pub services: Vec<Service>,
}
