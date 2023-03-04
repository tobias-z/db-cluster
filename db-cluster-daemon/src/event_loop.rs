use crate::{AppSender, AppReceiver};

pub fn start_event_loop(sender: AppSender, receiver: AppReceiver) {
    loop {
        match receiver.recv() {
            Ok(data) => println!("Received data: {}", data),
            // There are no senders left. So the server is prob down.
            Err(err) => {
                println!("Shutting down");
                break;
            }
        }
    }
}
