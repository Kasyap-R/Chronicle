use crate::args::{Commands, UserArgs};

use anyhow::{Result, anyhow};
use clap::Parser;
use std::path::Path;

mod hashing;
mod initialize;
mod objects;
mod paths;
mod staging;

pub fn process_command() -> Result<()> {
    let user_args = UserArgs::parse();

    ensure_valid_repo_state(&user_args)?;

    match user_args.command {
        Commands::Init => initialize::init_chronicle_repo()?,
        Commands::Add { path } => staging::handle_staging(&path)?,
        Commands::Commit => (),
        Commands::Branch(_branch_commands) => (),
    }

    Ok(())
}

fn ensure_valid_repo_state(user_args: &UserArgs) -> Result<()> {
    let repo_exists = git_repo_exists();
    let command_is_init = matches!(user_args.command, Commands::Init);

    if repo_exists && command_is_init {
        return Err(anyhow!("Chronicle repo already exists."));
    } else if !repo_exists && !command_is_init {
        return Err(anyhow!("Chronicle repo does not exist in this directory."));
    }
    Ok(())
}

fn git_repo_exists() -> bool {
    let path = Path::new(".chronicle/");
    return path.exists();
}
