use std::{fs, io, path::Path};

pub fn assert_writable(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_writable(path.as_ref())
}

fn _assert_writable(path: &Path) -> io::Result<()> {
    match fs::OpenOptions::new().write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("File '{}' is not writable: {e}", path.display())))
        }
    }
}
