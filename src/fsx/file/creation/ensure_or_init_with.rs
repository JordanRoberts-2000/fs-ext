use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub fn ensure_or_init_with<F, C>(path: impl AsRef<Path>, content_fn: F) -> io::Result<File>
where
    F: FnOnce() -> C,
    C: AsRef<[u8]>,
{
    _ensure_or_init_with(path.as_ref(), content_fn)
}

fn _ensure_or_init_with<F, C>(path: &Path, content_fn: F) -> io::Result<File>
where
    F: FnOnce() -> C,
    C: AsRef<[u8]>,
{
    match OpenOptions::new().write(true).open(path) {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let mut file = OpenOptions::new().write(true).create(true).open(path).map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!("Failed to create file at '{}': {e}", path.display()),
                )
            })?;

            let content = content_fn();
            file.write_all(content.as_ref()).map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!("Failed to write to file at '{}': {e}", path.display()),
                )
            })?;

            Ok(file)
        }
        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("Failed to open file at '{}': {e}", path.display()),
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

        let _file = ensure_or_init_with(&file_path, || "from closure").unwrap();

        assert!(file_path.exists(), "File should exist after ensure_or_init_with()");
        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "from closure");
    }

    #[test]
    fn opens_existing_file_without_overwriting() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, "original").unwrap();

        let _file = ensure_or_init_with(&file_path, || "should not overwrite").unwrap();

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "original", "Existing file content must remain unchanged");
    }

    #[test]
    fn errors_if_path_is_directory() {
        let dir = tempdir().unwrap();
        let subdir_path = dir.path().join("folder");

        fs::create_dir(&subdir_path).unwrap();

        let result = ensure_or_init_with(&subdir_path, || "data");
        assert!(result.is_err(), "Expected error when path is a directory");

        // Platform-specific: Unix often IsADirectory; Windows often PermissionDenied.
        let kind = result.unwrap_err().kind();
        assert!(
            matches!(
                kind,
                io::ErrorKind::IsADirectory
                    | io::ErrorKind::PermissionDenied
                    | io::ErrorKind::Other
            ),
            "Unexpected error kind for directory: {kind:?}"
        );
    }

    #[test]
    fn supports_vec_u8_return_type_from_closure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bin");

        let _file = ensure_or_init_with(&file_path, || vec![1u8, 2, 3]).unwrap();

        assert!(file_path.exists(), "File should exist");
        let data = fs::read(&file_path).unwrap();
        assert_eq!(data, vec![1u8, 2, 3]);
    }

    #[test]
    fn supports_string_return_type_from_closure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("string.txt");

        let _file = ensure_or_init_with(&file_path, || String::from("Hello")).unwrap();

        assert!(file_path.exists(), "File should exist");
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hello");
    }
}
