use std::{fs, io, path::Path};

pub fn exists(path: impl AsRef<Path>) -> io::Result<bool> {
    _exists(path.as_ref())
}

fn _exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(meta) => Ok(meta.is_file()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("Failed to access '{}': {e}", path.display())))
        }
    }
}
