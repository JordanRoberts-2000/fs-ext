use {
    std::{io, path::Path},
    tokio::{
        fs::{File, OpenOptions},
        io::AsyncWriteExt,
    },
};

pub async fn ensure_or_init_with<F, C>(path: impl AsRef<Path>, content_fn: F) -> io::Result<File>
where
    F: FnOnce() -> C,
    C: AsRef<[u8]>,
{
    let path = path.as_ref();

    match OpenOptions::new().write(true).open(path).await {
        Ok(file) => Ok(file),

        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let content = content_fn();
            let bytes = content.as_ref();

            let mut file =
                OpenOptions::new().write(true).create(true).open(path).await.map_err(|e| {
                    io::Error::new(
                        e.kind(),
                        format!("Failed to create file at '{}': {e}", path.display()),
                    )
                })?;

            file.write_all(bytes).await.map_err(|e| {
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
    use {
        super::ensure_or_init_with,
        std::{
            io,
            sync::atomic::{AtomicU32, Ordering},
        },
        tempfile::tempdir,
        tokio::fs,
    };

    #[tokio::test]
    async fn creates_file_and_writes_closure_content() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new.txt");

        let _file = ensure_or_init_with(&file_path, || "from closure").await.unwrap();

        assert!(
            fs::try_exists(&file_path).await.unwrap(),
            "File should exist after ensure_or_init_with()"
        );
        let contents = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "from closure");
    }

    #[tokio::test]
    async fn opens_existing_file_without_overwriting() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, "original").await.unwrap();

        let _file = ensure_or_init_with(&file_path, || "should not overwrite").await.unwrap();

        let contents = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "original", "Existing file content must remain unchanged");
    }

    #[tokio::test]
    async fn errors_if_path_is_directory() {
        let dir = tempdir().unwrap();
        let subdir_path = dir.path().join("folder");

        fs::create_dir(&subdir_path).await.unwrap();

        let result = ensure_or_init_with(&subdir_path, || "data").await;
        assert!(result.is_err(), "Expected error when path is a directory");

        // Platform differences: IsADirectory on Unix, PermissionDenied on Windows, occasionally Other.
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

    #[tokio::test]
    async fn supports_vec_u8_return_type_from_closure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bin");

        let _file = ensure_or_init_with(&file_path, || vec![1u8, 2, 3]).await.unwrap();

        assert!(fs::try_exists(&file_path).await.unwrap(), "File should exist");
        let data = fs::read(&file_path).await.unwrap();
        assert_eq!(data, vec![1u8, 2, 3]);
    }

    #[tokio::test]
    async fn supports_string_return_type_from_closure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("string.txt");

        let _file = ensure_or_init_with(&file_path, || String::from("Hello")).await.unwrap();

        assert!(fs::try_exists(&file_path).await.unwrap(), "File should exist");
        let content = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "Hello");
    }

    #[tokio::test]
    async fn closure_not_called_when_file_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");
        fs::write(&file_path, "original").await.unwrap();

        let call_count = AtomicU32::new(0);
        let _ = ensure_or_init_with(&file_path, || {
            call_count.fetch_add(1, Ordering::SeqCst);
            "unused"
        })
        .await
        .unwrap();

        assert_eq!(
            call_count.load(Ordering::SeqCst),
            0,
            "Closure should not be called when file exists"
        );
    }

    #[tokio::test]
    async fn closure_called_once_when_file_missing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("missing.txt");

        let call_count = AtomicU32::new(0);
        let _ = ensure_or_init_with(&file_path, || {
            call_count.fetch_add(1, Ordering::SeqCst);
            "generated content"
        })
        .await
        .unwrap();

        assert_eq!(call_count.load(Ordering::SeqCst), 1, "Closure should be called exactly once");
        assert_eq!(fs::read_to_string(&file_path).await.unwrap(), "generated content");
    }
}
