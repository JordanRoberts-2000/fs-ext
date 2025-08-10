use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn size(path: impl AsRef<Path>) -> io::Result<u64> {
    _size(path.as_ref()).await
}

async fn _size(path: &Path) -> io::Result<u64> {
    fs::metadata(path).await.map(|m| m.len()).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to get size of '{}': {e}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use {super::size, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_size_for_regular_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");

        fs::write(&file, b"hello").await.unwrap();

        let len = size(&file).await.unwrap();
        assert_eq!(len, 5);
    }

    #[tokio::test]
    async fn returns_zero_for_empty_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.bin");

        fs::File::create(&file).await.unwrap();

        let len = size(&file).await.unwrap();
        assert_eq!(len, 0);
    }

    #[tokio::test]
    async fn err_when_missing() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = size(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
