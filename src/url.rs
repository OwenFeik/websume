use std::{os::unix::ffi::OsStrExt, path::PathBuf};

#[derive(Debug)]
pub enum UrlError {
    RelativePath,
    IllegalByte(u8),
}

pub fn parse_path(path_bytes: &[u8]) -> Result<PathBuf, UrlError> {
    let mut path = PathBuf::new();
    let mut segment = Vec::new();

    for &byte in path_bytes {
        if byte == b'/' {
            if !segment.is_empty() {
                if segment == b".." {
                    return Err(UrlError::RelativePath);
                }
                path.push(std::ffi::OsStr::from_bytes(&segment));
                segment.clear();
            }
        } else {
            segment.push(byte);
        }
    }
    if !segment.is_empty() {
        if segment == b".." {
            return Err(UrlError::RelativePath);
        }
        path.push(std::ffi::OsStr::from_bytes(&segment));
    }

    Ok(path)
}
