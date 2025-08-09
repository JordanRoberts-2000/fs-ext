use {
    std::{io, path::Path},
    tokio::fs::{File, OpenOptions},
};

pub async fn overwrite(path: impl AsRef<Path>) -> io::Result<File> {
    _overwrite(path.as_ref()).await
}

async fn _overwrite(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).create(true).truncate(true).open(path).await.map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open or overwrite file at '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {
        super::overwrite,
        std::io,
        tempfile::tempdir,
        tokio::{
            fs,
            io::{AsyncReadExt, AsyncWriteExt},
        },
    };

    #[tokio::test]
    async fn creates_file_if_missing_and_is_empty() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new_file.txt");

        let _file = overwrite(&file_path).await.unwrap();

        assert!(fs::try_exists(&file_path).await.unwrap(), "File should exist after overwrite()");
        let meta = fs::metadata(&file_path).await.unwrap();
        assert!(meta.is_file(), "Path should be a file");
        assert_eq!(meta.len(), 0, "Newly created file should be empty");
    }

    #[tokio::test]
    async fn truncates_existing_file_to_zero_length() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, b"some existing content").await.unwrap();
        let meta_before = fs::metadata(&file_path).await.unwrap();
        assert!(meta_before.len() > 0, "Precondition: file should have content");

        let _file = overwrite(&file_path).await.unwrap();

        let meta_after = fs::metadata(&file_path).await.unwrap();
        assert_eq!(meta_after.len(), 0, "File should be truncated to zero length");

        let data = fs::read(&file_path).await.unwrap();
        assert!(data.is_empty(), "Contents should be empty after overwrite()");
    }

    #[tokio::test]
    async fn returned_file_is_writable_after_overwrite() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("writable.txt");

        let mut file = overwrite(&file_path).await.unwrap();
        file.write_all(b"hello").await.unwrap();
        file.flush().await.unwrap();

        let mut read_back = String::new();
        fs::File::open(&file_path).await.unwrap().read_to_string(&mut read_back).await.unwrap();
        assert_eq!(read_back, "hello", "Should be able to write via returned handle");
    }

    #[tokio::test]
    async fn errors_if_path_is_a_directory() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join("folder");

        fs::create_dir(&dir_path).await.unwrap();

        let err = overwrite(&dir_path).await.unwrap_err();

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
