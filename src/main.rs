mod config;
use std::process::{Command, exit};
use clap::Parser;
use cyonix::args::{Cli, Commands, GitsCommands};
use crate::config::{base_directory, config_directory};

fn main() {
    let args = Cli::parse();
    
    match args.command {
        Commands::Add { file } => {
            println!("Cloning {file}");
        }
        Commands::Git(git) => {
            match git.command {
                GitsCommands::Init => {
                    println!("Initializing git repo");
                    let dotfiles_dir = config_directory(&base_directory());
                    let git_dir = dotfiles_dir.join(".git");
                    if git_dir.exists() {
                        println!("Git repository already exists");
                        exit(1);
                    }
                    Command::new("git")
                        .args(["init", dotfiles_dir.to_str().unwrap()])
                        .output()
                        .unwrap();
                }
                GitsCommands::Push => {
                    println!("Pushing dotfiles to git repo");
                }
            }
        }
        Commands::Restore { file } => {
            if let Some(file) = file {
                println!("Restoring {file}");
            } else {
                println!("Restoring all files");
            }
        }
    }
}
