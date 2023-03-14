use std::sync::Arc;

use async_trait::async_trait;

use super::{
    request::{ContainerRequester, ContainerRequesterImpl},
    runtime::{Container, ContainerRuntime, ManagedContainer},
};

const DOCKER_SOCKET_URI: &str = "/var/run/docker.sock";

/// API used for reference: https://docs.docker.com/engine/api/v1.42/
pub struct DockerRuntime {
    request: Arc<dyn ContainerRequester + Sync + Send>,
}

impl Default for DockerRuntime {
    fn default() -> Self {
        Self {
            request: Arc::new(ContainerRequesterImpl::new(DOCKER_SOCKET_URI.to_string())),
        }
    }
}

#[async_trait]
impl ContainerRuntime for DockerRuntime {
    async fn run(
        &self,
        container: Container,
    ) -> Result<ManagedContainer, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn stop(
        &self,
        container: Container,
    ) -> Result<ManagedContainer, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn kill(
        &self,
        container: Container,
    ) -> Result<ManagedContainer, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn remove(
        &self,
        container: Container,
    ) -> Result<ManagedContainer, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn get_running(&self) -> Result<Vec<ManagedContainer>, Box<dyn std::error::Error>> {
        let body = self.request.get("/containers/json?all=1&size=1");
        Ok(vec![])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct RequesterMock;

    #[async_trait]
    impl ContainerRequester for RequesterMock {
        async fn post(&self, path: &str, body: &str) -> Result<String, hyper::Error> {
            todo!()
        }

        async fn get(&self, path: &str) -> Result<String, hyper::Error> {
            todo!()
        }
    }

    #[tokio::test]
    async fn something() {
        let runtime = DockerRuntime {
            request: Arc::new(RequesterMock),
        };
        runtime.get_running().await.unwrap();
    }
}
