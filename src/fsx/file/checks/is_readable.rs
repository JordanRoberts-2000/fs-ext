use std::{fs, path::Path};

pub fn is_readable(path: impl AsRef<Path>) -> bool {
    fs::File::open(path).is_ok()
}

#[cfg(test)]
mod tests {
    use {super::is_readable, std::fs, tempfile::tempdir};

    #[test]
    fn returns_true_for_readable_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("file.txt");
        fs::write(&file, "hello").unwrap();

        assert!(is_readable(&file), "expected true for readable file");
    }

    #[test]
    fn returns_false_for_missing_file() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        assert!(!is_readable(&missing), "expected false for missing file");
    }

    #[cfg(unix)]
    #[test]
    fn returns_false_for_non_readable_file() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let file = dir.path().join("locked.txt");
        fs::write(&file, "secret").unwrap();

        // Remove all read/write/execute permissions
        let mut perms = fs::metadata(&file).unwrap().permissions();
        perms.set_mode(0o000);
        fs::set_permissions(&file, perms).unwrap();

        assert!(!is_readable(&file), "expected false for file without read permissions");
    }
}
