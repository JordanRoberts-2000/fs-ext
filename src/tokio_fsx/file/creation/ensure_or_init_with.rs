use {
    std::{io, path::Path},
    tokio::{fs, io::AsyncWriteExt},
};

pub async fn ensure_or_init_with<F, C>(path: impl AsRef<Path>, content_fn: F) -> io::Result<bool>
where
    F: FnOnce() -> C,
    C: AsRef<[u8]>,
{
    _ensure_or_init_with(path.as_ref(), content_fn).await
}

async fn _ensure_or_init_with<F, C>(path: &Path, content_fn: F) -> io::Result<bool>
where
    F: FnOnce() -> C,
    C: AsRef<[u8]>,
{
    match fs::OpenOptions::new().write(true).create_new(true).open(path).await {
        Ok(mut file) => {
            let content = content_fn();
            file.write_all(content.as_ref()).await?;
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
    use {super::ensure_or_init_with, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn creates_file_and_writes_closure_content() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new.txt");

        let created = ensure_or_init_with(&file_path, || "from closure").await.unwrap();
        assert!(created);

        let contents = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "from closure");
    }

    #[tokio::test]
    async fn returns_false_if_file_already_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, "original").await.unwrap();

        let created = ensure_or_init_with(&file_path, || "should not overwrite").await.unwrap();
        assert!(!created);

        let contents = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "original");
    }

    #[tokio::test]
    async fn errors_if_path_is_directory() {
        let dir = tempdir().unwrap();
        let subdir_path = dir.path().join("folder");

        fs::create_dir(&subdir_path).await.unwrap();

        let result = ensure_or_init_with(&subdir_path, || "data").await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[tokio::test]
    async fn supports_vec_u8_return_type_from_closure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bin");

        let created = ensure_or_init_with(&file_path, || vec![1u8, 2, 3]).await.unwrap();
        assert!(created);

        let data = fs::read(&file_path).await.unwrap();
        assert_eq!(data, vec![1u8, 2, 3]);
    }

    #[tokio::test]
    async fn supports_string_return_type_from_closure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("string.txt");

        let created = ensure_or_init_with(&file_path, || String::from("Hello")).await.unwrap();
        assert!(created);

        let content = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "Hello");
    }
}
