use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_file, rejects_existing_dir))]
pub fn assert_not_exists(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_not_exists(path.as_ref())
}

fn _assert_not_exists(path: &Path) -> io::Result<()> {
    match fs::metadata(path).with_path_context("Failed to get metadata", path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Ok(meta) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "Path '{}' unexpectedly exists (found: {:#?})",
                path.display(),
                meta.file_type()
            ),
        )),
        Err(e) => Err(e),
    }
}
