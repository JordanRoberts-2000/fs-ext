#[cfg(test)]
use fs_ext_test_macros::fs_test;
use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_test(rejects_missing_path, rejects_file, existing_dir_ok))]
pub fn is_empty(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref();
    let mut entries = fs::read_dir(&path).with_path_context("Failed to read directory ", &path)?;
    Ok(entries.next().is_none())
}
