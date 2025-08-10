use std::{fs, io, path::Path};

pub fn read_string(path: impl AsRef<Path>) -> io::Result<String> {
    _read_string(path.as_ref())
}

fn _read_string(path: &Path) -> io::Result<String> {
    fs::read_to_string(path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to read file '{}' as string: {e}", path.display()))
    })
}
