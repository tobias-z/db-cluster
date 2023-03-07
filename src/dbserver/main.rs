use crate::cli::DBServerArgs;
use clap::Parser;

pub mod cli;

fn main() {
    let args = DBServerArgs::parse();
    args.command.execute();
}
