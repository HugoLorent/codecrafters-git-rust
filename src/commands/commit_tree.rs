use crate::git_objects;
use anyhow::Result;

pub fn run(tree_sha: String, parent_sha: Option<String>, message: String) -> Result<()> {
    let commit_hash =
        git_objects::create_commit_object(&tree_sha, parent_sha.as_deref(), &message)?;
    println!("{commit_hash}");
    Ok(())
}
