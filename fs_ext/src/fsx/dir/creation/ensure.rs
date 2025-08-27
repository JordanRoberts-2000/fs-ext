#[cfg(test)]
use fs_ext_test_macros::fs_test;
use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_test(rejects_file, existing_dir_ok, new_dir_ok))]
pub fn ensure(path: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&path).with_path_context("failed to create directory", path)
}
