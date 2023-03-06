pub mod completions;
pub mod init;

use clap::{Parser, Subcommand};

use crate::cli::{completions::CompletionArgs, init::InitArgs};

// TODO: Ensure that we have good messages

/// DBServer help about message
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct DBServerArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init(InitArgs),
    Completions(CompletionArgs),
}

trait DBServerCommand {
    fn execute(self);
}

impl Command {
    pub fn execute(self) {
        match self {
            Command::Init(args) => args.execute(),
            Command::Completions(args) => args.execute(),
        }
    }
}
