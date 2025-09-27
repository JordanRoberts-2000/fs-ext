use std::{fs, io, path::Path};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir, existing_file_ok))]
pub fn assert_exists(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_exists(path.as_ref())
}

fn _assert_exists(path: &Path) -> io::Result<()> {
    let meta = fs::metadata(path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to access '{}': {e}", path.display()))
    })?;

    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Path '{}' exists but is not a file (found: {:#?})",
                path.display(),
                meta.file_type()
            ),
        ));
    }

    Ok(())
}
