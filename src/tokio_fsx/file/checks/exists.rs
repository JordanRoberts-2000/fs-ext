use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn exists(path: impl AsRef<Path>) -> io::Result<bool> {
    _exists(path.as_ref()).await
}

async fn _exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path).await {
        Ok(meta) => Ok(meta.is_file()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("Failed to access '{}': {e}", path.display())))
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::exists, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_true_for_regular_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("a.txt");
        fs::write(&file, "hi").await.unwrap();

        let res = exists(&file).await.unwrap();
        assert!(res, "expected true for regular file");
    }

    #[tokio::test]
    async fn returns_false_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let res = exists(&missing).await.unwrap();
        assert!(!res, "expected false for missing path");
    }

    #[tokio::test]
    async fn returns_false_for_directory() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("folder");
        fs::create_dir_all(&subdir).await.unwrap();

        let res = exists(&subdir).await.unwrap();
        assert!(!res, "expected false for directory");
    }
}
