use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn assert_readable(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_readable(path.as_ref()).await
}

async fn _assert_readable(path: &Path) -> io::Result<()> {
    match fs::File::open(path).await {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("File '{}' is not readable: {e}", path.display())))
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::assert_readable, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn ok_when_file_is_readable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "hello").await.unwrap();

        let res = assert_readable(&file_path).await;
        assert!(res.is_ok(), "expected Ok(()), got {res:?}");
    }

    #[tokio::test]
    async fn err_when_file_missing() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = assert_readable(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn err_when_file_not_readable() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("locked.txt");
        fs::write(&file_path, "secret").await.unwrap();

        // Remove all read/write/execute permissions
        let mut perms = tokio::fs::metadata(&file_path).await.unwrap().permissions();
        perms.set_mode(0o000);
        tokio::fs::set_permissions(&file_path, perms).await.unwrap();

        let err = assert_readable(&file_path).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
    }
}
