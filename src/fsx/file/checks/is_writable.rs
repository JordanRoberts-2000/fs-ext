use std::{fs, path::Path};

pub fn is_writable(path: &Path) -> bool {
    fs::OpenOptions::new().write(true).open(path).is_ok()
}

#[cfg(test)]
mod tests {
    use {super::is_writable, std::fs, tempfile::tempdir};

    #[test]
    fn returns_true_for_writable_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("file.txt");
        fs::write(&file, "hello").unwrap();

        assert!(is_writable(&file), "expected true for writable file");
    }

    #[test]
    fn returns_false_for_missing_file() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        assert!(!is_writable(&missing), "expected false for missing file");
    }

    #[cfg(unix)]
    #[test]
    fn returns_false_for_non_writable_file() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let file = dir.path().join("locked.txt");
        fs::write(&file, "secret").unwrap();

        // Remove all write permissions (read-only: 0o444)
        let mut perms = fs::metadata(&file).unwrap().permissions();
        perms.set_mode(0o444);
        fs::set_permissions(&file, perms).unwrap();

        assert!(!is_writable(&file), "expected false for file without write permissions");
    }
}
