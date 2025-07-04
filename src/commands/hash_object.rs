use crate::git_objects::{self, create_blob_object};
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn run(write: bool, file_path: PathBuf) -> Result<()> {
    // Read file content as bytes
    let file_content = fs::read(&file_path)
        .with_context(|| format!("Failed to read input file: {}", file_path.display()))?;

    let object_content = create_blob_object(&file_content);

    let hash_hex = if write {
        git_objects::write_git_object(&object_content)?
    } else {
        git_objects::calculate_object_hash(&object_content)
    };
    println!("{hash_hex}");

    Ok(())
}
