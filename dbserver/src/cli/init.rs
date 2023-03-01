use clap::Args;
use fork::{fork, Fork};

use crate::cli::DBServerCommand;

#[derive(Args)]
pub struct InitArgs {
    /// Path to the config file
    #[arg(short, long)]
    config: String,
}

impl DBServerCommand for InitArgs {
    fn execute(self) {
        match fork() {
            Ok(Fork::Parent(child)) => {
                println!(
                    "Continuing execution in parent process, new child has pid: {}",
                    child
                );
            }
            Ok(Fork::Child) => {
                std::thread::sleep(std::time::Duration::from_secs(5));
                println!("started server");
            },
            Err(_) => println!("Failed to initialize the server"),
        }
    }
}
