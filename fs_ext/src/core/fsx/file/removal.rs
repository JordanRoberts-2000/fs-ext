use std::{fs, io, path::Path};

pub fn remove(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    fs::remove_file(path).map_err(|e| {
        io::Error::new(e.kind(), format!("failed to remove '{}': {}", path.display(), e))
    })
}

pub fn trash(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();

    trash::delete(path).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to trash '{}': {}", path.display(), e))
    })
}

pub fn trash_or_remove(path: impl AsRef<Path>) -> io::Result<()> {
    trash(&path).or_else(|trash_err| {
        remove(path).map_err(|remove_err| {
            let msg = format!("trash failed: {trash_err}; remove failed: {remove_err}");
            io::Error::new(remove_err.kind(), msg)
        })
    })
}
