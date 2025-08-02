use std::{fs, io, path::Path};

pub fn dir_create_if_missing(path: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(path)
}
