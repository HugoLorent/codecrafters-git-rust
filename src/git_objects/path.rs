use std::path::{Path, PathBuf};

/// Creates the path for a Git object from its hash
pub fn git_object_path(object_hash: &str) -> PathBuf {
    Path::new(".git")
        .join("objects")
        .join(&object_hash[..2])
        .join(&object_hash[2..])
}
