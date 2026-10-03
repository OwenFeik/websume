use std::path::PathBuf;

pub enum UrlError {
    RelativePath,
    IllegalByte(u8),
}

pub fn parse_path(path: &[u8]) -> Result<PathBuf, UrlError> {
    let mut path = PathBuf::new();
    let mut segment = Vec::new();

    for byte in path {
        if byte == b'/' {
            if !segment.is_empty() {
                path.push(&segment);
            }
        }
    }

    Ok(path)
}
