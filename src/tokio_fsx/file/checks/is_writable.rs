use {std::path::Path, tokio::fs};

pub async fn is_writable(path: &Path) -> bool {
    fs::OpenOptions::new().write(true).open(path).await.is_ok()
}

#[cfg(test)]
mod tests {
    use {super::is_writable, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_true_for_writable_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("file.txt");
        fs::write(&file, "hello").await.unwrap();

        assert!(is_writable(&file).await, "expected true for writable file");
    }

    #[tokio::test]
    async fn returns_false_for_missing_file() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        assert!(!is_writable(&missing).await, "expected false for missing file");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn returns_false_for_non_writable_file() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let file = dir.path().join("locked.txt");
        fs::write(&file, "secret").await.unwrap();

        // Remove all write permissions (read-only: 0o444)
        let mut perms = fs::metadata(&file).await.unwrap().permissions();
        perms.set_mode(0o444);
        fs::set_permissions(&file, perms).await.unwrap();

        assert!(!is_writable(&file).await, "expected false for file without write permissions");
    }
}
