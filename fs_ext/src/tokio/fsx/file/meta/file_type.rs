use {
    std::{fs::FileType, io, path::Path},
    tokio::fs,
};

pub async fn file_type(path: impl AsRef<Path>) -> io::Result<FileType> {
    _file_type(path.as_ref()).await
}

async fn _file_type(path: &Path) -> io::Result<FileType> {
    fs::metadata(path).await.map(|meta| meta.file_type()).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to get file type for '{}': {e}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use {super::file_type, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_filetype_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("a.txt");
        fs::write(&file, "hi").await.unwrap();

        let ft = file_type(&file).await.unwrap();
        assert!(ft.is_file(), "expected is_file() to be true");
    }

    #[tokio::test]
    async fn err_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = file_type(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
