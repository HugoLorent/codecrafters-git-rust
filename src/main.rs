mod commands;
mod git_objects;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "A simple Git implementation in Rust", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize a new Git repository
    Init,
    /// Provide contents or details of repository objects
    CatFile {
        /// Pretty-print the contents of the object
        #[arg(short = 'p', help = "Pretty-print the contents of the object")]
        pretty_print: bool,
        /// The SHA-1 hash of the object to display
        #[arg(help = "The SHA-1 hash of the object to display")]
        object_hash: String,
    },
    /// Compute object ID and optionally create an object from a file
    HashObject {
        /// Write the object into the object database
        #[arg(short = 'w', help = "Write the object into the object database")]
        write: bool,
        /// Path to the file to hash
        #[arg(help = "Path to the file to hash")]
        file_path: PathBuf,
    },
    /// List the contents of a tree object
    LsTree {
        #[arg(help = "The hash of the tree to inspect")]
        tree_sha: String,
        #[arg(
            long = "name-only",
            help = "List only filenames (instead of the \"long\" output), one per line"
        )]
        name_only: bool,
    },
    /// Create a tree object from the current directory
    WriteTree,
    /// Create a commit object
    CommitTree {
        #[arg(help = "The hash of the tree to commit")]
        tree_sha: String,
        #[arg(short = 'p', help = "The hash of the parent commit")]
        parent_sha: Option<String>,
        #[arg(short = 'm', value_name = "MESSAGE", help = "The commit message")]
        message: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Init => {
            commands::init::run()?;
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            commands::cat_file::run(pretty_print, object_hash)?;
        }
        Command::HashObject { write, file_path } => {
            commands::hash_object::run(write, file_path)?;
        }
        Command::LsTree {
            tree_sha,
            name_only,
        } => {
            commands::ls_tree::run(tree_sha, name_only)?;
        }
        Command::WriteTree => {
            commands::write_tree::run()?;
        }
        Command::CommitTree {
            tree_sha,
            parent_sha,
            message,
        } => {
            commands::commit_tree::run(tree_sha, parent_sha, message)?;
        }
    }
    Ok(())
}
