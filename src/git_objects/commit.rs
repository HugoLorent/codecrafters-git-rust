use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

/// Creates a commit object
pub fn create_commit_object(
    tree_hash: &str,
    parent_hash: Option<&str>,
    message: &str,
) -> Result<String> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    // Default author info
    let author_name = "Hugo";
    let author_email = "test@example.com";
    let timezone = "+0000";

    let mut commit_content = String::new();

    // Tree line
    commit_content.push_str(&format!("tree {tree_hash}\n"));

    // Parent line (if exists)
    if let Some(parent) = parent_hash {
        commit_content.push_str(&format!("parent {parent}\n"));
    }

    // Author line
    commit_content.push_str(&format!(
        "author {author_name} <{author_email}> {timestamp} {timezone}\n",
    ));

    // Committer line (same as author)
    commit_content.push_str(&format!(
        "committer {author_name} <{author_email}> {timestamp} {timezone}\n",
    ));

    // Empty line before message
    commit_content.push('\n');

    // Commit message
    commit_content.push_str(message);

    // Empty line after message
    commit_content.push('\n');

    // Create commit object with header
    let header = format!("commit {}\0", commit_content.len());
    let mut object_content = Vec::new();
    object_content.extend_from_slice(header.as_bytes());
    object_content.extend_from_slice(commit_content.as_bytes());

    // Write object and return hash
    crate::git_objects::write_git_object(&object_content)
}
