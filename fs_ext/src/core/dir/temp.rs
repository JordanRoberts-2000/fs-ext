#[cfg(test)]
use fs_ext_test_macros::fs_test;
use {
    crate::TempDir,
    std::{io, path::Path},
};

pub fn temp() -> io::Result<TempDir> {
    TempDir::new()
}

#[cfg_attr(test, fs_test(rejects_missing_path, rejects_file, existing_dir_ok))]
pub fn temp_in(dir: impl AsRef<Path>) -> io::Result<TempDir> {
    TempDir::in_dir(dir)
}
