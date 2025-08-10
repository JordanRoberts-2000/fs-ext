use {
    std::{io, path::Path, time::SystemTime},
    tokio::fs,
};

pub async fn last_modified(path: impl AsRef<Path>) -> io::Result<SystemTime> {
    _last_modified(path.as_ref()).await
}

async fn _last_modified(path: &Path) -> io::Result<SystemTime> {
    let meta = fs::metadata(path).await.map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to get last modified time for '{}': {e}", path.display()),
        )
    })?;

    meta.modified().map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to get last modified time for '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {super::last_modified, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_time_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("file.txt");
        fs::write(&file, "hello").await.unwrap();

        last_modified(&file).await.unwrap();
    }

    #[tokio::test]
    async fn err_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = last_modified(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
