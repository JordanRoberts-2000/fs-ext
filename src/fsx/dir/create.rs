use std::{fs, io, path::Path};

pub fn ensure(path: impl AsRef<Path>) -> io::Result<()> {
    let p = path.as_ref();
    fs::create_dir_all(p).map_err(|e| {
        io::Error::new(e.kind(), format!("failed to create directory at '{}': {e}", p.display()))
    })
}

pub fn create_new(path: impl AsRef<Path>) -> io::Result<()> {
    let p = path.as_ref();
    fs::create_dir(p).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("failed to create new directory at '{}': {e}", p.display()),
        )
    })
}
