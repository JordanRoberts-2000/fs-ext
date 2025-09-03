use {
    crate::{IoResultExt, PathExt},
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir))]
pub fn remove(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    path.assert_file()?;
    fs::remove_file(path).with_path_context("failed to remove", path)
}

pub fn trash(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();

    trash::delete(path).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to trash '{}': {}", path.display(), e))
    })
}

pub fn trash_or_remove(path: impl AsRef<Path>) -> io::Result<()> {
    trash(&path).or_else(|trash_err| {
        remove(path).map_err(|remove_err| {
            let msg = format!("trash failed: {trash_err}; remove failed: {remove_err}");
            io::Error::new(remove_err.kind(), msg)
        })
    })
}
