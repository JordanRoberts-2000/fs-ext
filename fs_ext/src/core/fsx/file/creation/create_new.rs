use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_dir, rejects_existing_file))]
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
            io::{Read, Write},
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
    fn returned_file_is_writable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("writable.txt");

        let mut file = create_new(&file_path).unwrap();
        write!(file, "hello").unwrap();

        let mut read_back = String::new();
        fs::File::open(&file_path).unwrap().read_to_string(&mut read_back).unwrap();
        assert_eq!(read_back, "hello", "Should be able to write via returned handle");
    }
}
