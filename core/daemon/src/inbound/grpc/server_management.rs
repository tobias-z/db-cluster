use super::proto::{server_management_server::ServerManagement, InitRequest, InitResponse};
use crate::daemon::Daemon;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct ServerManagementController {
    daemon: Arc<Daemon>,
}

impl ServerManagementController {
    pub fn new(daemon: Arc<Daemon>) -> Self {
        Self { daemon }
    }
}

// Generate join key for the server
// Change the state, and emit that a change has happened in the state.
// Return the join key.
//
// Comunication using asymmetric encryption. RSA
#[tonic::async_trait]
impl ServerManagement for ServerManagementController {
    async fn init(&self, _: Request<InitRequest>) -> Result<Response<InitResponse>, Status> {
        // TODO: What kind of config should we require here?
        let join_token = auth::token::generate_join_token();
        let mut state = self.daemon.desired_state.lock().unwrap();
        if state.set_join_token(Some(join_token.clone())).is_err() {
            return Err(Status::internal("Unable to keep the system up to date"));
        }
        Ok(Response::new(InitResponse { join_token }))
    }
}
