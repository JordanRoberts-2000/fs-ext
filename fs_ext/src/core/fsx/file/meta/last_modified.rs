use {
    crate::IoResultExt,
    std::{fs, io, path::Path, time::SystemTime},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir))]
pub fn last_modified(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    _last_modified(path.as_ref())
}

fn _last_modified(path: &Path) -> io::Result<SystemTime> {
    let meta = fs::metadata(path).with_path_context("Failed to get metadata", path)?;

    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Path '{}' is not a regular file", path.display()),
        ));
    }

    meta.modified()
}

#[cfg(test)]
mod tests {
    use {super::last_modified, std::fs, tempfile::tempdir};

    #[test]
    fn returns_time_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("file.txt");
        fs::write(&file, "hello").unwrap();

        last_modified(&file).unwrap();
    }
}
