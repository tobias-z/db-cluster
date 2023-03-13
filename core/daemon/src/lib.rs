// - Runs on all nodes
// - Spec of containers that should run in a pod.
//  - Some kind of event loop?
// - actions like making this an admin node, joining an admin node

use std::sync::Arc;

use crate::daemon::Daemon;
use crossbeam_channel::{Receiver, Sender};
use inbound::{
    event::{start_defered_events_loop, DaemonEvent},
    grpc::start_grpc_server,
};

pub mod daemon;
pub mod inbound;
pub mod container;

pub type AppSender = Sender<DaemonEvent>;
pub type AppReceiver = Receiver<DaemonEvent>;

#[tokio::main]
pub async fn run() {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let daemon = Arc::new(Daemon::new(sender));
    tokio::spawn(start_grpc_server(Arc::clone(&daemon)));
    start_defered_events_loop(Arc::clone(&daemon), receiver);
}
