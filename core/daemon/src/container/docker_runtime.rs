use super::runtime::{Container, ContainerRuntime, ManagedContainer};

#[derive(Default)]
pub struct DockerRuntime;

impl ContainerRuntime for DockerRuntime {
    fn run(&self, container: Container) -> Result<ManagedContainer, Box<dyn std::error::Error>> {
        todo!()
    }

    fn stop(&self, container: Container) -> Result<ManagedContainer, Box<dyn std::error::Error>> {
        todo!()
    }

    fn kill(&self, container: Container) -> Result<ManagedContainer, Box<dyn std::error::Error>> {
        todo!()
    }

    fn remove(&self, container: Container) -> Result<ManagedContainer, Box<dyn std::error::Error>> {
        todo!()
    }

    fn get_running(&self) -> Vec<ManagedContainer> {
        todo!()
    }
}
