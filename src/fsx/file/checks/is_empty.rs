use {
    crate::fsx,
    std::{io, path::Path},
};

pub fn is_empty(path: &Path) -> io::Result<bool> {
    Ok(fsx::file::size(path)? == 0)
}

#[cfg(test)]
mod tests {
    use {
        super::is_empty,
        std::{fs, io},
        tempfile::tempdir,
    };

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

    #[test]
    fn propagates_error_for_missing_file() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("nope.txt");

        let err = is_empty(&missing).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
