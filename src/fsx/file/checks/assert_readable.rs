use std::{fs, io, path::Path};

pub fn assert_readable(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_readable(path.as_ref())
}

fn _assert_readable(path: &Path) -> io::Result<()> {
    match fs::File::open(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("File '{}' is not readable: {e}", path.display())))
        }
    }
}
