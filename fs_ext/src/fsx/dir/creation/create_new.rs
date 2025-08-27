#[cfg(test)]
use fs_ext_test_macros::fs_test;
use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_test(rejects_file, rejects_existing_dir, new_dir_ok))]
pub fn create_new(path: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir(&path).with_path_context("failed to create directory", path)
}
