use {
    crate::fsx::types::TempDir,
    std::{io, path::Path},
};

pub fn temp() -> io::Result<TempDir> {
    TempDir::new()
}

pub fn temp_in(dir: impl AsRef<Path>) -> io::Result<TempDir> {
    TempDir::in_dir(dir)
}
