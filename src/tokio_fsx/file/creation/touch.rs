use {
    filetime::FileTime,
    std::{io, path::Path, time::SystemTime},
    tokio::fs,
};

pub async fn touch(path: impl AsRef<Path>) -> io::Result<bool> {
    _touch(path.as_ref()).await
}

async fn _touch(path: &Path) -> io::Result<bool> {
    match fs::OpenOptions::new().write(true).create_new(true).open(path).await {
        Ok(_) => Ok(true), // File created

        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            let meta = fs::metadata(path).await.map_err(|e| {
                io::Error::new(e.kind(), format!("Failed to inspect '{}': {e}", path.display()))
            })?;

            if !meta.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Path '{}' exists but is not a file (file type: {:?})",
                        path.display(),
                        meta.file_type()
                    ),
                ));
            }

            // Update modification time in a blocking task
            let path_buf = path.to_path_buf();
            tokio::task::spawn_blocking(move || {
                let now = FileTime::from_system_time(SystemTime::now());
                filetime::set_file_times(&path_buf, now, now)
            })
            .await
            .map_err(|join_err| {
                io::Error::new(io::ErrorKind::Other, format!("Join error: {join_err}"))
            })??;

            Ok(false) // File existed already
        }

        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("Failed to open/create '{}': {e}", path.display()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use {
        super::touch,
        std::{io, time::Duration},
        tempfile::tempdir,
        tokio::{fs, time::sleep},
    };

    #[tokio::test]
    async fn creates_file_when_missing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new.txt");

        let created = touch(&file_path).await.unwrap();
        assert!(created, "Expected touch to create a missing file");

        let meta = fs::metadata(&file_path).await.unwrap();
        assert!(meta.is_file(), "Expected a file to exist after touch");
    }

    #[tokio::test]
    async fn returns_false_and_updates_mtime_when_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");

        // Create file first
        fs::write(&file_path, b"hello").await.unwrap();
        let meta_before = fs::metadata(&file_path).await.unwrap();
        let mtime_before = meta_before.modified().unwrap();

        // Some filesystems have 1s mtime resolution; wait to ensure a change is observable
        sleep(Duration::from_millis(1100)).await;

        let created = touch(&file_path).await.unwrap();
        assert!(!created, "Expected touch to return false when file already exists");

        let meta_after = fs::metadata(&file_path).await.unwrap();
        let mtime_after = meta_after.modified().unwrap();

        assert!(
            mtime_after > mtime_before,
            "Expected mtime to be updated. before={mtime_before:?}, after={mtime_after:?}"
        );
    }

    #[tokio::test]
    async fn preserves_existing_contents() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("keep.txt");

        fs::write(&file_path, b"original").await.unwrap();
        let _ = touch(&file_path).await.unwrap();

        let contents = fs::read(&file_path).await.unwrap();
        assert_eq!(contents, b"original", "Touch must not alter file contents");
    }

    #[tokio::test]
    async fn errors_if_path_is_directory() {
        let dir = tempdir().unwrap();
        let subdir_path = dir.path().join("a_dir");

        fs::create_dir(&subdir_path).await.unwrap();

        let result = touch(&subdir_path).await;
        assert!(result.is_err(), "Touch should error on a directory path");

        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
