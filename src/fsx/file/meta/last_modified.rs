use std::{fs, io, path::Path, time::SystemTime};

pub fn last_modified(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    _last_modified(path.as_ref())
}

fn _last_modified(path: &Path) -> io::Result<SystemTime> {
    fs::metadata(path).and_then(|meta| meta.modified()).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to get last modified time for '{}': {e}", path.display()),
        )
    })
}
