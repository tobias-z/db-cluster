use crate::{daemon::Daemon, AppReceiver};
use std::sync::Arc;

pub enum DaemonEvent {
    JoinTokenChanged(Option<String>, Option<String>),
}

impl DaemonEvent {
    fn handle(&self, daemon: &Daemon) {
        match self {
            DaemonEvent::JoinTokenChanged(old_token, new_token) => {
                if old_token.is_none() && new_token.is_some() {
                    daemon.init_as_admin(new_token.as_ref().unwrap());
                }
            }
        }
    }
}

pub fn start_defered_events_loop(daemon: Arc<Daemon>, receiver: AppReceiver) {
    loop {
        match receiver.recv() {
            Ok(event) => event.handle(&daemon),
            // There are no senders left. So the server is prob down.
            Err(_) => {
                // TODO: Logging
                println!("Shutting down");
                break;
            }
        }
    }
}
