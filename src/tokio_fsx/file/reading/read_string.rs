use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn read_string(path: impl AsRef<Path>) -> io::Result<String> {
    _read_string(path.as_ref()).await
}

async fn _read_string(path: &Path) -> io::Result<String> {
    fs::read_to_string(path).await.map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to read file '{}' as string: {e}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use {super::read_string, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_string_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.txt");
        fs::write(&file, "hello").await.unwrap();

        let s = read_string(&file).await.unwrap();
        assert_eq!(s, "hello");
    }

    #[tokio::test]
    async fn err_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = read_string(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
