use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn read_bytes(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    _read_bytes(path.as_ref()).await
}

async fn _read_bytes(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path).await.map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to read file '{}': {e}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use {super::read_bytes, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_bytes_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        fs::write(&file, b"hello").await.unwrap();

        let bytes = read_bytes(&file).await.unwrap();
        assert_eq!(bytes, b"hello");
    }

    #[tokio::test]
    async fn err_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = read_bytes(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
