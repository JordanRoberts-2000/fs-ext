use std::{fs, io, path::Path};

pub fn assert_not_exists(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_not_exists(path.as_ref())
}

fn _assert_not_exists(path: &Path) -> io::Result<()> {
    match fs::metadata(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),

        // Exists and is a file → reject
        Ok(meta) if meta.is_file() => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("File '{}' unexpectedly exists", path.display()),
        )),

        // Exists but not a file → also reject (wrong type in the way)
        Ok(meta) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Path '{}' exists but is not a file (found: {:#?})",
                path.display(),
                meta.file_type()
            ),
        )),

        // Any other access error
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("Failed to access '{}': {e}", path.display())))
        }
    }
}
