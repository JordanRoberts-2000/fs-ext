use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_file, existing_dir_ok))]
pub fn assert_exists(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_exists(path.as_ref())
}

fn _assert_exists(path: &Path) -> io::Result<()> {
    let meta = fs::metadata(path).with_path_context("Failed to access", &path)?;

    if !meta.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Path '{}' exists but is not a directory (found: {:#?})",
                path.display(),
                meta.file_type()
            ),
        ));
    }

    Ok(())
}
