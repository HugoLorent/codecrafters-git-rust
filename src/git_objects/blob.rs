/// Creates a blob object with proper Git header
pub fn create_blob_object(file_content: &[u8]) -> Vec<u8> {
    let header = format!("blob {}\0", file_content.len());
    let mut object_content = Vec::new();
    object_content.extend_from_slice(header.as_bytes());
    object_content.extend_from_slice(file_content);
    object_content
}
