use std::{fs, io, path::Path};

#[cfg_attr(test, fs_ext_test_macros::fs_test(existing_file_ok))]
pub fn exists(path: impl AsRef<Path>) -> io::Result<bool> {
    _exists(path.as_ref())
}

fn _exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(meta) => Ok(meta.is_file()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => {
            Err(io::Error::new(e.kind(), format!("Failed to access '{}': {e}", path.display())))
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::exists, std::fs, tempfile::tempdir};

    #[test]
    fn returns_false_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let res = exists(&missing).unwrap();
        assert!(!res, "expected false for missing path");
    }

    #[test]
    fn returns_false_for_directory() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("folder");
        fs::create_dir_all(&subdir).unwrap();

        let res = exists(&subdir).unwrap();
        assert!(!res, "expected false for directory");
    }
}
