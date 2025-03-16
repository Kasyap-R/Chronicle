use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "Chronicle",
    version = "1.0",
    about = "A simple version control CLI tool"
)]
pub struct UserArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a chronicle repository
    Init,
    /// Stage unsaved changes
    Add {
        /// Path to directory/file to save changes from
        directory: PathBuf,
    },
    /// Create a snapshot of the repository's state
    Commit,
    /// Manage branches
    Branch(BranchCommands),
}

#[derive(Args)]
pub struct BranchCommands {
    #[command(subcommand)]
    pub commands: BranchSubCommands,
}

#[derive(Subcommand)]
pub enum BranchSubCommands {
    /// Create a branch
    Create {
        /// Name of branch to create
        name: String,
    },
    /// Delete a branch
    Delete {
        /// Name of branch to delete
        name: String,
    },
}
