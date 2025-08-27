use {std::path::Path, tokio::fs};

pub async fn is_readable(path: impl AsRef<Path>) -> bool {
    fs::File::open(path).await.is_ok()
}

#[cfg(test)]
mod tests {
    use {super::is_readable, tempfile::tempdir, tokio::fs};

    #[tokio::test]
    async fn returns_true_for_readable_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("file.txt");
        fs::write(&file, "hello").await.unwrap();

        assert!(is_readable(&file).await, "expected true for readable file");
    }

    #[tokio::test]
    async fn returns_false_for_missing_file() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        assert!(!is_readable(&missing).await, "expected false for missing file");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn returns_false_for_non_readable_file() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let file = dir.path().join("locked.txt");
        fs::write(&file, "secret").await.unwrap();

        // Remove all read/write/execute permissions
        let mut perms = fs::metadata(&file).await.unwrap().permissions();
        perms.set_mode(0o000);
        fs::set_permissions(&file, perms).await.unwrap();

        assert!(!is_readable(&file).await, "expected false for file without read permissions");
    }
}
