use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn assert_exists(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_exists(path.as_ref()).await
}

async fn _assert_exists(path: &Path) -> io::Result<()> {
    let meta = fs::metadata(path).await.map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to access '{}': {e}", path.display()))
    })?;

    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Path '{}' exists but is not a file (found: {:#?})",
                path.display(),
                meta.file_type()
            ),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use {super::assert_exists, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn ok_when_regular_file_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("note.txt");
        fs::write(&file_path, "hi").await.unwrap();

        let res = assert_exists(&file_path).await;
        assert!(res.is_ok(), "expected Ok(()), got {res:?}");
    }

    #[tokio::test]
    async fn err_not_found_when_missing() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = assert_exists(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn err_invalid_input_when_path_is_dir() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("folder");
        fs::create_dir_all(&subdir).await.unwrap();

        let err = assert_exists(&subdir).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
