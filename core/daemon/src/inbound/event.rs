use crate::{daemon::Daemon, AppReceiver};
use std::sync::Arc;

pub enum DaemonEvent {
    JoinTokenChanged(JoinTokenChanged),
}

trait DaemonEventCommand {
    fn execute(&self, daemon: &Daemon);
}

impl DaemonEvent {
    fn handle(&self, daemon: &Daemon) {
        match self {
            DaemonEvent::JoinTokenChanged(command) => command.execute(daemon),
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

pub struct JoinTokenChanged {
    old_token: Option<String>,
    new_token: Option<String>,
}

impl JoinTokenChanged {
    pub fn new(old_token: Option<String>, new_token: Option<String>) -> Self {
        Self {
            old_token,
            new_token,
        }
    }
}

impl DaemonEventCommand for JoinTokenChanged {
    fn execute(&self, daemon: &Daemon) {
        if self.old_token.is_none() && self.new_token.is_some() {
            daemon.init_as_admin(self.new_token.as_ref().unwrap());
        }
    }
}
