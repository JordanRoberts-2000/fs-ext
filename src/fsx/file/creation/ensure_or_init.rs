use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub fn ensure_or_init(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> io::Result<File> {
    _ensure_or_init(path.as_ref(), content.as_ref())
}

fn _ensure_or_init(path: &Path, content: &[u8]) -> io::Result<File> {
    match OpenOptions::new().write(true).open(path) {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let mut file = OpenOptions::new().write(true).create(true).open(path).map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!("Failed to create file at '{}': {e}", path.display()),
                )
            })?;

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
        super::ensure_or_init,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn creates_file_and_writes_content() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        let _file = ensure_or_init(&file_path, "hello world").unwrap();

        assert!(file_path.exists(), "File should exist after ensure_or_init()");
        let meta = fs::metadata(&file_path).unwrap();
        assert!(meta.is_file(), "Path should be a file");

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "hello world");
    }

    #[test]
    fn opens_existing_file_without_overwriting() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");

        fs::write(&file_path, "original").unwrap();

        let _file = ensure_or_init(&file_path, "new content").unwrap();

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "original", "Existing file contents must remain unchanged");
    }

    #[test]
    fn errors_if_path_is_directory() {
        let dir = tempdir().unwrap();
        let sub_dir = dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        let result = ensure_or_init(&sub_dir, "data");
        assert!(result.is_err(), "Should error if path is a directory");

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
}
