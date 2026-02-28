use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

// ── Config types ────────────────────────────────────────────────────────────

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct VitalsConfig {
    pub require: RequireConfig,
    pub ports: PortsConfig,
    pub services: ServicesConfig,
    pub env: EnvConfig,
    #[serde(default)]
    pub commands: Vec<CommandConfig>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RequireConfig {
    pub node: Option<String>,
    pub python: Option<String>,
    pub rust: Option<String>,
    pub go: Option<String>,
    pub ruby: Option<String>,
    pub java: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct PortsConfig {
    pub check: Vec<u16>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ServicesConfig {
    pub docker: Option<bool>,
    pub redis: Option<ServiceDetail>,
    pub postgres: Option<ServiceDetail>,
    pub mysql: Option<ServiceDetail>,
    pub mongo: Option<ServiceDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceDetail {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct EnvConfig {
    pub required: Vec<String>,
    pub example: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CommandConfig {
    pub name: String,
    pub run: String,
}

// ── Loader ──────────────────────────────────────────────────────────────────

/// Load .vitals.toml from the given directory. Returns defaults if not found.
pub fn load(dir: &Path) -> Result<VitalsConfig> {
    let path = dir.join(".vitals.toml");
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let config: VitalsConfig = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(VitalsConfig::default())
    }
}

/// Template content for `vitals --init`.
pub fn template() -> &'static str {
    r#"# .vitals.toml — project health check configuration
# Commit this file so your whole team benefits from `vitals`

# Runtime version requirements (auto-detected from project files if omitted)
[require]
# node = ">=18"
# python = ">=3.11"
# rust = ">=1.75"
# go = ">=1.21"

# Ports that should be available
[ports]
# check = [3000, 5432, 6379]

# Services that should be running
[services]
# docker = true

# [services.redis]
# host = "localhost"
# port = 6379

# [services.postgres]
# host = "localhost"
# port = 5432

# Environment variables
[env]
# required = ["DATABASE_URL", "REDIS_URL", "API_KEY"]
# example = ".env.example"

# Custom checks (commands that should exit 0)
# [[commands]]
# name = "db-migrations"
# run = "npx prisma migrate status"
"#
}
