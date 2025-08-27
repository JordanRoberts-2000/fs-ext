use std::{fs, fs::FileType, io, path::Path};

pub fn file_type(path: impl AsRef<Path>) -> io::Result<FileType> {
    _file_type(path.as_ref())
}

fn _file_type(path: &Path) -> io::Result<FileType> {
    fs::metadata(path).map(|meta| meta.file_type()).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to get file type for '{}': {e}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use {
        super::file_type,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn returns_filetype_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("a.txt");
        fs::write(&file, "hi").unwrap();

        let ft = file_type(&file).unwrap();
        assert!(ft.is_file(), "expected is_file() to be true");
    }

    #[test]
    fn err_for_missing_path() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = file_type(&missing).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
