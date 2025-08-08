use std::{
    fs,
    io::{self, Write},
    path::Path,
};

pub fn ensure_or_init_with<F, C>(path: impl AsRef<Path>, content_fn: F) -> io::Result<bool>
where
    F: FnOnce() -> C,
    C: AsRef<[u8]>,
{
    _ensure_or_init_with(path.as_ref(), content_fn)
}

fn _ensure_or_init_with<F, C>(path: &Path, content_fn: F) -> io::Result<bool>
where
    F: FnOnce() -> C,
    C: AsRef<[u8]>,
{
    match fs::OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => {
            let content = content_fn();
            file.write_all(content.as_ref())?;
            Ok(true) // File created
        }

        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            let meta = fs::metadata(path).map_err(|e| {
                io::Error::new(e.kind(), format!("Failed to inspect '{}': {e}", path.display()))
            })?;

            if !meta.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Path '{}' exists but is not a file (file type: {:?})",
                        path.display(),
                        meta.file_type()
                    ),
                ));
            }

            Ok(false) // File exists already
        }

        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("Failed to create file at '{}': {e}", path.display()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use {
        super::ensure_or_init_with,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn creates_file_and_writes_closure_content() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new.txt");

        let created = ensure_or_init_with(&file_path, || "from closure").unwrap();
        assert!(created, "Expected file to be created");

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "from closure");
    }

    #[test]
    fn returns_false_if_file_already_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, "original").unwrap();

        let created = ensure_or_init_with(&file_path, || "should not overwrite").unwrap();
        assert!(!created, "Expected no creation when file already exists");

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "original", "File content should remain unchanged");
    }

    #[test]
    fn errors_if_path_is_directory() {
        let dir = tempdir().unwrap();
        let subdir_path = dir.path().join("folder");

        fs::create_dir(&subdir_path).unwrap();

        let result = ensure_or_init_with(&subdir_path, || "data");
        assert!(result.is_err(), "Expected error when path is a directory");

        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn supports_vec_u8_return_type_from_closure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bin");

        let created = ensure_or_init_with(&file_path, || vec![1u8, 2, 3]).unwrap();
        assert!(created);

        let data = fs::read(&file_path).unwrap();
        assert_eq!(data, vec![1u8, 2, 3]);
    }

    #[test]
    fn supports_string_return_type_from_closure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("string.txt");

        let created = ensure_or_init_with(&file_path, || String::from("Hello")).unwrap();
        assert!(created);

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hello");
    }
}
