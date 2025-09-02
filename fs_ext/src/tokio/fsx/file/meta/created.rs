use {
    std::{io, path::Path, time::SystemTime},
    tokio::fs,
};

pub async fn created(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    _created(path.as_ref()).await
}

async fn _created(path: &Path) -> io::Result<SystemTime> {
    let meta = fs::metadata(path).await.map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to get creation time for '{}': {e}", path.display()),
        )
    })?;

    meta.created().map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to get creation time for '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {super::created, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_creation_time_for_existing_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "hello").await.unwrap();

        created(&file_path).await.unwrap();
    }

    #[tokio::test]
    async fn returns_error_for_missing_file() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = created(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
