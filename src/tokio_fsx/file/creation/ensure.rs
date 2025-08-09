use {
    std::{io, path::Path},
    tokio::fs::{File, OpenOptions},
};

pub async fn ensure(path: impl AsRef<Path>) -> io::Result<File> {
    _ensure(path.as_ref()).await
}

async fn _ensure(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).create(true).open(path).await.map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open or create file at '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {super::ensure, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn creates_file_if_missing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new_file.txt");

        let _file = ensure(&file_path).await.unwrap();

        assert!(fs::try_exists(&file_path).await.unwrap(), "File should exist after ensure()");
        let meta = fs::metadata(&file_path).await.unwrap();
        assert!(meta.is_file(), "Path should be a file");
        assert_eq!(meta.len(), 0, "Newly created file should be empty");
    }

    #[tokio::test]
    async fn succeeds_if_file_already_exists_and_preserves_contents() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, b"hello").await.unwrap();

        let _file = ensure(&file_path).await.unwrap();

        let contents = fs::read(&file_path).await.unwrap();
        assert_eq!(contents, b"hello", "ensure() must not alter existing contents");
    }

    #[tokio::test]
    async fn errors_if_path_is_a_directory() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join("folder.ts");

        fs::create_dir(&dir_path).await.unwrap();

        let err = ensure(&dir_path).await.unwrap_err();

        // Platform differences: IsADirectory (Unix), PermissionDenied (Windows), sometimes Other.
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
