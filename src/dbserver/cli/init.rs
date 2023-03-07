use crate::cli::DBServerCommand;
use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Path to the config file
    #[arg(short, long)]
    config: String,
}

impl DBServerCommand for InitArgs {
    fn execute(self) {
        println!("executing");
    }
}
