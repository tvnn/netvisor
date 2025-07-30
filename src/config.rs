use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub web: WebSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSettings {
    pub external_path: Option<PathBuf>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 3000,
                log_level: "info".to_string(),
            },
            database: DatabaseSettings {
                path: PathBuf::from("./netzoot.db"),
            },
            web: WebSettings {
                external_path: None,
            },
        }
    }
}

impl ServerConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("netzoot.toml").required(false))
            .add_source(config::Environment::with_prefix("NETZOOT"))
            .build()?;

        let mut server_config: ServerConfig = config.try_deserialize().unwrap_or_default();

        // Ensure database directory exists
        if let Some(parent) = server_config.database.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(server_config)
    }

    pub fn database_url(&self) -> String {
        format!("sqlite:{}", self.database.path.display())
    }
}