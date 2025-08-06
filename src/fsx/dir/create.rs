use std::{fs, io, path::Path};

pub fn create(path: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(path)
}
