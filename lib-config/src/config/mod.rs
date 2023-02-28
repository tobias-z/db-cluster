use crate::config::auth_config::AuthConfig;

pub mod auth_config;

#[derive(serde::Deserialize)]
pub enum ClusterConfig {
    Auth(AuthConfig),
}
