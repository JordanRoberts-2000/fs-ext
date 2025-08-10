use std::{fs, io, path::Path};

pub fn size(path: impl AsRef<Path>) -> io::Result<u64> {
    _size(path.as_ref())
}

fn _size(path: &Path) -> io::Result<u64> {
    fs::metadata(path).map(|m| m.len()).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to get size of '{}': {e}", path.display()))
    })
}
