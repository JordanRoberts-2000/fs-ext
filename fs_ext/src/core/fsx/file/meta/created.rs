use {
    crate::IoResultExt,
    std::{fs, io, path::Path, time::SystemTime},
};

pub fn created(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    _created(path.as_ref())
}

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir))]
fn _created(path: &Path) -> io::Result<SystemTime> {
    let meta = fs::metadata(path).with_path_context("Failed to get metadata", path)?;

    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Path '{}' is not a regular file", path.display()),
        ));
    }

    meta.created().map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to get creation time for '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {super::created, std::fs, tempfile::tempdir};

    #[test]
    fn returns_creation_time_for_existing_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "hello").unwrap();

        created(&file_path).unwrap();
    }
}
