use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone)]
pub enum GitObjectType {
    Blob,
    Tree,
}

impl GitObjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GitObjectType::Blob => "blob",
            GitObjectType::Tree => "tree",
        }
    }

    fn from_mode(mode: &str) -> Result<Self> {
        match mode {
            m if m.starts_with("100") => Ok(GitObjectType::Blob), // regular file
            m if m.starts_with("120") => Ok(GitObjectType::Blob), // symlink
            "40000" => Ok(GitObjectType::Tree),                   // directory
            _ => Err(anyhow!("Unknown file mode: {}", mode)),
        }
    }
}

#[derive(Debug)]
pub struct TreeEntry {
    pub mode: String,
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

        let mode = std::str::from_utf8(&content[pos..pos + space_pos])
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
        if pos + 20 > content.len() {
            return Err(anyhow!("Invalid tree entry: incomplete SHA-1 hash"));
        }
        let sha1_bytes = &content[pos..pos + 20];
        let sha1_hex = sha1_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        pos += 20;

        let object_type = GitObjectType::from_mode(mode)
            .with_context(|| format!("Invalid mode '{}' for entry '{}'", mode, name))?;

        entries.push(TreeEntry {
            mode: mode.to_string(),
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
                entry.mode,
                entry.object_type.as_str(),
                entry.sha1,
                entry.name
            );
        }
    }
}
