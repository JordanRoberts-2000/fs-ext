use {
    crate::IoResultExt,
    std::{fs, io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir))]
pub fn size(path: impl AsRef<Path>) -> io::Result<u64> {
    _size(path.as_ref())
}

fn _size(path: &Path) -> io::Result<u64> {
    let meta = fs::metadata(path).with_path_context("Failed to get metadata", path)?;

    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Path '{}' is not a regular file", path.display()),
        ));
    }

    Ok(meta.len())
}

#[cfg(test)]
mod tests {
    use {super::size, std::fs, tempfile::tempdir};

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
}
