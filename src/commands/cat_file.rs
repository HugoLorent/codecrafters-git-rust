use crate::git_objects;
use anyhow::{Context, Result};

pub fn run(pretty_print: bool, object_hash: String) -> Result<()> {
    if pretty_print {
        let content = git_objects::read_git_object(&object_hash)?;

        // Convert to string for blob content
        let blob_content = String::from_utf8(content).context("Invalid UTF-8 in object content")?;

        // Remove header from blob content, get only real file content by splitting from the null byte
        let file_content = blob_content
            .split('\0')
            .nth(1)
            .context("Invalid object format: missing null separator")?;
        print!("{}", file_content.trim_end());
    }
    Ok(())
}
