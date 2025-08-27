use {
    std::{io, path::Path},
    tokio::fs,
};

pub async fn assert_writable(path: impl AsRef<Path>) -> io::Result<()> {
    _assert_writable(path.as_ref()).await
}

async fn _assert_writable(path: &Path) -> io::Result<()> {
    match fs::OpenOptions::new().write(true).open(path).await {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("File '{}' is not writable: {e}", path.display())))
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::assert_writable, std::io, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn ok_when_file_is_writable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("writable.txt");
        fs::write(&file_path, "hello").await.unwrap();

        let res = assert_writable(&file_path).await;
        assert!(res.is_ok(), "expected Ok(()), got {res:?}");
    }

    #[tokio::test]
    async fn err_when_file_missing() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("missing.txt");

        let err = assert_writable(&missing).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        let msg = err.to_string();
        assert!(msg.contains("is not writable"), "msg={msg}");
        assert!(msg.contains(missing.to_string_lossy().as_ref()), "msg={msg}");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn err_when_file_not_writable() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("locked.txt");
        fs::write(&file_path, "secret").await.unwrap();

        // Make file read-only (no write bits)
        let mut perms = fs::metadata(&file_path).await.unwrap().permissions();
        perms.set_mode(0o444);
        fs::set_permissions(&file_path, perms).await.unwrap();

        let err = assert_writable(&file_path).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
        let msg = err.to_string();
        assert!(msg.contains("is not writable"), "msg={msg}");
    }
}
