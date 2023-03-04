// - Runs on all nodes
// - Spec of containers that should run in a pod.
//  - Some kind of event loop?
// - actions like making this an admin node, joining an admin node

use std::sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex};

use application::start_grpc_server;

pub mod application;
pub mod event_loop;

pub type AppSender = Arc<Mutex<Sender<String>>>;
pub type AppReceiver = Receiver<String>;

#[tokio::main]
async fn main() {
    let (sender, receiver) = channel::<String>();
    let sender = Arc::new(Mutex::new(sender));
    tokio::spawn(start_grpc_server(Arc::clone(&sender)));
    event_loop::start_event_loop(Arc::clone(&sender), receiver);
}
