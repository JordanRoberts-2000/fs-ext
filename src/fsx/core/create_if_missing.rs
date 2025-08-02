use {
    crate::utils::create_file_or_dir,
    std::{fs, io, path::Path},
};

pub fn create_if_missing(path: impl AsRef<Path>) -> io::Result<()> {
    _create_if_missing(path.as_ref())
}

fn _create_if_missing(path: &Path) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => create_file_or_dir(path),
        Err(e) => Err(e),
    }
}
