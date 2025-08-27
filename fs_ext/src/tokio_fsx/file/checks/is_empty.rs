use {
    crate::tokio::fsx,
    std::{io, path::Path},
};

pub async fn is_empty(path: impl AsRef<Path>) -> io::Result<bool> {
    Ok(fsx::file::size(path).await? == 0)
}

#[cfg(test)]
mod tests {
    use {super::is_empty, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_true_for_empty_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.txt");
        fs::File::create(&file).await.unwrap();

        let res = is_empty(&file).await.unwrap();
        assert!(res, "expected true for empty file");
    }

    #[tokio::test]
    async fn returns_false_for_non_empty_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.txt");
        fs::write(&file, "hello").await.unwrap();

        let res = is_empty(&file).await.unwrap();
        assert!(!res, "expected false for non-empty file");
    }

    #[tokio::test]
    async fn propagates_error_for_missing_file() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = is_empty(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
