use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

pub fn ensure(path: impl AsRef<Path>) -> io::Result<File> {
    _ensure(path.as_ref())
}

pub fn _ensure(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).create(true).open(path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open or create file at '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {
        super::ensure,
        std::{fs, io},
        tempfile::tempdir,
    };

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
    fn succeeds_if_file_already_exists_and_preserves_contents() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, b"hello").unwrap();

        let _file = ensure(&file_path).unwrap();

        let contents = fs::read(&file_path).unwrap();
        assert_eq!(contents, b"hello", "ensure() must not alter existing contents");
    }

    #[test]
    fn errors_if_path_is_a_directory() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join("folder.ts");

        fs::create_dir(&dir_path).unwrap();

        let err = ensure(&dir_path).unwrap_err();

        // On Unix this is commonly IsADirectory; on Windows often PermissionDenied.
        let kind = err.kind();
        assert!(
            matches!(
                kind,
                io::ErrorKind::IsADirectory
                    | io::ErrorKind::PermissionDenied
                    | io::ErrorKind::Other
            ),
            "Unexpected error kind for directory: {kind:?}"
        );
    }
}
