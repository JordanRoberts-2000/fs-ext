use std::{fs, io, path::Path, time::SystemTime};

pub fn created(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    _created(path.as_ref())
}

fn _created(path: &Path) -> io::Result<SystemTime> {
    fs::metadata(path).and_then(|meta| meta.created()).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to get creation time for '{}': {e}", path.display()),
        )
    })
}
