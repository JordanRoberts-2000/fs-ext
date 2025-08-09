use {
    std::{io, path::Path},
    tokio::{
        fs::{File, OpenOptions},
        io::AsyncWriteExt,
    },
};

pub async fn ensure_or_init(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> io::Result<File> {
    _ensure_or_init(path.as_ref(), content.as_ref()).await
}

async fn _ensure_or_init(path: &Path, content: &[u8]) -> io::Result<File> {
    match OpenOptions::new().write(true).open(path).await {
        Ok(file) => Ok(file),

        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let mut file =
                OpenOptions::new().write(true).create(true).open(path).await.map_err(|e| {
                    io::Error::new(
                        e.kind(),
                        format!("Failed to create file at '{}': {e}", path.display()),
                    )
                })?;

            file.write_all(content).await.map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!("Failed to write to file at '{}': {e}", path.display()),
                )
            })?;

            Ok(file)
        }

        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("Failed to open file at '{}': {e}", path.display()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use {super::ensure_or_init, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn creates_file_and_writes_content() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        let _file = ensure_or_init(&file_path, "hello world").await.unwrap();

        assert!(
            fs::try_exists(&file_path).await.unwrap(),
            "File should exist after ensure_or_init()"
        );
        let contents = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "hello world");
    }

    #[tokio::test]
    async fn opens_existing_file_without_overwriting() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");

        fs::write(&file_path, "original").await.unwrap();

        let _file = ensure_or_init(&file_path, "new content").await.unwrap();

        let contents = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "original", "Existing file contents must remain unchanged");
    }

    #[tokio::test]
    async fn errors_if_path_is_directory() {
        let dir = tempdir().unwrap();
        let sub_dir = dir.path().join("subdir");

        fs::create_dir(&sub_dir).await.unwrap();

        let result = ensure_or_init(&sub_dir, "data").await;
        assert!(result.is_err(), "Should error if path is a directory");

        // Platform differences: IsADirectory (Unix), PermissionDenied (Windows), sometimes Other.
        let kind = result.unwrap_err().kind();
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
