use {
    std::{io, path::Path},
    tokio::{fs, io::AsyncWriteExt},
};

pub async fn ensure_or_init(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> io::Result<bool> {
    _ensure_or_init(path.as_ref(), content.as_ref()).await
}

async fn _ensure_or_init(path: &Path, content: &[u8]) -> io::Result<bool> {
    match fs::OpenOptions::new().write(true).create_new(true).open(path).await {
        Ok(mut file) => {
            file.write_all(content).await?;
            Ok(true)
        }

        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => match fs::metadata(path).await {
            Ok(meta) if meta.is_file() => Ok(false),

            Ok(meta) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Path '{}' exists but is not a file (file type: {:?})",
                    path.display(),
                    meta.file_type()
                ),
            )),

            Err(e) => Err(io::Error::new(
                e.kind(),
                format!("Failed to inspect existing path '{}': {e}", path.display()),
            )),
        },

        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("Failed to create file at '{}': {e}", path.display()),
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

        let result = ensure_or_init(&file_path, "hello world").await.unwrap();
        assert!(result, "Expected file to be created");

        let contents = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "hello world");
    }

    #[tokio::test]
    async fn returns_false_if_file_already_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");

        fs::write(&file_path, "original").await.unwrap();

        let result = ensure_or_init(&file_path, "new content").await.unwrap();
        assert!(!result, "Expected existing file to not be recreated");

        let contents = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "original", "File contents should remain unchanged");
    }

    #[tokio::test]
    async fn errors_if_path_is_directory() {
        let dir = tempdir().unwrap();
        let sub_dir = dir.path().join("subdir");

        fs::create_dir(&sub_dir).await.unwrap();

        let result = ensure_or_init(&sub_dir, "data").await;
        assert!(result.is_err(), "Should error if path is a directory");

        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
