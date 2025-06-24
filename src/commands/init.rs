use anyhow::{Context, Result};
use std::fs;

pub fn run() -> Result<()> {
    fs::create_dir(".git").context("Failed to create .git directory")?;
    fs::create_dir(".git/objects").context("Failed to create .git/objects directory")?;
    fs::create_dir(".git/refs").context("Failed to create .git/refs directory")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n").context("Failed to write HEAD file")?;
    println!("Initialized git directory");
    Ok(())
}
