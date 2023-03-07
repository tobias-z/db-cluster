mod node;

use crate::{inbound::event::DaemonEvent, AppNotifier};
use std::sync::{Arc, Mutex};

pub type AppState = Arc<Mutex<DesiredState>>;

pub struct Daemon {
    pub desired_state: AppState,
    pub notifier: AppNotifier,
}
impl Daemon {
    pub fn new(notifier: AppNotifier) -> Self {
        Self {
            desired_state: Arc::new(Mutex::new(DesiredState::new(Arc::clone(&notifier)))),
            notifier,
        }
    }
}

impl Default for Daemon {
    fn default() -> Self {
        todo!()
    }
}

pub struct DesiredState {
    // Notifier used to notify our event loop about any change happening to our desired state
    notifier: AppNotifier,
    join_token: Option<String>,
}

type NotifiedSetter = Result<(), std::sync::mpsc::SendError<DaemonEvent>>;

impl DesiredState {
    pub fn new(notifier: AppNotifier) -> Self {
        Self {
            notifier,
            join_token: None,
        }
    }

    fn notify(&self, event: DaemonEvent) -> NotifiedSetter {
        let notifier = self.notifier.lock().unwrap();
        notifier.send(event)
    }

    pub fn set_join_token(&mut self, join_token: Option<String>) -> NotifiedSetter {
        let old_token = self.join_token.clone();
        self.join_token = join_token;
        self.notify(DaemonEvent::JoinTokenChanged(
            old_token,
            self.join_token.clone(),
        ))
    }
}
