use {
    filetime::FileTime,
    std::{io, path::Path, time::SystemTime},
    tokio::fs::{File, OpenOptions},
};

pub async fn touch(path: impl AsRef<Path>) -> io::Result<File> {
    _touch(path.as_ref()).await
}

async fn _touch(path: &Path) -> io::Result<File> {
    let file = OpenOptions::new().write(true).create(true).open(path).await.map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open or create file at '{}': {e}", path.display()),
        )
    })?;

    let path_buf = path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        let now = FileTime::from_system_time(SystemTime::now());
        filetime::set_file_times(&path_buf, now, now).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to update file atime & mtime for '{}': {e}", path_buf.display()),
            )
        })
    })
    .await
    .map_err(|join_err| {
        io::Error::new(io::ErrorKind::Other, format!("Join error: {join_err}"))
    })??;

    Ok(file)
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

        let _file = touch(&file_path).await.unwrap();

        let meta = fs::metadata(&file_path).await.unwrap();
        assert!(meta.is_file(), "Expected a file to exist after touch");
    }

    #[tokio::test]
    async fn updates_mtime_when_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");

        fs::write(&file_path, b"hello").await.unwrap();
        let mtime_before = fs::metadata(&file_path).await.unwrap().modified().unwrap();

        // Many filesystems have ~1s mtime resolution; wait to avoid flakes
        sleep(Duration::from_millis(1100)).await;

        let _file = touch(&file_path).await.unwrap();

        let mtime_after = fs::metadata(&file_path).await.unwrap().modified().unwrap();

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
