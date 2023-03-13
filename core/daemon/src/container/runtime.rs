use std::collections::HashMap;

pub struct Container {
    pub container_name: String,
    pub image_name: String,
    pub environment: Option<HashMap<String, String>>,
}

pub trait ContainerRuntime {
    // TOOD: Use better errors
    fn run(&self, container: Container) -> Result<ManagedContainer, Box<dyn std::error::Error>>;
    fn stop(&self, container: Container) -> Result<ManagedContainer, Box<dyn std::error::Error>>;
    fn kill(&self, container: Container) -> Result<ManagedContainer, Box<dyn std::error::Error>>;
    fn remove(&self, container: Container) -> Result<ManagedContainer, Box<dyn std::error::Error>>;
    fn get_running(&self) -> Vec<ManagedContainer>;
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
