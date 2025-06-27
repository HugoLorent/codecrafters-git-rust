use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

// Declare submodules
mod blob;
mod hash;
mod path;
mod tree;

// Re-export public items
pub use blob::create_blob_object;
pub use hash::{calculate_object_hash, calculate_sha1};
pub use path::git_object_path;
pub use tree::{display_tree_entries, parse_tree_entries};

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
