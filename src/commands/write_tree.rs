use crate::git_objects;
use anyhow::Result;

pub fn run() -> Result<()> {
    let tree_hash = git_objects::write_tree(".")?;
    println!("{tree_hash}");
    Ok(())
}
