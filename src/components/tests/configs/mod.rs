pub mod connectivity;
pub mod dns;
pub mod vpn;
pub mod service;
pub mod remote;

// Re-export all config types
pub use connectivity::*;
pub use dns::*;
pub use vpn::*;
pub use service::*;
pub use remote::*;