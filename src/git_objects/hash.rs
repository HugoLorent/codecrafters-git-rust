use sha1::{Digest, Sha1};

/// Calculates SHA1 hash of content and returns hex string
pub fn calculate_sha1(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

/// Calculates object hash (alias for calculate_sha1 for clarity)
pub fn calculate_object_hash(content: &[u8]) -> String {
    calculate_sha1(content)
}
