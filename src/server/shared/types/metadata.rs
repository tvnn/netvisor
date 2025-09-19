use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct TypeRegistry {
    pub services: Vec<TypeMetadata>,
    pub subnet_types: Vec<TypeMetadata>,
    pub edge_types: Vec<TypeMetadata>
}

#[derive(Serialize, Debug, Clone)]
pub struct TypeMetadata {
    pub id: String,           // "VpnTunnel", "VpnServer", "SshAccess"
    pub display_name: String, // "VPN Tunnel", "VPN Server", "SSH Access"  
    pub description: String,  // Full description
    pub category: String,     // "VPN", "Infrastructure", "Remote Access"
    pub icon: String,         // "shield", "server", "terminal"
    pub color: String,        // "text-orange-400"
    pub metadata: serde_json::Value, // Type-specific extra data
}

// Universal trait for all domain entities
pub trait TypeMetadataProvider {
    fn id(&self) -> String;
    fn display_name(&self) -> &str;
    fn description(&self) -> &str;
    fn category(&self) -> &str;
    fn icon(&self) -> &str;
    fn color(&self) -> &str;
    fn metadata(&self) -> serde_json::Value;
    
    fn to_metadata(&self) -> TypeMetadata {
        TypeMetadata {
            id: self.id(),
            display_name: self.display_name().to_string(),
            description: self.description().to_string(),
            category: self.category().to_string(),
            icon: self.icon().to_string(),
            color: self.color().to_string(),
            metadata: self.metadata(),
        }
    }
}