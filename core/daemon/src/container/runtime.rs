use std::collections::HashMap;

use async_trait::async_trait;

pub struct Container {
    pub container_name: String,
    pub image_name: String,
    pub environment: Option<HashMap<String, String>>,
}

#[async_trait]
pub trait ContainerRuntime {
    // TOOD: Use better errors
    async fn run(
        &self,
        container: Container,
    ) -> Result<ManagedContainer, Box<dyn std::error::Error>>;
    async fn stop(
        &self,
        container: Container,
    ) -> Result<ManagedContainer, Box<dyn std::error::Error>>;
    async fn kill(
        &self,
        container: Container,
    ) -> Result<ManagedContainer, Box<dyn std::error::Error>>;
    async fn remove(
        &self,
        container: Container,
    ) -> Result<ManagedContainer, Box<dyn std::error::Error>>;
    async fn get_running(&self) -> Result<Vec<ManagedContainer>, Box<dyn std::error::Error>>;
}

pub struct ManagedContainer {
    pub container: Container,
    pub state: ContainerState,
}

pub enum ContainerState {
    Started,
    Running,
    Stopped,
    Killed,
}
