use anyhow::{Context, Result};
use sha1::{Digest, Sha1};

pub const SHA1_LENGTH: usize = 40;
pub const SHA1_BYTES: usize = 20;

/// Validates a SHA-1 hash string
pub fn validate_sha1(hash: &str) -> Result<()> {
    if hash.len() != SHA1_LENGTH {
        return Err(anyhow::anyhow!(
            "Invalid SHA-1 length: expected {}, got {}",
            SHA1_LENGTH,
            hash.len()
        ));
    }
    if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(anyhow::anyhow!(
            "Invalid SHA-1 format: contains non-hex characters"
        ));
    }
    Ok(())
}

/// Calculates SHA1 hash of content and returns hex string
pub fn calculate_object_hash(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    format!("{hash:x}")
}

/// Converts hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>> {
    validate_sha1(hex)?; // Utilise la validation interne

    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .with_context(|| format!("Invalid hex byte: {}", &hex[i..i + 2]))
        })
        .collect()
}
