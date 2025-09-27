use {
    crate::file,
    std::{io, path::Path},
};

#[cfg_attr(test, fs_ext_test_macros::fs_test(rejects_missing_path, rejects_dir, existing_file_ok))]
pub fn is_empty(path: impl AsRef<Path>) -> io::Result<bool> {
    Ok(file::size(path)? == 0)
}

#[cfg(test)]
mod tests {
    use {super::is_empty, std::fs, tempfile::tempdir};

    #[test]
    fn returns_true_for_empty_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("empty.txt");
        fs::File::create(&file).unwrap();

        let res = is_empty(&file).unwrap();
        assert!(res, "expected true for empty file");
    }

    #[test]
    fn returns_false_for_non_empty_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("data.txt");
        fs::write(&file, "hello").unwrap();

        let res = is_empty(&file).unwrap();
        assert!(!res, "expected false for non-empty file");
    }
}
