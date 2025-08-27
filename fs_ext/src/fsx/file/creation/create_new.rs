use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

pub fn create_new(path: impl AsRef<Path>) -> io::Result<File> {
    _create_new(path.as_ref())
}

fn _create_new(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).create_new(true).open(path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to create file at '{}': {e}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use {
        super::create_new,
        std::{
            fs,
            io::{self, Read, Write},
        },
        tempfile::tempdir,
    };

    #[test]
    fn creates_file_when_missing_and_is_empty() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new.txt");

        let _file = create_new(&file_path).unwrap();

        assert!(file_path.exists(), "File should exist after create_new()");
        let meta = fs::metadata(&file_path).unwrap();
        assert!(meta.is_file(), "Path should be a file");
        assert_eq!(meta.len(), 0, "Newly created file should be empty");
    }

    #[test]
    fn errors_if_file_already_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");

        fs::write(&file_path, b"hello").unwrap();

        let err = create_new(&file_path).unwrap_err();
        assert_eq!(
            err.kind(),
            io::ErrorKind::AlreadyExists,
            "create_new() must fail with AlreadyExists when file is present"
        );
    }

    #[test]
    fn returned_file_is_writable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("writable.txt");

        let mut file = create_new(&file_path).unwrap();
        write!(file, "hello").unwrap();

        let mut read_back = String::new();
        fs::File::open(&file_path).unwrap().read_to_string(&mut read_back).unwrap();
        assert_eq!(read_back, "hello", "Should be able to write via returned handle");
    }

    #[test]
    fn errors_if_path_is_a_directory() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().join("a_dir");
        fs::create_dir(&dir_path).unwrap();

        let err = create_new(&dir_path).unwrap_err();

        // On Unix commonly IsADirectory; on Windows often PermissionDenied; sometimes Other.
        let kind = err.kind();
        assert!(
            matches!(
                kind,
                io::ErrorKind::IsADirectory
                    | io::ErrorKind::PermissionDenied
                    | io::ErrorKind::AlreadyExists
                    | io::ErrorKind::Other
            ),
            "Unexpected error kind for directory: {kind:?}"
        );
    }
}
