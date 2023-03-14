use async_trait::async_trait;
use hyper::{body, Body, Client, Method, Request};
use hyperlocal::{UnixConnector, Uri};

#[async_trait]
pub trait ContainerRequester {
    async fn post(&self, path: &str, body: &str) -> Result<String, hyper::Error>;
    async fn get(&self, path: &str) -> Result<String, hyper::Error>;
}

pub struct ContainerRequesterImpl {
    socket_uri: String,
}

impl ContainerRequesterImpl {
    pub fn new(socket_uri: String) -> Self {
        Self { socket_uri }
    }

    async fn request(
        &self,
        method: Method,
        path: &str,
        body: &str,
    ) -> Result<String, hyper::Error> {
        let uri = Uri::new(&self.socket_uri, path);
        let client = Client::builder().build::<UnixConnector, Body>(UnixConnector);
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .body(body.to_string().into_bytes().into())
            .expect("Unable to build request to docker");
        let response = client.request(request).await?;
        Ok(String::from_utf8(body::to_bytes(response.into_body()).await?.to_vec()).unwrap())
    }
}

#[async_trait]
impl ContainerRequester for ContainerRequesterImpl {
    async fn post(&self, path: &str, body: &str) -> Result<String, hyper::Error> {
        self.request(Method::POST, path, body).await
    }

    async fn get(&self, path: &str) -> Result<String, hyper::Error> {
        self.request(Method::GET, path, "").await
    }
}
