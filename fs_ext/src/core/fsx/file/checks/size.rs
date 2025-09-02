use std::{fs, io, path::Path};

pub fn size(path: impl AsRef<Path>) -> io::Result<u64> {
    _size(path.as_ref())
}

fn _size(path: &Path) -> io::Result<u64> {
    fs::metadata(path).map(|m| m.len()).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to get size of '{}': {e}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use {
        super::size,
        std::{fs, io},
        tempfile::tempdir,
    };

    #[test]
    fn returns_size_for_regular_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.bin");

        fs::write(&file, b"hello").unwrap();

        let len = size(&file).unwrap();
        assert_eq!(len, 5);
    }

    #[test]
    fn returns_zero_for_empty_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.bin");

        fs::File::create(&file).unwrap();

        let len = size(&file).unwrap();
        assert_eq!(len, 0);
    }

    #[test]
    fn err_when_missing() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = size(&missing).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
