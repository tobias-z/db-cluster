// - Runs on all nodes
// - Spec of containers that should run in a pod.
//  - Some kind of event loop?
// - actions like making this an admin node, joining an admin node

use crate::daemon::Daemon;
use inbound::{
    event::{start_defered_events_loop, DaemonEvent},
    grpc::start_grpc_server,
};
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

pub mod daemon;
pub mod inbound;

pub type AppNotifier = Arc<Mutex<Sender<DaemonEvent>>>;
pub type AppReceiver = Receiver<DaemonEvent>;

#[tokio::main]
pub async fn run() {
    let (notifier, receiver) = channel::<DaemonEvent>();
    let daemon = Arc::new(Daemon::new(Arc::new(Mutex::new(notifier))));
    tokio::spawn(start_grpc_server(Arc::clone(&daemon)));
    start_defered_events_loop(Arc::clone(&daemon), receiver);
}
