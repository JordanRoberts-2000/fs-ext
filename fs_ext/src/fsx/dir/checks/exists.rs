#[cfg(test)]
use fs_ext_test_macros::fs_test;
use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_test(existing_dir_ok))]
pub fn exists(path: impl AsRef<Path>) -> io::Result<bool> {
    _exists(path.as_ref())
}

fn _exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(meta) => Ok(meta.is_dir()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e).with_path_context("Failed to access", path),
    }
}
