use crate::git_objects;
use anyhow::Result;

pub fn run(tree_sha: String, name_only: bool) -> Result<()> {
    let tree_content = git_objects::read_git_object(&tree_sha)?;
    let entries = git_objects::parse_tree_entries(&tree_content)?;
    git_objects::display_tree_entries(&entries, name_only);
    Ok(())
}
