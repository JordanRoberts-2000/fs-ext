use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

pub fn overwrite(path: impl AsRef<Path>) -> io::Result<File> {
    _overwrite(path.as_ref())
}

fn _overwrite(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).create(true).truncate(true).open(path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open or overwrite file at '{}': {e}", path.display()),
        )
    })
}

#[cfg(test)]
mod tests {
    use {
        super::overwrite,
        std::{
            fs,
            io::{self, Read},
        },
        tempfile::tempdir,
    };

    #[test]
    fn creates_file_if_missing_and_is_empty() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new_file.txt");

        let _file = overwrite(&file_path).unwrap();

        assert!(file_path.exists(), "File should exist after overwrite()");
        let meta = fs::metadata(&file_path).unwrap();
        assert!(meta.is_file(), "Path should be a file");
        assert_eq!(meta.len(), 0, "Newly created file should be empty");
    }

    #[test]
    fn truncates_existing_file_to_zero_length() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("existing.txt");

        fs::write(&file_path, b"some existing content").unwrap();
        let meta_before = fs::metadata(&file_path).unwrap();
        assert!(meta_before.len() > 0, "Precondition: file should have content");

        let _file = overwrite(&file_path).unwrap();

        let meta_after = fs::metadata(&file_path).unwrap();
        assert_eq!(meta_after.len(), 0, "File should be truncated to zero length");

        let contents = fs::read(&file_path).unwrap();
        assert!(contents.is_empty(), "Contents should be empty after overwrite()");
    }

    #[test]
    fn returned_file_is_writable_after_overwrite() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("writable.txt");

        let mut file = overwrite(&file_path).unwrap();
        use std::io::Write;
        write!(file, "hello").unwrap();

        let mut read_back = String::new();
        fs::File::open(&file_path).unwrap().read_to_string(&mut read_back).unwrap();
        assert_eq!(read_back, "hello", "Should be able to write via returned handle");
    }

    #[test]
    fn errors_if_path_is_a_directory() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join("folder");

        fs::create_dir(&dir_path).unwrap();

        let err = overwrite(&dir_path).unwrap_err();

        // On Unix commonly IsADirectory; on Windows often PermissionDenied; sometimes Other.
        let kind = err.kind();
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
