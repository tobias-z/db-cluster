use std::{fmt::Display, path::{Path, PathBuf}};

use clap::{Args, CommandFactory};
use clap_complete::{generate_to, Generator, Shell};

use crate::cli::{DBServerArgs, DBServerCommand};

#[derive(Args, Debug)]
pub struct CompletionArgs {
    /// Create completions for bash
    #[arg(short, long)]
    bash: bool,

    /// Create completions for zsh
    #[arg(short, long)]
    zsh: bool,

    /// Create completions for fish
    #[arg(short, long)]
    fish: bool,

    /// Create completions for powershell
    #[arg(short, long)]
    powershell: bool,

    /// Create completions for elvish
    #[arg(short, long)]
    elvish: bool,

    /// The directory you want the completions to be generated at. Defaults to your current working
    /// directory
    #[arg(short, long)]
    outdir: Option<String>,
}

impl DBServerCommand for CompletionArgs {
    fn execute(self) {
        let mut cmd = DBServerArgs::command();
        let outdir = match self.outdir {
            Some(path) => PathBuf::from(path),
            None => std::env::current_dir().unwrap(),
        };
        if self.bash {
            generate_completions(Shell::Bash, &mut cmd, outdir.as_path());
        }
        if self.zsh {
            generate_completions(Shell::Zsh, &mut cmd, outdir.as_path());
        }
        if self.fish {
            generate_completions(Shell::Fish, &mut cmd, outdir.as_path());
        }
        if self.powershell {
            generate_completions(Shell::PowerShell, &mut cmd, outdir.as_path());
        }
        if self.elvish {
            generate_completions(Shell::Elvish, &mut cmd, outdir.as_path());
        }
    }
}

fn generate_completions<G>(gen: G, cmd: &mut clap::Command, outdir: &Path)
where
    G: Generator + Display,
{
    let generator = gen.to_string();
    if let Err(err) = generate_to(gen, cmd, cmd.get_name().to_string(), outdir) {
        let path = outdir.to_str().unwrap_or("");
        println!(
            "unable to generate completions for {} at path {} \nFull error: {}",
            generator, path, err
        );
    }
}
