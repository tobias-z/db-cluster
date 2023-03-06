use clap::Parser;

use crate::cli::DBServerArgs;

pub mod cli;

fn main() {
    let args = DBServerArgs::parse();
    args.command.execute();
}
