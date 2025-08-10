use std::{fs, io, path::Path};

pub fn read_bytes(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    _read_bytes(path.as_ref())
}

fn _read_bytes(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to read file '{}': {e}", path.display()))
    })
}
