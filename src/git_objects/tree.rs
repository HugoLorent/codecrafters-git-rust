use crate::git_objects::hash::SHA1_BYTES;
use crate::git_objects::{
    create_blob_object, hex_to_bytes, write_git_object, FileMode, GitObjectType,
};
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct TreeEntry {
    pub mode: FileMode,
    pub name: String,
    pub sha1: String,
    pub object_type: GitObjectType,
}

/// Parses tree content and returns a vector of entries
pub fn parse_tree_entries(content: &[u8]) -> Result<Vec<TreeEntry>> {
    let mut entries = Vec::new();

    // Find the null byte that separates the header from the content
    let null_pos = content
        .iter()
        .position(|&b| b == 0)
        .context("Invalid tree object: missing null separator")?;

    // Skip the header (e.g., "tree 123\0")
    let mut pos = null_pos + 1;

    while pos < content.len() {
        // Find the space that separates mode from name
        let space_pos = content[pos..]
            .iter()
            .position(|&b| b == b' ')
            .context("Invalid tree entry: missing space")?;

        let mode_str = std::str::from_utf8(&content[pos..pos + space_pos])
            .context("Invalid mode in tree entry")?;
        pos += space_pos + 1;

        // Find the null byte that separates name from SHA-1
        let null_pos = content[pos..]
            .iter()
            .position(|&b| b == 0)
            .context("Invalid tree entry: missing null separator")?;

        let name = std::str::from_utf8(&content[pos..pos + null_pos])
            .context("Invalid name in tree entry")?;
        pos += null_pos + 1;

        // Read the 20-byte SHA-1 hash
        if pos + SHA1_BYTES > content.len() {
            return Err(anyhow!("Invalid tree entry: incomplete SHA-1 hash"));
        }
        let sha1_bytes = &content[pos..pos + SHA1_BYTES];
        let sha1_hex = sha1_bytes
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        pos += SHA1_BYTES;

        let mode = FileMode::from_str(mode_str)?;
        let object_type = mode.to_object_type();

        entries.push(TreeEntry {
            mode,
            name: name.to_string(),
            sha1: sha1_hex,
            object_type,
        });
    }

    Ok(entries)
}

/// Displays tree entries
pub fn display_tree_entries(entries: &[TreeEntry], name_only: bool) {
    for entry in entries {
        if name_only {
            println!("{}", entry.name);
        } else {
            println!(
                "{} {} {}\t{}",
                entry.mode.as_str(),
                entry.object_type.as_str(),
                entry.sha1,
                entry.name
            );
        }
    }
}

/// Creates a tree object from a directory
pub fn write_tree(directory_path: &str) -> Result<String> {
    let mut entries = Vec::new();

    // Read directory contents
    let dir_entries = fs::read_dir(directory_path)
        .with_context(|| format!("Failed to read directory: {directory_path}"))?;

    for entry in dir_entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip .git directory
        if name == ".git" {
            continue;
        }

        let file_type = entry.file_type().context("Failed to get file type")?;

        if file_type.is_file() {
            // Create blob object
            let file_content = fs::read(&path)
                .with_context(|| format!("Failed to read file: {}", path.display()))?;
            let blob_content = create_blob_object(&file_content);
            let blob_hash = write_git_object(&blob_content)?;

            // Determine file mode
            let mode = if is_executable(&path)? {
                FileMode::ExecutableFile
            } else {
                FileMode::RegularFile
            };
            entries.push((mode, name, blob_hash));
        } else if file_type.is_dir() {
            // Recursively create tree object
            let subtree_hash = write_tree(&path.to_string_lossy())?;
            entries.push((FileMode::Directory, name, subtree_hash));
        }
    }

    // Sort entries by name (Git requirement)
    entries.sort_by(|a, b| a.1.cmp(&b.1));

    // Build tree content
    let tree_content = build_tree_content(&entries)?;
    let tree_hash = write_git_object(&tree_content)?;

    Ok(tree_hash)
}

/// Builds the binary content of a tree object
fn build_tree_content(entries: &[(FileMode, String, String)]) -> Result<Vec<u8>> {
    let mut content = Vec::new();

    // Calculate total size first
    let mut size = 0;
    for (mode, name, _) in entries {
        size += mode.as_str().len() + 1 + name.len() + 1 + SHA1_BYTES; // mode + space + name + null + 20 bytes hash
    }

    // Add header
    let header = format!("tree {size}\0");
    content.extend_from_slice(header.as_bytes());

    // Add entries
    for (mode, name, hash) in entries {
        content.extend_from_slice(mode.as_str().as_bytes());
        content.push(b' ');
        content.extend_from_slice(name.as_bytes());
        content.push(0); // null byte
        content.extend_from_slice(&hex_to_bytes(hash)?);
    }

    Ok(content)
}

/// Checks if a file is executable
fn is_executable(path: &Path) -> Result<bool> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(path)?;
        let permissions = metadata.permissions();
        Ok(permissions.mode() & 0o111 != 0)
    }

    #[cfg(not(unix))]
    {
        // On Windows, check file extension
        Ok(path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("exe"))
            .unwrap_or(false))
    }
}
