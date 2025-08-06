use std::{fs, io, path::Path};

pub fn ensure(path: impl AsRef<Path>) -> io::Result<bool> {
    _ensure(path.as_ref())
}

fn _ensure(path: &Path) -> io::Result<bool> {
    match fs::OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(_) => Ok(true),

        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => match fs::metadata(path) {
            Ok(meta) if meta.is_file() => Ok(false),

            Ok(meta) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Path '{}' exists but is not a file (file type: {:?})",
                    path.display(),
                    meta.file_type()
                ),
            )),

            Err(e) => Err(io::Error::new(
                e.kind(),
                format!("Failed to inspect existing path '{}': {e}", path.display()),
            )),
        },

        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("Failed to create file at '{}': {e}", path.display()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use {
        super::ensure,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn creates_file_if_missing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new_file.txt");

        let created = ensure(&file_path).unwrap();

        assert!(created, "File should be created");
        assert!(file_path.exists());
        assert!(file_path.is_file());
    }

    #[test]
    fn returns_false_if_file_already_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, "hello").unwrap();
        let created = ensure(&file_path).unwrap();

        assert!(!created, "File already existed, should return false");
    }

    #[test]
    fn errors_if_path_is_a_directory() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join("folder.ts");

        fs::create_dir(&dir_path).unwrap();
        let err = ensure(&dir_path).unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
