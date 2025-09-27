use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(existing_file_ok, rejects_dir, new_file_ok))]
pub fn ensure(path: impl AsRef<Path>) -> io::Result<File> {
    _ensure(path.as_ref())
}

fn _ensure(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).create(true).open(path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open or create file at '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {super::ensure, std::fs, tempfile::tempdir};

    #[test]
    fn creates_file_if_missing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new_file.txt");

        let _file = ensure(&file_path).unwrap();

        assert!(file_path.exists(), "File should exist after ensure()");
        let meta = fs::metadata(&file_path).unwrap();
        assert!(meta.is_file(), "Path should be a file");

        assert_eq!(meta.len(), 0, "Newly created file should be empty");
    }

    #[test]
    fn preserves_contents_if_file_already_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, b"hello").unwrap();

        let _file = ensure(&file_path).unwrap();

        let contents = fs::read(&file_path).unwrap();
        assert_eq!(contents, b"hello", "ensure() must not alter existing contents");
    }
}
