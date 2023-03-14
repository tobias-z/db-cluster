mod node;

use crossbeam_channel::SendError;

use crate::{
    container::{docker_runtime::DockerRuntime, runtime::ContainerRuntime},
    inbound::event::{DaemonEvent, JoinTokenChanged},
    AppSender,
};
use std::sync::{Arc, Mutex};

pub type AppState = Arc<Mutex<DesiredState>>;

pub struct Daemon {
    pub desired_state: AppState,
    pub sender: AppSender,
    pub container_runtime: Arc<dyn ContainerRuntime + Send + Sync>,
}

impl Daemon {
    pub fn new(sender: AppSender) -> Self {
        Self {
            desired_state: Arc::new(Mutex::new(DesiredState::new(sender.clone()))),
            sender,
            container_runtime: Arc::new(DockerRuntime::default()),
        }
    }
}

impl Default for Daemon {
    fn default() -> Self {
        todo!()
    }
}

pub struct DesiredState {
    // Sender used to notify our event loop about any change happening to our desired state
    sender: AppSender,
    join_token: Option<String>,
}

type NotifiedSetter = Result<(), SendError<DaemonEvent>>;

impl DesiredState {
    pub fn new(sender: AppSender) -> Self {
        Self {
            sender,
            join_token: None,
        }
    }

    pub fn set_join_token(&mut self, join_token: Option<String>) -> NotifiedSetter {
        let old_token = self.join_token.clone();
        self.join_token = join_token;
        self.sender
            .send(DaemonEvent::JoinTokenChanged(JoinTokenChanged::new(
                old_token,
                self.join_token.clone(),
            )))
    }
}
