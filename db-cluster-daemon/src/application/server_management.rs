use tonic::{Request, Response, Status};

use crate::AppSender;

use super::proto::{server_management_server::ServerManagement, InitRequest, InitResponse};

pub struct ServerManagementController {
    sender: AppSender,
}

impl ServerManagementController {
    pub fn new(sender: AppSender) -> Self {
        Self { sender }
    }
}

#[tonic::async_trait]
impl ServerManagement for ServerManagementController {
    async fn init(&self, request: Request<InitRequest>) -> Result<Response<InitResponse>, Status> {
        // TODO: Handle unwrap
        let sender = self.sender.lock().unwrap();
        sender.send(request.get_ref().config.clone());
        Ok(Response::new(InitResponse {
            response_message: "hello".to_string(),
        }))
    }
}
