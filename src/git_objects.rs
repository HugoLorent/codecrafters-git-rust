use anyhow::{anyhow, Context, Result};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

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

/// Reads and decompresses a Git object from its hash
pub fn read_git_object(object_hash: &str) -> Result<Vec<u8>> {
    let file_path = git_object_path(object_hash);
    let bytes = fs::read(&file_path)
        .with_context(|| format!("Failed to read object file: {}", file_path.display()))?;

    let mut decoder = ZlibDecoder::new(&bytes[..]);
    let mut content = Vec::new();
    decoder
        .read_to_end(&mut content)
        .context("Failed to decompress object")?;

    Ok(content)
}

/// Compresses and writes a Git object, returns the hash
pub fn write_git_object(content: &[u8]) -> Result<String> {
    // Calculate the hash
    let hash_hex = calculate_sha1(content);

    // Compress the content
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(content)
        .context("Failed to compress object")?;
    let compressed = encoder.finish().context("Failed to finish compression")?;

    // Create directory structure and write the file
    let dir_path = Path::new(".git").join("objects").join(&hash_hex[..2]);
    fs::create_dir_all(&dir_path)
        .with_context(|| format!("Failed to create object directory: {}", dir_path.display()))?;

    let file_path = git_object_path(&hash_hex);
    fs::write(&file_path, compressed)
        .with_context(|| format!("Failed to write object file: {}", file_path.display()))?;

    Ok(hash_hex)
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

/// Displays tree entries (replaces the old parse_tree_content)
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

pub fn calculate_object_hash(content: &[u8]) -> String {
    calculate_sha1(content)
}

pub fn create_blob_object(file_content: &[u8]) -> Vec<u8> {
    let header = format!("blob {}\0", file_content.len());
    let mut object_content = Vec::new();
    object_content.extend_from_slice(header.as_bytes());
    object_content.extend_from_slice(file_content);
    object_content
}

fn git_object_path(object_hash: &str) -> PathBuf {
    Path::new(".git")
        .join("objects")
        .join(&object_hash[..2])
        .join(&object_hash[2..])
}

fn calculate_sha1(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    format!("{:x}", hash)
}
