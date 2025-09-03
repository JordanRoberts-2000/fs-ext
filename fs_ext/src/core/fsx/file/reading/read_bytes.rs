use std::{fs, io, path::Path};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir))]
pub fn read_bytes(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    _read_bytes(path.as_ref())
}

fn _read_bytes(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to read file '{}': {e}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use {super::read_bytes, std::fs, tempfile::tempdir};

    #[test]
    fn returns_bytes_for_existing_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");
        fs::write(&file, b"hello").unwrap();

        let bytes = read_bytes(&file).unwrap();
        assert_eq!(bytes, b"hello");
    }
}
