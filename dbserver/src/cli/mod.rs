pub mod init;

use clap::{Parser, Subcommand};

use crate::cli::init::InitArgs;

// TODO: Ensure that we have good messages

/// DBServer help about message
#[derive(Parser)]
#[clap(author, version, about)]
pub struct DBServerArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init(InitArgs),
}

trait DBServerCommand {
    fn execute(self);
}

impl Command {
    pub fn execute(self) {
        match self {
            Command::Init(args) => args.execute()
        }
    }
}
